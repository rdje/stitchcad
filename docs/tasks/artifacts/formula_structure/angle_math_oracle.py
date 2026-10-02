"""Independent standard-library math check of defined curated angular fixture values."""
from pathlib import Path
import math
import re
ROOT = Path(__file__).resolve().parents[4]

def round_away(value):
    return math.floor(abs(value) + 0.5) * (-1 if value < 0 else 1)

def argument(source):
    match = re.fullmatch(r'(-?[0-9]+(?:\.[0-9]+)?)(?: (deg|um|mm|m))?(?: / ([0-9]+))?', source)
    assert match, ('oracle fixture argument scope', source)
    number, unit, divisor = match.groups()
    number = float(number)
    factor = {'deg': 1000000, 'um': 1, 'mm': 1000, 'm': 1000000, None: 1000000}[unit]
    # This oracle owns only fixtures whose literal input is already exactly representable.
    # Decimal literal quantization has its separate independent Decimal oracle.
    assert (number * factor).is_integer(), ('nonintegral literal outside math-oracle scope', source)
    internal = number * factor
    if unit == 'deg' or unit is None:
        internal /= 1000000
    return internal / (int(divisor) if divisor else 1)

checks = 0
for line in (ROOT / 'docs/tasks/artifacts/formula_structure/angle_cases.tsv').read_text().splitlines():
    if not line or line.startswith('#'):
        continue
    source, kind, expected = line.split('\t')
    match = re.fullmatch(r'([a-z0-9_]+)\(([^()]+)\)', source)
    assert match, ('oracle fixture expression scope', source)
    name, arguments = match.groups()
    values = [argument(arg) for arg in arguments.split(', ')]
    if name in ['sin', 'cos', 'tan']:
        value = getattr(math, name)(math.radians(values[0])) * 1000000
        result_kind = 'ratio'
    elif name == 'arc_length':
        value = math.radians(values[0]) * values[1]
        result_kind = 'length'
    else:
        assert name in ['atan', 'atan2'], ('unknown oracle function', name)
        value = math.degrees(getattr(math, name)(*values)) * 1000000
        result_kind = 'angle'
    actual = round_away(value)
    if result_kind == 'angle':
        actual %= 360000000
    assert (result_kind, actual) == (kind, int(expected)), ('independent math disagreement', source, value, actual, expected)
    checks += 1
assert checks == 42
print('angle independent math oracle: 42 defined curated rows agree; no arbitrary-input correctness claim')
