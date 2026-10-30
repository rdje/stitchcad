"""Independent complete source-backed recipes through the actual book evaluator.

Recursive lazy source demand uses only actual unresolved Derived requests. Source roots stay
unbound;only original recipe statements invoke storage. No Rust/native/frozen expected inputs.
The finite population checks available scalar execution,not whole Design/platform readiness.
"""
from fractions import Fraction
from graphlib import TopologicalSorter
from pathlib import Path
import runpy

ROOT = Path(__file__).resolve().parents[4]
HELPER = ROOT / 'docs/tasks/artifacts/formula_structure/static_signature_contract.py'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
PROFILE['plan_producer'](ROOT, sources=(Path(__file__), HELPER, PROFILE_SOURCE))
CONTRACT = runpy.run_path(str(HELPER))
PROFILE['plan_producer'](ROOT, sources=(Path(__file__), *CONTRACT['REFERENCE_INPUTS']))
# Actual canonical/parameter facts;these are declarations,never computed value seeds.
DECLARATIONS = (
    ('declared_width', 'L', 'length', '40', '9003'),
    ('tripled_source', 'S', 'length', '50', '9004'),
    ('width_alias', 'L', 'length', '40', '9003'),
    ('ratio_value', 'S', 'ratio', '20000', '9100'),
    ('count_value', 'S', 'count', '20001', '9101'),
    ('boolean_value', 'S', 'boolean', '20002', '9102'),
    ('angle_value', 'S', 'angle', '20003', '9103'),
    ('area_value', 'S', 'area', '20004', '9104'),
    ('length_mirror', 'S', 'length', '20005', '9003'),
)
DEFAULT_SOURCES = (
    ('9003', '1 mm/3'), ('9004', 'declared_width*3'), ('9100', '1/3'),
    ('9101', '1/2.0'), ('9102', 'cos(0 deg)==0.0'), ('9103', '-720 deg'),
    ('9104', '(3 mm)^2/2'), ('9105', 'eps_phys'),
)
# Each intended failure is asserted against the original evaluator's real first exception.
CASES = (
    ('empty', {}, '', None),
    ('unbound_shared', {}, 'let a:length=declared_width*3\nlet b:length=tripled_source\n'
     'let c:length=width_alias+length_mirror\nassert same:eps_num=a==b', None),
    ('once_storage', {}, 'let a:length=declared_width\nlet b:length=a+a\n'
     'let c:length=width_alias+length_mirror\nassert stored:eps_num=b==666 um', None),
    ('all_kinds', {}, 'let r:ratio=ratio_value*3\nlet n:count=count_value\n'
     'let f:boolean=boolean_value\nlet a:angle=angle_value/3\nlet s:area=area_value/3\n'
     'let l:length=if(f,eps_phys,declared_width*3)\nassert check:eps_geo=l==1 mm', None),
    ('wide_unbound_count', {'9101': '9223372036854775807*2'},
     'let n:count=count_value/2.0\nassert max:eps_num=n==9223372036854775807', None),
    ('zero_negative', {'9003': '-1 um/3'},
     'let a:length=declared_width\nlet b:length=width_alias*3\nassert zero:eps_num=a==0 um\n'
     'assert negative:eps_num=b==-1 um', None),
    ('lazy_unknown', {'9003': 'if(flag,1 mm,2 mm)'},
     'let a:length=if(1==2,tripled_source,1 mm)\nassert safe:eps_num=a==1 mm', None),
    ('lazy_division', {'9003': 'if(1==1,1 mm/3,1 mm/0)'},
     'let a:length=declared_width*3\nlet b:length=if(1==1,a,eps_phys)', None),
    ('flag_false', {}, 'let f:boolean=boolean_value\nlet a:length=if(f,eps_phys,1 mm)\n'
     'let z:length=a-a\nassert geometric:eps_geo=z==0 um', None),
    ('flag_supplier', {'9003': '1 mm/3+cos(0 deg)*0 mm'},
     'let a:length=tripled_source\nassert geometric:eps_geo=a==1 mm', None),
    ('within_source', {'9003': 'if(within(1 mm,1 mm,eps_num),1 mm,eps_phys)'},
     'let a:length=declared_width\nassert equal:eps_num=a==1 mm', None),
    ('source_unknown', {'9003': 'if(flag,1 mm,2 mm)'},
     'let safe:length=1 mm\nlet blocked:length=tripled_source', '2:blocked:9003:formula_unknown'),
    ('source_unknown_first', {'9003': 'if(flag,1 mm,2 mm)'},
     'let blocked:length=min(tripled_source,1 mm/0)', '1:blocked:9003:formula_unknown'),
    ('recipe_division_first', {'9003': 'if(flag,1 mm,2 mm)'},
     'let blocked:length=1 mm/0+tripled_source', '1:blocked:recipe:formula_division'),
    ('source_provider', {'9003': 'eps_phys'},
     'let safe:length=1 mm\nlet blocked:length=tripled_source', '2:blocked:9003:formula_tolerance_unbound'),
    ('source_domain', {'9003': 'sqrt(-(1 um)^2)'},
     'let blocked:length=tripled_source', '1:blocked:9003:formula_domain'),
    ('binding_overflow', {'9101': '9223372036854775807*2'},
     'let safe:length=declared_width\nlet blocked:count=count_value', '2:blocked:recipe:formula_domain'),
    ('assert_false', {}, 'let a:length=declared_width*3\nassert wrong:eps_num=a==3 mm',
     '2:wrong:recipe:formula_assertion'),
    ('assert_left', {'9003': '1 mm/0'},
     'assert failed:eps_phys=tripled_source==2 mm', '1:failed:9003:formula_division'),
    ('assert_right', {'9003': '1 mm/0'},
     'assert failed:eps_phys=1 mm==tripled_source', '1:failed:9003:formula_division'),
    ('assert_header', {}, 'assert failed:eps_phys=tripled_source==1 mm',
     '1:failed:recipe:formula_tolerance_unbound'),
    ('flags_t1', {'9003': '1 mm+cos(0 deg)*0 mm'},
     'let a:length=declared_width\nassert failed:eps_num=a==1 mm', '2:failed:recipe:formula_domain'),
    ('false_flags_t1', {}, 'let f:boolean=boolean_value\nlet a:length=if(f,eps_phys,1 mm)\n'
     'assert failed:eps_num=a==1 mm', '3:failed:recipe:formula_domain'),
)


def scalar(value):
    payload = ('true' if value.v else 'false') if value.kind == 'boolean' else (
        '%d/%d' % (Fraction(value.v).numerator, Fraction(value.v).denominator))
    return ':'.join((value.kind, payload, ','.join(sorted(value.sources)) or '-'))


def evaluate_case(sources, recipe, intended):
    namespace, reference = CONTRACT['load_reference']()
    canonical = {name: dict(kind=kind, origin='parameter', state='derived', value=None)
                 for name, family, kind, identity, supplier in DECLARATIONS}
    canonical['flag'] = dict(kind='boolean', origin='parameter', state='unknown', value=None)
    targets = {name: (identity, supplier) for name, family, kind, identity, supplier in DECLARATIONS}
    targets['flag'] = ('8000', None)
    snapshot = {name: dict(value) for name, value in canonical.items()}
    trees = {identity: reference.parse(body) for identity, body in sources.items()}
    for tree in trees.values():
        reference.infer(tree, canonical)
    for name, family, kind, identity, supplier in DECLARATIONS:
        assert reference.infer(trees[supplier], canonical) == kind
    def suppliers(tree):
        if tree[0] == 'name' and tree[1] in targets and targets[tree[1]][1] is not None:
            yield targets[tree[1]][1]
        for child in reference.syntax_children(tree):
            yield from suppliers(child)
    tuple(TopologicalSorter({identity: set(suppliers(tree)) for identity, tree in trees.items()}).static_order())
    plan = reference.preflight(recipe, canonical.items())
    environment, completed, active = dict(canonical), {}, []
    source_reads, recipe_reads, storage_inputs = {identity: [] for identity in sources}, [], []
    original_read, original_stored = reference.value_of_name, reference.stored
    failed = None
    def execute(identity):
        nonlocal failed
        if identity in completed:
            return completed[identity][0]
        assert identity not in active
        active.append(identity)
        try:
            value = reference.evaluate(trees[identity], canonical)
        except namespace['FErr']:
            if failed is None:
                failed = identity
            raise
        else:
            completed[identity] = (value, len(completed) + 1)
            return value
        finally:
            assert active.pop() == identity
    def read(name, actual_environment):
        owner = active[-1] if active else None
        assert actual_environment is (canonical if owner else environment)
        target, supplier = targets.get(name, ('-', None))
        if supplier is None:
            value = original_read(name, actual_environment)
            role = 'direct'
        else:
            try:
                original_read(name, actual_environment)
            except namespace['ReferenceInputRequired'] as request:
                assert (request.name, request.state, request.requirement) == (name, 'derived', 'derived_evaluation')
            else:
                raise AssertionError('actual canonical Derived must remain without a seeded value')
            value, role = execute(supplier), 'derived_' + supplier
        if owner:
            source_reads[owner].append(':'.join((name, target, role, scalar(value))))
        else:
            recipe_reads.append(name + ':' + scalar(value))
        return value
    def stored(value, operation):
        assert not active, 'bare sources must never introduce a binding storage boundary'
        storage_inputs.append(value)
        return original_stored(value, operation)
    reference.value_of_name, reference.stored = read, stored
    outcomes, error = ['ok'], None
    for ordinal, (start, end, checked) in enumerate(plan, 1):
        try:
            returned = reference.statement(recipe[start:end], environment)
        except namespace['FErr'] as refusal:
            error = '%d:%s:%s:%s' % (ordinal, checked[1], failed or 'recipe', refusal.token)
            assert error == intended, (error, intended, recipe)
            break
        if checked[0] == 'let':
            _, name, kind, value, _ = returned
            outcomes.append('%d:let:%s:%s>%s' % (ordinal, name, scalar(storage_inputs[-1]), scalar(value)))
            environment[name] = dict(kind=kind, origin='recipe', state='derived', value=value.v, sources=value.sources)
        else:
            _, name, holds, left, right = returned
            assert holds and recipe_reads[-1].startswith(checked[2] + ':')
            outcomes.append('%d:assert:%s:%s:%s==%s' % (ordinal, name, checked[2], scalar(left), scalar(right)))
    assert canonical == snapshot, 'original canonical facts/states may never become the private cache'
    if error:
        return ('error:' + error, '-', '-', '-')
    assert intended is None, ('intended refusal missing', intended)
    roots = ['%s@%d:%s' % (identity, completed[identity][1], scalar(completed[identity][0]))
             for identity in sources if identity in completed]
    reads = [identity + ':' + read for identity in sources for read in source_reads[identity]]
    return (';'.join(outcomes), ';'.join(roots) or '-', ';'.join(recipe_reads) or '-', ';'.join(reads) or '-')


def fixture_rows():
    yield '# case\toriginal sources\toriginal recipe\tcomplete outcome or first refusal\tunbound roots\trecipe reads\tsource reads'
    seen = set()
    for label, overrides, recipe, intended in CASES:
        assert label not in seen and set(overrides) <= dict(DEFAULT_SOURCES).keys()
        seen.add(label)
        sources = {**dict(DEFAULT_SOURCES), **overrides}
        yield '\t'.join((label, ';'.join(identity + '=' + body for identity, body in sources.items()),
                         recipe.replace('\n', r'\n'), *evaluate_case(sources, recipe, intended)))


if __name__ == '__main__':
    print('\n'.join(fixture_rows()))
