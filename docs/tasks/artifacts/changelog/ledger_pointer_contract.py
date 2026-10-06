"""D96: independent actual POINTER verdicts; labels do not establish retained link targets."""
from pathlib import Path
import os
import runpy
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
WORK = ROOT / 'target/scratch/ledger_pointer_contract'
SOURCE = ROOT / 'docs/tasks/artifacts/changelog/run_changelog_ledger_probes.sh'
record = 'stitchcad-changelog-part24.md'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'


def load_profile():
    parent = PROFILE_SOURCE.parent
    if (parent.is_symlink() or not parent.is_dir() or parent.stat().st_dev != ROOT.stat().st_dev or
            (parent / '.git').exists() or (parent / '.git').is_symlink() or
            PROFILE_SOURCE.is_symlink() or not PROFILE_SOURCE.is_file() or
            PROFILE_SOURCE.stat().st_dev != ROOT.stat().st_dev):
        raise ValueError('unsafe ledger fixture profile source')
    return runpy.run_path(str(PROFILE_SOURCE))


try:
    PROFILE = load_profile()
except (ValueError, OSError) as error:
    print('ledger fixtures refused:', error, file=sys.stderr)
    raise SystemExit(2) from None

SOURCES = (Path(__file__), SOURCE, SOURCE.with_name('changelog_pointers.py'), PROFILE_SOURCE,
           ROOT / 'scripts/history_archive.sh', ROOT / 'scripts/history_archive.py')


def backing_inputs():
    value = os.environ.get('HISTORY_ARCHIVE_ROOT') or str(ROOT)
    authority = Path(value)
    if not authority.is_absolute():
        authority = ROOT / authority
    PROFILE['producer_path'](ROOT, authority)
    PROFILE['workspace'](authority)
    PROFILE['require'](not authority.is_relative_to(WORK), 'archive input overlaps fixture workspace')
    files = []
    folders = [authority / '.doctrine/history_archive', authority / 'docs/history']
    for folder in folders:
        PROFILE['directory'](ROOT, PROFILE['producer_path'](ROOT, folder))
        for path in sorted(folder.iterdir()):
            PROFILE['require'](not path.is_symlink(), 'symlink ledger archive input')
            if path.is_dir():
                folders.append(path)
            else:
                files.append(path)
    return tuple(files)


def fixture_paths():
    directories, files = {WORK.parent, WORK}, set()
    for index in range(len(CASES)):
        fixture = WORK / str(index)
        directories.update((fixture, fixture / 'docs', fixture / 'docs/history'))
        files.update((fixture / 'docs/history' / record, fixture / 'CHANGELOG.md',
                      fixture / 'order.txt', fixture / 'verdict.log'))
    return directories, files


def work_plan(*, activate=False):
    directories, files = fixture_paths()
    PROFILE['plan_producer'](ROOT, sources=(*SOURCES, *backing_inputs()),
                             directories=tuple(sorted(directories)), output_files=tuple(sorted(files)))
    if WORK.exists():
        for folder, children, leaves in os.walk(WORK, followlinks=False):
            for name in [*children, *leaves]:
                path = Path(folder) / name
                PROFILE['require'](path in directories or path in files, 'unexpected ledger fixture entry')
    environment = PROFILE['environment_for'](ROOT, os.environ, create=activate)
    if activate:
        os.environ.update(environment)
    return environment


def remove_work():
    work_plan()
    if WORK.exists():
        PROFILE['require'](shutil.rmtree.avoids_symlink_attacks, 'runtime lacks safe ledger fixture removal')
        shutil.rmtree(WORK)


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
    work_plan()
except (PROFILE['Refusal'], OSError) as error:
    print('ledger fixtures refused:', error, file=sys.stderr)
    raise SystemExit(2) from None
source = SOURCE.read_text()
start, stop = 'ledger_verdicts() {', 'mkroot() {'
assert source.count(start) == source.count(stop) == 1, 'actual ledger function boundary changed'
function = start + source.split(start, 1)[1].split(stop, 1)[0]
program = 'set -uo pipefail\nROOT="$1"\n' + function + '\nledger_verdicts "$2" "$3" /dev/null\n'
environment = work_plan(activate=True)
try:
    if WORK.exists():
        assert not list(WORK.rglob('.git')), 'fixture crosses another Git repository'
        remove_work()
    work_plan()
    WORK.mkdir(parents=True)
    content = subprocess.check_output(['bash', 'scripts/history_archive.sh', 'read', 'docs/history/' + record], cwd=ROOT, env=environment)
    for index, (name, row, expected) in enumerate(CASES):
        fixture = WORK / str(index)
        history = fixture / 'docs/history'
        work_plan()
        history.mkdir(parents=True)
        work_plan()
        (history / record).write_bytes(content)
        work_plan()
        (fixture / 'CHANGELOG.md').write_text('# Fixture\n\n| Segment |\n| --- |\n' + row)
        order = fixture / 'order.txt'
        work_plan()
        order.write_text('')
        environment = work_plan()
        result = subprocess.run(['bash', '-c', program, 'ledger-pointer', str(ROOT), str(fixture), str(order)], cwd=ROOT, env=environment, capture_output=True, text=True)
        work_plan()
        (fixture / 'verdict.log').write_text(result.stdout + result.stderr)
        failures = [line for line in result.stdout.splitlines() if ' FAIL ' in line]
        assert result.returncode == 0, (name, result.returncode, result.stderr)
        assert (not failures) == expected, (name, expected, failures, result.stdout, result.stderr)
        if not expected:
            assert len(failures) == 1 and failures[0].startswith('POINTER FAIL'), (name, failures)
        print('  pointer control:', name, 'PASS' if expected else 'named POINTER refusal')
    print('ledger pointer contracts: %d independent actual verdicts / 0 fail' % len(CASES))
finally:
    remove_work()
