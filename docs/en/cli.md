# Complete CLI reference

[Authoritative command contract](../../skills/northstar-re/references/v11-native-contract.md) lists every interface, input and limitation.

Engine 0.3.0 provides scan, index, export, diff, probe, analyze, doctor, tools, init, resume, status, report, validate-docs, verify, quality-gate, progress, session, checkpoint, dependency-audit, jobs, memory, search, benchmark, hash, mcp and api. --help lists commands; --version prints the engine version.

Use full executable paths on Windows, for example:

```powershell
& "C:/path/to/nsre.exe" doctor
& "C:/path/to/nsre.exe" init --project "C:/owned-project"
& "C:/path/to/nsre.exe" index --root "C:/owned-corpus" --db "C:/analysis/index.sqlite" --mode strict --jobs 4
```

The corpus and approved output parent must already exist. A new scan output must not exist. Review JSON inputs and pass --reviewed-redacted when required. Do not mistake jobs for worker execution, dependency-audit for vulnerability scanning, or doctor for tool smoke-test certification.
