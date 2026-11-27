//! Explicitly enabled desktop bridge. Typed actions only; never an arbitrary command runner.
use serde_json::{json, Value};
use std::{
    ffi::OsString,
    io::{self, Read, Write},
    path::Path,
    process::{Command, Stdio},
    time::{Duration, Instant},
};
fn invalid(s: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, s)
}
pub(crate) fn tools() -> Vec<Value> {
    vec![
        json!({"name":"nsre_desktop_observe","description":"Explicit desktop session: list windows, inspect one window, read bounded UIA controls or save a PNG under the bound project reports. No password values are read.","inputSchema":{"type":"object","properties":{"action":{"type":"string","enum":["list","capabilities","inspect","controls","capture","capture-wgc"]},"hwnd":{"type":"string","pattern":"^[0-9]+$"},"pid":{"type":"integer","minimum":1}},"required":["action"],"additionalProperties":false},"annotations":{"readOnlyHint":false}}),
        json!({"name":"nsre_desktop_act","description":"Explicit selected-window action. Requires apply=true. Input must remain foreground; UIA Invoke uses a unique AutomationId. Insertion/Invoke is not independent proof of application effect.","inputSchema":{"type":"object","properties":{"action":{"type":"string","enum":["restore","maximize","focus","move","resize","click","pointer","right-click","middle-click","double-click","drag","scroll","key","type","invoke"]},"hwnd":{"type":"string","pattern":"^[0-9]+$"},"pid":{"type":"integer","minimum":1},"apply":{"const":true},"x":{"type":"integer"},"y":{"type":"integer"},"width":{"type":"integer"},"height":{"type":"integer"},"text":{"type":"string","maxLength":1024},"key":{"type":"string","maxLength":64},"to_x":{"type":"integer"},"to_y":{"type":"integer"},"delta":{"type":"integer"},"automation_id":{"type":"string","maxLength":256}},"required":["action","hwnd","pid","apply"],"additionalProperties":false},"annotations":{"readOnlyHint":false,"destructiveHint":true}}),
    ]
}
fn prepare(
    value: &Value,
    mutating: bool,
    capture_path: &Path,
) -> io::Result<(Vec<OsString>, Option<String>)> {
    let object = value
        .as_object()
        .ok_or_else(|| invalid("desktop arguments required"))?;
    let action = value["action"]
        .as_str()
        .ok_or_else(|| invalid("desktop action required"))?;
    let fields: &[&str] = match (mutating, action) {
        (false, "list" | "capabilities") => &["action"],
        (false, "inspect" | "controls" | "capture" | "capture-wgc") => &["action", "hwnd", "pid"],
        (true, "restore" | "maximize" | "focus") => &["action", "hwnd", "pid", "apply"],
        (true, "move" | "click" | "pointer" | "right-click" | "middle-click" | "double-click") => {
            &["action", "hwnd", "pid", "apply", "x", "y"]
        }
        (true, "drag") => &["action", "hwnd", "pid", "apply", "x", "y", "to_x", "to_y"],
        (true, "scroll") => &["action", "hwnd", "pid", "apply", "x", "y", "delta"],
        (true, "resize") => &["action", "hwnd", "pid", "apply", "width", "height"],
        (true, "key") => &["action", "hwnd", "pid", "apply", "key"],
        (true, "type") => &["action", "hwnd", "pid", "apply", "text"],
        (true, "invoke") => &["action", "hwnd", "pid", "apply", "automation_id"],
        _ => return Err(invalid("desktop action is not allowed in this tool")),
    };
    if object.len() != fields.len() || object.keys().any(|k| !fields.contains(&k.as_str())) {
        return Err(invalid("missing or unexpected desktop argument"));
    }
    if mutating && value["apply"] != true {
        return Err(invalid("desktop mutation requires apply=true"));
    }
    let mut args = vec![
        OsString::from("desktop"),
        OsString::from(if action == "type" {
            "type-stdin"
        } else {
            action
        }),
    ];
    if !matches!(action, "list" | "capabilities") {
        let hwnd = value["hwnd"]
            .as_str()
            .filter(|s| !s.is_empty() && s.len() <= 20 && s.bytes().all(|b| b.is_ascii_digit()))
            .and_then(|s| s.parse::<isize>().ok())
            .filter(|v| *v > 0)
            .ok_or_else(|| invalid("invalid HWND"))?;
        let pid = value["pid"]
            .as_u64()
            .filter(|v| *v > 0 && *v <= u32::MAX as u64)
            .ok_or_else(|| invalid("invalid PID"))?;
        args.extend([hwnd.to_string().into(), pid.to_string().into()]);
    }
    let mut stdin = None;
    for field in fields.iter().skip(4) {
        match *field {
            "x" | "y" | "width" | "height" | "to_x" | "to_y" | "delta" => {
                let n = value[field]
                    .as_i64()
                    .filter(|v| i32::try_from(*v).is_ok())
                    .ok_or_else(|| invalid("invalid desktop coordinate"))?;
                args.push(n.to_string().into());
            }
            "text" => {
                let text = value[field]
                    .as_str()
                    .filter(|s| {
                        !s.is_empty()
                            && !s.contains('\0')
                            && s.len() <= 4096
                            && s.encode_utf16().count() <= 1024
                    })
                    .ok_or_else(|| invalid("invalid bounded text"))?;
                stdin = Some(text.to_owned());
            }
            "key" | "automation_id" => {
                let text = value[field]
                    .as_str()
                    .filter(|s| !s.is_empty() && s.len() <= 256 && !s.contains('\0'))
                    .ok_or_else(|| invalid("invalid desktop selector/key"))?;
                args.push(text.into());
            }
            _ => {}
        }
    }
    if matches!(action, "capture" | "capture-wgc") {
        args.push(capture_path.as_os_str().to_owned());
    }
    if mutating {
        args.push("--apply".into());
    }
    Ok((args, stdin))
}
fn append(path: &Path, event: &Value) -> io::Result<()> {
    let meta = std::fs::symlink_metadata(path);
    if meta
        .as_ref()
        .is_ok_and(|m| crate::unsafe_entry(m) || !m.is_file())
    {
        return Err(invalid("unsafe desktop audit file"));
    }
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)?;
    let mut bytes = serde_json::to_vec(event).map_err(io::Error::other)?;
    bytes.push(b'\n');
    file.write_all(&bytes)?;
    file.sync_all()
}
fn image_base64(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let a = chunk[0];
        let b = chunk.get(1).copied().unwrap_or(0);
        let c = chunk.get(2).copied().unwrap_or(0);
        out.push(ALPHABET[(a >> 2) as usize] as char);
        out.push(ALPHABET[(((a & 3) << 4) | (b >> 4)) as usize] as char);
        out.push(if chunk.len() > 1 {
            ALPHABET[(((b & 15) << 2) | (c >> 6)) as usize] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            ALPHABET[(c & 63) as usize] as char
        } else {
            '='
        });
    }
    out
}
pub(crate) fn execute(project: &Path, value: &Value, mutating: bool) -> Result<Value, String> {
    (||->Result<Value,Box<dyn std::error::Error>>{
  let(report_dir,audit)=crate::framework::rpc_desktop_paths(project)?;
  let stamp=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_nanos();
  let capture_path=report_dir.join(format!("desktop-{}-{stamp}.png",std::process::id()));
  let(args,stdin)=prepare(value,mutating,&capture_path)?;
  if !cfg!(windows){return Err(invalid("native desktop bridge requires Windows").into());}
  // Session-aware routing (WISM-01/02/11): a service or SSH session has no interactive
  // desktop, so direct execution is refused with the routing decision instead of acting
  // on a hidden desktop or silently doing nothing.
  if let Ok(context)=crate::session::context() {
    if context["direct_desktop_execution_permitted"]!=true {
      return Err(invalid(&format!(
        "SESSION_ROUTING_REFUSED: this process runs in {} (session {}, lock state {}); the interactive desktop is session {:?}. Run the authorized interactive worker inside that session; do not bypass Windows session boundaries",
        context["classification"], context["session_id"], context["own_session_lock_state"], context["active_console_session"]
      )).into());
    }
  }
  let event=json!({"schema":1,"id":stamp.to_string(),"action":value["action"],"hwnd":value["hwnd"],"pid":value["pid"],"mutating":mutating,"text_logged":false});
  let mut intent=event.clone();intent["state"]=json!("intent");append(&audit,&intent)?;
  let mut child=Command::new(std::env::current_exe()?).args(args).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::null()).spawn()?;
  let stdout=child.stdout.take().ok_or_else(||invalid("missing bridge stdout"))?;
  let reader=std::thread::spawn(move||{let mut bytes=Vec::new();stdout.take(1024*1024+1).read_to_end(&mut bytes).map(|_|bytes)});
  if let Some(text)=stdin {if let Err(e)=child.stdin.take().ok_or_else(||invalid("missing stdin"))?.write_all(text.as_bytes()){let _=child.kill();let _=child.wait();let _=reader.join();return Err(e.into());}}
  else{drop(child.stdin.take());}
  let start=Instant::now();let mut timeout=false;
  let status=loop{match child.try_wait(){Ok(Some(s))=>break s,Ok(None)=>{},Err(e)=>{let _=child.kill();let _=child.wait();let _=reader.join();return Err(e.into());}}
   if start.elapsed()>Duration::from_secs(20){timeout=true;let _=child.kill();break child.wait()?;}std::thread::sleep(Duration::from_millis(20));};
  let bytes=reader.join().map_err(|_|invalid("desktop response reader failed"))??;
  let result=if bytes.len()<=1024*1024 {serde_json::from_slice::<Value>(&bytes).ok()}else{None};
  let mut outcome=event;outcome["state"]=json!(if timeout{"timeout"}else if status.success(){"returned"}else{"failed"});outcome["exit_code"]=json!(status.code());
  if let Some(ref result)=result {use sha2::{Digest,Sha256};outcome["result_sha256"]=json!(format!("{:x}",Sha256::digest(serde_json::to_vec(result)?)));}
  append(&audit,&outcome)?;
  if timeout{return Err(invalid("owned desktop action timed out; effect may be unknown; do not automatically retry").into());}
  let result=result.ok_or_else(||invalid("desktop command returned no valid bounded report"))?;
  let mut envelope=json!({"schema":1,"exit_code":status.code(),"command_succeeded":status.success(),"result":result,"audit_id":stamp.to_string()});
        if status.success() && matches!(value["action"].as_str(),Some("capture"|"capture-wgc")) {
            let metadata=std::fs::symlink_metadata(&capture_path)?;
            if !metadata.is_file()||crate::unsafe_entry(&metadata){return Err(invalid("unsafe captured image").into());}
            if metadata.len()<=8*1024*1024 {
                let mut bytes=Vec::new();std::fs::File::open(&capture_path)?.take(8*1024*1024+1).read_to_end(&mut bytes)?;
                use sha2::{Digest,Sha256};
                if bytes.len()>8*1024*1024||!bytes.starts_with(&[137,80,78,71,13,10,26,10])||envelope["result"]["sha256"]!=format!("{:x}",Sha256::digest(&bytes)){return Err(invalid("captured PNG changed before image delivery").into());}
                envelope["_image"]=json!({"type":"image","mimeType":"image/png","data":image_base64(&bytes)});
                envelope["image_included"]=json!(true);
            } else {envelope["image_included"]=json!(false);envelope["image_omitted_reason"]=json!("PNG exceeds 8 MiB inline limit; artifact retained locally");}
        }
        Ok(envelope)
 })().map_err(|e|e.to_string())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn base64_matches_rfc4648_vectors() {
        for (input, expected) in [
            ("", ""),
            ("f", "Zg=="),
            ("fo", "Zm8="),
            ("foo", "Zm9v"),
            ("foob", "Zm9vYg=="),
            ("fooba", "Zm9vYmE="),
            ("foobar", "Zm9vYmFy"),
        ] {
            assert_eq!(image_base64(input.as_bytes()), expected);
        }
    }
    #[test]
    fn typed_bridge_rejects_shell_paths_and_missing_apply() {
        let path = Path::new("frame.png");
        for v in [
            json!({"action":"shell","hwnd":"1","pid":1,"apply":true}),
            json!({"action":"click","hwnd":"1","pid":1,"apply":false,"x":1,"y":1}),
            json!({"action":"capture","hwnd":"1","pid":1,"path":"outside"}),
            json!({"action":"list","pid":1}),
        ] {
            assert!(prepare(&v, true, path).is_err());
        }
        let (args, text) = prepare(
            &json!({"action":"type","hwnd":"12","pid":3,"apply":true,"text":"مرحبا 😀"}),
            true,
            path,
        )
        .unwrap();
        assert!(!args.iter().any(|s| s.to_string_lossy().contains("مرحبا")));
        assert_eq!(text.as_deref(), Some("مرحبا 😀"));
        assert!(prepare(
            &json!({"action":"capture","hwnd":"12","pid":3}),
            false,
            path
        )
        .is_ok());
        assert!(prepare(&json!({"action":"list"}), false, path).is_ok());
    }
}
