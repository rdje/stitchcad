"""Capture fresh same-source native search renderings under an explicit repository-local target.

Diagnostic only: the normal publisher still refuses every renderer warning. This capture retains
the actual default warning and warning-free chapter candidate, without changing public output.
"""
from pathlib import Path
import argparse
import ast
import hashlib
import json
import os
import re
import runpy
import subprocess

ROOT = Path(__file__).resolve().parents[4]
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PUBLICATION_SOURCE = Path(__file__).with_name('book_publication.py')
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
PROFILE['plan_producer'](ROOT, sources=(Path(__file__), PROFILE_SOURCE, PUBLICATION_SOURCE))
BOOK = runpy.run_path(str(PUBLICATION_SOURCE))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', required=True)
    args = parser.parse_args()
    relative = Path(args.output)
    assert not relative.is_absolute() and '..' not in relative.parts and relative.parts[:1] == ('target',), 'repository-local target required'
    work = ROOT / relative
    PROFILE['directory'](ROOT, relative, missing_ok=True)
    assert not work.exists(), 'fresh capture directory required; preserve earlier snapshots'
    BOOK['main_inputs'](ROOT)
    input_dirs, inputs = BOOK['build_inputs'](ROOT)
    outputs = (work / 'source-sha256.json', work / 'measurements.json', work / 'tool-version.txt',
               *(work / ('level%d.log' % level) for level in (3, 1)))
    PROFILE['enter_producer'](ROOT, directories=(work, *(work / ('level%d' % level) for level in (3, 1))),
        output_files=outputs, sources=(Path(__file__), PUBLICATION_SOURCE, *inputs))
    work.mkdir(parents=True, exist_ok=True)
    hashes = {str(path.relative_to(ROOT)): hashlib.sha256(path.read_bytes()).hexdigest() for path in inputs}
    (work / 'source-sha256.json').write_text(json.dumps(hashes, sort_keys=True, indent=2) + '\n')
    version = subprocess.run(['mdbook', '--version'], cwd=ROOT, capture_output=True, text=True)
    assert version.returncode == 0, version.stderr
    (work / 'tool-version.txt').write_text(version.stdout)
    measurements = {}
    for level in (3, 1):
        destination = work / ('level%d' % level)
        env = dict(os.environ, MDBOOK_LOG='warn', RUST_LOG='warn',
                   MDBOOK_OUTPUT__HTML__SEARCH__HEADING_SPLIT_LEVEL=str(level))
        result = subprocess.run(['mdbook', 'build', 'docs/book', '--dest-dir', str(destination)],
                                cwd=ROOT, env=env, capture_output=True, text=True)
        diagnostic = result.stdout + result.stderr
        (work / ('level%d.log' % level)).write_text(diagnostic)
        assert result.returncode == 0, (level, result.returncode, diagnostic)
        plain = re.sub(r'\x1b\[[0-9;]*m', '', diagnostic)
        warnings = re.findall(r'(?m)^\s*WARN\b.*$', plain)
        if level == 1:
            assert not warnings, (level, warnings)
        else:
            assert warnings and all('search index is very large' in warning for warning in warnings), (level, warnings)
        _, files = BOOK['source_tree'](destination)
        PROFILE['plan_producer'](ROOT, sources=(Path(__file__), PUBLICATION_SOURCE, *inputs, *files))
        data = BOOK['search_data'](destination)
        index = next(destination.glob('searchindex-*.js'))
        content = index.read_bytes()
        # The native warning measures the raw serialized JSON, not its JavaScript wrapper.
        text = content.decode()
        raw = ast.literal_eval(text[text.index('JSON.parse(') + len('JSON.parse('):-3])
        measurements[str(level)] = dict(documents=len(data['doc_urls']), json_bytes=len(raw.encode()),
            script_bytes=len(content), sha256=hashlib.sha256(content).hexdigest(), warnings=warnings)
        assert BOOK['build_inputs'](ROOT) == (input_dirs, inputs), 'complete native source population changed'
        assert hashes == {str(path.relative_to(ROOT)): hashlib.sha256(path.read_bytes()).hexdigest() for path in inputs}, 'native source bytes changed'
    (work / 'measurements.json').write_text(json.dumps(measurements, sort_keys=True, indent=2) + '\n')
    print('native search capture:', json.dumps(measurements, sort_keys=True), 'sources unchanged; rc=0')


if __name__ == '__main__':
    main()
