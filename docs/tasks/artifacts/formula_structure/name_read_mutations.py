"""Exclusive name read faults: guarded sources, assertion sites and complete restoration."""
from pathlib import Path
import os
import re
import runpy
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-core/src/recipe/namespace.rs'
CONTRACT = ROOT / 'crates/sc-core/tests/formula_name_read_contract.rs'
SOURCES = (SOURCE, CONTRACT)
WORK = ROOT / 'target/name_read_mutations'
runpy.run_path(str(ROOT / 'scripts/local_environment.py'))['enter_producer'](
    ROOT, directories=(WORK,), sources=SOURCES)
original = SOURCE.read_bytes()
WORK.mkdir(parents=True, exist_ok=True)
CASES = (('value availability read',
  '        self.entries\n'
  '            .get(name)\n'
  '            .copied()\n'
  '            .ok_or(FormulaUnboundName { name })',
  '        if let Some(d) = self.entries.get(name) { if let FormulaDeclarationSource::LengthInput '
  '{ declaration, .. } = d.source() { if declaration.authored_value().is_err() { return '
  'Err(FormulaUnboundName { name }); } } }\n'
  '        self.entries\n'
  '            .get(name)\n'
  '            .copied()\n'
  '            .ok_or(FormulaUnboundName { name })'),
 ('wrong declaration', '.get(name)', '.get("size_index")'),
 ('implicit name repair', '.get(name)', '.get(name.strip_prefix("missing_").unwrap_or(name))'),
 ('missing name fallback',
  '.ok_or(FormulaUnboundName { name })',
  '.or_else(|| self.entries.get("size_index").copied()).ok_or(FormulaUnboundName { name })'),
 ('searched domains',
  '&FormulaOrigin::ALL',
  '&[FormulaOrigin::Measurement, FormulaOrigin::Ease, FormulaOrigin::Parameter, '
  'FormulaOrigin::Profile, FormulaOrigin::Material, FormulaOrigin::Geometry, FormulaOrigin::Size, '
  'FormulaOrigin::Tolerance, FormulaOrigin::Tolerance]'),
 ('unbound token', '        "formula_unbound_name"', '        "formula_unknown"'),
 ('query lost',
  "pub const fn name(self) -> &'a str {\n        self.name",
  'pub const fn name(self) -> &\'a str {\n        "different_query"'),
 ('debug payload',
  'f.debug_struct("FormulaUnboundName")',
  'f.debug_struct("FormulaUnboundName").field("name", &self.name)'),
 ('display payload',
  "impl fmt::Display for FormulaUnboundName<'_> {\n"
  "    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {\n"
  '        f.write_str(self.token())',
  "impl fmt::Display for FormulaUnboundName<'_> {\n"
  "    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {\n"
  '        write!(f, "{}: {}", self.token(), self.name())'))
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
    print('name read fault controls:9 unique actual anchors; name/expect/compiler noise refused')
    sys.exit(0)
environment = dict(os.environ)
try:
    for index, (name, before, after) in enumerate(CASES, 1):
        SOURCE.write_text(original.decode().replace(before, after, 1))
        result = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test',
                                 'formula_name_read_contract'],
                                cwd=ROOT, env=environment, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('fault-%d.log' % index)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output, (name, output.decode())
        assert assertion_failure(output) and b'could not compile' not in output, (name, output.decode())
        print('name read fault %d %s:rc=101 actual compiled body assertion red' % (index, name), flush=True)
        SOURCE.write_bytes(original)
finally:
    SOURCE.write_bytes(original)
    restored = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test',
                               'formula_name_read_contract'],
                              cwd=ROOT, env=environment, capture_output=True)
    (WORK / 'restored.log').write_bytes(restored.stdout + restored.stderr)
    assert restored.returncode == 0, ('restored name read artifact failed', restored.stderr.decode())
assert SOURCE.read_bytes() == original
print('name read faults:9 actual compiled assertion reds; source/current artifact restored')
