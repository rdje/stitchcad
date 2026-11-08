"""Run unchanged public numeric consumers natively and in actual Chrome WebAssembly.

Only test-discovery attributes are removed in isolated copies;original assertion bodies stay
unchanged. Six host-only Python fixture regenerators are explicitly excluded. A wrong independent
expectation must fail native execution and trap in the actual browser after successful compilation.
"""
from pathlib import Path
from datetime import datetime, timezone
from html.parser import HTMLParser
import argparse
import ast
import base64
import hashlib
import json
import os
import platform
import re
import runpy
import shutil
import signal
import subprocess
import time
from types import SimpleNamespace

ROOT = Path(__file__).resolve().parents[4]
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
CLEANUP = ROOT / 'docs/tasks/artifacts/artifact_cleanup/cleanup.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
PROFILE['plan_producer'](ROOT, sources=(Path(__file__), PROFILE_SOURCE, CLEANUP))
CONSUMERS = (
    'formula_root_value_contract.rs', 'formula_arc_value_contract.rs',
    'formula_sincos_value_contract.rs', 'formula_inverse_value_contract.rs',
    'formula_tangent_oracle_contract.rs', 'formula_tangent_value_contract.rs',
    'formula_evaluated_tangent_contract.rs', 'formula_tangent_resource_contract.rs',
    'formula_tangent_prepare_contract.rs',
)
EXCLUDED = {
    'independent_isqrt_fraction_reference_regenerates_the_entire_root_fixture',
    'independent_fraction_pi_reference_regenerates_every_original_arc_fixture_byte',
    'retained_oracle_is_reproduced_without_native_folding_or_pi_input',
    'independent_oracle_reproduces_the_retained_fixture',
    'independent_fraction_producer_regenerates_every_frozen_byte',
    'independent_resource_producer_regenerates_every_frozen_byte',
}
STORE = ROOT / 'target/irrational_platform_certificate'
RUN = STORE / ('0640-' + datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%S%fZ-') + str(os.getpid()))
WORK = RUN / 'workspace'
TEMP = ROOT / 'target' / ('t' + hashlib.sha256(RUN.name.encode()).hexdigest()[:9])
LATEST = STORE / 'latest.json'
PENDING = STORE / ('latest-' + RUN.name + '.tmp')
TESTS = ROOT / 'crates/sc-core/tests'
TEST_PATTERN = re.compile(r'^#\[test\]\n(?:#\[allow[^\n]+\]\n)*fn ([a-z0-9_]+)\(', re.M)
OWNED_RUN_ID = None


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def relative(path):
    return path.relative_to(ROOT).as_posix()


def inventory_environment():
    """Inspect this project's index without borrowing unrelated Git configuration."""
    prepared = PROFILE['environment_for'](ROOT, os.environ, create=False)
    environment = {name: value for name, value in prepared.items() if not name.startswith('GIT_')}
    environment.update(GIT_CONFIG_NOSYSTEM='1', GIT_CONFIG_GLOBAL=os.devnull,
                       GIT_OPTIONAL_LOCKS='0', GIT_LITERAL_PATHSPECS='1')
    return environment


def save(path, value):
    PROFILE['plan_producer'](ROOT, output_files=(path,))
    path.write_text(value)


def save_new(path, value):
    """An immutable receipt or pending pointer must never replace an earlier writer's file."""
    PROFILE['plan_producer'](ROOT, output_files=(path,))
    with path.open('x') as stream:
        stream.write(value)


def reserve_directory(path):
    """Only exclusive creation earns ownership of a capture or temporary root."""
    path.mkdir()
    info = path.stat()
    return info.st_dev, info.st_ino


def write_failure(error):
    """A reused PID or refused preflight must never amend an earlier retained capture."""
    if RUN.is_dir() and not RUN.is_symlink():
        current = RUN.stat()
        if (current.st_dev, current.st_ino) == OWNED_RUN_ID:
            save_new(RUN / 'failure.json', json.dumps(dict(status='incomplete',error_type=type(error).__name__,
                error=str(error).replace(str(ROOT),'.'),temporary_root=relative(TEMP),
                scope='failed capture;no numerical verdict'),indent=2)+'\n')


def protocol_controls():
    """Exercise the actual failure writer with a fake sink;no fixture or browser side effects."""
    original = Path(__file__).read_bytes()
    function = next(node for node in ast.parse(original).body
        if isinstance(node, ast.FunctionDef) and node.name == 'write_failure')
    class Capture:
        def __init__(self, link=False):
            self.link = link
        def is_dir(self):
            return True
        def is_symlink(self):
            return self.link
        def stat(self):
            return SimpleNamespace(st_dev=1, st_ino=2)
        def __truediv__(self, name):
            return 'controlled/' + name
    def cases(body):
        for owner, link, expected in ((None,False,0), ((1,99),False,0), ((1,2),True,0), ((1,2),False,1)):
            writes = []
            scope = dict(globals(), RUN=Capture(link), OWNED_RUN_ID=owner,
                save_new=lambda path,value: writes.append((path,value)))
            exec(compile(ast.fix_missing_locations(ast.Module(body=[body],type_ignores=[])),__file__,'exec'),scope)
            scope['write_failure'](AssertionError('controlled'))
            assert len(writes) == expected, 'capture ownership refusal bypassed'
    cases(function)
    mutated = next(node for node in ast.parse(original).body
        if isinstance(node, ast.FunctionDef) and node.name == 'write_failure')
    comparisons = [node for node in ast.walk(mutated) if isinstance(node, ast.If)
        and isinstance(node.test, ast.Compare)]
    assert len(comparisons) == 1
    comparisons[0].test = ast.Constant(value=True)
    try:
        cases(mutated)
    except AssertionError as error:
        assert str(error) == 'capture ownership refusal bypassed'
    else:
        raise AssertionError('actual capture-ownership omission survived')
    assert Path(__file__).read_bytes() == original
    print('platform protocol:4 ownership cases/1 actual omission red;no filesystem writes;rc=0',flush=True)


def command(arguments, log, environment, *, timeout=180):
    PROFILE['verify'](ROOT, environment)
    PROFILE['plan_producer'](ROOT, output_files=(log,))
    process = subprocess.Popen(arguments, cwd=ROOT, env=environment, stdout=subprocess.PIPE,
        stderr=subprocess.PIPE, text=True, start_new_session=True)
    try:
        stdout, stderr = process.communicate(timeout=timeout)
    finally:
        try:
            os.killpg(process.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        except PermissionError:
            # macOS answers EPERM for a group whose members have exited but are unreaped (D409).
            if process.poll() is None:
                raise
        process.communicate()
    save(log, (stdout + stderr).replace(str(ROOT), '.'))
    return subprocess.CompletedProcess(arguments, process.returncode, stdout, stderr)


def browser(environment):
    for name in ('google-chrome', 'chromium', 'chrome'):
        found = shutil.which(name, path=environment.get('PATH'))
        if found:
            return Path(found)
    assert platform.system() == 'Darwin', 'Chrome launcher unavailable'
    executable = (Path(Path.home().anchor) / 'Applications/Google Chrome.app'
                  / 'Contents/MacOS/Google Chrome')
    assert executable.is_file(), 'installed Chrome executable unavailable'
    return executable


class PageResult(HTMLParser):
    def __init__(self):
        super().__init__()
        self.active, self.count, self.parts = False, 0, []

    def handle_starttag(self, tag, attributes):
        if tag == 'pre' and dict(attributes).get('id') == 'result':
            self.active, self.count = True, self.count + 1

    def handle_endtag(self, tag):
        if tag == 'pre':
            self.active = False

    def handle_data(self, data):
        if self.active:
            self.parts.append(data)


def page(wasm, marker):
    encoded = json.dumps(base64.b64encode(wasm.read_bytes()).decode())
    return '''<!doctype html><meta charset="utf-8">
<meta http-equiv="Content-Security-Policy" content="default-src 'none';script-src 'unsafe-inline' 'wasm-unsafe-eval'">
<title>Owned irrational numerical execution</title><pre id="result">pending</pre><script>
let phase='decoding';let result;
try {
 const bytes=Uint8Array.from(atob(ENCODED),c=>c.charCodeAt(0));
 const module=new WebAssembly.Module(bytes);phase='compiled';
 if(WebAssembly.Module.imports(module).length!==0)throw Error('unexpected host imports');
 const instance=new WebAssembly.Instance(module,{});phase='instantiated';
 if(typeof instance.exports.main!=='function')throw Error('actual main unavailable');
 phase='executing';const marker=instance.exports.main(0,0);phase='completed';
 if(marker!==MARKER)throw Error('wrong actual completion marker:'+marker);
 result={status:'pass',phase,marker,imports:WebAssembly.Module.imports(module),userAgent:navigator.userAgent};
} catch(error) {
 result={status:error instanceof WebAssembly.RuntimeError?'trap':'error',phase,name:error.name,message:String(error)};
}
document.getElementById('result').textContent=JSON.stringify(result);
</script>'''.replace('ENCODED', encoded).replace('MARKER', str(marker))


def ambient_census(path):
    if path is None or not path.is_dir():
        return {}
    result = {}
    for entry in path.iterdir():
        if not any(token in entry.name.lower() for token in ('chrome', 'chromium', 'singleton')):
            continue
        try:
            info = entry.lstat()
        except FileNotFoundError:
            continue
        result[entry.name] = (info.st_size, info.st_mtime_ns)
    return result


def chromium(executable, label, wasm, marker, environment):
    PROFILE['verify'](ROOT, environment)
    html, dom, log = (RUN / (label + suffix) for suffix in ('.html', '-dom.html', '-chrome.log'))
    profile, cache, crashes = (RUN / (label + suffix) for suffix in ('-profile', '-cache', '-crashes'))
    PROFILE['plan_producer'](ROOT, sources=(wasm,), directories=(profile, cache, crashes, TEMP),
        output_files=(html, dom, log))
    save(html, page(wasm, marker))
    arguments = [str(executable), '--headless', '--no-first-run', '--no-default-browser-check',
        '--disable-background-networking', '--disable-component-update', '--disable-sync',
        '--disable-crash-reporter', '--disable-breakpad', '--disable-gpu',
        '--user-data-dir=' + str(profile), '--disk-cache-dir=' + str(cache),
        '--crash-dumps-dir=' + str(crashes), '--dump-dom', '--timeout=60000', html.as_uri()]
    process = subprocess.Popen(arguments, cwd=ROOT, env=environment, stdout=subprocess.PIPE,
        stderr=subprocess.PIPE, text=True, start_new_session=True)
    observed = set()
    try:
        deadline = time.monotonic() + 90
        while True:
            for item in TEMP.iterdir():
                observed.add(item.name)
                assert item.lstat().st_dev == ROOT.stat().st_dev, 'browser temporary volume drift'
            if time.monotonic() >= deadline:
                raise TimeoutError('owned browser execution timeout')
            try:
                stdout, stderr = process.communicate(timeout=0.05)
                break
            except subprocess.TimeoutExpired:
                pass
    finally:
        try:
            os.killpg(process.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        except PermissionError:
            # macOS answers EPERM for a group whose members have exited but are unreaped (D409).
            if process.poll() is None:
                raise
        process.communicate()
    save(dom, stdout.replace(str(ROOT), '.'))
    save(log, stderr.replace(str(ROOT), '.'))
    assert process.returncode == 0, ('Chrome process failed', process.returncode)
    parsed = PageResult()
    parsed.feed(stdout)
    assert parsed.count == 1, 'one actual DOM result required'
    result = json.loads(''.join(parsed.parts))
    assert result['phase'] in ('completed', 'executing'), 'compilation/protocol failure is not execution'
    return result, sorted(observed)


def main():
    global OWNED_RUN_ID
    OWNED_RUN_ID = None
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--plan-only', action='store_true')
    args = parser.parse_args()
    protocol_controls()
    PROFILE['plan_producer'](ROOT, directories=(STORE, RUN, WORK, TEMP))
    inventory = subprocess.run(['git', 'ls-files', '-z', 'crates'], cwd=ROOT,
        env=inventory_environment(),
        check=True, capture_output=True).stdout.decode().split('\0')
    sources = {ROOT / name for name in inventory if name}
    sources.update((Path(__file__), PROFILE_SOURCE, CLEANUP, ROOT / 'Cargo.toml', ROOT / 'Cargo.lock',
        ROOT / 'rust-toolchain.toml', ROOT / 'scripts/check_handoff.sh', ROOT / 'scripts/handoff_census.py'))
    PROFILE['plan_producer'](ROOT, sources=tuple(sources))
    frozen = {path:path.read_bytes() for path in sources}
    before = {relative(path):hashlib.sha256(data).hexdigest() for path,data in frozen.items()}
    copied, selected, fixtures, exclusions, fixture_sources = {}, {}, {}, set(), set()
    for name in CONSUMERS:
        source = TESTS / name
        text = frozen[source].decode()
        functions = TEST_PATTERN.findall(text)
        assert len(functions) == text.count('#[test]') and '#[cfg(test)]' not in text
        exclusions.update(set(functions) & EXCLUDED)
        selected[name] = [name for name in functions if name not in EXCLUDED]
        wrapper = '\npub fn platform_check() -> u8 {\n' + ''.join(
            '    ' + name + '();\n' for name in selected[name]) + '    %d\n}\n' % len(selected[name])
        copied[WORK / 'consumers' / name] = '#![allow(dead_code)]\n' + text.replace('#[test]\n', '') + wrapper
        for fixture in re.findall(r'include_str!\("([^"\n]+)"\)', text):
            original = source.parent / fixture
            assert original in frozen, 'consumer fixture is outside the declared original inventory'
            fixture_sources.add(original)
            fixtures[WORK / 'consumers' / fixture] = frozen[original]
    assert exclusions == EXCLUDED and sum(map(len, selected.values())) == 31
    assert len(fixtures) == 6 and sum(len(data.splitlines()) - 1 for data in fixtures.values()) == 405
    logs = ('rustc', 'chrome-version', 'rustc-final', 'chrome-version-final', 'lock', 'handoff', *(f'{label}-{kind}'
        for label in ('baseline', 'wrong-fixture') for kind in ('native-build', 'wasm-build', 'native-run')))
    outputs = (WORK / 'Cargo.toml', WORK / 'Cargo.lock', WORK / 'main.rs', *copied, *fixtures,
        *(RUN / (name + '.log') for name in logs), RUN / 'report.json', RUN / 'failure.json', LATEST, PENDING,
        *(RUN / (label + suffix) for label in ('baseline', 'wrong-fixture')
          for suffix in ('.native', '.wasm', '.html', '-dom.html', '-chrome.log')))
    sealed_temp = RUN / 'browser-temporary'
    # The copy builds into its own directory, never the shared target, which cargo would then watch (D400, D407).
    directories = {STORE, RUN, WORK, TEMP, sealed_temp, RUN / 'cargo-target', *(path.parent for path in outputs),
        *(RUN / (label + suffix) for label in ('baseline', 'wrong-fixture')
          for suffix in ('-profile', '-cache', '-crashes'))}
    PROFILE['plan_producer'](ROOT, sources=tuple(sources), directories=tuple(directories), output_files=outputs)
    assert not RUN.exists() and not TEMP.exists() and not PENDING.exists(), 'fresh owned capture/temp roots required'
    if args.plan_only:
        print('irrational platform plan:9 original consumers/31 selected/6 host-only exclusions;'
            '6 original fixtures/405 rows;no writes,compiler or browser dispatch;rc=0')
        return
    cleanup = runpy.run_path(str(CLEANUP))
    assert 'target/irrational_platform_certificate' in cleanup['PROTECTED'], 'register retained capture before creation'
    # The complete plan above precedes store activation;only the common parent may be reused.
    environment = PROFILE['prepare_producer'](ROOT, sources=tuple(sources),
        directories=(STORE,), output_files=outputs)
    OWNED_RUN_ID = reserve_directory(RUN)
    temp_owner = reserve_directory(TEMP)
    environment = PROFILE['prepare_producer'](ROOT, sources=tuple(sources),
        directories=tuple(directories), output_files=outputs)
    environment['TMPDIR'] = str(TEMP)
    environment['MAKE_TMPDIR'] = str(TEMP)
    environment['TMP'] = str(TEMP)
    environment['TEMP'] = str(TEMP)
    assert TEMP.stat().st_dev == ROOT.stat().st_dev
    ambient = Path(os.environ['TMPDIR']) if os.environ.get('TMPDIR') else None
    ambient_before = ambient_census(ambient)
    executable = browser(environment)
    browser_digest = digest(executable)
    versions = {
        'rustc': command(['rustc', '-vV'], RUN / 'rustc.log', environment),
        'chrome': command([str(executable), '--version'], RUN / 'chrome-version.log', environment),
    }
    assert all(result.returncode == 0 for result in versions.values())
    manifest = '''[package]
name = "sc-irrational-platform"
version = "0.0.0"
edition = "2021"
publish = false
[workspace]
resolver = "2"
[dependencies]
sc-core = { path = %s }
[[bin]]
name = "sc-irrational-platform"
path = "main.rs"
[lints.rust]
unsafe_code = "forbid"
''' % json.dumps(os.path.relpath(ROOT / 'crates/sc-core', WORK).replace(os.sep, '/'))
    save(WORK / 'Cargo.toml', manifest)
    for destination, text in copied.items():
        save(destination, text)
    for destination, data in fixtures.items():
        PROFILE['plan_producer'](ROOT, output_files=(destination,))
        destination.write_bytes(data)
    driver = '#![forbid(unsafe_code)]\n' + ''.join(
        '#[path = "consumers/%s"]\nmod c%d;\n' % (name, index) for index, name in enumerate(CONSUMERS))
    driver += 'fn main() -> std::process::ExitCode {\n    let mut completed = 0_u8;\n'
    driver += ''.join('    completed += c%d::platform_check();\n' % index for index in range(len(CONSUMERS)))
    driver += '    assert_eq!(completed, 31);\n    std::process::ExitCode::from(completed + 73)\n}\n'
    save(WORK / 'main.rs', driver)
    result = command(['cargo', 'generate-lockfile', '--offline', '--manifest-path', str(WORK / 'Cargo.toml')],
        RUN / 'lock.log', environment)
    assert result.returncode == 0, 'offline adapter dependency resolution'
    def packages(text):
        return {tuple(re.search(r'(?m)^' + field + r' = "([^"\n]+)"$', block).group(1)
            if re.search(r'(?m)^' + field + r' = "([^"\n]+)"$', block) else None
            for field in ('name', 'version', 'source', 'checksum')) for block in text.split('\n[[package]]\n')[1:]}
    resolved = packages((WORK / 'Cargo.lock').read_text())
    pinned = packages((ROOT / 'Cargo.lock').read_text())
    assert {item for item in resolved if item[0] != 'sc-irrational-platform'} <= pinned, 'adapter dependency version drift'
    print('platform capture:31 unchanged public-API checks;compiler and Chrome identified', flush=True)
    wrong = WORK / 'consumers/fixtures/formula_tangent.tsv'
    PROFILE['plan_producer'](ROOT, sources=(wrong,), mutable_sources=(wrong,), output_files=(wrong,))
    original = wrong.read_bytes()
    anchor = b'degree_-720\t-720000000\t1\tvalue\t0\n'
    assert original.count(anchor) == 1
    outcomes = []
    try:
        for label in ('baseline', 'wrong-fixture'):
            if label == 'wrong-fixture':
                PROFILE['plan_producer'](ROOT, sources=(wrong,), mutable_sources=(wrong,), output_files=(wrong,))
                wrong.write_bytes(original.replace(anchor, anchor.replace(b'\t0\n', b'\t1\n'), 1))
            binaries = {}
            for kind, target, suffix in (('native', None, '.native'), ('wasm', 'wasm32-unknown-unknown', '.wasm')):
                arguments = ['cargo', 'build', '--offline', '--locked', '--release', '--manifest-path',
                    str(WORK / 'Cargo.toml'), '--target-dir', str(RUN / 'cargo-target'), '--message-format=json']
                if target:
                    arguments.extend(('--target', target))
                built = command(arguments, RUN / (label + '-' + kind + '-build.log'), environment)
                messages = [json.loads(line) for line in built.stdout.splitlines() if line.startswith('{')]
                assert built.returncode == 0 and any(item.get('reason') == 'build-finished' and item['success']
                    for item in messages), 'compiler failure cannot count as runtime rejection'
                artifacts = [Path(item['executable']) for item in messages
                    if item.get('reason') == 'compiler-artifact' and item.get('executable')
                    and item.get('target', {}).get('name') == 'sc-irrational-platform']
                assert len(artifacts) == 1
                destination = RUN / (label + suffix)
                PROFILE['plan_producer'](ROOT, sources=(artifacts[0],), output_files=(destination,))
                shutil.copyfile(artifacts[0], destination)
                destination.chmod(destination.stat().st_mode | 0o100)
                binaries[kind] = destination
            native = command([str(binaries['native'])], RUN / (label + '-native-run.log'), environment)
            actual, observed = chromium(executable, label, binaries['wasm'], 104, environment)
            if label == 'baseline':
                assert native.returncode == 104 and actual['status'] == 'pass' and actual['marker'] == 104
            else:
                assert native.returncode == 101 and 'degree_-720' in native.stderr
                assert actual['status'] == 'trap' and actual['phase'] == 'executing' and actual['name'] == 'RuntimeError'
            outcomes.append(dict(label=label, native_rc=native.returncode, browser=actual,
                binaries={kind:dict(path=relative(path),sha256=digest(path)) for kind,path in binaries.items()},
                observed_local_temp_names=observed))
            print('platform:' + label + (' native+actual Chrome pass' if label == 'baseline'
                else ' compiled wrong expectation rejected natively+actual browser trap'), flush=True)
    finally:
        PROFILE['plan_producer'](ROOT, sources=(wrong,), mutable_sources=(wrong,), output_files=(wrong,))
        wrong.write_bytes(original)
    assert all(path.read_text() == text for path, text in copied.items())
    assert all(path.read_bytes() == data for path, data in fixtures.items())
    assert {relative(path):digest(path) for path in sources} == before, 'original project source drift'
    for name, arguments in (('rustc', ['rustc', '-vV']), ('chrome', [str(executable), '--version'])):
        final = command(arguments, RUN / (name.replace('chrome', 'chrome-version') + '-final.log'), environment)
        assert final.returncode == 0 and final.stdout == versions[name].stdout, 'tool version changed during capture'
    ambient_after = ambient_census(ambient)
    added_ambient = sorted(set(ambient_after) - set(ambient_before))
    assert not added_ambient, ('new ambient Chrome temporary entries require investigation', added_ambient)
    handoff = command(['bash', 'scripts/check_handoff.sh', '--idle-cua'], RUN / 'handoff.log', environment)
    assert handoff.returncode == 0 and 'handoff: OK' in handoff.stdout, 'project process census is not clear;inspect handoff.log'
    assert digest(executable) == browser_digest, 'browser launcher changed during capture'
    def temp_census(directory):
        entries = []
        for path in sorted(directory.rglob('*')):
            info = path.lstat()
            assert info.st_dev == ROOT.stat().st_dev and path.name != '.git'
            entries.append(dict(path=path.relative_to(directory).as_posix(), bytes=info.st_size,
                kind='link' if path.is_symlink() else 'directory' if path.is_dir() else 'file',
                sha256=digest(path) if not path.is_symlink() and path.is_file() else None,
                link=os.readlink(path) if path.is_symlink() else None))
        return entries
    assert not TEMP.is_symlink() and (TEMP.stat().st_dev, TEMP.stat().st_ino) == temp_owner, \
        'owned temporary root changed before sealing'
    temp_entries = temp_census(TEMP)
    PROFILE['plan_producer'](ROOT, directories=(TEMP, sealed_temp))
    assert sealed_temp.is_dir() and not list(sealed_temp.iterdir()), 'owned empty temp destination changed'
    sealed_temp.rmdir()
    TEMP.rename(sealed_temp)
    assert not TEMP.exists() and temp_census(sealed_temp) == temp_entries, 'temporary capture move verification'
    report = dict(version=1,scope='31 original public numeric consumer checks;actual native and Chrome WebAssembly',
        utc=datetime.now(timezone.utc).isoformat(), host=dict(system=platform.system(),release=platform.release(),machine=platform.machine()),
        rustc=versions['rustc'].stdout,chrome=versions['chrome'].stdout.strip(),
        consumers=selected,excluded_host_only=sorted(EXCLUDED),fixtures={relative(source):digest(source)
            for source in sorted(fixture_sources)}, chrome_executable_sha256=browser_digest,
        original_sources_unchanged=before,selected_checks=31,original_fixture_rows=405,
        outcomes=outcomes,temporary_root=relative(TEMP),temporary_sealed_at=relative(sealed_temp),temporary_entries=temp_entries,
        new_ambient_chrome_entries=added_ambient,workspace_restored=True,
        locality_scope='direct source/artifact/profile/temp boundaries;complete transitive compiler/browser review remains n16')
    save_new(RUN / 'report.json', json.dumps(report, indent=2) + '\n')
    save_new(PENDING, json.dumps(dict(report=relative(RUN / 'report.json'),sha256=digest(RUN / 'report.json')),indent=2)+'\n')
    PROFILE['plan_producer'](ROOT, sources=(PENDING,), output_files=(LATEST,))
    PENDING.replace(LATEST)
    print('irrational platform:31 checks/native+actual Chrome/bad expectation rejected/originals restored;report='
        + relative(RUN / 'report.json') + ';rc=0')


if __name__ == '__main__':
    try:
        main()
    except BaseException as error:
        write_failure(error)
        raise
