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

- UI Automation semantic tree, stable selectors, bounded read and standard control patterns; do not read password values.
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

`nsre desktop maximize HWND PID --apply` requests maximization and verifies window state. `nsre desktop inspect` includes DPI and client geometry. `nsre desktop click HWND PID X Y --apply` accepts client-relative coordinates only; the selected window must already be foreground, the hit point must belong to that window, and cursor position and owner are rechecked immediately before a single ordinary left click. It refuses covered points and does not bypass Windows input isolation. Exit 0 for click means input insertion, not a successful application action; independently observe the expected process or UI state. The short check-to-input race cannot be eliminated; use an idle local desktop and serialize GUI actions. The fixture now creates a START button and observes a title change when normal foreground access succeeds. No actual Windows 11 fixture result yet. UI Automation, window capture and full DPI-awareness remain unimplemented.
