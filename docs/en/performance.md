# Performance and scalability lab

Run engine/benchmarks/Measure-Engine.ps1 with an actual release engine and a new output directory. Default capacities: 1,000; 10,000; 100,000; 1,000,000 files. Fixtures contain deterministic 64-byte synthetic content; every exported hash and count is checked. No user/game data is used. Corpus creation is excluded from scan wall time and removed after measurement; raw reports remain local.

Measured fields: wall time, files/second, bytes/second, child CPU time where accessible, sampled peak working set, DB size, cache hit ratio, strict and incremental modes, output accuracy and an explicit unmeasured recovery field. Sampling is not an OS-level peak guarantee. Cache conditions and filesystem throughput are not controlled, and small-file throughput does not generalize to large binaries. Concurrent shared-host load can materially affect results. CPU utilization can be derived from CPU seconds / wall seconds relative to the recorded processor count when available. No automatic power-loss recovery claim is made.

[Actual recorded dashboard](../reports/PERFORMANCE-AR.md). Missing capacities are BLOCKED or UNVERIFIED, never extrapolated. No Windows throughput claim is derived from Linux measurements.
