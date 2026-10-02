"""Actual worked/refusal populations; authored bytes, reference syntax only, no execution."""
from pathlib import Path
import runpy

ROOT = Path(__file__).resolve().parents[4]
HERE = ROOT / 'docs/tasks/artifacts/formula_structure'
context = runpy.run_path(str(HERE / 'recipe_byte_contract.py'))
statement_bytes = context['statement_bytes']
recipe_bytes = context['recipe_bytes']
reference = context['reference']
book = (ROOT / 'docs/book/src/spec/formula-language/examples.md').read_text()

def rows(name):
    return [line.split('\t') for line in (HERE / name).read_text().splitlines()
            if line and not line.startswith('#')]

section = ''
worked = []
refusals = []
for line in book.splitlines():
    if line.startswith('## '):
        section = line
    if not line.startswith('| `'):
        continue
    cells = [cell.strip() for cell in line.split('|')]
    if section.startswith('## 2.'):
        name, kind, expression = [cell.strip('`') for cell in cells[1:4]]
        worked.append('let %s: %s = %s' % (name, kind, expression))
    elif section.startswith('## 3.'):
        worked.append(cells[1].strip('`'))
    elif section.startswith('## 4.'):
        refusals.append((cells[1].split('`')[1], cells[2].strip('`')))

authored = rows('canonical_worked_recipe_cases.tsv')
assert len(authored) == len(worked) == 21
assert worked == [source for source, _ in authored], 'actual complete worked population'
assert sum(source.startswith('let ') for source in worked) == 17
assert sum(source.startswith('assert ') for source in worked) == 4
tokens = []
operands = 0
for source, expected in authored:
    assert statement_bytes(source) == expected, ('worked identity', source)
    operands += len(context['parsed'])
    tokens.extend(reference.tokenize(source)[0])
assert reference.tokenize('\n'.join(worked))[0] == tokens
assert operands == 25
assert recipe_bytes([statement_bytes(source) for source in worked]) == recipe_bytes(
    [expected for _, expected in authored])

stages = rows('recipe_refusal_stage_cases.tsv')
assert len(stages) == len(refusals) == 13
assert refusals == [(source, diagnostic) for source, _, diagnostic, _ in stages]
counts = {'syntax': 0, 'semantic': 0}
for source, stage, diagnostic, expected in stages:
    assert stage in counts
    statement = source if source.startswith('let ') else 'let candidate: length = ' + source
    if stage == 'syntax':
        try:
            statement_bytes(statement)
        except Exception as error:
            assert error.__class__.__name__ == 'FErr' and error.token == diagnostic, (
                source, 'actual reference syntax family', error)
        else:
            raise AssertionError((source, 'syntax refusal unexpectedly accepted'))
        assert expected == '-'
    else:
        assert statement_bytes(statement) == expected, ('semantic checks are deferred', source)
    counts[stage] += 1
assert counts == {'syntax': 3, 'semantic': 10}
print('whole input review:actual17 bindings/four assertions/21 statements/25 operands; '
      '13 refusal examples split3 syntax/10 deferred semantic; authored bytes/reference syntax agree')
print('whole input review:ordered token coverage; semantics trapped; no independent whole-recipe parser')
