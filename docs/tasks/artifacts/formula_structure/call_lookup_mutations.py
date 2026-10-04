"""Exclusive call-lookup faults; actual compiled assertion reds with byte restoration."""
from pathlib import Path
import os
import re
import runpy
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-core/src/recipe/calls.rs'
CONTRACTS = (ROOT / 'crates/sc-core/tests/formula_call_lookup_contract.rs',)
SOURCES = (SOURCE, *CONTRACTS)
WORK = ROOT / 'target/call_lookup_mutations'
runpy.run_path(str(ROOT / 'scripts/local_environment.py'))['enter_producer'](
    ROOT, directories=(WORK,), sources=SOURCES)
original = SOURCE.read_bytes()
WORK.mkdir(parents=True, exist_ok=True)
CASES = (('unknown accepted',
  'return FormulaBuiltin::from_token(name).ok_or(FormulaCallRefusal {',
  'return FormulaBuiltin::from_token(name).or(Some(FormulaBuiltin::Sin)).ok_or(FormulaCallRefusal '
  '{'),
 ('known identity changed',
  'return FormulaBuiltin::from_token(name).ok_or(FormulaCallRefusal {',
  'return FormulaBuiltin::from_token(name).map(|_| FormulaBuiltin::Cos).ok_or(FormulaCallRefusal '
  '{'),
 ('query forged',
  'name,\n                kind: FormulaCallRefusalKind::Unbound,',
  'name: "missing",\n                kind: FormulaCallRefusalKind::Unbound,'),
 ('curve alias omitted', '"nurbs" | "spline" | "bspline" =>', '"nurbs" | "spline" =>'),
 ('envelope alias widened',
  '"solve" | "constraint" | "fixpoint" =>',
  '"solve" | "constraint" | "fixpoint" | "solve_extra" =>'),
 ('curve family changed',
  '=> FormulaCallRefusalKind::Nurbs,',
  '=> FormulaCallRefusalKind::SketchConstraints,'),
 ('unbound token changed',
  'FormulaCallRefusalKind::Unbound => "formula_unbound_name",',
  'FormulaCallRefusalKind::Unbound => "formula_unknown",'),
 ('envelope token changed',
  'FormulaCallRefusalKind::Nurbs => "env_nurbs",',
  'FormulaCallRefusalKind::Nurbs => "formula_unsupported",'),
 ('source order changed',
  'FormulaCallRefusalKind::Unbound => &[Envelope, BuiltinCatalog],',
  'FormulaCallRefusalKind::Unbound => &[BuiltinCatalog, Envelope],'),
 ('envelope invents search', '&[Envelope]\n            }', '&[Envelope, BuiltinCatalog]\n            }'),
 ('curve alternative omitted',
  'FormulaCallRefusalKind::Nurbs => &[LineSegment, CircularArc, CubicBezier],',
  'FormulaCallRefusalKind::Nurbs => &[LineSegment, CircularArc],'),
 ('recipe alternative wrong',
  'FormulaCallRefusalKind::SketchConstraints => &[OrderedConstructionRecipe],',
  'FormulaCallRefusalKind::SketchConstraints => &[LineSegment],'),
 ('unbound false replacement',
  'FormulaCallRefusalKind::Unbound => &[],',
  'FormulaCallRefusalKind::Unbound => &[OrderedConstructionRecipe],'),
 ('scope conflated', '        "formula_call"', '        "data_name"'),
 ('source metadata tag changed',
  'Self::BuiltinCatalog => "builtin_catalog",',
  'Self::BuiltinCatalog => "recipe",'),
 ('alternative metadata tag changed',
  'Self::CircularArc => "circular_arc",',
  'Self::CircularArc => "nurbs",'),
 ('Debug discloses authored callee',
  '.field("kind", &self.kind)',
  '.field("kind", &self.kind).field("name", &self.name)'),
 ('Display discloses authored callee', 'f.write_str(self.token())', 'f.write_str(self.name)'))
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
    print('call lookup fault controls:18 unique actual anchors; current test assertion sites; passing/expect/compiler noise refused')
    sys.exit(0)

environment = dict(os.environ)
try:
    for index, (name, before, after) in enumerate(CASES, 1):
        SOURCE.write_text(original.decode().replace(before, after, 1))
        result = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test', 'formula_call_lookup_contract'],
                                cwd=ROOT, env=environment, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('fault-%d.log' % index)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output, (name, output.decode())
        assert assertion_failure(output) and b'could not compile' not in output, (name, output.decode())
        print('call lookup fault %d %s:rc=101 actual compiled body assertion red'%(index,name))
        SOURCE.write_bytes(original)
finally:
    SOURCE.write_bytes(original)
    restored = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test', 'formula_call_lookup_contract'],
                              cwd=ROOT, env=environment, capture_output=True)
    (WORK / 'restored.log').write_bytes(restored.stdout + restored.stderr)
    assert restored.returncode == 0, ('restored call lookup artifact failed', restored.stderr.decode())
assert SOURCE.read_bytes() == original
print('call lookup faults:18 actual compiled assertion reds; exact source/current artifact restored')
