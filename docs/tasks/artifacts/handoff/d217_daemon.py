"""D217: controlled installed-daemon descriptor provenance and launch containment.

Only newly owned private Codex homes/processes are changed. No thread/model turn is
requested. Package copy is exclusive, hash-checked, and removed after owned teardown.
"""
import argparse
import copy
import ctypes
import errno
import fcntl
import hashlib
import json
import os
from pathlib import Path
import runpy
import shutil
import signal
import stat
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[4]
PROBE = Path(__file__).with_name('d217_probe.py')
PROFILE = ROOT / 'scripts/local_environment.py'
VERSION = '0.164.0-aarch64-apple-darwin'
CASES = ('inherit', 'closed', 'cloexec', 'removed-directory', 'revoked-pty')
FAULT_CASES = ('inherit', 'removed-directory', 'revoked-pty')


def digest(path):
    h = hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b''):
            h.update(block)
    return h.hexdigest()


def read_record(home, name):
    path = home / 'app-server-daemon' / (name + '.pid')
    for _ in range(40):
        if path.is_file():
            return json.loads(path.read_text())
        time.sleep(.05)
    raise RuntimeError('private daemon did not publish ' + name)


def wait_updater_ready(home):
    path = home / 'app-server-daemon/daemon-updater.sock'
    for _ in range(800):
        if path.exists() and stat.S_ISSOCK(path.stat().st_mode):
            return
        time.sleep(.05)
    raise RuntimeError('private updater did not publish its ready socket')


def process_field(pid, field):
    r = subprocess.run(['ps', '-p', str(pid), '-o', field + '='],
                       capture_output=True, text=True, check=False, timeout=5)
    return r.stdout.strip() if r.returncode == 0 else None


def verify_owned(record, binary):
    pid = record['pid']
    assert pid > 1 and pid != os.getpid()
    if process_field(pid, 'lstart') != record['processStartTime']:
        raise RuntimeError('private process identity changed')
    command = process_field(pid, 'command')
    if not command or not command.startswith(str(binary) + ' app-server'):
        raise RuntimeError('private process executable/role changed')
    return pid


def stopped(record):
    value = process_field(record['pid'], 'stat')
    return value is None or value.startswith('Z')


def terminate_owned(record, binary):
    if stopped(record):
        return
    pid = verify_owned(record, binary)
    os.kill(pid, signal.SIGTERM)
    for _ in range(200):
        if stopped(record):
            return
        time.sleep(.05)
    raise RuntimeError('private process did not stop; no broader signal sent')


def metadata_ok(row):
    return row['info']['bytes'] == 176 and row['path_info']['bytes'] == 1200


def complete_snapshot(module, lib, pids, result):
    for _ in range(5):
        observation = module['snapshot'](lib, pids)
        result.setdefault('snapshot_attempts', []).append(observation)
        rows = [r for s in observation['snapshots'] for r in s['vnodes']]
        # Retain failed snapshots. Never retry away the actual ENOENT defect.
        if any(r['info']['errno'] == errno.ENOENT or r['path_info']['errno'] == errno.ENOENT
               for r in rows):
            raise AssertionError('unexpected ENOENT in private clear-state control')
        if all(s['enumeration_unchanged'] for s in observation['snapshots']) and all(
                metadata_ok(r) for r in rows):
            return observation
        time.sleep(.05)
    raise AssertionError('no complete private snapshot; attempts retained')


def revoked(row):
    return (row['listed_kind'] == 1 and row['info'] == dict(bytes=0, errno=errno.ENOENT)
            and row['path_info'] == dict(bytes=0, errno=errno.ENOENT)
            and row['lsof']['status'] == 0 and not row['lsof']['stderr_present']
            and len(row['lsof']['rows']) == 1
            and row['lsof']['rows'][0].get('marker') == '(revoked)'
            and 'type' not in row['lsof']['rows'][0])


def check_arm(arm):
    identity = arm['identity']
    assert arm['mode'] in CASES
    leak = arm['mode'] in FAULT_CASES and not arm.get('expect_contained', False)
    for row in arm['before'].values():
        if leak and arm['mode'] != 'inherit':
            assert revoked(row)
        elif leak:
            assert metadata_ok(row)
            assert (row['info']['device'], row['info']['inode']) == tuple(identity)
            assert row['info']['file_type'] == stat.S_IFDIR
            assert not row['info']['fd_flags'] & 2, 'inherited fd unexpectedly close-on-exec'
        else:
            assert not metadata_ok(row) or (
                row['info']['device'], row['info']['inode']) != tuple(identity)
    for row in arm['after'].values():
        if leak:
            assert revoked(row)
        else:
            assert not revoked(row)
    if arm['mode'] in FAULT_CASES:
        assert arm['restart_retained_updater']
        assert revoked(arm['updater_after_restart']) == leak
        assert arm['replacement_updater'] != arm['original_updater']
        assert arm['recovery_metadata_complete']


def cli(binary, home, args, environment, receipt, descriptor=None, cloexec=False):
    command = [str(binary), 'app-server', 'daemon', *args]
    if cloexec:
        # Calibrates the launcher-entry repair on the same supplied descriptor.
        command = [sys.executable, '-I', '-B', '-c',
                   'import os,sys; os.set_inheritable(int(sys.argv[1]),False); '
                   'os.execv(sys.argv[2],sys.argv[2:])', str(descriptor), *command]
    env = dict(environment, CODEX_HOME=str(home))
    # A caller's separate SQLite override must not escape the private Codex home.
    env.pop('CODEX_SQLITE_HOME', None)
    with subprocess.Popen(command, cwd=home, env=env, close_fds=True,
                          pass_fds=() if descriptor is None else (descriptor,),
                          start_new_session=True, stdout=subprocess.PIPE,
                          stderr=subprocess.PIPE, text=True) as child:
        try:
            stdout, stderr = child.communicate(timeout=30)
        except subprocess.TimeoutExpired:
            os.killpg(child.pid, signal.SIGTERM)
            child.communicate(timeout=5)
            raise RuntimeError('private lifecycle CLI timed out') from None
    receipt.setdefault('cli', []).append(dict(arguments=args, status=child.returncode,
                                              stderr_sha256=hashlib.sha256(stderr.encode()).hexdigest()))
    if child.returncode:
        raise RuntimeError('private CLI failed: ' + stderr[-2000:])
    return json.loads(stdout)


def arm(mode, work, binary, module, profile, environment, result):
    home, watched = work / mode, work / (mode + '-watched')
    packages, state = home / 'packages', home / 'app-server-daemon'
    config, settings = home / 'config.toml', state / 'settings.json'
    plan = dict(sources=(Path(__file__), PROBE, PROFILE),
                directories=(home, watched, packages, state), output_files=(config, settings))
    profile['prepare_producer'](ROOT, **plan)
    config.write_text('[analytics]\nenabled=false\n[history]\npersistence="none"\n')
    settings.write_text(json.dumps(dict(shutdownGraceSeconds=1, updater=dict(
        autoUpdateEnabled=True, updateIntervalMinutes=1440))))
    # Both link ends are inside this newly owned protected fixture, never host state.
    (packages / 'app-server-daemon').symlink_to(work / 'package', target_is_directory=True)
    master = None
    if mode == 'revoked-pty':
        master, fd = os.openpty()
    else:
        fd = os.open(watched, os.O_EVTONLY)
    descriptor = fcntl.fcntl(fd, fcntl.F_DUPFD_CLOEXEC, 64)
    os.close(fd)
    os.set_inheritable(descriptor, True)
    value = os.fstat(descriptor)
    result.update(mode=mode, descriptor=descriptor, identity=[value.st_dev, value.st_ino])
    records = []
    lib = module['library']()
    try:
        if mode == 'removed-directory':
            profile['plan_producer'](ROOT, directories=(watched,))
            watched.rmdir()
        elif mode == 'revoked-pty':
            # Only this freshly allocated, unattached fixture terminal is revoked.
            libc = ctypes.CDLL(None, use_errno=True)
            libc.revoke.argtypes = [ctypes.c_char_p]
            libc.revoke.restype = ctypes.c_int
            assert libc.revoke(os.fsencode(os.ttyname(descriptor))) == 0
        cli(binary, home, ['start'], environment, result,
            None if mode == 'closed' else descriptor, mode == 'cloexec')
        server, updater = read_record(home, 'daemon'), read_record(home, 'daemon-updater')
        records.extend((server, updater))
        wait_updater_ready(home)
        for record in records:
            verify_owned(record, binary)
        result['original_updater'] = updater['pid']
        result['before'] = {name: module['observe'](lib, r['pid'], descriptor)
                            for name, r in [('server', server), ('updater', updater)]}
        assert os.get_inheritable(descriptor), 'launch changed parent descriptor flags'
        if mode in ('removed-directory', 'revoked-pty'):
            expected_error = errno.ENOENT if mode == 'removed-directory' else errno.EBADF
            assert module['self_stat'](descriptor)['errno'] == expected_error
        else:
            assert os.fstat(descriptor).st_ino == value.st_ino
        os.close(descriptor)
        descriptor = None
        profile['plan_producer'](ROOT, sources=(Path(__file__),), directories=(watched,))
        if watched.exists():
            watched.rmdir()
        result['after'] = {name: module['observe'](lib, r['pid'], result['descriptor'])
                           for name, r in [('server', server), ('updater', updater)]}
        if mode in FAULT_CASES:
            cli(binary, home, ['restart'], environment, result)
            server = read_record(home, 'daemon')
            records.append(server)
            result['restart_retained_updater'] = read_record(home, 'daemon-updater') == updater
            result['updater_after_restart'] = module['observe'](lib, updater['pid'], result['descriptor'])
            terminate_owned(updater, binary)
            cli(binary, home, ['start'], environment, result)
            replacement = read_record(home, 'daemon-updater')
            records.append(replacement)
            wait_updater_ready(home)
            result['replacement_updater'] = replacement['pid']
            observation = complete_snapshot(module, lib, [server['pid'], replacement['pid']], result)
            result['recovery_metadata_complete'] = all(metadata_ok(row) for s in
                observation['snapshots'] for row in s['vnodes'])
        else:
            observation = complete_snapshot(module, lib, [server['pid'], updater['pid']], result)
            assert all(metadata_ok(row) for s in observation['snapshots'] for row in s['vnodes'])
        check_arm(result)
    finally:
        if descriptor is not None:
            os.close(descriptor)
        if master is not None:
            os.close(master)
        # Discover only this fixture's published processes, including partially failed start.
        for name in ('daemon', 'daemon-updater'):
            path = state / (name + '.pid')
            if path.is_file():
                record = json.loads(path.read_text())
                if record not in records:
                    records.append(record)
        for record in reversed(records):
            terminate_owned(record, binary)
        result['all_owned_stopped'] = all(stopped(r) for r in records)
        assert result['all_owned_stopped']
    return result


def run(executable, expect_contained=False, cases=CASES):
    if sys.platform != 'darwin' or not executable.is_absolute() or not executable.is_file():
        raise RuntimeError('requires Darwin and an absolute installed binary')
    for path in (ROOT / 'scripts', PROFILE, PROBE):
        if path.is_symlink() or path.stat().st_dev != ROOT.stat().st_dev:
            raise RuntimeError('unsafe diagnostic bootstrap')
    profile = runpy.run_path(str(PROFILE))
    module = runpy.run_path(str(PROBE))
    work = ROOT / 'target/artifact_cleanup_audit' / ('d217-daemon-' + str(os.getpid()))
    if work.exists() or work.is_symlink():
        raise RuntimeError('private fixture must be absent')
    release = work / 'package/releases' / VERSION
    binary, marker = release / 'bin/codex', work / 'package/auto-update-version'
    receipt = work / 'receipt.json'
    plan = dict(sources=(Path(__file__), PROBE, PROFILE),
                directories=(work, release / 'bin'), output_files=(binary, marker, receipt))
    environment = profile['enter_producer'](ROOT, **plan)
    expected = digest(executable)
    with executable.open('rb') as source, binary.open('xb') as destination:
        shutil.copyfileobj(source, destination, 1024 * 1024)
    binary.chmod(0o755)
    assert digest(binary) == expected
    marker.write_text(VERSION)
    (work / 'package/current').symlink_to(release, target_is_directory=True)
    result = dict(binary_sha256=expected, arms=[], complete=False)
    try:
        for mode in cases:
            row = dict(expect_contained=expect_contained)
            result['arms'].append(row)
            arm(mode, work, binary, module, profile, environment, row)
        result['complete'] = True
    finally:
        profile['plan_producer'](ROOT, sources=(Path(__file__),), output_files=(receipt,))
        with receipt.open('x') as stream:
            json.dump(result, stream, sort_keys=True, indent=2)
            stream.write('\n')
        if result['arms'] and all(a.get('all_owned_stopped') for a in result['arms']):
            profile['plan_producer'](ROOT, output_files=(binary,))
            assert binary.is_file() and binary.stat().st_nlink == 1 and digest(binary) == expected
            binary.unlink()
    print(json.dumps(dict(receipt=receipt.relative_to(ROOT).as_posix(), complete=result['complete'])))


def self_test():
    live = dict(listed_kind=1, info=dict(bytes=176, device=1, inode=2,
                file_type=stat.S_IFDIR, fd_flags=1), path_info=dict(bytes=1200))
    dead = dict(listed_kind=1, info=dict(bytes=0, errno=2), path_info=dict(bytes=0, errno=2),
                lsof=dict(status=0, stderr_present=False, rows=[dict(marker='(revoked)')]))
    arm = dict(mode='inherit', identity=[1, 2], before=dict(server=live, updater=live),
               after=dict(server=dead, updater=dead), restart_retained_updater=True,
               updater_after_restart=dead, original_updater=10, replacement_updater=11,
               recovery_metadata_complete=True)
    check_arm(arm)
    mutations = [lambda a: a['identity'].__setitem__(1, 3),
                 lambda a: a['before']['server']['info'].__setitem__('fd_flags', 2),
                 lambda a: a['after']['server']['info'].__setitem__('errno', 9),
                 lambda a: a.__setitem__('restart_retained_updater', False),
                 lambda a: a.__setitem__('replacement_updater', 10),
                 lambda a: a.__setitem__('recovery_metadata_complete', False),
                 lambda a: a.__setitem__('expect_contained', True)]
    for mutation in mutations:
        changed = copy.deepcopy(arm)
        mutation(changed)
        try:
            check_arm(changed)
        except AssertionError:
            continue
        raise AssertionError('causal oracle accepted a false observation')
    for mode in ('removed-directory', 'revoked-pty'):
        changed = copy.deepcopy(arm)
        changed.update(mode=mode, before=copy.deepcopy(arm['after']))
        check_arm(changed)
        changed['before']['server']['info']['errno'] = 9
        try:
            check_arm(changed)
        except AssertionError:
            continue
        raise AssertionError('already invalid resource oracle accepted a false observation')
    print(json.dumps(dict(independent_refusals=len(mutations) + 2)))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', type=Path)
    parser.add_argument('--expect-contained', action='store_true')
    parser.add_argument('--self-test', action='store_true')
    parser.add_argument('--case', choices=CASES, action='append')
    args = parser.parse_args()
    if args.self_test and (args.binary or args.expect_contained or args.case) or not args.self_test and not args.binary:
        parser.error('choose --self-test or --binary [--expect-contained]')
    if args.self_test:
        self_test()
    else:
        run(args.binary, args.expect_contained, args.case or CASES)
