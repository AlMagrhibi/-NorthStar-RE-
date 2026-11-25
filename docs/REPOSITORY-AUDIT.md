# Repository audit — 2026-11-25

## Status at audit baseline

- Baseline commit: `7e5b0de30fc0105d4796e2412b460f0b28ff69b6`.
- Private repository; write access verified. 8 tracked files before the audit.
- Missing from baseline: `skills/`, `tests/`, `.github/workflows/ci.yml`, and the release manifest.
- Local original V8 archive: 168 files, valid ZIP CRC, SHA-256 `537ad9b9e846f2d2bc295dbda560f9a69f85829451acefd0d2d4815c981ce166`.
- Five baseline repository files exactly match the archive Git blob hashes: README, CHANGELOG, CONTRIBUTING, SECURITY, .gitignore. Both existing installation/publishing guides differ from their archive counterparts. The publication utility is an extra file.
- Local original V8 test rerun: 79 tests passed on Linux. No live Windows 11 or remote CI results.
- Local verifier acceptance: valid archive passes (167 manifest entries, 45 specialists); altered file, missing file and missing manifest each fail.
- Whitespace review: the 20 Markdown templates have an extra final blank line. No remaining errors when excluding the blank-at-EOF rule; preserve the original bytes during initial publication and clean up in a separate versioned change with a regenerated manifest.
- Local Git network test failed: DNS could not resolve github.com. GitHub connector reads/writes work independently, but this does not establish direct Git connectivity for archive publication.

## Actions

Add an explicit incomplete-publication warning, read-only release verification, an agent upgrade checklist, and a reproducible publication guide. Require explicit approval to replace existing bootstrap docs in the publisher. Preserve project data and unrelated repository files.

## Release acceptance

Publish the original archive as individual files, verify the manifest at the exact remote commit, run tests (nonzero test count), confirm GitHub Actions and conduct Windows 11 acceptance before claiming live readiness. This report is a historical snapshot; consult the current repository tree and verifier for present status.
