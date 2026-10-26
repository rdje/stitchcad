"""Verify whole temporary transition receipts against their hashes and named Git byte intervals."""
import argparse
import hashlib
from pathlib import Path
import re
import runpy
import subprocess

ROOT = Path(__file__).resolve().parents[4]


def check(data):
    text = data.decode('utf-8')
    headings = list(re.finditer(r'^### Transitional receipt — [^\n]+\n', text, re.M))
    reader = runpy.run_path(str(ROOT / 'scripts/history_archive.py'))
    git_backed = 0
    for index, heading in enumerate(headings):
        section = text[heading.end():headings[index + 1].start() if index + 1 < len(headings) else len(text)]
        row = re.search(r'^(Source: [0-9a-f]{40}|Pre-commit0342 source), ([A-Za-z0-9_./-]+) bytes '
                        r'([0-9]+)–([0-9]+); ([0-9]+) bytes, sha256:([0-9a-f]{64})\.\n\n'
                        r'```text\n(.*?)^```\n', section, re.M | re.S)
        assert row, 'unrecognized transition receipt: ' + heading.group().strip()
        authority, path, begin, end, size, checksum, body = row.groups()
        payload = body.encode('utf-8')
        begin, end, size = int(begin), int(end), int(size)
        assert 0 <= begin < end <= 160000 and size == end - begin == len(payload), 'receipt extent mismatch'
        assert hashlib.sha256(payload).hexdigest() == checksum, 'receipt hash mismatch'
        assert path.startswith('docs/') and '..' not in Path(path).parts and path.endswith('.md'), 'unsafe source path'
        if authority.startswith('Source: '):
            revision = authority.removeprefix('Source: ')
            environment = reader['metadata_environment'](ROOT)
            name = revision + ':' + path
            count = subprocess.check_output(['git', 'cat-file', '-s', name], cwd=ROOT, env=environment)
            assert re.fullmatch(rb'[0-9]+\n', count) and int(count) <= 160000, 'source blob bound'
            original = subprocess.check_output(['git', 'show', name], cwd=ROOT, env=environment)
            assert len(original) == int(count) and original[begin:end] == payload, 'source interval mismatch'
            git_backed += 1
    return len(headings), git_backed


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--record', help='retained logical record instead of live CHANGELOG.md')
    args = parser.parse_args()
    if args.record:
        reader = runpy.run_path(str(ROOT / 'scripts/history_archive.py'))
        data = reader['inventory'](ROOT)[0][args.record]
    else:
        data = (ROOT / 'CHANGELOG.md').read_bytes()
    total, backed = check(data)
    print('transition receipts: %d exact hashes/extents; %d exact Git intervals; %d pre-commit authored capture; rc=0' %
          (total, backed, total - backed))


if __name__ == '__main__':
    main()
