"""Physical generation mutations and inclusive budgets; all effects stay in declared target fixtures."""
import argparse
import copy
import gzip
import hashlib
import io
import json
import os
from pathlib import Path
import runpy
import subprocess
import sys
import tarfile

ROOT = Path(__file__).resolve().parents[4]
READER = ROOT / 'scripts/history_archive.py'
GENERATIONS = ROOT / 'scripts/history_generations.py'
PROFILE = ROOT / 'scripts/local_environment.py'
MASTER = '.doctrine/history_archive/windows.json'


def document(value):
    return (json.dumps(value, indent=2) + '\n').encode()


def identity(path, data):
    return dict(path=path, lines=data.count(b'\n'), bytes=len(data), sha256=hashlib.sha256(data).hexdigest())


def tree_hash(files, control):
    """Independent canonical stream construction from the format specification."""
    group = json.loads(files[control])
    paths = [control, *(row['path'] for row in group['legacy_catalogs'])]
    for name in group['windows']:
        value = json.loads(files[name])
        paths.extend((name, value['payload'], value['catalog']))
    assert len(paths) == len(set(paths))
    body = b'StitchCAD-history-generation-tree-v1\0' + b''.join(
        ('%s\0%d\0%s\n' % (path, len(files[path]), hashlib.sha256(files[path]).hexdigest())).encode()
        for path in sorted(paths))
    return hashlib.sha256(body).hexdigest(), body


def refresh(files, index):
    root = json.loads(files[MASTER])
    path = '.doctrine/history_archive/generation%d.json' % index
    row = identity(path, files[path])
    row['tree_sha256'] = tree_hash(files, path)[0]
    if index > len(root['generations']):
        assert index == len(root['generations']) + 1
        root['generations'].append(row)
    else:
        root['generations'][index - 1] = row
    files[MASTER] = document(root)


def window(number, values, revision):
    path = '.doctrine/history_archive/window%d.json' % number
    catalog = 'docs/history/window%d.md' % number
    text = '# Native generation boundary\n\n' + ''.join('### %s\n\n' % Path(name).name for name in values)
    output = io.BytesIO()
    with tarfile.open(fileobj=output, mode='w', format=tarfile.USTAR_FORMAT) as archive:
        for name, data in values.items():
            entry = tarfile.TarInfo(name)
            entry.size, entry.mode, entry.mtime = len(data), 0o444, 0
            archive.addfile(entry, io.BytesIO(data))
    packed = gzip.compress(output.getvalue(), mtime=0)
    digest = hashlib.sha256(packed).hexdigest()
    payload = 'docs/history/payloads/' + digest + '.tar.gz'
    manifest = dict(version=2, id='window%d' % number, source_revision=revision, created='2026-10-27',
                    owner='G1-SLICE.6.4.5.capacity.apply.cutover', reason='Owned physical calibration fixture.',
                    payload=payload, payload_sha256=digest, catalog=catalog,
                    members=[dict(identity(name, data), kind='record') for name, data in values.items()])
    return path, {path: document(manifest), catalog: text.encode(), payload: packed}


def add_window(files, index, number, values, revision):
    path, additions = window(number, values, revision)
    assert not set(additions).intersection(files)
    files.update(additions)
    control = '.doctrine/history_archive/generation%d.json' % index
    group = json.loads(files[control])
    group['windows'].append(path)
    files[control] = document(group)
    refresh(files, index)


def bounded_text(size):
    return (b'x' * 399 + b'\n') * (size // 400) + b'x' * (size % 400)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('source', nargs='?', default='.')
    args = parser.parse_args()
    assert args.source == '.' or (args.source.startswith('target/') and '..' not in Path(args.source).parts)
    source = ROOT / args.source
    work = ROOT / 'target/scratch/history_v3_reader' / str(os.getpid())
    profile = runpy.run_path(str(PROFILE))
    profile['plan_producer'](ROOT, sources=(Path(__file__), READER, GENERATIONS, PROFILE), directories=(work,))
    api = dict(__file__=str(READER), __name__='generation_reader_contract_input')
    exec(compile(READER.read_bytes(), str(READER), 'exec'), api)
    inputs = api['backing_sources'](source, profile)
    sources = (Path(__file__), *inputs)
    profile['plan_producer'](ROOT, sources=sources, directories=(work,))
    assert not work.exists()
    files = {path.relative_to(source).as_posix(): path.read_bytes() for path in inputs
             if path.is_relative_to(source / 'docs/history') or path.is_relative_to(source / '.doctrine/history_archive')}
    frozen = {path: path.read_bytes() for path in sources}
    root = json.loads(files[MASTER])
    assert root['version'] == 3, 'generation source required'
    revision = api['_head_revision'](ROOT)
    # Independent OS tar constructs the entire expected record/backing population.
    records, backings = {}, {}
    for row in root['generations']:
        checksum, body = tree_hash(files, row['path'])
        profile['plan_producer'](ROOT, sources=sources)
        oracle = subprocess.check_output(['shasum', '-a', '256'], input=body, cwd=ROOT).split()[0].decode()
        assert checksum == row['tree_sha256'] == oracle
        for name in json.loads(files[row['path']])['windows']:
            manifest = json.loads(files[name])
            profile['plan_producer'](ROOT, sources=sources)
            assert subprocess.check_output(['tar', '-tzf', str(source / manifest['payload'])]).decode().splitlines() == [v['path'] for v in manifest['members']]
            profile['plan_producer'](ROOT, sources=sources)
            content = subprocess.check_output(['tar', '-xOzf', str(source / manifest['payload'])])
            offset = 0
            for item in manifest['members']:
                data = content[offset:offset + item['bytes']]
                assert hashlib.sha256(data).hexdigest() == item['sha256']
                (records if item['kind'] == 'record' else backings)[item['path']] = data
                offset += len(data)
            assert offset == len(content)
    records.update({name: data for name, data in files.items() if api['SEGMENT'].fullmatch(name)})
    current_files, current_records = dict(files), dict(records)
    # Freeze the original packed cohort, reconstructing its three original raw records through the
    # public logical inventory. Future capture/generation growth must not invalidate this fixture.
    first_control = root['generations'][0]['path']
    original = json.loads(files[first_control])
    retained = {first_control, *(item['path'] for item in original['legacy_catalogs'])}
    baseline_names = set()
    for name in original['windows']:
        value = json.loads(files[name])
        retained.update((name, value['payload'], value['catalog']))
        baseline_names.update(item['path'] for item in value['members'] if item['kind'] == 'record')
    files = {name: current_files[name] for name in retained}
    active = '.doctrine/history_archive/generation2.json'
    files[active] = document(dict(version=2, windows=[], legacy_catalogs=[]))
    rows = []
    for name in (first_control, active):
        row = identity(name, files[name]); row['tree_sha256'] = tree_hash(files, name)[0]
        rows.append(row)
    root = dict(version=3, generations=rows); files[MASTER] = document(root)
    for number in (538, 539, 540):
        name = 'docs/history/stitchcad-devnotes-part%d.md' % number
        files[name] = current_records[name]
        baseline_names.add(name)
    records = {name: current_records[name] for name in baseline_names}
    raw_bytes = sum(len(data) for name, data in files.items() if api['SEGMENT'].fullmatch(name))
    raw_lines = sum(data.count(b'\n') for name, data in files.items() if api['SEGMENT'].fullmatch(name))
    cases = []

    def case(label, changed, expected=None, extra=None, retention=False):
        cases.append((label, dict(changed), expected, {**records, **(extra or {})}, retention))

    def change_root(update):
        changed = dict(files)
        value = copy.deepcopy(root)
        update(value)
        changed[MASTER] = document(value)
        return changed

    def change_group(index, update):
        changed = dict(files)
        path = '.doctrine/history_archive/generation%d.json' % index
        value = json.loads(changed[path])
        update(value)
        changed[path] = document(value)
        try:
            refresh(changed, index)
        except AssertionError:
            # An intentionally duplicate window still needs a correctly pinned control body.
            value = json.loads(changed[MASTER])
            value['generations'][index - 1].update(identity(path, changed[path]))
            changed[MASTER] = document(value)
        return changed

    case('native original bytes', dict(files))
    case('current complete retention', current_files, extra=current_records, retention=True)
    legacy = {**backings, **{item['path']: files[item['path']] for item in original['legacy_catalogs']}}
    case('schema downgrade refused', legacy, 'unsupported committed archive schema', retention=True)
    case('unknown index key', change_root(lambda v: v.update(extra=True)), 'unknown/missing schema fields')
    case('bool index version', change_root(lambda v: v.update(version=True)), 'unknown/missing schema fields')
    case('one generation', change_root(lambda v: v.update(generations=v['generations'][:1])), 'generation count bound')
    case('nine generations', change_root(lambda v: v.update(generations=v['generations'] * 5)), 'generation count bound')
    case('generation order', change_root(lambda v: v['generations'].reverse()), 'generation path/order mismatch')
    case('generation digest', change_root(lambda v: v['generations'][0].update(sha256='0' * 64)), 'generation control identity mismatch')
    case('tree digest', change_root(lambda v: v['generations'][0].update(tree_sha256='0' * 64)), 'generation tree identity mismatch')
    case('index plus one', {**files, MASTER: files[MASTER] + b' ' * (8193 - len(files[MASTER]))}, 'byte bound exceeded')
    case('unknown group key', change_group(2, lambda v: v.update(extra=True)), 'unknown/missing schema fields')
    first_windows = json.loads(files[root['generations'][0]['path']])['windows']
    case('duplicate group window', change_group(2, lambda v: v.update(windows=[first_windows[0]] * 2)), 'duplicate group window')
    case('shared group window', change_group(2, lambda v: v.update(windows=[first_windows[0]])), 'window belongs to multiple generations')
    case('seventeen group windows', change_group(2, lambda v: v.update(windows=[first_windows[0]] * 17)), 'group window count bound')
    case('later legacy catalog', change_group(2, lambda v: v.update(legacy_catalogs=[root['generations'][0]])), 'group legacy catalog count')
    selected = first_windows[-1]
    manifest = json.loads(files[selected])
    for label, update, error in (
        ('unknown kind', lambda v: v['members'][0].update(kind='unknown'), 'invalid member kind'),
        ('escape path', lambda v: v['members'][0].update(path='../escape.md'), 'invalid logical path'),
        ('bool size', lambda v: v['members'][0].update(bytes=True), 'invalid integer/bound'),
        ('wrong record digest', lambda v: v['members'][0].update(sha256='0' * 64), 'member identity mismatch'),
        ('duplicate member', lambda v: v['members'].append(v['members'][0]), 'duplicate logical identity'),
    ):
        value = copy.deepcopy(manifest)
        update(value)
        case(label, {**files, selected: document(value)}, error)
    case('orphan control', {**files, '.doctrine/history_archive/orphan.json': b'{}'}, 'orphan/missing generation backing')
    old_record = manifest['members'][0]['path']
    case('raw packed collision', {**files, old_record: records[old_record]}, 'raw/packed duplicate logical identity')
    changed = dict(files)
    damaged = bytearray(changed[manifest['payload']]); damaged[-1] ^= 1
    changed[manifest['payload']] = bytes(damaged)
    case('physical payload digest', changed, 'payload digest mismatch')
    # A valid metadata edit cannot preserve a sealed physical-tree identity.
    value = copy.deepcopy(manifest); value['owner'] = 'uncommitted-owner'
    changed = {**files, selected: document(value)}
    case('sealed metadata drift', changed, 'generation tree identity mismatch')
    changed = {**current_files, selected: document(value)}
    refresh(changed, 1)
    case('rehashed committed drift', changed, ('committed manifest changed', 'committed generation seal changed'),
         extra=current_records, retention=True)
    # Payload expansion is bounded before tar parsing; this is a physical gzip file.
    changed = dict(files); changed.pop(manifest['payload'])
    packed = gzip.compress(b'\0' * (api['TAR_BYTES'] + 1), mtime=0)
    value = copy.deepcopy(manifest); sha = hashlib.sha256(packed).hexdigest()
    value.update(payload='docs/history/payloads/' + sha + '.tar.gz', payload_sha256=sha)
    changed[selected], changed[value['payload']] = document(value), packed
    case('physical decompression limit', changed, 'decompressed tar bound exceeded')
    for fault, error in (('link', 'unexpected/unsafe tar member'),
                         ('escape', 'unexpected/unsafe tar member'),
                         ('duplicate', 'duplicate tar member'),
                         ('missing', 'tar membership mismatch'),
                         ('tail', 'unparsed nonzero tar tail')):
        output = io.BytesIO()
        with tarfile.open(fileobj=output, mode='w', format=tarfile.USTAR_FORMAT) as archive:
            for ordinal, item in enumerate(manifest['members']):
                if fault == 'missing' and ordinal == 0:
                    continue
                data = records[item['path']]
                entry = tarfile.TarInfo('../escape' if fault == 'escape' and ordinal == 0 else item['path'])
                entry.size = len(data)
                if fault == 'link' and ordinal == 0:
                    entry.type, entry.linkname, entry.size = tarfile.SYMTYPE, '../escape', 0
                    archive.addfile(entry)
                else:
                    archive.addfile(entry, io.BytesIO(data))
                    if fault == 'duplicate' and ordinal == 0:
                        archive.addfile(entry, io.BytesIO(data))
        raw = output.getvalue() + (b'nonzero-tail' if fault == 'tail' else b'')
        packed = gzip.compress(raw, mtime=0); sha = hashlib.sha256(packed).hexdigest()
        changed = dict(files); changed.pop(manifest['payload'])
        value = dict(manifest, payload='docs/history/payloads/' + sha + '.tar.gz', payload_sha256=sha)
        changed[selected], changed[value['payload']] = document(value), packed
        case('physical tar ' + fault, changed, error)
    for axis in ('decoded', 'resident', 'lines'):
        names = ['docs/history/stitchcad-devnotes-part%d.md' % (50000 + i) for i in range(30 if axis == 'lines' else 25)]
        if axis == 'resident':
            amount = 4000000 - raw_bytes - len(files[root['generations'][1]['path']])
            values = {name: bounded_text(min(160000, max(0, amount - i * 160000))) for i, name in enumerate(names)}
            changed = {**files, **values}
        else:
            # Catalog length depends only on the names, before any body-size choice.
            _, skeleton = window(9000, dict.fromkeys(names, b''), revision)
            cat = skeleton['docs/history/window9000.md']
            amount = (60000 - raw_lines - cat.count(b'\n')) if axis == 'lines' else (4000000 - raw_bytes - len(cat))
            bound = 2000 if axis == 'lines' else 160000
            values = {name: ((b'\n' * min(bound, max(0, amount - i * bound))) if axis == 'lines'
                             else bounded_text(min(bound, max(0, amount - i * bound)))) for i, name in enumerate(names)}
            changed = dict(files); add_window(changed, 2, 9000, values, revision)
        case(axis + ' inclusive', changed, extra=values)
        exceeded = dict(values); exceeded[names[-1]] += b'\n' if axis == 'lines' else b'x'
        if axis == 'resident':
            changed = {**files, **exceeded}
        else:
            changed = dict(files); add_window(changed, 2, 9000, exceeded, revision)
        case(axis + ' plus one', changed, 'generation decoded bound exceeded' if axis != 'resident' else 'generation resident bound exceeded')
    additions = {'docs/history/stitchcad-devnotes-part%d.md' % (60000 + i): b'' for i in range(61)}
    case('working64 inclusive', {**files, **additions}, extra=additions)
    case('working65 refused', {**files, **additions, 'docs/history/stitchcad-devnotes-part60999.md': b''}, 'group working Markdown bound exceeded')
    # Global fanout is real storage, not repeated descriptors or simulated counters.
    changed = dict(files); extra = {}
    for index in range(2, 9):
        control = '.doctrine/history_archive/generation%d.json' % index
        if control not in changed:
            changed[control] = document(dict(version=2, windows=[], legacy_catalogs=[]))
            refresh(changed, index)
        if index < 8:
            values = {'docs/history/stitchcad-devnotes-part%d.md' % (70000 + index): b''}
            add_window(changed, index, 9100 + index, values, revision); extra.update(values)
    case('eight real groups', changed, extra=extra)
    ninth = dict(changed); ninth['.doctrine/history_archive/generation9.json'] = document(dict(version=2, windows=[], legacy_catalogs=[]))
    refresh(ninth, 9)
    case('ninth real group refused', ninth, 'generation count bound')
    # Group aggregate controls can reach their own bound without raising a per-control ceiling.
    control_path = root['generations'][0]['path']
    group = json.loads(files[control_path])
    controls = [control_path, *group['windows']]
    current_bytes = sum(len(files[name]) for name in controls)
    padding = 262144 - current_bytes
    assert 0 < padding and len(files[selected]) + padding + 1 <= 65536
    pad = (b' ' * 399 + b'\n') * (padding // 400) + b' ' * (padding % 400)
    changed = dict(files); changed[selected] += pad; refresh(changed, 1)
    case('control262144 inclusive', changed)
    changed = dict(changed); changed[selected] += b' '; refresh(changed, 1)
    case('control262145 refused', changed, 'group control bound exceeded')
    # Exact aggregate line boundaries, while every individual control remains within2200 lines.
    changed = dict(files); extra = {}
    for index in range(6):
        values = {'docs/history/stitchcad-devnotes-part%d.md' % (80000 + index): b''}
        add_window(changed, 1, 9200 + index, values, revision); extra.update(values)
    controls = [control_path, *json.loads(changed[control_path])['windows']]
    missing = 20000 - sum(changed[name].count(b'\n') for name in controls)
    assert missing > 0
    final = None
    for name in controls:
        amount = min(missing, 2200 - changed[name].count(b'\n'))
        changed[name] += b'\n' * amount
        missing -= amount
        final = name
        if missing == 0:
            break
    assert missing == 0 and changed[final].count(b'\n') < 2200
    refresh(changed, 1)
    case('control20000 lines inclusive', changed, extra=extra)
    changed = dict(changed); changed[final] += b'\n'; refresh(changed, 1)
    case('control20001 lines refused', changed, 'group control bound exceeded')

    count = 0
    for label, content, error, expected, retention in cases:
        fixture = work / ('case%02d' % count)
        outputs = [fixture / name for name in content]
        directories = {work, fixture}
        for path in outputs:
            directories.update(parent for parent in path.parents if parent.is_relative_to(work))
        plan = dict(sources=sources, directories=tuple(sorted(directories)), output_files=tuple(outputs))
        profile['enter_producer'](ROOT, **plan)
        for name, data in content.items():
            profile['plan_producer'](ROOT, **plan)
            with (fixture / name).open('xb') as stream:
                stream.write(data)
        try:
            original_loader = api['load_profile']
            def forbidden_profile():
                raise AssertionError('read-only generation inventory loaded the producer profile')
            api['load_profile'] = forbidden_profile
            try:
                result, pressure = api['inventory'](fixture)
            finally:
                api['load_profile'] = original_loader
            if retention:
                api['guard_committed'](fixture)
        except api['Refusal'] as issue:
            accepted = (error,) if isinstance(error, str) else error
            assert accepted and any(text in str(issue) for text in accepted), (label, str(issue), error)
        else:
            assert error is None and result == expected, (label, 'unexpected success or bytes')
            if count == 0:
                assert api['backing_inventory'](fixture) == backings
                source_set = (*sources, *api['backing_sources'](fixture, profile))
                profile['plan_producer'](ROOT, sources=source_set)
                command = [sys.executable, '-I', '-B', str(READER), '--root', str(fixture)]
                environment = dict(os.environ, PATH=str(work / 'no-git'))
                assert subprocess.check_output([*command, 'list'], cwd=ROOT, env=environment).decode().splitlines() == sorted(records)
                profile['plan_producer'](ROOT, sources=source_set)
                name = sorted(records)[0]
                assert subprocess.check_output([*command, 'read', name], cwd=ROOT, env=environment) == records[name]
                review = fixture / 'target/decoded'
                review_outputs = [review / name for name in records]
                review_dirs = {review}
                for output in review_outputs:
                    review_dirs.update(parent for parent in output.parents if parent.is_relative_to(review))
                profile['plan_producer'](ROOT, sources=source_set, directories=tuple(sorted(review_dirs)), output_files=tuple(review_outputs))
                subprocess.run([*command, 'materialize', 'target/decoded'], cwd=ROOT, env=environment, check=True, capture_output=True)
                assert {path.relative_to(review).as_posix(): path.read_bytes() for path in review.rglob('*.md')} == records
        count += 1
        print('generation control %02d: %s OK' % (count, label), flush=True)
    assert all(path.read_bytes() == data for path, data in frozen.items()), 'original source drift'
    print('v3 reader: %d physical controls/0 failures; profile-free inventory, Git-free CLI, native tar/tree hashes and full materialization exact' % count)
    print('owned fixtures retained: ' + work.relative_to(ROOT).as_posix())


if __name__ == '__main__':
    main()
