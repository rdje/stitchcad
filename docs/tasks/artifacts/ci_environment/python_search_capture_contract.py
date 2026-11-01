"""Watch fresh search captures with intercepted renderers; native search evidence stays separate."""
from pathlib import Path
import contextlib
import hashlib
import io
import json
import os
import runpy
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
ENTRY = ROOT / 'docs/tasks/artifacts/book_publication/search_migration_capture.py'
PUBLICATION = ENTRY.with_name('book_publication.py')
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
CORE = (Path(__file__), ENTRY, PUBLICATION, PROFILE_SOURCE)
WORK = ROOT / 'target/search-capture-controls' / str(os.getpid())
PROFILE['prepare_producer'](ROOT, sources=CORE, directories=(WORK,))
INPUTS = (ROOT / 'docs/book/book.toml', *sorted(path for path in (ROOT / 'docs/book/src').rglob('*') if path.is_file()),
          ROOT / 'scripts/book_tool.py', PROFILE_SOURCE, ROOT / 'target/cargo-home/bin/mdbook')
PROFILE['plan_producer'](ROOT, sources=(*CORE, *INPUTS))
FROZEN = {path: path.read_bytes() for path in (*CORE, *INPUTS)}


def module(script=None):
    namespace = dict(__file__=str(ENTRY), __name__='search_capture_control')
    exec(compile(ENTRY.read_text() if script is None else script, str(ENTRY), 'exec'), namespace)
    return namespace


def capture(label, script=None, *, outcome='success'):
    """Run the actual entry; only the three mdBook children are replaced with data fixtures."""
    ns, work = module(script), WORK / label
    assert not work.exists()
    dirs = {work, work / 'level3', work / 'level1'}
    outputs = {work / name for name in ('source-sha256.json', 'measurements.json', 'tool-version.txt', 'level3.log', 'level1.log')}
    sources = {ENTRY, PROFILE_SOURCE, PUBLICATION, *INPUTS}
    real = ns['PROFILE']
    originals = Path.mkdir, Path.write_text, Path.read_bytes, subprocess.run
    saved, argv = dict(os.environ), sys.argv[:]
    stores = {key: str(WORK / 'caller-stores' / key.lower()) for key in PROFILE['STORES']}
    PROFILE['prepare_producer'](ROOT, sources=CORE, directories=tuple(Path(value) for value in stores.values()))
    os.environ.update(stores, SEARCH_CAPTURE_CALLER='retained')
    planned_files, planned_dirs, planned_sources = set(), set(), set()
    writes, children, created = [], [], []
    activated = False

    def complete():
        assert sources <= planned_sources, 'search capture input plan omitted'
        assert dirs <= planned_dirs, 'search capture directory plan omitted'
        assert outputs <= planned_files, 'search capture file plan omitted'

    def plan(root, **kwargs):
        assert root == ROOT
        planned_sources.update(kwargs.get('sources', ()))
        planned_dirs.update(kwargs.get('directories', ()))
        planned_files.update(kwargs.get('output_files', ()))
        return real['plan_producer'](root, **kwargs)

    def enter(root, **kwargs):
        nonlocal activated
        complete()
        assert sources <= set(kwargs['sources']) and dirs == set(kwargs['directories'])
        assert outputs == set(kwargs['output_files'])
        result = real['enter_producer'](root, **kwargs)
        activated = True
        return result

    def mkdir(path, *args, **kwargs):
        if sys._getframe(1).f_globals is ns:
            complete()
            assert path == work and not args and not kwargs, 'search capture reservation is not exclusive'
            created.append(path)
        return originals[0](path, *args, **kwargs)

    def write(path, data, *args, **kwargs):
        if sys._getframe(1).f_globals is ns:
            complete()
            assert activated and path in outputs, 'search capture unplanned write'
            writes.append(path)
        return originals[1](path, data, *args, **kwargs)

    def read(path, *args, **kwargs):
        value = originals[2](path, *args, **kwargs)
        if outcome == 'source-change' and len(children) >= 2 and path == INPUTS[-1]:
            return value + b'controlled changed read'
        return value

    def child(command, **kwargs):
        complete()
        assert activated and kwargs['cwd'] == ROOT and kwargs['capture_output'] and kwargs['text']
        environment = kwargs.get('env', dict(os.environ))
        assert all(environment.get(key) == value for key, value in stores.items()), 'search capture caller stores reset'
        PROFILE['verify'](ROOT, environment)
        assert environment['SEARCH_CAPTURE_CALLER'] == 'retained'
        children.append(command)
        if len(children) == 1:
            assert command == [str(ROOT / 'target/cargo-home/bin/mdbook'), '--version']
            return subprocess.CompletedProcess(command, 1 if outcome == 'version-failure' else 0, 'mdbook controlled\n', '')
        level = (3, 1)[len(children) - 2]
        destination = work / ('level%d' % level)
        assert command == [str(ROOT / 'target/cargo-home/bin/mdbook'), 'build', 'docs/book', '--dest-dir', str(destination)]
        assert environment['MDBOOK_LOG'] == environment['RUST_LOG'] == 'warn'
        assert environment['MDBOOK_OUTPUT__HTML__SEARCH__HEADING_SPLIT_LEVEL'] == str(level)
        assert environment.get('MDBOOK_BUILD__CREATE_MISSING') == 'false', 'search capture source creation enabled'
        generated = destination / 'searchindex-controlled.js', destination / 'intro.html'
        PROFILE['plan_producer'](ROOT, sources=CORE, output_files=generated)
        data = dict(doc_urls=['intro.html#one', 'intro.html#two'] if level == 3 else ['intro.html'])
        raw = json.dumps(data)
        originals[1](generated[0], 'window.search = Object.assign(window.search, JSON.parse(' + repr(raw) + '));')
        originals[1](generated[1], '<h1>controlled child output</h1>')
        warning = '\x1b[33m WARN search index is very large\x1b[0m\n' if level == 3 else ''
        if outcome == 'unexpected-warning':
            warning = ' WARN unrelated warning\n'
        if outcome == 'candidate-warning' and level == 1:
            warning = ' WARN search index is very large\n'
        if outcome == 'warning-free-section' and level == 3:
            warning = ''
        return subprocess.CompletedProcess(command, 1 if outcome == 'build-failure' else 0, '', warning)

    try:
        ns['PROFILE'] = dict(real, plan_producer=plan, enter_producer=enter)
        Path.mkdir, Path.write_text, Path.read_bytes, subprocess.run = mkdir, write, read, child
        sys.argv = [str(ENTRY), '--output', work.relative_to(ROOT).as_posix()]
        with contextlib.redirect_stdout(io.StringIO()):
            try:
                ns['main']()
            except AssertionError as error:
                if outcome in ('success', 'warning-free-section'):
                    raise
                if outcome == 'source-change':
                    assert error.args == ('native source bytes changed',), error
                elif outcome == 'version-failure':
                    assert error.args == ('',), error
                else:
                    detail = error.args[0]
                    assert isinstance(detail, tuple) and detail[0] == (1 if outcome == 'candidate-warning' else 3), error
                    if outcome == 'build-failure':
                        assert detail[1] == 1
                    else:
                        assert isinstance(detail[1], list)
            else:
                assert outcome in ('success', 'warning-free-section'), 'search capture accepted controlled ' + outcome
        if outcome in ('success', 'warning-free-section'):
            assert len(children) == 3 and writes == [work / name for name in (
                'source-sha256.json', 'tool-version.txt', 'level3.log', 'level1.log', 'measurements.json')]
            assert created == [work], 'search capture exclusive reservation omitted'
            expected = {path.relative_to(ROOT).as_posix(): hashlib.sha256(FROZEN[path]).hexdigest() for path in INPUTS}
            assert json.loads((work / 'source-sha256.json').read_text()) == expected
            result = json.loads((work / 'measurements.json').read_text())
            assert set(result) == {'3', '1'} and result['3']['documents'] == 2 and result['1']['documents'] == 1
            for level in (3, 1):
                file = work / ('level%d' % level) / 'searchindex-controlled.js'
                assert file in planned_sources, 'search capture generated input plan omitted'
                assert result[str(level)]['sha256'] == hashlib.sha256(originals[2](file)).hexdigest()
        else:
            assert work / 'measurements.json' not in writes, 'failed capture published success measurements'
        assert all(os.environ[key] == value for key, value in stores.items())
    finally:
        Path.mkdir, Path.write_text, Path.read_bytes, subprocess.run = originals
        sys.argv = argv
        os.environ.clear(); os.environ.update(saved)
    assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'search capture original inputs changed'


def collision(script, label, *, original=False):
    """Insert a competing owner at the actual mkdir boundary, then trap any attempted overwrite."""
    ns, work = module(script), WORK / label
    real = ns['PROFILE']
    writer, mkdir, child = Path.write_text, Path.mkdir, subprocess.run
    saved, argv = dict(os.environ), sys.argv[:]
    sentinel = work / 'source-sha256.json'
    attempts = []

    def competitor():
        PROFILE['plan_producer'](ROOT, sources=CORE, directories=(work,), output_files=(sentinel,))
        mkdir(work)
        writer(sentinel, 'competing capture remains exact\n')

    def enter(root, **kwargs):
        if original:
            competitor()
        return real['enter_producer'](root, **kwargs)

    def reserve(path, *args, **kwargs):
        if not original and path == work and sys._getframe(1).f_globals is ns:
            competitor()
        return mkdir(path, *args, **kwargs)

    def write(path, data, *args, **kwargs):
        if sys._getframe(1).f_globals is ns:
            attempts.append(path)
            raise RuntimeError('trapped original capture overwrite')
        return writer(path, data, *args, **kwargs)

    def forbidden(*args, **kwargs):
        raise AssertionError('colliding capture reached child')

    try:
        ns['PROFILE'] = dict(real, enter_producer=enter)
        Path.mkdir, Path.write_text, subprocess.run = reserve, write, forbidden
        sys.argv = [str(ENTRY), '--output', work.relative_to(ROOT).as_posix()]
        try:
            ns['main']()
        except RuntimeError as error:
            assert original and str(error) == 'trapped original capture overwrite'
        except FileExistsError:
            assert not original
        else:
            raise AssertionError('colliding capture accepted')
        assert attempts == ([sentinel] if original else [])
        assert sentinel.read_text() == 'competing capture remains exact\n'
        assert dict(os.environ) == saved or original
    finally:
        Path.mkdir, Path.write_text, subprocess.run = mkdir, writer, child
        sys.argv = argv
        os.environ.clear(); os.environ.update(saved)


def refusals():
    ns = module()
    root = WORK / 'refusals'
    regular, link, dangling, existing = (root / name for name in ('file', 'link', 'dangling', 'existing'))
    PROFILE['prepare_producer'](ROOT, sources=CORE, directories=(root, existing), output_files=(regular, link, dangling))
    regular.write_text('unchanged refusal fixture\n')
    link.symlink_to(existing, target_is_directory=True);dangling.symlink_to(root / 'absent')
    cases = (str(root / 'absolute'), 'target/../escape', 'outside-target/capture',
             (regular / 'child').relative_to(ROOT).as_posix(), link.relative_to(ROOT).as_posix(),
             dangling.relative_to(ROOT).as_posix(), existing.relative_to(ROOT).as_posix(),
             (link / 'child').relative_to(ROOT).as_posix(), 'target/.git/capture')
    original, argv, saved = subprocess.run, sys.argv[:], dict(os.environ)

    def forbidden(*args, **kwargs):
        raise RuntimeError('refused search capture reached child')

    try:
        subprocess.run = forbidden
        for value in cases:
            sys.argv = [str(ENTRY), '--output', value]
            try:
                ns['main']()
            except (AssertionError, ns['PROFILE']['Refusal']):
                pass
            else:
                raise AssertionError('invalid search destination accepted: ' + value)
            assert dict(os.environ) == saved
            assert regular.read_text() == 'unchanged refusal fixture\n'
        assert not (root / 'absolute').exists() and not (existing / 'child').exists()
    finally:
        subprocess.run = original
        sys.argv = argv
        os.environ.clear(); os.environ.update(saved)
    return len(cases)


def input_refusals():
    """Use controlled filesystem observations without modifying any real book source."""
    labels = ('missing-source', 'foreign-device', 'renderer-override', 'toolchain')
    for label in labels:
        ns, destination = module(), WORK / ('input-' + label)
        file = INPUTS[-1]
        is_file, stat, child = Path.is_file, Path.stat, subprocess.run
        saved, argv = dict(os.environ), sys.argv[:]
        if label == 'renderer-override':
            os.environ['MDBOOK_OUTPUT'] = '{"foreign":{"command":"unowned-child"}}'
        if label == 'toolchain':
            os.environ['RUSTUP_TOOLCHAIN'] = 'invalid/toolchain'
        configured = dict(os.environ)

        def observed_file(path, *args, **kwargs):
            return False if label == 'missing-source' and path == file else is_file(path, *args, **kwargs)

        def observed_stat(path, *args, **kwargs):
            result = stat(path, *args, **kwargs)
            if label == 'foreign-device' and path == file:
                fields = list(result);fields[2] += 1
                return os.stat_result(fields)
            return result

        def forbidden(*args, **kwargs):
            raise RuntimeError('refused input reached child')

        try:
            Path.is_file, Path.stat, subprocess.run = observed_file, observed_stat, forbidden
            sys.argv = [str(ENTRY), '--output', destination.relative_to(ROOT).as_posix()]
            try:
                ns['main']()
            except (ns['PROFILE']['Refusal'], ns['BOOK']['PublicationError']):
                pass
            else:
                raise AssertionError('unsafe search input accepted: ' + label)
            assert not destination.exists() and dict(os.environ) == configured
        finally:
            Path.is_file, Path.stat, subprocess.run = is_file, stat, child
            sys.argv = argv
            os.environ.clear();os.environ.update(saved)
    return len(labels)


def faults():
    source = ENTRY.read_text()
    variants = (
        ('input-plan', 'sources=sources)', 'sources=())'),
        ('file-plan', 'output_files=outputs, sources=sources)', 'output_files=(), sources=sources)'),
        ('reservation', '    work.mkdir()\n', ''),
        ('store-reset', 'env = dict(os.environ, MDBOOK_LOG=', 'env = dict(MDBOOK_LOG='),
        ('source-create', "MDBOOK_BUILD__CREATE_MISSING='false'", "MDBOOK_BUILD__CREATE_MISSING='true'"),
        ('generated-plan', "PROFILE['plan_producer'](ROOT, sources=(Path(__file__), PUBLICATION_SOURCE, *inputs, *files))", 'pass'),
    )
    for label, old, new in variants:
        assert old in source
        variant = source.replace(old, new)
        try:
            capture('fault-' + label, variant)
        except AssertionError:
            pass
        else:
            raise AssertionError('search capture actual body fault survived: ' + label)
    checks = (
        ('byte-watch', source.replace("        assert hashes == {str(path.relative_to(ROOT)): hashlib.sha256(path.read_bytes()).hexdigest() for path in inputs}, 'native source bytes changed'", '        pass'), 'source-change'),
        ('warning-watch', source.replace('            assert not warnings, (level, warnings)', '            pass'), 'candidate-warning'),
    )
    for label, variant, outcome in checks:
        assert variant != source
        try:
            capture('fault-' + label, variant, outcome=outcome)
        except AssertionError:
            pass
        else:
            raise AssertionError('search capture actual body fault survived: ' + label)
    try:
        collision(source.replace('    work.mkdir()\n', '    work.mkdir(exist_ok=True)\n'), 'fault-nonexclusive')
    except AssertionError:
        pass
    else:
        raise AssertionError('nonexclusive capture collision fault survived')
    return len(variants) + len(checks) + 1


def main():
    capture('complete')
    capture('warning-free-section', outcome='warning-free-section')
    for outcome in ('version-failure', 'build-failure', 'unexpected-warning', 'candidate-warning', 'source-change'):
        capture(outcome, outcome=outcome)
    git_environment = {key: value for key, value in os.environ.items() if not key.startswith('GIT_')}
    git_environment.update(GIT_OPTIONAL_LOCKS='0', GIT_CONFIG_NOSYSTEM='1', GIT_CONFIG_GLOBAL=os.devnull)
    old = subprocess.check_output(['git', 'show', '5acc96a603eab7ea15e80b894d7e36b4bd59bce1:' + ENTRY.relative_to(ROOT).as_posix()],
                                  cwd=ROOT, env=git_environment, text=True)
    collision(old, 'original-collision', original=True)
    collision(ENTRY.read_text(), 'fixed-collision')
    rejected, inputs, mutations = refusals(), input_refusals(), faults()
    assert all(path.read_bytes() == data for path, data in FROZEN.items())
    print('search capture controls: warning/warning-free section baselines/5 writes;5 failure paths;original/fixed collision;'
          f'{rejected} physical/CLI and{inputs} input refusals;{mutations} actual body faults;'
          'original inputs/caller environment exact;native evidence separate;rc=0')


if __name__ == '__main__':
    main()
