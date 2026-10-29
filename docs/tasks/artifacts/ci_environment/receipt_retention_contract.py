"""D209: actual staged checker and public archive byte bounds; no fixture Git repository."""
from pathlib import Path
import hashlib
import re
import runpy
import subprocess

ROOT = Path(__file__).resolve().parents[4]
CHECK = ROOT / 'scripts/check_fresh_acceptance_evidence.sh'
READER = ROOT / 'scripts/history_archive.py'
TREE = ROOT / 'docs/tasks/G1-SLICE-producers.md'
BEFORE = '5d24d9e0da39d01425eda76718e3cfc631776c9a'
PROFILE = runpy.run_path(str(ROOT / 'scripts/local_environment.py'))
WORK = ROOT / 'target/receipt-retention-controls'
DIFF = WORK / 'staged.patch'
OLD_CHECK = WORK / 'original-check.sh'
SOURCES = (Path(__file__), CHECK, READER, TREE, ROOT / 'scripts/local_environment.py',
           ROOT / 'scripts/check_task_acceptance.sh', ROOT / '.doctrine/code_paths.txt',
           ROOT / '.doctrine/evidence_tokens.txt', ROOT / 'LIVE_STATUS.md',
           ROOT / 'docs/tasks/PLANNING.md')
PLAN = dict(sources=SOURCES, directories=(WORK,), output_files=(DIFF, OLD_CHECK))
ENVIRONMENT = PROFILE['prepare_producer'](ROOT, **PLAN)
ARCHIVE = runpy.run_path(str(READER))
ORIGINAL = subprocess.check_output(['git', 'show', BEFORE + ':' + TREE.relative_to(ROOT).as_posix()], cwd=ROOT)
OLD = subprocess.check_output(['git', 'show', BEFORE + ':' + CHECK.relative_to(ROOT).as_posix()], cwd=ROOT)
PROFILE['plan_producer'](ROOT, **PLAN)
OLD_CHECK.write_bytes(OLD)
HARNESS = r'''
git() {
  case "$*" in
    'rev-parse --show-toplevel') printf '%s\n' "$RECEIPT_ROOT" ;;
    'diff --cached --name-only --diff-filter=ACM')
      if [ "${RECEIPT_DOC_ONLY:-0}" = 0 ]; then printf '%s\n' 'scripts/independent.rs'; fi
      printf '%s\n' 'docs/tasks/independent.md' ;;
    'diff --cached -U0 -- docs/tasks/independent.md') cat "$RECEIPT_DIFF" ;;
    *) return 2 ;;
  esac
}
source "$1"
'''


def gate(script, text, *, docs_only=False):
    PROFILE['plan_producer'](ROOT, **PLAN)
    DIFF.write_text(text)
    environment = dict(ENVIRONMENT, RECEIPT_ROOT=str(ROOT), RECEIPT_DIFF=str(DIFF),
                       RECEIPT_DOC_ONLY='1' if docs_only else '0')
    PROFILE['plan_producer'](ROOT, **dict(PLAN, sources=(*SOURCES, script, DIFF)))
    return subprocess.run(['bash', '-c', HARNESS, 'receipt-control', str(script)],
                          cwd=ROOT, env=environment, capture_output=True, text=True)


def padded(size, unicode=False):
    prefix = '- [x] LOCKSTEP: '
    remaining = size - len(prefix.encode())
    return prefix + ('é' * (remaining // 2) + 'x' * (remaining % 2) if unicode else 'x' * remaining)


def controls():
    base = ''.join('+%s\n' % ('- [x] ' + label + ': independent command rc=0')
                   for label in ('ROOT CAUSE', 'ADDRESSED', 'NO REGRESSION'))
    count = 0
    for unicode in (False, True):
        for size in (399, 400, 401, 414):
            line = padded(size, unicode)
            assert len(line.encode()) == size
            try:
                ARCHIVE['record_stats'](line.encode() + b'\n')
            except ARCHIVE['Refusal'] as error:
                assert size > 400 and str(error) == 'logical maxline bound exceeded'
            else:
                assert size <= 400
            result = gate(CHECK, base + '+' + line + '\n')
            assert result.returncode == (0 if size <= 400 else 1), (size, result.stderr)
            if size > 400:
                assert '%d-byte task line; archive limit=400' % size in result.stderr
                original = gate(OLD_CHECK, base + '+' + line + '\n')
                assert original.returncode == 0, 'D209 original admission changed'
            count += 1
    # A code change with fresh multiline evidence remains accepted; bare/stale evidence refuses.
    multiline = base.replace(' independent command rc=0', '\n+  independent command rc=0')
    assert gate(CHECK, multiline).returncode == 0
    assert gate(CHECK, '+# unrelated text\n').returncode == 1
    assert gate(CHECK, base.replace('rc=0', 'no measured output')).returncode == 1
    assert gate(CHECK, '+' + padded(400, True) + '\n', docs_only=True).returncode == 0
    assert gate(CHECK, '+' + padded(401, True) + '\n', docs_only=True).returncode == 1
    print('receipt byte controls:', count, 'actual archive/gate ASCII/UTF-8 verdicts; four original admissions; multiline positive/two evidence refusals; rc=0')
    print('receipt documentation-only boundary: 400-byte positive/401-byte refusal; rc=0')


def continuity():
    old_text, new_text = ORIGINAL.decode(), TREE.read_text()
    leaf = '- ID: `G1-SLICE.5b.4c.h2.b.p.o.a17`'
    next_leaf = '- ID: `G1-SLICE.5b.4c.h2.b.p.o.a18`'
    old = old_text[old_text.index(leaf):old_text.index(next_leaf)]
    new = new_text[new_text.index(leaf):new_text.index(next_leaf)]
    folded = old.replace('route .a18. --continuity', 'route .a18.\n  --continuity')
    if new != folded:
        records, _ = ARCHIVE['inventory'](ROOT)
        record = records['docs/history/stitchcad-devnotes-part286.md']
        payload = record.split(b'\n---\n\n', 1)[1]
        assert payload.startswith(folded.encode()), 'D209 retained whole receipt differs'
        new = payload[:len(folded.encode())].decode()
    unfolded = new.replace('route .a18.\n  --continuity', 'route .a18. --continuity')
    assert unfolded == old, 'D209 receipt text or data lost'
    old_max = max(len(line) for line in old.encode().split(b'\n'))
    new_max = max(len(line) for line in new.encode().split(b'\n'))
    assert old_max == 414 and new_max <= 400
    try:
        ARCHIVE['record_stats'](old.encode())
    except ARCHIVE['Refusal'] as error:
        assert str(error) == 'logical maxline bound exceeded'
    else:
        raise AssertionError('D209 exact original archive refusal escaped')
    ARCHIVE['record_stats'](new.encode())
    original_diff = ''.join('+' + line + '\n' for line in old.splitlines())
    assert gate(OLD_CHECK, original_diff).returncode == 0
    assert gate(CHECK, original_diff).returncode == 1
    common = '  Acceptance: apply the public-boundary/effect/borrowed-root plan above; actual captures/faults/focused consumer and exact restoration before committing this group.'
    assert '**Ordered group acceptance:** ' + common.split('Acceptance: ', 1)[1] in new_text
    for block in new_text.split('- ID: ')[1:]:
        if '  Acceptance: ordered group acceptance above.' in block:
            identity = block.split('`')[1]
            marker = '- ID: `' + identity + '`'
            if marker not in old_text:
                continue  # Newly authored children inherit the current complete group contract.
            original = old_text.split(marker, 1)[1].split('- ID: ', 1)[0]
            assert common in original, ('D209 projected acceptance drift', identity)
    print('receipt continuity: whole committed .a17 reconstructs exactly sha256=' + hashlib.sha256(old.encode()).hexdigest() + '; 414-byte line folded within400; shared acceptance exact; rc=0')


def retained_closure(records, planning, original):
    """Check the retained D209 lifecycle, without classifying the whole live ledger."""
    pattern = r'^- \*\*D([0-9]+)\*\*|^### Defect D([0-9]+)\b'
    identities = lambda text: [int(a or b) for a, b in re.findall(pattern, text, re.M)]
    owners = {}
    for name, data in records.items():
        if re.fullmatch(r'docs/history/stitchcad-defects-part[0-9]+\.md', name):
            for identity in identities(data.decode()):
                owners.setdefault(identity, []).append(name)
    assert all(len(names) == 1 for names in owners.values()), 'duplicate retained defect report'
    report = 'docs/history/stitchcad-defects-part131.md'
    assert owners.get(209) == [report], 'D209 retained closure missing'
    assert records[report] == original, 'D209 retained closure bytes changed'
    assert 209 not in identities(planning) and not re.search(r'^## D209\b', planning, re.M), 'D209 live report overlaps closure'
    links = re.findall(r'^D209\b[^\n]*?\]\(([^)\n]+)\)', planning, re.M)
    assert any(link.endswith('/stitchcad-defects-part131.md') or
               link.endswith('#stitchcad-defects-part131md') for link in links), 'D209 closure pointer missing'
    return len(owners)


def defect_census():
    records, _ = ARCHIVE['inventory'](ROOT)
    planning = (ROOT / 'docs/tasks/PLANNING.md').read_text()
    revision = 'e45c21d383dcd281a7a25864e1b4e77d5898f03f'
    report = 'docs/history/stitchcad-defects-part131.md'
    original = subprocess.check_output(['git', 'show', revision + ':' + report], cwd=ROOT)
    count = retained_closure(records, planning, original)
    controls = (
        ('missing', {name: data for name, data in records.items() if name != report}, planning,
         'D209 retained closure missing'),
        ('altered', {**records, report: records[report] + b'\n'}, planning,
         'D209 retained closure bytes changed'),
        ('duplicate', {**records, 'docs/history/stitchcad-defects-part999999.md': records[report]}, planning,
         'duplicate retained defect report'),
        ('legacy overlap', records, planning + '\n- **D209** — independent live report\n',
         'D209 live report overlaps closure'),
        ('heading overlap', records, planning + '\n## D209 — independent live report\n',
         'D209 live report overlaps closure'),
        ('pointer omitted', records, re.sub(r'^D209\b[^\n]*\n?', '', planning, flags=re.M),
         'D209 closure pointer missing'),
    )
    for label, changed_records, changed_planning, expected in controls:
        try:
            retained_closure(changed_records, changed_planning, original)
        except AssertionError as error:
            assert str(error) == expected, (label, str(error))
        else:
            raise AssertionError('D209 closure control admitted: ' + label)
    print('receipt retained closure:', count, 'distinct canonical sealed reports; D209 whole original bytes/pointer/no live report; six refusal controls; rc=0')
    print('receipt scope: current global open/closed census remains deferred to PLANNING.5; rc=0')


if __name__ == '__main__':
    controls()
    continuity()
    defect_census()
