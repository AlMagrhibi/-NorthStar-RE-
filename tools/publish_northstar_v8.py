#!/usr/bin/env python3
"""Publish verified NorthStar RE V8 source files as ordinary Git files."""
import argparse
import hashlib
from pathlib import Path, PurePosixPath
import stat
import subprocess
import tempfile
import zipfile

URL = 'https://github.com/AlMagrhibi/-NorthStar-RE-.git'
EXPECTED = '537ad9b9e846f2d2bc295dbda560f9a69f85829451acefd0d2d4815c981ce166'
ROOT = 'NorthStar-RE-Skills'
MANIFEST = 'docs/SHA256SUMS.txt'
INITIAL_DOCS = {'README.md', 'docs/INSTALL-UPGRADE.md', 'docs/GITHUB-PUBLISH.md'}

def digest(data):
    return hashlib.sha256(data).hexdigest()

def read_package(archive):
    if digest(archive.read_bytes()) != EXPECTED:
        raise SystemExit('Unexpected archive SHA-256; do not publish.')
    files = {}
    with zipfile.ZipFile(archive) as z:
        if z.testzip() is not None:
            raise SystemExit('Corrupt archive.')
        for entry in z.infolist():
            if entry.is_dir():
                continue
            p = PurePosixPath(entry.filename)
            if (p.is_absolute() or len(p.parts) < 2 or p.parts[0] != ROOT
                    or any(x in ('.', '..', '.git') or ':' in x or '\\' in x for x in p.parts)
                    or stat.S_ISLNK(entry.external_attr >> 16)):
                raise SystemExit('Unsafe archive entry: ' + entry.filename)
            name = '/'.join(p.parts[1:])
            if name in files:
                raise SystemExit('Duplicate archive entry: ' + name)
            files[name] = z.read(entry)
    hashes = dict((line.split('  ', 1)[1], line.split('  ', 1)[0])
                  for line in files[MANIFEST].decode('utf-8').splitlines() if line.strip())
    if len(files) != 168 or set(hashes) != set(files) - {MANIFEST}:
        raise SystemExit('Unexpected file inventory.')
    for name, data in files.items():
        if name != MANIFEST and digest(data) != hashes[name]:
            raise SystemExit('File hash mismatch: ' + name)
    return files

def git(*args, cwd=None, capture=False):
    return subprocess.run(['git', '-c', 'core.autocrlf=false', *args], cwd=cwd,
                          check=True, stdout=subprocess.PIPE if capture else None).stdout

def stage(repo, files):
    if git('status', '--porcelain', cwd=repo, capture=True).strip():
        raise SystemExit('Repository is not clean.')
    for name, data in files.items():
        p = repo / name
        if p.exists() and p.read_bytes() != data and name not in INITIAL_DOCS:
            raise SystemExit('Existing file differs; review manually: ' + name)
    for name, data in files.items():
        p = repo / name
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_bytes(data)
    git('add', '--', *files, cwd=repo)
    for name, data in files.items():
        if git('show', ':' + name, cwd=repo, capture=True) != data:
            raise SystemExit('Staged Git blob differs: ' + name)
    git('diff', '--cached', '--stat', cwd=repo)

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('archive', type=Path)
    parser.add_argument('--publish', action='store_true', help='Commit and push after reviewing the diff.')
    args = parser.parse_args()
    files = read_package(args.archive)
    print('Verified 168 archive files and 167 manifest entries.', flush=True)
    with tempfile.TemporaryDirectory(prefix='northstar-v8-publish-') as temp:
        repo = Path(temp) / 'repo'
        git('clone', '--branch', 'main', '--single-branch', URL, str(repo))
        if git('remote', 'get-url', 'origin', cwd=repo, capture=True).decode().strip() != URL:
            raise SystemExit('Unexpected remote URL.')
        stage(repo, files)
        if not args.publish:
            print('Dry run only; no commit or push. Re-run with --publish to publish.')
            return
        if input('Publish these source files to private GitHub main? [yes/no] ').strip().lower() != 'yes':
            raise SystemExit('Cancelled.')
        if git('diff', '--cached', '--name-only', cwd=repo, capture=True).strip():
            git('commit', '-m', 'Publish NorthStar RE V8 source and documentation', cwd=repo)
        git('push', 'origin', 'HEAD:main', cwd=repo)
        local = git('rev-parse', 'HEAD', cwd=repo, capture=True).decode().strip()
        remote = git('ls-remote', 'origin', 'refs/heads/main', cwd=repo, capture=True).decode().split()[0]
        if local != remote:
            raise SystemExit('Remote head changed or verification failed.')
        print('Verified remote commit:', local)

if __name__ == '__main__':
    main()
