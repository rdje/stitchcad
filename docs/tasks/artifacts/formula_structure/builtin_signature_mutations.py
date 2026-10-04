"""Exclusive built-in-signature faults; actual compiled assertion reds with byte restoration."""
from pathlib import Path
import os
import re
import runpy
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-core/src/recipe/builtins.rs'
CONTRACT = ROOT / 'crates/sc-core/tests/formula_builtin_signature_contract.rs'
SOURCES = (SOURCE, CONTRACT)
WORK = ROOT / 'target/builtin_signature_mutations'
runpy.run_path(str(ROOT / 'scripts/local_environment.py'))['enter_producer'](
    ROOT, directories=(WORK,), sources=SOURCES)
original = SOURCE.read_bytes()
WORK.mkdir(parents=True, exist_ok=True)
CASES = (('closed population', '        Self::PointAt,\n', '        Self::ParamAt,\n'),
 ('exact token', 'Self::RoundTo => "round_to",', 'Self::RoundTo => "round",'),
 ('lookup trimming', 'builtin.token() == token', 'builtin.token() == token.trim()'),
 ('selector category',
  '| Self::PointAt => FormulaBuiltinCategory::Selector,',
  '| Self::PointAt => FormulaBuiltinCategory::Function,'),
 ('fixed arity',
  'Self::Clamp | Self::If | Self::Within => FormulaBuiltinArity::Fixed(3),',
  'Self::Clamp | Self::If | Self::Within => FormulaBuiltinArity::Fixed(2),'),
 ('variadic starts two', 'Self::OneOrMore => count >= 1,', 'Self::OneOrMore => count >= 2,'),
 ('variadic starts zero', 'Self::OneOrMore => count >= 1,', 'Self::OneOrMore => true,'),
 ('tolerance ordinary kind',
  'Self::Tolerance(_) => K::Length,',
  'Self::Tolerance(_) => K::Area,'),
 ('generic kind membership',
  '(arithmetic(first) && operands.iter().all(|operand| operand.kind() == first))',
  '(true && operands.iter().all(|operand| operand.kind() == first))'),
 ('generic homogeneity',
  '(arithmetic(first) && operands.iter().all(|operand| operand.kind() == first))',
  '(arithmetic(first))'),
 ('sqrt area result', 'K::Area => Some(K::Length),', 'K::Area => Some(K::Area),'),
 ('hypot mixed operands',
  'a.kind() == K::Length && b.kind() == K::Length',
  'a.kind() == K::Length && b.kind() == K::Angle'),
 ('trigonometric operand',
  'if a.kind() == K::Angle => Some(K::Ratio),',
  'if a.kind() == K::Ratio => Some(K::Ratio),'),
 ('atan2 matching components',
  'if a.kind() == b.kind() && matches!(a.kind(), K::Length | K::Ratio) =>',
  'if matches!(a.kind(), K::Length | K::Ratio) =>'),
 ('arc argument order',
  'if a.kind() == K::Angle && b.kind() == K::Length',
  'if a.kind() == K::Length && b.kind() == K::Angle'),
 ('conditional condition',
  'if condition.kind() == K::Boolean',
  'if condition.kind() == K::Length'),
 ('symbolic tolerance role',
  '[left, right, FormulaBuiltinOperand::Tolerance(_)]',
  '[left, right, _]'),
 ('tolerance result',
  'same_arithmetic(&[*left, *right]).map(|_| K::Boolean)',
  'same_arithmetic(&[*left, *right]).map(|_| K::Length)'),
 ('direction result',
  'if a.kind() == K::Point && b.kind() == K::Point => Some(K::Angle),',
  'if a.kind() == K::Point && b.kind() == K::Point => Some(K::Length),'),
 ('edge parameter order',
  'if edge.kind() == K::Edge && length.kind() == K::Length',
  'if edge.kind() == K::Length && length.kind() == K::Edge'),
 ('point selector result',
  '                Some(K::Point)\n',
  '                Some(K::Length)\n'))
for name, before, _ in CASES:
    assert original.decode().count(before) == 1, (name, 'actual fault anchor not unique')


def assertion_sites(source, path):
    sites = set()
    for line_number, line in enumerate(source.splitlines(), 1):
        match = re.match(r'^(\s*)assert(?:_eq|_ne)?!\s*\(', line)
        if match is not None:
            sites.add((path, line_number, len(match.group(1).encode()) + 1))
    return sites


def assertion_failure(output):
    parts = output.split(b'\nfailures:\n', 2)
    if len(parts) != 3:
        return False
    locations = re.finditer(rb'panicked at ([^:\n]+):([0-9]+):([0-9]+):\n', parts[1])
    return any((match.group(1).decode('utf-8', 'replace'), int(match.group(2)), int(match.group(3)))
               in ASSERTION_SITES for match in locations)


ASSERTION_SITES = assertion_sites(CONTRACT.read_text(), CONTRACT.relative_to(ROOT).as_posix())
assert ASSERTION_SITES, 'focused contract has no assertion sites'
assert not assertion_failure(b'\nfailures:\nthread panicked:\nexpect-only\n\nfailures:\nassertion_name\n')
assert not assertion_failure(b'assertion: compiler output\n')
assert sys.argv[1:] in [[], ['--classifier-only']]
if sys.argv[1:] == ['--classifier-only']:
    print('built-in signature fault controls:21 unique actual anchors; current test assertion sites; passing/expect/compiler noise refused')
    sys.exit(0)

environment = dict(os.environ)
try:
    for index, (name, before, after) in enumerate(CASES, 1):
        SOURCE.write_text(original.decode().replace(before, after, 1))
        result = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test', 'formula_builtin_signature_contract'],
                                cwd=ROOT, env=environment, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('fault-%d.log' % index)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output, (name, output.decode())
        assert assertion_failure(output) and b'could not compile' not in output, (name, output.decode())
        print('built-in signature fault %d %s:rc=101 actual compiled body assertion red'%(index,name))
        SOURCE.write_bytes(original)
finally:
    SOURCE.write_bytes(original)
    restored = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test', 'formula_builtin_signature_contract'],
                              cwd=ROOT, env=environment, capture_output=True)
    (WORK / 'restored.log').write_bytes(restored.stdout + restored.stderr)
    assert restored.returncode == 0, ('restored builtin signature artifact failed', restored.stderr.decode())
assert SOURCE.read_bytes() == original
print('built-in signature faults:21 actual compiled assertion reds; exact source/current artifact restored')
