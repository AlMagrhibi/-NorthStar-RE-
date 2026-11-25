# Agent integration contracts

Current adapters are documented filesystem skill registrations, not a compiled SDK. [Compatibility matrix](../docs/en/compatibility.md) records discovery roots, invocation, reload, unverified versions and limitations. Manage.ps1 accepts an explicit agent/root and never edits unrelated settings or provider credentials.

For a new adapter, establish official host/version documentation, discovery path, invocation/reload semantics and recursive discovery behavior. Test against a disposable profile, preserve existing skills, then record exact host/version and evidence. Until then use UNVERIFIED. A plugin ABI, worker protocol and automatic MCP registration are planned and not supplied here.
