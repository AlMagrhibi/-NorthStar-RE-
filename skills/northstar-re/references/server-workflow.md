# Server engineering workflow — V6

1. Establish ownership and scope; inventory all tools, actual project files, PE/CLR/ABI evidence, dependencies and database configuration (redacted).
2. Map topology from actual evidence: client → authentication boundary → game/world service → persistence; optional services only if found. Service names and transport are not fixed. Do not assume a single server language.
3. Choose needed specialists and verify required tools before dependent work. C: MSVC C mode for .c files; C++: MSVC or existing configured compiler; C#: project-matched .NET/Framework and Visual Studio. SDK libraries are capabilities, not standalone executables.
4. Create localhost-only trusted test environment with disposable DB, synthetic accounts and test certificates. No public listeners or firewall changes without separate approval. Unknown binaries stay in an offline isolated guest.
5. Test positive and negative login, session expiration/revocation, version mismatch, partial frames, server outage, restart, rollback and graceful shutdown. Do not weaken security to pass.
6. Produce topology, environment readiness, protocol/schema evidence, build/test log, compatibility matrix and unresolved blockers. Mark source-inferred vs observed vs verified. Never store secrets or real account records.
