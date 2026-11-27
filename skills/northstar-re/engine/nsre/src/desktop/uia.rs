//! Bounded UI Automation tree and unique AutomationId invocation. No Value/Text reads.
use super::*;
use std::{collections::VecDeque, time::Instant};
use windows::{
    core::Interface,
    Win32::{
        Foundation::HWND,
        System::Com::{
            CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_INPROC_SERVER,
            COINIT_MULTITHREADED,
        },
        UI::Accessibility::{
            CUIAutomation8, IUIAutomation, IUIAutomation2, IUIAutomationElement,
            IUIAutomationInvokePattern, UIA_InvokePatternId,
        },
    },
};
fn failure(e: windows::core::Error) -> io::Error {
    io::Error::other(e.to_string())
}
struct Apartment;
impl Apartment {
    fn new() -> io::Result<Self> {
        unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) }
            .ok()
            .map_err(failure)?;
        Ok(Self)
    }
}
impl Drop for Apartment {
    fn drop(&mut self) {
        unsafe { CoUninitialize() }
    }
}
fn automation() -> io::Result<IUIAutomation> {
    let timed: IUIAutomation2 =
        unsafe { CoCreateInstance(&CUIAutomation8, None, CLSCTX_INPROC_SERVER) }
            .map_err(failure)?;
    unsafe { timed.SetConnectionTimeout(1500) }.map_err(failure)?;
    unsafe { timed.SetTransactionTimeout(1500) }.map_err(failure)?;
    timed.cast().map_err(failure)
}
fn tree(
    hwnd: Hwnd,
    pid: u32,
    api: &IUIAutomation,
) -> io::Result<(Vec<(IUIAutomationElement, Value)>, bool)> {
    identity(hwnd, Some(pid))?;
    let root =
        unsafe { api.ElementFromHandle(HWND(hwnd as *mut std::ffi::c_void)) }.map_err(failure)?;
    let walker = unsafe { api.ControlViewWalker() }.map_err(failure)?;
    let mut pending = VecDeque::from([(root, 0usize)]);
    let mut nodes = Vec::new();
    let mut truncated = false;
    let started = Instant::now();
    while let Some((element, depth)) = pending.pop_front() {
        identity(hwnd, Some(pid))?;
        if nodes.len() >= 256 || started.elapsed() > Duration::from_secs(5) {
            truncated = true;
            break;
        }
        let owner = unsafe { element.CurrentProcessId() }.map_err(failure)?;
        if owner as u32 != pid {
            truncated = true;
            continue;
        }
        let password = unsafe { element.CurrentIsPassword() }
            .map(|b| b.as_bool())
            .unwrap_or(true);
        let id = unsafe { element.CurrentAutomationId() }
            .ok()
            .map(|s| s.to_string())
            .filter(|s| s.len() <= 256)
            .unwrap_or_default();
        let name = if password {
            None
        } else {
            unsafe { element.CurrentName() }
                .ok()
                .map(|s| s.to_string().chars().take(512).collect::<String>())
        };
        let control = unsafe { element.CurrentControlType() }
            .map(|c| c.0)
            .unwrap_or(0);
        let enabled = unsafe { element.CurrentIsEnabled() }
            .map(|b| b.as_bool())
            .unwrap_or(false);
        let focused = unsafe { element.CurrentHasKeyboardFocus() }
            .map(|b| b.as_bool())
            .unwrap_or(false);
        let data = json!({"automation_id":id,"name":name,"control_type":control,"pid":owner,"is_password":password,"enabled":enabled,"focused":focused,"depth":depth,"values_read":false});
        let mut next = unsafe { walker.GetFirstChildElement(&element) }.ok();
        if depth >= 8 {
            truncated |= next.is_some();
            next = None;
        }
        while let Some(child) = next {
            if nodes.len() + pending.len() >= 255 {
                truncated = true;
                break;
            }
            next = unsafe { walker.GetNextSiblingElement(&child) }.ok();
            pending.push_back((child, depth + 1));
        }
        nodes.push((element, data));
    }
    identity(hwnd, Some(pid))?;
    Ok((nodes, truncated))
}
pub(super) fn inspect(hwnd: Hwnd, pid: u32) -> io::Result<Value> {
    let _apartment = Apartment::new()?;
    let api = automation()?;
    let (nodes, truncated) = tree(hwnd, pid, &api)?;
    Ok(
        json!({"schema":1,"method":"UI Automation control view","hwnd":hwnd.to_string(),"pid":pid,"complete":!truncated,"truncated":truncated,"max_depth":8,"max_nodes":256,"values_read":false,"controls":nodes.into_iter().map(|(_,v)|v).collect::<Vec<_>>()}),
    )
}
pub(super) fn invoke(hwnd: Hwnd, pid: u32, selector: &str) -> io::Result<bool> {
    if selector.is_empty() || selector.len() > 256 {
        return Err(invalid("nonempty bounded AutomationId required"));
    }
    let _apartment = Apartment::new()?;
    let api = automation()?;
    let (nodes, truncated) = tree(hwnd, pid, &api)?;
    if truncated {
        return Err(invalid(
            "incomplete UIA tree; cannot establish selector uniqueness",
        ));
    }
    let matching: Vec<_> = nodes
        .into_iter()
        .filter(|(_, data)| data["automation_id"] == selector)
        .collect();
    if matching.len() != 1 {
        return Err(invalid("AutomationId absent or ambiguous"));
    }
    let (element, data) = &matching[0];
    if data["is_password"] != false || data["enabled"] != true {
        return Err(invalid(
            "selected control is unavailable or password-protected",
        ));
    }
    identity(hwnd, Some(pid))?;
    let pattern: IUIAutomationInvokePattern =
        unsafe { element.GetCurrentPatternAs(UIA_InvokePatternId) }.map_err(failure)?;
    unsafe { pattern.Invoke() }.map_err(failure)?;
    println!(
        "{}",
        json!({"schema":1,"method":"UIA InvokePattern","hwnd":hwnd.to_string(),"pid":pid,"automation_id":selector,"dispatched":true,"application_effect_verified":false})
    );
    Ok(true)
}
