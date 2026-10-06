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


def defect_census():
    records, _ = ARCHIVE['inventory'](ROOT)
    pattern = r'^- \*\*D([0-9]+)\*\*|^### Defect D([0-9]+)\b'
    identities = lambda text: {int(a or b) for a, b in re.findall(pattern, text, re.M)}
    closed = set()
    for name, data in records.items():
        if re.fullmatch(r'docs/history/stitchcad-defects-part[0-9]+\.md', name):
            closed.update(identities(data.decode()))
    opened = identities((ROOT / 'docs/tasks/PLANNING.md').read_text())
    published = re.search(r'([0-9]+) open, ([0-9]+) sealed', (ROOT / 'LIVE_STATUS.md').read_text())
    assert published and (len(opened), len(closed)) == (int(published[1]), int(published[2])), 'D209 live defect census drift'
    assert 209 in closed and not opened & closed, 'D209 closure missing or overlapping'
    print('receipt defect census:', len(opened), 'open /', len(closed), 'sealed; no overlap; D209 retained; rc=0')


if __name__ == '__main__':
    controls()
    continuity()
    defect_census()
