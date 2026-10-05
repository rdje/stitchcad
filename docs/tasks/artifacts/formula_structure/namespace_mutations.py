"""Exclusive namespace faults: compiled public assertion reds and exact byte restoration."""
from pathlib import Path
import os
import re
import runpy
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-core/src/recipe/namespace.rs'
CONTRACT = ROOT / 'crates/sc-core/tests/formula_namespace_contract.rs'
SOURCES = (SOURCE, CONTRACT)
WORK = ROOT / 'target/namespace_mutations'
CASES = (
    ('recipe admission', 'Err(declaration)\n            }', 'Ok(Self(declaration))\n            }'),
    ('initial projection', '        self.0\n', '        FormulaDeclaration::reserved(FormulaReservedName::SizeIndex)\n'),
    ('availability read', '        match declaration.source() {',
     '        if let FormulaDeclarationSource::LengthInput { declaration: record, .. } = declaration.source() { if record.authored_value().is_err() { return Err(declaration); } }\n        match declaration.source() {'),
    ('reserved population', 'for reserved in FormulaReservedName::ALL {', 'for reserved in FormulaReservedName::ALL.into_iter().take(7) {'),
    ('optional context hidden', '            let declaration = FormulaDeclaration::reserved(reserved);',
     '            if reserved.required_context() != super::FormulaReservedContext::Always { continue; }\n            let declaration = FormulaDeclaration::reserved(reserved);'),
    ('authored value replaced', 'slot.insert(attempted);', 'slot.insert(FormulaDeclaration::reserved(FormulaReservedName::SizeIndex));'),
    ('collision accepted', '                Entry::Occupied(slot) => {', '                Entry::Occupied(mut slot) => { slot.insert(attempted); continue;'),
    ('authored order reversed', 'for initial in declarations {', 'for initial in declarations.into_iter().collect::<Vec<_>>().into_iter().rev() {'),
    ('reserved token', 'Self::ReservedBinding { .. } => "formula_rebinding",', 'Self::ReservedBinding { .. } => "formula_ambiguous_name",'),
    ('ambiguity token', 'Self::AmbiguousName { .. } => "formula_ambiguous_name",', 'Self::AmbiguousName { .. } => "formula_rebinding",'),
    ('first collision source lost', 'sources: Box::new([*slot.get(), attempted]),', 'sources: Box::new([attempted, attempted]),'),
    ('source argument order swapped', 'Self::AmbiguousName { sources } => **sources,', 'Self::AmbiguousName { sources } => [sources[1], sources[0]],'),
    ('reserved source changed', 'FormulaDeclarationSource::Reserved(reserved) => {',
     'FormulaDeclarationSource::Reserved(_) => { let reserved = FormulaReservedName::SizeIndex;'),
    ('attempted source changed', '} => [FormulaDeclaration::reserved(*reserved), *attempted],',
     '} => [FormulaDeclaration::reserved(*reserved), FormulaDeclaration::reserved(*reserved)],'),
    ('inspection order', 'self.entries.values().copied()', 'self.entries.values().rev().copied()'),
    ('namespace debug payload', 'f.debug_struct("FormulaNamespace")', 'f.debug_struct("FormulaNamespace").field("entries", &self.entries)'),
    ('display payload', "impl fmt::Display for FormulaNamespaceError<'_> {\n    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {\n        f.write_str(self.token())", 'impl fmt::Display for FormulaNamespaceError<\'_> {\n    fn fmt(&self, f: &mut fmt::Formatter<\'_>) -> fmt::Result {\n        write!(f, "{}: {}", self.token(), self.name())'),
)
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

ASSERTION_SITES = assertion_sites(CONTRACT.read_text(), CONTRACT.relative_to(ROOT).as_posix())
assert ASSERTION_SITES, 'focused contract has no assertion sites'
site = min(ASSERTION_SITES)
location = ("\nfailures:\nthread 'fixture' panicked at %s:%d:%d:\n" % site).encode()
assert assertion_failure(location + b'custom macro message\n\nfailures:\nfixture\n')
assert not assertion_failure(location.replace(b"thread 'fixture'", b'compiler note:') +
                             b'nonthread location mention\n\nfailures:\nfixture\n')
assert not assertion_failure(b'\nfailures:\nthread panicked:\nexpect-only\n\nfailures:\nassertion_name\n')
assert not assertion_failure(b'assertion: compiler output\n')

if CLASSIFIER_ONLY:
    print('namespace fault controls:17 unique actual anchors; name/expect/compiler noise refused')
    sys.exit(0)
environment = dict(os.environ)
try:
    for index, (name, before, after) in enumerate(CASES, 1):
        SOURCE.write_text(original.decode().replace(before, after, 1))
        result = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test',
                                 'formula_namespace_contract'],
                                cwd=ROOT, env=environment, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('fault-%d.log' % index)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output, (name, output.decode())
        assert assertion_failure(output) and b'could not compile' not in output, (name, output.decode())
        print('namespace fault %d %s:rc=101 actual compiled body assertion red' % (index, name), flush=True)
        SOURCE.write_bytes(original)
finally:
    SOURCE.write_bytes(original)
    assert SOURCE.read_bytes() == original, 'namespace source not restored'
    restored = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test',
                               'formula_namespace_contract'],
                              cwd=ROOT, env=environment, capture_output=True)
    (WORK / 'restored.log').write_bytes(restored.stdout + restored.stderr)
    assert restored.returncode == 0, ('restored namespace artifact failed', restored.stderr.decode())
assert SOURCE.read_bytes() == original
print('namespace faults:17 actual compiled assertion reds; source/current artifact restored')
