"""Pure complete replay fixture plans shared by the producing entry and its exclusive source writer."""
from pathlib import Path
import os
import runpy
import shutil
import sys

ROOT = Path(__file__).resolve().parents[4]
HERE = ROOT / 'docs/tasks/artifacts/formula_structure'
ENTRY = HERE / 'binding_replay_contract.py'
WRITER = HERE / 'binding_replay_mutations.py'
INPUT = HERE / 'formula_input.py'
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
BOOK = ROOT / 'docs/book/src'
WORK = ROOT / 'target/formula_binding_replay'
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
    """Reuse the public source walker with this provider's borrowed repository root."""
    return CONTEXT['tree_plan'](base, output=output)


def consumer_inputs(environment=None):
    """Validate public shell selectors through the pure context provider."""
    return CONTEXT['consumer_inputs'](environment)


def source_plan():
    directories, files = tree_plan(BOOK)
    sources = tuple(dict.fromkeys((*REFERENCE_INPUTS, *files)))
    PROFILE['plan_producer'](ROOT, sources=sources)
    return directories, files, sources


def fixture_plan(*, activate=False):
    """Return the complete pure plan, or explicitly prepare it for producing effects."""
    book_directories, book_files, sources = source_plan()
    parts, selected = consumer_inputs(dict(os.environ, FORMULA_BOOK=str(BOOK)))
    sources = tuple(dict.fromkeys((*sources, *selected)))
    names = {path.relative_to(BOOK) for path in book_files} | {Path('replay.log')}
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
    fixture_plan()
    if WORK.exists():
        shutil.rmtree(WORK)
    shutil.copytree(BOOK, WORK)
    copied_plan()
    return WORK
