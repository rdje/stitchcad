"""Focused flat-v2 reader contracts; copied fixtures and outputs stay under project target/."""
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
PROFILE = ROOT / 'scripts/local_environment.py'
MASTER = '.doctrine/history_archive/windows.json'


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('source', nargs='?', default='.', help='project root or target/ candidate')
    args = parser.parse_args()
    assert args.source == '.' or (args.source.startswith('target/') and '..' not in Path(args.source).parts)
    source = ROOT / args.source
    work = ROOT / 'target/scratch/history_v2_reader' / str(os.getpid())
    fixture = work / 'fixture'
    profile = runpy.run_path(str(PROFILE))
    profile['plan_producer'](ROOT, sources=(Path(__file__), READER, PROFILE), directories=(work,))
    ns = dict(__file__=str(READER), __name__='v2_reader_contract')
    exec(compile(READER.read_bytes(), str(READER), 'exec'), ns)
    inputs = ns['backing_sources'](source, profile)
    sources = (Path(__file__), *inputs)
    profile['plan_producer'](ROOT, sources=sources, directories=(work,))
    assert not work.exists(), 'fresh contract workspace required'
    master = json.loads((source / MASTER).read_bytes())
    assert master['version'] == 2, 'v2 candidate required'
    files = {path.relative_to(source).as_posix(): path.read_bytes()
             for path in inputs if path.is_relative_to(source / 'docs/history')
             or path.is_relative_to(source / '.doctrine/history_archive')}
    frozen = {path: path.read_bytes() for path in sources}
    manifests = {name: json.loads(files[name]) for name in master['windows']}
    records, backings = {}, {}
    for manifest in manifests.values():
        archive = source / manifest['payload']
        names = subprocess.check_output(['tar', '-tf', str(archive)], cwd=ROOT).decode().splitlines()
        assert names == [row['path'] for row in manifest['members']]
        combined = subprocess.check_output(['tar', '-xOf', str(archive)], cwd=ROOT)
        offset = 0
        for row in manifest['members']:
            data = combined[offset:offset + row['bytes']]
            assert hashlib.sha256(data).hexdigest() == row['sha256']
            (records if row['kind'] == 'record' else backings)[row['path']] = data
            offset += row['bytes']
        assert offset == len(combined)
    for name, data in files.items():
        if ns['SEGMENT'].fullmatch(name):
            assert name not in records
            records[name] = data
    legacy_ids = {row['path'] for name, data in backings.items()
                  if name != MASTER and name.endswith('.json')
                  for row in json.loads(data)['members']}
    legacy_id = sorted(legacy_ids)[0]
    selected = next(name for name, manifest in manifests.items()
                    if any(row['path'] == legacy_id for row in manifest['members']))
    selected_manifest = manifests[selected]
    collision = fixture / legacy_id
    extras = [collision, fixture / '.doctrine/history_archive/orphan.json', fixture / 'docs/history/payloads/orphan.bin']
    output_files = [fixture / name for name in files] + extras + [fixture / 'target/decoded' / name for name in records]
    directories = {work, fixture}
    for path in output_files:
        directories.update(parent for parent in path.parents if parent.is_relative_to(work))
    plan = dict(sources=sources, directories=tuple(sorted(directories)), output_files=tuple(output_files))
    count = 0
    original_read = ns['bounded_read']

    def check(label, replacements, expected):
        nonlocal count
        def virtual_read(path, maximum):
            key = path.relative_to(fixture).as_posix()
            if key in replacements:
                data = replacements[key]
                ns['require'](len(data) <= maximum, 'byte bound exceeded: ' + path.name)
                return data
            return original_read(path, maximum)
        temporary = {name: data for name, data in replacements.items()
                     if name.startswith('docs/history/payloads/') and name not in files}
        removed = {manifests[name]['payload']: files[manifests[name]['payload']]
                   for name in replacements if name in manifests
                   and json.loads(replacements[name])['payload'] != manifests[name]['payload']}
        profile['plan_producer'](ROOT, **plan)
        for name in removed:
            (fixture / name).unlink()
        for name, data in temporary.items():
            with (fixture / name).open('xb') as stream:
                stream.write(data)
        ns['bounded_read'] = virtual_read
        try:
            actual = ns['inventory'](fixture)
        except ns['Refusal'] as error:
            assert expected and expected in str(error), (label, str(error), expected)
        else:
            assert expected is None and actual[0] == records, (label, 'unexpected success/data')
        finally:
            ns['bounded_read'] = original_read
            profile['plan_producer'](ROOT, **plan)
            for name in temporary:
                (fixture / name).unlink()
            for name, data in removed.items():
                with (fixture / name).open('xb') as stream:
                    stream.write(data)
        count += 1

    def document(name, change):
        value = copy.deepcopy(master if name == MASTER else manifests[name])
        change(value)
        return {name: (json.dumps(value, indent=2) + '\n').encode()}

    def payload(change=None, raw_override=None):
        manifest = copy.deepcopy(selected_manifest)
        output = io.BytesIO()
        with tarfile.open(fileobj=output, mode='w', format=tarfile.USTAR_FORMAT) as archive:
            for row in manifest['members']:
                data = records[row['path']] if row['kind'] == 'record' else backings[row['path']]
                if change:
                    data = change(row, data)
                row.update(lines=data.count(b'\n'), bytes=len(data), sha256=hashlib.sha256(data).hexdigest())
                entry = tarfile.TarInfo(row['path'])
                entry.size = len(data)
                archive.addfile(entry, io.BytesIO(data))
        raw = output.getvalue() if raw_override is None else raw_override(output.getvalue())
        packed = gzip.compress(raw, mtime=0)
        checksum = hashlib.sha256(packed).hexdigest()
        manifest.update(payload='docs/history/payloads/' + checksum + '.tar.gz', payload_sha256=checksum)
        return {selected: (json.dumps(manifest, indent=2) + '\n').encode(), manifest['payload']: packed}

    cases = []
    cases.append(('all native tar bytes', {}, None))
    cases.append(('unknown master field', document(MASTER, lambda value: value.update(extra=True)), 'unknown/missing schema fields'))
    cases.append(('duplicate active window', document(MASTER, lambda value: value['windows'].append(value['windows'][0])), 'duplicate window'))
    cases.append(('active window bound', document(MASTER, lambda value: value.update(windows=value['windows'] * 5)), 'window count bound'))
    cases.append(('unknown kind', document(selected, lambda value: value['members'][0].update(kind='unknown')), 'invalid member kind'))
    index = next(i for i, row in enumerate(selected_manifest['members']) if row['path'] == legacy_id)
    cases.append(('record as backing', document(selected, lambda value: value['members'][index].update(kind='backing')), 'invalid backing path'))
    cases.append(('escape', document(selected, lambda value: value['members'][index].update(path='../escape.md')), 'invalid logical path'))
    cases.append(('bool integer', document(selected, lambda value: value['members'][index].update(bytes=True)), 'invalid integer/bound'))
    cases.append(('digest', document(selected, lambda value: value['members'][index].update(sha256='0' * 64)), 'member identity mismatch'))
    cases.append(('duplicate member', document(selected, lambda value: value['members'].append(value['members'][0])),
          'member count bound' if len(selected_manifest['members']) == 256 else 'duplicate logical identity'))
    catalog = selected_manifest['catalog']
    cases.append(('catalog membership', {catalog: files[catalog].replace(('### ' + Path(legacy_id).name).encode(), b'### absent.md', 1)}, 'catalog membership/order mismatch'))
    legacy_catalog = master['legacy_catalogs'][0]['path']
    cases.append(('legacy catalog pin', {legacy_catalog: files[legacy_catalog] + b'changed\n'}, 'legacy catalog identity mismatch'))
    cases.append(('record width', payload(lambda row, data: b'a' * 401 + b'\n' if row['path'] == legacy_id else data), 'logical maxline bound exceeded'))
    cases.append(('legacy record binding', payload(lambda row, data: b'changed\n' if row['path'] == legacy_id else data), 'legacy logical identity mismatch'))
    controls = {}
    for name in manifests:
        data = files[name]
        extra = 65536 - len(data)
        padding = (b'\n' + b' ' * 299) * (extra // 300) + b'\n' + b' ' * max(0, extra % 300 - 1)
        controls[name] = data + padding[:extra]
        assert len(controls[name]) == 65536
    cases.append(('control aggregate', controls, 'aggregate control bound exceeded'))
    cases.append(('tar tail', payload(raw_override=lambda raw: raw + b'nonzero tail'), 'unparsed nonzero tar tail'))
    cases.append(('decompression', payload(raw_override=lambda _raw: b'\0' * (ns['TAR_BYTES'] + 1)), 'decompressed tar bound exceeded'))
    chosen = {row['path'] for row in selected_manifest['members'] if row['kind'] == 'record'}
    chosen = set(sorted(chosen)[:26])
    cases.append(('decoded evidence budget', payload(lambda row, data: (b'a' * 399 + b'\n') * 400 if row['path'] in chosen else data), 'decoded history aggregate bound exceeded'))
    output_files.extend(fixture / name for _label, replacements, _expected in cases
                        for name in replacements if name.startswith('docs/history/payloads/') and name not in files)
    plan['output_files'] = tuple(output_files)
    # Validate all destinations, while leaving the public materializer's fresh root absent.
    creation_plan = dict(plan, directories=tuple(path for path in plan['directories']
                                                if not path.is_relative_to(fixture / 'target/decoded')))
    profile['enter_producer'](ROOT, **creation_plan)
    for name, data in files.items():
        profile['plan_producer'](ROOT, **plan)
        with (fixture / name).open('xb') as stream:
            stream.write(data)
    try:
        for label, replacements, expected in cases:
            check(label, replacements, expected)
        for path, content, expected in [(collision, records[legacy_id], 'raw/packed duplicate'),
                                       (extras[1], b'{}', 'orphan/missing archive control'),
                                       (extras[2], b'orphan', 'orphan/missing payload')]:
            profile['plan_producer'](ROOT, **plan)
            with path.open('xb') as stream:
                stream.write(content)
            try:
                check('physical ' + path.name, {}, expected)
            finally:
                profile['plan_producer'](ROOT, **plan)
                path.unlink()
        args = [sys.executable, '-I', '-B', str(READER), '--root', str(fixture)]
        assert subprocess.check_output([*args, 'list'], cwd=ROOT).decode().splitlines() == sorted(records)
        for name in [sorted(records)[0], sorted(records)[-1]]:
            assert subprocess.check_output([*args, 'read', name], cwd=ROOT) == records[name]
        materialization = subprocess.run([*args, 'materialize', 'target/decoded'], cwd=ROOT, capture_output=True)
        assert materialization.returncode == 0, (materialization.stdout, materialization.stderr)
        materialized = {path.relative_to(fixture / 'target/decoded').as_posix(): path.read_bytes()
                        for path in (fixture / 'target/decoded').rglob('*.md')}
        assert materialized == records
        check('restored', {}, None)
        assert all(path.read_bytes() == data for path, data in frozen.items()), 'source drift'
        print('v2 reader: %d calibrated cases/0 failures; native tar and public materialization %d records exact; sources unchanged' % (count, len(records)))
    finally:
        profile['plan_producer'](ROOT, **plan)
        present = list(work.rglob('*'))
        assert all(not path.is_symlink() and not (path.is_dir() and (path / '.git').exists()) for path in present)
        assert {path for path in present if path.is_file()} <= set(output_files), 'unowned cleanup entry'
        for path in present:
            if path.is_file():
                path.unlink()
        for path in sorted((path for path in present if path.is_dir()), key=lambda path: len(path.parts), reverse=True):
            path.rmdir()
        work.rmdir()


if __name__ == '__main__':
    main()
