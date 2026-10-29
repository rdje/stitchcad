"""D152/D314: current static scope and proof routes, independent of chapter layout."""
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'docs/book/src/annexes/formula-static-validation.md'
# Independently authored scope requirements. Detailed kind/catalog/limit behavior
# remains in the routed contracts and their existing native/reference controls.
EXPECTED = {
    'implemented stages': 'Current product APIs provide sourced namespaces, ordered scopes, expression signatures, whole recipe proofs and coupled static acceptance.',
    'name metadata': 'Names bind once, origins never shadow, reserved names keep known kinds without values, and both conditional branches must resolve.',
    'operand kinds': 'Boolean/point/edge operands cannot silently become arithmetic values; an if condition must be Boolean, and both branches must have the same arithmetic kind.',
    'both branches': "Both branches' names are inspected even when only one will later execute.",
    'phase order': 'Whole source syntax/structure, all literal inputs and ordered name/kind inference finish in that order before execution.',
    'no accepted prefix': 'A later syntax/input refusal prevents earlier inference; no accepted plan prefix escapes.',
    'provider scope': 'These controls do not prove arbitrary provider metadata, whole runtime rollback, product operation identity or physical geometry.',
    'reference scope': 'The complete static review covers its stated expression/header outcomes; local provider checking adds no product accepted-expression, complete operation graph or physical geometry proof.',
    'open ordinal owner': "D154's reference refusal ordinal repair remains OPEN under .5b.4c; the director's existing arithmetic/product exception does not close it.",
    'pending runtime': 'Persisted-cycle/replay repair remains .5e/.7;',
    'pending geometry': 'physical geometry and computed cross-platform verification remain G2/.5g.',
    'proof boundary': 'Static proofs grant no numerical result, physical geometry, release readiness or human approval.',
}
ROUTES = (
    'formula-recipe-inputs.md', 'formula-declarations.md', 'formula-name-scopes.md',
    'formula-operator-signatures.md', 'formula-builtin-signatures.md', 'formula-wanted-signatures.md',
    'formula-call-lookup.md', 'formula-checked-recipes.md', 'formula-runtime-validation.md',
    '../spec/formula-language.md', '../spec/formula-language/grammar.md',
)
OLD_PROSE = 'Product static namespaces, immutable dependency graphs and complete typed diagnostics remain .5b.2–.4.'
OLD_FUTURE = 'The whole-validator slice .5b.4 will inspect'


def check(text):
    prose = ' '.join(text.split())
    assert OLD_PROSE not in prose, 'D152 obsolete product prerequisite prose'
    assert OLD_FUTURE not in prose, 'D152 obsolete product future prose'
    for name, required in EXPECTED.items():
        assert required in prose, 'D152 current scope missing: ' + name
    targets = {target.split('#', 1)[0] for target in re.findall(r'\]\(([^)]+)\)', text)}
    for route in ROUTES:
        assert route in targets, 'D152 available proof route missing: ' + route


if __name__ == '__main__':
    assert sys.argv[1:] in ([], ['--mutations'])
    original = SOURCE.read_bytes()
    text = original.decode()
    check(text)
    # Paragraph wrapping and unrelated prose are not semantic status changes.
    check(' \n '.join(text.split()))
    check(text + '\nAn unrelated maintenance sentence.\n')
    print('D152 status:%d independent scope clauses / %d proof routes; reflow/prose positives; rc=0' %
          (len(EXPECTED), len(ROUTES)))
    if sys.argv[1:]:
        flat = ' '.join(text.split())
        faults = [('obsolete prerequisites', flat + ' ' + OLD_PROSE),
                  ('obsolete future', flat + ' ' + OLD_FUTURE)]
        for name, required in EXPECTED.items():
            assert flat.count(required) == 1, (name, 'D152 actual copied-text anchor')
            faults.append((name, flat.replace(required, 'Scoped status removed.', 1)))
        for route in ROUTES:
            assert route in flat, (route, 'D152 actual copied-link anchor')
            # A route can appear repeatedly; remove every actual occurrence.
            faults.append((route, flat.replace(route, 'missing-' + route)))
        for name, faulty in faults:
            try:
                check(faulty)
            except AssertionError as error:
                assert str(error).startswith(('D152 obsolete product', 'D152 current scope missing:',
                                              'D152 available proof route missing:')), (name, error)
            else:
                raise AssertionError(('D152 copied scope/route fault escaped', name))
        print('D152 status faults:%d actual copied-text body reds; source unchanged; rc=0' % len(faults))
    assert SOURCE.read_bytes() == original, 'D152 actual text changed'
