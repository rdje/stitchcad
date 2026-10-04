"""Independent shared cases through actual reference and compiled public Rust APIs."""
from itertools import product
from pathlib import Path
import json
import os
import re
import runpy
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
HERE = ROOT / 'docs/tasks/artifacts/formula_structure'
RULES = runpy.run_path(str(HERE / 'dimension_payload_contract.py'))
STATIC = runpy.run_path(str(HERE / 'static_review_contract.py'))
KINDS, ARITH, CALLS = RULES['KINDS'], RULES['ARITH'], RULES['CALL_ROWS']
SOURCE = RULES['SOURCE']
DRIVER = HERE / 'static_coupled_driver.rs'
WORK = ROOT / 'target/static_coupled_contract'
WORK.mkdir(parents=True, exist_ok=True)
_, _, worked_env = STATIC['load']()
CATALOG = [('v_' + kind, kind, 'geometry' if kind in ('point', 'edge') else 'parameter') for kind in KINDS]
CATALOG += [(name, entry['kind'], entry['origin']) for name, entry in worked_env.items()]
CATALOG += [(name, 'length', 'parameter') for name in STATIC['ORDINARY_NAMES']]
INITIAL = {name: (kind, origin, None) for name, kind, origin in CATALOG}
assert len(INITIAL) == len(CATALOG), 'coupled authored initial name collision'
for name, kind in RULES['RESERVED'].items():
    INITIAL[name] = (kind, 'tolerance' if name.startswith('eps_') else 'size', None)
CASES = []


def deps(source, names):
    # Independent lexical occurrence oracle for these authored sources; callees in the catalog
    # are never accepted as ordinary calls. Kind/unit/header words are outside RHS slices.
    return ','.join('%s:%s:%s' % (name, names[name][1], names[name][2] if names[name][2] is not None else 'none')
                    for name in re.findall(r'[a-z][a-z0-9_]*', source) if name in names)


def dim(operation, kinds, rows, roles=None):
    roles = roles if roles is not None else (None,) * len(kinds)
    encoded = sorted(','.join(args) + '|%d|' % variadic + result for args, variadic, result in rows)
    hint = operation == '*' and set(kinds) == {'angle', 'length'}
    return 'DIM:%s;%s;%s;%s;%d' % (operation, ','.join(kinds),
        ','.join(role or 'none' for role in roles), '#'.join(encoded), hint)


def add_expr(source, kind=None, token='formula_dimension', payload=None):
    if kind is not None:
        expected = ('OK', kind, deps(source, INITIAL))
    else:
        expected = ('ERR', token, payload or '-')
    CASES.append(('E', source, expected))


def add_recipe(source, kinds=None, token='formula_dimension', payload=None):
    if kinds is None:
        CASES.append(('R', source, ('ERR', token, payload or '-')))
        return
    # Authored statement chunks are supplied with their independently expected root kind(s).
    names, summaries = dict(INITIAL), []
    for ordinal, (chunk, kind) in enumerate(kinds, 1):
        header, rhs = chunk.split('=', 1)
        role, rest = header.strip().split(None, 1)
        name = rest.split(':', 1)[0].strip()
        if role == 'let':
            summaries.append('%d:let:%s:%s' % (ordinal, kind, deps(rhs, names)))
            names[name] = (kind, 'recipe', ordinal)
        else:
            left, right = rhs.split('==')
            summaries.append('%d:assert:%s:%s~%s' % (ordinal, ','.join(kind), deps(left, names), deps(right, names)))
    CASES.append(('R', source, ('OK', str(len(kinds)), ';'.join(summaries))))


for kind in KINDS:
    add_expr('-v_' + kind, kind if kind in KINDS[:4] else None,
             payload=dim('-', (kind,), RULES['UNARY']['-']))
    add_expr('v_' + kind + '^2', {'length': 'area', 'ratio': 'ratio', 'count': 'count'}.get(kind),
             payload=dim('^2', (kind,), RULES['UNARY']['^2']))
for operation, rows in RULES['BINARY'].items():
    relation = RULES['expanded'](rows)
    for kinds in product(KINDS, repeat=2):
        add_expr(('v_%s' + operation + 'v_%s') % kinds, relation.get(kinds), payload=dim(operation, kinds, rows))
for name, rows in CALLS.items():
    if name in ('if', 'within'):
        continue
    relation = RULES['expanded'](rows)
    variadic = name in ('min', 'max')
    for count in ((1, 2, 3, 4) if variadic else (len(rows[0][0]),)):
        for kinds in product(KINDS, repeat=count):
            kind = (kinds[0] if len(set(kinds)) == 1 and kinds[0] in ARITH else None) if variadic else relation.get(kinds)
            add_expr(name + '(' + ','.join('v_' + k for k in kinds) + ')', kind, payload=dim(name, kinds, rows))
    add_expr(name + '()', token='formula_parse')
    if not variadic:
        for count in sorted({max(1, len(rows[0][0]) - 1), len(rows[0][0]) + 1} - {len(rows[0][0])}):
            add_expr(name + '(' + ','.join(['v_length'] * count) + ')', payload=dim(name, ('length',) * count, rows))
for kinds in product(KINDS, repeat=3):
    kind = kinds[1] if kinds[0] == 'boolean' and kinds[1] == kinds[2] and kinds[1] in ARITH else None
    add_expr('if(' + ','.join('v_' + k for k in kinds) + ')', kind, payload=dim('if', kinds, CALLS['if']))
    add_expr('within(' + ','.join('v_' + k for k in kinds) + ')', payload=dim('within', kinds, CALLS['within']))
for tolerance, third_kind in RULES['RESERVED'].items():
    for left, right in product(KINDS, repeat=2):
        roles = (None, None, tolerance if tolerance in RULES['TOLERANCES'] else None)
        kind = 'boolean' if left == right and left in ARITH and roles[2] else None
        add_expr('within(v_%s,v_%s,%s)' % (left, right, tolerance), kind,
                 payload=dim('within', (left, right, third_kind), CALLS['within'], roles))
for third in ('v_length', '1 mm', '(eps_num+eps_num)', 'abs(eps_num)', '-eps_geo'):
    add_expr('within(v_length,v_length,' + third + ')', payload=dim('within', ('length',) * 3, CALLS['within']))
for name in RULES['TOLERANCES']:
    add_expr('within(v_length,v_length,((' + name + ')))', 'boolean')
for declared, actual in product(KINDS[:6], KINDS):
    chunk = 'let saved:%s=v_%s' % (declared, actual)
    add_recipe(chunk, [(chunk, actual)] if declared == actual else None,
               payload='BIND:%s:%s:%s' % (declared, actual, declared))
for tolerance in RULES['TOLERANCES']:
    for left, right in product(KINDS, repeat=2):
        chunk = 'assert check:%s=v_%s==v_%s' % (tolerance, left, right)
        add_recipe(chunk, [(chunk, (left, right))] if left == right and left in ARITH else None,
                   payload=dim('==', (left, right), RULES['BINARY']['==']))
# Multiple simultaneous static defects: caller/callee domain, child order and untaken branches.
for source, token, name in (
    ('if(1,missing_left,v_length)', 'formula_unbound_name', 'missing_left'),
    ('if(missing_condition,missing_left,missing_right)', 'formula_unbound_name', 'missing_condition'),
    ('if(1,v_length,missing_right)', 'formula_unbound_name', 'missing_right'),
    ('v_point+missing_right', 'formula_unbound_name', 'missing_right'),
    ('within(v_length,missing_right,size_count)', 'formula_unbound_name', 'missing_right'),
    ('within(v_length,v_length,missing_class)', 'formula_unbound_name', 'missing_class'),
):
    add_expr(source, token=token, payload='NAME:%s:%s' % (name, ','.join(sorted(STATIC['BASE']['ORIGINS']))))
for callee, token in {**STATIC['ENVELOPE'], 'loop': 'formula_unbound_name', 'v_length': 'formula_unbound_name'}.items():
    nurbs = token == 'env_nurbs'
    searched = 'envelope' if token.startswith('env_') else 'envelope,builtin_catalog'
    alternatives = 'line_segment,circular_arc,cubic_bezier' if nurbs else 'ordered_construction_recipe' if token.startswith('env_') else ''
    add_expr(callee + '(missing)', token=token, payload='CALL:%s:%s:%s' % (callee, searched, alternatives))
for name in STATIC['ORDINARY_NAMES']:
    add_expr(name, 'length')
for source, token in STATIC['OBSERVED']:
    # Exact payloads are checked for resolved static names/callees above; grammar-stage refusals
    # have distinct public source-bearing forms rather than a fabricated common payload.
    add_expr(source, token=token)
for source, kind in [('v_length/0', 'length'), ('sqrt(-v_area)', 'length'), ('if(v_boolean,1 mm/0,1 mm)', 'length'),
                     ('round_to(v_ratio,0.0)', 'ratio'), ('v_angle/v_count', 'angle')]:
    add_expr(source, kind)
for name in ('min', 'max'):
    for kind in ARITH:
        add_expr(name + '(' + ','.join(['v_' + kind] * 255) + ')', kind)
        add_expr(name + '(' + ','.join(['v_' + kind] * 256) + ')', token='formula_domain')
for prefix in ('let bad:length=missing\n', 'let bad:length=1.0\n', 'let eps_num:length=missing\n'):
    add_recipe(prefix + 'assert late:eps_chord=1 mm==1 mm', token='formula_parse')
for raw in ('point', 'edge', 'unknown'):
    add_recipe('let invalid:%s=missing' % raw, token='formula_dimension')
# Complete actual book population is compared with independently retained source rows and kinds.
rows = [line.split('\t') for line in (HERE / 'canonical_worked_recipe_cases.tsv').read_text().splitlines()
        if line and not line.startswith('#')]
assert len(rows) == 21 and len(STATIC['WORKED']) == 17, 'coupled authored worked coverage'
worked = [(row[0], kind) for row, (_, kind) in zip(rows[:17], STATIC['WORKED'])]
worked += [(row[0], ('length', 'length')) for row in rows[17:]]
worked_source = '\n'.join(chunk for chunk, _ in worked)
add_recipe(worked_source, worked)
for source, token, kind in STATIC['REFUSALS']:
    chunk = source if source.startswith('let ') else 'let candidate:%s=%s' % (kind or 'length', source)
    add_recipe(worked_source + '\n' + chunk, worked + [(chunk, kind)] if token is None else None, token=token)

chain = [('let first:length=v_length', 'length'), ('assert check:eps_fmt=first==v_length', ('length', 'length')),
         ('let second:length=if(v_boolean,first,first)', 'length'), ('let last:length=second+first', 'length')]
add_recipe(' \t' + '\r\n'.join(chunk for chunk, _ in chain), chain)
add_recipe('let saved:length=saved', token='formula_unbound_name', payload='NAME:saved:' + ','.join(sorted(STATIC['BASE']['ORIGINS'])))
add_recipe('let saved:length=later\nlet later:length=v_length', token='formula_unbound_name', payload='NAME:later:' + ','.join(sorted(STATIC['BASE']['ORIGINS'])))
add_recipe('let v_length:length=missing', token='formula_ambiguous_name', payload='COLLISION:v_length:parameter,recipe')
add_recipe('let first:length=v_length\nassert check:eps_num=first==first\nlet first:length=missing', token='formula_rebinding', payload='COLLISION:first:recipe,recipe')


def documentation(replacement=None):
    obsolete = {
        'formula-static-validation.md': 'complete expression/recipe acceptance remains the next two stages.',
        'formula-wanted-signatures.md': 'Current statement context remains .5b.3c.3',
        'implementation-status.md': 'whole-recipe preflight remains pending.',
    }
    for name, phrase in obsolete.items():
        text = (ROOT / 'docs/book/src/annexes' / name).read_text()
        if replacement and replacement[0] == name:
            before, after = replacement[1:]
            assert text.count(before) == 1, ('D149 copied-text actual anchor', name)
            text = text.replace(before, after)
        assert phrase not in ' '.join(text.split()), ('D151 ambiguous proof status' if name == 'implementation-status.md' else 'D149 obsolete implemented status', name)
        if name != 'implementation-status.md':
            assert 'check-the-actual-current-statement' in text, ('D149 missing implemented source route', name)
    return len(obsolete)


def ref_dimension(arguments, hint):
    rows = [(row['operands'], row['variadic'], row['result']) for row in arguments['wanted_signatures']]
    result = dim(arguments['operation'], arguments['operand_kinds'], rows, arguments['operand_tolerances'])
    assert result.endswith(str(int(hint))), 'coupled reference actual hint'
    return result


def check_reference(replacement=None):
    ns, reference = STATIC['BASE']['LOADER']['load_reference'](replacement)
    declaration = STATIC['Declaration']
    reference.reserved = {name: STATIC['BASE']['ReservedKind'](kind) for name, kind in RULES['RESERVED'].items()}
    env = {name: declaration(kind, origin) for name, kind, origin in CATALOG}
    before = dict(env)
    for method in ('evaluate', '_evaluate', 'value_of_name', 'resolve_geometry', 'call', 'stored'):
        def trapped(*args, **kwargs):
            raise AssertionError('coupled reference numerical/provider/state read')
        setattr(reference, method, trapped)
    read_names = []
    original_read = reference.kind_of_name
    def read(name, metadata):
        result = original_read(name, metadata)
        read_names.append(name)
        return result
    reference.kind_of_name = read
    for index, (mode, source, expected) in enumerate(CASES):
        read_names.clear()
        try:
            if mode == 'E':
                kind = reference.infer(reference.parse(source), env)
                assert expected[0] == 'OK', ('coupled reference accepted refusal', index, source, expected, kind)
                actual = ('OK', kind, deps(source, INITIAL))
                assert read_names == [entry.split(':', 1)[0] for entry in expected[2].split(',') if entry], ('coupled reference ordered reads', index, source, read_names, expected)
            else:
                plan = reference.preflight(source, env.items())
                assert expected[0] == 'OK', ('coupled reference accepted recipe refusal', index, source, expected)
                actual = ('OK', str(len(plan)), expected[2])
                # Independently authored root/header kinds, not merely a copied expected string.
                records = expected[2].split(';') if plan else []
                assert len(records) == len(plan), ('coupled reference complete plan', index)
                wanted_reads = [entry.split(':', 1)[0] for record in records
                                for entry in record.split(':', 3)[3].replace('~', ',').split(',') if entry]
                assert read_names == wanted_reads, ('coupled reference ordered recipe reads', index, source, read_names, wanted_reads)
                local = dict(env)
                for (_, _, checked), record in zip(plan, records):
                    if checked[0] == 'let':
                        assert checked[2] == record.split(':')[2], ('coupled reference binding kind', index)
                        local[checked[1]] = declaration(checked[2], 'recipe')
                    else:
                        got = (reference.infer(checked[3], local), reference.infer(checked[4], local))
                        assert ','.join(got) == record.split(':')[2], ('coupled reference assertion kinds', index)
        except ns['FErr'] as error:
            args = error.arguments
            payload = '-'
            if 'operand_kinds' in args:
                payload = ref_dimension(args, "an arc's length is arc_length(angle, radius)" in error.msg)
            elif args.get('diagnostic_scope') == 'binding_kind':
                payload = 'BIND:%s:%s:%s' % (args['declared_kind'], args['expression_kind'], args['wanted_kinds'][0])
            elif error.token in ('formula_ambiguous_name', 'formula_rebinding'):
                origins = args.get('origins') or (args.get('prior_source', args.get('reserved_source', {})).get('origin'), args['attempted_source']['origin'])
                payload = 'COLLISION:%s:%s' % (args['name'], ','.join(origins))
            elif 'name' in args and 'origins_searched' in args:
                if args.get('lookup_scope') == 'formula_call':
                    alternatives = args.get('supported_curve_set', ()) if error.token == 'env_nurbs' else (args['recipe_alternative'],) if error.token == 'env_sketch_constraints' else ()
                    payload = 'CALL:%s:%s:%s' % (args['name'], ','.join(args['origins_searched']), ','.join(alternatives))
                else:
                    payload = 'NAME:%s:%s' % (args['name'], ','.join(sorted(args['origins_searched'])))
            actual = ('ERR', error.token, payload)
        assert actual[:2] == expected[:2] and (expected[2] == '-' or actual[2] == expected[2]), ('coupled reference oracle', index, source, expected, actual)
        assert env == before, 'coupled reference caller namespace changed'
    return len(CASES)


def build_and_check(label="baseline"):
    output_dir = WORK / label
    output_dir.mkdir(parents=True, exist_ok=True)
    environment = dict(os.environ, CARGO_HOME=str(ROOT / 'target/cargo-home'), CARGO_TARGET_DIR=str(ROOT / 'target'), TMPDIR=str(ROOT / 'target/scratch'))
    built = subprocess.run(['cargo', 'build', '-p', 'sc-core', '--message-format=json'], cwd=ROOT, env=environment, capture_output=True, text=True)
    (output_dir / 'cargo-artifacts.json').write_text(built.stdout)
    (output_dir / 'cargo.log').write_text(built.stderr)
    assert built.returncode == 0, ('coupled Cargo build refused', built.stderr)
    libraries, directories = set(), set()
    for line in built.stdout.splitlines():
        message = json.loads(line)
        if message.get('reason') != 'compiler-artifact':
            continue
        directories.update(Path(path).parent for path in message['filenames'] if path.endswith(('.rlib', '.rmeta')))
        if message['target']['name'] == 'sc_core' and message['target']['kind'] == ['lib']:
            libraries.update(Path(path) for path in message['filenames'] if path.endswith('.rlib'))
    assert len(libraries) == 1 and directories, 'coupled current compiler artifact coverage'
    library = libraries.pop()
    assert all(path.is_relative_to(ROOT / 'target') for path in [library, *directories]), 'coupled foreign build output'
    search = [argument for path in sorted(directories) for argument in ('-L', 'dependency=' + str(path))]
    compiled = subprocess.run(['rustc', '--edition=2021', '-Dwarnings', '--extern', 'sc_core=' + str(library), *search,
                               str(DRIVER), '-o', str(output_dir / 'driver')], cwd=ROOT, env=environment, capture_output=True, text=True)
    (output_dir / 'rustc.log').write_text(compiled.stderr)
    assert compiled.returncode == 0, ('coupled driver compilation refused; never body proof', compiled.stderr)
    inputs = ';'.join(':'.join(entry) for entry in CATALOG) + '\n'
    inputs += ''.join('%d\t%s\t%s\n' % (index, mode, source.encode().hex()) for index, (mode, source, _) in enumerate(CASES))
    (output_dir / 'cases.hex.tsv').write_text(inputs)
    actual = subprocess.run([str(output_dir / 'driver')], input=inputs, cwd=ROOT, env=environment, capture_output=True, text=True)
    (output_dir / 'driver.tsv').write_text(actual.stdout)
    (output_dir / 'driver.log').write_text(actual.stderr)
    assert actual.returncode == 0, ('coupled public driver body failed', actual.stderr)
    lines = actual.stdout.splitlines()
    assert len(lines) == len(CASES), ('coupled product complete output', len(lines), len(CASES))
    for index, (line, (_, source, expected)) in enumerate(zip(lines, CASES)):
        fields = line.split('\t')
        assert len(fields) == 4 and fields[0] == str(index), ('coupled product identity/order', index, line)
        result = tuple(fields[1:])
        assert result[:2] == expected[:2] and (expected[2] == '-' or result[2] == expected[2]), ('coupled product oracle', index, source, expected, result)
    return len(CASES)


RUST_FAULTS = (
    ('result kind', 'crates/sc-core/src/recipe/checked.rs',
     '// Computed results never acquire the direct-name tolerance role.\n                    kinds.push(O::Value(kind));',
     'kinds.push(O::Value(FormulaKind::Count));', 1),
    ('direct tolerance role', 'crates/sc-core/src/recipe/checked.rs',
     ')) => O::Tolerance(class),', ')) => O::Value(FormulaKind::Length),', 1),
    ('computed role forged', 'crates/sc-core/src/recipe/checked.rs',
     '// Computed results never acquire the direct-name tolerance role.\n                    kinds.push(O::Value(kind));',
     'kinds.push(if kind == FormulaKind::Length { O::Tolerance(super::FormulaToleranceName::Geometric) } else { O::Value(kind) });', 1),
    ('dependency prefix', 'crates/sc-core/src/recipe/checked.rs',
     'dependencies.push(FormulaNameDependency {\n                            span: self.nodes[index].span,\n                            declaration,\n                        });',
     'if dependencies.is_empty() { dependencies.push(FormulaNameDependency {\n                            span: self.nodes[index].span,\n                            declaration,\n                        }); }', 1),
    ('call argument order', 'crates/sc-core/src/recipe/checked.rs',
     'for child in arguments.iter().rev() {', 'for child in arguments.iter() {', 1),
    ('annotation guard', 'crates/sc-core/src/recipe/checked_statement.rs',
     'if expression.kind() != FormulaKind::from(*declared_kind)', 'if false', 1),
    ('right operand replaced', 'crates/sc-core/src/recipe/checked_statement.rs',
     'let right = check(right, FormulaStatementExpression::AssertionRight)?;',
     'let right = check(left.expression(), FormulaStatementExpression::AssertionRight)?;', 1),
    ('scope namespace discarded', 'crates/sc-core/src/recipe/namespace/ordered.rs',
     '            self.namespace,\n        )', '            &FormulaNamespace::new([]).unwrap(),\n        )', 1),
    ('canonical owner replaced', 'crates/sc-core/src/recipe/checked_statement.rs',
     'self.statement.canonical_form()',
     'super::FormulaStatement::parse("let fabricated:count=1").unwrap().normalize_literals().unwrap().canonical_form()', 2),
)
REFERENCE_FAULTS = (
    ('operation arguments', '"operation": operation, "operand_kinds"', '"operation": "unknown", "operand_kinds"'),
    ('conditional condition', 'if c != "boolean":', 'if False:'),
    ('variadic consistency', 'if not all(k == first for k in kinds): return False', 'if False: return False'),
    ('complete operand kinds', '"operand_kinds": tuple(kinds),', '"operand_kinds": tuple(kinds[:-1]),'),
    ('metadata value queried', 'if name in env: return env[name]["kind"]', 'if name in env: return env[name].get("value") or env[name]["kind"]'),
    ('whole phase interleaved', 'checked = self._syntax_statement_at(src[start:end], ordinal, start, literal_inputs=False)',
     'checked = self._static_statement(src[start:end], env, ordinal, start)'),
)


def mutations():
    import traceback
    documentation()
    doc_faults = (
        ('formula-static-validation.md', 'bounded expression and current-statement proofs are implemented; whole-recipe acceptance remains .4.',
         'complete expression/recipe acceptance remains the next two stages.'),
        ('formula-wanted-signatures.md', 'at .5b.3c.3b; atomic whole-recipe acceptance remains .4.',
         'Current statement context remains .5b.3c.3'),
        ('implementation-status.md', 'library-owned whole-recipe acceptance remains .5b.4.',
         'whole-recipe preflight remains pending.'),
    )
    for fault in doc_faults:
        try:
            documentation(fault)
        except AssertionError as error:
            assert str(error).startswith(("('D149 obsolete implemented status'", "('D151 ambiguous proof status'")), ('not a copied-annex body refusal', error)
            print('  actual copied-annex status body red:', fault[0], flush=True)
        else:
            raise AssertionError(('copied-annex status fault escaped', fault[0]))
    original_reference = SOURCE.read_bytes()
    paths = {ROOT / path for _, path, _, _, _ in RUST_FAULTS}
    originals = {path: path.read_bytes() for path in paths}
    for name, before, after in REFERENCE_FAULTS:
        try:
            check_reference((before, after))
        except AssertionError as error:
            assert str(error).startswith(("('coupled reference ", "('static namespace read value/state/geometry'")), (name, 'unrelated refusal', error)
            (WORK / ('reference-' + name.replace(' ', '-') + '.log')).write_text(traceback.format_exc())
            print('  actual compiled reference oracle body red:', name, flush=True)
        else:
            raise AssertionError(('coupled reference fault escaped', name))
    try:
        for index, (name, path, before, after, count) in enumerate(RUST_FAULTS, 1):
            path = ROOT / path
            original = originals[path].decode()
            assert original.count(before) == count, ('coupled real fault anchor', name)
            path.write_text(original.replace(before, after, count))
            try:
                build_and_check('fault-%d' % index)
            except AssertionError as error:
                # Build and link assertions never count. A running public driver may fail its
                # pointer/identity body assertion, independently of the Python outcome oracle.
                mismatch = str(error).startswith("('coupled product oracle'")
                body = str(error).startswith("('coupled public driver body failed'")
                if body:
                    log = (WORK / ('fault-%d' % index) / 'driver.log').read_text()
                    body = 'panicked at' in log and re.search(r'(?m)^assertion(?:[ :`]|$)', log) is not None
                assert mismatch or body, (name, 'compiler/link/expect/noise refused', error)
                (WORK / ('fault-%d' % index) / 'oracle.log').write_text(traceback.format_exc())
                print('  actual compiled public Rust oracle body red:', name, flush=True)
            else:
                raise AssertionError(('coupled real Rust fault escaped', name))
            finally:
                path.write_bytes(originals[path])
    finally:
        for path, data in originals.items():
            path.write_bytes(data)
    assert SOURCE.read_bytes() == original_reference and all(path.read_bytes() == data for path, data in originals.items()), 'coupled sources not exact'
    check_reference()
    build_and_check('restored')  # Restore the compiled artifact as well as all source bytes.
    print('coupled faults: six reference/nine public Rust actual compiled body reds; sources/artifact restored; rc=0')


if __name__ == '__main__':
    assert sys.argv[1:] in ([], ['--mutations']), 'coupled producer arguments'
    original = SOURCE.read_bytes()
    documentation()
    reference_count = check_reference()
    product_count = build_and_check()
    print('coupled static review: %d shared cases; independent oracle/reference/actual public Rust kinds, arguments, reads, owners and phase; rc=0' % product_count, flush=True)
    assert reference_count == product_count and SOURCE.read_bytes() == original, 'coupled reference source exact'
    if sys.argv[1:]:
        mutations()
