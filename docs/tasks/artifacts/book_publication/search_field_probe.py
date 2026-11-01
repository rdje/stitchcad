"""Exercise native mdBook field isolation through a tiny public-input book in a fresh local capture.

The optional ambient renderer is a read-only host dependency used solely as a diagnostic baseline.
All fixture inputs, output, logs and temporary stores remain on the repository volume.
"""
import argparse
import json
import os
from pathlib import Path
import runpy
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[4]
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PUBLICATION_SOURCE = Path(__file__).with_name('book_publication.py')
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
PROFILE['plan_producer'](ROOT, sources=(Path(__file__), PROFILE_SOURCE, PUBLICATION_SOURCE))
BOOK = runpy.run_path(str(PUBLICATION_SOURCE))
TOOL = Path('target/cargo-home/bin/mdbook')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', required=True)
    parser.add_argument('--ambient', action='store_true')
    args = parser.parse_args()
    relative = Path(args.output)
    assert not relative.is_absolute() and '..' not in relative.parts and relative.parts[:1] == ('target',), 'local target required'
    work = ROOT / relative
    PROFILE['directory'](ROOT, relative, missing_ok=True)
    assert not work.exists(), 'fresh capture required'
    assert not any(key.upper().startswith('MDBOOK_') and key != 'MDBOOK_LOG' for key in os.environ), 'unplanned mdBook override'
    executable = shutil.which('mdbook') if args.ambient else str(ROOT / TOOL)
    assert executable, 'ambient renderer unavailable'
    sources = (Path(__file__), PROFILE_SOURCE, PUBLICATION_SOURCE)
    if not args.ambient:
        sources += (ROOT / TOOL,)
    source = work / 'book' / 'src'
    destination = work / 'rendered'
    inputs = {work / 'book' / 'book.toml': '[book]\ntitle = "Probe"\nsrc = "src"\n\n[output.html.search]\nheading-split-level = 1\n',
              source / 'SUMMARY.md': '# Summary\n\n- [Juniper](intro.md)\n',
              source / 'intro.md': '# Saffron Saffron Saffron\n\nSaffron cobalt\n'}
    outputs = (*inputs, work / 'build.log', work / 'result.json')
    PROFILE['plan_producer'](ROOT, directories=(work, source, destination), output_files=outputs, sources=sources)
    # Reserve the capture before activation: another writer must never be overwritten.
    PROFILE['directory'](ROOT, relative.parent, create=True)
    work.mkdir()
    PROFILE['enter_producer'](ROOT, directories=(source, destination), output_files=outputs, sources=sources)
    for path, text in inputs.items():
        path.write_text(text)
    version = subprocess.run([executable, '--version'], cwd=ROOT, capture_output=True, text=True, check=True)
    environment = dict(os.environ, MDBOOK_LOG='warn', MDBOOK_BUILD__CREATE_MISSING='false')
    result = subprocess.run([executable, 'build', str(work / 'book'), '--dest-dir', str(destination)],
                            cwd=ROOT, env=environment, capture_output=True, text=True)
    diagnostic = result.stdout + result.stderr
    (work / 'build.log').write_text(diagnostic)
    assert result.returncode == 0, diagnostic
    assert not diagnostic.strip(), 'native probe must build without warnings'
    _, files = BOOK['source_tree'](destination)
    PROFILE['plan_producer'](ROOT, sources=(*sources, *files))
    data = BOOK['search_data'](destination)
    assert len(data['doc_urls']) == 1 and len(data['index']['documentStore']['docs']) == 1
    fields = data['index']['index']
    def frequency(field, term):
        node = fields[field]['root']
        for char in term:
            node = node.get(char, {})
        return node.get('docs', {}).get('0', {}).get('tf')
    observed = dict(body_cobalt=frequency('body', 'cobalt'), body_saffron=frequency('body', 'saffron'),
                    breadcrumb_cobalt=frequency('breadcrumbs', 'cobalt'), title_cobalt=frequency('title', 'cobalt'))
    passed = observed == dict(body_cobalt=1.0, body_saffron=1.0, breadcrumb_cobalt=None, title_cobalt=None)
    report = dict(tool='ambient' if args.ambient else TOOL.as_posix(), version=version.stdout.strip(),
                  observed=observed, passed=passed)
    (work / 'result.json').write_text(json.dumps(report, sort_keys=True, indent=2) + '\n')
    print(json.dumps(report, sort_keys=True))
    assert passed, 'native field leakage/frequency error; retain capture for diagnosis'
    print('native search field isolation: body frequency1; unrelated fields exclude cobalt; rc=0')


if __name__ == '__main__':
    main()
