"""Compare same-source native mdBook search snapshots without executing or replacing search code.

The diagnostic renderings and source hashes stay repository-local. This read-only verifier checks
complete stored words and actual indexed terms per chapter across different result groupings.
"""
from pathlib import Path
import argparse
import collections
import hashlib
import json
import re
import runpy

ROOT = Path(__file__).resolve().parents[4]
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PUBLICATION_SOURCE = Path(__file__).with_name('book_publication.py')
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
PROFILE['plan_producer'](ROOT, sources=(Path(__file__), PROFILE_SOURCE, PUBLICATION_SOURCE))
BOOK = runpy.run_path(str(PUBLICATION_SOURCE))


def census(data):
    paths = {key: data['doc_urls'][int(key)].split('#')[0]
             for key in data['index']['documentStore']['docs']}
    words, terms = collections.defaultdict(set), collections.defaultdict(set)
    for key, record in data['index']['documentStore']['docs'].items():
        for field in ('body', 'title', 'breadcrumbs'):
            words[paths[key]].update(re.findall(r'\w+', record[field].casefold()))
    for field in data['index']['fields']:
        pending = [('', data['index']['index'][field]['root'])]
        while pending:
            prefix, node = pending.pop()
            for key in node.get('docs', {}):
                terms[paths[key]].add(prefix)
            pending.extend((prefix + char, child) for char, child in node.items()
                           if char not in ('df', 'docs'))
    return dict(words), dict(terms)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ('baseline', 'candidate', 'sources'):
        parser.add_argument('--' + name, required=True)
    args = parser.parse_args()
    paths = {}
    for name in ('baseline', 'candidate', 'sources'):
        relative = Path(getattr(args, name))
        assert not relative.is_absolute() and '..' not in relative.parts and relative.parts[:1] == ('target',), 'repository-local diagnostic path required'
        paths[name] = ROOT / relative
        PROFILE['producer_path'](ROOT, paths[name])
    BOOK['main_inputs'](ROOT)
    _, inputs = BOOK['build_inputs'](ROOT)
    snapshots = []
    for name in ('baseline', 'candidate'):
        PROFILE['directory'](ROOT, PROFILE['producer_path'](ROOT, paths[name]))
        files = tuple(paths[name].glob('searchindex-*.js'))
        assert len(files) == 1, 'one actual native index required'
        snapshots.extend(files)
    PROFILE['plan_producer'](ROOT, sources=(Path(__file__), PUBLICATION_SOURCE, *inputs,
                                           paths['sources'], *snapshots))
    hashes = json.loads(paths['sources'].read_text())
    expected = {str(path.relative_to(ROOT)): hashlib.sha256(path.read_bytes()).hexdigest() for path in inputs}
    assert hashes == expected, 'native snapshots must name the exact current complete build input hashes'
    a, b = (BOOK['search_data'](paths[name]) for name in ('baseline', 'candidate'))
    assert len(a['doc_urls']) > len(b['doc_urls']), 'baseline must have the actual larger section population'
    aw, at = census(a)
    bw, bt = census(b)
    assert aw == bw, 'searchable stored chapter words lost/added'
    assert at == bt, 'actual indexed chapter terms lost/added'
    registered = BOOK['chapters']((ROOT / BOOK['SOURCE'] / 'SUMMARY.md').read_text())
    BOOK['search_contract'](b, registered)
    expected_chapters = {Path(chapter).with_suffix('.html').as_posix() for chapter in registered}
    assert set(bw) == set(bt) == expected_chapters, 'complete chapter coverage required'
    print('search migration: %d→%d documents / %d chapters / %d chapter-word pairs / %d indexed-term pairs exact; sources unchanged; rc=0' % (
        len(a['doc_urls']), len(b['doc_urls']), len(bw), sum(map(len, bw.values())), sum(map(len, bt.values()))))


if __name__ == '__main__':
    main()
