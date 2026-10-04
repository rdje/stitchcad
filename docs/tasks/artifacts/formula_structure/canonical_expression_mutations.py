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
SOURCES = (SOURCE, CONTRACT)
WORK = ROOT / 'target/canonical_expression_mutations'
runpy.run_path(str(ROOT / 'scripts/local_environment.py'))['enter_producer'](
    ROOT, directories=(WORK,), sources=SOURCES)
ORIGINAL = SOURCE.read_bytes()
WORK.mkdir(parents=True, exist_ok=True)
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

coupled = sys.argv[1:] == ['--coupled']
assert not sys.argv[1:] or coupled, 'only --coupled is accepted'
if coupled:
    CASES = [
        ('worked addition', 'FormulaBinaryOperator::Add => "+",',
         'FormulaBinaryOperator::Add => "-",'),
        ('wide call order', 'arguments.iter().rev()', 'arguments.iter()'),
        ('wide call final argument', 'arguments.iter().rev()',
         'arguments.iter().rev().take(254)'),
    ]
original_text = ORIGINAL.decode()
for name, before, _ in CASES:
    assert original_text.count(before) == 1, (name, 'actual anchor not unique')
assert original_text.count('Action::Text(" ")') == 5
if not coupled:
    CASES.append(('spacing', 'Action::Text(" ")', 'Action::Text("  ")'))
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
