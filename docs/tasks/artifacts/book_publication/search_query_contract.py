"""Run the generated search engine/index under read-only host Node; check real chapter landings.

Node receives guarded asset bytes on stdin. No npm packages, browser profile or filesystem cache
is used. This verifies the search engine, not browser controls, ranking quality or reader latency.
"""
import argparse
import copy
import hashlib
import json
import os
from pathlib import Path
import runpy
import shutil
import subprocess
from urllib.parse import unquote, urlsplit

ROOT = Path(__file__).resolve().parents[4]
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PUBLICATION_SOURCE = Path(__file__).with_name('book_publication.py')
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
PROFILE['plan_producer'](ROOT, sources=(Path(__file__), PROFILE_SOURCE, PUBLICATION_SOURCE))
BOOK = runpy.run_path(str(PUBLICATION_SOURCE))
QUERIES = (
    ('JoinSourceIdentityManifest', 'annexes/drafting-join-inventory.html'),
    ('RequiredJoinEffects', 'annexes/drafting-join-inventory.html'),
    ('JoinConsumerSources', 'annexes/drafting-join-inventory.html'),
    ('PrintedLabel', 'annexes/drafting-join-inventory.html'),
    ('JoinSourceIdentityMan', 'annexes/drafting-join-inventory.html'),
    ('JOINSOURCEIDENTITYMANIFEST', 'annexes/drafting-join-inventory.html'),
    ('CloseDart', 'annexes/drafting-operation-contracts.html'),
    ('grainline', 'annexes/drafting-operation-contracts.html'),
    ('Bézier', 'annexes/drafting-operation-contracts.html'),
    ('Buttonhole', 'spec/ontology-closures.html'),
    ('explain', 'introduction.html'),
    ('Full-text book search', 'annexes/book-search.html'),
    ('qzxvabsentquerytoken', None),
    ('', None),
)
JAVASCRIPT = r'''
const fs = require('node:fs');
const vm = require('node:vm');
const input = JSON.parse(fs.readFileSync(0, 'utf8'));
const context = vm.createContext({window: {search: {}}, queries: input.queries, console},
                                {microtaskMode: 'afterEvaluate'});
vm.runInContext(input.engine, context, {timeout: 10000});
vm.runInContext(input.index, context, {timeout: 10000});
const results = vm.runInContext(`
    const data = window.search;
    const index = elasticlunr.Index.load(data.index);
    queries.map(query => index.search(query, data.search_options).map(hit => ({
        url: data.doc_urls[Number(hit.ref)], score: hit.score
    })));
`, context, {timeout: 10000});
process.stdout.write(JSON.stringify({node: process.version, results}));
'''


def check_results(results, urls, anchors):
    assert isinstance(results, list) and len(results) == len(QUERIES), 'query population'
    for (query, expected), hits in zip(QUERIES, results):
        assert isinstance(hits, list), ('query hits', query)
        seen, paths = set(), set()
        for hit in hits:
            assert isinstance(hit, dict) and hit.get('url') in urls, ('foreign result', query)
            assert hit['url'] not in seen, ('duplicate result', query)
            seen.add(hit['url'])
            url = urlsplit(hit['url'])
            path = unquote(url.path)
            assert path in anchors and (not url.fragment or unquote(url.fragment) in anchors[path]), ('result landing', query)
            paths.add(path)
        assert expected in paths if expected is not None else not hits, ('query coverage', query, expected)


def controls(results, urls, anchors):
    for kind in ('missing-query', 'missing-hit', 'foreign-hit', 'duplicate-hit'):
        changed = copy.deepcopy(results)
        if kind == 'missing-query':
            changed.pop()
        elif kind == 'missing-hit':
            changed[0] = []
        elif kind == 'foreign-hit':
            changed[0].append(dict(url='foreign.html', score=1))
        else:
            changed[0].append(dict(changed[0][0]))
        try:
            check_results(changed, urls, anchors)
        except AssertionError:
            pass
        else:
            raise AssertionError('query control accepted ' + kind)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--book', default='docs/book/book')
    args = parser.parse_args()
    relative = Path(args.book)
    assert not relative.is_absolute() and '..' not in relative.parts, 'repository-relative book required'
    output = PROFILE['directory'](ROOT, relative)
    _, files = BOOK['source_tree'](output)
    summary = ROOT / 'docs/book/src/SUMMARY.md'
    sources = (Path(__file__), PROFILE_SOURCE, PUBLICATION_SOURCE, summary, *files)
    PROFILE['plan_producer'](ROOT, sources=sources)
    frozen = {path: hashlib.sha256(path.read_bytes()).digest() for path in sources}
    engines, indexes = tuple(output.glob('elasticlunr-*.min.js')), tuple(output.glob('searchindex-*.js'))
    assert len(engines) == len(indexes) == 1, 'one native engine/index required'
    data = BOOK['search_data'](output)
    anchors = {}
    for chapter in BOOK['chapters'](summary.read_text()):
        path = Path(chapter).with_suffix('.html')
        page = BOOK['Page']()
        page.feed((output / path).read_text())
        anchors[path.as_posix()] = page.ids
    BOOK['search_contract'](data, BOOK['chapters'](summary.read_text()), anchors)
    node = shutil.which('node')
    assert node, 'read-only host Node22.8+ is required for native query verification'
    environment = PROFILE['environment_for'](ROOT, os.environ, create=False)
    environment = {key: value for key, value in environment.items() if not key.startswith('NODE_')}
    environment['NODE_DISABLE_COMPILE_CACHE'] = '1'
    payload = dict(engine=engines[0].read_text(), index=indexes[0].read_text(), queries=[q for q, _ in QUERIES])
    result = subprocess.run([node, '-e', JAVASCRIPT], cwd=ROOT, env=environment,
                            input=json.dumps(payload), capture_output=True, text=True, timeout=30)
    assert result.returncode == 0 and not result.stderr.strip(), (result.returncode, result.stderr)
    report = json.loads(result.stdout)
    version = tuple(map(int, report['node'].lstrip('v').split('.')))
    assert version >= (22, 8, 0), 'Node22.8+ is required for explicit cache disabling'
    check_results(report['results'], set(data['doc_urls']), anchors)
    controls(report['results'], set(data['doc_urls']), anchors)
    assert frozen == {path: hashlib.sha256(path.read_bytes()).digest() for path in sources}, 'query inputs changed'
    print('native search queries:%d cases/%d chapter landings/4 data refusals;Node%s;source bytes unchanged;rc=0' % (
        len(QUERIES), len(anchors), report['node']))


if __name__ == '__main__':
    main()
