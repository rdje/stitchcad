"""Project handoff census: unavailable evidence refuses; idle CUA needs attestation."""
import argparse
from dataclasses import dataclass
import os
from pathlib import Path
import re
import subprocess
import sys


class Refusal(Exception):
    """A complete process census could not be established."""


@dataclass(frozen=True)
class Process:
    pid: int
    parent: int
    elapsed: str
    command: str


def command_output(command, run):
    result = run(command, capture_output=True, text=True)
    if result.returncode != 0:
        raise Refusal(command[0] + ' unavailable (exit ' + str(result.returncode) + ')')
    if not result.stdout.strip():
        raise Refusal(command[0] + ' returned empty evidence')
    return result.stdout


def parse_processes(text):
    processes = {}
    for line in text.splitlines():
        fields = line.strip().split(None, 3)
        if (len(fields) != 4 or not fields[0].isdigit() or not fields[1].isdigit()
                or not re.fullmatch(r'[\d:-]+', fields[2]) or not fields[3]):
            raise Refusal('malformed ps evidence')
        pid, parent = map(int, fields[:2])
        if pid <= 0 or pid in processes:
            raise Refusal('duplicate/invalid ps identity')
        processes[pid] = Process(pid, parent, fields[2], fields[3])
    return processes


def parse_handles(text, root):
    """Read lsof's process/file/name fields without guessing from command names."""
    handles, cwd, seen = {}, set(), set()
    pid = fd = kind = None
    named = False
    complete = set()
    for line in text.splitlines():
        if not line:
            raise Refusal('malformed lsof evidence')
        field, value = line[0], line[1:]
        if field in ('p', 'f') and fd is not None and not named:
            raise Refusal('truncated lsof file evidence')
        if field == 'p':
            if not value.isdigit() or int(value) <= 0 or int(value) in seen:
                raise Refusal('duplicate/invalid lsof identity')
            pid, fd, kind = int(value), None, None
            seen.add(pid)
        elif field == 'f':
            if pid is None or not value:
                raise Refusal('malformed lsof file field')
            fd, kind, named = value, None, False
        elif field == 't':
            if pid is None or fd is None or kind is not None or not value:
                raise Refusal('malformed lsof type field')
            kind = value
        elif field == 'n':
            if pid is None or fd is None or kind is None or named:
                raise Refusal('malformed lsof name field')
            if not value and kind not in ('PIPE', 'NPOLICY', 'NEXUS'):
                raise Refusal('unnamed filesystem/unknown lsof handle')
            named = True
            complete.add(pid)
            if value == root or value.startswith(root + os.sep):
                if fd == 'cwd':
                    cwd.add(pid)
                else:
                    handles[pid] = handles.get(pid, 0) + 1
        else:
            raise Refusal('unexpected lsof field')
    if not named or complete != seen:
        raise Refusal('truncated lsof process evidence')
    return handles, cwd, seen


def caller_ancestry(processes, caller):
    excluded = set()
    current = caller
    while current > 1:
        if current in excluded or current not in processes:
            raise Refusal('incomplete/cyclic caller ancestry')
        excluded.add(current)
        current = processes[current].parent
    if caller not in excluded:
        raise Refusal('caller missing from ps evidence')
    return excluded


def cua_launch(command, root):
    """Only the observed kernel/worker argument shapes, never a process-name exemption."""
    prefix = r'\S+/cua_node/bin/node --experimental-vm-modules \S+/'
    kernel = prefix + r'kernel\.js --session-id [A-Za-z0-9-]+ --working-dir ' + re.escape(root)
    worker = prefix + r'trusted-worker\.js ' + re.escape(root)
    return bool(re.fullmatch(kernel, command) or re.fullmatch(worker, command))


def idle_cua_pairs(processes, handles, root):
    idle = set()
    for child in processes.values():
        if not cua_launch(child.command, root):
            continue
        parent = processes.get(child.parent)
        if parent is None:
            continue
        launcher, separator, tail = parent.command.partition(' -- ')
        shape = bool(re.match(r'^\S+/codex-cli/CodexCLI\.app/Contents/MacOS/codex sandbox ', launcher))
        if (shape and separator and tail == child.command
                and 'default_permissions="node_repl"' in launcher
                and 'permissions.node_repl=' in launcher
                and not handles.get(child.pid, 0) and not handles.get(parent.pid, 0)):
            idle.update((child.pid, parent.pid))
    return idle


def census(root, caller, uid, idle_cua=False, run=subprocess.run):
    processes = parse_processes(command_output(['ps', '-Ao', 'pid=,ppid=,etime=,command='], run))
    excluded = caller_ancestry(processes, caller)
    handles, cwd, seen = parse_handles(command_output(['lsof', '-u', str(uid), '-Fpftn'], run), root)
    if caller not in seen:
        raise Refusal('caller missing from lsof evidence')
    idle = idle_cua_pairs(processes, handles, root) if idle_cua else set()
    blocking, advisory = [], []
    for pid in sorted(set(processes) | set(handles) | cwd):
        if pid in excluded:
            continue
        process = processes.get(pid)
        names_root = process is not None and root in process.command
        count = handles.get(pid, 0)
        if count > 0 or (names_root and pid not in idle):
            blocking.append((pid, count, 'repo handle' if count else 'checkout argument'))
        elif pid in cwd or pid in idle:
            advisory.append((pid, 'attested idle CUA metadata' if pid in idle else 'inherited cwd'))
    return blocking, advisory


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--all', action='store_true', help='list advisory PIDs')
    parser.add_argument('--idle-cua', action='store_true',
                        help='attest that no CUA call/result remains in flight; real handles still block')
    args = parser.parse_args()
    root = str(Path(__file__).resolve().parent.parent)
    try:
        blocking, advisory = census(root, os.getpid(), os.getuid(), args.idle_cua)
    except (Refusal, OSError, UnicodeError) as error:
        print('handoff: REFUSED — ' + str(error), file=sys.stderr)
        return 2
    if blocking:
        print('handoff: BLOCKED — %d project process(es) still running' % len(blocking))
        for pid, count, reason in blocking:
            print('  PID %d: %s (%d repo file handles)' % (pid, reason, count))
        return 1
    print('handoff: OK — no project-owned background job is running')
    if args.all:
        for pid, reason in advisory:
            print('  advisory PID %d: %s' % (pid, reason))
    else:
        print('  advisory: %d inherited-cwd/attested-idle process(es)' % len(advisory))
    return 0


if __name__ == '__main__':
    sys.exit(main())
