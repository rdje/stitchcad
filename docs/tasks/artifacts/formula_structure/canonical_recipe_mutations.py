"""Exclusive owned-identity production faults; only compiled failed-body assertions count."""
from pathlib import Path
import os
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-core/src/recipe/canonical_recipe.rs'
original = SOURCE.read_bytes()
cases = [
 ('binding opcode', 'text.push_str("(bind ");', 'text.push_str("(let ");'),
 ('assertion opcode', 'text.push_str("(assert ");', 'text.push_str("(check ");'),
 ('binding name', 'text.push_str(statement.name());\n            text.push(\' \');\n            text.push_str(declared_kind.token());',
  'text.push_str("scrubbed");\n            text.push(\' \');\n            text.push_str(declared_kind.token());'),
 ('assertion name', 'text.push_str(statement.name());\n            text.push(\' \');\n            text.push_str(tolerance.token());',
  'text.push_str("scrubbed");\n            text.push(\' \');\n            text.push_str(tolerance.token());'),
 ('declared kind', 'text.push_str(declared_kind.token());', 'let _ = declared_kind; text.push_str("angle");'),
 ('symbolic tolerance', 'text.push_str(tolerance.token());', 'let _ = tolerance; text.push_str("eps_geo");'),
 ('left operand', 'text.push_str(left.canonical_form().as_str());', 'text.push_str(right.canonical_form().as_str());'),
 ('right operand', 'text.push_str(right.canonical_form().as_str());', 'text.push_str(left.canonical_form().as_str());'),
 ('extra equality wrapper', 'text.push_str(left.canonical_form().as_str());\n            text.push(\' \');\n            text.push_str(right.canonical_form().as_str());',
  'text.push_str("(== "); text.push_str(left.canonical_form().as_str());\n            text.push(\' \');\n            text.push_str(right.canonical_form().as_str()); text.push(\')\');'),
 ('recipe opcode', 'let mut text = String::from("(recipe");', 'let mut text = String::from("(sequence");'),
 ('statement order', 'for statement in self.statements() {', 'for statement in self.statements().iter().rev() {'),
 ('complete count', 'for statement in self.statements() {', 'for statement in self.statements().iter().take(4095) {'),
 ('recipe separator', "text.push(' ');\n            append_statement(&mut text, statement);",
  "text.push('\\n');\n            append_statement(&mut text, statement);"),
 ('empty identity', 'FormulaCanonicalRecipe { text }', 'if self.statements().is_empty() { text.clear(); } FormulaCanonicalRecipe { text }'),
 ('statement closer', "    text.push(')');\n}", "    text.push(' ');\n}"),
 ('statement terminal newline', 'FormulaCanonicalStatement { text }', "text.push('\\n'); FormulaCanonicalStatement { text }"),
 ('recipe terminal newline', 'FormulaCanonicalRecipe { text }', "text.push('\\n'); FormulaCanonicalRecipe { text }"),
 ('statement Debug privacy', 'f.debug_struct("FormulaCanonicalStatement")\n            .field("byte_count", &self.text.len())',
  'f.debug_struct("FormulaCanonicalStatement")\n            .field("text", &self.text)'),
 ('recipe Debug privacy', 'f.debug_struct("FormulaCanonicalRecipe")\n            .field("byte_count", &self.text.len())',
  'f.debug_struct("FormulaCanonicalRecipe")\n            .field("text", &self.text)'),
 ('statement extraction', 'self.text\n    }\n}\nimpl fmt::Debug for FormulaCanonicalStatement',
  'self.text + " "\n    }\n}\nimpl fmt::Debug for FormulaCanonicalStatement'),
 ('recipe extraction', 'self.text\n    }\n}\nimpl fmt::Debug for FormulaCanonicalRecipe',
  'self.text + " "\n    }\n}\nimpl fmt::Debug for FormulaCanonicalRecipe'),
]
for name, before, _ in cases:
    assert original.decode().count(before) == 1, (name, 'actual anchor not unique')

def assertion_failure(output):
    parts = output.split(b'\nfailures:\n', 2)
    return len(parts) == 3 and re.search(rb'(?m)^assertion(?:[ :`]|$)', parts[1]) is not None

assert not assertion_failure(b'test assertion_ok ... ok\n\nfailures:\nthread panicked:\nexpect-only\n\nfailures:\nassertion_ok\n')
assert not assertion_failure(b'assertion: compiler output\n')
assert assertion_failure(b'\nfailures:\nthread panicked:\nassertion `left == right` failed\n\nfailures:\ncontract\n')
assert sys.argv[1:] in [[], ['--classifier-only']]
if sys.argv[1:] == ['--classifier-only']:
    print('canonical recipe fault controls:21 unique actual anchors; passing/expect/compiler noise refused')
    sys.exit(0)

work = ROOT / 'target/canonical_recipe_mutations'
work.mkdir(parents=True, exist_ok=True)
env = dict(os.environ, CARGO_HOME=str(ROOT / 'target/cargo-home'), TMPDIR=str(ROOT / 'target/scratch'))
try:
    for index, (name, before, after) in enumerate(cases, 1):
        SOURCE.write_text(original.decode().replace(before, after, 1))
        result = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test', 'formula_canonical_recipe_contract'],
                                cwd=ROOT, env=env, capture_output=True)
        output = result.stdout + result.stderr
        (work / ('fault-%d.log' % index)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output and assertion_failure(output), (name, output.decode())
        assert b'could not compile' not in output, (name, 'compiler refusal is not assertion proof')
        print('canonical recipe fault %d %s:rc=101 actual compiled assertion red' % (index, name), flush=True)
        SOURCE.write_bytes(original)
finally:
    SOURCE.write_bytes(original)
assert SOURCE.read_bytes() == original
print('canonical recipe faults:21 actual compiled assertion reds; source restored exactly')
