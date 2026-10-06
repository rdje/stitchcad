"""D91: independently authored copied-book verdicts with complete producing plans."""
from pathlib import Path
import os
import re
import runpy
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
WORK = ROOT / 'target/formula_inline_context'
BOOK = ROOT / 'docs/book/src'
TOOL = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
BOOT_INPUTS = (Path(__file__), PROFILE_SOURCE, TOOL)
try:
    PROFILE['plan_producer'](ROOT, sources=BOOT_INPUTS)
except (PROFILE['Refusal'], OSError) as error:
    print('local-environment: REFUSED — ' + str(error), file=sys.stderr)
    raise SystemExit(2) from None

CASES = [
    ('annotated Rust', 'spec/units-and-tolerances.md', '<!-- stitchcad-inline: rust -->`(left + right)?`', 0, ''),
    ('ordinary formula', 'spec/units-and-tolerances.md', '`left + right`', 0, ''),
    ('unannotated question mark', 'spec/units-and-tolerances.md', '`(left + right)?`', 1, 'L6b'),
    ('adjacent bad formula', 'spec/units-and-tolerances.md', '<!-- stitchcad-inline: rust -->`(left + right)?` and `left + right ?`', 1, 'L6b'),
    ('adjacent valid formula', 'spec/units-and-tolerances.md', '<!-- stitchcad-inline: rust -->`(left + right)?` and `left + right`', 0, ''),
    ('unknown language', 'spec/units-and-tolerances.md', '<!-- stitchcad-inline: rustish -->`(left + right)?`', 1, 'L6e'),
    ('unknown annotation on ordinary span', 'spec/units-and-tolerances.md', '<!-- stitchcad-inline: rustish -->`left + right`', 1, 'L6e'),
    ('detached marker', 'spec/units-and-tolerances.md', '<!-- stitchcad-inline: rust --> words `(left + right)?`', 1, 'L6e'),
    ('next-line marker', 'spec/units-and-tolerances.md', '<!-- stitchcad-inline: rust -->\n`(left + right)?`', 1, 'L6e'),
    ('unclosed marker', 'spec/units-and-tolerances.md', '<!-- stitchcad-inline: rust `(left + right)?`', 1, 'L6e'),
    ('normative formula position', 'spec/formula-language/grammar.md', '<!-- stitchcad-inline: rust -->`left + right`', 1, 'L6e'),
    ('duplicate marker', 'spec/units-and-tolerances.md', '<!-- stitchcad-inline: rust --><!-- stitchcad-inline: rust -->`(left + right)?`', 1, 'L6e'),
    ('Rust fence', 'spec/units-and-tolerances.md', '```rust\nlet value = (left + right)?;\n```', 0, ''),
]


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


def source_plan():
    """Inspect the entire copied book and its loaders without creating or activating anything."""
    directories, files = tree_plan(BOOK)
    sources = (*BOOT_INPUTS, *files)
    PROFILE['plan_producer'](ROOT, sources=sources)
    return directories, files, sources


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

def fixture_plan(*, activate=True):
    """Inspect the complete book and replacement before any activation or copy effect."""
    book_directories, book_files, sources = source_plan()
    parts, selected = consumer_inputs(dict(os.environ, FORMULA_BOOK=str(BOOK)))
    sources = tuple(dict.fromkeys((*sources, *selected)))
    base = WORK / 'src'
    file_names = {path.relative_to(BOOK) for path in book_files}
    directory_names = {path.relative_to(BOOK) for path in book_directories}
    PROFILE['require'](parts != base and not parts.is_relative_to(base),
                       'copied-book output aliases selected input directory')
    directories = (WORK, *(base / name for name in sorted(directory_names)))
    outputs = tuple(base / name for name in sorted(file_names))
    PROFILE['require'](not set(outputs) & set(sources), 'copied-book output aliases selected input')
    old_directories, old_files = tree_plan(base, output=True)
    PROFILE['require'](all(path.relative_to(base) in directory_names for path in old_directories)
                       and all(path.relative_to(base) in file_names for path in old_files),
                       'unexpected copied-book fixture entry')
    plan = dict(directories=directories, output_files=outputs, sources=sources)
    PROFILE['plan_producer'](ROOT, **plan)
    return PROFILE['prepare_producer'](ROOT, **plan) if activate else plan


def copy_book():
    """Freshly inspect every replacement component immediately before deleting the old copy."""
    fixture_plan(activate=False)
    book = WORK / 'src'
    if book.exists():
        shutil.rmtree(book)
    shutil.copytree(BOOK, book)
    _, files = tree_plan(book, output=True)
    PROFILE['plan_producer'](ROOT, sources=(TOOL, *files), output_files=files)
    return book


def contracts():
    environment = fixture_plan()
    WORK.mkdir(parents=True, exist_ok=True)
    for name, path, snippet, verdict, reason in CASES:
        book = copy_book()
        target = book / path
        target.write_text(target.read_text() + '\n\n' + snippet + '\n')
        source_plan()
        consumer_inputs(dict(environment, FORMULA_BOOK=str(book)))
        result = subprocess.run(['bash', str(TOOL)], env={**environment, 'FORMULA_BOOK': str(book)},
                                cwd=ROOT, capture_output=True, text=True)
        output = result.stdout + result.stderr
        assert result.returncode == verdict, (name, result.returncode, verdict, output)
        assert not reason or reason in output, (name, 'wrong refusal', output)
        print('inline context %s: rc=%d, expected verdict' % (name, result.returncode))
    print('inline context contracts: %d independently authored book verdicts / 0 fail' % len(CASES))

if __name__ == '__main__':
    try:
        contracts()
    except (PROFILE['Refusal'], OSError) as error:
        print('local-environment: REFUSED — ' + str(error), file=sys.stderr)
        raise SystemExit(2) from None
