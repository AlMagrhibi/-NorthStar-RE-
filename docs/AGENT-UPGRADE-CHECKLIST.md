# Agent install / upgrade checklist

Use [INSTALL-UPGRADE.md](INSTALL-UPGRADE.md) as the procedural guide. The source is the private repository at https://github.com/AlMagrhibi/-NorthStar-RE-. The host agent must have authenticated read access and permission to modify its own skill directory. Never request raw tokens in chat.

## Acceptance gates

1. Discover host, supported skill directory, Git/Python availability, installed NorthStar commit and local customizations. Do not assume a universal location.
2. Clone into a fresh staging folder with `git -c core.autocrlf=false clone ...`. Inspect origin, exact commit, CHANGELOG, changed scripts, tool references, permissions and licenses. A branch name alone is not a stable version.
3. Verify the trusted release with `py -3 tools/verify_northstar_release.py .` (use `--expect-v8` only for the original V8 release). If the manifest is missing or invalid, stop. Never regenerate checksums simply to make an unverified download pass.
4. Review and run source tests. Confirm test discovery is nonzero, results are successful and applicable to the target host. Record the actual count. If source or tests are missing, stop.
5. Back up the existing skill outside active discovery paths. Preserve project docs, .northstar state and local edits. Do not overwrite conflicting local changes. Stop sessions using the skill before switching directories if necessary.
6. Activate the entire northstar-re master folder, including specialists and references. Reload the host's skill list and verify master discovery, routing and environment inventory. Additional installations require scoped approval.
7. Roll back if activation or smoke tests fail. Document the prior/new commit, tool versions, backup path, commands, actual results and blockers in a local Markdown installation log. Use no credentials or sensitive project details in logs.

## Copyable agent request

> Install or upgrade NorthStar RE from https://github.com/AlMagrhibi/-NorthStar-RE-. Follow README.md, docs/INSTALL-UPGRADE.md and docs/AGENT-UPGRADE-CHECKLIST.md. Verify the exact release, inspect changes, run tests before activation, preserve all local project state and customizations, keep a rollback copy, and provide an Arabic Markdown evidence report. If publication, authentication, tests or host support are incomplete, stop and report the blocker instead of claiming success.

A GitHub link does not initiate an automatic upgrade by itself. This workflow requires an agent capable of Git and local filesystem operations.
