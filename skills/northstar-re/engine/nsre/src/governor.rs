//! Windows-enforced limits for an owned native batch worker. No arbitrary executable or shell.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    ffi::OsString,
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
    path::{Path, PathBuf},
    time::Instant,
};
use windows::{
    core::{PCWSTR, PWSTR},
    Win32::{
        Foundation::{CloseHandle, HANDLE, WAIT_OBJECT_0, WAIT_TIMEOUT},
        System::{JobObjects::*, Threading::*},
    },
};
fn invalid(s: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, s)
}
fn failure(e: windows::core::Error) -> io::Error {
    io::Error::other(e.to_string())
}
struct Handle(HANDLE);
impl Drop for Handle {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.0);
        }
    }
}
struct Worker {
    process: Handle,
    thread: Handle,
}
impl Drop for Worker {
    fn drop(&mut self) {
        unsafe {
            let _ = TerminateProcess(self.process.0, 2);
            WaitForSingleObject(self.process.0, 5000);
        }
    }
}
struct Temp(PathBuf);
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn quote(s: &str) -> io::Result<String> {
    if s.contains('\0') {
        return Err(invalid("NUL in worker argument"));
    }
    let mut out = String::from("\"");
    let mut slashes = 0;
    for c in s.chars() {
        if c == '\\' {
            slashes += 1;
            continue;
        }
        if c == '"' {
            out.extend(std::iter::repeat_n('\\', slashes * 2 + 1));
        } else {
            out.extend(std::iter::repeat_n('\\', slashes));
        }
        out.push(c);
        slashes = 0;
    }
    out.extend(std::iter::repeat_n('\\', slashes * 2));
    out.push('"');
    Ok(out)
}
fn limits(memory: usize, cpu: u32, timeout: u32) -> io::Result<()> {
    if !(16..=16384).contains(&memory)
        || !(1..=100).contains(&cpu)
        || !(1..=600000).contains(&timeout)
    {
        return Err(invalid(
            "memory MiB 16..=16384, CPU percent 1..=100 and timeout ms 1..=600000 required",
        ));
    }
    Ok(())
}
fn bounded(path: &Path, limit: u64) -> io::Result<Vec<u8>> {
    let m = fs::symlink_metadata(path)?;
    if !m.is_file() || crate::unsafe_entry(&m) || m.len() > limit {
        return Err(invalid("invalid bounded worker input/result"));
    }
    let mut bytes = Vec::new();
    File::open(path)?.take(limit + 1).read_to_end(&mut bytes)?;
    if bytes.len() as u64 > limit {
        return Err(invalid("worker input/result exceeded size bound"));
    }
    Ok(bytes)
}
fn run(
    input: &Path,
    output: &Path,
    jobs: usize,
    memory: usize,
    cpu: u32,
    timeout: u32,
) -> io::Result<bool> {
    limits(memory, cpu, timeout)?;
    let bytes = bounded(input, 1024 * 1024)?;
    let manifest: Value = serde_json::from_slice(&bytes).map_err(io::Error::other)?;
    crate::batch::validate(&manifest, jobs)?;
    let parent = crate::collect::normal_dir(
        output
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(Path::new(".")),
    )?;
    let name = output
        .file_name()
        .and_then(|s| s.to_str())
        .filter(|s| !s.contains(':') && !s.ends_with(['.', ' ']))
        .ok_or_else(|| invalid("normal output filename required"))?;
    let output = parent.join(name);
    if output.try_exists()? {
        return Err(invalid("governed output already exists"));
    }
    let temp = Temp(parent.join(format!(
            ".nsre-governed-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(io::Error::other)?
                .as_nanos()
        )));
    fs::create_dir(&temp.0)?;
    let snapshot = temp.0.join("input.json");
    let artifact = temp.0.join("result.json");
    let report_path = temp.0.join("report.json");
    {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&snapshot)?;
        file.write_all(&bytes)?;
        file.sync_all()?;
    }
    let job = Handle(unsafe { CreateJobObjectW(None, PCWSTR::null()) }.map_err(failure)?);
    let mut settings = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
    settings.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE
        | JOB_OBJECT_LIMIT_PROCESS_MEMORY
        | JOB_OBJECT_LIMIT_JOB_MEMORY
        | JOB_OBJECT_LIMIT_ACTIVE_PROCESS;
    settings.BasicLimitInformation.ActiveProcessLimit = 1;
    settings.ProcessMemoryLimit = memory * 1024 * 1024;
    settings.JobMemoryLimit = settings.ProcessMemoryLimit;
    unsafe {
        SetInformationJobObject(
            job.0,
            JobObjectExtendedLimitInformation,
            &settings as *const _ as *const _,
            std::mem::size_of_val(&settings) as u32,
        )
    }
    .map_err(failure)?;
    let cpu_settings = JOBOBJECT_CPU_RATE_CONTROL_INFORMATION {
        ControlFlags: JOB_OBJECT_CPU_RATE_CONTROL_ENABLE | JOB_OBJECT_CPU_RATE_CONTROL_HARD_CAP,
        Anonymous: JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0 { CpuRate: cpu * 100 },
    };
    unsafe {
        SetInformationJobObject(
            job.0,
            JobObjectCpuRateControlInformation,
            &cpu_settings as *const _ as *const _,
            std::mem::size_of_val(&cpu_settings) as u32,
        )
    }
    .map_err(failure)?;
    let exe = std::env::current_exe()?;
    let exe = exe
        .to_str()
        .ok_or_else(|| invalid("invalid executable path"))?;
    let words = [
        exe.to_owned(),
        "batch".to_owned(),
        "--input".to_owned(),
        snapshot
            .to_str()
            .ok_or_else(|| invalid("invalid input path"))?
            .to_owned(),
        "--out".to_owned(),
        artifact
            .to_str()
            .ok_or_else(|| invalid("invalid result path"))?
            .to_owned(),
        "--jobs".to_owned(),
        jobs.to_string(),
        "--reviewed-redacted".to_owned(),
    ];
    let command = words
        .iter()
        .map(|s| quote(s))
        .collect::<io::Result<Vec<_>>>()?
        .join(" ");
    if command.encode_utf16().count() > 32766 {
        return Err(invalid("worker command exceeds Windows limit"));
    }
    let mut wide: Vec<u16> = command.encode_utf16().chain([0]).collect();
    let exe_wide: Vec<u16> = exe.encode_utf16().chain([0]).collect();
    let startup = STARTUPINFOW {
        cb: std::mem::size_of::<STARTUPINFOW>() as u32,
        ..Default::default()
    };
    let mut info = PROCESS_INFORMATION::default();
    unsafe {
        CreateProcessW(
            PCWSTR(exe_wide.as_ptr()),
            Some(PWSTR(wide.as_mut_ptr())),
            None,
            None,
            false,
            CREATE_SUSPENDED | CREATE_NO_WINDOW,
            None,
            PCWSTR::null(),
            &startup,
            &mut info,
        )
    }
    .map_err(failure)?;
    let worker = Worker {
        process: Handle(info.hProcess),
        thread: Handle(info.hThread),
    };
    unsafe { AssignProcessToJobObject(job.0, worker.process.0) }.map_err(failure)?;
    // No user batch code runs until assignment and all OS limits have succeeded.
    let started = Instant::now();
    if unsafe { ResumeThread(worker.thread.0) } == u32::MAX {
        return Err(io::Error::last_os_error());
    }
    let wait = unsafe { WaitForSingleObject(worker.process.0, timeout) };
    let timed_out = wait == WAIT_TIMEOUT;
    if timed_out {
        unsafe { TerminateJobObject(job.0, 2) }.map_err(failure)?;
        if unsafe { WaitForSingleObject(worker.process.0, 5000) } != WAIT_OBJECT_0 {
            return Err(invalid("owned worker termination not confirmed"));
        }
    } else if wait != WAIT_OBJECT_0 {
        return Err(io::Error::last_os_error());
    }
    let elapsed = started.elapsed();
    let mut exit = 0;
    unsafe { GetExitCodeProcess(worker.process.0, &mut exit) }.map_err(failure)?;
    let mut measured = JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION::default();
    unsafe {
        QueryInformationJobObject(
            Some(job.0),
            JobObjectBasicAndIoAccountingInformation,
            &mut measured as *mut _ as *mut _,
            std::mem::size_of_val(&measured) as u32,
            None,
        )
    }
    .map_err(failure)?;
    let mut verified = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
    unsafe {
        QueryInformationJobObject(
            Some(job.0),
            JobObjectExtendedLimitInformation,
            &mut verified as *mut _ as *mut _,
            std::mem::size_of_val(&verified) as u32,
            None,
        )
    }
    .map_err(failure)?;
    let mut cpu_verified = JOBOBJECT_CPU_RATE_CONTROL_INFORMATION::default();
    unsafe {
        QueryInformationJobObject(
            Some(job.0),
            JobObjectCpuRateControlInformation,
            &mut cpu_verified as *mut _ as *mut _,
            std::mem::size_of_val(&cpu_verified) as u32,
            None,
        )
    }
    .map_err(failure)?;
    let enforced = verified.BasicLimitInformation.LimitFlags
        == settings.BasicLimitInformation.LimitFlags
        && verified.ProcessMemoryLimit == settings.ProcessMemoryLimit
        && verified.JobMemoryLimit == settings.JobMemoryLimit
        && cpu_verified.ControlFlags == cpu_settings.ControlFlags
        && unsafe { cpu_verified.Anonymous.CpuRate } == cpu * 100;
    if !enforced {
        return Err(invalid("OS resource limit readback mismatch"));
    }
    let (result, result_sha256) = if !timed_out && artifact.try_exists()? {
        let body = bounded(&artifact, 16 * 1024 * 1024)?;
        let v: Value = serde_json::from_slice(&body).map_err(io::Error::other)?;
        (Some(v), Some(format!("{:x}", Sha256::digest(&body))))
    } else {
        (None, None)
    };
    if bounded(&snapshot, 1024 * 1024)? != bytes {
        return Err(invalid("worker input snapshot changed"));
    }
    let passed = !timed_out
        && exit == 0
        && result
            .as_ref()
            .is_some_and(|r| r["complete"] == true && r["passed"] == true);
    let report = json!({"schema":1,"passed":passed,"timed_out":timed_out,"worker_exit_code":exit,"worker_pid":info.dwProcessId,"worker_stopped":true,"input_sha256":format!("{:x}",Sha256::digest(&bytes)),"result_sha256":result_sha256,"result":result,"measured_by_engine":true,"os_limits_verified":enforced,"limits":{"memory_mib":memory,"cpu_percent":cpu,"timeout_ms":timeout,"active_process_limit":1},"measurements":{"wall_ms":elapsed.as_millis(),"cpu_user_100ns":measured.BasicInfo.TotalUserTime,"cpu_kernel_100ns":measured.BasicInfo.TotalKernelTime,"peak_job_committed_bytes":verified.PeakJobMemoryUsed,"io_read_bytes":measured.IoInfo.ReadTransferCount,"io_write_bytes":measured.IoInfo.WriteTransferCount},"scope":"owned native offline batch only; not queue leases, filesystem sandbox or live target diagnostics"});
    {
        let mut f = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&report_path)?;
        serde_json::to_writer(&mut f, &report).map_err(io::Error::other)?;
        f.write_all(b"\n")?;
        f.sync_all()?;
    }
    fs::hard_link(&report_path, &output)?;
    println!("{report}");
    Ok(passed)
}
pub fn command(args: impl Iterator<Item = OsString>) -> io::Result<bool> {
    let a: Vec<_> = args.collect();
    if a.len() != 13
        || a[0] != "--input"
        || a[2] != "--out"
        || a[4] != "--jobs"
        || a[6] != "--timeout-ms"
        || a[8] != "--memory-mib"
        || a[10] != "--cpu-percent"
        || a[12] != "--reviewed-redacted"
    {
        return Err(invalid("governed-batch --input FILE --out NEW --jobs N --timeout-ms T --memory-mib M --cpu-percent C --reviewed-redacted"));
    }
    let number = |i: usize| -> io::Result<u32> {
        a[i].to_str()
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| invalid("invalid resource budget"))
    };
    run(
        Path::new(&a[1]),
        Path::new(&a[3]),
        number(5)? as usize,
        number(9)? as usize,
        number(11)?,
        number(7)?,
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn budgets_and_windows_argument_quoting() {
        assert!(limits(512, 50, 30000).is_ok());
        for (m, c, t) in [(0, 50, 1), (512, 0, 1), (512, 101, 1), (512, 50, 0)] {
            assert!(limits(m, c, t).is_err());
        }
        assert_eq!(quote("a b").unwrap(), "\"a b\"");
        assert_eq!(quote("x\\").unwrap(), "\"x\\\\\"");
        assert!(quote("\0").is_err());
    }
}
