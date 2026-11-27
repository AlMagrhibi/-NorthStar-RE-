# Governed native batch worker — Windows candidate

`nsre governed-batch --input FILE --out NEW_REPORT --jobs N --timeout-ms T --memory-mib M --cpu-percent C --reviewed-redacted` snapshots and validates the reviewed offline batch, launches only its own native executable suspended, attaches it to a Windows Job Object, and resumes it only after limits are configured. It never accepts an arbitrary executable, shell, network action or target program.

The required budget bounds are 16–16384 MiB committed memory, 1–100 CPU percent and 1–600000 ms deadline. Job/process committed-memory limits, CPU hard cap, one-active-process limit and kill-on-job-close are enforced by Windows, then queried back. Rayon worker threads are allowed; child-process creation is excluded. A parent crash closes its noninherited job handle so the owned worker is terminated. This does not implement durable queue ownership, leases or automatic reconciliation.

At timeout the supervisor terminates only its owned job and requires confirmation that the worker stopped. It does not return a passed result after timeout even if an artifact exists. No user process is killed. Environments that refuse job assignment or CPU-rate controls fail closed rather than claiming enforcement. Hosted Windows nested-job behavior is tested separately.

The report includes OS-measured user/kernel CPU time in 100 ns units, wall time, peak job committed bytes and IO read/write transfer bytes, worker status, verified limits, snapshot input SHA-256 and result SHA-256. IO counters include OS/process activity and are not a storage-device throughput benchmark. No performance gain is claimed. Existing report-only lab operations retain their old evidence flags.

Input/output bytes are bounded and private scratch files are scoped to a newly created directory beside the requested output. The final report is published without overwrite. Source files are not executed. This is resource governance for the native offline batch, not a filesystem sandbox or complete process isolation. Use a trusted stable project directory; adversarial filesystem races remain outside this contract. Queue-based collection and desktop actions do not automatically inherit these limits.

Windows CI exercises a successful real native batch with readback/measurement and a one-millisecond timeout at a one-percent CPU cap. Actual Windows 11/FreePi acceptance and full engine-specific gates remain separate.
