"""Platform child preflight and cleanup controls; simulated children are no numeric proof."""
from pathlib import Path
import contextlib
import io
import json
import os
import runpy
import signal
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
ENTRY = ROOT / 'docs/tasks/artifacts/formula_structure/irrational_platform.py'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
WORK = ROOT / 'target/platform-dispatch-controls' / str(os.getpid())
ENV = PROFILE['prepare_producer'](ROOT, sources=(Path(__file__), ENTRY, PROFILE_SOURCE), directories=(WORK,))
ORIGINAL = ENTRY.read_bytes()


def module(script=None):
    scope = dict(__file__=str(ENTRY), __name__='platform_dispatch_control')
    exec(compile(ORIGINAL.decode() if script is None else script, str(ENTRY), 'exec'), scope)
    scope['RUN'], scope['TEMP'] = WORK, WORK / 'temporary'
    return scope


def exercise(kind, defect=None, script=None):
    scope = module(script)
    folder = WORK / (kind + '-' + (defect or 'success'))
    PROFILE['prepare_producer'](ROOT, sources=(ENTRY,), directories=(folder, scope['TEMP']))
    scope['RUN'] = folder
    log = folder / ('capture.log' if kind == 'command' else 'baseline-chrome.log')
    sentinel = folder / 'sentinel'
    wasm = folder / 'module.wasm'
    PROFILE['plan_producer'](ROOT, output_files=(log, sentinel, wasm))
    sentinel.write_bytes(b'unchanged\n')
    wasm.write_bytes(b'SIMULATED wasm;not compiled or executed\n')
    environment = dict(ENV)
    if defect == 'log-symlink':
        log.symlink_to(sentinel)
    elif defect == 'log-hardlink':
        os.link(sentinel, log)
    elif defect == 'stores':
        environment['CARGO_HOME'] = str(ROOT.parent / 'unowned-cargo')
    elif defect == 'temp-store':
        environment['TMPDIR'] = str(ROOT.parent / 'unowned-temporary')
    calls, killed = [], []
    process_method, kill_method = subprocess.Popen, os.killpg

    class Process:
        pid = 2_000_000_000
        returncode = 0

        def communicate(self, timeout=None):
            if kind == 'command':
                return 'SIMULATED stdout\n', 'SIMULATED stderr\n'
            value = dict(status='pass', phase='completed', marker=104, imports=[], userAgent='SIMULATED')
            return '<pre id="result">' + json.dumps(value) + '</pre>', 'SIMULATED browser stderr\n'

    def launch(arguments, **kwargs):
        calls.append(arguments)
        assert kwargs == dict(cwd=ROOT, env=environment, stdout=subprocess.PIPE,
                              stderr=subprocess.PIPE, text=True, start_new_session=True)
        if kind == 'browser':
            assert '--user-data-dir=' + str(folder / 'baseline-profile') in arguments
            assert '--disk-cache-dir=' + str(folder / 'baseline-cache') in arguments
            assert '--crash-dumps-dir=' + str(folder / 'baseline-crashes') in arguments
        return Process()

    def kill(pid, selected):
        assert pid == Process.pid and selected == signal.SIGKILL
        killed.append(pid)

    error = None
    try:
        subprocess.Popen, os.killpg = launch, kill
        try:
            if kind == 'command':
                result = scope['command'](['controlled-child', 'literal argument'], log, environment)
                assert result.returncode == 0 and result.stdout == 'SIMULATED stdout\n'
            else:
                result, observed = scope['chromium'](Path('controlled-chrome'), 'baseline', wasm, 104, environment)
                assert result['marker'] == 104 and observed == []
        except (ValueError, OSError) as caught:
            error = caught
    finally:
        subprocess.Popen, os.killpg = process_method, kill_method
    assert sentinel.read_bytes() == b'unchanged\n'
    if defect:
        assert error is not None, 'unsafe child configuration accepted'
        assert not calls, 'invalid destination/store reached child dispatch: ' + kind + '/' + defect
        assert not killed
        if kind == 'browser':
            assert not (folder / 'baseline.html').exists(), 'browser wrote page before preflight'
    else:
        assert error is None and len(calls) == len(killed) == 1, (kind, error, calls, killed)
    return len(calls)


def cleanup_controls():
    scope = module()
    for failure in ('timeout', 'interrupt'):
        class Process:
            pid = 2_000_000_001
            returncode = -9
            calls = 0

            def communicate(self, timeout=None):
                self.calls += 1
                if self.calls == 1:
                    if failure == 'timeout':
                        raise subprocess.TimeoutExpired(['controlled'], timeout)
                    raise KeyboardInterrupt('controlled')
                return '', ''
        for kind in ('command', 'browser'):
            child, killed = Process(), []
            real_process, real_kill = subprocess.Popen, os.killpg
            real_clock = scope['time'].monotonic
            try:
                subprocess.Popen = lambda *args, **kwargs: child
                os.killpg = lambda pid, sig: killed.append((pid, sig))
                if kind == 'browser' and failure == 'timeout':
                    clock = iter((0, 0, 100))
                    scope['time'].monotonic = lambda: next(clock)
                with contextlib.redirect_stdout(io.StringIO()):
                    try:
                        if kind == 'command':
                            scope['command'](['controlled'], WORK / 'failure.log', ENV)
                        else:
                            scope['chromium'](Path('controlled-chrome'), 'failure',
                                WORK / 'command-success/module.wasm', 104, ENV)
                    except (subprocess.TimeoutExpired, TimeoutError, KeyboardInterrupt):
                        pass
                    else:
                        raise AssertionError('child failure swallowed')
                assert killed == [(child.pid, signal.SIGKILL)] and child.calls == 2
            finally:
                subprocess.Popen, os.killpg = real_process, real_kill
                scope['time'].monotonic = real_clock


def main():
    defects = ('log-symlink', 'log-hardlink', 'stores', 'temp-store')
    if '--diagnose' in sys.argv:
        import re
        revision = sys.argv[sys.argv.index('--diagnose') + 1]
        assert re.fullmatch('[0-9a-f]{40}', revision), 'full diagnostic Git revision required'
        environment = {key:value for key,value in ENV.items() if not key.startswith('GIT_')}
        environment.update(GIT_CONFIG_NOSYSTEM='1', GIT_CONFIG_GLOBAL=os.devnull, GIT_OPTIONAL_LOCKS='0')
        prior = subprocess.check_output(['git', 'show', revision + ':' + ENTRY.relative_to(ROOT).as_posix()],
            cwd=ROOT, env=environment, text=True)
        for kind in ('command', 'browser'):
            for defect in defects:
                try:
                    exercise(kind, defect, prior)
                except AssertionError as error:
                    print('before:', error)
        return
    for kind in ('command', 'browser'):
        exercise(kind)
        for defect in defects:
            exercise(kind, defect)
    cleanup_controls()
    source = ORIGINAL.decode()
    faults = (
        ('command-plan', "    PROFILE['plan_producer'](ROOT, output_files=(log,))\n", '', 'command', 'log-symlink'),
        ('command-store', "    PROFILE['verify'](ROOT, environment)\n", '', 'command', 'stores'),
        ('browser-store', "    PROFILE['verify'](ROOT, environment)\n", '', 'browser', 'stores'),
    )
    for label, old, new, kind, defect in faults:
        assert old in source, label
        # Environment guard appears in both public helpers; omit only the selected helper's guard.
        if label == 'command-store':
            changed = source.replace(old, new, 1)
        elif label == 'browser-store':
            prefix, rest = source.split('def chromium(', 1)
            changed = prefix + 'def chromium(' + rest.replace(old, new, 1)
        else:
            changed = source.replace(old, new, 1)
        # Fresh fixture names keep immutable refusal fixtures independent.
        original_work = globals()['WORK']
        globals()['WORK'] = original_work / label
        PROFILE['prepare_producer'](ROOT, sources=(ENTRY,), directories=(globals()['WORK'],))
        try:
            try:
                exercise(kind, defect, changed)
            except AssertionError as error:
                expected = ('invalid destination/store reached child dispatch' if label == 'command-plan'
                            else 'unsafe child configuration accepted')
                assert str(error).startswith(expected), (label, error)
            else:
                raise AssertionError('actual platform dispatch omission survived: ' + label)
        finally:
            globals()['WORK'] = original_work
    page_line = '    save(html, page(wasm, marker))\n'
    anchor = "    html, dom, log = (RUN / (label + suffix) for suffix in ('.html', '-dom.html', '-chrome.log'))\n"
    assert source.count(page_line) == source.count(anchor) == 1
    changed = source.replace(page_line, '').replace(anchor, anchor + page_line)
    original_work = globals()['WORK']
    globals()['WORK'] = original_work / 'early-browser-page'
    PROFILE['prepare_producer'](ROOT, sources=(ENTRY,), directories=(globals()['WORK'],))
    try:
        try:
            exercise('browser', 'log-symlink', changed)
        except AssertionError as error:
            assert str(error) == 'browser wrote page before preflight', error
        else:
            raise AssertionError('actual early page write omission survived')
    finally:
        globals()['WORK'] = original_work
    assert ENTRY.read_bytes() == ORIGINAL
    print('platform dispatch:2 simulated successes/8 physical path+store refusals/'
          '4 simulated timeout+interrupt cleanups/4 actual body omissions rejected;no native/browser verdict;rc=0')


if __name__ == '__main__':
    main()
