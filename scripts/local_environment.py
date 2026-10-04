"""Run project producers with guarded, repository-local build and temporary stores."""
from pathlib import Path
import argparse
import json
import os
import sys

ROOT = Path(__file__).resolve().parent.parent
STORES = {
    'CARGO_HOME': 'target/cargo-home',
    'RUSTUP_HOME': 'target/cargo-home/rustup-ci',
    'CARGO_TARGET_DIR': 'target',
    'TMPDIR': 'target/scratch',
}


class Refusal(ValueError):
    """A producer environment would escape its project-owned output area."""


def require(condition, message):
    if not condition:
        raise Refusal(message)


def directory(root, relative, *, create=False, missing_ok=False):
    """Inspect existing components before making only the selected local directories."""
    path = Path(relative)
    require(not path.is_absolute() and '..' not in path.parts and '.git' not in path.parts,
            'invalid store components')
    current, device = root, root.stat().st_dev
    for part in path.parts:
        current = current / part
        require(not current.is_symlink(), 'symlink store component')
        if not current.exists():
            if create:
                current.mkdir()
            else:
                require(missing_ok, 'missing store directory')
                continue
        require(current.is_dir(), 'store component is not a directory')
        require(current.stat().st_dev == device, 'store is on another volume')
        marker = current / '.git'
        require(not (marker.exists() or marker.is_symlink()), 'store crosses another Git repository')
    return current


def workspace(root):
    require(root.is_absolute() and '..' not in root.parts, 'invalid workspace path')
    require(root == ROOT or root.is_relative_to(ROOT / 'target'), 'workspace outside project target')
    if root != ROOT:
        directory(ROOT, root.relative_to(ROOT).as_posix())


def select(root, name, value):
    """Ambient external paths do not become project stores; local overrides remain inspectable."""
    if value is None or value == '':
        return root / STORES[name]
    require(isinstance(value, str) and not any(c in value for c in '\x00\r\n'),
            'invalid ' + name + ' value')
    path = Path(value)
    require('..' not in path.parts and '.git' not in path.parts, 'invalid store components')
    if not path.is_absolute():
        path = root / path
    if not path.is_relative_to(root):
        return root / STORES[name]
    require(path.is_relative_to(root / 'target'), 'local store must be under target')
    return path


def environment_for(root, environment, *, create=True):
    """Plan and validate every store before any directory creation; retain unrelated variables."""
    workspace(root)
    toolchain = environment.get('RUSTUP_TOOLCHAIN')
    if toolchain is not None:
        require(isinstance(toolchain, str) and toolchain and
                not any(c in toolchain for c in '/\\\x00\r\n'), 'toolchain override must be a name')
    selected = {name: select(root, name, environment.get(name)) for name in STORES}
    for path in selected.values():
        directory(root, path.relative_to(root).as_posix(), missing_ok=True)
    if create:
        for path in selected.values():
            directory(root, path.relative_to(root).as_posix(), create=True)
    result = dict(environment)
    result.update({name: str(path) for name, path in selected.items()})
    # Installation is an explicit producer operation, using --no-self-update, rather than a proxy side effect.
    result['RUSTUP_AUTO_INSTALL'] = '0'
    result['MAKE_TMPDIR'] = result['TMPDIR']
    return result


def verify(root, environment):
    """Freshly verify effective values; a caller cannot bypass preparation with a marker."""
    wanted = environment_for(root, environment, create=False)
    for name in (*STORES, 'RUSTUP_AUTO_INSTALL', 'MAKE_TMPDIR'):
        require(environment.get(name) == wanted[name], 'effective ' + name + ' is not local')
    for name in STORES:
        directory(root, Path(wanted[name]).relative_to(root).as_posix())


def producer_path(root, value):
    """Resolve a declared owned path without inspecting any external destination."""
    require(isinstance(value, (str, Path)), 'invalid producer path type')
    path = Path(value)
    require(not any(char in str(path) for char in '\x00\r\n'), 'invalid producer path bytes')
    require('..' not in path.parts and '.git' not in path.parts, 'invalid producer path components')
    if not path.is_absolute():
        path = root / path
    require(path.is_relative_to(root), 'producer path outside workspace')
    return path.relative_to(root).as_posix()


def prepare_producer(root, *, directories=(), sources=()):
    """Plan all stores and owned paths before any creation, then activate the child environment."""
    environment_for(root, os.environ, create=False)
    outputs = [producer_path(root, path) for path in directories]
    inputs = [producer_path(root, path) for path in sources]
    for relative in outputs:
        require(Path(relative).parts[:1] == ('target',), 'producer output must be under target')
        directory(root, relative, missing_ok=True)
    for relative in inputs:
        path = root / relative
        directory(root, Path(relative).parent.as_posix())
        require(not path.is_symlink(), 'symlink producer source')
        require(path.is_file() and path.stat().st_dev == root.stat().st_dev,
                'producer source is not a local regular file')
    environment = environment_for(root, os.environ)
    os.environ.update(environment)
    for relative in outputs:
        directory(root, relative, create=True)
    return environment


def enter_producer(root, *, directories=(), sources=()):
    """Standalone Python entry: refuse cleanly before its first owned write or child dispatch."""
    try:
        return prepare_producer(root, directories=directories, sources=sources)
    except (Refusal, OSError) as error:
        print('local-environment: REFUSED — ' + str(error), file=sys.stderr)
        raise SystemExit(2) from None


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--verify', action='store_true')
    parser.add_argument('--inspect', action='store_true')
    parser.add_argument('command', nargs=argparse.REMAINDER)
    args = parser.parse_args()
    require(not (args.verify and args.inspect), 'choose one inspection mode')
    command = args.command[1:] if args.command[:1] == ['--'] else args.command
    if args.verify or args.inspect:
        require(not args.command, 'inspection takes no command')
    else:
        require(bool(command), 'a producer command is required')
    if args.verify:
        verify(ROOT, os.environ)
        return
    environment = environment_for(ROOT, os.environ, create=not args.inspect)
    if args.inspect:
        print(json.dumps({name: Path(environment[name]).relative_to(ROOT).as_posix() for name in STORES}))
        return
    os.execvpe(command[0], command, environment)


if __name__ == '__main__':
    try:
        main()
    except (Refusal, OSError) as error:
        print('local-environment: REFUSED — ' + str(error), file=sys.stderr)
        sys.exit(2)
