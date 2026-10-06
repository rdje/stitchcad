"""D76: actual reference input guards must discriminate; restore exact production bytes."""
from pathlib import Path
import runpy
import subprocess
import sys
ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
WORK = ROOT / 'target/formula_input_mutations'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
HERE = ROOT / 'docs/tasks/artifacts/formula_structure'
INPUTS = (Path(__file__), PROFILE_SOURCE, SOURCE, HERE / 'formula_input.py',
          HERE / 'formula_structure.py', HERE / 'static_signature_contract.py',
          *(ROOT / 'docs/book/src' / name for name in (
              'spec/formula-language.md', 'spec/formula-language/grammar.md',
              'spec/formula-language/examples.md', 'spec/reference-skirt.md',
              'spec/units-and-tolerances.md')))
CASES = [
    ('ASCII preflight', 'if not src.isascii():', 'if False and not src.isascii():'),
    ('identifier spelling', 'if not re.fullmatch(r"[a-z][a-z0-9]*(?:_[a-z0-9]+)*", text):', 'if False and not re.fullmatch(r"[a-z][a-z0-9]*(?:_[a-z0-9]+)*", text):'),
    ('keyword reservation', 'if not allow_keyword and text in {"let", "assert", "if"}:', 'if False and not allow_keyword and text in {"let", "assert", "if"}:'),
    ('bare keyword position', '            self._identifier(text)\n            self.take()', '            self.take()'),
    ('call keyword position', 'if name != "if":', 'if False and name != "if":'),
    ('declaration keyword position', 'return self._identifier(self._want("id"))', 'return self._want("id")'),
    ('original unit separator', 'if gap != " ":', 'if False and gap != " ":'),
    ('nonempty call arguments', 'raise FErr("formula_parse", "call arguments require at least one expression")', 'self.take(); return args'),
    ('general ASCII whitespace', r'(?P<sp>[ \t\n\r\f\v]+)', r'(?P<sp>[ \t]+)'),
]

def main():
    plan = dict(directories=(WORK,), sources=INPUTS, mutable_sources=(SOURCE,),
                output_files=tuple(WORK / ('mutation-%d.log' % n) for n in range(1, len(CASES) + 1)))
    PROFILE['plan_producer'](ROOT, **plan)
    environment = PROFILE['prepare_producer'](ROOT, **plan)
    WORK.mkdir(exist_ok=True)
    ORIGINAL = SOURCE.read_bytes()
    TEXT = ORIGINAL.decode()
    try:
        for number, (name, before, after) in enumerate(CASES, 1):
            assert TEXT.count(before) == 1, (name, 'nonunique mutation anchor')
            PROFILE['plan_producer'](ROOT, **plan)
            SOURCE.write_text(TEXT.replace(before, after, 1))
            PROFILE['plan_producer'](ROOT, **plan)
            result = subprocess.run([sys.executable, '-I', '-B',
                                     'docs/tasks/artifacts/formula_structure/formula_input.py',
                                     '--contracts-only'], cwd=ROOT, env=environment, capture_output=True)
            output = result.stdout + result.stderr
            PROFILE['plan_producer'](ROOT, **plan)
            (WORK / ('mutation-%d.log' % number)).write_bytes(output)
            assert result.returncode == 1 and b'AssertionError' in output and b'formula_input.py' in output and b'SyntaxError' not in output, (name, output.decode())
            print('mutation %d %s: rc=1, machine-input contract assertion failed' % (number, name))
            PROFILE['plan_producer'](ROOT, sources=(SOURCE,), mutable_sources=(SOURCE,))
            SOURCE.write_bytes(ORIGINAL)
    finally:
        PROFILE['plan_producer'](ROOT, sources=(SOURCE,), mutable_sources=(SOURCE,))
        SOURCE.write_bytes(ORIGINAL)
    assert SOURCE.read_bytes() == ORIGINAL
    print('formula input mutations: nine actual reds; reference source restored byte-identically')


if __name__ == '__main__':
    try:
        main()
    except (PROFILE['Refusal'], OSError) as error:
        print('local-environment: REFUSED — ' + str(error), file=sys.stderr)
        raise SystemExit(2) from None
