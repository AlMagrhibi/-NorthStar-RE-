# Local API and MCP reference

[Documentation index](../README.md) · [العربية](../ar/api.md)

## Transport
nsre api --project DIR uses newline-delimited JSON-RPC 2.0 over stdin/stdout. No HTTP service, socket listener or remote-control authentication is implemented. Requests are bounded to 1 MiB; invalid JSON returns -32700. Bind a trusted project explicitly.

## API methods
doctor or nsre_doctor returns environment discovery. status or nsre_status returns documentation and project state. ping returns an empty result. Unknown methods return -32601; invalid requests -32600; runtime failures -32000. Do not send secrets.

## MCP candidate
nsre mcp --project DIR supports initialize, notifications/initialized, ping, tools/list and tools/call. It negotiates candidate protocol 2025-11-25 and exposes nsre_doctor/nsre_status only. Tools accept an empty arguments object. Host/version compatibility is UNVERIFIED; do not claim current MCP conformance. No agent config is modified.

Example initialize request:

```json
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25"}}
```

After the initialized notification, request tools/list. A successful handshake fixture is not proof of production host interoperability.
