"""D152: current status rows name available proofs and honest remaining owners."""
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'docs/book/src/annexes/formula-static-validation.md'
# Independently authored current statuses; old reference counts remain historical receipts.
EXPECTED = {
    'Eight kinds, six let kinds; no implicit conversion': 'Implemented declarations and scoped checks',
    'Nine origins, eight reserved names, collisions and spelling': 'Implemented sourced namespaces and ordered scopes',
    'Every operator/function/selector signature and arity': 'Implemented closed catalogs and expression checks',
    'Tolerance-name roles; Boolean test; both branches': 'Implemented operand roles and both-branch checks',
    'Declaration order, headers, no accepted prefix on late error': 'Complete whole recipe proof and coupled static review',
    'Statement4096, expression256, conditional16 boundaries': 'Implemented and coupled syntax/input/proof limits',
    'All worked and refusal static outcomes': 'Whole static review complete; runtime .5e',
    'Envelope dispatch before operand semantics': 'Implemented call lookup and child priority',
    'Exact source and canonical identity': 'Implemented expression/statement/whole owner context',
    'Complete typed diagnostic arguments': 'Implemented static payloads; D154 reference context; command .6',
    'Persisted cycles and atomic runtime/replay behavior': 'Pending .5e/.5f and storage .7',
    'Physical geometry and cross-platform computed values': 'Pending G2 and .5g',
}
OLD_PROSE = 'Product static namespaces, immutable dependency graphs and complete typed diagnostics remain .5b.2–.4.'
NEW_PROSE = 'Product sourced namespaces, kind signatures, scoped checks and immutable whole recipe proofs are implemented.'


def check(text):
    assert OLD_PROSE not in ' '.join(text.split()), 'D152 obsolete product prerequisite prose'
    assert 'The whole-validator slice .5b.4 will inspect' not in text, 'D152 obsolete product future prose'
    header = '| Requirement | Reference evidence | Current product status / remaining owner |'
    assert header in text, 'D152 current table missing'
    rows = text.split(header + '\n| --- | --- | --- |\n', 1)[1].split('\n\n', 1)[0].splitlines()
    actual = {}
    for row in rows:
        cells = [cell.strip() for cell in row.split('|')[1:-1]]
        assert len(cells) == 3 and cells[0] not in actual, 'D152 current row shape/duplicate'
        actual[cells[0]] = cells[2]
    assert actual == EXPECTED, ('D152 current product status cells', actual, EXPECTED)
    assert NEW_PROSE in ' '.join(text.split()), 'D152 implemented scope missing'
    assert 'formula-checked-recipes.md' in text, 'D152 available whole proof route missing'


if __name__ == '__main__':
    assert sys.argv[1:] in ([], ['--mutations'])
    original = SOURCE.read_bytes()
    text = original.decode()
    check(text)
    print('D152 status:12 independently authored current cells and implemented/remaining scope; rc=0')
    if sys.argv[1:]:
        faults = [('prerequisite prose', NEW_PROSE, OLD_PROSE),
                  ('whole validator future prose', 'The whole-validator .5b.4a inspects',
                   'The whole-validator slice .5b.4 will inspect')]
        faults += [('status row ' + key, value, '.5b.2–.4') for key, value in EXPECTED.items()]
        for name, before, after in faults:
            assert text.count(before) == 1, (name, 'D152 actual copied-text anchor')
            try:
                check(text.replace(before, after, 1))
            except AssertionError as error:
                assert str(error).startswith(('D152 obsolete product', "('D152 current product status cells'")), (name, error)
            else:
                raise AssertionError(('D152 copied status fault escaped', name))
        print('D152 status faults:14 actual copied-text body reds; source unchanged; rc=0')
    assert SOURCE.read_bytes() == original, 'D152 actual text changed'
