# Operational reference — network-protocol-analysis

- Wireshark user guide: https://www.wireshark.org/docs/wsug_html_chunked/
- TShark manual: https://www.wireshark.org/docs/man-pages/tshark.html ; verify `tshark --version`, read an authorized sanitized fixture with `tshark -r fixture.pcapng -T fields -e frame.number -e frame.len` (metadata only).
- Windows Packet Monitor: https://learn.microsoft.com/en-us/windows-server/networking/technologies/pktmon/pktmon ; check help, do not enable capture without scoped approval.
- Winsock: https://learn.microsoft.com/en-us/windows/win32/winsock/getting-started-with-winsock
- .NET sockets: https://learn.microsoft.com/en-us/dotnet/api/system.net.sockets
- Deliver protocol evidence table, versioned schema, sanitized fixtures and parser tests; explicitly mark unknown fields.

Use official sources, validate compatibility and publisher; do not execute unknown project downloads. Local test data only; redact secrets.
