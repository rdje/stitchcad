"""D150: independent competing-phase cases through real public/reference whole inputs."""
from fractions import Fraction
from pathlib import Path
import runpy
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
COUPLED = runpy.run_path(str(ROOT / 'docs/tasks/artifacts/formula_structure/static_coupled_contract.py'))
LOADER = COUPLED['STATIC']['BASE']['LOADER']
SOURCE = LOADER['SOURCE']
HUGE = '340282366920938463463374607431768211456'
assert int(HUGE) == 1 << 128  # first unsigned129-bit magnitude
DEEP = '1 mm'
for _ in range(17):
    DEEP = 'if(1==1,1 mm,' + DEEP + ')'
LITERALS = (('count', HUGE), ('ratio', HUGE + '.0'),
            ('length', '1000001 mm'), ('angle', HUGE + ' deg'))
LATE = (
    ('assert late:eps_chord=1==1', 'formula_parse'),
    ('let late:length=(', 'formula_parse'),
    ('let late:length=1+', 'formula_parse'),
    ('let late:length=unknown(1,)', 'formula_parse'),
    ('assert late:eps_geo=1==1==1', 'formula_parse'),
    ('let late:point=missing', 'formula_dimension'),
    ('let late:length=' + '-' * 256 + '1 mm', 'formula_domain'),
    ('let late:length=' + DEEP, 'formula_domain'),
)
CASES = []
for kind, literal in LITERALS:
    for tail, token in LATE:
        for separator in (' ', '\n', '\r\n\t', '\f\v'):
            for prefix in ('', ' \tassert prior:eps_num=1==1\n'):
                CASES.append((prefix + 'let earlier:' + kind + '=' + literal + separator + tail,
                              token, 'syntax', 3 if prefix else 2))
CASES.extend((
    ('let earlier:point=missing\nassert late:eps_chord=1==1', 'formula_dimension', 'syntax', 1),
    ('let earlier:count=' + HUGE + '\nlet late:length=1 mm', 'formula_domain', 'literal', 1),
    ('let earlier:length=missing\nlet late:count=' + HUGE, 'formula_domain', 'literal', 2),
    ('let earlier:length=unknown(1 mm,' + HUGE + ')', 'formula_domain', 'literal', 1),
    ('let earlier:length=if(1==1,1 mm,' + HUGE + ')', 'formula_domain', 'literal', 1),
    ('assert earlier:eps_num=' + HUGE + '==1', 'formula_domain', 'literal', 1),
    ('assert earlier:eps_num=1==' + HUGE, 'formula_domain', 'literal', 1),
))
CASES = tuple(CASES)


def trapped(*args, **kwargs):
    raise AssertionError('D150 execution/provider/state read')


def reference_contract(replacement=None, verbose=True):
    ns, reference = LOADER['load_reference'](replacement)
    for method in ('evaluate', '_evaluate', 'value_of_name', 'resolve_geometry', 'call', 'stored'):
        setattr(reference, method, trapped)
    phase, ordinal = None, None
    syntax, literal = reference._syntax_statement_at, reference._literal_statement_at
    def at_syntax(source, index=None, offset=0, **kwargs):
        nonlocal phase, ordinal
        try:
            return syntax(source, index, offset, **kwargs)
        except ns['FErr']:
            phase, ordinal = 'syntax', index
            raise
    def at_literal(source, checked, index=None, offset=0):
        nonlocal phase, ordinal
        try:
            return literal(source, checked, index, offset)
        except ns['FErr']:
            phase, ordinal = 'literal', index
            raise
    reference._syntax_statement_at, reference._literal_statement_at = at_syntax, at_literal
    for source, token, wanted_phase, wanted_ordinal in CASES:
        phase, ordinal = None, None
        try:
            reference.preflight(source, [])
        except ns['FErr'] as error:
            actual = error.token, phase, ordinal
        else:
            raise AssertionError(('D150 reference accepted invalid input', source))
        assert actual == (token, wanted_phase, wanted_ordinal), (
            'D150 reference competing phase', source, actual, (token, wanted_phase, wanted_ordinal))
    reference._syntax_statement_at, reference._literal_statement_at = syntax, literal

    # The detached APIs still normalize their own literals while parsing. Do not silently
    # grant them a whole-recipe priority contract or change their returned tuple format.
    for kind, text in LITERALS:
        for adapter, source in ((reference.parse, text + '+'),
                                (reference.syntax_statement, 'let a:' + kind + '=' + text + '+')):
            try:
                adapter(source)
            except ns['FErr'] as error:
                assert error.token == 'formula_domain' and error.msg.startswith('literal '), (
                    'D150 detached input scope changed', source, error.token, error.msg)
            else:
                raise AssertionError('D150 detached bad input accepted')

    # A numeric conversion must not even start if later syntax is invalid.
    original_input = reference._literal_input
    reference._literal_input = trapped
    try:
        reference.preflight('let first:count=1\nassert later:eps_chord=1==1', [])
    except ns['FErr'] as error:
        assert error.token == 'formula_parse', 'D150 pure syntax refusal'
    else:
        raise AssertionError('D150 pure syntax accepted bad class')
    finally:
        reference._literal_input = original_input

    # Real phase calls and raw/normalized owners: all syntax, all inputs, then all static.
    chunks = ('let first:length=if(1==1,1.234 mm,2.345 mm)\n',
              'assert check:eps_geo=first==-(3.456 mm+4.567 mm)\n',
              'let last:area=min(first^2,5.678 mm*6.789 mm)')
    source = ' \t' + ''.join(chunks)
    events, raw, normalized, conversions = [], [], [], []
    original_syntax, original_literal, original_static = syntax, literal, reference._static_statement
    def syntax_trace(text, index=None, offset=0, **kwargs):
        assert kwargs.get('literal_inputs') is False, 'D150 whole syntax mode'
        result = original_syntax(text, index, offset, **kwargs)
        raw.append(result); events.append(('syntax', index))
        return result
    def input_trace(text, checked, index=None, offset=0):
        assert len(raw) == 3 and checked is raw[index - 1], 'D150 raw owner/order'
        result = original_literal(text, checked, index, offset)
        normalized.append(result); events.append(('literal', index))
        return result
    def static_trace(text, env, index=None, offset=0, prior_sources=None, **kwargs):
        assert len(normalized) == 3 and kwargs.get('checked') is normalized[index - 1], 'D150 normalized owner/order'
        result = original_static(text, env, index, offset, prior_sources, **kwargs)
        events.append(('static', index))
        return result
    def convert_trace(text, unit):
        assert len(raw) == 3, 'D150 conversion before complete syntax'
        conversions.append((text, unit))
        return original_input(text, unit)
    reference._syntax_statement_at, reference._literal_statement_at = syntax_trace, input_trace
    reference._static_statement, reference._literal_input = static_trace, convert_trace
    try:
        try:
            plan = reference.preflight(source, [])
        except ns['FErr'] as error:
            raise AssertionError(('D150 valid source refused', error.token, error.msg)) from error
        assert events == [(phase, index) for phase in ('syntax', 'literal', 'static') for index in (1, 2, 3)], 'D150 real phase order'
        assert conversions == [('1', None), ('1', None), ('1.234', 'mm'), ('2.345', 'mm'),
                               ('3.456', 'mm'), ('4.567', 'mm'), ('5.678', 'mm'), ('6.789', 'mm')], 'D150 literal source/untaken order'
        assert normalized[0][3] == ('if', ('cmp', '==', ('lit', 'count', Fraction(1)), ('lit', 'count', Fraction(1))),
                                    ('lit', 'length', Fraction(1234)), ('lit', 'length', Fraction(2345))), 'D150 exact literal conversion'
        assert normalized[1][3] is raw[1][3], 'D150 existing name leaf replaced'
        cursor = 2
        for entry, chunk, owner in zip(plan, chunks, normalized):
            assert entry == (cursor, cursor + len(chunk), owner) and entry[2] is owner, 'D150 genuine source/normalized owner'
            cursor += len(chunk)
    finally:
        reference._syntax_statement_at, reference._literal_statement_at = original_syntax, original_literal
        reference._static_statement, reference._literal_input = original_static, original_input
    if verbose:
        print('D150 reference:%d competing token/phase/ordinal cases, eight detached controls, pure syntax/ordered conversion/owners/execution traps; rc=0' % len(CASES))
    return len(CASES)


def product_contract():
    COUPLED['build_and_check']('first-phase')
    catalog = ';'.join(':'.join(entry) for entry in COUPLED['CATALOG']) + '\n'
    payload = catalog + ''.join('%d\tR\t%s\n' % (index, source.encode().hex())
                                for index, (source, _, _, _) in enumerate(CASES))
    product = subprocess.run([str(COUPLED['WORK'] / 'first-phase/driver')], input=payload,
                             cwd=ROOT, text=True, capture_output=True)
    (COUPLED['WORK'] / 'first-phase/competing.tsv').write_text(product.stdout)
    (COUPLED['WORK'] / 'first-phase/competing.log').write_text(product.stderr)
    assert product.returncode == 0, ('D150 public producer refused', product.stderr)
    lines = product.stdout.splitlines()
    assert len(lines) == len(CASES), 'D150 complete public output'
    for index, ((source, token, phase, ordinal), line) in enumerate(zip(CASES, lines)):
        assert line.split('\t') == [str(index), 'ERR', token, 'STAGE:%s:%d' % (phase, ordinal)], ('D150 public token/phase/ordinal', source, line)
    print('D150 public:%d independent token/phase/ordinal cases through Cargo-current adapter; rc=0' % len(CASES))


FAULTS = (
    ('normalize during syntax', 'return self.parse(src, literal_inputs=False)', 'return self.parse(src, literal_inputs=True)', 1),
    ('interleave input after each syntax', 'parsed.append((start, end, checked))',
     'self._literal_statement_at(src[start:end], checked, ordinal, start)\n            parsed.append((start, end, checked))', 1),
    ('input phase reversed', 'enumerate(parsed, 1)', 'enumerate(reversed(parsed), 1)', 1),
    ('last input omitted', 'enumerate(parsed, 1)', 'enumerate(parsed[:-1], 1)', 1),
    ('static before input complete', 'inputs.append((start, end, checked))',
     'self._static_statement(src[start:end], env, ordinal, start, checked=checked)\n            inputs.append((start, end, checked))', 1),
    ('literal ordinal fabricated', 'self._literal_statement_at(src[start:end], checked, ordinal, start)',
     'self._literal_statement_at(src[start:end], checked, 1, start)', 1),
    ('normalization repeated', 'checked = self._literal_statement_at(src[start:end], checked, ordinal, start)',
     'self._literal_statement_at(src[start:end], checked, ordinal, start)\n            checked = self._literal_statement_at(src[start:end], checked, ordinal, start)', 1),
    ('literal child order reversed', 'for child in reversed(children)', 'for child in children', 1),
    ('raw literal spelling lost', 'return ("raw_lit", text, unit)', 'return ("raw_lit", "1", unit)', 1),
    ('unit dropped', 'return ("raw_lit", text, unit)', 'return ("raw_lit", text, None)', 1),
)


def mutations():
    original = SOURCE.read_bytes()
    for name, before, after, count in FAULTS:
        assert original.decode().count(before) == count, ('D150 actual fault anchor', name)
        try:
            reference_contract((before, after), False)
        except AssertionError as error:
            assert 'D150 ' in str(error), (name, 'unrelated compiler/loader/body failure refused', error)
            print('  actual compiled D150 body assertion red:', name)
        else:
            raise AssertionError(('D150 actual fault escaped', name))
    assert SOURCE.read_bytes() == original, 'D150 tracked source changed'
    print('D150 faults:%d actual compiled body assertion reds; source exact' % len(FAULTS))


if __name__ == '__main__':
    assert sys.argv[1:] in ([], ['--mutations']), 'D150 arguments'
    product_contract()
    reference_contract()
    if sys.argv[1:]:
        mutations()
