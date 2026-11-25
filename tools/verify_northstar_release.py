#!/usr/bin/env python3
"""Read-only release integrity check; does not execute skill code."""
import argparse
import hashlib
from pathlib import Path, PurePosixPath
import re
import sys

REQUIRED = {
    'README.md', 'CHANGELOG.md', 'docs/INSTALL-UPGRADE.md',
    'skills/northstar-re/SKILL.md', '.github/workflows/ci.yml',
}
MANIFEST = 'docs/SHA256SUMS.txt'


def verify(root):
    root = root.resolve()
    manifest = root / MANIFEST
    if not manifest.is_file() or manifest.is_symlink() or manifest.resolve().parent != root / 'docs':
        raise ValueError('Missing or unsafe release manifest; publication incomplete.')
    entries = {}
    casefolded = set()
    for line in manifest.read_text(encoding='utf-8').splitlines():
        if not line.strip():
            continue
        match = re.fullmatch(r'([0-9a-f]{64})  (.+)', line)
        if not match:
            raise ValueError('Malformed manifest line.')
        sha, name = match.groups()
        path = PurePosixPath(name)
        if (path.is_absolute() or str(path) != name or name == MANIFEST
                or any(part in ('.', '..') or ':' in part or '\\' in part or part.casefold() == '.git' for part in path.parts)
                or name.casefold() in casefolded):
            raise ValueError('Unsafe or duplicate manifest path: ' + name)
        casefolded.add(name.casefold())
        entries[name] = sha
    if not REQUIRED <= entries.keys() or not any(n.startswith('tests/test_') and n.endswith('.py') for n in entries):
        raise ValueError('Required source, tests or documentation missing from manifest.')
    for name, expected in entries.items():
        path = root / name
        if not path.is_file() or path.is_symlink() or not path.resolve().is_relative_to(root):
            raise ValueError('Missing or unsafe file: ' + name)
        if hashlib.sha256(path.read_bytes()).hexdigest() != expected:
            raise ValueError('SHA-256 mismatch: ' + name)
    specialists = sum(n.startswith('skills/northstar-re/specialists/') and n.endswith('/SKILL.md') for n in entries)
    if not specialists:
        raise ValueError('No specialist skills in manifest.')
    return len(entries), specialists


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('root', nargs='?', type=Path, default=Path.cwd())
    parser.add_argument('--expect-v8', action='store_true', help='Require the original V8 inventory counts.')
    args = parser.parse_args()
    try:
        count, specialists = verify(args.root)
        if args.expect_v8 and (count, specialists) != (167, 45):
            raise ValueError('Unexpected V8 inventory counts.')
    except (OSError, ValueError) as exc:
        print('BLOCKED:', exc, file=sys.stderr)
        return 1
    print(f'PASS: {count} manifest entries; {specialists} specialists. Extra files not certified.')
    return 0


if __name__ == '__main__':
    sys.exit(main())
