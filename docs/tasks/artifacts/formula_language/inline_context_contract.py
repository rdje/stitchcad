"""D91: independently authored scratch-book verdicts for single-span foreign-code context."""
from pathlib import Path
import os
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[4]
WORK = ROOT / 'target/formula_inline_context'
TOOL = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
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

def contracts():
    WORK.mkdir(parents=True, exist_ok=True)
    for name, path, snippet, verdict, reason in CASES:
        book = WORK / 'src'
        if book.exists(): shutil.rmtree(book)
        shutil.copytree(ROOT / 'docs/book/src', book)
        target = book / path
        target.write_text(target.read_text() + '\n\n' + snippet + '\n')
        result = subprocess.run(['bash', str(TOOL)], env={**os.environ, 'FORMULA_BOOK': str(book)},
                                cwd=ROOT, capture_output=True, text=True)
        output = result.stdout + result.stderr
        assert result.returncode == verdict, (name, result.returncode, verdict, output)
        assert not reason or reason in output, (name, 'wrong refusal', output)
        print('inline context %s: rc=%d, expected verdict' % (name, result.returncode))
    print('inline context contracts: %d independently authored book verdicts / 0 fail' % len(CASES))

if __name__ == '__main__': contracts()
