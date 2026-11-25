# Operational reference — network-cryptography

- .NET SslStream: https://learn.microsoft.com/en-us/dotnet/api/system.net.security.sslstream
- Windows CNG: https://learn.microsoft.com/en-us/windows/win32/seccng/cng-portal
- OpenSSL docs: https://docs.openssl.org/ ; verify `openssl version` after provenance review; use a test-only local certificate chain and known-answer fixtures, no production key export.
- OWASP TLS: https://cheatsheetseries.owasp.org/cheatsheets/Transport_Layer_Security_Cheat_Sheet.html
- Tests: valid local trust chain, wrong identity, expired certificate, modified ciphertext, truncated messages and clean connection shutdown. Deliver algorithm inventory and migration recommendations without secrets.

Use official sources, validate compatibility and publisher; do not execute unknown project downloads. Local test data only; redact secrets.
