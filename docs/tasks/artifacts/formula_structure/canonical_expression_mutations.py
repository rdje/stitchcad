"""Exclusive actual Rust faults: require compiled public assertion reds and exact restoration."""
from pathlib import Path
import os
import re
import runpy
import sys
import subprocess

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-core/src/recipe/canonical.rs'
CONTRACT = ROOT / 'crates/sc-core/tests/formula_canonical_contract.rs'
FIXTURE_SOURCES = tuple(ROOT / ('docs/tasks/artifacts/formula_structure/' + name) for name in (
    'canonical_expression_cases.tsv', 'literal_normalization_cases.tsv', 'canonical_worked_cases.tsv')) + (
    ROOT / 'docs/book/src/annexes/formula-literals.md',
    ROOT / 'docs/book/src/spec/formula-language/examples.md')
SOURCES = (SOURCE, CONTRACT, *FIXTURE_SOURCES)
WORK = ROOT / 'target/canonical_expression_mutations'
assert sys.argv[1:] in [[], ['--coupled'], ['--classifier-only']]
CLASSIFIER_ONLY = sys.argv[1:] == ['--classifier-only']
coupled = sys.argv[1:] == ['--coupled']
CASES = [
    ('root', 'vec![Action::Node(self.root)]', 'vec![Action::Node(0)]'),
    ('literal kind', 'text.push_str(literal.kind().token());', 'text.push_str("count");'),
    ('literal magnitude', 'literal.magnitude().to_string()', '0_u128.to_string()'),
    ('wide magnitude', 'literal.magnitude().to_string()',
     'u128::from(literal.magnitude() as u64).to_string()'),
    ('name', 'NormalizedData::Name(name) => text.push_str(name),',
     'NormalizedData::Name(_name) => text.push_str("hidden"),'),
    ('unary identity', 'Action::Text("(- ")', 'Action::Text("(neg ")'),
    ('square identity', 'Action::Text("(^2 ")', 'Action::Text("(square ")'),
    ('addition', 'FormulaBinaryOperator::Add => "+",', 'FormulaBinaryOperator::Add => "-",'),
    ('comparison', 'FormulaBinaryOperator::LessEqual => "<=",',
     'FormulaBinaryOperator::LessEqual => "<",'),
    ('left edge', 'pending.push(Action::Node(*left));', 'pending.push(Action::Node(*right));'),
    ('right edge', 'pending.push(Action::Node(*right));', 'pending.push(Action::Node(*left));'),
    ('call order', 'arguments.iter().rev()', 'arguments.iter()'),
    ('call coverage', 'arguments.iter().rev()', 'arguments.iter().rev().take(1)'),
    ('call name', 'pending.push(Action::Text(name));', 'pending.push(Action::Text("renamed"));'),
    ('else branch', 'pending.push(Action::Node(*else_branch));',
     'pending.push(Action::Node(*then_branch));'),
    ('condition', 'pending.push(Action::Node(*condition));',
     'pending.push(Action::Node(*else_branch));'),
    ('newline', 'FormulaCanonicalExpression { text }',
     'FormulaCanonicalExpression { text: text + "\\n" }'),
    ('debug privacy', '.field("byte_count", &self.text.len())', '.field("text", &self.text)'),
]
if coupled:
    CASES = [
        ('worked addition', 'FormulaBinaryOperator::Add => "+",',
         'FormulaBinaryOperator::Add => "-",'),
        ('wide call order', 'arguments.iter().rev()', 'arguments.iter()'),
        ('wide call final argument', 'arguments.iter().rev()',
         'arguments.iter().rev().take(254)'),
    ]
ANCHOR_CASES = tuple(CASES)
if not coupled:
    CASES.append(('spacing', 'Action::Text(" ")', 'Action::Text("  ")'))
OUTPUT_FILES = (() if CLASSIFIER_ONLY else
                tuple(WORK / ('fault-%d.log' % index) for index in range(1, len(CASES) + 1)) +
                (WORK / 'restored.log',))
MUTABLE_SOURCES = () if CLASSIFIER_ONLY else (SOURCE,)
runpy.run_path(str(ROOT / 'scripts/local_environment.py'))['enter_producer'](
    ROOT, directories=(WORK,), output_files=OUTPUT_FILES, sources=SOURCES, mutable_sources=MUTABLE_SOURCES)
ORIGINAL = SOURCE.read_bytes()
WORK.mkdir(parents=True, exist_ok=True)

def assertion_sites(source, path):
    """Current curated test macros, with Rust panic byte columns."""
    sites = set()
    for number, line in enumerate(source.splitlines(), 1):
        match = re.match(r'^(\s*)assert(?:_eq|_ne)?!\s*\(', line)
        if match is not None:
            sites.add((path, number, len(match.group(1).encode()) + 1))
    return sites


def assertion_failure(output):
    """A failed-body panic must name a guarded current assertion macro location."""
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

original_text = ORIGINAL.decode()
for name, before, _ in ANCHOR_CASES:
    assert original_text.count(before) == 1, (name, 'actual anchor not unique')
assert original_text.count('Action::Text(" ")') == 5
if CLASSIFIER_ONLY:
    print('canonical expression classifier:19 unique/count-bound anchors; current thread macros; nonthread/expect/compiler noise refused')
    sys.exit(0)
environment = dict(os.environ)
try:
    for index, (name, before, after) in enumerate(CASES, 1):
        SOURCE.write_text(original_text.replace(before, after))
        command = ['cargo', 'test', '-p', 'sc-core', '--test', 'formula_canonical_contract']
        if coupled:
            command.append('coupled_')
        result = subprocess.run(command, cwd=ROOT,
                                env=environment, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('fault-%d.log' % index)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output, (name, output.decode())
        assert assertion_failure(output) and b'could not compile' not in output, (name, output.decode())
        print('canonical fault %d %s: rc=101, actual compiled public assertion red'
              % (index, name))
        SOURCE.write_bytes(ORIGINAL)
finally:
    SOURCE.write_bytes(ORIGINAL)
    restored = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test',
                               'formula_canonical_contract'],
                              cwd=ROOT, env=environment, capture_output=True)
    (WORK / 'restored.log').write_bytes(restored.stdout + restored.stderr)
    assert restored.returncode == 0, ('restored canonical_expression artifact failed', restored.stderr.decode())
assert SOURCE.read_bytes() == ORIGINAL
print('canonical faults: %d compiled actual assertion reds; source/current artifact restored' % len(CASES))
