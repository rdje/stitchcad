"""D75: independent fixtures test the actual reference parser/walkers, not copied implementations."""
from pathlib import Path
from fractions import Fraction
import os
import re
import runpy
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
TOOL = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
WORK = ROOT / 'target/formula_structure'
BOOK = ROOT / 'docs/book/src'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
BOOT_INPUTS = (Path(__file__), PROFILE_SOURCE, TOOL,
               ROOT / 'docs/tasks/artifacts/formula_structure/static_signature_contract.py')
REFERENCE_INPUTS = (*BOOT_INPUTS, *(BOOK / name for name in (
    'spec/formula-language.md', 'spec/formula-language/grammar.md',
    'spec/formula-language/examples.md', 'spec/reference-skirt.md', 'spec/units-and-tolerances.md')))
LABELS = ('call_nodes', 'call_if_depth')
try:
    PROFILE['plan_producer'](ROOT, sources=BOOT_INPUTS)
except (PROFILE['Refusal'], OSError) as error:
    print('local-environment: REFUSED — ' + str(error), file=sys.stderr)
    raise SystemExit(2) from None


def load_reference():
    """Load actual definitions/catalogs; no fixture execution or copied wanted rules."""
    PROFILE['plan_producer'](ROOT, sources=REFERENCE_INPUTS)
    helper = runpy.run_path(str(ROOT / 'docs/tasks/artifacts/formula_structure/static_signature_contract.py'))
    namespace, evaluator = helper['load_reference']()
    contract = (ROOT / 'docs/book/src/spec/formula-language.md').read_text()
    limits = {name: int(value) for name, value in re.findall(
        r'^\| `(max_[a-z_]+)` \| ([0-9]+) \|', contract, re.M)}
    assert limits == {'max_expression_nodes': 256, 'max_recipe_statements': 4096,
                      'max_if_depth': 16, 'max_rational_bits': 128}, 'normative limits changed'
    assert evaluator.limits == limits, 'actual catalog loader limits drift'
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



def tree_plan(base, *, output=False):
    """Walk only inspected directories; do not follow links or enter another repository."""
    relative = PROFILE['producer_path'](ROOT, base)
    if output:
        PROFILE['producer_output_path'](ROOT, base)
    PROFILE['directory'](ROOT, relative, missing_ok=output)
    if not base.exists():
        return (), ()
    directories, files = [base], []
    for folder in directories:
        PROFILE['directory'](ROOT, folder.relative_to(ROOT).as_posix())
        for path in sorted(folder.iterdir()):
            PROFILE['require'](not path.is_symlink(), 'symlink copied-book entry')
            if path.is_dir():
                directories.append(path)
            else:
                files.append(path)
    PROFILE['plan_producer'](ROOT, **{'output_files' if output else 'sources': tuple(files)})
    return tuple(directories), tuple(files)


def source_plan():
    """Inspect the entire copied book and its loaders without creating or activating anything."""
    directories, files = tree_plan(BOOK)
    sources = (*BOOT_INPUTS, *files)
    PROFILE['plan_producer'](ROOT, sources=sources)
    return directories, files, sources


def consumer_inputs(environment=None):
    """Match the shell's effective selectors and lexical reference resolution before effects."""
    environment = os.environ if environment is None else environment
    book = Path(environment.get('FORMULA_BOOK') or BOOK)
    book = book if book.is_absolute() else ROOT / book
    parts = Path(environment.get('FORMULA_PARTS_DIR') or book / 'spec/formula-language')
    selected = tuple(Path(environment.get(name) or default) for name, default in (
        ('FORMULA_CONTRACT', book / 'spec/formula-language.md'),
        ('FORMULA_GRAMMAR', parts / 'grammar.md'),
        ('FORMULA_EXAMPLES', parts / 'examples.md'),
        ('FORMULA_FIXTURE', book / 'spec/reference-skirt.md')))
    # The shell runs in ROOT; keep a relative caller selector relative to that same root.
    selected = tuple(path if path.is_absolute() else ROOT / path for path in selected)
    parts = parts if parts.is_absolute() else ROOT / parts
    PROFILE['directory'](ROOT, PROFILE['producer_path'](ROOT, parts))
    PROFILE['plan_producer'](ROOT, sources=selected)
    references = []
    for path in selected[:3]:
        for _, target in re.findall(r'\[([^\]]*)\]\(([^)]+)\)', path.read_text()):
            if target.startswith(('http://', 'https://', '#')):
                continue
            # Concatenation intentionally matches the public resolver, including a leading slash.
            references.append(Path(os.path.normpath(str(path.parent) + '/' + target.split('#')[0])))
    sources = tuple(dict.fromkeys((*selected, *sorted(parts.glob('*.md')), *references)))
    PROFILE['plan_producer'](ROOT, sources=sources)
    return parts, sources


def fixture_plan(labels, *, activate=True):
    """Inspect complete copies and old fixtures before preparing local child stores."""
    PROFILE['require'](set(labels) <= set(LABELS), 'unowned input fixture label')
    book_directories, book_files, sources = source_plan()
    parts, selected_inputs = consumer_inputs(dict(os.environ, FORMULA_BOOK=str(BOOK)))
    sources = tuple(dict.fromkeys((*sources, *selected_inputs)))
    file_names = {path.relative_to(BOOK) for path in book_files}
    directory_names = {path.relative_to(BOOK) for path in book_directories}
    bases = tuple(WORK / label / 'src' for label in labels)
    PROFILE['require'](not any(parts == base or parts.is_relative_to(base) for base in bases),
                       'copied-book output aliases selected input directory')
    directories = (WORK, *(base / name for base in bases for name in sorted(directory_names)))
    outputs = (*(base / name for base in bases for name in sorted(file_names)),
               *(WORK / (label + '.log') for label in labels))
    PROFILE['require'](not set(outputs) & set(sources), 'copied-book output aliases selected input')
    for base in bases:
        existing_directories, existing_files = tree_plan(base, output=True)
        PROFILE['require'](all(path.relative_to(base) in directory_names for path in existing_directories)
                           and all(path.relative_to(base) in file_names for path in existing_files),
                           'unexpected copied-book fixture entry')
    plan = dict(directories=directories, output_files=outputs, sources=sources)
    PROFILE['plan_producer'](ROOT, **plan)
    return PROFILE['prepare_producer'](ROOT, **plan) if activate else None


def copy_book(label):
    """Recheck the complete selected replacement immediately before deleting and copying."""
    fixture_plan((label,), activate=False)
    book = WORK / label / 'src'
    if book.exists():
        shutil.rmtree(book)
    shutil.copytree(BOOK, book)
    # These files now become the compiled consumer's inputs and may be edited as owned outputs.
    _, files = tree_plan(book, output=True)
    PROFILE['plan_producer'](ROOT, sources=(TOOL, *files), output_files=files)
    return book


def end_to_end():
    environment = fixture_plan(LABELS)
    WORK.mkdir(exist_ok=True)
    base = 'if(dart_intake > dart_intake_max, 2, 1)'
    cases = [
        ('call_nodes', 'abs(' + ' + '.join(['1'] + ['0'] * 128) + ')',
         'expression has 258 nodes, the limit is 256'),
        ('call_if_depth', 'abs(' + conditional(17) + ')',
         'conditional depth is 17, the limit is 16'),
    ]
    for name, expression, signature in cases:
        book = copy_book(name)
        examples = book / 'spec/formula-language/examples.md'
        text = examples.read_text()
        assert text.count('`' + base + '`') == 1, 'example mutation anchor changed'
        examples.write_text(text.replace('`' + base + '`', '`' + expression + '`'))
        source_plan()
        consumer_inputs(dict(environment, FORMULA_BOOK=str(book)))
        result = subprocess.run(['bash', str(TOOL)], cwd=ROOT, env=dict(environment, FORMULA_BOOK=str(book)),
                                capture_output=True, text=True)
        output = result.stdout + result.stderr
        (WORK / (name + '.log')).write_text(output)
        assert result.returncode == 1 and signature in output, (name, result.returncode, output)
        print('  end-to-end:', name, 'rc=1, named structural refusal')
    print('formula structural end-to-end: 2 pass / 0 fail')


if __name__ == '__main__':
    try:
        contracts()
        if '--contracts-only' not in sys.argv:
            end_to_end()
    except (PROFILE['Refusal'], OSError) as error:
        print('local-environment: REFUSED — ' + str(error), file=sys.stderr)
        raise SystemExit(2) from None
