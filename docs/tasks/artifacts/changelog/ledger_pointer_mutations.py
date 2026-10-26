"""D96: mutate actual pointer recognition, require assertion reds, restore every source byte."""
from pathlib import Path
import ast
import os
import runpy
import sys
import subprocess

ROOT = Path(__file__).resolve().parents[4]
WORK = ROOT / 'target/ledger_pointer_mutations'
shell = ROOT / 'docs/tasks/artifacts/changelog/run_changelog_ledger_probes.sh'
helper = ROOT / 'docs/tasks/artifacts/changelog/changelog_pointers.py'
CASES = [
    ('filename text instead of targets', shell, [
        ('pointed="$(python3 -I -B "$ROOT/docs/tasks/artifacts/changelog/changelog_pointers.py" "$root" "$ROOT")" || target_rc=$?',
         'pointed="$(grep -oE \'stitchcad-changelog-part[0-9]+\\.md\' "$log" | LC_ALL=C sort -u)"'),
    ]),
    ('unregistered catalog route accepted', helper, [
        ('elif target in routes:', 'elif target in routes or target.startswith("docs/history/window999.md#"):'),
        ('name = routes[target]', 'name = routes.get(target, "stitchcad-changelog-part24.md")'),
    ]),
    ('invalid extra target status ignored', shell, [
        ('if [ "$target_rc" -eq 0 ] && [ -z "${missing// /}" ]', 'if [ -z "${missing// /}" ]'),
    ]),
    ('code fence mistaken for an index', helper, [
        ('if opening:', 'if False:'),
    ]),
]
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
CONTRACT = shell.with_name('ledger_pointer_contract.py')


def load_profile():
    parent = PROFILE_SOURCE.parent
    if (parent.is_symlink() or not parent.is_dir() or parent.stat().st_dev != ROOT.stat().st_dev or
            (parent / '.git').exists() or (parent / '.git').is_symlink() or
            PROFILE_SOURCE.is_symlink() or not PROFILE_SOURCE.is_file() or
            PROFILE_SOURCE.stat().st_dev != ROOT.stat().st_dev):
        raise ValueError('unsafe ledger mutation profile source')
    return runpy.run_path(str(PROFILE_SOURCE))


try:
    PROFILE = load_profile()
except (ValueError, OSError) as error:
    print('ledger mutations refused:', error, file=sys.stderr)
    raise SystemExit(2) from None

SOURCES = (Path(__file__), shell, helper, CONTRACT, PROFILE_SOURCE,
           ROOT / 'scripts/history_archive.sh', ROOT / 'scripts/history_archive.py')
FILES = tuple(WORK / ('mutation-%d.log' % index) for index in range(1, len(CASES) + 1))
EXPECTED = ('catalog target, short label', 'missing catalog',
            'valid pointer plus invalid target', 'fenced pseudo-index')


def backing_inputs():
    authority = Path(os.environ.get('HISTORY_ARCHIVE_ROOT') or str(ROOT))
    if not authority.is_absolute():
        authority = ROOT / authority
    PROFILE['producer_path'](ROOT, authority)
    PROFILE['workspace'](authority)
    PROFILE['require'](not authority.is_relative_to(WORK), 'archive input overlaps mutation workspace')
    files = []
    folders = [authority / '.doctrine/history_archive', authority / 'docs/history']
    for folder in folders:
        PROFILE['directory'](ROOT, PROFILE['producer_path'](ROOT, folder))
        for path in sorted(folder.iterdir()):
            PROFILE['require'](not path.is_symlink(), 'symlink ledger mutation archive input')
            (folders if path.is_dir() else files).append(path)
    return tuple(files)


def producer_plan(*, activate=False):
    PROFILE['plan_producer'](ROOT, sources=(*SOURCES, *backing_inputs()), mutable_sources=(shell, helper),
                             directories=(WORK,), output_files=FILES)
    environment = PROFILE['environment_for'](ROOT, os.environ, create=activate)
    if activate:
        os.environ.update(environment)
    return environment


def assert_pointer_red(index, output):
    text = output.decode('utf-8')
    assert 'assert (not failures) == expected' in text, 'pointer red wrong assertion site'
    assert text.count('AssertionError: ') == 1, 'pointer red malformed evidence'
    try:
        evidence = ast.literal_eval(text.split('AssertionError: ', 1)[1].strip())
    except (SyntaxError, ValueError):
        raise AssertionError('pointer red malformed evidence') from None
    assert isinstance(evidence, tuple) and len(evidence) == 5, 'pointer red malformed evidence'
    name, expected, failures, stdout, stderr = evidence
    assert name == EXPECTED[index - 1], 'pointer red wrong case'
    assert (type(expected) is bool and expected == (index == 1) and isinstance(failures, list) and
            isinstance(stdout, str) and isinstance(stderr, str) and
            failures == [line for line in stdout.splitlines() if ' FAIL ' in line] and
            all(line.startswith('POINTER FAIL') for line in failures) and
            (not failures) != expected), 'pointer red invalid verdict'


try:
    producer_plan()
except (PROFILE['Refusal'], OSError) as error:
    print('ledger mutations refused:', error, file=sys.stderr)
    raise SystemExit(2) from None
original = {path: path.read_bytes() for path in (shell, helper)}
environment = producer_plan(activate=True)
WORK.mkdir(parents=True, exist_ok=True)
try:
    for index, (name, path, changes) in enumerate(CASES, 1):
        text = original[path].decode()
        for before, after in changes:
            assert text.count(before) == 1, (name, 'nonunique actual mutation anchor')
            text = text.replace(before, after)
        if path == helper:
            ast.parse(text)
        producer_plan()
        path.write_text(text)
        environment = producer_plan()
        subprocess.run(['bash', '-n', str(shell)], check=True, cwd=ROOT, env=environment)
        environment = producer_plan()
        result = subprocess.run(['python3', '-I', '-B', 'docs/tasks/artifacts/changelog/ledger_pointer_contract.py'],
                                cwd=ROOT, env=environment, capture_output=True)
        output = result.stdout + result.stderr
        producer_plan()
        (WORK / ('mutation-%d.log' % index)).write_bytes(output)
        assert result.returncode == 1 and b'AssertionError' in output, (name, result.returncode, output)
        assert b'ledger_pointer_contract.py' in output, (name, 'wrong assertion source')
        assert_pointer_red(index, output)
        producer_plan()
        path.write_bytes(original[path])
        print('pointer mutation %d %s: rc=1, actual contract assertion red' % (index, name))
finally:
    for path, content in original.items():
        producer_plan()
        path.write_bytes(content)
for path, content in original.items():
    producer_plan()
    assert path.read_bytes() == content, ('source restoration', str(path))
environment = producer_plan()
subprocess.run(['python3', '-I', '-B', 'docs/tasks/artifacts/changelog/ledger_pointer_contract.py'],
               cwd=ROOT, env=environment, check=True)
print('ledger pointer mutations: four syntax-checked actual reds; sources restored byte-identically')
