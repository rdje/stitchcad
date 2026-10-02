"""Exclusive actual Rust faults: require compiled public assertion reds and exact restoration."""
from pathlib import Path
import os
import sys
import subprocess

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-core/src/recipe/canonical.rs'
ORIGINAL = SOURCE.read_bytes()
WORK = ROOT / 'target/canonical_expression_mutations'
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
environment = dict(os.environ, CARGO_HOME=str(ROOT / 'target/cargo-home'),
                   TMPDIR=str(ROOT / 'target/scratch'))
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
        assert b'assertion' in output and b'could not compile' not in output, (name, output.decode())
        print('canonical fault %d %s: rc=101, actual compiled public assertion red'
              % (index, name))
        SOURCE.write_bytes(ORIGINAL)
finally:
    SOURCE.write_bytes(ORIGINAL)
assert SOURCE.read_bytes() == ORIGINAL
print('canonical faults: %d compiled actual assertion reds; exact source restored' % len(CASES))
