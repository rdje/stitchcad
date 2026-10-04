"""Compile actual recipe-owner documentation guards against Cargo's current reported artifact."""
from pathlib import Path
import json
import os
import re
import subprocess

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-core/src/recipe/checked_recipe.rs'
EXPECTED = ('E0451', 'E0515', 'E0451', 'E0515', 'E0515')


def codes_match(diagnostics, expected):
    return bool(diagnostics) and set(diagnostics) == {expected}


assert codes_match(['E0451'], 'E0451')
assert codes_match(['E0515', 'E0515'], 'E0515')
for diagnostics in ([], ['E0432'], ['E0451', 'E0514'], ['E0451', 'E0560']):
    assert not codes_match(diagnostics, 'E0451'), ('compiler guard accepted noise', diagnostics)
blocks = re.findall(r'(?m)^/// ```compile_fail\n((?:^///.*\n)+?)^/// ```$', SOURCE.read_text())
assert len(blocks) == len(EXPECTED), 'actual recipe-owner guard population'
WORK = ROOT / 'target/recipe_owner_contract'
WORK.mkdir(parents=True, exist_ok=True)
environment = dict(os.environ, CARGO_HOME=str(ROOT / 'target/cargo-home'),
                   CARGO_TARGET_DIR=str(ROOT / 'target'), TMPDIR=str(ROOT / 'target/scratch'))
built = subprocess.run(['cargo', 'build', '-p', 'sc-core', '--message-format=json'],
                       cwd=ROOT, env=environment, capture_output=True, text=True)
(WORK / 'artifacts.json').write_text(built.stdout)
(WORK / 'build.log').write_text(built.stderr)
assert built.returncode == 0, ('Cargo current artifact build failed', built.stderr)
artifacts = set()
library_directories = set()
for line in built.stdout.splitlines():
    message = json.loads(line)
    if message.get('reason') == 'compiler-artifact':
        library_directories.update(Path(filename).parent for filename in message['filenames']
                                   if filename.endswith(('.rlib', '.rmeta')))
    if (message.get('reason') == 'compiler-artifact' and message['target']['name'] == 'sc_core'
            and message['target']['kind'] == ['lib']):
        artifacts.update(Path(filename) for filename in message['filenames'] if filename.endswith('.rlib'))
assert len(artifacts) == 1, ('current Cargo artifact refused', artifacts)
artifact = artifacts.pop()
assert artifact.is_relative_to(ROOT / 'target') and artifact.is_file(), 'foreign/missing compiler artifact'
assert library_directories and all(path.is_relative_to(ROOT / 'target') and path.is_dir()
                                   for path in library_directories), 'foreign/missing dependency directory'
search_paths = [argument for path in sorted(library_directories) for argument in ('-L', 'dependency=' + str(path))]
for index, (block, expected) in enumerate(zip(blocks, EXPECTED), 1):
    code = '\n'.join(line.removeprefix('/// ').removeprefix('///') for line in block.splitlines()) + '\nfn main() {}\n'
    source = WORK / ('guard_%d.rs' % index)
    source.write_text(code)
    result = subprocess.run(['rustc', '--edition=2021', '--error-format=json', '--crate-name', 'recipe_guard_%d' % index,
                             '--extern', 'sc_core=' + str(artifact), *search_paths,
                             str(source), '-o', str(WORK / ('guard_%d' % index))],
                            cwd=ROOT, env=environment, capture_output=True, text=True)
    (WORK / ('guard_%d.log' % index)).write_text(result.stderr)
    diagnostics = []
    for line in result.stderr.splitlines():
        message = json.loads(line)
        if message.get('level') == 'error' and message.get('code') is not None:
            diagnostics.append(message['code']['code'])
    assert result.returncode == 1 and codes_match(diagnostics, expected), (index, expected, diagnostics, result.stderr)
    print('actual recipe-owner compiler guard %d: rc=1, only %s' % (index, expected))
print('recipe-owner contracts: five actual privacy/recipe/record compiler guards; unrelated errors refused; rc=0')

book = ROOT / 'docs/book/src/annexes/formula-checked-recipes.md'
examples = re.findall(r'(?m)^```rust\n(.*?)^```$', book.read_text(), re.S)
assert len(examples) == 4, 'whole recipe book example population'
for index, code in enumerate(examples, 1):
    source = WORK / ('example_%d.rs' % index)
    binary = WORK / ('example_%d' % index)
    source.write_text('fn main() {\n' + code + '\n}\n')
    result = subprocess.run(['rustc', '--edition=2021', '--crate-name', 'recipe_example_%d' % index,
                             '--extern', 'sc_core=' + str(artifact), *search_paths,
                             str(source), '-o', str(binary)], cwd=ROOT, env=environment,
                            capture_output=True, text=True)
    (WORK / ('example_%d.log' % index)).write_text(result.stderr)
    assert result.returncode == 0, ('actual book example compilation', index, result.stderr)
    result = subprocess.run([str(binary)], cwd=ROOT, env=environment, capture_output=True, text=True)
    assert result.returncode == 0, ('actual book example body', index, result.stderr)
print('recipe book contracts:four actual compiled/executed chapter examples; rc=0')
