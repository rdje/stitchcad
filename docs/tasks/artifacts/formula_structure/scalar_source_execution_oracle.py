"""Actual whole-source rows via the independently parsed book evaluator and recursive demand.

No Rust implementation/native output/frozen expected fixture is read. Original case declarations
stay unchanged:only actual returned unbound reference roots fill a separate private source cache.
This finite scalar population is not recipe/Design/geometry/currentness/platform acceptance.
"""
from fractions import Fraction
from graphlib import TopologicalSorter
from pathlib import Path
import runpy

ROOT = Path(__file__).resolve().parents[4]
HERE = ROOT / 'docs/tasks/artifacts/formula_structure'
CASES = HERE / 'source_execution_cases.tsv'
HELPER = HERE / 'static_signature_contract.py'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
PROFILE['plan_producer'](ROOT, sources=(Path(__file__), CASES, HELPER, PROFILE_SOURCE))
CONTRACT = runpy.run_path(str(HELPER))
PROFILE['plan_producer'](ROOT, sources=(Path(__file__), CASES, *CONTRACT['REFERENCE_INPUTS']))


def scalar(value):
    payload = ('true' if value.v else 'false') if value.kind == 'boolean' else (
        '%d/%d' % (Fraction(value.v).numerator, Fraction(value.v).denominator))
    return ':'.join((value.kind, payload, ','.join(sorted(value.sources)) or '-'))


def case_rows():
    seen = set()
    for row in CASES.read_text().splitlines():
        if not row or row.startswith('#'):
            continue
        label, declarations, sources, entry, refusal = row.split('\t')
        assert label not in seen
        seen.add(label)
        yield label, declarations, sources, entry, refusal


def evaluate_case(declarations, sources, entry, intended):
    namespace, reference = CONTRACT['load_reference']()
    environment, targets, canonical = {}, {}, {}
    for declaration in declarations.split(';') if declarations != '-' else ():
        name, family, kind, identity, state, payload = declaration.split(':')
        assert name not in environment and family in ('L', 'S')
        assert family != 'L' or kind == 'length'
        fact = (family, kind, state, payload)
        assert canonical.setdefault(identity, fact) == fact
        value = None if state in ('derived', 'unknown') else (
            Fraction(int(payload == 'true')) if kind == 'boolean' else Fraction(payload))
        environment[name] = dict(kind=kind, origin='parameter', state=state, value=value)
        targets[name] = (identity, payload if state == 'derived' else None)
    original = {identity: dict(value) for identity, value in environment.items()}
    bodies = dict(pair.split('=', 1) for pair in sources.split(';'))
    assert len(bodies) == len(sources.split(';')) and set(bodies).isdisjoint(canonical)
    assert entry in bodies
    trees = {identity: reference.parse(body) for identity, body in bodies.items()}
    for tree in trees.values():
        reference.infer(tree, environment)
    for family, kind, state, payload in canonical.values():
        if state == 'derived':
            assert payload in trees and reference.infer(trees[payload], environment) == kind
    def suppliers(tree):
        if tree[0] == 'name' and tree[1] in targets:
            supplier = targets[tree[1]][1]
            if supplier is not None:
                yield supplier
        for child in reference.syntax_children(tree):
            yield from suppliers(child)
    # A separate standard-library graph algorithm admits all sources/branches before values.
    tuple(TopologicalSorter({identity: set(suppliers(tree)) for identity, tree in trees.items()}).static_order())
    completed, active, reads = {}, [], {identity: [] for identity in bodies}
    original_read = reference.value_of_name
    failed = None

    def execute(identity):
        nonlocal failed
        if identity in completed:
            return completed[identity][0]
        assert identity not in active
        active.append(identity)
        try:
            value = reference.evaluate(trees[identity], environment)
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
        assert actual_environment is environment
        target, supplier = targets.get(name, ('-', None))
        if supplier is None:
            value = original_read(name, actual_environment)
            role = 'direct'
        else:
            # Demand comes from the actual evaluator's typed unresolved Derived requirement.
            try:
                original_read(name, actual_environment)
            except namespace['ReferenceInputRequired'] as request:
                assert (request.name, request.state, request.requirement) == (name, 'derived', 'derived_evaluation')
            else:
                raise AssertionError('original canonical Derived state must remain without authored value')
            value = execute(supplier)
            role = 'derived_' + supplier
        reads[active[-1]].append(':'.join((name, target, role, scalar(value))))
        return value

    reference.value_of_name = read
    def forbidden_storage(*args, **kwargs):
        raise AssertionError('bare source evaluation must never invoke a binding storage boundary')
    reference.stored = forbidden_storage
    try:
        execute(entry)
    except namespace['FErr'] as error:
        assert '%s:%s' % (failed, error.token) == intended, (failed, error.token, intended)
        outcome, trace = 'error:%s:%s' % (failed, error.token), '-'
    else:
        assert intended == '-'
        outcome = ';'.join(['ok', *('%s@%d:%s' % (identity, completed[identity][1], scalar(completed[identity][0]))
                                   for identity in bodies if identity in completed)])
        trace = ';'.join(identity + ':' + read for identity in bodies for read in reads[identity]) or '-'
    assert environment == original, 'canonical input facts must remain unchanged'
    return outcome, trace


def fixture_rows():
    yield '# case\tdeclarations\toriginal sources\tentry\tcomplete outcomes or refusal\tactual reads'
    for label, declarations, sources, entry, intended in case_rows():
        yield '\t'.join((label, declarations, sources, entry, *evaluate_case(declarations, sources, entry, intended)))


if __name__ == '__main__':
    print('\n'.join(fixture_rows()))
