# Configuration reference

[Documentation index](../README.md) · [العربية](../ar/configuration.md)

## Management parameters
- -Action Install|Update|Doctor|Build-Engine|Backup|Uninstall|Rollback.
- -Agent FreePi|Codex|Pi|Claude|Other: choose a documented conventional skill root.
- -SkillRoot: explicit absolute directory containing northstar-re.
- -NonInteractive: caller explicitly authorized this operation and destination; ambiguity still fails.
- -AllowDependencyDownload: permits Cargo downloads for this source build only.
- -BackupPath: explicit owned central backup used for Rollback.

## Local files
%LOCALAPPDATA%/NorthStarRE/registration.json records skill/release/version/engine_sha256/package_sha256/agent. releases/ holds immutable per-install engine copies; backups/ preserves skills and registration. installation.jsonl records actions with local paths, never credentials. Local paths are private operational data and must not be published.

## Engine flags
scan uses --root/--out/--jobs. index uses --root/--db/--jobs/--mode strict|reuse-metadata. export uses --db/--run; diff --db/--old/--new. Project commands use --project, optional --input and explicit --reviewed-redacted. Unknown/duplicate flags are rejected in ordinary project/index commands. No unattended update policy is enabled.

## Framework version metadata

framework.json defines product generation, candidate version, engine version, target, toolchain and catalog counts. The validator requires its version to match .github/release-request.json. The installer records that declared version instead of a hardcoded old candidate. Project database schema versions remain separate and unchanged.
