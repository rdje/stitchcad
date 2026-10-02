"""D99: independent authored rows exercise the published book replay consumer."""
from pathlib import Path
import os
import runpy
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[4]
WORK = ROOT / 'target/formula_binding_replay'
SOURCE = ROOT / 'docs/book/src'
checks = 0


def replay(rows, refusal=None, remove_boolean=False):
    global checks
    if WORK.exists():
        assert not (WORK / '.git').exists(), 'scratch cannot be a Git repository'
        shutil.rmtree(WORK)
    shutil.copytree(SOURCE, WORK)
    examples = WORK / 'spec/formula-language/examples.md'
    text = examples.read_text()
    anchor = [line for line in text.splitlines() if line.startswith('| `dart_count` |')]
    assert len(anchor) == 1, 'binding replay anchor changed'
    added = ''.join('\n| `%s` | %s | `%s` | %s | independent replay |' % row for row in rows)
    examples.write_text(text.replace(anchor[0], anchor[0] + added, 1))
    if remove_boolean:
        contract = WORK / 'spec/formula-language.md'
        lines = contract.read_text().splitlines()
        matches = [i for i, line in enumerate(lines) if line.startswith('| `boolean` |')]
        assert len(matches) == 1
        index = matches[0]
        assert lines[index].count('yes') == 1
        lines[index] = lines[index].replace('yes', 'no', 1)
        contract.write_text('\n'.join(lines) + '\n')
    result = subprocess.run(
        ['bash', 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'],
        cwd=ROOT, env=dict(os.environ, FORMULA_BOOK=str(WORK)), capture_output=True, text=True)
    output = result.stdout + result.stderr
    (WORK / 'replay.log').write_text(output)
    if refusal is None:
        expected = 'bindings: %d · mismatches: 0' % (17 + len(rows))
        assert result.returncode == 0 and expected in output, (result.returncode, expected, output)
    else:
        assert result.returncode == 1 and refusal in output, (result.returncode, refusal, output)
    checks += 1


# Independently computed values; source syntax gets no Area or Boolean literal extension.
valid = [
    ('replay_area', 'area', '2 mm * 2 mm', '0.04 cm²'),
    ('replay_negative', 'area', '-2 mm * 2 mm', '-0.04 cm²'),
    ('replay_half', 'area', '(1 um / 2) * 1 um', '0.00000001 cm²'),
    ('replay_negative_half', 'area', '(-1 um / 2) * 1 um', '-0.00000001 cm²'),
    ('replay_under_half', 'area', '(1 um / 10) * 1 um', '0.00000000 cm²'),
    ('replay_area_read', 'area', 'replay_half + replay_half', '0.00000002 cm²'),
    ('replay_true', 'boolean', '1 == 1', 'true'),
    ('replay_false', 'boolean', '1 != 1', 'false'),
    ('replay_bool_read', 'boolean', 'replay_false', 'false'),
    ('replay_conditional', 'count', 'if(replay_false, 9, 2)', '2'),
    ('replay_conditional_true', 'count', 'if(replay_true, 3, 9)', '3'),
    ('replay_count', 'count', '1 / 2.0', '1'),
    ('replay_ratio', 'ratio', '1 / 2', '0.500000'),
    ('replay_length', 'length', '-1 um / 2', '-0.0001 cm'),
    ('replay_angle', 'angle', '-720 deg', '-720.000000 deg'),
]
replay(valid)
# Each cell is deliberately wrong in a different way; a parser crash is not a refusal.
for kind, expr, cell, signature in [
    ('area', '2 mm * 2 mm', '4.00 cm²', 'computes to 0.04'),
    ('area', '2 mm * 2 mm', '0.04 cm', 'publishes cm but the expression is area'),
    ('area', '2 mm * 2 mm', '0.04 deg', 'publishes deg but the expression is area'),
    ('area', '2 mm * 2 mm', 'true', 'is not a number'),
    ('boolean', '1 == 1', 'false', 'computes to true'),
    ('boolean', '1 != 1', 'true', 'computes to false'),
    ('boolean', '1 == 1', '1', 'is not true or false'),
    ('boolean', '1 == 1', 'TRUE', 'is not true or false'),
    ('boolean', '1 == 1', 'true deg', 'is not true or false'),
    ('length', '1 cm', '1.0 cm²', 'publishes cm² but the expression is length'),
    ('angle', '1 deg', '1.0 cm²', 'publishes cm² but the expression is angle'),
    ('ratio', '1.0', '1.0 cm²', 'publishes cm² but the expression is ratio'),
    ('count', '1', '1 cm²', 'publishes cm² but the expression is count'),
]:
    replay([('replay_invalid', kind, expr, cell)], 'examples §2 `replay_invalid`: ' + signature
           if signature.startswith('publishes') else signature)
for kind, expr in [('point', 'dart_apex_front'), ('edge', 'cb_seam')]:
    replay([('replay_opaque', kind, expr, '0')], '`%s` is no kind a let may bind' % kind)
replay([('replay_removed', 'boolean', '1 == 1', 'true')],
       '`boolean` is no kind a let may bind', remove_boolean=True)

# Directly load the actual formatter, never a second implementation. Invalid Boolean state
# must not become truthy text. Both valid states have already passed real copied-book replay.
namespace = runpy.run_path(str(ROOT / 'docs/tasks/artifacts/formula_structure/formula_input.py'))['load_context']()[0]
for invalid in [-1, 2]:
    error = None
    try:
        namespace['render']('boolean', invalid, 0)
    except namespace['FErr'] as refused:
        error = refused
    assert error is not None and error.token == 'formula_domain', ('Boolean formatter accepted invalid state', invalid, error)
    assert error.msg == 'Boolean display requires 0 or 1, measured=%s' % invalid, error
    checks += 1
print('reference binding replay: %d independent consumer/format/declaration controls / 0 fail' % checks)
