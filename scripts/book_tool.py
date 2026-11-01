"""Install or select the published project-local mdBook, without changing host executables."""
import argparse
import os
from pathlib import Path
import runpy
import subprocess
import sys
import uuid

ROOT = Path(__file__).resolve().parent.parent
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
VERSION = '0.5.4'
STORE = Path('target/cargo-home')
TOOL = STORE / 'bin/mdbook'
PROBE = ROOT / 'docs/tasks/artifacts/book_publication/search_field_probe.py'
PUBLICATION = PROBE.with_name('book_publication.py')


def renderer_inputs():
    """Call before output preparation; missing, aliased or foreign tool inputs refuse."""
    sources = (Path(__file__).absolute(), PROFILE_SOURCE, ROOT / TOOL)
    PROFILE['plan_producer'](ROOT, sources=sources)
    PROFILE['require'](os.access(ROOT / TOOL, os.X_OK),
                       'local mdBook is not executable; run scripts/book_tool.py install')
    return sources


def renderer():
    renderer_inputs()
    return str(ROOT / TOOL)


def install(*, force=False):
    """Use Cargo's public install command, then test the installed renderer through its public CLI."""
    sources = (Path(__file__).absolute(), PROFILE_SOURCE, PROBE, PUBLICATION)
    outputs = (ROOT / TOOL, ROOT / STORE / '.crates.toml', ROOT / STORE / '.crates2.json')
    PROFILE['enter_producer'](ROOT, directories=(STORE / 'bin',), output_files=outputs, sources=sources)
    command = ['cargo', 'install', 'mdbook', '--version', VERSION, '--root', str(ROOT / STORE)]
    if force:
        command.append('--force')
    subprocess.run(command, cwd=ROOT, check=True)
    executable = renderer()
    version = subprocess.run([executable, '--version'], cwd=ROOT, capture_output=True, text=True, check=True)
    PROFILE['require'](version.stdout.strip() == 'mdbook v' + VERSION, 'unexpected installed mdBook version')
    # Cargo may skip an existing same-version install that still has the old dependency.
    # The native probe, rather than its version string, determines whether indexing is corrected.
    capture = Path('target/book_search_install_checks') / uuid.uuid4().hex
    subprocess.run([sys.executable, '-I', '-B', str(PROBE), '--output', capture.as_posix()],
                   cwd=ROOT, check=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=('install', 'version', 'build'))
    parser.add_argument('--force', action='store_true', help='rebuild an existing local install')
    args = parser.parse_args()
    PROFILE['require'](not args.force or args.action == 'install', '--force requires install')
    if args.action == 'install':
        install(force=args.force)
    elif args.action == 'build':
        PROFILE['plan_producer'](ROOT, sources=(Path(__file__).absolute(), PROFILE_SOURCE, PUBLICATION))
        publication = runpy.run_path(str(PUBLICATION))
        publication['build'](ROOT)
        print('mdBook: published docs/book/book with the guarded local renderer')
    else:
        executable = renderer()
        PROFILE['enter_producer'](ROOT, sources=renderer_inputs())
        subprocess.run([executable, '--version'], cwd=ROOT, check=True)


if __name__ == '__main__':
    try:
        main()
    except (PROFILE['Refusal'], OSError, subprocess.SubprocessError) as error:
        print('book-tool: REFUSED — ' + str(error), file=sys.stderr)
        print('Use python3 -I -B scripts/book_tool.py install --force to rebuild the local tool.', file=sys.stderr)
        raise SystemExit(2) from None
