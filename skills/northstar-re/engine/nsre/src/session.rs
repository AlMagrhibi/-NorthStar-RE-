//! Windows session discovery and execution-context detection (WISM-01/02/15/11).
//!
//! The engine must never assume that its own session is the interactive one. This module
//! answers three questions with operating-system evidence: which sessions exist, which one
//! is the active console, and in what context is this process actually running (Session 0,
//! a console session, RDP, or an SSH service session).
use serde_json::{json, Value};
use std::io;

fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message)
}

#[cfg(windows)]
mod wts {
    use std::io;
    use windows::core::PWSTR;
    use windows::Win32::System::RemoteDesktop::{
        ProcessIdToSessionId, WTSEnumerateSessionsW, WTSFreeMemory, WTSGetActiveConsoleSessionId,
        WTSQuerySessionInformationW, WTSINFOEXW, WTS_CURRENT_SERVER_HANDLE, WTS_INFO_CLASS,
        WTS_SESSIONSTATE_LOCK, WTS_SESSIONSTATE_UNKNOWN, WTS_SESSIONSTATE_UNLOCK,
        WTS_SESSION_INFOW,
    };

    pub const SESSION_STATE_CLASS: WTS_INFO_CLASS = WTS_INFO_CLASS(25); // WTSSessionInfoEx
    pub const SESSION_NAME_CLASS: WTS_INFO_CLASS = WTS_INFO_CLASS(6); // WTSWinStationName
    pub const SESSION_USER_CLASS: WTS_INFO_CLASS = WTS_INFO_CLASS(5); // WTSUserName

    /// Session id of the active console, 0xFFFFFFFF when none is active.
    pub fn active_console_session_id() -> u32 {
        unsafe { WTSGetActiveConsoleSessionId() }
    }

    pub fn current_session_id() -> io::Result<u32> {
        let mut id: u32 = 0;
        unsafe { ProcessIdToSessionId(std::process::id(), &mut id) }
            .map_err(|e| io::Error::other(e.to_string()))?;
        Ok(id)
    }

    /// NUL-terminated UTF-16 from a WTS-owned buffer.
    pub unsafe fn wide_string(pointer: *const u16) -> String {
        let mut length = 0usize;
        while *pointer.add(length) != 0 {
            length += 1;
        }
        String::from_utf16_lossy(std::slice::from_raw_parts(pointer, length))
    }

    pub struct SessionEntry {
        pub id: u32,
        pub state: i32,
        pub name: String,
    }

    pub fn enumerate() -> io::Result<Vec<SessionEntry>> {
        let mut count: u32 = 0;
        let mut buffer: *mut WTS_SESSION_INFOW = std::ptr::null_mut();
        unsafe {
            WTSEnumerateSessionsW(
                Some(WTS_CURRENT_SERVER_HANDLE),
                0,
                1,
                &mut buffer,
                &mut count,
            )
        }
        .map_err(|e| io::Error::other(e.to_string()))?;
        let mut sessions = Vec::new();
        if !buffer.is_null() {
            for index in 0..count {
                let entry = unsafe { buffer.add(index as usize).read() };
                let name = unsafe { wide_string(entry.pWinStationName.as_ptr()) };
                sessions.push(SessionEntry {
                    id: entry.SessionId,
                    state: entry.State.0,
                    name,
                });
            }
            unsafe { WTSFreeMemory(buffer.cast()) };
        }
        Ok(sessions)
    }

    fn query_raw(session_id: u32, class: WTS_INFO_CLASS) -> io::Result<Option<Vec<u8>>> {
        let mut bytes: u32 = 0;
        let mut buffer = PWSTR::null();
        unsafe {
            WTSQuerySessionInformationW(
                Some(WTS_CURRENT_SERVER_HANDLE),
                session_id,
                class,
                &mut buffer,
                &mut bytes,
            )
        }
        .map_err(|e| io::Error::other(e.to_string()))?;
        if buffer.is_null() || bytes == 0 {
            return Ok(None);
        }
        let data =
            unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const u8, bytes as usize) }
                .to_vec();
        unsafe { WTSFreeMemory(buffer.as_ptr().cast()) };
        Ok(Some(data))
    }

    /// WTSINFOEXW carries SessionFlags: 0 = locked, 1 = unlocked per WTS_SESSIONSTATE_*.
    pub fn session_flags(session_id: u32) -> io::Result<Option<i32>> {
        let Some(data) = query_raw(session_id, SESSION_STATE_CLASS)? else {
            return Ok(None);
        };
        if data.len() < std::mem::size_of::<WTSINFOEXW>() {
            return Ok(None);
        }
        let info = unsafe { std::ptr::read_unaligned(data.as_ptr() as *const WTSINFOEXW) };
        let flags = unsafe { info.Data.WTSInfoExLevel1.SessionFlags };
        Ok(Some(flags))
    }

    pub fn session_name(session_id: u32) -> io::Result<Option<String>> {
        let Some(data) = query_raw(session_id, SESSION_NAME_CLASS)? else {
            return Ok(None);
        };
        let wide: Vec<u16> = data
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .take_while(|c| *c != 0)
            .collect();
        Ok(Some(String::from_utf16_lossy(&wide)))
    }

    pub fn session_user(session_id: u32) -> io::Result<Option<String>> {
        let Some(data) = query_raw(session_id, SESSION_USER_CLASS)? else {
            return Ok(None);
        };
        let wide: Vec<u16> = data
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .take_while(|c| *c != 0)
            .collect();
        Ok(Some(String::from_utf16_lossy(&wide)))
    }

    pub fn flag_name(flags: i32) -> &'static str {
        match flags as u32 {
            WTS_SESSIONSTATE_LOCK => "locked",
            WTS_SESSIONSTATE_UNLOCK => "unlocked",
            WTS_SESSIONSTATE_UNKNOWN => "unknown",
            _ => "unrecognized",
        }
    }
}

#[cfg(not(windows))]
mod wts {
    pub fn current_session_id() -> io::Result<u32> {
        Err(io::Error::other("session discovery requires Windows"))
    }
}

#[cfg(windows)]
fn state_name(state: i32) -> &'static str {
    match state {
        0 => "active",
        1 => "connected",
        2 => "connect-query",
        3 => "shadow",
        4 => "disconnected",
        5 => "idle",
        6 => "listen",
        7 => "reset",
        8 => "down",
        9 => "init",
        _ => "unknown",
    }
}

/// Which sessions exist, which one is the active console, and the caller's own context.
pub fn inventory() -> io::Result<Value> {
    #[cfg(windows)]
    {
        let sessions = wts::enumerate()?;
        let console = wts::active_console_session_id();
        let mine = wts::current_session_id()?;
        let mut rows = Vec::new();
        for session in &sessions {
            let flags = wts::session_flags(session.id)?;
            rows.push(json!({
                "session_id": session.id,
                "name": session.name,
                "wts_station_name": wts::session_name(session.id)?,
                "connection_state": state_name(session.state),
                "session_flags": wts::flag_name(flags.unwrap_or(-1)),
                "user": wts::session_user(session.id)?,
                "is_active_console": session.id == console && console != 0xFFFFFFFF,
                "is_current_process_session": session.id == mine,
                "is_session_zero": session.id == 0
            }));
        }
        Ok(json!({
            "schema":1,
            "current_process_session":mine,
            "active_console_session": if console == 0xFFFFFFFF { json!(null) } else { json!(console) },
            "process_in_session_zero": mine == 0,
            "process_in_active_console": mine != 0 && mine == console,
            "sessions":rows
        }))
    }
    #[cfg(not(windows))]
    {
        Err(invalid("session discovery requires Windows"))
    }
}

/// The execution context of this process, classified for routing decisions (WISM-01/15).
pub fn context() -> io::Result<Value> {
    #[cfg(windows)]
    {
        let mine = wts::current_session_id()?;
        let console = wts::active_console_session_id();
        let session_name = std::env::var("SESSIONNAME").unwrap_or_default();
        let ssh_markers = ["SSH_CONNECTION", "SSH_TTY", "SSH_CLIENT"]
            .iter()
            .filter(|key| std::env::var(key).is_ok())
            .count();
        let parent_is_sshd = parent_process_is_sshd()?;
        let classification = if mine == 0 {
            "session-zero"
        } else if ssh_markers > 0 || parent_is_sshd {
            "ssh-service-session"
        } else if session_name.starts_with("RDP-") {
            "rdp-session"
        } else if mine == console && console != 0xFFFFFFFF {
            "active-console"
        } else {
            "non-console-session"
        };
        let flags = wts::session_flags(mine)?;
        Ok(json!({
            "schema":1,
            "session_id":mine,
            "session_name":session_name,
            "active_console_session": if console == 0xFFFFFFFF { json!(null) } else { json!(console) },
            "own_session_lock_state": flags.map(wts::flag_name).unwrap_or("unavailable"),
            "ssh_environment_markers":ssh_markers,
            "parent_is_sshd":parent_is_sshd,
            "classification":classification,
            "interactive_desktop_available": classification == "active-console",
            "direct_desktop_execution_permitted": classification == "active-console"
        }))
    }
    #[cfg(not(windows))]
    {
        Err(invalid("session context detection requires Windows"))
    }
}

#[cfg(windows)]
fn parent_process_is_sshd() -> io::Result<bool> {
    use windows::Win32::Foundation::CloseHandle;
    use windows::Win32::System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W,
        TH32CS_SNAPPROCESS,
    };
    let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
    let Ok(snapshot) = snapshot else {
        return Ok(false);
    };
    let me = std::process::id();
    let mut parent: u32 = 0;
    let mut entry = PROCESSENTRY32W {
        dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
        ..PROCESSENTRY32W::default()
    };
    unsafe {
        if Process32FirstW(snapshot, &mut entry).is_ok() {
            loop {
                if entry.th32ProcessID == me {
                    parent = entry.th32ParentProcessID;
                    break;
                }
                if Process32NextW(snapshot, &mut entry).is_err() {
                    break;
                }
            }
        }
    }
    if parent == 0 {
        unsafe {
            let _ = CloseHandle(snapshot);
        }
        return Ok(false);
    }
    let mut found = false;
    unsafe {
        if Process32FirstW(snapshot, &mut entry).is_ok() {
            loop {
                if entry.th32ProcessID == parent {
                    let name = String::from_utf16_lossy(
                        &entry.szExeFile
                            [..entry.szExeFile.iter().position(|c| *c == 0).unwrap_or(0)],
                    );
                    found = name.eq_ignore_ascii_case("sshd.exe");
                    break;
                }
                if Process32NextW(snapshot, &mut entry).is_err() {
                    break;
                }
            }
        }
        let _ = CloseHandle(snapshot);
    }
    Ok(found)
}

#[cfg(not(windows))]
fn parent_process_is_sshd() -> io::Result<bool> {
    Ok(false)
}

pub fn command(args: impl Iterator<Item = std::ffi::OsString>) -> io::Result<bool> {
    let args: Vec<_> = args.collect();
    let usage = "winsessions list | winsessions context";
    if args.len() != 1 {
        return Err(invalid(usage));
    }
    match args[0].to_str() {
        Some("list") => println!("{}", inventory()?),
        Some("context") => println!("{}", context()?),
        _ => return Err(invalid(usage)),
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(windows)]
    #[test]
    fn context_classifies_the_real_execution_context() {
        let report = context().unwrap();
        assert_eq!(report["schema"], 1);
        let session = report["session_id"].as_u64().unwrap();
        let classification = report["classification"].as_str().unwrap();
        // The engine must know whether it is in Session 0, and never claim an interactive
        // desktop from a service or SSH context.
        if session == 0 {
            assert_eq!(classification, "session-zero");
            assert_eq!(report["direct_desktop_execution_permitted"], false);
            assert_eq!(report["interactive_desktop_available"], false);
        } else {
            assert!([
                "active-console",
                "ssh-service-session",
                "rdp-session",
                "non-console-session"
            ]
            .contains(&classification));
        }
    }

    #[cfg(windows)]
    #[test]
    fn inventory_reports_the_active_console_without_hardcoding_it() {
        let report = inventory().unwrap();
        assert_eq!(report["schema"], 1);
        let sessions = report["sessions"].as_array().unwrap();
        assert!(!sessions.is_empty(), "a Windows host always has sessions");
        // Session 0 exists on every Windows host and never hosts the interactive desktop.
        assert!(sessions
            .iter()
            .any(|s| s["is_session_zero"] == true && s["name"] == "Services"));
        // The active console is discovered from the OS, not assumed to be 1.
        if let Some(console) = report["active_console_session"].as_u64() {
            assert!(sessions
                .iter()
                .any(|s| s["session_id"].as_u64() == Some(console)));
        }
        // Our own session is reported exactly once.
        let mine = report["current_process_session"].as_u64().unwrap();
        assert_eq!(
            sessions
                .iter()
                .filter(|s| s["is_current_process_session"] == true)
                .count(),
            1
        );
        let _ = mine;
    }
}
