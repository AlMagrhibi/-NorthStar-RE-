# Operational reference — account-server-engineering

- Native C/C++: use the Windows SDK Winsock sample as a transport fixture, then integrate reviewed identity libraries. C# options: ASP.NET Core Identity or an existing reviewed service abstraction. Do not assume HTTP or a particular protocol.
- Microsoft Winsock: https://learn.microsoft.com/en-us/windows/win32/winsock/getting-started-with-winsock
- ASP.NET Core Identity: https://learn.microsoft.com/en-us/aspnet/core/security/authentication/identity
- OWASP Authentication: https://cheatsheetseries.owasp.org/cheatsheets/Authentication_Cheat_Sheet.html
- Fixture: create disposable fake accounts; verify correct/incorrect password, disabled account, expired session, duplicate requests, rate limit, revocation and no secret leakage. Report auth contract, trust boundaries, state diagram, test results and migration risks.

Use official sources, validate compatibility and publisher; do not execute unknown project downloads. Local test data only; redact secrets.
