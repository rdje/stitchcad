"""Diagnostic bounded adaptive tangent request model;no numerical-domain cutoff.

Selected resources bound per-candidate integer width and cumulative charged series terms.
Original input/limits/counters survive refusal;only certified whole intervals return values.
"""
from collections import namedtuple
from fractions import Fraction as F
from pathlib import Path
import ast
import runpy

ROOT = Path(__file__).resolve().parents[4]
SOURCE = Path(__file__).with_name('tangent_reduction_plan.py')
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
PROFILE['plan_producer'](ROOT, sources=(Path(__file__), SOURCE, PROFILE_SOURCE))
MODEL = runpy.run_path(str(SOURCE))
FOLD_BITS = (2 * MODEL['A'] * MODEL['U']).bit_length()
Budget = namedtuple('Budget', 'maximum_integer_bits maximum_series_terms')
Refusal = namedtuple('Refusal', 'resource maximum consumed requested')
Outcome = namedtuple('Outcome', 'kind value numerator denominator limits precision terms reserved_integer_bits refusal')


def evaluate(numerator, denominator, limits):
    """Finite selected-budget request;resource refusal is no numerical diagnostic or value."""
    assert abs(numerator) <= MODEL['U'] and 0 < denominator <= MODEL['U']
    assert all(isinstance(value, int) and value >= 0 for value in limits)
    used_terms = reserved_bits = 0

    def finish(kind, value, precision, refusal=None):
        return Outcome(kind, value, numerator, denominator, limits, precision, used_terms, reserved_bits, refusal)

    # Complete fixed folding extent precedes private period/reflection arithmetic.
    if limits.maximum_integer_bits < FOLD_BITS:
        return finish('resources', None, 0, Refusal('integer_bits', limits.maximum_integer_bits, 0, FOLD_BITS))
    reserved_bits = FOLD_BITS
    folded = MODEL['fold'](numerator, denominator)
    if folded.role != 'quotient':
        kind, value = MODEL['decision'](MODEL['enclose'](numerator, denominator, 0))
        return finish(kind, value, 0)
    precision = 128
    while True:
        bits, terms = MODEL['integer_limit'](precision), 4 * precision
        # Check both complete candidate reservations before charging or computing that iteration.
        if bits > limits.maximum_integer_bits:
            return finish('resources', None, precision,
                          Refusal('integer_bits', limits.maximum_integer_bits, reserved_bits, bits))
        if terms > limits.maximum_series_terms - used_terms:
            return finish('resources', None, precision,
                          Refusal('series_terms', limits.maximum_series_terms, used_terms, terms))
        reserved_bits, used_terms = max(reserved_bits, bits), used_terms + terms
        kind, value = MODEL['decision'](MODEL['enclose'](numerator, denominator, precision))
        if kind != 'undecided':
            return finish(kind, value, precision)
        precision *= 2


def contracts(evaluator=evaluate):
    m, a, u = MODEL['M'], MODEL['A'], MODEL['U']
    assert FOLD_BITS == 156
    n = 247385123034593398372138492992328993860
    d = 8635369834810561370044731507908276509
    # Expected rounding comes from the independent inverse/alternate-pi oracle,not the controller.
    pi = MODEL['fraction_pi'](288)
    angle = MODEL['fraction_atan'](F(1, 2 * m), 16)
    lower, upper = 2 * a * angle[0] / pi[1], 2 * a * angle[1] / pi[0]
    assert F(n, d) < lower < upper
    expected = 0
    cases = (
        ('empty', 0, 1, Budget(0, 0), 'resources', None, 0, 0, 0, Refusal('integer_bits', 0, 0, 156)),
        ('fold_short', 0, 1, Budget(155, 0), 'resources', None, 0, 0, 0, Refusal('integer_bits', 155, 0, 156)),
        ('zero', 0, 1, Budget(156, 0), 'rounded', 0, 0, 0, 156, None),
        ('turn', 180 * m, 1, Budget(156, 0), 'rounded', 0, 0, 0, 156, None),
        ('unit', 45 * m, 1, Budget(156, 0), 'rounded', m, 0, 0, 156, None),
        ('negative_unit', -45 * m, 1, Budget(156, 0), 'rounded', -m, 0, 0, 156, None),
        ('pole', a, 1, Budget(156, 0), 'pole', None, 0, 0, 156, None),
        ('negative_pole', -a, 1, Budget(156, 0), 'pole', None, 0, 0, 156, None),
        ('first', 30 * m, 1, Budget(2073, 512), 'rounded', 577350, 128, 512, 2073, None),
        ('first_more', 30 * m, 1, Budget(8219, 3584), 'rounded', 577350, 128, 512, 2073, None),
        ('first_bits', 30 * m, 1, Budget(2072, 3584), 'resources', None, 128, 0, 156,
         Refusal('integer_bits', 2072, 156, 2073)),
        ('first_terms', 30 * m, 1, Budget(8219, 511), 'resources', None, 128, 0, 156,
         Refusal('series_terms', 511, 0, 512)),
        ('both_first', 30 * m, 1, Budget(2072, 511), 'resources', None, 128, 0, 156,
         Refusal('integer_bits', 2072, 156, 2073)),
        ('later_bits', n, d, Budget(4122, 3584), 'resources', None, 512, 1536, 4122,
         Refusal('integer_bits', 4122, 4122, 8219)),
        ('later_terms', n, d, Budget(8219, 1536), 'resources', None, 512, 1536, 4122,
         Refusal('series_terms', 1536, 1536, 2048)),
        ('both_later', n, d, Budget(4122, 1536), 'resources', None, 512, 1536, 4122,
         Refusal('integer_bits', 4122, 4122, 8219)),
        ('refined', n, d, Budget(8219, 3584), 'rounded', expected, 512, 3584, 8219, None),
        ('refined_more', n, d, Budget(16412, 7680), 'rounded', expected, 512, 3584, 8219, None),
        ('actual_width', a * ((u + 1) // a) - 1, (u + 1) // a,
         Budget(2073, 512), 'width', None, 128, 512, 2073, None),
    )
    for label, numerator, denominator, limits, kind, value, precision, terms, bits, refusal in cases:
        result = evaluator(numerator, denominator, limits)
        assert (result.kind, result.value, result.precision, result.terms, result.reserved_integer_bits, result.refusal) == (
            kind, value, precision, terms, bits, refusal), (label, result)
        assert result.limits is limits and (result.numerator, result.denominator) == (numerator, denominator)
        assert result.terms <= limits.maximum_series_terms and result.reserved_integer_bits <= limits.maximum_integer_bits
        assert result.kind == 'rounded' or result.value is None
    return len(cases)


def main():
    count = contracts()
    # In-memory actual controller faults:positive mathematics alone cannot prove budget admission.
    original = Path(__file__).read_bytes()
    tree = ast.parse(original)
    function = next(node for node in tree.body if isinstance(node, ast.FunctionDef) and node.name == 'evaluate')
    body = ast.get_source_segment(original.decode(), function)
    reds = 0
    for before, after, wanted in (
        ('if limits.maximum_integer_bits < FOLD_BITS:', 'if False:', 'empty'),
        ('if bits > limits.maximum_integer_bits:', 'if False:', 'first_bits'),
        ('if terms > limits.maximum_series_terms - used_terms:', 'if False:', 'first_terms'),
    ):
        assert body.count(before) == 1
        scope = dict(globals())
        PROFILE['plan_producer'](ROOT, sources=(Path(__file__), SOURCE, PROFILE_SOURCE))
        exec(compile(body.replace(before, after, 1), __file__, 'exec'), scope)
        try:
            contracts(scope['evaluate'])
        except AssertionError as error:
            assert error.args and isinstance(error.args[0], tuple) and error.args[0][0] == wanted, (wanted, error)
            reds += 1
        else:
            raise AssertionError(('actual budget guard fault survived', wanted))
    assert Path(__file__).read_bytes() == original
    print(f'tangent budget plan:{count} exact outcomes/refusals/original-input-policy/counter controls;'
          f'{reds} actual guard-body reds;source unchanged;rc=0')
    print('refinement128/256/512:3584 cumulative terms/8219 candidate integer bits certifies actual boundary zero;'
          'smaller budgets return resources,not numerical domain or a value')


if __name__ == '__main__':
    main()
