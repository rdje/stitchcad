"""Pure complete signed_angle plans shared by the producing entry and its exclusive source writer."""
from pathlib import Path
import os
import re
import runpy
import shutil
import sys

ROOT = Path(__file__).resolve().parents[4]
HERE = ROOT / 'docs/tasks/artifacts/formula_structure'
ENTRY = HERE / 'signed_angle_contract.py'
WRITER = HERE / 'signed_angle_mutations.py'
INPUT = HERE / 'formula_input.py'
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
BOOK = ROOT / 'docs/book/src'
WORK = ROOT / 'target/formula_signed_angle_replay'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
BOOT_INPUTS = (Path(__file__), PROFILE_SOURCE, SOURCE, ENTRY, WRITER, INPUT,
               HERE / 'formula_structure.py', HERE / 'static_signature_contract.py')
try:
    PROFILE['plan_producer'](ROOT, sources=BOOT_INPUTS)
except (PROFILE['Refusal'], OSError) as error:
    print('local-environment: REFUSED — ' + str(error), file=sys.stderr)
    raise SystemExit(2) from None
CONTEXT = runpy.run_path(str(INPUT))
PROFILE = CONTEXT['PROFILE']
REFERENCE_INPUTS = tuple(dict.fromkeys((*BOOT_INPUTS, *CONTEXT['REFERENCE_INPUTS'])))

def tree_plan(base, *, output=False):
    """Walk only inspected directories; do not follow links or enter another repository."""
    relative = PROFILE['producer_path'](ROOT, base)
    if output:
        PROFILE['producer_output_path'](ROOT, base)
    PROFILE['directory'](ROOT, relative, missing_ok=output)
    if not base.exists():
        return (), ()
    directories, files = [base], []
    for folder in directories:
        PROFILE['directory'](ROOT, folder.relative_to(ROOT).as_posix())
        for path in sorted(folder.iterdir()):
            PROFILE['require'](not path.is_symlink(), 'symlink copied-book entry')
            if path.is_dir():
                directories.append(path)
            else:
                files.append(path)
    PROFILE['plan_producer'](ROOT, **{'output_files' if output else 'sources': tuple(files)})
    return tuple(directories), tuple(files)

def consumer_inputs(environment=None):
    """Match the shell's effective selectors and lexical reference resolution before effects."""
    environment = os.environ if environment is None else environment
    book = Path(environment.get('FORMULA_BOOK') or BOOK)
    book = book if book.is_absolute() else ROOT / book
    parts = Path(environment.get('FORMULA_PARTS_DIR') or book / 'spec/formula-language')
    selected = tuple(Path(environment.get(name) or default) for name, default in (
        ('FORMULA_CONTRACT', book / 'spec/formula-language.md'),
        ('FORMULA_GRAMMAR', parts / 'grammar.md'),
        ('FORMULA_EXAMPLES', parts / 'examples.md'),
        ('FORMULA_FIXTURE', book / 'spec/reference-skirt.md')))
    # The shell runs in ROOT; keep a relative caller selector relative to that same root.
    selected = tuple(path if path.is_absolute() else ROOT / path for path in selected)
    parts = parts if parts.is_absolute() else ROOT / parts
    PROFILE['directory'](ROOT, PROFILE['producer_path'](ROOT, parts))
    PROFILE['plan_producer'](ROOT, sources=selected)
    references = []
    for path in selected[:3]:
        for _, target in re.findall(r'\[([^\]]*)\]\(([^)]+)\)', path.read_text()):
            if target.startswith(('http://', 'https://', '#')):
                continue
            # Concatenation intentionally matches the public resolver, including a leading slash.
            references.append(Path(os.path.normpath(str(path.parent) + '/' + target.split('#')[0])))
    sources = tuple(dict.fromkeys((*selected, *sorted(parts.glob('*.md')), *references)))
    PROFILE['plan_producer'](ROOT, sources=sources)
    return parts, sources


def source_plan():
    directories, files = tree_plan(BOOK)
    sources = tuple(dict.fromkeys((*REFERENCE_INPUTS, *files)))
    PROFILE['plan_producer'](ROOT, sources=sources)
    return directories, files, sources


def signed_angle_plan(*, activate=False):
    """Return the complete pure plan, or explicitly prepare it for producing effects."""
    book_directories, book_files, sources = source_plan()
    parts, selected = consumer_inputs(dict(os.environ, FORMULA_BOOK=str(BOOK)))
    sources = tuple(dict.fromkeys((*sources, *selected)))
    names = {path.relative_to(BOOK) for path in book_files} | {Path('replay.log'), Path('refusal.log')}
    dirs = {path.relative_to(BOOK) for path in book_directories}
    outputs = tuple(WORK / name for name in sorted(names))
    directories = tuple(WORK / name for name in sorted(dirs))
    PROFILE['require'](parts != WORK and not parts.is_relative_to(WORK),
                       'copied-book output aliases selected input directory')
    PROFILE['require'](not set(outputs) & set(sources), 'copied-book output aliases selected input')
    old_dirs, old_files = tree_plan(WORK, output=True)
    PROFILE['require'](all(path.relative_to(WORK) in dirs for path in old_dirs)
                       and all(path.relative_to(WORK) in names for path in old_files),
                       'unexpected copied-book fixture entry')
    plan = dict(directories=directories, output_files=outputs, sources=sources)
    PROFILE['plan_producer'](ROOT, **plan)
    return PROFILE['prepare_producer'](ROOT, **plan) if activate else plan


def copied_plan():
    """Validate all current copied inputs and writable files before reference reads or edits."""
    _, files = tree_plan(WORK, output=True)
    PROFILE['plan_producer'](ROOT, sources=(SOURCE, *files), output_files=files)


def copy_book():
    signed_angle_plan()
    if WORK.exists():
        shutil.rmtree(WORK)
    signed_angle_plan()
    shutil.copytree(BOOK, WORK)
    copied_plan()
    return WORK
