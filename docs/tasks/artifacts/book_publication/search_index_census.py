"""Read the actual native search index and measure field coverage without executing JavaScript."""
import argparse
import hashlib
import json
from pathlib import Path
import runpy

ROOT = Path(__file__).resolve().parents[4]
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PUBLICATION_SOURCE = Path(__file__).with_name('book_publication.py')
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
PROFILE['plan_producer'](ROOT, sources=(Path(__file__), PROFILE_SOURCE, PUBLICATION_SOURCE))
BOOK = runpy.run_path(str(PUBLICATION_SOURCE))


def field_terms(index):
    """Keep document references: equal vocabularies alone do not prove equal chapter coverage."""
    pending = [('', index['root'])]
    pairs, nodes = set(), 0
    while pending:
        prefix, node = pending.pop()
        nodes += 1
        pairs.update((prefix, key) for key in node.get('docs', {}))
        pending.extend((prefix + char, child) for char, child in node.items()
                       if char not in ('df', 'docs'))
    return pairs, nodes


def census(data):
    size = lambda value: len(json.dumps(value, ensure_ascii=False, separators=(',', ':')).encode())
    store = data['index']['documentStore']['docs']
    fields, pairs = {}, {}
    for field in data['index']['fields']:
        native = data['index']['index'][field]
        pairs[field], nodes = field_terms(native)
        fields[field] = dict(index_bytes=size(native), nodes=nodes,
                             term_document_pairs=len(pairs[field]),
                             stored_text_bytes=sum(len(doc[field].encode()) for doc in store.values()))
    return dict(documents=len(data['doc_urls']), fields=fields,
                body_without_breadcrumbs=len(pairs['body'] - pairs['breadcrumbs']),
                breadcrumbs_without_body=len(pairs['breadcrumbs'] - pairs['body']))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--book', default='docs/book/book')
    args = parser.parse_args()
    relative = Path(args.book)
    assert not relative.is_absolute() and '..' not in relative.parts, 'repository-relative book required'
    output = PROFILE['directory'](ROOT, relative)
    files = tuple(output.glob('searchindex-*.js'))
    assert len(files) == 1, 'one actual native index required'
    PROFILE['plan_producer'](ROOT, sources=(Path(__file__), PROFILE_SOURCE, PUBLICATION_SOURCE, *files))
    content = files[0].read_bytes()
    result = census(BOOK['search_data'](output))
    result.update(index_path=files[0].relative_to(ROOT).as_posix(), script_bytes=len(content),
                  sha256=hashlib.sha256(content).hexdigest())
    print(json.dumps(result, sort_keys=True, indent=2))
    print('native search field census: read-only; rc=0')


if __name__ == '__main__':
    main()
