"""Conservative, repository-local artifact cleanup with a frozen plan and safety refusals."""
from pathlib import Path
import hashlib
import json
import os
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
TREES = ('target/doctrine_scratch', 'target/scratch', 'target/tmp',
         'target/debug/incremental', 'target/wasm32-unknown-unknown/debug/incremental',
         'docs/book/book')
AUDIT = 'target/artifact_cleanup_audit'
PROTECTED = (AUDIT, 'target/cargo-home', 'target/scaffold_backup',
             'target/debug/deps', 'target/release/deps')
SUFFIXES = ('.log', '.bin', '.tmp', '.orig', '.rej')


class Refusal(Exception):
    """A selected path cannot be proved safe to remove."""


def require(condition, reason):
    if not condition:
        raise Refusal(reason)


def git(*args):
    return subprocess.check_output(['git', '-C', str(ROOT), *args])


def tracked():
    return {part.decode() for part in git('ls-files', '-z').split(b'\0') if part}


def under(path, prefix):
    return path == prefix or path.startswith(prefix + '/')


def local(path):
    require(not path.is_absolute() and '..' not in path.parts and bool(path.parts), 'path locality')
    full = ROOT / path
    for parent in [full, *full.parents]:
        if parent == ROOT:
            break
        require(not parent.is_symlink(), 'symlink ancestor: ' + str(path))
        if parent.is_dir():
            require(parent.stat().st_dev == ROOT.stat().st_dev, 'foreign device: ' + str(path))
            require(not (parent / '.git').exists(), 'nested repository: ' + str(path))
    return full


def inspect(name, tracked_paths):
    """Fingerprint every selected entry; reject links, Git boundaries and tracked descendants."""
    path = local(Path(name))
    require(not any(under(name, p) for p in PROTECTED), 'protected output/store: ' + name)
    require(not any(under(p, name) for p in tracked_paths), 'tracked descendant: ' + name)
    require(subprocess.run(['git', '-C', str(ROOT), 'check-ignore', '-q', '--', name]).returncode == 0,
            'not ignored: ' + name)
    digest = hashlib.sha256()
    files = size = 0
    entries = [path]
    while entries:
        entry = entries.pop()
        rel = entry.relative_to(ROOT).as_posix()
        require(not entry.is_symlink(), 'symlink entry: ' + rel)
        require(entry.stat().st_dev == ROOT.stat().st_dev, 'foreign device: ' + rel)
        require(entry.name != '.git', 'nested repository: ' + rel)
        require(entry.is_file() or entry.is_dir(), 'special entry: ' + rel)
        digest.update((rel + '\0').encode())
        if entry.is_dir():
            digest.update(b'directory\0')
            entries.extend(sorted(entry.iterdir(), reverse=True))
        else:
            digest.update(b'file\0')
            content = hashlib.sha256()
            file_size = 0
            with entry.open('rb') as stream:
                while True:
                    data = stream.read(1024 * 1024)
                    if not data:
                        break
                    size += len(data)
                    file_size += len(data)
                    content.update(data)
            digest.update(str(file_size).encode() + b'\0' + content.digest())
            files += 1
    return {'path': name, 'kind': 'tree' if path.is_dir() else 'file',
            'files': files, 'bytes': size, 'sha256': digest.hexdigest()}


def tracked_digest(paths):
    digest = hashlib.sha256()
    for name in sorted(paths):
        path = local(Path(name))
        require(path.is_file(), 'missing tracked input: ' + name)
        digest.update((name + '\0').encode())
        digest.update(path.read_bytes())
    return digest.hexdigest()


def census():
    paths = tracked()
    candidates = []
    skipped = []
    for name in TREES:
        if (ROOT / name).exists() or (ROOT / name).is_symlink():
            try:
                candidates.append(inspect(name, paths))
            except Refusal as error:
                skipped.append({'path': name, 'reason': str(error)})
    # Probe/build stores and source inputs are preserved. Walk only the project's target output.
    for directory, dirs, files in os.walk(ROOT / 'target', followlinks=False):
        rel = Path(directory).relative_to(ROOT).as_posix()
        if any(under(rel, prefix) for prefix in (*TREES, *PROTECTED)):
            dirs[:] = []
            continue
        if '.git' in dirs or '.git' in files:
            skipped.append({'path': rel, 'reason': 'nested repository'})
            dirs[:] = []
            continue
        safe_dirs = []
        for name in dirs:
            child = Path(directory) / name
            if child.is_symlink() or child.stat().st_dev != ROOT.stat().st_dev:
                skipped.append({'path': child.relative_to(ROOT).as_posix(), 'reason': 'link/device'})
            else:
                safe_dirs.append(name)
        dirs[:] = safe_dirs
        for name in files:
            if name.endswith(SUFFIXES) or name == '.DS_Store':
                item = (Path(directory) / name).relative_to(ROOT).as_posix()
                try:
                    candidates.append(inspect(item, paths))
                except Refusal as error:
                    skipped.append({'path': item, 'reason': str(error)})
    return {'producer_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
            'head': git('rev-parse', 'HEAD').decode().strip(),
            'tracked_sha256': tracked_digest(paths),
            'candidates': sorted(candidates, key=lambda row: row['path']),
            'skipped': sorted(skipped, key=lambda row: row['path'])}


def record_directory(name):
    path = Path(name)
    require(len(path.parts) == 3 and path.parts[:2] == ('target', 'artifact_cleanup_audit'),
            'record must be target/artifact_cleanup_audit/<run>')
    return local(path)


def save(path, value):
    with path.open('x') as stream:
        json.dump(value, stream, sort_keys=True, indent=2)
        stream.write('\n')


def validate_plan(plan, current):
    require(current == plan, 'plan drift: regenerate with a fresh run identity')


def apply(directory):
    plan_path = local((directory / 'plan.json').relative_to(ROOT))
    plan = json.loads(plan_path.read_text())
    require(not (directory / 'result.json').exists(), 'run already applied')
    require(shutil.rmtree.avoids_symlink_attacks, 'runtime lacks safe directory removal')
    validate_plan(plan, census())
    subprocess.run(['bash', str(ROOT / 'scripts/check_no_background_jobs.sh')], cwd=ROOT, check=True)
    for row in plan['candidates']:
        # Recheck immediately before each removal as well as against the complete frozen census.
        require(inspect(row['path'], tracked()) == row, 'candidate drift: ' + row['path'])
        path = local(Path(row['path']))
        if row['kind'] == 'tree':
            shutil.rmtree(path)
        else:
            path.unlink()
    require(all(not (ROOT / row['path']).exists() and not (ROOT / row['path']).is_symlink()
                for row in plan['candidates']), 'selected residue')
    require(tracked_digest(tracked()) == plan['tracked_sha256'], 'tracked input changed')
    result = {'removed_trees': sum(row['kind'] == 'tree' for row in plan['candidates']),
              'removed_strays': sum(row['kind'] == 'file' for row in plan['candidates']),
              'removed_files': sum(row['files'] for row in plan['candidates']),
              'removed_bytes': sum(row['bytes'] for row in plan['candidates']),
              'residue': 0, 'tracked_change': False, 'skipped': plan['skipped']}
    save(directory / 'result.json', result)
    return result


def main():
    require(len(sys.argv) == 3 and sys.argv[1] in ('plan', 'apply'),
            'usage: cleanup.py plan|apply target/artifact_cleanup_audit/<run>')
    directory = record_directory(sys.argv[2])
    if sys.argv[1] == 'plan':
        directory.mkdir(parents=True, exist_ok=True)
        value = census()
        save(directory / 'plan.json', value)
        print('cleanup plan:', len(value['candidates']), 'candidates;', len(value['skipped']), 'skipped')
    else:
        print('cleanup result:', json.dumps(apply(directory), sort_keys=True))


if __name__ == '__main__':
    try:
        main()
    except (Refusal, OSError, subprocess.CalledProcessError, ValueError) as error:
        print('cleanup refused:', error, file=sys.stderr)
        sys.exit(2)
