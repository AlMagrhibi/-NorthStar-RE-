//! Dynamic execution context, session mobility and capability reassessment.
//!
//! The framework must behave the same whether the caller is local, on SSH, or on RDP,
//! from inside the home network or outside it. Nothing here may assume a fixed session
//! id, PID, username, IP or display resolution: every value is discovered from the
//! operating system and re-evaluated whenever the connection or session state changes.
//!
//! The context identity is a content hash of the stable execution facts, so a reconnect
//! that lands on a different session produces a different identity and invalidates stale
//! worker bindings, while a reconnect that resumes the same session keeps them.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::io;

fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message)
}

/// A stable, content-addressed identity of the current execution context.
/// No raw session id, PID, username or address is used as the key by itself; they are
/// only evidence inputs hashed into the identity.
pub struct ContextIdentity {
    pub identity_hash: String,
    pub session_id: u32,
    pub classification: String,
    pub lock_state: String,
    pub interactive: bool,
    pub evaluated_utc_ms: u64,
}

fn utc_millis() -> io::Result<u64> {
    let duration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(io::Error::other)?;
    u64::try_from(duration.as_millis()).map_err(io::Error::other)
}

/// Discover and hash the current execution context. Called on every reassessment.
pub fn evaluate() -> io::Result<ContextIdentity> {
    #[cfg(windows)]
    {
        let context = crate::session::context()?;
        let session_id = context["session_id"].as_u64().unwrap_or(0) as u32;
        let classification = context["classification"]
            .as_str()
            .unwrap_or("unknown")
            .to_owned();
        let lock_state = context["own_session_lock_state"]
            .as_str()
            .unwrap_or("unavailable")
            .to_owned();
        let interactive = classification == "active-console" && lock_state == "unlocked";
        // The identity covers the facts that change execution semantics. Deliberately
        // excluded: PID, username, IP addresses and display resolution, so a reconnect
        // from a different address or a new process is not mistaken for a new context.
        let material = format!(
            "session={session_id}|class={classification}|lock={lock_state}|host-class={}",
            host_class()?
        );
        let identity_hash = format!("{:x}", Sha256::digest(material.as_bytes()));
        Ok(ContextIdentity {
            identity_hash,
            session_id,
            classification,
            lock_state,
            interactive,
            evaluated_utc_ms: utc_millis()?,
        })
    }
    #[cfg(not(windows))]
    {
        Err(invalid("dynamic context requires Windows"))
    }
}

/// The host class distinguishes a workstation from a server so Windows Server CI is
/// never mistaken for the Windows 11 reference environment (ADD OS-detection note).
fn host_class() -> io::Result<String> {
    #[cfg(windows)]
    {
        let system = std::env::var("OS").unwrap_or_default();
        let product = win_product_type()?;
        Ok(format!("{system}/{product}"))
    }
    #[cfg(not(windows))]
    {
        Ok("unsupported".to_owned())
    }
}

#[cfg(windows)]
fn win_product_type() -> io::Result<String> {
    // OS_PRODUCT_TYPE: 1 = workstation, 2 = domain controller, 3 = server. This keeps a
    // Windows Server CI runner from being reported as the Windows 11 reference machine.
    use windows::Win32::System::SystemInformation::{GetProductInfo, OS_PRODUCT_TYPE};
    let mut product = OS_PRODUCT_TYPE(0);
    let ok = unsafe { GetProductInfo(10, 0, 0, 0, &mut product) };
    if !ok.as_bool() {
        return Err(io::Error::other("GetProductInfo failed"));
    }
    Ok(match product.0 {
        1 => "workstation",
        2 => "domain-controller",
        3 => "server",
        other => return Ok(format!("product-type-{other}")),
    }
    .to_owned())
}

#[cfg(not(windows))]
fn win_product_type() -> io::Result<String> {
    Ok("unsupported".to_owned())
}

/// The capability envelope for the current context. Non-interactive contexts keep the
/// queue, batch, collection and supervision capabilities; desktop verbs appear only in
/// an unlocked interactive console session.
pub fn capabilities() -> io::Result<Value> {
    let identity = evaluate()?;
    let desktop = identity.interactive;
    Ok(json!({
        "schema":1,
        "context_identity":identity.identity_hash,
        "session_id":identity.session_id,
        "classification":identity.classification,
        "lock_state":identity.lock_state,
        "evaluated_utc_ms":identity.evaluated_utc_ms,
        "capabilities":{
            "queue":true,"batch":true,"collect":true,"supervisor":true,
            "desktop_observe":desktop,"desktop_act":desktop,
            "interactive_worker_required":!desktop
        },
        "routing":{
            "direct_desktop":desktop,
            "requires_authorized_interactive_worker":!desktop,
            "non_interactive_tasks_run_here":true
        }
    }))
}

/// Compare two context evaluations and classify the transition. Reconnection, unlock
/// and lock all flow through here, and the caller acts on the delta without ever
/// re-running an effectful action whose outcome is unknown.
pub fn compare(previous: &str) -> io::Result<Value> {
    let current = evaluate()?;
    let changed = current.identity_hash != previous;
    Ok(json!({
        "schema":1,
        "previous_identity":previous,
        "current_identity":current.identity_hash,
        "changed":changed,
        "session_id":current.session_id,
        "classification":current.classification,
        "lock_state":current.lock_state,
        "interactive":current.interactive,
        "policy":{
            "non_interactive_tasks":"continue across reconnects; ownership lives in the durable queue",
            "effectful_tasks":"never re-executed automatically after an identity change",
            "stale_bindings":"invalidated when the identity changes"
        }
    }))
}

pub fn command(args: impl Iterator<Item = std::ffi::OsString>) -> io::Result<bool> {
    let args: Vec<_> = args.collect();
    let usage = "context evaluate | context capabilities | context compare PREVIOUS_HASH";
    if args.is_empty() {
        return Err(invalid(usage));
    }
    match args[0].to_str() {
        Some("evaluate") => {
            let identity = evaluate()?;
            println!(
                "{}",
                json!({
                    "schema":1,"identity_hash":identity.identity_hash,
                    "session_id":identity.session_id,"classification":identity.classification,
                    "lock_state":identity.lock_state,"interactive":identity.interactive,
                    "evaluated_utc_ms":identity.evaluated_utc_ms
                })
            );
        }
        Some("capabilities") => println!("{}", capabilities()?),
        Some("compare") if args.len() == 2 => {
            let previous = args[1]
                .to_str()
                .ok_or_else(|| invalid("previous identity hash required"))?;
            println!("{}", compare(previous)?);
        }
        _ => return Err(invalid(usage)),
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(windows)]
    #[test]
    fn identity_is_stable_within_the_same_context() {
        let first = evaluate().unwrap();
        let second = evaluate().unwrap();
        assert_eq!(first.identity_hash, second.identity_hash);
        assert_eq!(first.session_id, second.session_id);
    }

    #[cfg(windows)]
    #[test]
    fn identity_changes_only_with_execution_semantics() {
        // The identity must not depend on volatile inputs. Re-evaluate twice and compare
        // against a hand-built identity from the same facts.
        let first = evaluate().unwrap();
        let material = format!(
            "session={}|class={}|lock={}|host-class={}",
            first.session_id,
            first.classification,
            first.lock_state,
            host_class().unwrap()
        );
        let expected = format!("{:x}", Sha256::digest(material.as_bytes()));
        assert_eq!(first.identity_hash, expected);
    }

    #[cfg(windows)]
    #[test]
    fn capabilities_never_offer_a_desktop_outside_an_interactive_session() {
        let report = capabilities().unwrap();
        let desktop = report["capabilities"]["desktop_act"].as_bool().unwrap();
        let interactive = report["routing"]["direct_desktop"].as_bool().unwrap();
        assert_eq!(desktop, interactive);
        if report["classification"].as_str().unwrap() != "active-console" {
            assert!(!desktop);
            assert_eq!(
                report["routing"]["requires_authorized_interactive_worker"],
                true
            );
        }
        // Non-interactive capabilities are always present, whatever the context.
        assert_eq!(report["capabilities"]["queue"], true);
        assert_eq!(report["capabilities"]["supervisor"], true);
    }

    #[cfg(windows)]
    #[test]
    fn compare_reports_unchanged_for_the_same_context() {
        let first = evaluate().unwrap();
        let report = compare(&first.identity_hash).unwrap();
        assert_eq!(report["changed"], false);
        assert_eq!(report["current_identity"], first.identity_hash);
    }

    #[cfg(windows)]
    #[test]
    fn compare_flags_a_changed_context() {
        let report = compare("stale-identity-from-a-previous-session").unwrap();
        assert_eq!(report["changed"], true);
        assert_eq!(
            report["policy"]["effectful_tasks"],
            "never re-executed automatically after an identity change"
        );
    }
}
