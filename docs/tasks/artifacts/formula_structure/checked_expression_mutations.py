"""Exclusive real expression-checker faults; body assertions only, exact source restoration."""
from pathlib import Path
import os
import runpy
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-core/src/recipe/checked.rs'
CONTRACT = ROOT / 'crates/sc-core/tests/formula_checked_expression_contract.rs'
SOURCES = (SOURCE, CONTRACT)
WORK = ROOT / 'target/checked_expression_mutations'
assert sys.argv[1:] in [[], ['--classifier-only']]
CLASSIFIER_ONLY = sys.argv[1:] == ['--classifier-only']
CASES = (
    ('literal kind changed', (('FormulaLiteralKind::Angle => FormulaKind::Angle,',
                              'FormulaLiteralKind::Angle => FormulaKind::Length,'),)),
    ('result kind changed', (('// Computed results never acquire the direct-name tolerance role.\n                    let operand = O::Value(kind);',
                             'let operand = O::Value(FormulaKind::Count);'),)),
    ('direct class lost', ((')) => O::Tolerance(class),',
                           ')) => O::Value(FormulaKind::Length),'),)),
    ('computed class forged', (('// Computed results never acquire the direct-name tolerance role.\n                    let operand = O::Value(kind);',
                               'let operand = if kind == FormulaKind::Length { '
                               'O::Tolerance(super::FormulaToleranceName::Geometric) } '
                               'else { O::Value(kind) };'),)),
    # Seed only this mutant's new bookkeeping so the original skipped-else fault
    # reaches its external assertions; visited nodes overwrite the placeholders.
    ('else branch replaced', (('pending.push(Action::Enter(*else_branch));',
                              'pending.push(Action::Enter(*then_branch));'),
                             ('let mut certificates = vec![None; self.nodes.len()];',
                              'let mut certificates = vec![Some(NodeCertificate { '
                              'operand: O::Value(FormulaKind::Count), dependency: None, '
                              'builtin: None }); self.nodes.len()];'))),
    ('conditional child order', (('pending.push(Action::Enter(*then_branch));\n'
                                 '                        pending.push(Action::Enter(*condition));',
                                 'pending.push(Action::Enter(*condition));\n'
                                 '                        pending.push(Action::Enter(*then_branch));'),)),
    ('call argument order', (('for child in arguments.iter().rev() {',
                             'for child in arguments.iter() {'),)),
    ('callee checked after children', (
        ('let builtin = lookup_call_name(name).map_err(|error| {\n'
         '                            refuse(index, FormulaExpressionCheckRefusal::Call(error))\n'
         '                        })?;',
         'let builtin = lookup_call_name(name).unwrap_or(FormulaBuiltin::Hypot);'),
        ('let operands = kinds.split_off(kinds.len() - arity);',
         'if let NormalizedData::Call { name, .. } = &self.nodes[index].data {\n'
         '                        lookup_call_name(name).map_err(|error| '
         'refuse(index, FormulaExpressionCheckRefusal::Call(error)))?;\n'
         '                    }\n'
         '                    let operands = kinds.split_off(kinds.len() - arity);'))),
    ('immediate operand order', (('let operands = kinds.split_off(kinds.len() - arity);',
                                 'let mut operands = kinds.split_off(kinds.len() - arity); '
                                 'operands.reverse();'),)),
    ('dependency prefix only', (('dependencies.push(FormulaNameDependency {\n'
                                '                            span: self.nodes[index].span,\n'
                                '                            declaration,\n'
                                '                        });',
                                'if dependencies.is_empty() { dependencies.push(FormulaNameDependency {\n'
                                '                            span: self.nodes[index].span,\n'
                                '                            declaration,\n'
                                '                        }); }'),)),
    ('dependency span forged', (('span: self.nodes[index].span,\n'
                                '                            declaration,',
                                'span: self.nodes[self.root].span,\n'
                                '                            declaration,'),)),
    ('refusal span forged', (('span: self.nodes[index].span,\n            refusal,',
                             'span: self.nodes[self.root].span,\n            refusal,'),)),
    ('wanted rows discarded', (('self.operation.signatures()', '&[]'),)),
    ('arc hint lost', (('operation.arc_length_hint(left.kind(), right.kind())', 'false'),)),
    ('debug discloses expression', (('.field("token", &self.token())',
                                   '.field("token", &self.token()).field("expression", '
                                   '&self.expression.canonical_form().as_str())'),)),
    ('dimension token changed', (('Self::Dimension(_) => "formula_dimension",',
                                 'Self::Dimension(_) => "formula_domain",'),)),
)
OUTPUT_FILES = (() if CLASSIFIER_ONLY else
                tuple(WORK / ('fault-%d.log' % index) for index in range(1, len(CASES) + 1)) +
                (WORK / 'restored.log',))
MUTABLE_SOURCES = () if CLASSIFIER_ONLY else (SOURCE,)
runpy.run_path(str(ROOT / 'scripts/local_environment.py'))['enter_producer'](
    ROOT, directories=(WORK,), output_files=OUTPUT_FILES, sources=SOURCES, mutable_sources=MUTABLE_SOURCES)
original = SOURCE.read_bytes()
WORK.mkdir(parents=True, exist_ok=True)
for name, steps in CASES:
    for before, _ in steps:
        assert original.decode().count(before) == 1, (name, 'fault anchor not unique')

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
    print('checked expression faults:16 unique real body transformations; current thread macros; nonthread/expect/compiler/name noise refused')
    sys.exit(0)
environment = dict(os.environ)
try:
    for index, (name, steps) in enumerate(CASES, 1):
        modified = original.decode()
        for before, after in steps:
            modified = modified.replace(before, after, 1)
        SOURCE.write_text(modified)
        result = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test',
                                 'formula_checked_expression_contract'],
                                cwd=ROOT, env=environment, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('fault-%d.log' % index)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output, (name, output.decode())
        assert assertion_failure(output) and b'could not compile' not in output, (name, output.decode())
        print('checked expression fault %d %s:rc=101 actual compiled body assertion red' % (index, name), flush=True)
        SOURCE.write_bytes(original)
finally:
    SOURCE.write_bytes(original)
    restored = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test',
                               'formula_checked_expression_contract'],
                              cwd=ROOT, env=environment, capture_output=True)
    (WORK / 'restored.log').write_bytes(restored.stdout + restored.stderr)
    assert restored.returncode == 0, ('restored checked_expression artifact failed', restored.stderr.decode())
    assert SOURCE.read_bytes() == original
print('checked expression faults:16 actual compiled assertion reds; source/current artifact restored')
