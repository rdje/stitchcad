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
assert sys.argv[1:] in [[], ['--classifier-only']]
CLASSIFIER_ONLY = sys.argv[1:] == ['--classifier-only']
OUTPUT_FILES = (() if CLASSIFIER_ONLY else
                tuple(WORK / ('fault-%d.log' % index) for index in range(1, len(CASES) + 1)) +
                (WORK / 'restored.log',))
MUTABLE_SOURCES = () if CLASSIFIER_ONLY else (SOURCE,)
runpy.run_path(str(ROOT / 'scripts/local_environment.py'))['enter_producer'](
    ROOT, directories=(WORK,), output_files=OUTPUT_FILES, sources=SOURCES, mutable_sources=MUTABLE_SOURCES)
original = SOURCE.read_bytes()
WORK.mkdir(parents=True, exist_ok=True)
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
    """A failed-body thread panic must name a current guarded assertion macro."""
    parts = output.split(b'\nfailures:\n', 2)
    if len(parts) != 3:
        return False
    for match in re.finditer(
            rb'(?m)^thread\b[^\r\n]*? panicked at ([^:\r\n]+):([0-9]+):([0-9]+):\r?$', parts[1]):
        try:
            path = match.group(1).decode('utf-8')
        except UnicodeDecodeError:
            continue
        if (path, int(match.group(2)), int(match.group(3))) in ASSERTION_SITES:
            return True
    return False


ASSERTION_SITES = set().union(*(assertion_sites(path.read_text(), path.relative_to(ROOT).as_posix())
                                for path in CONTRACTS))
assert ASSERTION_SITES, 'focused contract has no assertion sites'
site = min(ASSERTION_SITES)
location = ("\nfailures:\nthread 'fixture' panicked at %s:%d:%d:\n" % site).encode()
assert assertion_failure(location + b'custom macro message\n\nfailures:\nfixture\n')
assert not assertion_failure(location.replace(b"thread 'fixture'", b'compiler note:') +
                             b'nonthread location mention\n\nfailures:\nfixture\n')
assert not assertion_failure(b'\nfailures:\nthread panicked:\nexpect-only\n\nfailures:\nassertion_name\n')
assert not assertion_failure(b'assertion: compiler output\n')
if CLASSIFIER_ONLY:
    print('call lookup fault controls:18 unique actual anchors; current test assertion sites; thread/name/expect/compiler noise refused')
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
        print('call lookup fault %d %s:rc=101 actual compiled body assertion red' % (index, name), flush=True)
        SOURCE.write_bytes(original)
finally:
    SOURCE.write_bytes(original)
    assert SOURCE.read_bytes() == original, 'call lookup source not restored'
    restored = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test', 'formula_call_lookup_contract'],
                              cwd=ROOT, env=environment, capture_output=True)
    (WORK / 'restored.log').write_bytes(restored.stdout + restored.stderr)
    assert restored.returncode == 0, ('restored call lookup artifact failed', restored.stderr.decode())
assert SOURCE.read_bytes() == original
print('call lookup faults:18 actual compiled assertion reds; exact source/current artifact restored')
