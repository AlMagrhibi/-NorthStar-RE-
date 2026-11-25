# V10 engine continuity in V11
The original scan/index/export/diff/probe commands and schema remain. V11 adds project-local governance and read-only stdio APIs. Read V11 command contract. `index` commits bounded batches; interrupted runs remain incomplete, a fresh run may reuse metadata only in advisory mode. No exact resume guarantee.
