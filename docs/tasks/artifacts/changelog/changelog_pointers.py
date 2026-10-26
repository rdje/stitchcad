"""Resolve changelog archive-index link targets; displayed filename text is not a pointer."""
from pathlib import Path
import json
import re
import runpy
import sys

root, authority = map(Path, sys.argv[1:])
master = json.loads((authority / '.doctrine/history_archive/windows.json').read_text())
reader = runpy.run_path(str(Path(__file__).resolve().parents[4] / 'scripts/history_archive.py'))
records = reader['inventory'](authority)[0]
routes = {}
manifests = [json.loads((authority / path).read_text()) for path in master['windows']]
if master['version'] == 2:
    backings = reader['backing_inventory'](authority)
    legacy = json.loads(backings[reader['MASTER']])
    manifests.extend(json.loads(backings[path]) for path in legacy['windows'])
for manifest in manifests:
    for row in manifest['members']:
        if row.get('kind', 'record') != 'record':
            continue
        name = Path(row['path']).name
        routes[manifest['catalog'] + '#' + name.replace('.', '')] = name

# The ledger's archive index has a link in its first table cell. Code examples and HTML comments
# are not index entries. Labels can contain ordinary inline code or escaped punctuation.
link = re.compile(r'^ {0,3}\|[ \t]*\[(?:\\.|[^\]])*\]\(([^ \t)]+)\)')
fence = None
comment = False
pointed = set()
invalid = []
for original in (root / 'CHANGELOG.md').read_text().splitlines():
    if fence is not None:
        closing = re.fullmatch(r' {0,3}(' + re.escape(fence[0]) + r'{%d,})[ \t]*' % fence[1], original)
        if closing:
            fence = None
        continue
    # Strip comment contents without scanning code examples for comment delimiters.
    line = ''
    remaining = original
    while remaining:
        if comment:
            end = remaining.find('-->')
            if end < 0:
                break
            comment = False
            remaining = remaining[end + 3:]
        else:
            start = remaining.find('<!--')
            if start < 0:
                line += remaining
                break
            line += remaining[:start]
            comment = True
            remaining = remaining[start + 4:]
    opening = re.match(r'^ {0,3}(`{3,}|~{3,})', line)
    if opening:
        fence = (opening[1][0], len(opening[1]))
        continue
    match = link.match(line)
    if match is None:
        continue
    target = match[1]
    raw = reader['SEGMENT'].fullmatch(target)
    name = None
    if raw and ((root / target).is_file() or target in records):
        name = Path(target).name
    elif target in routes:
        name = routes[target]
    else:
        invalid.append(target)
    if name and re.fullmatch(r'stitchcad-changelog-part[1-9][0-9]*\.md', name):
        pointed.add(name)
print('\n'.join(sorted(pointed)))
if invalid:
    print('POINTER invalid archive-index target(s): ' + ', '.join(invalid), file=sys.stderr)
    sys.exit(1)
