"""Focused local-tool path/dispatch controls; fake executables are not native search evidence."""
import hashlib
import os
from pathlib import Path
import runpy
import subprocess
import sys
import uuid

ROOT = Path(__file__).resolve().parents[4]
PROFILE = runpy.run_path(str(ROOT / 'scripts/local_environment.py'))
SOURCES = (Path(__file__), ROOT / 'scripts/book_tool.py', ROOT / 'scripts/local_environment.py',
           ROOT / 'docs/tasks/artifacts/book_publication/search_field_probe.py',
           ROOT / 'docs/tasks/artifacts/book_publication/book_publication.py')
WORK = ROOT / 'target/book_tool_contracts' / uuid.uuid4().hex
PROFILE['prepare_producer'](ROOT, sources=SOURCES, directories=(WORK,))
frozen = {path: hashlib.sha256(path.read_bytes()).digest() for path in SOURCES}


def fixture(label):
    root = WORK / label
    outputs = tuple(root / source.relative_to(ROOT) for source in SOURCES[1:])
    PROFILE['prepare_producer'](ROOT, sources=SOURCES, directories=(root, *(path.parent for path in outputs)),
                                output_files=outputs)
    for source, output in zip(SOURCES[1:], outputs):
        output.write_bytes(source.read_bytes())
    return root


def execute(root, action, *, path=None):
    environment = dict(os.environ)
    if path is not None:
        environment['PATH'] = str(path) + os.pathsep + environment['PATH']
    return subprocess.run([sys.executable, '-I', '-B', str(root / 'scripts/book_tool.py'), action],
                          cwd=root, env=environment, capture_output=True, text=True)


root = fixture('missing')
result = execute(root, 'version')
assert result.returncode == 2 and 'REFUSED' in result.stderr and not (root / 'target').exists()

root = fixture('aliased-bin')
store, alias = root / 'target/cargo-home', root / 'alias'
tool = alias / 'mdbook'
PROFILE['prepare_producer'](ROOT, sources=SOURCES, directories=(store, alias), output_files=(tool,))
tool.write_text('#!/bin/sh\necho UNEXPECTED-DISPATCH\n')
tool.chmod(0o755)
(store / 'bin').symlink_to(alias, target_is_directory=True)
result = execute(root, 'version')
assert result.returncode == 2 and 'symlink' in result.stderr and 'UNEXPECTED-DISPATCH' not in result.stdout

for kind in ('symlink', 'hardlink'):
    root = fixture(kind + '-metadata')
    store, sentinel = root / 'target/cargo-home', root / 'sentinel'
    PROFILE['prepare_producer'](ROOT, sources=SOURCES, directories=(store,), output_files=(sentinel,))
    sentinel.write_text('preserve this fixture input\n')
    metadata = store / '.crates.toml'
    if kind == 'symlink':
        metadata.symlink_to(sentinel)
    else:
        os.link(sentinel, metadata)
    result = execute(root, 'install')
    assert result.returncode == 2 and 'REFUSED' in result.stderr
    assert not (store / 'bin').exists() and sentinel.read_text() == 'preserve this fixture input\n'

root = fixture('version-dispatch')
tool = root / 'target/cargo-home/bin/mdbook'
PROFILE['prepare_producer'](ROOT, sources=SOURCES, directories=(tool.parent,), output_files=(tool,))
tool.write_text('#!/bin/sh\necho "controlled mdBook argument:$1"\n')
tool.chmod(0o755)
result = execute(root, 'version')
assert result.returncode == 0 and result.stdout.strip() == 'controlled mdBook argument:--version'

root = fixture('cargo-dispatch')
cargo = root / 'controlled-bin/cargo'
PROFILE['prepare_producer'](ROOT, sources=SOURCES, directories=(cargo.parent,), output_files=(cargo,))
cargo.write_text('#!/bin/sh\nprintf "%s\\n" "$@"\nexit 7\n')
cargo.chmod(0o755)
result = execute(root, 'install', path=cargo.parent)
assert result.returncode == 2 and 'exit status 7' in result.stderr
assert result.stdout.splitlines() == ['install', 'mdbook', '--version', '0.5.4', '--root', str(root / 'target/cargo-home')]
assert not (root / 'target/cargo-home/bin/mdbook').exists()
assert frozen == {path: hashlib.sha256(path.read_bytes()).digest() for path in SOURCES}
print('book tool:4 pre-dispatch path refusals/2 exact argv+status controls;source bytes unchanged;simulated children only;rc=0')
