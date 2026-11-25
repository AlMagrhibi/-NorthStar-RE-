---
name: ilspy-dotnet-decompilation
description: Analyze authorized managed .NET assemblies (C#, VB, F#) with ilspycmd, the official command-line ILSpy decompiler, on Windows 11. Covers triage of types and resources, targeted decompilation of one type or member, IL view, full-project export, single-file bundles, obfuscation signals, and evidence-labelled reporting. Use after reverse-engineering-preflight routes a file to ilspy, and whenever the user mentions ILSpy, ilspycmd, decompiling a .NET DLL or EXE, recovering C# from an assembly, reading IL, Unity Assembly-CSharp.dll, or legacy .NET Framework game clients, even if they do not name the tool.
---

# ILSpy: managed .NET assembly analysis

**Mandatory shared gate:** Read `../environment-discovery-setup/SKILL.md`, `../../references/evidence-contract.md`, and `references/operational-reference.md` before work. Verify required tools, install only approved missing dependencies, and use actual target evidence rather than a fixed client or version.


`ilspycmd` reads an assembly and prints or writes decompiled C#. It never runs the
assembly. Decompiled output is a reconstruction: names, comments and some control flow
are lost or rewritten, and an exported project is not the original repository.

This skill focuses on the CLI; ILSpy GUI is optional. Verify installed version locally.

## Preconditions

- `reverse-engineering-preflight` returned `PASS` (or reviewed `NEEDS_REVIEW`) with
  `route: ilspy` or `both`, and `managed_metadata_verified: true`. Native PE
  (`managed_dotnet: false`): stop and use `ghidra-native-analysis`.
- Authorization is recorded. Untrusted sample: disposable VM.
- Record the exact tool build (`ilspycmd --disable-updatecheck -v`). Requirements per the
  NuGet page for 11.2: .NET SDK 10.0.100+ to install or update; Verify the installed Windows build and runtime by launching the tool itself. Do not assume
  one runtime fits every release.
- If the CLI is absent, **propose, do not run**: `dotnet tool install --global ilspycmd`
  (then confirm `~/.dotnet/tools` is on PATH).
- **Always pass `--disable-updatecheck`**: otherwise `ilspycmd` makes a network call on start.

## Workspace

Output goes to `analysis/managed/<sha12>/` and `reports/`, never next to the sample. Keep the
original read-only in an isolated workspace. **Do not build or run the decompiled project**
unless the user explicitly wants that inside an isolated VM; a compilable project is one
`dotnet run` from executing hostile code.

## Workflow: narrow first, expand when needed

```powershell
ilspycmd --disable-updatecheck -l c Client.dll                 # classes; also i, s, d, e = interface, struct, delegate, enum
ilspycmd --disable-updatecheck --list-resources Client.dll
ilspycmd --disable-updatecheck -t Namespace.Type Client.dll     # one type to stdout
ilspycmd --disable-updatecheck -m "M:Namespace.Type.Method(System.String)" Client.dll   # doc-id or metadata token
ilspycmd --disable-updatecheck -il -t Namespace.Type Client.dll # IL when C# is wrong, empty or obfuscated
ilspycmd --disable-updatecheck -p -o analysis/managed/<sha12>/out --nested-directories Client.dll   # full project; -p needs -o
ilspycmd --disable-updatecheck --dump-table TypeDef --json Client.dll   # metadata tables
ilspycmd --disable-updatecheck --resource <name> -o analysis/managed/<sha12>/res Client.dll
```

| Need | Option |
|---|---|
| Dependencies elsewhere | `-r <dir>` |
| Keep going past failing methods | `--ignore-decompilation-errors` (errors stay in the output) |
| Language level | `-lv CSharp12_0` ... `Latest` |
| Decompiler settings | `-ds <setting>` (repeatable) or `--ilspy-settingsfile <path>` |
| Single-file bundle / package | run once to see entries, then `--bundle-entry <name>`; `-d -o <dir>` dumps the package |
| Debug info | `--il-sequence-points`, `--dump-pdb`, `-usepdb` |

Flags come from the `ilspycmd 11.2.0.10050` help on NuGet. **If a flag is rejected, trust
`ilspycmd --help` over this table.**

## Interpreting results

- Preserve assembly identity, references, resource manifests, target framework indicators,
  namespaces and type names. Separate compiler-generated artifacts (async state machines,
  closures, `<>c__DisplayClass`) from application logic.
- Obfuscation signals: single-character or non-printable identifiers, huge `switch` state
  machines, strings fetched via a decrypt helper, methods that decompile to errors. Switch
  to IL and say plainly the code is obfuscated. Deobfuscation is a separate decision.
- Interop points (`DllImport`/P/Invoke, COM, Win32, WinForms/WPF, DirectX calls) are porting
  targets, not mechanically transferable to Go.
- Compare C# with IL when behavior is ambiguous; use signatures and cross references, not guesses.
- Do not execute target-defined code to inspect resources.

## Diagnostics

- `ilspycmd: command not found`: check the tool is installed, PATH, SDK versus runtime. Do not reinstall blindly.
- Missing references: list unresolved assemblies; never claim a complete rebuild.
- **ReadyToRun**: Microsoft documents that R2R binaries contain both IL and the native version
  of the code, so ILSpy still works; Ghidra is optional, only for the precompiled native code.
- **Mixed-mode (C++/CLI)**: ILSpy sees only the IL; native parts need Ghidra. C++/CLI binaries
  are not eligible for ReadyToRun.
- **NativeAOT**: no IL to decompile; use Ghidra and do not promise C#.
- An exported project that does not compile does not mean decompilation failed: record missing
  NuGet packages, resources and generated build assumptions.

## Client compatibility research

Read `client-compatibility-analysis`. Trace observed resource resolution, file readers, interop, data formats and error handling. Derive every offset from evidence; do not assume a client version or target language.
## Evidence labels and report

Label each interpretation `proven` (directly visible in decompiled output or IL), `likely`
(reasoned, say how), or `unknown`. Deliver `reports/ILSPY_ANALYSIS.md`:

```
Sample: <name>  SHA-256: <hash>   ilspycmd: <version>   Date: <date>
Evidence of managed IL: <preflight fields>
Commands run: <exact commands>
Inventory: types, resources, exports; interop points
Facts (proven): ...   Inferences (likely, with reasoning): ...   Unknown / other tool needed: ...
Limitations and reproducibility notes; validation against the running client where permitted
```

Quote only short excerpts with type and method names so a reader can reproduce them.

## Official references

- https://github.com/icsharpcode/ILSpy
- https://github.com/icsharpcode/ILSpy/blob/master/ICSharpCode.ILSpyCmd/README.md
- https://www.nuget.org/packages/ilspycmd
- https://learn.microsoft.com/en-us/dotnet/core/deploying/ready-to-run
- https://learn.microsoft.com/en-us/dotnet/core/deploying/native-aot/

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
