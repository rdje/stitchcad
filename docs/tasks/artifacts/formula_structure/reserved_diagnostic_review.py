"""Independent D131 diagnostic arguments against the actual reference.

Reference sources locate metadata pairs and parsed source spans; canonical record
identity and product typed namespace proof remain separate owners.
"""
from collections.abc import Mapping
from pathlib import Path
import runpy
import sys

ROOT = Path(__file__).resolve().parents[4]
LOADER = runpy.run_path(str(ROOT / 'docs/tasks/artifacts/formula_structure/static_signature_contract.py'))
SOURCE = LOADER['SOURCE']
CONTRACT = ROOT / 'docs/book/src/spec/formula-language.md'
RESERVED = (
    ('eps_num', 'length', 'tolerance', 'always'),
    ('eps_geo', 'length', 'tolerance', 'always'),
    ('eps_fmt', 'length', 'tolerance', 'export'),
    ('eps_imp', 'length', 'tolerance', 'export'),
    ('eps_phys', 'length', 'tolerance', 'profile'),
    ('size_index', 'count', 'size', 'size'),
    ('size_count', 'count', 'size', 'size'),
    ('is_base_size', 'boolean', 'size', 'size'),
)
ORIGINS = ('measurement', 'ease', 'parameter', 'profile', 'material',
           'geometry', 'recipe', 'size', 'tolerance')
KINDS = ('length', 'angle', 'area', 'ratio', 'count', 'boolean', 'point', 'edge')
ROW = '| `formula_rebinding` | a recipe binding repeats a name, or any origin attempts a reserved name | the name and case-specific binding sources (§5.2.1) |'


class Declaration(Mapping):
    def __init__(self, kind, origin):
        self.metadata = {'kind': kind, 'origin': origin}

    def __getitem__(self, key):
        assert key in self.metadata, ('D131 non-metadata read', key)
        return self.metadata[key]

    def __iter__(self):
        return iter(self.metadata)

    def __len__(self):
        return len(self.metadata)


class ReservedKind:
    def __init__(self, kind):
        self.kind = kind

    def __getitem__(self, index):
        assert index == 0, ('D131 reserved availability/value read', index)
        return self.kind


def contracts(replacement=None, verbose=True):
    assert ROW in CONTRACT.read_text(), 'D131 canonical diagnostic row drift'
    ns, reference = LOADER['load_reference'](replacement)
    assert {name: data[0] for name, data in reference.reserved.items()} == {
        name: kind for name, kind, _, _ in RESERVED}, 'D131 reserved population drift'
    reference.reserved = {name: ReservedKind(kind) for name, kind, _, _ in RESERVED}
    for method in ('evaluate', '_evaluate', 'value_of_name', 'resolve_geometry', 'call', 'stored'):
        def trapped(*args, **kwargs):
            raise AssertionError('D131 invoked execution')
        setattr(reference, method, trapped)
    checks = 0

    def refusal(action, expected):
        nonlocal checks
        result = sentinel = object()
        try:
            result = action()
        except ns['FErr'] as error:
            assert error.token == 'formula_rebinding', ('D131 refusal token', error.token)
            assert error.arguments == expected, ('D131 argument payload', error.arguments, expected)
        else:
            raise AssertionError(('D131 refusal accepted', expected))
        assert result is sentinel, 'D131 partial plan escaped'
        checks += 1

    for name, reserved_kind, reserved_origin, context in RESERVED:
        reserved_source = {'role': 'reserved', 'kind': reserved_kind,
                           'origin': reserved_origin, 'required_context': context}
        for origin in ORIGINS:
            for kind in KINDS:
                for position in (1, 2, 3):
                    declarations = [('input_%d' % i, Declaration('length', 'measurement'))
                                    for i in range(1, position)]
                    declarations.append((name, Declaration(kind, origin)))
                    before = list(declarations)
                    expected = {'name': name, 'reason': 'reserved_name', 'reserved_source': reserved_source,
                                'attempted_source': {'role': 'initial_declaration', 'kind': kind,
                                                     'origin': origin, 'declaration_index': position}}
                    refusal(lambda: reference.namespace(declarations), expected)
                    refusal(lambda: reference.preflight('', declarations), expected)
                    assert declarations == before, 'D131 initial declarations changed'
        for kind in KINDS[:6]:
            source = ' \tlet %s: %s = 1 ' % (name, kind)
            attempt = {'role': 'recipe', 'kind': kind, 'origin': 'recipe',
                       'span': (0, len(source)), 'name_span': (6, 6 + len(name))}
            expected = {'name': name, 'reason': 'reserved_name', 'reserved_source': reserved_source,
                        'attempted_source': attempt}
            for method in (reference.static_statement, reference.statement):
                env = {}
                refusal(lambda: method(source, env), expected)
                assert env == {}, 'D131 detached environment changed'
            # Leading whitespace is outside an identified whole statement; an assertion counts
            # toward ordinals but binds no value. The authored chunks determine expected spans.
            chunks = ['let first:length=1 cm\n', 'assert check:eps_geo=first==first\r\n', source[2:]]
            prefix = ' \t' + ''.join(chunks[:2])
            whole = prefix + chunks[2]
            start = len(prefix)
            attempt = {'role': 'recipe', 'kind': kind, 'origin': 'recipe', 'statement_index': 3,
                       'span': (start, len(whole)), 'name_span': (start + 4, start + 4 + len(name))}
            refusal(lambda: reference.preflight(whole, []),
                    {'name': name, 'reason': 'reserved_name', 'reserved_source': reserved_source,
                     'attempted_source': attempt})

    # Every scalar annotation; prior/attempted sources come from different actual positions.
    for kind, value in (('length', '1 cm'), ('angle', '1 deg'), ('area', '(1 cm)^2'),
                        ('ratio', '0.5'), ('count', '1'), ('boolean', '1<2')):
        first = 'let width:%s=%s\n' % (kind, value)
        detached = ' \tlet width:%s=%s ' % (kind, value)
        prior = {'role': 'recipe', 'kind': kind, 'origin': 'recipe'}
        attempt = {'role': 'recipe', 'kind': kind, 'origin': 'recipe', 'span': (0, len(detached)),
                   'name_span': (6, 11)}
        env = {'width': Declaration(kind, 'recipe')}
        refusal(lambda: reference.static_statement(detached, env),
                {'name': 'width', 'reason': 'recipe_name', 'prior_source': prior, 'attempted_source': attempt})
        for intervening in ('', 'assert check:eps_fmt=1 cm==1 cm\r\n',
                            'let other:length=1 cm\nassert check:eps_geo=other==other\n'):
            prefix = ' \t' + first + intervening
            tail = 'let width:%s=%s ' % (kind, value)
            whole = prefix + tail
            ordinal = 2 + intervening.count('let ') + intervening.count('assert ')
            prior = {'role': 'recipe', 'kind': kind, 'origin': 'recipe', 'statement_index': 1,
                     'span': (2, 2 + len(first)), 'name_span': (6, 11)}
            attempt = {'role': 'recipe', 'kind': kind, 'origin': 'recipe', 'statement_index': ordinal,
                       'span': (len(prefix), len(whole)), 'name_span': (len(prefix) + 4, len(prefix) + 9)}
            refusal(lambda: reference.preflight(whole, []),
                    {'name': 'width', 'reason': 'recipe_name', 'prior_source': prior,
                     'attempted_source': attempt, 'prior_statement_index': 1, 'statement_index': ordinal})
    if verbose:
        print('D131 diagnostic contract: %d actual argument cases / eight reserved sources / real indices and spans; execution/state/value trapped' % checks)
    return checks


FAULTS = (
    ('reserved token', 'raise FErr("formula_rebinding", "reserved `%s` cannot be declared',
     'raise FErr("formula_ambiguous_name", "reserved `%s` cannot be declared'),
    ('reserved reason', '"reason": "reserved_name",', '"reason": "recipe_name",'),
    ('reserved kind', '"kind": self.reserved[name][0]', '"kind": "area"'),
    ('reserved origin', '"origin": "size" if size else "tolerance"', '"origin": "size" if size else "profile"'),
    ('required profile context', 'else "profile" if name == "eps_phys" else "export"',
     'else "always" if name == "eps_phys" else "export"'),
    ('initial position', '"declaration_index": declaration_index', '"declaration_index": declaration_index + 1'),
    ('initial origin', '"origin": origin, "declaration_index":', '"origin": "parameter", "declaration_index":'),
    ('initial recipe index invented', '"declaration_index": declaration_index}})',
     '"declaration_index": declaration_index, "statement_index": 0}})'),
    ('attempted annotation', '"role": "recipe", "kind": kind, "origin": "recipe",',
     '"role": "recipe", "kind": "length", "origin": "recipe",'),
    ('global offset lost', 'name_start = offset + starts[1]', 'name_start = starts[1]'),
    ('name span', 'name_start + len(name)', 'name_start + len(name) + 1'),
    ('whole span', '"span": (offset, offset + len(src))', '"span": (0, len(src))'),
    ('ordinal', 'source["statement_index"] = ordinal', 'source["statement_index"] = ordinal + 1'),
    ('detached zero invented', 'if ordinal is not None:\n            source["statement_index"] = ordinal',
     'if True:\n            source["statement_index"] = ordinal if ordinal is not None else 0'),
    ('prior source lost', '"prior_source": dict(prior) if prior is not None else', '"prior_source": {} if prior is not None else'),
    ('prior index swapped', 'arguments["prior_statement_index"] = prior["statement_index"]',
     'arguments["prior_statement_index"] = ordinal'),
    ('value observed', 'kind, origin = entry["kind"], entry["origin"]',
     'kind, origin = entry["kind"], entry["origin"]\n                entry.get("value")'),
    ('availability observed', '"kind": self.reserved[name][0]', '"kind": self.reserved[name][1]'),
    ('whole context bypass', 'checked = self._static_statement(src[start:end], env, ordinal, start, prior_sources, checked=checked)',
     'checked = self.static_statement(src[start:end], env)'),
)


if __name__ == '__main__':
    original = SOURCE.read_bytes()
    contracts()
    if '--mutations' in sys.argv:
        for label, before, after in FAULTS:
            # Two reserved-case sites deliberately share the reason; target the initial guard.
            if label == 'reserved reason':
                before = '"name": name, "reason": "reserved_name",\n                            "reserved_source": self._reserved_source(name),\n                            "attempted_source": {"role": "initial_declaration"'
                after = before.replace('"reserved_name"', '"recipe_name"')
            try:
                contracts((before, after), False)
            except AssertionError as error:
                assert any(marker in str(error) for marker in ('D131 refusal token', 'D131 argument payload',
                           'D131 non-metadata read', 'D131 reserved availability/value read')), (label, error)
                print('  actual compiled diagnostic assertion red:', label)
            else:
                raise AssertionError(('D131 fault escaped', label))
        assert SOURCE.read_bytes() == original, 'D131 producer bytes changed'
        print('D131 diagnostic faults: %d actual assertion reds; producer unchanged' % len(FAULTS))
