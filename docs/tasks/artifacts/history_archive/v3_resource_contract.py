"""Eight full decoded generations and isolated actual-CLI RSS; native tar verifies fixture bytes."""
import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import platform
import re
import runpy
import stat
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[4]
HERE = Path(__file__).parent
READER = ROOT / 'scripts/history_archive.py'
GENERATIONS = ROOT / 'scripts/history_generations.py'
PROFILE = ROOT / 'scripts/local_environment.py'
FIXTURE = HERE / 'v3_reader_contract.py'
RSS_LIMIT = 128 * 1024 * 1024
MASTER = '.doctrine/history_archive/windows.json'


def rss_bytes(value, system):
    if type(value) is not int or value <= 0:
        raise ValueError('positive integer RSS required')
    if system not in ('darwin', 'linux'):
        raise ValueError('RSS units unavailable for this platform')
    return value * (1024 if system == 'linux' else 1)


def admit_rss(value):
    if type(value) is not int or not 0 < value <= RSS_LIMIT:
        raise ValueError('128MiB isolated CLI regression budget exceeded')


def controls():
    assert rss_bytes(1024, 'darwin') == 1024 and rss_bytes(1024, 'linux') == 1048576
    admit_rss(RSS_LIMIT)
    checked = 3
    for function, args in ((admit_rss, (RSS_LIMIT + 1,)), (admit_rss, (True,)),
                            (rss_bytes, (1024, 'unknown')), (rss_bytes, (0, 'darwin')),
                            (rss_bytes, (True, 'linux'))):
        try:
            function(*args)
        except ValueError:
            checked += 1
        else:
            raise AssertionError('invalid resource evidence admitted')
    print('resource evidence: %d unit/inclusive/+1/refusal controls; rc=0' % checked)


def physical_plus_one(api, profile, root, sources):
    """Falsify the inclusive457 physical-file boundary using one exclusively owned empty file."""
    extra = root / 'docs/history/stitchcad-devnotes-part9900000.md'
    plan = dict(sources=sources, output_files=(extra,))
    profile['plan_producer'](ROOT, **plan)
    with extra.open('xb') as stream:
        owner = os.fstat(stream.fileno())
    try:
        try:
            api['inventory'](root)
        except api['Refusal'] as error:
            assert str(error) == 'archive physical count bound'
        else:
            raise AssertionError('458 physical files admitted')
    finally:
        profile['plan_producer'](ROOT, **plan)
        current = extra.lstat()
        assert stat.S_ISREG(current.st_mode) and current.st_nlink == 1 and current.st_size == 0
        assert (current.st_dev, current.st_ino) == (owner.st_dev, owner.st_ino), 'physical refusal fixture replaced'
        extra.unlink()
    print('physical topology:457 files accepted/458 refused before decode; exact owned extra removed; rc=0')


def observe(relative, *, unit_oracle=False):
    """Fresh observer: exactly one waited child, no inherited fixture-building allocations."""
    if sys.platform not in ('darwin', 'linux'):
        raise ValueError('RSS units unavailable for this platform')
    import resource
    api = runpy.run_path(str(READER))
    profile = runpy.run_path(str(PROFILE))
    root = api['archive_root'](ROOT / relative)
    sources = (Path(__file__), *api['backing_sources'](root, profile))
    profile['plan_producer'](ROOT, sources=sources)
    before = resource.getrusage(resource.RUSAGE_CHILDREN)
    assert before.ru_maxrss == 0 and before.ru_utime == before.ru_stime == 0, 'observer already waited for children'
    if unit_oracle:
        command = ['/usr/bin/time', *(['-l'] if sys.platform == 'darwin' else ['-f', 'RSS_RAW=%M']),
                   sys.executable, '-I', '-B', '-c', 'pass']
    else:
        command = [sys.executable, '-I', '-B', str(READER), '--root', str(root), 'verify']
    started = time.perf_counter()
    result = subprocess.run(command, cwd=ROOT, capture_output=True, text=True, timeout=30)
    elapsed = time.perf_counter() - started
    assert result.returncode == 0, result.stderr
    raw = resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss
    normalized = rss_bytes(raw, sys.platform)
    if unit_oracle:
        match = re.search(r'([0-9]+)\s+maximum resident set size' if sys.platform == 'darwin'
                          else r'RSS_RAW=([0-9]+)', result.stderr)
        assert match and int(match[1]) == raw, 'native time/getrusage RSS disagree'
    else:
        assert not result.stderr
        admit_rss(normalized)
    assert math.isfinite(elapsed) and 0 < elapsed <= 30
    print(json.dumps(dict(platform=sys.platform, machine=platform.machine(), python=platform.python_version(),
                          rss_raw=raw, rss_unit='bytes' if sys.platform == 'darwin' else 'KiB',
                          rss_bytes=normalized, elapsed_seconds=elapsed,
                          cli_output=result.stdout.strip(), unit_oracle=unit_oracle), sort_keys=True))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--observe', help='internal isolated observer: repository-relative target fixture')
    parser.add_argument('--unit-oracle', action='store_true', help='native time/getrusage unit check; OS visibility required')
    args = parser.parse_args()
    if args.observe:
        assert args.observe.startswith('target/') and '..' not in Path(args.observe).parts
        observe(args.observe, unit_oracle=args.unit_oracle)
        return
    if args.unit_oracle:
        raise ValueError('--unit-oracle requires --observe')
    controls()
    api = runpy.run_path(str(READER))
    engine = api['generation_reader']()
    fixture = runpy.run_path(str(FIXTURE))
    profile = runpy.run_path(str(PROFILE))
    sources = (Path(__file__), FIXTURE, *api['backing_sources'](ROOT, profile))
    profile['plan_producer'](ROOT, sources=sources)
    revision = api['_head_revision'](ROOT)
    frozen = {path: path.read_bytes() for path in sources}
    current = {path.relative_to(ROOT).as_posix(): path.read_bytes() for path in sources
               if path.is_relative_to(ROOT / 'docs/history') or path.is_relative_to(ROOT / '.doctrine/history_archive')}
    first = json.loads(current[MASTER])['generations'][0]
    group = json.loads(current[first['path']])
    retained = {first['path'], *(row['path'] for row in group['legacy_catalogs'])}
    for name in group['windows']:
        value = json.loads(current[name]); retained.update((name, value['payload'], value['catalog']))
    files = {name: current[name] for name in retained}
    files[MASTER] = fixture['document'](dict(version=3, generations=[first]))
    first_measure = engine['decode_group'](ROOT, api, (first, group, current[first['path']]), set(), set())[1]
    for index in range(1, 9):
        control = '.doctrine/history_archive/generation%d.json' % index
        if index != 1:
            files[control] = fixture['document'](dict(version=2, windows=[], legacy_catalogs=[]))
            fixture['refresh'](files, index)
        # Fill every window slot as well as decoded bytes. Each extra window has a unique
        # empty logical record, retaining genuine packets and controls rather than counters.
        occupied = len(json.loads(files[control])['windows'])
        padding_catalog_bytes = 0
        for slot in range(occupied + 1, 16):
            empty = {'docs/history/stitchcad-devnotes-part%d.md' % (index * 1000000 + 100 + slot): b''}
            number = 30000 + index * 100 + slot
            _path, additions = fixture['window'](number, empty, '0' * 40)
            padding_catalog_bytes += len(additions['docs/history/window%d.md' % number])
            fixture['add_window'](files, index, number, empty, '0' * 40)
        count = 1 if index == 1 else 25
        values = {'docs/history/stitchcad-devnotes-part%d.md' % (index * 1000000 + i): b'' for i in range(count)}
        number = 20000 + index
        _path, skeleton = fixture['window'](number, values, '0' * 40)
        amount = (4_000_000 - (first_measure[2] if index == 1 else 0) - padding_catalog_bytes
                  - len(skeleton['docs/history/window%d.md' % number]))
        assert 0 <= amount <= count * 160000
        values = {name: fixture['bounded_text'](min(160000, max(0, amount - i * 160000)))
                  for i, name in enumerate(values)}
        fixture['add_window'](files, index, number, values, '0' * 40)
    raw = {'docs/history/stitchcad-devnotes-part%d.md' % (9000000 + index): b'' for index in range(48)}
    files.update(raw)
    assert len(files) == 457
    work = ROOT / 'target/scratch/history_v3_resources' / str(os.getpid())
    root = work / 'full'
    outputs = tuple(root / name for name in files)
    directories = {work, root}
    for path in outputs:
        directories.update(parent for parent in path.parents if parent.is_relative_to(work))
    plan = dict(sources=sources, directories=tuple(sorted(directories)), output_files=outputs)
    profile['enter_producer'](ROOT, **plan)
    for name, data in files.items():
        profile['plan_producer'](ROOT, **plan)
        with (root / name).open('xb') as stream:
            stream.write(data)
    # Native tar independently emits the actual complete first-level bytes, including original
    # opaque evidence. Catalogs are decoded data too; index/control bytes are resident only.
    expected_records, expected_backings = dict(raw), {}
    group_totals = []
    for row in json.loads(files[MASTER])['generations']:
        group = json.loads(files[row['path']])
        total = sum(len(files[item['path']]) for item in group['legacy_catalogs'])
        for name in group['windows']:
            manifest = json.loads(files[name])
            inputs = (*sources, *(root / name for name in files))
            profile['plan_producer'](ROOT, sources=inputs)
            assert subprocess.check_output(['tar', '-tzf', str(root / manifest['payload'])]).decode().splitlines() == [item['path'] for item in manifest['members']]
            profile['plan_producer'](ROOT, sources=inputs)
            content = subprocess.check_output(['tar', '-xOzf', str(root / manifest['payload'])])
            assert len(content) == sum(item['bytes'] for item in manifest['members'])
            offset = 0
            for item in manifest['members']:
                data = content[offset:offset + item['bytes']]; offset += item['bytes']
                assert hashlib.sha256(data).hexdigest() == item['sha256']
                (expected_records if item['kind'] == 'record' else expected_backings)[item['path']] = data
            total += len(content) + len(files[manifest['catalog']])
        assert total == 4_000_000
        group_totals.append(total)
    actual, pressure = api['inventory'](root)
    assert actual == expected_records and api['backing_inventory'](root) == expected_backings
    assert pressure[2] == sum(group_totals) == 32_000_000
    assert pressure[0] == 192
    inputs = (*sources, *(root / name for name in files))
    profile['plan_producer'](ROOT, sources=inputs)
    result = subprocess.run([sys.executable, '-I', '-B', str(Path(__file__)), '--observe', root.relative_to(ROOT).as_posix()],
                            cwd=ROOT, capture_output=True, text=True, check=True, timeout=40)
    measurement = json.loads(result.stdout)
    assert not measurement['unit_oracle'] and '32000000 decoded bytes' in measurement['cli_output']
    physical_plus_one(api, profile, root, inputs)
    report = dict(source_revision=revision, generations=8, windows=128, physical_files=len(files),
                  independent_decoded_bytes=group_totals, logical_records=len(actual),
                  original_backings=len(expected_backings), pressure=pressure, measurement=measurement,
                  reader_sha256=hashlib.sha256(READER.read_bytes()).hexdigest(),
                  generation_reader_sha256=hashlib.sha256(GENERATIONS.read_bytes()).hexdigest())
    path = work / 'measurement.json'
    profile['plan_producer'](ROOT, sources=inputs, directories=(work,), output_files=(path,))
    with path.open('xb') as stream:
        stream.write((json.dumps(report, sort_keys=True, indent=2) + '\n').encode())
    assert all(path.read_bytes() == data for path, data in frozen.items()), 'resource fixture changed source inputs'
    assert api['_head_revision'](ROOT) == revision, 'resource source HEAD changed'
    print(json.dumps(report, sort_keys=True))
    print('full generation resources: native tar bytes exact;8 independent4000000B groups; isolated public CLI within128MiB; rc=0')
    print('owned fixture/report: ' + work.relative_to(ROOT).as_posix())


if __name__ == '__main__':
    main()
