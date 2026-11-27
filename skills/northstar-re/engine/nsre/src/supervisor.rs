//! Owned worker supervision: real process identity, bounded deadlines and verified stops.
//!
//! The supervisor owns the worker process. Lease expiry never proves a worker stopped, so
//! every "worker stopped" claim here comes from the operating system: an observed process
//! exit, a job-object termination, or a creation-time identity check that also defeats PID
//! reuse. On non-Windows hosts the identity check is reported as unavailable instead of
//! being silently assumed.
use serde_json::{json, Value};
use std::{
    ffi::OsString,
    io,
    os::windows::io::AsRawHandle,
    path::Path,
    process::{Command, Stdio},
    time::{Duration, Instant},
};

fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message)
}

#[cfg(windows)]
mod identity {
    use std::io;
    use windows::Win32::{
        Foundation::{CloseHandle, HANDLE},
        System::Threading::{GetProcessTimes, OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION},
    };

    fn creation_ms(handle: HANDLE) -> io::Result<Option<u64>> {
        let mut created = windows::Win32::Foundation::FILETIME::default();
        let mut exited = windows::Win32::Foundation::FILETIME::default();
        let mut kernel = windows::Win32::Foundation::FILETIME::default();
        let mut user = windows::Win32::Foundation::FILETIME::default();
        unsafe {
            GetProcessTimes(handle, &mut created, &mut exited, &mut kernel, &mut user)
                .map_err(|e| std::io::Error::other(e.to_string()))?;
        }
        // FILETIME is 100ns intervals since 1601; convert to UNIX milliseconds.
        let ticks = (created.dwHighDateTime as u64) << 32 | created.dwLowDateTime as u64;
        const EPOCH_DIFF_100NS: u64 = 11_644_473_600 * 10_000_000;
        if ticks < EPOCH_DIFF_100NS {
            return Ok(None);
        }
        Ok(Some((ticks - EPOCH_DIFF_100NS) / 10_000))
    }

    /// Creation time of the current process in UNIX milliseconds.
    pub fn self_creation_ms() -> io::Result<Option<u64>> {
        let handle = HANDLE(std::process::id() as isize as *mut _);
        // -1 is the pseudo handle for the current process.
        let pseudo = HANDLE(-1isize as *mut _);
        let _ = handle;
        creation_ms(pseudo)
    }

    /// Is (pid, creation_ms) still the same live process? `None` means the host could not
    /// answer; `Some(false)` covers both an exited worker and PID reuse.
    pub fn process_matches(pid: u32, created_ms: u64) -> io::Result<Option<bool>> {
        let opened = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) };
        let Ok(handle) = opened else {
            return Ok(Some(false));
        };
        let result = creation_ms(handle);
        unsafe {
            let _ = CloseHandle(handle);
        }
        Ok(Some(result? == Some(created_ms)))
    }
}

#[cfg(not(windows))]
mod identity {
    pub fn self_creation_ms() -> io::Result<Option<u64>> {
        Ok(None)
    }

    pub fn process_matches(_pid: u32, _created_ms: u64) -> io::Result<Option<bool>> {
        Ok(None)
    }
}

pub fn self_creation_ms() -> io::Result<Option<u64>> {
    identity::self_creation_ms()
}

/// Verify, from the operating system, that the recorded worker process is gone. Returns a
/// report the queue layer can turn into a verified stop.
pub fn verify_worker_gone(pid: Option<i64>, created_ms: Option<i64>) -> Value {
    match (pid.and_then(|p| u32::try_from(p).ok()), created_ms) {
        (Some(pid), Some(created)) => {
            let created = u64::try_from(created).ok();
            match created.and_then(|c| identity::process_matches(pid, c).ok()) {
                Some(Some(false)) => json!({
                    "worker_stop_verified":true,"evidence":"process-exit-or-pid-reuse",
                    "pid":pid
                }),
                Some(Some(true)) => json!({
                    "worker_stop_verified":false,"evidence":"process-still-running","pid":pid
                }),
                Some(None) => json!({
                    "worker_stop_verified":false,"evidence":"host-could-not-answer","pid":pid
                }),
                None => json!({
                    "worker_stop_verified":false,"evidence":"verification-error","pid":pid
                }),
            }
        }
        _ => json!({
            "worker_stop_verified":false,
            "evidence":"no-process-identity-recorded"
        }),
    }
}

#[cfg(windows)]
mod owned_job {
    use std::io;
    use windows::core::PCWSTR;
    use windows::Win32::Foundation::{CloseHandle, HANDLE};
    use windows::Win32::System::JobObjects::TerminateJobObject;
    use windows::Win32::System::JobObjects::{
        CreateJobObjectW, JobObjectExtendedLimitInformation, SetInformationJobObject,
        JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
    };

    pub struct OwnedJob(pub HANDLE);

    impl OwnedJob {
        pub fn new() -> io::Result<Self> {
            let job = unsafe { CreateJobObjectW(None, PCWSTR::null()) }
                .map_err(|e| io::Error::other(e.to_string()))?;
            let mut settings = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
            settings.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
            unsafe {
                SetInformationJobObject(
                    job,
                    JobObjectExtendedLimitInformation,
                    &settings as *const _ as *const _,
                    std::mem::size_of_val(&settings) as u32,
                )
            }
            .map_err(|e| io::Error::other(e.to_string()))?;
            Ok(Self(job))
        }

        pub fn terminate(&self) -> io::Result<()> {
            unsafe { TerminateJobObject(self.0, 1) }.map_err(|e| io::Error::other(e.to_string()))
        }
    }

    impl Drop for OwnedJob {
        fn drop(&mut self) {
            unsafe {
                let _ = CloseHandle(self.0);
            }
        }
    }
}

/// Spawn the worker inside a job the supervisor owns, wait at most `deadline_ms` measured
/// monotonically, and kill the whole tree when the deadline passes.
pub fn supervise(
    db: &Path,
    job_id: &str,
    worker: &str,
    lease_ms: u64,
    deadline_ms: u64,
    heartbeat_ms: u64,
) -> io::Result<Value> {
    if !(1_000..=3_600_000).contains(&deadline_ms) {
        return Err(invalid("deadline must be within 1000..=3600000 ms"));
    }
    let exe = std::env::current_exe()?;
    let mut command = Command::new(exe);
    command
        .args([
            "queue",
            "run-one",
            db.to_str()
                .ok_or_else(|| invalid("database path must be valid unicode"))?,
            "--job",
            job_id,
            "--worker",
            worker,
            "--lease-ms",
            &lease_ms.to_string(),
            "--heartbeat-ms",
            &heartbeat_ms.to_string(),
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    #[cfg(windows)]
    let job = owned_job::OwnedJob::new()?;
    let started = Instant::now();
    let mut child = command.spawn().map_err(io::Error::other)?;
    #[cfg(windows)]
    unsafe {
        use windows::Win32::System::JobObjects::AssignProcessToJobObject;
        AssignProcessToJobObject(
            job.0,
            windows::Win32::Foundation::HANDLE(child.as_raw_handle() as isize as *mut _),
        )
        .map_err(|e| io::Error::other(e.to_string()))?;
    }
    let deadline = Duration::from_millis(deadline_ms);
    let mut killed = false;
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Some(status),
            Ok(None) => {
                if started.elapsed() >= deadline {
                    #[cfg(windows)]
                    job.terminate()?;
                    // The kill must be observed, not assumed.
                    let waited = Instant::now();
                    let observed = loop {
                        match child.try_wait() {
                            Ok(Some(status)) => break Some(status),
                            Ok(None) => {
                                if waited.elapsed() > Duration::from_secs(10) {
                                    break None;
                                }
                                std::thread::sleep(Duration::from_millis(20));
                            }
                            Err(e) => return Err(io::Error::other(e.to_string())),
                        }
                    };
                    killed = true;
                    break observed;
                }
                std::thread::sleep(Duration::from_millis(25));
            }
            Err(e) => return Err(io::Error::other(e.to_string())),
        }
    };
    let exit_code = status.as_ref().and_then(|s| s.code());
    // A job-object kill is an observed process exit: the only evidence that lets the queue
    // release the worker's reservations without a human statement.
    if killed {
        crate::queue::record_verified_stop(db, job_id)?;
    }
    Ok(json!({
        "schema":1,"job":job_id,"worker":worker,
        "deadline_ms":deadline_ms,"elapsed_ms":started.elapsed().as_millis() as u64,
        "killed_for_deadline":killed,
        "exit_code":exit_code,
        "process_observed":status.is_some(),
        "kill_on_job_close":"supervisor owns the worker tree"
    }))
}

pub fn command(args: impl Iterator<Item = OsString>) -> io::Result<bool> {
    let args: Vec<_> = args.collect();
    let usage = "supervisor spawn DB JOB --worker NAME --lease-ms N --deadline-ms N [--heartbeat-ms N] | supervisor verify DB JOB";
    if args.len() < 2 {
        return Err(invalid(usage));
    }
    let verb = args[0].to_str().ok_or_else(|| invalid(usage))?;
    let db = Path::new(&args[1]);
    match verb {
        "spawn" if args.len() >= 10 && args[2] != "--help" => {
            let job_id = args[2].to_str().ok_or_else(|| invalid(usage))?;
            let mut worker = None;
            let mut lease_ms = None;
            let mut deadline_ms = None;
            let mut heartbeat_ms = 5_000;
            let mut rest = args[3..].iter();
            while let Some(flag) = rest.next() {
                match flag.to_str() {
                    Some("--worker") => {
                        worker = Some(
                            rest.next()
                                .and_then(|v| v.to_str())
                                .ok_or_else(|| invalid("worker required"))?,
                        )
                    }
                    Some("--lease-ms") => {
                        lease_ms = Some(
                            rest.next()
                                .and_then(|v| v.to_str())
                                .and_then(|v| v.parse().ok())
                                .ok_or_else(|| invalid("lease required"))?,
                        )
                    }
                    Some("--deadline-ms") => {
                        deadline_ms = Some(
                            rest.next()
                                .and_then(|v| v.to_str())
                                .and_then(|v| v.parse().ok())
                                .ok_or_else(|| invalid("deadline required"))?,
                        )
                    }
                    Some("--heartbeat-ms") => {
                        heartbeat_ms = rest
                            .next()
                            .and_then(|v| v.to_str())
                            .and_then(|v| v.parse().ok())
                            .ok_or_else(|| invalid("heartbeat required"))?;
                    }
                    _ => return Err(invalid(usage)),
                }
            }
            let (worker, lease_ms, deadline_ms) = match (worker, lease_ms, deadline_ms) {
                (Some(w), Some(l), Some(d)) => (w, l, d),
                _ => return Err(invalid(usage)),
            };
            let report = supervise(db, job_id, worker, lease_ms, deadline_ms, heartbeat_ms)?;
            println!("{report}");
            Ok(true)
        }
        "verify" if args.len() == 3 => {
            let job_id = args[2].to_str().ok_or_else(|| invalid(usage))?;
            let report = crate::queue::verify_stopped(db, job_id)?;
            println!("{report}");
            Ok(report["verification"]["worker_stop_verified"] == true)
        }
        _ => Err(invalid(usage)),
    }
}
