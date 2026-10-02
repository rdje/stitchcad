"""D96: mutate actual pointer recognition, require assertion reds, restore every source byte."""
from pathlib import Path
import ast
import subprocess

ROOT = Path(__file__).resolve().parents[4]
WORK = ROOT / 'target/ledger_pointer_mutations'
WORK.mkdir(parents=True, exist_ok=True)
shell = ROOT / 'docs/tasks/artifacts/changelog/run_changelog_ledger_probes.sh'
helper = ROOT / 'docs/tasks/artifacts/changelog/changelog_pointers.py'
original = {path: path.read_bytes() for path in (shell, helper)}
CASES = [
    ('filename text instead of targets', shell, [
        ('pointed="$(python3 -I -B "$ROOT/docs/tasks/artifacts/changelog/changelog_pointers.py" "$root" "$ROOT")" || target_rc=$?',
         'pointed="$(grep -oE \'stitchcad-changelog-part[0-9]+\\.md\' "$log" | LC_ALL=C sort -u)"'),
    ]),
    ('unregistered catalog route accepted', helper, [
        ('elif target in routes:', 'elif target in routes or target.startswith("docs/history/window999.md#"):'),
        ('pointed.add(routes[target])', 'pointed.add(routes.get(target, "stitchcad-changelog-part24.md"))'),
    ]),
    ('invalid extra target status ignored', shell, [
        ('if [ "$target_rc" -eq 0 ] && [ -z "${missing// /}" ]', 'if [ -z "${missing// /}" ]'),
    ]),
    ('code fence mistaken for an index', helper, [
        ('if opening:', 'if False:'),
    ]),
]
try:
    for index, (name, path, changes) in enumerate(CASES, 1):
        text = original[path].decode()
        for before, after in changes:
            assert text.count(before) == 1, (name, 'nonunique actual mutation anchor')
            text = text.replace(before, after)
        if path == helper:
            ast.parse(text)
        path.write_text(text)
        subprocess.run(['bash', '-n', str(shell)], check=True, cwd=ROOT)
        result = subprocess.run(['python3', '-I', '-B', 'docs/tasks/artifacts/changelog/ledger_pointer_contract.py'],
                                cwd=ROOT, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('mutation-%d.log' % index)).write_bytes(output)
        assert result.returncode == 1 and b'AssertionError' in output, (name, result.returncode, output)
        assert b'ledger_pointer_contract.py' in output, (name, 'wrong assertion source')
        path.write_bytes(original[path])
        print('pointer mutation %d %s: rc=1, actual contract assertion red' % (index, name))
finally:
    for path, content in original.items():
        path.write_bytes(content)
for path, content in original.items():
    assert path.read_bytes() == content, ('source restoration', str(path))
subprocess.run(['python3', '-I', '-B', 'docs/tasks/artifacts/changelog/ledger_pointer_contract.py'],
               cwd=ROOT, check=True)
print('ledger pointer mutations: four syntax-checked actual reds; sources restored byte-identically')
