"""Actual pure imports, complete producing plans and independently observed refusal effects."""
from pathlib import Path
import ast
import contextlib
import inspect
import io
import os
import re
import runpy
import shutil
import subprocess
import sys
import types

ROOT = Path(__file__).resolve().parents[4]
HERE = ROOT / 'docs/tasks/artifacts/formula_structure'
ENTRIES = (HERE / 'origin_value_contract.py', HERE / 'provenance_contract.py')
PROVIDER = HERE / 'origin_provenance_fixture.py'
INPUT = HERE / 'formula_input.py'
STRUCTURE = HERE / 'formula_structure.py'
SIGNATURE = HERE / 'static_signature_contract.py'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
BOOK = ROOT / 'docs/book/src'
GOOD = runpy.run_path(str(PROFILE_SOURCE))
BOOT = (PROVIDER, PROFILE_SOURCE, SOURCE, *ENTRIES, INPUT, STRUCTURE, SIGNATURE)
directories, files = [BOOK], []
for folder in directories:
    GOOD['directory'](ROOT, folder.relative_to(ROOT).as_posix())
    for path in sorted(folder.iterdir()):
        GOOD['require'](not path.is_symlink(), 'symlink origin control book entry')
        (directories if path.is_dir() else files).append(path)
BOOK_FILES = tuple(files)
WHOLE = set((*BOOT, *BOOK_FILES))
GOOD['plan_producer'](ROOT, sources=(*WHOLE, Path(__file__)))
REFERENCES = set((*BOOT, *(BOOK / name for name in ('spec/formula-language.md', 'spec/formula-language/grammar.md', 'spec/formula-language/examples.md', 'spec/reference-skirt.md', 'spec/units-and-tolerances.md'))))
FROZEN = {path: path.read_bytes() for path in WHOLE}
SCRIPTS = {path: FROZEN[path].decode() for path in (*ENTRIES, PROVIDER, INPUT, STRUCTURE, SIGNATURE)}
BEFORE = '7587d669141b7b81e1a9fffe7cef60caa898a3b9'
ORIGINALS = {path: subprocess.check_output(['git', 'show', BEFORE + ':' + path.relative_to(ROOT).as_posix()], text=True) for path in ENTRIES}
BASES = {
    'missing_size': ROOT / 'target/formula_structure/origin_value_consumer/missing_size/src',
    'missing_tolerance': ROOT / 'target/formula_structure/origin_value_consumer/missing_tolerance/src',
    'provenance': ROOT / 'target/formula_structure/provenance_consumer/src',
}
LABELS = {ENTRIES[0]: ('missing_size', 'missing_tolerance'), ENTRIES[1]: ('provenance',)}
COUNTS = dict(zip(ENTRIES, (1466, 425)))


def expected(labels):
    bases = tuple(BASES[label] for label in labels)
    copies = {base / path.relative_to(BOOK) for base in bases for path in BOOK_FILES}
    outputs = copies | {base.parent / 'consumer.log' for base in bases}
    if 'provenance' in labels:
        outputs.add(BASES['provenance'].parent / 'mutant.sh')
    dirs = {base / folder.relative_to(BOOK) for base in bases for folder in directories}
    return copies, outputs, dirs


def capture(entry, mode, *, scripts=None, selectors=None):
    """Execute actual bodies; intercept children, observe every persistent file effect."""
    scripts = dict(SCRIPTS) | (scripts or {})
    labels = LABELS[entry]
    copies, outputs, dirs = expected(labels)
    saved, argv = dict(os.environ), sys.argv
    stores = {name: str(ROOT / 'target/origin-plan-capture-stores' / suffix) for name, suffix in (
        ('CARGO_HOME', 'cargo'), ('RUSTUP_HOME', 'rustup'), ('CARGO_TARGET_DIR', 'build'), ('TMPDIR', 'scratch'))}
    for path in stores.values():
        GOOD['directory'](ROOT, Path(path).relative_to(ROOT).as_posix(), create=True)
    originals = (runpy.run_path, Path.read_text, Path.read_bytes, Path.write_text, Path.write_bytes,
                 Path.mkdir, os.mkdir, subprocess.run, shutil.rmtree, shutil.copytree)
    latest, plans, prepared, writes, children = {}, [], [], [], []
    edit, child_mark, activation = [0], [0], [0]

    def owner():
        frame = sys._getframe(1)
        while frame and Path(frame.f_globals.get('__file__', '')) not in scripts:
            frame = frame.f_back
        return Path(frame.f_globals['__file__']) if frame else None

    def plan(root, **kwargs):
        assert root == ROOT, 'origin plan root changed'
        before = dict(os.environ)
        result = GOOD['plan_producer'](root, **kwargs)
        assert dict(os.environ) == before, 'origin pure plan changed state'
        plans.append((owner(), kwargs)); latest[owner()] = (kwargs, len(plans))
        return result

    def prepare(root, **kwargs):
        assert latest.get(PROVIDER, ({}, 0))[0] == kwargs, 'origin activation before complete plan'
        assert set(kwargs.get('sources', ())) >= WHOLE, 'origin producing sources incomplete'
        assert set(kwargs.get('output_files', ())) == outputs, 'origin producing files incomplete'
        assert set(kwargs.get('directories', ())) == dirs, 'origin producing directories incomplete'
        assert not kwargs.get('mutable_sources'), 'origin source authority changed'
        prepared.append(kwargs); activation[0] = len(plans)
        return GOOD['prepare_producer'](root, **kwargs)

    def load(file, *args, **kwargs):
        path = Path(file)
        if path == PROFILE_SOURCE:
            return dict(GOOD, plan_producer=plan, prepare_producer=prepare)
        who = owner()
        assert path in scripts, 'origin unexpected loader'
        if who == entry:
            assert latest.get(entry, ({}, 0))[0].get('sources') == (entry, PROFILE_SOURCE, PROVIDER, SIGNATURE), 'origin loader before bootstrap'
        elif who == PROVIDER:
            assert latest.get(PROVIDER, ({}, 0))[0].get('sources') == BOOT, 'origin context before bootstrap'
        ns = dict(__file__=str(path), __name__='borrowed_origin')
        exec(compile(scripts[path], str(path), 'exec'), ns)
        if path == SIGNATURE:
            real = ns['load_reference']
            def reference(*args, **kwargs):
                assert set(latest.get(PROVIDER, ({}, 0))[0].get('sources', ())) == REFERENCES, 'origin pure source recheck omitted'
                return real(*args, **kwargs)
            ns['load_reference'] = reference
        return ns

    def read(path, binary, *args, **kwargs):
        if path in copies:
            base = next(base for base in BASES.values() if path.is_relative_to(base))
            current = {base / item.relative_to(BOOK) for item in BOOK_FILES}
            planned, generation = latest.get(PROVIDER, ({}, 0))
            if owner() == entry:
                assert current <= set(planned.get('sources', ())) and current <= set(planned.get('output_files', ())) and generation > edit[0], 'origin copied source recheck omitted'
            else:
                assert path in latest.get(INPUT, ({}, 0))[0].get('sources', ()), 'origin selector read before guard'
        return originals[2 if binary else 1](path, *args, **kwargs)

    def authorize(path):
        assert prepared and any(path in item.get('output_files', ()) for item in prepared), 'origin effect before producing plan'
        assert path in outputs, 'origin output unowned'
        assert all(os.environ.get(name) == value for name, value in stores.items()), 'origin stores lost'

    def write(path, data, binary, *args, **kwargs):
        authorize(path)
        if path.name == 'consumer.log':
            planned, generation = latest.get(PROVIDER, ({}, 0))
            assert generation > child_mark[0] and set(planned.get('output_files', ())) == outputs, 'origin log recheck omitted'
        elif path.name == 'mutant.sh':
            planned, generation = latest.get(PROVIDER, ({}, 0))
            assert generation > edit[0] and set(planned.get('output_files', ())) == outputs, 'origin mutant recheck omitted'
        else:
            assert latest.get(PROVIDER, ({}, 0))[1] > edit[0], 'origin copied edit recheck omitted'
            edit[0] = len(plans)
        result = originals[4 if binary else 3](path, data, *args, **kwargs)
        writes.append(path); return result

    def path_mkdir(path, *args, **kwargs):
        assert prepared, 'origin mkdir before producing plan'
        return originals[5](path, *args, **kwargs)

    def mkdir(path, *args, **kwargs):
        assert prepared and Path(path) in dirs, 'origin directory unplanned'
        return originals[6](path, *args, **kwargs)

    def remove(path, *args, **kwargs):
        assert prepared, 'origin delete before producing plan'
        planned, generation = latest.get(PROVIDER, ({}, 0))
        label = next(label for label, base in BASES.items() if base == Path(path))
        assert generation > activation[0] and set(planned.get('output_files', ())) == expected((label,))[1], 'origin delete recheck omitted'
        return originals[8](path, *args, **kwargs)

    def copyfile(source, destination):
        source, destination = Path(source), Path(destination)
        assert source in BOOK_FILES, 'origin copy source unplanned'
        authorize(destination); GOOD['plan_producer'](ROOT, sources=(source,), output_files=(destination,))
        writes.append(destination); return shutil.copy2(source, destination)

    def copytree(source, destination, *args, **kwargs):
        if Path(source) == BOOK:
            assert Path(destination) in {BASES[label] for label in labels} and not args and not kwargs, 'origin copy interface changed'
            assert prepared and latest.get(PROVIDER, ({}, 0))[1] > activation[0], 'origin copy recheck omitted'
            return originals[9](source, destination, copy_function=copyfile)
        bound = inspect.signature(originals[9]).bind(source, destination, *args, **kwargs); bound.apply_defaults()
        assert bound.arguments['copy_function'] is copyfile, 'origin recursive copy changed'
        return originals[9](source, destination, *args, **kwargs)

    def child(arguments, *args, **kwargs):
        environment = kwargs.get('env', {})
        book = Path(environment['FORMULA_BOOK'])
        label = next(label for label, base in BASES.items() if base == book)
        script = book.parent / 'mutant.sh' if mode == 'consumer-fault' or (mode == 'cli' and len(children) == 1 and entry == ENTRIES[1]) else SOURCE
        assert arguments == ['bash', str(script)] and not args and kwargs.get('cwd') == ROOT and kwargs.get('text') is True and kwargs.get('capture_output') is True, 'origin child interface changed'
        assert all(environment.get(name) == value for name, value in stores.items()) and environment.get('MAKE_TMPDIR') == stores['TMPDIR'], 'origin child stores lost'
        assert environment.get('ORIGIN_UNRELATED') == 'preserved' and all(environment.get(name) == value for name, value in (selectors or {}).items()), 'origin caller environment lost'
        assert any(who == PROVIDER and WHOLE <= set(item.get('sources', ())) for who, item in plans[edit[0]:]), 'origin child source recheck omitted'
        parts = Path(environment.get('FORMULA_PARTS_DIR') or book / 'spec/formula-language')
        parts = parts if parts.is_absolute() else ROOT / parts
        selected = [Path(environment.get(name) or default) for name, default in (
            ('FORMULA_CONTRACT', book / 'spec/formula-language.md'), ('FORMULA_GRAMMAR', parts / 'grammar.md'),
            ('FORMULA_EXAMPLES', parts / 'examples.md'), ('FORMULA_FIXTURE', book / 'spec/reference-skirt.md'))]
        selected = [path if path.is_absolute() else ROOT / path for path in selected]
        refs = [Path(os.path.normpath(str(path.parent) + '/' + target.split('#')[0]))
                for path in selected[:3] for unused, target in re.findall(r'\[([^\]]*)\]\(([^)]+)\)', originals[1](path))
                if not target.startswith(('http://', 'https://', '#'))]
        assert set(latest.get(INPUT, ({}, 0))[0].get('sources', ())) == set((*selected, *sorted(parts.glob('*.md')), *refs)), 'origin child selector recheck omitted'
        if entry == ENTRIES[1]:
            assert latest.get(entry, ({}, 0))[0].get('sources') == (script,), 'origin child script recheck omitted'
        child_mark[0] = len(plans); children.append(arguments)
        output = 'statically accepted statements: 21\n'
        if label == 'provenance':
            output += 'formula_domain allocation_balance eps_num contributions=hypot\n' if script == SOURCE else 'no approximation evidence\n'
        else:
            token, name = ('formula_unbound_name', 'size_index') if label == 'missing_size' else ('formula_tolerance_unbound', 'eps_fmt')
            output += 'examples §2 `garment_waist`: ' + token + ' ' + name + '\n'
        return types.SimpleNamespace(returncode=1, stdout=output, stderr='')

    environment = dict(saved, **stores, MAKE_TMPDIR=stores['TMPDIR'], RUSTUP_AUTO_INSTALL='0', RUSTUP_TOOLCHAIN='1.99.0', ORIGIN_UNRELATED='preserved')
    for name in ('FORMULA_ROOT', 'FORMULA_BOOK', 'FORMULA_PARTS_DIR', 'FORMULA_CONTRACT', 'FORMULA_GRAMMAR', 'FORMULA_EXAMPLES', 'FORMULA_FIXTURE'):
        environment.pop(name, None)
    environment.update(selectors or {})
    try:
        os.environ.clear(); os.environ.update(environment); sys.argv = [str(entry), '--mutations', 'unrelated']; current_argv = sys.argv
        runpy.run_path = load
        Path.read_text = lambda path, *args, **kwargs: read(path, False, *args, **kwargs)
        Path.read_bytes = lambda path, *args, **kwargs: read(path, True, *args, **kwargs)
        Path.write_text = lambda path, data, *args, **kwargs: write(path, data, False, *args, **kwargs)
        Path.write_bytes = lambda path, data, *args, **kwargs: write(path, data, True, *args, **kwargs)
        Path.mkdir, os.mkdir, subprocess.run, shutil.rmtree, shutil.copytree = path_mkdir, mkdir, child, remove, copytree
        ns = dict(__file__=str(entry), __name__='__main__' if mode == 'cli' else 'borrowed_origin')
        with contextlib.redirect_stdout(io.StringIO()):
            exec(compile(scripts[entry], str(entry), 'exec'), ns)
            assert ns['PROFILE'] is ns['FIXTURE']['PROFILE'] is ns['HELPER']['PROFILE'], 'origin actual provider identity changed'
            if mode == 'contracts':
                assert ns['contracts'](verbose=False) == COUNTS[entry], 'origin authored count changed'
            elif mode == 'plan':
                result = ns['FIXTURE']['fixture_plan'](labels)
                assert set(result['output_files']) == outputs and set(result['directories']) == dirs and set(result['sources']) >= WHOLE, 'origin pure plan incomplete'
            elif mode == 'consumer':
                ns['consumer']()
            elif mode == 'consumer-fault':
                try:
                    ns['consumer'](('env[token]["sources"] = val.sources', 'env[token]["sources"] = frozenset()'), False)
                except AssertionError as error:
                    assert 'provenance consumer refusal' in str(error), ('origin unrelated copied fault', error)
                else:
                    raise AssertionError('origin copied fault escaped')
        assert sys.argv is current_argv and dict(os.environ) == environment, 'origin borrowed state changed'
        if mode in ('consumer', 'cli', 'consumer-fault'):
            count = 2 if entry == ENTRIES[0] or mode == 'cli' else 1
            assert len(children) == count and len(writes) == count * (len(BOOK_FILES) + 2) + (1 if entry == ENTRIES[1] and (count == 2 or mode == 'consumer-fault') else 0), 'origin authored effects changed'
        else:
            assert not prepared and not writes and not children, 'origin pure call produced effects'
        assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'origin capture changed tracked source'
        return len(plans), len(writes), len(children)
    finally:
        runpy.run_path, Path.read_text, Path.read_bytes, Path.write_text, Path.write_bytes, Path.mkdir, os.mkdir, subprocess.run, shutil.rmtree, shutil.copytree = originals
        os.environ.clear(); os.environ.update(saved); sys.argv = argv


def original_controls():
    for entry in ENTRIES:
        old, new = ast.parse(ORIGINALS[entry]), ast.parse(SCRIPTS[entry])
        def function(tree, name):
            return next(node for node in tree.body if isinstance(node, ast.FunctionDef) and node.name == name)
        body = function(new, 'contracts'); body.body = body.body[1:]
        assert ast.dump(body) == ast.dump(function(old, 'contracts')), 'origin numeric contracts changed'
        if entry == ENTRIES[1]:
            assert ast.dump(function(old, 'entry')) == ast.dump(function(new, 'entry')), 'provenance entry contract changed'
        names = ('ORIGINS', 'KINDS', 'STATES', 'TOLERANCES', 'RESERVED', 'FAULTS') if entry == ENTRIES[0] else ('CLASSES', 'CALLS', 'FAULTS', 'PRODUCERS')
        for name in names:
            def value(tree):
                return next(node.value for node in tree.body if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == name for target in node.targets))
            assert ast.dump(value(old)) == ast.dump(value(new)), ('origin authored data changed', name)
        try:
            capture(entry, 'import', scripts={entry: ORIGINALS[entry]})
        except AssertionError as error:
            assert str(error) == 'origin loader before bootstrap', ('origin unrelated original failure', str(error))
        else:
            raise AssertionError('origin old bootstrap gap escaped')
        raw = ast.get_source_segment(ORIGINALS[entry], function(old, 'consumer'))
        candidate = SCRIPTS[entry].replace(ast.get_source_segment(SCRIPTS[entry], function(new, 'consumer')), raw, 1)
        candidate = candidate.replace('import runpy\n', 'import runpy\nimport shutil\n', 1)
        try:
            capture(entry, 'consumer', scripts={entry: candidate})
        except AssertionError as error:
            assert str(error) in ('origin delete before producing plan', 'origin mkdir before producing plan'), ('origin unrelated old effect failure', str(error))
        else:
            raise AssertionError('origin old consumer gap escaped')
        common = (
            (entry, 'import', "    PROFILE['plan_producer'](ROOT, sources=BOOT_INPUTS)", '    None', 'origin loader before bootstrap'),
            (PROVIDER, 'import', "    PROFILE['plan_producer'](ROOT, sources=BOOT_INPUTS)", '    None', 'origin context before bootstrap'),
            (entry, 'contracts', "    FIXTURE['reference_plan']()\n    ns, ref", '    ns, ref', 'origin pure source recheck omitted'),
            (PROVIDER, 'consumer', "    PROFILE['plan_producer'](ROOT, **plan)\n    return", '    return', 'origin activation before complete plan'),
            (PROVIDER, 'consumer', 'output_files=outputs, sources=sources', 'output_files=outputs[:-1], sources=sources', 'origin producing files incomplete'),
            (PROVIDER, 'consumer', 'directories=directories, output_files=outputs', 'directories=directories[:-1], output_files=outputs', 'origin producing directories incomplete'),
            (PROVIDER, 'consumer', 'output_files=outputs, sources=sources', 'output_files=outputs, sources=sources[:-1]', 'origin producing sources incomplete'),
            (PROVIDER, 'consumer', "    fixture_plan((label,))\n    book", '    book', 'origin delete recheck omitted'),
            (PROVIDER, 'consumer', "    PROFILE['plan_producer'](ROOT, sources=(SOURCE, *files), output_files=files)", '    None', 'origin copied source recheck omitted'),
            (entry, 'consumer', "env=dict(environment, FORMULA_BOOK=str(book))", "env=dict(environment, FORMULA_BOOK=str(book), TMPDIR=str(ROOT/'target/wrong-origin-store'))", 'origin child stores lost'),
            (entry, 'import', "PROFILE = FIXTURE['PROFILE']\n", '', 'origin actual provider identity changed'),
            (entry, 'import', "HELPER['PROFILE'] = PROFILE\n", '', 'origin actual provider identity changed'),
        )
        indent = '        ' if entry == ENTRIES[0] else '    '
        log = "(book.parent / 'consumer.log')" if entry == ENTRIES[0] else "(work / 'consumer.log')"
        planning = "FIXTURE['fixture_plan'](labels)" if entry == ENTRIES[0] else "FIXTURE['fixture_plan'](('provenance',))"
        extra = (
            (entry, 'consumer', indent + "FIXTURE['source_plan']()\n" + indent + "FIXTURE['consumer_inputs']", indent + "FIXTURE['consumer_inputs']", 'origin child source recheck omitted'),
            (entry, 'consumer', indent + "FIXTURE['consumer_inputs'](dict(environment, FORMULA_BOOK=str(book)))\n", '', 'origin child selector recheck omitted'),
            (entry, 'consumer', indent + planning + '\n' + indent + log, indent + log, 'origin log recheck omitted'),
        )
        for path, mode, anchor, replacement, wanted in (*common, *extra):
            assert SCRIPTS[path].count(anchor) == 1, ('origin omission anchor changed', anchor)
            try:
                capture(entry, mode, scripts={path: SCRIPTS[path].replace(anchor, replacement, 1)})
            except AssertionError as error:
                assert str(error) == wanted, ('origin unrelated omission red', anchor, str(error))
            else:
                raise AssertionError(('origin actual omission escaped', anchor))
    entry = ENTRIES[1]
    for anchor, replacement, wanted in (
        ("        FIXTURE['fixture_plan'](('provenance',))\n        script.write_text", '        script.write_text', 'origin mutant recheck omitted'),
        ("    PROFILE['plan_producer'](ROOT, sources=(script,))\n", '', 'origin child script recheck omitted'),
    ):
        assert SCRIPTS[entry].count(anchor) == 1
        try:
            capture(entry, 'consumer-fault', scripts={entry: SCRIPTS[entry].replace(anchor, replacement, 1)})
        except AssertionError as error:
            assert wanted in str(error), ('origin unrelated copied omission', str(error))
        else:
            raise AssertionError(('origin copied omission escaped', anchor))
    print('origin guards:32 actual omission reds/four original gaps; numeric bodies/authored data exact')


def mirror(label, index):
    sandbox = ROOT / ('target/origin-plan-' + label) / str(os.getpid()) / str(index)
    assert not sandbox.exists(), 'origin mirror already exists'
    mirrors = {path: sandbox / path.relative_to(ROOT) for path in WHOLE}
    dirs = {sandbox}
    for path in mirrors.values():
        dirs.update(parent for parent in path.parents if parent.is_relative_to(sandbox))
    GOOD['prepare_producer'](ROOT, sources=tuple(WHOLE), directories=tuple(dirs), output_files=tuple(mirrors.values()))
    for source, destination in mirrors.items():
        shutil.copy2(source, destination)
    return sandbox, mirrors


def physical_controls():
    """Use real isolated trees and actual modules; no writes/children permitted after setup."""
    cases = ('missing-source', 'linked-source', 'directory-source', 'nested-source', 'missing-helper',
             'linked-helper', 'directory-helper', 'linked-book-directory', 'unexpected-copy', 'linked-copy',
             'hardlinked-copy', 'linked-log', 'directory-log', 'hardlinked-log', 'blocked-directory',
             'outside-selector', 'output-alias', 'directory-alias', 'linked-store', 'invalid-toolchain')
    count, borrowed = 0, 0
    for entry in ENTRIES:
        for case in cases:
            sandbox, mirrors = mirror('physical', count); count += 1
            label = LABELS[entry][-1]; base = sandbox / BASES[label].relative_to(ROOT)
            source = mirrors[BOOK / 'spec/units-and-tolerances.md']
            helper = mirrors[SIGNATURE]
            output = base / 'spec/formula-language.md'; log = base.parent / 'consumer.log'
            GOOD['prepare_producer'](ROOT, directories=(output.parent, log.parent), output_files=(output, log))
            extra = {}; expected = None
            if case in ('missing-source', 'directory-source', 'missing-helper', 'directory-helper'):
                victim = helper if case.endswith('helper') else source
                victim.unlink()
                if case.startswith('directory'):
                    victim.mkdir()
                expected = 'producer source is not a local regular file'
            elif case in ('linked-source', 'linked-helper'):
                victim = helper if case.endswith('helper') else source
                victim.unlink(); victim.symlink_to(mirrors[PROFILE_SOURCE])
                expected = 'symlink producer source'
            elif case == 'nested-source':
                (source.parent / '.git').mkdir(); expected = 'store crosses another Git repository'
            elif case == 'linked-book-directory':
                (sandbox / 'docs/book/src/linked').symlink_to(source.parent, target_is_directory=True); expected = 'symlink copied-book entry'
            elif case == 'unexpected-copy':
                (base / 'unexpected.txt').write_text('owned sentinel\n'); expected = 'unexpected copied-book fixture entry'
            elif case in ('linked-copy', 'hardlinked-copy'):
                if case.startswith('linked'):
                    output.symlink_to(source); expected = 'symlink copied-book entry'
                else:
                    os.link(source, output); expected = 'hard-linked producer output'
            elif case in ('linked-log', 'directory-log', 'hardlinked-log'):
                if case.startswith('linked'):
                    log.symlink_to(source); expected = 'symlink producer output'
                elif case.startswith('directory'):
                    log.mkdir(); expected = 'producer output is not a local regular file'
                else:
                    os.link(source, log); expected = 'hard-linked producer output'
            elif case == 'blocked-directory':
                (base / 'annexes').write_text('owned sentinel\n'); expected = 'unexpected copied-book fixture entry'
            elif case == 'outside-selector':
                extra['FORMULA_CONTRACT'] = str(entry); expected = 'producer path outside workspace'
            elif case == 'output-alias':
                output.write_text('owned sentinel\n'); extra['FORMULA_CONTRACT'] = str(output); expected = 'copied-book output aliases selected input'
            elif case == 'directory-alias':
                parts = base / 'spec/formula-language'; GOOD['directory'](ROOT, parts.relative_to(ROOT).as_posix(), create=True)
                extra.update(FORMULA_PARTS_DIR=str(parts), FORMULA_GRAMMAR=str(mirrors[BOOK / 'spec/formula-language/grammar.md']), FORMULA_EXAMPLES=str(mirrors[BOOK / 'spec/formula-language/examples.md']))
                expected = 'copied-book output aliases selected input directory'
            elif case == 'linked-store':
                (sandbox / 'target/scratch').symlink_to(source.parent, target_is_directory=True); expected = 'symlink store component'
            else:
                extra['RUSTUP_TOOLCHAIN'] = 'invalid/toolchain'; expected = 'toolchain override must be a name'
            saved, argv = dict(os.environ), sys.argv
            originals = (Path.mkdir, os.mkdir, Path.write_text, Path.write_bytes, subprocess.run, shutil.copytree, shutil.rmtree)
            def effect(*args, **kwargs):
                raise AssertionError('origin physical refusal produced effects')
            stderr = io.StringIO()
            try:
                for name in ('FORMULA_ROOT', 'FORMULA_BOOK', 'FORMULA_PARTS_DIR', 'FORMULA_CONTRACT', 'FORMULA_GRAMMAR', 'FORMULA_EXAMPLES', 'FORMULA_FIXTURE', 'RUSTUP_TOOLCHAIN'):
                    os.environ.pop(name, None)
                os.environ.update({name: str(sandbox / suffix) for name, suffix in GOOD['STORES'].items()}); os.environ.update(extra)
                current = dict(os.environ); sys.argv = [str(mirrors[entry]), '--mutations']; current_argv = sys.argv
                Path.mkdir = os.mkdir = Path.write_text = Path.write_bytes = subprocess.run = shutil.copytree = shutil.rmtree = effect
                with contextlib.redirect_stderr(stderr), contextlib.redirect_stdout(io.StringIO()):
                    try:
                        runpy.run_path(str(mirrors[entry]), run_name='__main__')
                    except SystemExit as error:
                        assert error.code == 2 and stderr.getvalue() == 'local-environment: REFUSED — ' + expected + '\n', (case, 'unrelated CLI refusal', stderr.getvalue())
                    else:
                        raise AssertionError(('origin physical refusal escaped', case))
                    if case in ('missing-source', 'linked-source', 'directory-source', 'nested-source'):
                        ns = runpy.run_path(str(mirrors[entry]))
                        for unused in range(2):
                            try:
                                ns['contracts'](verbose=False)
                            except ns['PROFILE']['Refusal'] as error:
                                assert str(error) == expected
                            else:
                                raise AssertionError('origin borrowed refusal escaped')
                            borrowed += 1
                    if case in ('hardlinked-log', 'directory-alias'):
                        target = mirrors[PROFILE_SOURCE if case == 'hardlinked-log' else PROVIDER]
                        text = target.read_text()
                        if case == 'hardlinked-log':
                            tree = ast.parse(text); body = next(node for node in tree.body if isinstance(node, ast.FunctionDef) and node.name == 'plan_producer')
                            lines = text.splitlines(keepends=True)
                            disabled = ''.join(lines[:body.body[0].lineno - 1]) + '    return ()\n' + ''.join(lines[body.end_lineno:])
                        else:
                            anchor = 'not any(parts == base or parts.is_relative_to(base) for base in bases)'; assert text.count(anchor) == 1
                            disabled = text.replace(anchor, 'True', 1)
                        originals[2](target, disabled)
                        try:
                            ns = runpy.run_path(str(mirrors[entry]))
                            ns['FIXTURE']['fixture_plan'](LABELS[entry])
                        finally:
                            originals[2](target, text)
                        assert target.read_text() == text, 'origin counterfactual source not restored'
                assert dict(os.environ) == current and sys.argv is current_argv, 'origin physical refusal changed borrowed state'
            finally:
                Path.mkdir, os.mkdir, Path.write_text, Path.write_bytes, subprocess.run, shutil.copytree, shutil.rmtree = originals
                os.environ.clear(); os.environ.update(saved); sys.argv = argv
    assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'origin physical controls changed tracked source'
    print('origin physical:', count, 'actual CLI refusals;', borrowed, 'borrowed pure refusals; actual planner/D202 omissions admit same unsafe fixture without effects')


def pure_controls():
    """Unused sidebars and unsafe unused outputs must not change pure numerical interfaces."""
    count = 0
    for entry in ENTRIES:
        for case in ('unused-sidebar-and-log', 'readonly-hardlink'):
            sandbox, mirrors = mirror('pure', count); count += 1
            source = mirrors[BOOK / 'spec/units-and-tolerances.md']
            sidebar = mirrors[BOOK / 'annexes/formula-static-validation.md']
            log = sandbox / BASES[LABELS[entry][-1]].parent.relative_to(ROOT) / 'consumer.log'
            GOOD['prepare_producer'](ROOT, directories=(log.parent,), output_files=(log, sandbox / 'target/readonly-alias'))
            if case == 'unused-sidebar-and-log':
                sidebar.unlink(); sidebar.symlink_to(source)
                log.symlink_to(source)
            else:
                os.link(source, sandbox / 'target/readonly-alias')
            saved, argv = dict(os.environ), sys.argv
            originals = (Path.mkdir, os.mkdir, Path.write_text, Path.write_bytes, subprocess.run, shutil.copytree, shutil.rmtree)
            def effect(*args, **kwargs):
                raise AssertionError('origin pure positive produced effects')
            try:
                for name in ('FORMULA_ROOT', 'FORMULA_BOOK', 'FORMULA_PARTS_DIR', 'FORMULA_CONTRACT', 'FORMULA_GRAMMAR', 'FORMULA_EXAMPLES', 'FORMULA_FIXTURE', 'RUSTUP_TOOLCHAIN'):
                    os.environ.pop(name, None)
                os.environ.update({name: str(sandbox / suffix) for name, suffix in GOOD['STORES'].items()})
                current = dict(os.environ)
                Path.mkdir = os.mkdir = Path.write_text = Path.write_bytes = subprocess.run = shutil.copytree = shutil.rmtree = effect
                ns = runpy.run_path(str(mirrors[entry]))
                assert ns['contracts'](verbose=False) == COUNTS[entry]
                assert dict(os.environ) == current and sys.argv is argv, 'origin pure positive changed borrowed state'
            finally:
                Path.mkdir, os.mkdir, Path.write_text, Path.write_bytes, subprocess.run, shutil.copytree, shutil.rmtree = originals
                os.environ.clear(); os.environ.update(saved); sys.argv = argv
    print('origin pure positives:', count, 'actual unused-sidebar/output and readonly-hardlink calls; no effects')


def provider_controls():
    """Independent actual profile classes and delegated refusals expose adoption omissions."""
    for entry in ENTRIES:
        def check(scripts=None, delegated=False):
            texts = dict(SCRIPTS) | (scripts or {})
            originals = (runpy.run_path, Path.mkdir, os.mkdir, Path.write_text, Path.write_bytes, subprocess.run)
            saved, argv = dict(os.environ), sys.argv
            def effect(*args, **kwargs):
                raise AssertionError('origin provider check produced effects')
            def load(file, *args, **kwargs):
                path = Path(file)
                if path in texts and texts[path] != SCRIPTS[path]:
                    ns = dict(__file__=str(path), __name__='borrowed_origin_provider')
                    exec(compile(texts[path], str(path), 'exec'), ns)
                else:
                    ns = originals[0](file, *args, **kwargs)
                if path == SIGNATURE and delegated:
                    def fail(*args, **kwargs):
                        raise ns['PROFILE']['Refusal']('isolated delegated origin refusal')
                    ns['load_reference'] = fail
                return ns
            stderr = io.StringIO()
            try:
                runpy.run_path = load
                Path.mkdir = os.mkdir = Path.write_text = Path.write_bytes = subprocess.run = effect
                ns = dict(__file__=str(entry), __name__='__main__' if delegated else 'borrowed_origin_provider')
                with contextlib.redirect_stderr(stderr), contextlib.redirect_stdout(io.StringIO()):
                    if delegated:
                        try:
                            exec(compile(texts[entry], str(entry), 'exec'), ns)
                        except SystemExit as error:
                            assert error.code == 2 and stderr.getvalue() == 'local-environment: REFUSED — isolated delegated origin refusal\n', 'origin delegated CLI identity changed'
                        else:
                            raise AssertionError('origin delegated refusal escaped')
                    else:
                        exec(compile(texts[entry], str(entry), 'exec'), ns)
                        assert ns['PROFILE'] is ns['HELPER']['PROFILE'] is ns['FIXTURE']['PROFILE'] is ns['FIXTURE']['CONTEXT']['PROFILE'], 'origin independent provider identity changed'
                assert dict(os.environ) == saved and sys.argv is argv, 'origin provider changed borrowed state'
            finally:
                runpy.run_path, Path.mkdir, os.mkdir, Path.write_text, Path.write_bytes, subprocess.run = originals
                os.environ.clear(); os.environ.update(saved); sys.argv = argv
        check(); check(delegated=True)
        for path, anchor in ((entry, "PROFILE = FIXTURE['PROFILE']\n"), (entry, "HELPER['PROFILE'] = PROFILE\n"), (PROVIDER, "PROFILE = CONTEXT['PROFILE']\n")):
            assert SCRIPTS[path].count(anchor) == 1
            try:
                check({path: SCRIPTS[path].replace(anchor, '', 1)})
            except AssertionError as error:
                assert str(error) == 'origin independent provider identity changed'
            else:
                raise AssertionError('origin independent adoption omission escaped')
    print('origin providers:four actual inert/delegated positives; six actual adoption omission reds; state/source exact')


def main():
    for entry in ENTRIES:
        for mode in ('import', 'plan', 'contracts', 'consumer', 'cli'):
            print('origin capture:', entry.name, mode, capture(entry, mode))
        selectors = {name: str(file.relative_to(ROOT)) for name, file in (
            ('FORMULA_CONTRACT', BOOK / 'spec/formula-language.md'), ('FORMULA_GRAMMAR', BOOK / 'spec/formula-language/grammar.md'),
            ('FORMULA_EXAMPLES', BOOK / 'spec/formula-language/examples.md'), ('FORMULA_FIXTURE', BOOK / 'spec/reference-skirt.md'),
            ('FORMULA_PARTS_DIR', BOOK / 'spec/formula-language'))}
        selectors['FORMULA_ROOT'] = 'unused-root-preserved'
        capture(entry, 'consumer', selectors=selectors)
    capture(ENTRIES[1], 'consumer-fault')
    original_controls()
    provider_controls()
    physical_controls()
    pure_controls()
    print('origin/provenance controls: pure/producing contracts and actual refusals; intercepted children; real numerical consumers verified separately')


if __name__ == '__main__':
    main()
