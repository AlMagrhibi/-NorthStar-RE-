# Verified V8 publication

This guide is for the initial upload only. The repository remains private. Do not upload project data, client binaries, credentials, analysis dumps or local environments. Obtain the original V8 ZIP from the owner. Its SHA-256 is recorded in [REPOSITORY-AUDIT.md](REPOSITORY-AUDIT.md).

## Windows PowerShell

From a clean folder with Git authenticated through Git Credential Manager or GitHub CLI, clone the private repository. Review the publisher before executing it.

```powershell
git clone https://github.com/AlMagrhibi/-NorthStar-RE-.git NorthStar-RE-source
Set-Location NorthStar-RE-source
py -3 tools/publish_northstar_v8.py "C:\Downloads\NorthStar-RE-Skills-v8-Windows11.zip" --replace-bootstrap-docs
```

Replace the example ZIP path with its actual location. This is a dry run: no commit or push. It verifies every archived byte and stages files in an independent temporary clone. The flag permits replacement of the three bootstrap documentation files only (README, installation and publishing guide); previous versions remain in Git history. Any other conflicting file aborts.

After reviewing the changes, use the same command with `--publish` and confirm interactively. It runs the 79-test minimum gate before pushing. No force push. If remote HEAD changes, stop and reconcile. The archive is published as ordinary source files, not as an uploaded ZIP.

## Remote acceptance

Clone the resulting commit afresh into a separate directory. On Windows, prevent checkout line-ending conversions when verifying the exact release bytes:

```powershell
git -c core.autocrlf=false clone https://github.com/AlMagrhibi/-NorthStar-RE-.git NorthStar-RE-verify
Set-Location NorthStar-RE-verify
git config core.autocrlf false
git rev-parse HEAD
py -3 tools/verify_northstar_release.py . --expect-v8
py -3 -m unittest discover -s tests -v
```

Expect 167 manifest entries plus the manifest itself, 45 specialist skills, and 79 original tests. Extra repository administration files are not certified by the archive manifest. A checksum proves byte consistency, not code safety. Review [GitHub Actions](https://github.com/AlMagrhibi/-NorthStar-RE-/actions) and run live Windows acceptance separately. For subsequent releases, update CHANGELOG, regenerate the reviewed release manifest, review changes in PRs, and tag the verified commit. Do not keep using the V8-only publication script for new versions.
