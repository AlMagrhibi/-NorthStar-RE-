---
name: ghidra-native-analysis
description: Analyze authorized native binaries and firmware with Ghidra disassembly, decompilation and cross references; automate with Java and record evidence without executing unknown inputs.
---

# Ghidra: native binary and firmware analysis

**Mandatory shared gate:** Read `../environment-discovery-setup/SKILL.md`, `../../references/evidence-contract.md`, and `references/operational-reference.md` before work. Verify required tools, install only approved missing dependencies, and use actual target evidence rather than a fixed client or version.


Static reverse engineering of native code: disassembly, decompilation, cross references,
pseudo-C is an approximation, never the restored original source.

## Preconditions

- `reverse-engineering-preflight` returned `PASS` (or reviewed `NEEDS_REVIEW`) with
  `route: ghidra` or `both`. It verified the release's declared JDK floor
  (`application.java.min` in `Ghidra/application.properties`), a full JDK (`javac`, same
  `java` Ghidra will use), and the native decompiler binary.
- The JDK requirement changes between lines: JDK 21 for 12.0/12.1, JDK 25 for 12.2 per the
  official docs. Never universalize one number; trust the installed release's docs.
- Locate Ghidra via `GHIDRA_HOME` / `GHIDRA_INSTALL_DIR` / `--ghidra-dir`; verify `ghidraRun.bat`
  and `support/analyzeHeadless.bat` come from that distribution. If Ghidra is missing, stop and
  propose the official release plus supported JDK; no unofficial binaries or speculative installers.

## Safety

- **Static only.** Ghidra's Debugger drives native debuggers and can execute the target: never
  use it on an untrusted sample outside an isolated VM with defined egress.
- Untrusted sample: disposable VM, unprivileged user, network off or host-only. Never as Administrator.
- Do not run third-party scripts or extensions unread; they run with your privileges.
- Keep Ghidra current and verify the download's SHA-256 (`Get-FileHash -Algorithm SHA256`).
  Before upgrading, back up each project's `.rep` directory and `.gpr` file; projects saved
  by a newer release cannot be opened by an older one.
- Keep projects outside the sample directory: `analysis/ghidra/<sha12>/`. Do not patch
  anti-cheat/DRM or extract third-party secrets.

## Modes (from the install directory)

| Mode | Command | Use for |
|---|---|---|
| GUI | ` .\ghidraRun.bat` | exploration, renaming, type work |
| Headless | `support/analyzeHeadless.bat` | repeatable batch analysis, pre-analysis of big binaries |

Launch trouble: inspect the Windows launcher output and bundled documentation.

## Importing

**Known formats (ELF, PE, Mach-O):** let Ghidra choose loader, language and compiler spec,
then confirm they match what preflight reported (architecture, 32/64-bit, endianness).

**Raw firmware (no header):** nothing in the file tells Ghidra the architecture or load
address; you supply both and must justify them.

1. Pick the ARM little-endian **Cortex** language. Take the exact ID from
   `Ghidra/Processors/ARM/data/languages/ARM.ldefs` (the `id` attribute, matched exactly
   including case) rather than from memory.
2. Base address from the linker script or the part's reference manual. STM32 main flash
   commonly starts at `0x08000000`; confirm for the exact part.
3. The first words of a Cortex-M image are the initial SP and the reset handler (odd =
   Thumb). Preflight's `cortex_m_vector_table` hint checks exactly this.
4. After import, add memory blocks for SRAM and peripheral regions (Window > Memory Map),
   otherwise register accesses look like random pointers.

**Headless**, flags verified against the official `analyzeHeadlessREADME.md` (also shipped in
the install's `support/`; file type varies by release):

```powershell
New-Item -ItemType Directory -Force analysis/ghidra/case01,reports | Out-Null
& "$env:GHIDRA_HOME/support/analyzeHeadless.bat" analysis/ghidra/case01 case01 -import 'C:/samples/target.dll' -analysisTimeoutPerFile 1800 -log reports/ghidra-headless.log -scriptlog reports/ghidra-script.log
# Raw firmware: consult installed README for exact processor and loader options.
```

Notes from the README: `-cspec` needs `-processor`; `-readOnly` with `-import` means the
import is **not saved**; `-deleteProject` removes a project created in that run; `-noanalysis`
skips auto-analysis; the headless analyzer may refuse to run if the project is open in the GUI;
`-log` redirection works only when Log4J is used. The README itself is inconsistent on the base
address form (footnote: hex without `0x`, example: `0x1000`). After import, check the block
start in Memory Map. `PeLoader` also has `-loader-parseCliHeaders`. Full list and
`-preScript/-postScript/-scriptPath` details: `references/headless-cheatsheet.md`.

## Analysis loop

1. Run auto-analysis with defaults first; re-run specific analyzers only with a reason.
   Review warnings, unsupported language/compiler spec and timeouts, not only the exit code.
2. Orient: entry point, imports/exports, strings, then `main`-like functions.
3. Follow cross references (`X`) from interesting strings and imports into code.
4. Rename and retype as you learn (`L`, `T`); unnamed `FUN_xxxx` everywhere means lost work.
   Rename only supported functions and structs.
5. Label every interpretation `proven`, `likely` or `unknown` and attach the address and
   bytes. "Function at 0x08001234 writes 0x0F to a peripheral register" is evidence;
   "it initializes the clock" is an inference.
6. Record ABI, pointer sizes, packing/alignment, endianness, signedness, float encoding and
   type-recovery uncertainty. Optimization and inlining hide original control flow and symbols.
7. If preflight warned about entropy, look for an unpacking or decryption stage first and
   check per-section entropy in Ghidra.

## Client compatibility research

Read `client-compatibility-analysis`. Derive file formats, ABI, rendering and data transformations only from authorized artifacts and the actual project stack; never assume product, version, language or graphics API.
## Scripting

Use Java scripts via Ghidra Script Manager and analyzeHeadless. Validate on known fixtures; no interpreter bridge.

## Report: `reports/GHIDRA_ANALYSIS.md`

```
Sample: <name>  SHA-256: <hash>   Ghidra: <version>   JDK: <version>   Date: <date>
Import: loader, language, compiler spec, base address, and how each was chosen
Analysis settings, logs, timeouts
Facts (address-backed, proven): ...   Inferences (likely, with reasoning): ...   Unknown: ...
Function/asset cross-reference table with supporting bytes; unresolved symbols
Regression fixtures; recommended project changes (not applied by default)
```

## References

- https://github.com/NationalSecurityAgency/ghidra
- https://github.com/NationalSecurityAgency/ghidra/blob/master/GhidraDocs/GettingStarted.md
- https://github.com/NationalSecurityAgency/ghidra/blob/master/Ghidra/RuntimeScripts/support/analyzeHeadlessREADME.md

## V5 shared routing
Read `../../references/tool-selection.md` and the central tool catalog. Verify task-required tools using their upstream usage references and synthetic fixtures; route directory, archive, format, asset and authorized data-recovery tasks to the appropriate V5 specialists. No fixed client version.

## V6 integration
For authorized client/server work, read `../../references/server-workflow.md` and use the master routing for account, protocol, secure transport, game services, database and integration tasks. Preserve project-neutral discovery, tool verification and evidence boundaries.

## V7 shared workflow
Follow `../../references/v7-engineering-workflow.md`: environment readiness, project evidence, scoped plan, fixture verification, bounded repair and redacted knowledge. Report blocked capabilities and actual test status; no fixed client, language or version.

## Mandatory V8 documentation
Follow `../../references/v8-project-governance.md` and `engineering-documentation`: record material actions, results, redacted evidence, decisions, blockers and handoff in project Markdown. Do not overwrite existing documentation or log secrets.

## V10 shared engine contract
For large file inventories, format hints, evidence exports and hash comparisons when relevant to this specialist, use `../../references/v10-engineering-engine.md` after local acceptance. Do not substitute the engine for specialist analysis or bypass existing evidence and security gates.
