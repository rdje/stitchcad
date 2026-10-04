"""D153: actual generator retains the curated body through closed HTML header forms."""
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'knowledge-map/scripts/gen_knowledge_map.sh'
WORK = ROOT / 'target/knowledge_map_contract'
OLD = "sed '/^<!--/,/-->/d' knowledge-map/subsystems.md"
NEW = "sed '/^<!--/ {/-->/d;}; /^<!--/,/-->/d' knowledge-map/subsystems.md"
BODY = '\n- `crates/sc-core/src/lib.rs`; G1-SLICE.5.\n- `docs/book/src/SUMMARY.md`; G0-CONTRACT.\n'
HEADERS = ('', '<!-- one closed line -->\n', '<!--\n two closed lines -->\n',
           '\n<!-- closed line after blank -->\n', '<!-- first -->\n<!-- second -->\n')


def contracts(replacement=None, verbose=True):
    original = SOURCE.read_bytes()
    script = original.decode()
    if replacement:
        before, after = replacement
        assert script.count(before) == 1, 'D153 real generator fault anchor'
        script = script.replace(before, after, 1)
    WORK.mkdir(parents=True, exist_ok=True)
    for index, header in enumerate(HEADERS):
        path = WORK / ('header_%d.md' % index)
        path.write_text(header + BODY)
        relative = path.relative_to(ROOT).as_posix()
        # Only the two actual input reads are redirected. The real generator, Git root,
        # task/decision inventories and stdout framing remain unchanged.
        assert script.count('if [ -f knowledge-map/subsystems.md ]; then') == 1
        assert script.count(' knowledge-map/subsystems.md\n') == 1
        compiled = script.replace('if [ -f knowledge-map/subsystems.md ]; then', 'if [ -f ' + relative + ' ]; then')
        compiled = compiled.replace(' knowledge-map/subsystems.md\n', ' ' + relative + '\n')
        result = subprocess.run(['bash'], input=compiled, cwd=ROOT, capture_output=True, text=True)
        (WORK / ('header_%d.log' % index)).write_text(result.stdout + result.stderr)
        assert result.returncode == 0, ('D153 generator refused valid header', index, result.stderr)
        actual = result.stdout.split('## Key subsystems\n', 1)[1].split('\n## Active task-trees\n', 1)[0]
        # A leading blank before the comment is body whitespace, retained independently.
        expected = ('\n' if header.startswith('\n') else '') + BODY
        assert actual == expected, ('D153 curated body lost or header leaked', index, repr(actual), repr(expected))
        assert path.read_text() == header + BODY, 'D153 fixture changed'
    # The current source's complete body also survives, including every known subsystem route.
    content = (ROOT / 'knowledge-map/subsystems.md').read_text()
    expected = content.split('-->', 1)[1].removeprefix('\n')
    result = subprocess.run(['bash'], input=script, cwd=ROOT, capture_output=True, text=True)
    assert result.returncode == 0, 'D153 current generator refused'
    actual = result.stdout.split('## Key subsystems\n', 1)[1].split('\n## Active task-trees\n', 1)[0]
    assert actual == expected, ('D153 current curated routes lost', repr(actual), repr(expected))
    assert SOURCE.read_bytes() == original, 'D153 actual source changed'
    if verbose:
        print('D153 map contracts:five actual header forms/current complete routes; source exact; rc=0')


if __name__ == '__main__':
    assert sys.argv[1:] in ([], ['--mutations']), 'D153 arguments'
    contracts()
    if sys.argv[1:]:
        for name, after in (('old single-line loss', OLD),
                            ('header not stripped', "sed '' knowledge-map/subsystems.md")):
            try:
                contracts((NEW, after), False)
            except AssertionError as error:
                assert str(error).startswith("('D153 curated body lost or header leaked'"), (name, 'not a body assertion', error)
                print('  actual compiled D153 body assertion red:', name)
            else:
                raise AssertionError(('D153 fault escaped', name))
        print('D153 faults:two actual compiled generator body reds; source exact; rc=0')
