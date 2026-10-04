"""D150: competing complete syntax versus literal input, through actual public/reference APIs."""
from pathlib import Path
import runpy
import subprocess

ROOT = Path(__file__).resolve().parents[4]
COUPLED = runpy.run_path(str(ROOT / 'docs/tasks/artifacts/formula_structure/static_coupled_contract.py'))
HUGE = '340282366920938463463374607431768211456'  # exactly2^128, first unsigned129-bit magnitude
assert int(HUGE) == 1 << 128
CASES = (
    ('literal before class', 'let earlier:count=' + HUGE + '\nassert late:eps_chord=1==1', 'formula_parse', 'syntax', 2),
    ('literal before parenthesis', 'let earlier:count=' + HUGE + '\nlet late:length=(', 'formula_parse', 'syntax', 2),
    ('literal before node limit', 'let earlier:count=' + HUGE + '\nlet late:length=' + '-' * 256 + '1 mm', 'formula_domain', 'syntax', 2),
    ('raw annotation before class', 'let earlier:point=missing\nassert late:eps_chord=1==1', 'formula_dimension', 'syntax', 1),
    ('complete syntax then literal', 'let earlier:count=' + HUGE + '\nlet late:length=1 mm', 'formula_domain', 'literal', 1),
)


def contracts():
    # Compile and validate the real public adapter against Cargo's current artifact first.
    COUPLED['build_and_check']('first-phase')
    catalog = ';'.join(':'.join(entry) for entry in COUPLED['CATALOG']) + '\n'
    payload = catalog + ''.join('%d\tR\t%s\n' % (index, source.encode().hex())
                                for index, (_, source, _, _, _) in enumerate(CASES))
    product = subprocess.run([str(COUPLED['WORK'] / 'first-phase/driver')], input=payload,
                             cwd=ROOT, text=True, capture_output=True)
    (COUPLED['WORK'] / 'first-phase/competing.tsv').write_text(product.stdout)
    (COUPLED['WORK'] / 'first-phase/competing.log').write_text(product.stderr)
    assert product.returncode == 0, ('D150 public producer refused', product.stderr)
    lines = product.stdout.splitlines()
    assert len(lines) == len(CASES), 'D150 complete public output'
    ns, reference = COUPLED['STATIC']['BASE']['LOADER']['load_reference']()
    for method in ('evaluate', '_evaluate', 'value_of_name', 'resolve_geometry', 'call', 'stored'):
        def trapped(*args, **kwargs):
            raise AssertionError('D150 execution/provider/state read')
        setattr(reference, method, trapped)
    ordinal = None
    original = reference._syntax_statement_at
    def syntax(source, index=None, offset=0):
        nonlocal ordinal
        try:
            return original(source, index, offset)
        except ns['FErr']:
            ordinal = index  # Actual enclosing call context, not the last successful syntax ordinal.
            raise
    reference._syntax_statement_at = syntax
    if hasattr(reference, '_literal_statement_at'):
        original_literal = reference._literal_statement_at
        def literal(source, checked, index=None, offset=0):
            nonlocal ordinal
            try:
                return original_literal(source, checked, index, offset)
            except ns['FErr']:
                ordinal = index
                raise
        reference._literal_statement_at = literal
    for index, ((name, source, token, phase, wanted_ordinal), line) in enumerate(zip(CASES, lines)):
        assert line.split('\t') == [str(index), 'ERR', token, 'STAGE:%s:%d' % (phase, wanted_ordinal)], ('D150 public token/phase/ordinal', name, line)
        ordinal = None
        try:
            reference.preflight(source, [])
        except ns['FErr'] as error:
            actual_phase = 'literal' if error.msg.startswith('literal ') else 'syntax'
            actual = error.token, actual_phase, ordinal
        else:
            raise AssertionError(('D150 reference accepted invalid input', name))
        assert actual == (token, phase, wanted_ordinal), ('D150 reference competing phase', name, actual, (token, phase, wanted_ordinal))
    print('D150 first phases:five real public/reference token/phase/ordinal cases; execution trapped; rc=0')


if __name__ == '__main__':
    contracts()
