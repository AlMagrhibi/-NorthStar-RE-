//! Local newline JSON-RPC over stdio; read-only project-bound tools. Legacy MCP 2025-11-25 candidate.
use serde_json::{json, Value};
use std::{
    io::{self, BufRead, Write},
    path::Path,
};
fn reply(id: &Value, result: Value) -> Value {
    json!({"jsonrpc":"2.0","id":id,"result":result})
}
fn error(id: &Value, code: i32, msg: &str) -> Value {
    json!({"jsonrpc":"2.0","id":id,"error":{"code":code,"message":msg}})
}
fn tools(desktop: bool) -> Value {
    let mut listed = json!({"tools":[{"name":"nsre_doctor","description":"Read-only environment discovery; paths are not verified installations","inputSchema":{"type":"object","properties":{},"additionalProperties":false},"annotations":{"readOnlyHint":true}},{"name":"nsre_status","description":"Read-only status for the explicitly bound project","inputSchema":{"type":"object","properties":{},"additionalProperties":false},"annotations":{"readOnlyHint":true}}]});
    listed["tools"].as_array_mut().unwrap().extend([
        json!({"name":"nsre_capabilities","description":"Embedded engine registry and queue contracts; incomplete capabilities remain explicit","inputSchema":{"type":"object","properties":{},"additionalProperties":false},"annotations":{"readOnlyHint":true}}),
        json!({"name":"nsre_queue_status","description":"Hash-verified job status in the bound project .northstar/state/jobs.sqlite; no caller-controlled database path","inputSchema":{"type":"object","properties":{"id":{"type":"string","minLength":1,"maxLength":64,"pattern":"^[A-Za-z0-9-]+$"}},"required":["id"],"additionalProperties":false},"annotations":{"readOnlyHint":true}})
    ]);
    if desktop {
        listed["tools"]
            .as_array_mut()
            .unwrap()
            .extend(crate::desktop_api::tools());
    }
    listed
}
#[cfg(test)]
fn handle(v: &Value, p: Option<&Path>, mcp: bool, phase: &mut u8) -> Option<Value> {
    handle_mode(v, p, mcp, phase, false)
}
fn handle_mode(
    v: &Value,
    p: Option<&Path>,
    mcp: bool,
    phase: &mut u8,
    desktop: bool,
) -> Option<Value> {
    let id = &v["id"];
    if v["jsonrpc"] != "2.0" || !v["method"].is_string() {
        return Some(error(id, -32600, "invalid request"));
    }
    let method = v["method"].as_str().unwrap();
    if !v.as_object().unwrap().contains_key("id") {
        if mcp && method == "notifications/initialized" && *phase == 1 {
            *phase = 2;
        }
        return None;
    }
    if !(id.is_string() || id.is_number()) {
        return Some(error(&Value::Null, -32600, "invalid ID"));
    }
    if mcp && method == "initialize" && *phase == 0 {
        if !v["params"]["protocolVersion"].is_string() {
            return Some(error(id, -32602, "protocol version required"));
        }
        *phase = 1;
        return Some(reply(
            id,
            json!({"protocolVersion":"2025-11-25","capabilities":{"tools":{}},"serverInfo":{"name":"northstar-re","version":env!("CARGO_PKG_VERSION")}}),
        ));
    }
    if method == "ping" {
        return Some(reply(id, json!({})));
    }
    if mcp && *phase != 2 {
        return Some(error(id, -32000, "initialize first"));
    }
    if mcp && method == "tools/list" {
        return Some(reply(id, tools(desktop)));
    }
    let name = if mcp && method == "tools/call" {
        v["params"]["name"].as_str().unwrap_or("")
    } else if !mcp {
        method
    } else {
        return Some(error(id, -32601, "unknown method"));
    };
    let arguments = if mcp {
        &v["params"]["arguments"]
    } else {
        &v["params"]
    };
    if matches!(name, "nsre_desktop_observe" | "nsre_desktop_act") {
        if !desktop {
            return Some(error(id, -32601, "desktop tools not enabled"));
        }
        let result = p
            .ok_or_else(|| "project not bound".to_owned())
            .and_then(|p| crate::desktop_api::execute(p, arguments, name == "nsre_desktop_act"));
        return Some(match result {
            Ok(mut data) => {
                if mcp {
                    let image = data
                        .as_object_mut()
                        .and_then(|object| object.remove("_image"));
                    let mut content = vec![json!({"type":"text","text":data.to_string()})];
                    if let Some(image) = image {
                        content.push(image);
                    }
                    reply(
                        id,
                        json!({"content":content,"isError":data["command_succeeded"]!=true}),
                    )
                } else {
                    reply(id, data)
                }
            }
            Err(message) => {
                if mcp {
                    reply(
                        id,
                        json!({"content":[{"type":"text","text":message}],"isError":true}),
                    )
                } else {
                    error(id, -32000, &message)
                }
            }
        });
    }
    let queue_status = matches!(name, "queue-status" | "nsre_queue_status");
    let job_id = if queue_status {
        let Some(object) = arguments.as_object() else {
            return Some(error(id, -32602, "job id required"));
        };
        let Some(job) = object.get("id").and_then(Value::as_str).filter(|s| {
            !s.is_empty()
                && s.len() <= 64
                && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
        }) else {
            return Some(error(id, -32602, "invalid job id"));
        };
        if object.len() != 1 {
            return Some(error(id, -32602, "unexpected arguments"));
        }
        Some(job)
    } else {
        if !arguments.is_null() && arguments.as_object().is_none_or(|a| !a.is_empty()) {
            return Some(error(id, -32602, "arguments not supported"));
        }
        None
    };
    let result = match name {
        "doctor" | "nsre_doctor" => Ok(crate::framework::doctor()),
        "capabilities" | "nsre_capabilities" => Ok(json!({
            "schema":1,"producer":{"name":"nsre","version":env!("CARGO_PKG_VERSION")},
            "engines":serde_json::from_str::<Value>(include_str!("engine-registry.json")).expect("embedded registry"),
            "queue":{"database_schemas":[1,2],"job_input_schemas":[1,2],"read_status_tool":true,
                "mcp_submit":false,"automatic_recovery":false,"gui_mutations":false},
            "desktop_bridge":{"available":cfg!(windows),"enabled":desktop,"gui_actions_in_queue":false},
            "agent_acceptance":"unverified"
        })),
        "queue-status" | "nsre_queue_status" => p
            .ok_or_else(|| "project not bound".to_owned())
            .and_then(|p| {
                crate::framework::rpc_queue_status(p, job_id.unwrap()).map_err(|e| e.to_string())
            }),
        "status" | "nsre_status" => p
            .ok_or_else(|| "project not bound".to_owned())
            .and_then(|p| crate::framework::rpc_status(p).map_err(|e| e.to_string())),
        _ => return Some(error(id, -32601, "unknown tool")),
    };
    match result {
        Ok(data) => Some(reply(
            id,
            if mcp {
                json!({"content":[{"type":"text","text":data.to_string()}],"isError":false})
            } else {
                data
            },
        )),
        Err(e) => Some(if mcp {
            reply(
                id,
                json!({"content":[{"type":"text","text":e}],"isError":true}),
            )
        } else {
            error(id, -32000, &e)
        }),
    }
}
pub fn serve_mode(
    p: Option<&Path>,
    mcp: bool,
    desktop: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut input = io::stdin().lock();
    let mut out = io::stdout().lock();
    let mut phase = 0;
    loop {
        let mut line = Vec::new();
        let n = std::io::Read::take(&mut input, 1024 * 1024 + 1).read_until(b'\n', &mut line)?;
        if n == 0 {
            break;
        }
        if n > 1024 * 1024 {
            return Err(io::Error::other("request exceeds 1 MiB").into());
        }
        let response = match serde_json::from_slice::<Value>(&line) {
            Ok(v) => handle_mode(&v, p, mcp, &mut phase, desktop),
            Err(_) => Some(error(&Value::Null, -32700, "parse error")),
        };
        if let Some(v) = response {
            serde_json::to_writer(&mut out, &v)?;
            out.write_all(b"\n")?;
            out.flush()?;
        }
    }
    Ok(true)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn desktop_tools_require_explicit_session_opt_in() {
        assert_eq!(tools(false)["tools"].as_array().unwrap().len(), 4);
        assert_eq!(tools(true)["tools"].as_array().unwrap().len(), 6);
        let mut phase = 2;
        let request = json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"nsre_desktop_act","arguments":{"action":"click","hwnd":"1","pid":1,"apply":true,"x":0,"y":0}}});
        assert_eq!(
            handle_mode(&request, None, true, &mut phase, false).unwrap()["error"]["code"],
            -32601
        );
    }

    #[test]
    fn handshake() {
        let mut p = 0;
        assert!(handle(
            &json!({"jsonrpc":"2.0","id":1,"method":"tools/list"}),
            None,
            true,
            &mut p
        )
        .unwrap()["error"]
            .is_object());
        let v=handle(&json!({"jsonrpc":"2.0","id":2,"method":"initialize","params":{"protocolVersion":"2025-11-25"}}),None,true,&mut p).unwrap();
        assert_eq!(v["result"]["protocolVersion"], "2025-11-25");
        assert!(handle(
            &json!({"jsonrpc":"2.0","method":"notifications/initialized"}),
            None,
            true,
            &mut p
        )
        .is_none());
        assert!(handle(
            &json!({"jsonrpc":"2.0","id":3,"method":"tools/list"}),
            None,
            true,
            &mut p
        )
        .unwrap()["result"]["tools"]
            .is_array());
    }
}
