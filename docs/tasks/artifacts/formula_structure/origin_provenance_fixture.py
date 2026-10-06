"""Pure source and complete copied-book plans for origin and contribution controls."""
from pathlib import Path
import os
import runpy
import shutil
import sys

ROOT = Path(__file__).resolve().parents[4]
HERE = ROOT / 'docs/tasks/artifacts/formula_structure'
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
BOOK = ROOT / 'docs/book/src'
INPUT = HERE / 'formula_input.py'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
BOOT_INPUTS = (Path(__file__), PROFILE_SOURCE, SOURCE, HERE / 'origin_value_contract.py',
               HERE / 'provenance_contract.py', INPUT, HERE / 'formula_structure.py',
               HERE / 'static_signature_contract.py')
try:
    PROFILE['plan_producer'](ROOT, sources=BOOT_INPUTS)
except (PROFILE['Refusal'], OSError) as error:
    print('local-environment: REFUSED — ' + str(error), file=sys.stderr)
    raise SystemExit(2) from None
CONTEXT = runpy.run_path(str(INPUT))
PROFILE = CONTEXT['PROFILE']
REFERENCE_INPUTS = tuple(dict.fromkeys((*BOOT_INPUTS, *CONTEXT['REFERENCE_INPUTS'])))
BASES = {
    'missing_size': ROOT / 'target/formula_structure/origin_value_consumer/missing_size/src',
    'missing_tolerance': ROOT / 'target/formula_structure/origin_value_consumer/missing_tolerance/src',
    'provenance': ROOT / 'target/formula_structure/provenance_consumer/src',
}


def reference_plan():
    """Guard only the public numerical reference's actual inputs; no copied fixture needed."""
    PROFILE['plan_producer'](ROOT, sources=REFERENCE_INPUTS)


def source_plan():
    """Inspect complete reference sources without inspecting or preparing any output."""
    directories, files = CONTEXT['tree_plan'](BOOK)
    sources = tuple(dict.fromkeys((*REFERENCE_INPUTS, *files)))
    PROFILE['plan_producer'](ROOT, sources=sources)
    return directories, files, sources


def consumer_inputs(environment=None):
    return CONTEXT['consumer_inputs'](environment)


def fixture_plan(labels, *, activate=False):
    """Check every requested replacement, selector, log and copied mutant before effects."""
    PROFILE['require'](len(set(labels)) == len(labels) and set(labels) <= set(BASES),
                       'unowned origin/provenance fixture label')
    book_directories, book_files, sources = source_plan()
    parts, selected = consumer_inputs(dict(os.environ, FORMULA_BOOK=str(BOOK)))
    sources = tuple(dict.fromkeys((*sources, *selected)))
    names = {path.relative_to(BOOK) for path in book_files}
    dirs = {path.relative_to(BOOK) for path in book_directories}
    bases = tuple(BASES[label] for label in labels)
    outputs = tuple(base / name for base in bases for name in sorted(names))
    outputs += tuple(base.parent / 'consumer.log' for base in bases)
    if 'provenance' in labels:
        outputs += (BASES['provenance'].parent / 'mutant.sh',)
    directories = tuple(base / name for base in bases for name in sorted(dirs))
    PROFILE['require'](not any(parts == base or parts.is_relative_to(base) for base in bases),
                       'copied-book output aliases selected input directory')
    PROFILE['require'](not set(outputs) & set(sources), 'copied-book output aliases selected input')
    for base in bases:
        old_dirs, old_files = CONTEXT['tree_plan'](base, output=True)
        PROFILE['require'](all(path.relative_to(base) in dirs for path in old_dirs)
                           and all(path.relative_to(base) in names for path in old_files),
                           'unexpected copied-book fixture entry')
    plan = dict(directories=directories, output_files=outputs, sources=sources)
    PROFILE['plan_producer'](ROOT, **plan)
    return PROFILE['prepare_producer'](ROOT, **plan) if activate else plan


def copied_plan(label):
    """Refresh all copied inputs and mutable destinations immediately before an edit."""
    PROFILE['require'](label in BASES, 'unowned origin/provenance fixture label')
    _, files = CONTEXT['tree_plan'](BASES[label], output=True)
    PROFILE['plan_producer'](ROOT, sources=(SOURCE, *files), output_files=files)


def copy_book(label):
    fixture_plan((label,))
    book = BASES[label]
    if book.exists():
        shutil.rmtree(book)
    shutil.copytree(BOOK, book)
    copied_plan(label)
    return book
