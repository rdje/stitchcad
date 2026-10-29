"""Pure population rows from the existing independent reference parser; no Rust/result input."""
from pathlib import Path
import runpy

ROOT = Path(__file__).resolve().parents[4]
HELPER = ROOT / 'docs/tasks/artifacts/formula_structure/static_signature_contract.py'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
PROFILE['plan_producer'](ROOT, sources=(Path(__file__), HELPER, PROFILE_SOURCE))
CONTRACT = runpy.run_path(str(HELPER))
PROFILE['plan_producer'](ROOT, sources=(Path(__file__), *CONTRACT['REFERENCE_INPUTS']))
CASES = (
    '',
    'let n:count=1\nassert check:eps_num=n==n',
    'let first:length=if(is_base_size,eps_num+eps_num,eps_num)\n'
    'assert check:eps_phys=first==eps_num\nlet zero:length=first-first',
    'let prior:length=1 mm/0\nassert later:eps_phys=1==2',
)


def rows():
    _, reference = CONTRACT['load_reference']()
    for source in CASES:
        plan = reference.preflight(source, ())
        bindings = nodes = names = 0
        for _, _, statement in plan:
            bindings += int(statement[0] == 'let')
            names += int(statement[0] == 'assert')  # actual symbolic header, not an AST node
            for tree in statement[3:]:
                nodes += reference.count_nodes(tree)
                pending = [tree]
                while pending:
                    node = pending.pop()
                    names += int(node[0] == 'name')
                    pending.extend(reference.syntax_children(node))
        count = len(plan)
        yield '\t'.join([source.replace('\n', r'\n'), *map(str, (
            count, bindings, count - bindings, nodes, names, 2 * nodes + names, count))])


if __name__ == '__main__':
    print('\n'.join(rows()))
