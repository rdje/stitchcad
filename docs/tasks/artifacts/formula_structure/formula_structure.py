"""D75: independent fixtures test the actual reference parser/walkers, not copied implementations."""
from pathlib import Path
from fractions import Fraction
import os
import re
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
TOOL = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
WORK = ROOT / 'target/formula_structure'


def load_reference():
    """Load actual definitions, before the census driver reads/runs book examples."""
    shell = TOOL.read_text()
    marker = "<<'PY'\n"
    assert shell.count(marker) == 1, 'reference Python entrypoint changed'
    program = shell.split(marker, 1)[1].rsplit('\nPY', 1)[0]
    marker = 'print("=== formula-language census ===")'
    assert program.count(marker) == 1, 'reference driver boundary changed'
    prefix = program.split(marker, 1)[0]
    namespace = {}
    original_args = sys.argv
    try:
        sys.argv = ['reference', *(['unused'] * 6)]
        exec(compile(prefix, 'reference_formula_language', 'exec'), namespace)
    finally:
        sys.argv = original_args
    contract = (ROOT / 'docs/book/src/spec/formula-language.md').read_text()
    limits = {name: int(value) for name, value in re.findall(
        r'^\| `(max_[a-z_]+)` \| ([0-9]+) \|', contract, re.M)}
    assert limits == {'max_expression_nodes': 256, 'max_recipe_statements': 4096,
                      'max_if_depth': 16, 'max_rational_bits': 128}, 'normative limits changed'
    domains = namespace['scalar_domains'](str(ROOT / 'docs/book/src'))
    evaluator = namespace['Evaluator']([], {}, {}, {}, {}, limits, {}, domains)
    return namespace, evaluator


def conditional(depth):
    result = '1'
    for _ in range(depth):
        result = 'if(dart_intake > dart_intake_max, ' + result + ', 1)'
    return result


def node_expression(nodes):
    # Ordinary calls/unary negations each add one semantic node to a literal.
    return '-' * (nodes - 2) + 'abs(1)'


def contracts():
    namespace, evaluator = load_reference()
    error_type = namespace['FErr']
    # Expected counts come from these explicitly constructed shapes, not from the tested walkers.
    cases = [
        ('literal', '1', 1, 0),
        ('call argument subtree', 'abs(1 + 0)', 4, 0),
        ('variadic argument subtrees', 'max(1 + 0, abs(1), 1 * 1)', 9, 0),
        ('grouping is not a semantic node', '(((abs(1 + 0))))', 4, 0),
        ('square exponent is operator payload', 'abs(1) ^ 2', 3, 0),
        ('nested call/if children', 'abs(if(a, abs(if(b, 1, 2)), 3))', 9, 2),
        ('if in either statically checked branch', 'if(a, abs(if(b, 1, 2)), abs(if(c, 3, 4)))', 12, 2),
        ('255 nodes', node_expression(255), 255, 0),
        ('256 nodes', node_expression(256), 256, 0),
        ('15 if levels inside call', 'abs(' + conditional(15) + ')', 77, 15),
        ('16 if levels inside call', 'abs(' + conditional(16) + ')', 82, 16),
    ]
    for name, source, nodes, depth in cases:
        tree = evaluator.parse(source)
        assert evaluator.count_nodes(tree) == nodes, (name, evaluator.count_nodes(tree), nodes)
        assert evaluator.if_depth(tree) == depth, (name, evaluator.if_depth(tree), depth)
        print('  control:', name, 'nodes=%d depth=%d' % (nodes, depth))
    for name, source, signature in [
        ('257 nodes', node_expression(257), 'expression has 257 nodes, the limit is 256'),
        ('258 call-argument nodes', 'abs(' + ' + '.join(['1'] + ['0'] * 128) + ')',
         'expression has 258 nodes, the limit is 256'),
        ('17 if levels inside call', 'abs(' + conditional(17) + ')',
         'conditional depth is 17, the limit is 16'),
        ('untaken-branch nested call', 'if(a, abs(' + conditional(16) + '), 1)',
         'conditional depth is 17, the limit is 16'),
    ]:
        refused = None
        try:
            evaluator.parse(source)
        except error_type as error:
            refused = error
        assert refused is not None, (name, 'structural form was accepted')
        assert refused.token == 'formula_domain' and refused.msg == signature, (name, refused)
        print('  refusal:', name, 'formula_domain:', signature)
    # Walkers themselves are iterative: an existing AST much deeper than Python's recursion limit.
    tree = ('lit', 'count', Fraction(1))
    for _ in range(5000):
        tree = ('call', 'abs', [tree])
    assert evaluator.count_nodes(tree) == 5001
    assert evaluator.if_depth(tree) == 0
    print('  control: 5001-node prebuilt AST traversed iteratively; not accepted as a valid expression')
    print('formula structural contracts: 16 pass / 0 fail')


def end_to_end():
    WORK.mkdir(exist_ok=True)
    base = 'if(dart_intake > dart_intake_max, 2, 1)'
    cases = [
        ('call_nodes', 'abs(' + ' + '.join(['1'] + ['0'] * 128) + ')',
         'expression has 258 nodes, the limit is 256'),
        ('call_if_depth', 'abs(' + conditional(17) + ')',
         'conditional depth is 17, the limit is 16'),
    ]
    for name, expression, signature in cases:
        book = WORK / name / 'src'
        if book.exists():
            shutil.rmtree(book)
        shutil.copytree(ROOT / 'docs/book/src', book)
        examples = book / 'spec/formula-language/examples.md'
        text = examples.read_text()
        assert text.count('`' + base + '`') == 1, 'example mutation anchor changed'
        examples.write_text(text.replace('`' + base + '`', '`' + expression + '`'))
        environment = dict(os.environ, FORMULA_BOOK=str(book))
        result = subprocess.run(['bash', str(TOOL)], cwd=ROOT, env=environment,
                                capture_output=True, text=True)
        output = result.stdout + result.stderr
        (WORK / (name + '.log')).write_text(output)
        assert result.returncode == 1 and signature in output, (name, result.returncode, output)
        print('  end-to-end:', name, 'rc=1, named structural refusal')
    print('formula structural end-to-end: 2 pass / 0 fail')


if __name__ == '__main__':
    contracts()
    if '--contracts-only' not in sys.argv:
        end_to_end()
