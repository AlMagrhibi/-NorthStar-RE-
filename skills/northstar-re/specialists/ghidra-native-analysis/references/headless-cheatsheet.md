# analyzeHeadless cheat sheet

Source: official `Ghidra/RuntimeScripts/support/analyzeHeadlessREADME.md` (master, fetched
2026-11-24; 1052 lines). The README shipped with your install is authoritative for your
release: confirm there before depending on any flag.

```
analyzeHeadless <project_location> <project_name>[/<folder_path>] | ghidra://<server>[:<port>]/<repo>[/<folder>]
    [-import [<dir>|<file>]+ | -process [<project_file>]]
    [-preScript <Name.ext> [<arg>]*] [-postScript <Name.ext> [<arg>]*]
    [-scriptPath "<p1>[;<p2>...]"] [-propertiesPath "<p1>[;...]"]
    [-scriptlog <file>] [-log <file>]
    [-overwrite] [-mirror] [-recursive [<depth>]] [-readOnly] [-deleteProject] [-noanalysis]
    [-processor <languageID>] [-cspec <compilerSpecID>] [-analysisTimeoutPerFile <sec>]
    [-max-cpu <n>] [-librarySearchPaths <p1>[;...]]
    [-loader <name>] [-loader-<arg> <value>]
    (server only: -keystore, -connect, -p, -commit, -okToDelete)
```

| Flag | Behavior worth knowing |
|---|---|
| `-import` / `-process` | Mutually exclusive. `-import` adds files; `-process` runs scripts/analysis on files already in the project (`'a*'` quoted to stop shell expansion). |
| `-readOnly` | With `-import`: imported files are **not saved**. With `-process`: changes are discarded. |
| `-deleteProject` | Deletes a project created in this run only; existing projects are never deleted. Implied by `-readOnly` import into a new project. |
| `-noanalysis` | Skip auto-analysis (on by default). |
| `-processor <id>` | Language ID from the `id` attribute in `Ghidra/Processors/<proc>/data/languages/*.ldefs`, matched exactly. Example `x86:LE:32:default`. |
| `-cspec <id>` | Compiler spec ID from the same `.ldefs`; **requires** `-processor`. |
| `-analysisTimeoutPerFile` | Seconds; on timeout, finished analyzers' results are kept and processing continues to post-scripts. |
| `-log` / `-scriptlog` | Default `application.log` / `script.log` in the user directory. `-log` redirection only works if Log4J is used. Only built-in print methods reach the script log (`print`, `println`, `printf`, `printerr`). |
| `-overwrite` | Replace an existing project file on import; otherwise conflicting imports are skipped. |
| `-recursive [<depth>]` | Recurse into directories; a positive depth also recurses into container files (zip, tar, `.a`). |
| `-max-cpu` | Cores to use; 0 or negative means 1. |
| `-scriptPath` | Unix: escape home variables, e.g. `"\$GHIDRA_HOME/Ghidra/Features/Base/ghidra_scripts;/myscripts"`. Default search: `$USER_HOME/ghidra_scripts` and every `ghidra_script` dir in the distribution. |

Loaders (README list): `BinaryLoader` (`-loader-blockName`, `-loader-baseAddr`,
`-loader-fileOffset`, `-loader-length`, `-loader-applyLabels`, `-loader-anchorLabels`),
`ElfLoader`, `PeLoader` (includes `-loader-parseCliHeaders <true|false>`), `MachoLoader`.
Full Java package loader paths are no longer recognized.

`-loader-baseAddr`: footnote says form `[space:]offset`, hex with no leading `0x`; Example 18
uses `0x1000`. Contradictory docs: verify the block start in Memory Map after import.

General rules: the analyzer may not run if the project is open in the GUI; without `-import` or
`-process`, only program-independent scripts can run; multiple `-preScript` run in command-line
order; `-okToDelete` is required for script-driven deletions in `-process` mode.

Official examples (verbatim pattern):

```bash
analyzeHeadless /path/projects Project1 -import /binaries/binary1.exe
analyzeHeadless /path/projects MyProject -import hello.exe -preScript GetInfoScript.java -readOnly
analyzeHeadless /path/projects MyProject -import hello.exe -loader BinaryLoader -loader-baseAddr 0x1000 -loader-blockName MyBlock -processor x86:LE:32:default
```
