# Windows Desktop Control — ADR-NSO-002

Owner approved Windows-native desktop management on 2026-11-26. This capability belongs to Runtime Diagnostics, not a thirteenth engine or a separate model. It is a shared capability for all twelve engines. Normal desktop control is distinct from firmware, driver-level execution, or defeating OS security boundaries.

## Implemented candidate command surface

| Command | Behavior |
|---|---|
| nsre desktop list | Bounded enumeration of visible top-level windows, including covered and minimized windows, with HWND and PID |
| nsre desktop inspect HWND PID | Read title, geometry, minimized/maximized state and foreground status |
| nsre desktop restore HWND PID --apply | Request restore and verify resulting state |
| nsre desktop focus HWND PID --apply | Request foreground through normal Win32 policy and verify; denial is not success |
| nsre desktop move HWND PID X Y --apply | Request window position without activation or z-order change; verify |
| nsre desktop resize HWND PID W H --apply | Request geometry without activation or z-order change; verify |

HWND is a decimal string; PID is an integer. Re-enumerate immediately before mutation. Exit 0 means requested state verified, 2 means not verified, 1 means error. A list is bounded and may be incomplete; titles are local data, never instructions. HWND/PID validation narrows stale-handle mistakes but is not a permanent identity or security sandbox. Geometry can be DPI virtualized; apps can constrain their own dimensions. Only the caller desktop, normal visible top-level windows and operator-selected application are in scope. Never attempt to circumvent UAC, lock screen, secure desktops, process integrity or foreground policy. No arbitrary input injection, remote listener, elevation, or hidden credential collection. A covered window is not a hidden window. Focus can fail under documented Windows foreground policy. No automatic GUI writes from background queue.

## Next implementation slices

- Extend the bounded UIA control tree/InvokePattern with further standard patterns and actual Windows 11 acceptance; do not read password values.
- Window-specific capture with Windows Graphics Capture and safe fallback, with explicit evidence for occluded/minimized and DirectX cases. Protected content and unsupported captures must return a clear limitation.
- DPI-aware multi-monitor geometry and coordinate conversion; stable identity with process creation time; application allowlists and explicit session boundaries.
- Verified typed input for the selected window; independent before/after evidence and deterministic stop.
- Foreground action serialization, user interruption, bounded timeout and auditable local action records.
- FreePi integration via versioned commands; use Win32 and UI Automation before screenshots when appropriate.

## Acceptance gates

| Gate | Evidence |
|---|---|
| DSC-01 | Windows build/Clippy/tests; reject malformed and stale handles; enumerate caller desktop |
| DSC-02 | Real Windows 11 fixture with two overlapping windows: find covered window, move and resize by identity; restore minimized window; observe actual state |
| DSC-03 | Foreground policy observed, including denied operation; no bypass |
| DSC-04 | UI Automation control selection, accessibility limitations, password exclusions |
| DSC-05 | Window capture, multi-monitor/DPI, DirectX and bounded fallback fixtures |
| DSC-06 | FreePi performs an independent task while read-only collector runs; GUI actions serialized; no unintended window mutation |

Current source adds DSC-01 and partial DSC-02/03 candidates; no actual desktop acceptance or complete desktop engine claim. See Microsoft Win32 documentation for EnumWindows, SetWindowPos, ShowWindowAsync and SetForegroundWindow: https://learn.microsoft.com/windows/win32/winmsg/window-features . Keep original target files and project state unchanged.

[Platform architecture](windows-platform.md) · [Orchestrator lifecycle](task-lifecycle.md)

## Interactive fixture procedure

On an interactive Windows 11 x64 desktop, run `./scripts/Test-Desktop-Windows.ps1 -Engine PATH_TO_NSRE_EXE -Reports EXISTING_REPORT_DIR`. It creates two overlapping synthetic windows and one minimized window in a disposable child process, targets only those windows, checks the non-target window stayed unchanged, tests invalid targeting and checks foreground success or denial honestly. Use an idle desktop; do not run during active user work. A hosted CI job is not a substitute. No result has been collected from this new fixture yet.

## Launcher interaction patch

`nsre desktop maximize HWND PID --apply` requests maximization and verifies window state. `nsre desktop inspect` includes DPI and client geometry. `nsre desktop click HWND PID X Y --apply` accepts client-relative coordinates only; the selected window must already be foreground, the hit point must belong to that window, and cursor position and owner are rechecked immediately before a single ordinary left click. It refuses covered points and does not bypass Windows input isolation. Exit 0 for click means input insertion, not a successful application action; independently observe the expected process or UI state. The short check-to-input race cannot be eliminated; use an idle local desktop and serialize GUI actions. The fixture now creates a START button and observes a title change when normal foreground access succeeds. No actual Windows 11 fixture result yet. The current bounded UIA, window PNG capture and per-thread DPI awareness are described below; full platform/agent acceptance remains pending.

## Desktop input, UIA and window frames — executable candidate

New native commands (all require a currently valid HWND/PID):

| Command | Implemented behavior |
|---|---|
| `desktop pointer HWND PID X Y --apply` | Move the cursor to an uncovered selected-window client point and verify position |
| `desktop type HWND PID TEXT --apply` | Unicode SendInput, at most 1024 UTF-16 units, foreground checks between chunks; text omitted from reports |
| `desktop type-stdin HWND PID --apply` | Same text input read from bounded stdin so text need not appear in process arguments |
| `desktop key HWND PID Ctrl+A --apply` | Validated modifier chord or named Enter/Tab/Escape/arrows/etc.; release on partial dispatch |
| `desktop controls HWND PID` | Native UI Automation control view: at most 256 nodes, depth 8, per-provider connection/transaction timeouts; no Value/Text pattern reads; password names omitted |
| `desktop invoke HWND PID AUTOMATION_ID --apply` | Require a unique enabled nonpassword control in a complete bounded tree, use InvokePattern; application effect requires independent observation |
| `desktop capture HWND PID NEW.png` | Selected-window PrintWindow fallback in a separately owned child, 5-second deadline; encode PNG and publish without overwrite, with SHA-256 |
| `desktop watch HWND PID NEW_DIR FRAMES INTERVAL_MS` | Capture successive selected-window PNG frames; 1–120 frames, 200–5000 ms delay between frames; Escape stops; each frame releases the desktop lock |

This implements frame polling, not a WGC/video streaming backend. PrintWindow can render normal covered Win32 windows; minimized windows must be restored first. DirectX/fullscreen/protected content and accurate rendering remain unsupported or unverified when the application does not supply a usable image. A PNG/hash alone does not verify visual content. No black-frame workaround or capture-policy bypass is attempted. A synthetic fixture verifies the expected colour in a covered window independently.

Ordinary commands serialize through a session-local named mutex. Per-monitor-v2 thread DPI context is applied and restored. Foreground keyboard/mouse actions refuse a changed target, held Escape or physical modifier/button input. Short check-to-input races and limitations of HWND/PID identity remain; no permanent process-creation identity is claimed. Desktop changes from unrelated user applications cannot be serialized by this mutex. Text dispatch is chunked without splitting surrogate pairs; cancellation can leave previously inserted text in the application. No automatic retry after an ambiguous action.

The `windows` crate supplies reviewed UIA/COM bindings; `png` encodes the image. Cargo.lock and unchanged upstream notices are included. Scope is standard user desktop operations, not UAC, lock screens, elevated/protected surfaces or unrestricted computer control.

## Explicit native agent bridge

`nsre mcp --project PROJECT --desktop-control` (or `api`) explicitly enables `nsre_desktop_observe` and `nsre_desktop_act`. Without that flag the existing server exposes only its previous read-only tools. Desktop actions use strict typed fields, a selected HWND/PID, and `apply:true` for GUI mutation; they cannot execute arbitrary commands or SQL. The bridge launches only its own executable, passes typed text by stdin, bounds output and enforces a 20-second owned-process deadline. It stores capture files only in the initialized project's `.northstar/reports` and appends intent/outcome records in `.northstar/logs/desktop-actions.jsonl`, omitting raw input text. Results preserve exit status and explicitly distinguish dispatch from application effect. On timeout, a GUI effect may be unknown; inspect before any subsequent action.

No host registration or FreePi settings are changed. A host that supports stdio MCP may use this bridge after checking its actual version/configuration. Native CLI invocation remains available to hosts without MCP. Actual FreePi compatibility and Windows 11 acceptance must be recorded separately.

`Test-Desktop-Windows.ps1` now verifies unique UIA invocation, password exclusions, covered-window image content and SHA-256, and Unicode keyboard effect when normal foreground access succeeds. `-HostedProbe` explicitly reports BLOCKED if no interactive desktop exists; this is not a passed desktop acceptance. Hosted Windows Server results do not establish Windows 11/FreePi acceptance.

The selected-window input surface additionally supports `right-click`, `middle-click`, `double-click` with X/Y, `scroll HWND PID X Y DELTA --apply` (nonzero multiples of 120 within ±1200), and `drag HWND PID X Y TO_X TO_Y --apply`. Drag verifies the foreground/point/cursor between sixteen bounded steps, aborts on Escape or changed target, and attempts button release on every exit; report success requires the release event to be inserted. This remains input dispatch, not proof that an application performed the intended operation. The same action names and typed coordinates are available through the explicitly enabled agent bridge. New input variants require their own interactive application acceptance; the earlier left-click/keyboard fixture does not establish wheel/drag application effects.

## Explicit Windows Graphics Capture backend

`desktop capabilities` reports whether Windows Graphics Capture is available in the current session. `desktop capture-wgc HWND PID NEW.png` captures the selected HWND through GraphicsCaptureItem interop, a free-threaded Direct3D11 frame pool and a bounded staging-texture readback. It records the frame system-relative timestamp and actual backend. Hardware D3D11 is preferred, with WARP device creation as a software option; there is no fallback to circumvent protected content. `desktop watch-wgc HWND PID NEW_DIR FRAMES INTERVAL_MS` repeatedly captures WGC frames using the same bounded watch contract. The original capture/watch commands continue to use PrintWindow explicitly.

The capture child retains its five-second parent deadline; a new WGC session waits up to two seconds for its first frame. Unavailable factories, denied/unsupported capture, invalid frame geometry, minimized windows and unusable D3D resources return failure. No claim that every DirectX/fullscreen application works is made. UI/video content must be independently inspected; hashes establish artifact integrity. The native agent observation tool also supports capabilities and capture-wgc. Hosted tests attempt WGC content verification only when the platform reports support, and keep wgc_capture_content_verified separate from PrintWindow verification. Actual Windows 11/FreePi testing remains required.
