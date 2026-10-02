"""D96: independent actual POINTER verdicts; labels do not establish retained link targets."""
from pathlib import Path
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
WORK = ROOT / 'target/scratch/ledger_pointer_contract'
SOURCE = ROOT / 'docs/tasks/artifacts/changelog/run_changelog_ledger_probes.sh'
record = 'stitchcad-changelog-part24.md'
source = SOURCE.read_text()
start, stop = 'ledger_verdicts() {', 'mkroot() {'
assert source.count(start) == source.count(stop) == 1, 'actual ledger function boundary changed'
function = start + source.split(start, 1)[1].split(stop, 1)[0]
program = 'set -uo pipefail\nROOT="$1"\n' + function + '\nledger_verdicts "$2" "$3" /dev/null\n'
if WORK.exists():
    assert not list(WORK.rglob('.git')), 'fixture crosses another Git repository'
    shutil.rmtree(WORK)
WORK.mkdir(parents=True)
content = subprocess.check_output(['bash', 'scripts/history_archive.sh', 'read', 'docs/history/' + record], cwd=ROOT)
CASES = [
    ('raw target, arbitrary label', '| [Earlier](docs/history/%s) |\n' % record, True),
    ('catalog target, short label', '| [Earlier](docs/history/window2.md#stitchcad-changelog-part24md) |\n', True),
    ('catalog target, incorrect label', '| [other.md](docs/history/window2.md#stitchcad-changelog-part24md) |\n', True),
    ('catalog target, full label', '| [%s](docs/history/window2.md#stitchcad-changelog-part24md) |\n' % record, True),
    ('label-only prose', record + '\n', False),
    ('filename code span', '`' + record + '`\n', False),
    ('correct label, wrong target', '| [%s](introduction.md) |\n' % record, False),
    ('missing catalog', '| [%s](docs/history/window999.md#stitchcad-changelog-part24md) |\n' % record, False),
    ('missing catalog anchor', '| [%s](docs/history/window2.md#stitchcad-changelog-part999md) |\n' % record, False),
    ('missing raw target', '| [%s](docs/history/stitchcad-changelog-part999.md) |\n' % record, False),
    ('valid pointer plus invalid target', '| [Earlier](docs/history/window2.md#stitchcad-changelog-part24md) |\n| [%s](introduction.md) |\n' % record, False),
    ('fenced pseudo-index', '```markdown\n| [%s](docs/history/window2.md#stitchcad-changelog-part24md) |\n```\n' % record, False),
    ('commented pseudo-index', '<!--\n| [%s](docs/history/window2.md#stitchcad-changelog-part24md) |\n-->\n' % record, False),
]
if '--mask-first' in sys.argv:
    CASES = [CASES[6]] + CASES[:6] + CASES[7:]
try:
    for index, (name, row, expected) in enumerate(CASES):
        fixture = WORK / str(index)
        history = fixture / 'docs/history'
        history.mkdir(parents=True)
        (history / record).write_bytes(content)
        (fixture / 'CHANGELOG.md').write_text('# Fixture\n\n| Segment |\n| --- |\n' + row)
        order = fixture / 'order.txt'
        order.write_text('')
        result = subprocess.run(['bash', '-c', program, 'ledger-pointer', str(ROOT), str(fixture), str(order)], cwd=ROOT, capture_output=True, text=True)
        (fixture / 'verdict.log').write_text(result.stdout + result.stderr)
        failures = [line for line in result.stdout.splitlines() if ' FAIL ' in line]
        assert result.returncode == 0, (name, result.returncode, result.stderr)
        assert (not failures) == expected, (name, expected, failures, result.stdout, result.stderr)
        if not expected:
            assert len(failures) == 1 and failures[0].startswith('POINTER FAIL'), (name, failures)
        print('  pointer control:', name, 'PASS' if expected else 'named POINTER refusal')
    print('ledger pointer contracts: %d independent actual verdicts / 0 fail' % len(CASES))
finally:
    shutil.rmtree(WORK)
