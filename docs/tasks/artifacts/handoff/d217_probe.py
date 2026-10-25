"""D217: compare Darwin vnode APIs with lsof; reproduce only on owned descriptors.

snapshot --pid PID [...] is read-only and withholds file paths/command arguments.
controls creates one guarded, unique audit fixture and an unattached owned PTY;
it never closes, revokes, signals or modifies an existing process's descriptors.
Run with OS visibility; an unavailable observation is reported, never made clear.
"""
import argparse
import ast
import ctypes as C
import ctypes.util
import errno
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import runpy
import select
import signal
import stat
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[4]
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'


class FD(C.Structure):
    _fields_ = [('number', C.c_int32), ('kind', C.c_uint32)]


def library():
    if sys.platform != 'darwin':
        raise RuntimeError('this diagnostic requires Darwin libproc')
    lib = C.CDLL(ctypes.util.find_library('proc'), use_errno=True)
    lib.proc_pidinfo.argtypes = [C.c_int, C.c_int, C.c_uint64, C.c_void_p, C.c_int]
    lib.proc_pidfdinfo.argtypes = [C.c_int, C.c_int, C.c_int, C.c_void_p, C.c_int]
    lib.proc_pidinfo.restype = lib.proc_pidfdinfo.restype = C.c_int
    return lib


def fd_list(lib, pid):
    entries = (FD * 16384)()
    C.set_errno(0)
    size = lib.proc_pidinfo(pid, 1, 0, entries, C.sizeof(entries))
    error = C.get_errno()
    if size <= 0 or size >= C.sizeof(entries) or size % C.sizeof(FD):
        raise RuntimeError('incomplete fd list: PID %d bytes %d errno %d' % (pid, size, error))
    return {row.number: row.kind for row in entries[:size // C.sizeof(FD)]}


def vnode(lib, pid, fd, flavor):
    # SDK proc_info.h: vnode_fdinfo=176; with MAXPATHLEN path=1200.
    # Refuse any different ABI rather than decoding a partial/unexpected buffer.
    expected = {1: 176, 2: 1200}[flavor]
    buf = C.create_string_buffer(expected)
    C.set_errno(0)
    count = lib.proc_pidfdinfo(pid, fd, flavor, buf, len(buf))
    row = dict(bytes=count, errno=C.get_errno())
    if count == expected:
        mode = int.from_bytes(buf.raw[28:30], sys.byteorder)
        row.update(mode=oct(mode), file_type=stat.S_IFMT(mode),
                   open_flags=int.from_bytes(buf.raw[:4], sys.byteorder),
                   fd_flags=int.from_bytes(buf.raw[4:8], sys.byteorder),
                   device=int.from_bytes(buf.raw[24:28], sys.byteorder),
                   inode=int.from_bytes(buf.raw[32:40], sys.byteorder))
        if flavor == 2:
            path = buf.raw[176:].split(b'\0', 1)[0]
            root = os.fsencode(ROOT)
            row.update(path_present=bool(path), path_sha256=hashlib.sha256(path).hexdigest(),
                       project=path == root or path.startswith(root + b'/'))
    elif count > 0:
        raise RuntimeError('unexpected vnode ABI: %d / %d' % (count, expected))
    return row


def lsof(pid, fd=None):
    command = ['lsof', '-a', '-p', str(pid), '-Fpftn']
    if fd is not None:
        command += ['-d', str(fd)]
    result = subprocess.run(command, capture_output=True, text=True, timeout=20)
    rows, row = [], None
    for line in result.stdout.splitlines():
        field, value = line[:1], line[1:]
        if field == 'f':
            row = dict(fd=value)
            rows.append(row)
        elif row is not None and field == 't':
            row['type'] = value
        elif row is not None and field == 'n':
            row['marker'] = value if value in ('(revoked)', 'no more information') else None
            row['name_sha256'] = hashlib.sha256(value.encode()).hexdigest()
    return dict(status=result.returncode, stderr_present=bool(result.stderr), rows=rows)


def self_stat(fd):
    try:
        value = os.fstat(fd)
        return dict(errno=0, mode=oct(value.st_mode), device=value.st_dev,
                    inode=value.st_ino, links=value.st_nlink)
    except OSError as error:
        return dict(errno=error.errno)


def observe(lib, pid, fd):
    return dict(pid=pid, fd=fd, listed_kind=fd_list(lib, pid).get(fd),
                info=vnode(lib, pid, fd, 1), path_info=vnode(lib, pid, fd, 2),
                lsof=lsof(pid, fd))


def snapshot(lib, pids):
    snapshots = []
    for pid in pids:
        before = fd_list(lib, pid)
        rows = [observe(lib, pid, fd) for fd, kind in before.items() if kind == 1]
        after = fd_list(lib, pid)
        snapshots.append(dict(pid=pid, enumeration_unchanged=before == after, vnodes=rows))
    return dict(kernel=platform.release(), kernel_build=platform.version(),
                lsof_version=subprocess.run(['lsof', '-v'], capture_output=True, text=True).stderr,
                snapshots=snapshots)


def controls(lib, profile):
    work = ROOT / 'target/artifact_cleanup_audit' / ('d217-controls-' + str(os.getpid()))
    ordinary, directory = work / 'ordinary.dat', work / 'removed-directory'
    event_file, event_dir = work / 'watched.dat', work / 'watched-directory'
    if work.exists() or work.is_symlink():
        raise RuntimeError('controlled fixture must be absent')
    plan = dict(sources=(Path(__file__), PROFILE_SOURCE), directories=(work, directory, event_dir),
                output_files=(ordinary, event_file))
    profile['enter_producer'](ROOT, **plan)
    descriptors, rows = [], []
    def capture(label, fd):
        row = observe(lib, os.getpid(), fd)
        row.update(label=label, fstat=self_stat(fd))
        rows.append(row)
    try:
        profile['plan_producer'](ROOT, **plan)
        fd = os.open(ordinary, os.O_CREAT | os.O_EXCL | os.O_RDWR, 0o600)
        descriptors.append(fd)
        capture('live regular file', fd)
        profile['plan_producer'](ROOT, **plan)
        ordinary.unlink()
        capture('unlinked regular file', fd)
        fd = os.open(directory, os.O_RDONLY)
        descriptors.append(fd)
        profile['plan_producer'](ROOT, **plan)
        directory.rmdir()
        capture('removed open directory', fd)
        profile['plan_producer'](ROOT, **plan)
        with event_file.open('xb'):
            pass
        fd = os.open(event_file, os.O_EVTONLY)
        descriptors.append(fd)
        capture('live event-only regular file', fd)
        profile['plan_producer'](ROOT, **plan)
        event_file.unlink()
        capture('unlinked event-only regular file', fd)
        fd = os.open(event_dir, os.O_EVTONLY)
        descriptors.append(fd)
        profile['plan_producer'](ROOT, **plan)
        event_dir.rmdir()
        capture('removed event-only directory', fd)
        master, slave = os.openpty()
        descriptors.extend((master, slave))
        capture('live unattached PTY slave', slave)
        # revoke only the freshly allocated, unattached slave owned by this
        # fixture. No terminal used by the harness or another process is touched.
        terminal = os.ttyname(slave)
        libc = C.CDLL(None, use_errno=True)
        libc.revoke.argtypes = [C.c_char_p]
        libc.revoke.restype = C.c_int
        C.set_errno(0)
        result = libc.revoke(os.fsencode(terminal))
        if result != 0:
            raise OSError(C.get_errno(), 'owned PTY revoke failed')
        capture('explicitly revoked owned PTY slave', slave)
        os.close(master)
        descriptors.remove(master)
        capture('PTY slave after owned master closed', slave)
        os.close(slave)
        descriptors.remove(slave)
        # fd_list still contains the other controlled file/directory, so a closed
        # descriptor can be compared without accepting an empty enumeration.
        capture('closed owned slave descriptor', slave)
        verify_controls(rows)
        return rows
    finally:
        for fd in reversed(descriptors):
            os.close(fd)
        profile['plan_producer'](ROOT, **plan)
        for path in (ordinary, event_file):
            if path.exists():
                path.unlink()
        for path in (directory, event_dir):
            if path.exists():
                path.rmdir()
        work.rmdir()


def verify_controls(rows):
    """Independently specified observations; the marker alone never proves a cause."""
    expected = [
        ('live regular file', 0, 0, 'REG'),
        ('unlinked regular file', 0, 0, 'REG'),
        ('removed open directory', 0, 0, 'DIR'),
        ('live event-only regular file', 0, 0, 'REG'),
        ('unlinked event-only regular file', 0, 0, 'REG'),
        ('removed event-only directory', errno.ENOENT, errno.ENOENT, None),
        ('live unattached PTY slave', 0, 0, 'CHR'),
        ('explicitly revoked owned PTY slave', errno.ENOENT, errno.EBADF, None),
        ('PTY slave after owned master closed', errno.ENOENT, errno.EBADF, None),
        ('closed owned slave descriptor', errno.EBADF, errno.EBADF, None),
    ]
    assert [row['label'] for row in rows] == [row[0] for row in expected], 'lifecycle coverage'
    for observed, (label, error, local_error, kind) in zip(rows, expected):
        assert observed['info']['errno'] == observed['path_info']['errno'] == error, label
        assert observed['fstat']['errno'] == local_error, label
        if error == errno.EBADF:
            assert observed['listed_kind'] is None and not observed['lsof']['rows'], label
        else:
            assert observed['listed_kind'] == 1 and observed['lsof']['status'] == 0, label
            assert len(observed['lsof']['rows']) == 1, label
            row = observed['lsof']['rows'][0]
            assert row.get('type') == kind, label
            assert row['marker'] == ('(revoked)' if error == errno.ENOENT else None), label


def self_test():
    """Pure observation controls run on every platform; real calibrations are explicit."""
    # An observation set assembled independently of the OS adapters must refuse
    # changed errors, a missing lifecycle or a falsely typed revoked descriptor.
    specifications = [('REG', 0, 0), ('REG', 0, 0), ('DIR', 0, 0),
                      ('REG', 0, 0), ('REG', 0, 0), (None, 2, 2),
                      ('CHR', 0, 0), (None, 2, 9), (None, 2, 9), (None, 9, 9)]
    labels = ['live regular file', 'unlinked regular file', 'removed open directory',
              'live event-only regular file', 'unlinked event-only regular file',
              'removed event-only directory', 'live unattached PTY slave',
              'explicitly revoked owned PTY slave', 'PTY slave after owned master closed',
              'closed owned slave descriptor']
    rows = [dict(label=label, info=dict(errno=error), path_info=dict(errno=error),
                 fstat=dict(errno=local_error), listed_kind=None if error == 9 else 1,
                 lsof=dict(status=0, rows=[] if error == 9 else [
                     dict(type=kind, marker='(revoked)' if error == 2 else None)]))
            for label, (kind, error, local_error) in zip(labels, specifications)]
    verify_controls(rows)
    failures = []
    missing = json.loads(json.dumps(rows))[:-1]
    failures.append(missing)
    for index, field, value in [(5, 'info', 0), (5, 'path_info', 13), (7, 'fstat', 2)]:
        altered = json.loads(json.dumps(rows))
        altered[index][field]['errno'] = value
        failures.append(altered)
    altered = json.loads(json.dumps(rows))
    altered[5]['lsof']['rows'][0]['type'] = 'DIR'
    failures.append(altered)
    for altered in failures:
        try:
            verify_controls(altered)
        except AssertionError:
            pass
        else:
            raise AssertionError('D217 diagnostic observation control escaped')
    return dict(lifecycles=len(rows), independent_refusals=len(failures))


def calibrate(lib, profile):
    original = Path(__file__).read_bytes()
    rows = controls(lib, profile)
    node = next(node for node in ast.parse(original).body
                if isinstance(node, ast.FunctionDef) and node.name == 'controls')
    body = ''.join(original.decode().splitlines(keepends=True)[node.lineno - 1:node.end_lineno])
    faults = [
        ('event-only directory', 'fd = os.open(event_dir, os.O_EVTONLY)',
         'fd = os.open(event_dir, os.O_RDONLY)'),
        ('terminal revocation', 'result = libc.revoke(os.fsencode(terminal))', 'result = 0'),
    ]
    for name, old, new in faults:
        assert body.count(old) == 1, (name, 'actual fault anchor')
        scope = dict(__file__=__file__, __name__='d217_fault')
        altered = original.decode().replace(body, body.replace(old, new), 1)
        exec(compile(altered, __file__, 'exec'), scope)
        try:
            scope['controls'](lib, profile)
        except AssertionError:
            pass
        else:
            raise AssertionError('D217 actual lifecycle fault escaped: ' + name)
    assert Path(__file__).read_bytes() == original, 'D217 producer changed'
    return dict(lifecycles=len(rows), actual_assertion_reds=len(faults), producer_unchanged=True)


def retained_plan(profile):
    """Validate every physical backing input; a logical record need not be a raw file."""
    authority = Path(os.environ.get('HISTORY_ARCHIVE_ROOT') or str(ROOT))
    if not authority.is_absolute():
        authority = ROOT / authority
    profile['producer_path'](ROOT, authority)
    profile['workspace'](authority)
    sources = [Path(__file__), PROFILE_SOURCE, ROOT / 'scripts/history_archive.sh',
               ROOT / 'scripts/history_archive.py', authority / '.doctrine/history_archive/windows.json']
    folders = [authority / '.doctrine/history_archive', authority / 'docs/history']
    for folder in folders:
        profile['directory'](ROOT, profile['producer_path'](ROOT, folder))
        for path in sorted(folder.iterdir()):
            profile['require'](not path.is_symlink(), 'symlink retained archive input')
            if path.is_dir():
                folders.append(path)
            elif path not in sources:
                sources.append(path)
    profile['plan_producer'](ROOT, sources=tuple(sources))
    environment = profile['environment_for'](ROOT, dict(os.environ), create=False)
    environment = {name: value for name, value in environment.items() if not name.startswith('GIT_')}
    environment.update(HISTORY_ARCHIVE_ROOT=str(authority), GIT_CONFIG_NOSYSTEM='1',
                       GIT_CONFIG_GLOBAL=os.devnull, GIT_OPTIONAL_LOCKS='0',
                       GIT_LITERAL_PATHSPECS='1', GIT_TERMINAL_PROMPT='0')
    return tuple(sources), environment


def retained(profile):
    logical = []
    for record in ('docs/history/stitchcad-devnotes-part303.md',
                   'docs/history/stitchcad-devnotes-part304.md',
                   'docs/history/stitchcad-changelog-part208.md'):
        environment = retained_plan(profile)[1]
        logical.append(subprocess.check_output(['bash', 'scripts/history_archive.sh', 'read', record],
                                               cwd=ROOT, env=environment))
    tree = 'docs/tasks/G1-SLICE-producers.md'
    environment = retained_plan(profile)[1]
    original = subprocess.check_output(['git', 'show', 'cdc6a77:' + tree], cwd=ROOT, env=environment)
    leaves = ('G1-SLICE.5b.4c.h2.b.p.o.f.cleanup4.fix', 'G1-SLICE.5b.4c.h2.b.p.o.a01.2',
              'G1-SLICE.5b.4c.h2.b.p.v.b1', 'G1-SLICE.5b.4c.h2.b.p.o.f.i',
              'G1-SLICE.5b.4c.h2.b.p.o.a23.3.1')
    records = []
    for leaf in leaves:
        start = original.index(('- ID: `' + leaf + '`').encode())
        end = original.find(b'\n- ID: ', start)
        records.append(original[start:end if end >= 0 else len(original)].rstrip(b'\n') + b'\n')
    body = logical[0].split(b'\n---\n\n', 1)[1]
    assert body == b'\n'.join(records), 'whole committed task records differ'
    environment = retained_plan(profile)[1]
    notes = subprocess.check_output(['git', 'show', 'ccc2a66:DEV_NOTES.md'], cwd=ROOT, env=environment)
    first = notes.index('## _(2026-10-25 UTC)_ — D217 needs causal discrimination'.encode())
    last = notes.index('## _(2026-10-24 UTC)_ — transport admission'.encode(), first)
    environment = retained_plan(profile)[1]
    old_tree = subprocess.check_output(['git', 'show', 'ccc2a66:' + tree], cwd=ROOT, env=environment)
    start = old_tree.index(b'- ID: `G1-SLICE.5b.4c.h2.b.p.o.f.cleanup4.hunt`')
    end = old_tree.index(b'\n- ID: ', start)
    expected = notes[first:last].rstrip(b'\n') + b'\n\n' + old_tree[start:end].rstrip(b'\n') + b'\n'
    retained_followup = logical[1].split(b'\n---\n\n', 1)[1]
    assert retained_followup == expected, 'whole prior diagnostic records differ'
    environment = retained_plan(profile)[1]
    changelog = subprocess.check_output(['git', 'show', 'ccc2a66:CHANGELOG.md'], cwd=ROOT, env=environment)
    whole_ledgers = []
    for unit in ('0234v', '0234', '0233', '0232', '0231', '0230', '0229', '0228'):
        start = re.search(rb'^## STITCHCAD-G1-' + unit.encode() + ' —'.encode(), changelog, re.M)
        end = re.search(rb'^(?:## |\|)', changelog[start.end():], re.M)
        whole_ledgers.append(changelog[start.start():start.end() + end.start()].rstrip(b'\n') + b'\n')
    retained_ledgers = logical[2].split(b'\n---\n\n', 1)[1]
    assert retained_ledgers == b'\n'.join(whole_ledgers), 'whole prior continuity ledgers differ'
    environment = retained_plan(profile)[1]
    subprocess.run(['bash', 'scripts/history_archive.sh', 'prove-source', 'window11'],
                   cwd=ROOT, env=environment, capture_output=True, check=True)
    return dict(complete_source_intervals=len(records), bytes=len(body), exact=True,
                retained_window_source_exact=True, followup_bytes=len(retained_followup),
                whole_ledger_bytes=len(retained_ledgers))


def runtime(lib, profile, executable, inherit):
    """Initialize an isolated installed app-server; send no thread or inference request."""
    if not executable.is_absolute() or not executable.is_file() or executable.name != 'codex':
        raise RuntimeError('runtime requires an absolute installed codex executable')
    work = ROOT / 'target/artifact_cleanup_audit' / ('d217-runtime-' + str(os.getpid()))
    state = work / 'codex-state'
    watched = work / 'inherited-directory'
    if work.exists() or work.is_symlink():
        raise RuntimeError('runtime fixture must be absent')
    plan = dict(sources=(Path(__file__), PROFILE_SOURCE), directories=(work, state, watched))
    environment = profile['enter_producer'](ROOT, **plan)
    # CODEX_HOME is used for its documented purpose, only in this child's
    # environment; the host binding and actual user's state are untouched.
    child_environment = dict(environment, CODEX_HOME=str(state))
    descriptor = os.open(watched, os.O_EVTONLY)
    if inherit == 'removed-event-directory':
        profile['plan_producer'](ROOT, **plan)
        watched.rmdir()
    inherited = () if inherit == 'none' else (descriptor,)
    child = None
    rows = {}
    try:
        child = subprocess.Popen([str(executable), 'app-server', '--listen', 'stdio://',
                              '-c', 'analytics.enabled=false',
                              '-c', 'history.persistence="none"'],
                             cwd=work, env=child_environment, stdin=subprocess.PIPE,
                             stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                             text=True, start_new_session=True, close_fds=True, pass_fds=inherited)
        response = None
        request = dict(method='initialize', id=1,
                       params=dict(clientInfo=dict(name='stitchcad_d217_probe', version='1')))
        child.stdin.write(json.dumps(request) + '\n')
        child.stdin.flush()
        if select.select([child.stdout], [], [], 10)[0]:
            line = child.stdout.readline()
            if line:
                response = json.loads(line)
        if response is None:
            raise RuntimeError('isolated runtime did not initialize within ten seconds')
        child.stdin.write(json.dumps(dict(method='initialized', params={})) + '\n')
        child.stdin.flush()
        time.sleep(0.5)
        rows = snapshot(lib, [child.pid])
        rows.update(initialize_ok='result' in response, state_directory=work.relative_to(ROOT).as_posix(),
                    inheritance=inherit, controlled_fd=descriptor,
                    controlled_observation=observe(lib, child.pid, descriptor) if inherited else None)
        return rows
    finally:
        os.close(descriptor)
        if child is not None:
            stop_runtime(child, rows)


def stop_runtime(child, rows):
    try:
        child.stdin.close()
    except BrokenPipeError:
        pass
    try:
        child.wait(timeout=10)
    except subprocess.TimeoutExpired:
        os.killpg(child.pid, signal.SIGTERM)
        try:
            child.wait(timeout=5)
        except subprocess.TimeoutExpired:
            os.killpg(child.pid, signal.SIGKILL)
            child.wait(timeout=5)
    # Only the newly owned, separate process group may be signalled.
    try:
        os.killpg(child.pid, signal.SIGTERM)
    except ProcessLookupError:
        pass
    child.stdout.close()
    child.stderr.close()
    rows['child_exit'] = child.returncode
    for attempt in range(40):
        try:
            os.killpg(child.pid, 0)
        except ProcessLookupError:
            rows['process_group_gone'] = True
            return
        if attempt == 20:
            os.killpg(child.pid, signal.SIGKILL)
        time.sleep(0.05)
    raise RuntimeError('isolated runtime descendants remain')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('mode', choices=('snapshot', 'controls', 'calibrate', 'self-test', 'retained', 'runtime'))
    parser.add_argument('--pid', type=int, nargs='+')
    parser.add_argument('--binary', type=Path)
    parser.add_argument('--inherit', choices=('none', 'live-event-directory', 'removed-event-directory'),
                        default='none')
    args = parser.parse_args()
    if (args.mode == 'snapshot') != bool(args.pid) or any(pid <= 0 for pid in args.pid or []):
        parser.error('snapshot requires positive --pid values; controls takes none')
    if (args.mode == 'runtime') != bool(args.binary):
        parser.error('runtime requires --binary; other modes take none')
    if args.mode != 'runtime' and args.inherit != 'none':
        parser.error('only runtime accepts --inherit')
    if args.mode == 'self-test':
        print(json.dumps(self_test(), sort_keys=True))
        return
    # Validate the bootstrap file/parent before loading the shared profile.
    for path in (ROOT / 'scripts', PROFILE_SOURCE):
        if path.is_symlink() or path.stat().st_dev != ROOT.stat().st_dev:
            raise RuntimeError('unsafe diagnostic bootstrap')
    if not PROFILE_SOURCE.is_file() or (ROOT / 'scripts/.git').exists():
        raise RuntimeError('unsafe diagnostic profile')
    profile = runpy.run_path(str(PROFILE_SOURCE))
    profile['plan_producer'](ROOT, sources=(Path(__file__), PROFILE_SOURCE))
    if args.mode == 'retained':
        print(json.dumps(retained(profile), sort_keys=True))
        return
    lib = library()
    if args.mode == 'snapshot':
        result = snapshot(lib, args.pid)
    elif args.mode == 'controls':
        result = controls(lib, profile)
    elif args.mode == 'calibrate':
        result = calibrate(lib, profile)
    else:
        result = runtime(lib, profile, args.binary, args.inherit)
    print(json.dumps(result, sort_keys=True, indent=2))


if __name__ == '__main__':
    main()
