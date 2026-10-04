"""Exclusive wanted-signature faults; actual compiled assertion reds with byte restoration."""
from pathlib import Path
import os
import re
import runpy
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-core/src/recipe/signatures.rs'
CONTRACTS = (ROOT / 'crates/sc-core/tests/formula_operator_signature_contract.rs', ROOT / 'crates/sc-core/tests/formula_builtin_signature_contract.rs')
SOURCES = (SOURCE, *CONTRACTS)
WORK = ROOT / 'target/wanted_signature_mutations'
runpy.run_path(str(ROOT / 'scripts/local_environment.py'))['enter_producer'](
    ROOT, directories=(WORK,), sources=SOURCES)
original = SOURCE.read_bytes()
WORK.mkdir(parents=True, exist_ok=True)
CASES = (('exact kind matcher',
  'R::Exact(wanted) => kind == *wanted,',
  'R::Exact(wanted) => kind != *wanted,'),
 ('arithmetic membership', 'arithmetic(kind) && same', 'same'),
 ('arithmetic consistency', 'arithmetic(kind) && same', 'arithmetic(kind)'),
 ('count negatable',
  'matches!(kind, K::Length | K::Angle | K::Area | K::Ratio)',
  'matches!(kind, K::Length | K::Angle | K::Area | K::Ratio | K::Count)'),
 ('symbolic class matcher',
  'R::ToleranceName => matches!(operand, O::Tolerance(_)),',
  'R::ToleranceName => kind == K::Length,'),
 ('fixed arity guard', 'if !self.arity().accepts(operands.len()) {', 'if false {'),
 ('two operand arity', 'Operands::Two(_) => A::Fixed(2),', 'Operands::Two(_) => A::Fixed(1),'),
 ('variadic arity',
  'Operands::Variadic(_) => A::OneOrMore,',
  'Operands::Variadic(_) => A::Fixed(1),'),
 ('variadic requirement',
  'operands: Operands::Variadic(R::Arithmetic),',
  'operands: Operands::Variadic(R::Exact(K::Length)),'),
 ('pair direction',
  'FormulaKindSignature::two(R::Exact(left), R::Exact(right), V::Exact(result))',
  'FormulaKindSignature::two(R::Exact(right), R::Exact(left), V::Exact(result))'),
 ('square population',
  'FormulaKindSignature::one(R::Exact(K::Length), V::Exact(K::Area)),\n'
  '                        FormulaKindSignature::one(R::Exact(K::Ratio), V::Exact(K::Ratio)),',
  'FormulaKindSignature::one(R::Exact(K::Length), V::Exact(K::Area)),\n'
  '                        FormulaKindSignature::one(R::Exact(K::Count), '
  'V::Exact(K::Count)),'),
 ('count quotient result',
  'pair(K::Count, K::Count, K::Ratio),',
  'pair(K::Count, K::Count, K::Count),'),
 ('product row omitted',
  'pair(K::Ratio, K::Length, K::Length),\n'
  '                        pair(K::Length, K::Count, K::Length),',
  'pair(K::Ratio, K::Length, K::Length),'),
 ('hypot result',
  'Self::Hypot => const { &[pair(K::Length, K::Length, K::Length)] },',
  'Self::Hypot => const { &[pair(K::Length, K::Length, K::Angle)] },'),
 ('conditional result kind', 'V::Operand(1),', 'V::Operand(0),'),
 ('conditional result position', 'V::Operand(1),', 'V::Operand(2),'),
 ('wanted class requirement',
  '                        R::ToleranceName,',
  '                        R::Exact(K::Length),'),
 ('point selector result',
  'Self::PointAt => const { &[pair(K::Edge, K::Ratio, K::Point)] },',
  'Self::PointAt => const { &[pair(K::Edge, K::Ratio, K::Length)] },'),
 ('square fixed-result descriptor',
  'FormulaKindSignature::one(R::Exact(K::Length), V::Exact(K::Area)),\n'
  '                        FormulaKindSignature::one(R::Exact(K::Ratio), V::Exact(K::Ratio)),',
  'FormulaKindSignature::one(R::Exact(K::Length), V::Exact(K::Area)),\n'
  '                        FormulaKindSignature::one(R::Exact(K::Ratio), V::Operand(0)),'))
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


ASSERTION_SITES = set().union(*(assertion_sites(path.read_text(), path.relative_to(ROOT).as_posix())
                                for path in CONTRACTS))
assert ASSERTION_SITES, 'focused contract has no assertion sites'
assert not assertion_failure(b'\nfailures:\nthread panicked:\nexpect-only\n\nfailures:\nassertion_name\n')
assert not assertion_failure(b'assertion: compiler output\n')
assert sys.argv[1:] in [[], ['--classifier-only']]
if sys.argv[1:] == ['--classifier-only']:
    print('wanted signature fault controls:19 unique actual anchors; current test assertion sites; passing/expect/compiler noise refused')
    sys.exit(0)

environment = dict(os.environ)
try:
    for index, (name, before, after) in enumerate(CASES, 1):
        SOURCE.write_text(original.decode().replace(before, after, 1))
        result = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test', 'formula_operator_signature_contract', '--test', 'formula_builtin_signature_contract'],
                                cwd=ROOT, env=environment, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('fault-%d.log' % index)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output, (name, output.decode())
        assert assertion_failure(output) and b'could not compile' not in output, (name, output.decode())
        print('wanted signature fault %d %s:rc=101 actual compiled body assertion red'%(index,name))
        SOURCE.write_bytes(original)
finally:
    SOURCE.write_bytes(original)
    restored = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test', 'formula_operator_signature_contract', '--test', 'formula_builtin_signature_contract'],
                              cwd=ROOT, env=environment, capture_output=True)
    (WORK / 'restored.log').write_bytes(restored.stdout + restored.stderr)
    assert restored.returncode == 0, ('restored wanted signature artifact failed', restored.stderr.decode())
assert SOURCE.read_bytes() == original
print('wanted signature faults:19 actual compiled assertion reds; exact source/current artifact restored')
