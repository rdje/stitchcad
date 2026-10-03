"""D144: compiled reference locations match original source; fault variants stay virtual."""
import ast
from pathlib import Path
import runpy
import sys
import traceback

ROOT = Path(__file__).resolve().parents[4]
HELPER_SOURCE = ROOT / 'docs/tasks/artifacts/formula_structure/static_signature_contract.py'
HELPER = runpy.run_path(str(HELPER_SOURCE))
SOURCE = HELPER['SOURCE']


def contracts(loader=None, verbose=True):
    loader = loader or HELPER['load_reference']
    original = SOURCE.read_bytes()
    header, program = original.decode().split("<<'PY'\n", 1)
    program = program.rsplit('\nPY', 1)[0].split('print("-- tables read from the chapter")', 1)[0]
    # Original-source AST positions are authored independently of the loader's offset variable.
    offset = len(header.splitlines())
    tree = ast.parse(program)
    expected = {}
    for node in tree.body:
        if isinstance(node, ast.FunctionDef):
            expected[None, node.name] = min([node.lineno] + [d.lineno for d in node.decorator_list]) + offset
        elif isinstance(node, ast.ClassDef):
            for method in node.body:
                if isinstance(method, ast.FunctionDef):
                    expected[node.name, method.name] = min([method.lineno] + [d.lineno for d in method.decorator_list]) + offset
    assert expected, 'D144 no source definitions inspected'
    ns, ref = loader()
    for (owner, name), line in expected.items():
        actual = ns[name] if owner is None else vars(ns[owner])[name]
        if isinstance(actual, (staticmethod, classmethod)):
            actual = actual.__func__
        assert actual.__code__.co_filename == str(SOURCE), ('D144 original filename', owner, name)
        assert actual.__code__.co_firstlineno == line, ('D144 source location', owner, name, actual.__code__.co_firstlineno, line)
    resolver = next(n for n in ast.walk(tree) if isinstance(n, ast.FunctionDef) and n.name == 'resolve_geometry')
    raised = next(n for n in ast.walk(resolver) if isinstance(n, ast.Raise))
    expected_raise = raised.lineno + offset
    entry = {'kind': 'point', 'exprs': {'x': '1.0', 'y': '2 deg'}}

    def frame(reference, namespace):
        try:
            reference.resolve_geometry(entry, {})
        except namespace['FErr'] as error:
            assert error.token == 'formula_dimension', 'D144 diagnostic changed'
            return traceback.extract_tb(error.__traceback__)[-1]
        raise AssertionError('D144 expected dimension refusal')

    actual = frame(ref, ns)
    assert actual.filename == str(SOURCE) and actual.lineno == expected_raise, ('D144 traceback location', actual, expected_raise)
    assert actual.line == original.decode().splitlines()[expected_raise - 1].strip(), ('D144 traceback source text', actual)
    # A real in-memory docstring alteration exercises the existing variant loading interface.
    ns, ref = loader(('"""An operation\'s argument formulas are evaluated at the operation\'s position (§4.1)."""',
                      '"""In-memory reference fault for source-location controls."""'))
    actual = frame(ref, ns)
    assert actual.filename == '<formula reference fault>' and actual.line in (None, ''), ('D144 virtual fault source', actual)
    assert SOURCE.read_bytes() == original, 'D144 actual source changed'
    if verbose:
        print('reference locator contracts:', len(expected), 'original code positions / real traceback text / virtual fault frame / source exact / 0 fail')
    return len(expected)


def mutations():
    original = HELPER_SOURCE.read_bytes()
    for label, before, after in (
        ('omit original offset', "compile('\\n' * line_offset + prefix, filename, 'exec')", "compile(prefix, filename, 'exec')"),
        ('fault impersonates original', "filename = '<formula reference fault>' if replacement else str(SOURCE)", 'filename = str(SOURCE)'),
    ):
        script = original.decode()
        assert script.count(before) == 1, ('D144 actual loader anchor', label)
        scope = {'__file__': str(HELPER_SOURCE), '__name__': 'reference_locator_fault'}
        exec(compile(script.replace(before, after), str(HELPER_SOURCE), 'exec'), scope)
        try:
            contracts(scope['load_reference'], verbose=False)
        except AssertionError as error:
            assert str(error).startswith("('D144 "), (label, 'not a body assertion red', error)
            print('  actual compiled reference loader assertion red:', label)
        else:
            raise AssertionError(('D144 missed actual loader fault', label))
    assert HELPER_SOURCE.read_bytes() == original, 'D144 helper changed on disk'
    print('reference locator faults:2 actual compiled loader body reds; source exact')


if __name__ == '__main__':
    contracts()
    if '--mutations' in sys.argv:
        mutations()
