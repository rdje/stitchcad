"""Native bounded append/seal fixtures, exact current Git capture and independent tar byte oracles."""
import base64
import hashlib
import json
import os
from pathlib import Path
import runpy
import subprocess

ROOT = Path(__file__).resolve().parents[4]
HERE = Path(__file__).parent
READER = ROOT / 'scripts/history_archive.py'
PROFILE = ROOT / 'scripts/local_environment.py'
GENERATIONS = ROOT / 'scripts/history_generations.py'
PRODUCER = HERE / 'v2_migration.py'
FIXTURE = HERE / 'v3_reader_contract.py'
MASTER = '.doctrine/history_archive/windows.json'
ACTIVE = '.doctrine/history_archive/generation2.json'
OWNER = 'G1-SLICE.6.4.5.capacity.apply.accept.authoring'


def main():
    profile = runpy.run_path(str(PROFILE))
    api = dict(__file__=str(READER), __name__='authoring_reader')
    exec(compile(READER.read_bytes(), str(READER), 'exec'), api)
    producer = runpy.run_path(str(PRODUCER))
    fixture = runpy.run_path(str(FIXTURE))
    source_paths = api['backing_sources'](ROOT, profile)
    sources = (Path(__file__), PRODUCER, FIXTURE, *source_paths)
    work = ROOT / 'target/scratch/history_v3_authoring' / str(os.getpid())
    profile['plan_producer'](ROOT, sources=sources, directories=(work,))
    frozen = {path: path.read_bytes() for path in sources}
    assert not work.exists()
    revision = api['_head_revision'](ROOT)
    profile['plan_producer'](ROOT, sources=sources)
    names = subprocess.check_output(['git', '-C', str(ROOT), 'ls-tree', '-r', '--name-only', revision,
                                      '--', 'docs/history', '.doctrine/history_archive'],
                                     env=api['metadata_environment'](ROOT)).decode().splitlines()
    current = {name: api['_git_blob'](ROOT, revision, name, producer['source_bound'](name, api)) for name in names}
    engine = api['generation_reader']()
    cases = []
    refusal_count = 0

    def append(files):
        return producer['generation_append_files'](files, json.loads(files[MASTER]),
                  {name for name in files if api['SEGMENT'].fullmatch(name)}, revision, '2026-10-27',
                  OWNER, engine, api['SEGMENT'])

    def refuse(operation, message):
        nonlocal refusal_count
        try:
            operation()
        except ValueError as error:
            assert message in str(error), (str(error), message)
            refusal_count += 1
        else:
            raise AssertionError('authoring refusal admitted')

    if any(api['SEGMENT'].fullmatch(name) for name in current):
        captured, windows = append(current)
        assert (captured, windows) == append(current), 'nondeterministic capture'
        cases.append(('actual HEAD capture', current, captured, windows, 'git'))
        sealed = producer['seal_files'](captured, json.loads(captured[MASTER]), engine)
        cases.append(('explicit seal', captured, sealed, [], None))
        raw = 'docs/history/stitchcad-defects-part990000.md'
        carried = {**captured, raw: b'Exact unsealed draft.\n'}
        moved = producer['seal_files'](carried, json.loads(carried[MASTER]), engine)
        assert moved[raw] == carried[raw]
        cases.append(('seal carries unsealed draft', carried, moved, [], None))

    # Freeze only generation1; later repository captures cannot change these synthetic predicates.
    first = json.loads(current[MASTER])['generations'][0]
    group = json.loads(current[first['path']])
    retained = {first['path'], *(row['path'] for row in group['legacy_catalogs'])}
    for name in group['windows']:
        value = json.loads(current[name]); retained.update((name, value['payload'], value['catalog']))
    base = {name: current[name] for name in retained}
    base[ACTIVE] = fixture['document'](dict(version=2, windows=[], legacy_catalogs=[]))
    base[MASTER] = fixture['document'](dict(version=3, generations=[first, {}]))
    fixture['refresh'](base, 2)

    def raw_values(count, number, content):
        return {'docs/history/stitchcad-devnotes-part%d.md' % (number + i): content(i) for i in range(count)}

    def indexed(files, index, number, values):
        fixture['add_window'](files, index, number, values, '0' * 40)

    # Incompressible bounded text forces real multi-payload authoring, below the source4M bound.
    def noise(index):
        data = base64.b64encode(hashlib.shake_256(str(index).encode()).digest(130000))
        return b'\n'.join(data[i:i + 399] for i in range(0, 159600, 399)) + b'\n'
    values = raw_values(16, 990100, noise)
    original = {**base, **values}
    split, windows = append(original)
    assert len(windows) >= 2
    cases.append(('compressed payload split cohort', original, split, windows, 'cohort'))

    crossing = dict(base)
    packed = raw_values(9, 990150, lambda _i: b'')
    _path, additions = fixture['window'](9899, packed, '0' * 40)
    amount = 3_999_950 - sum(map(len, values.values())) - len(additions['docs/history/window9899.md'])
    packed = {name: fixture['bounded_text'](min(160000, max(0, amount - i * 160000)))
              for i, name in enumerate(packed)}
    indexed(crossing, 2, 9899, packed)
    crossing.update(values)
    rolled, windows = append(crossing)
    assert len(json.loads(rolled[MASTER])['generations']) == 3 and len(windows) >= 3
    assert rolled[ACTIVE] != crossing[ACTIVE], 'existing active cohort was not filled before sealing'
    cases.append(('multi-window cohort crosses generation', crossing, rolled, windows, 'cohort'))

    # A valid source may have room for its raw bytes but not the added catalog; rollover is required.
    near = dict(base)
    values = raw_values(25, 990200, lambda _i: b'')
    _path, additions = fixture['window'](9900, values, '0' * 40)
    catalog_bytes = len(additions['docs/history/window9900.md'])
    sizes = [160000] * 24 + [3_999_900 - catalog_bytes - 24 * 160000]
    values = {name: fixture['bounded_text'](size) for name, size in zip(values, sizes)}
    indexed(near, 2, 9900, values)
    near['docs/history/stitchcad-changelog-part990001.md'] = b'x' * 50
    rollover, windows = append(near)
    assert len(json.loads(rollover[MASTER])['generations']) == 3
    assert rollover[ACTIVE] == near[ACTIVE]
    cases.append(('decoded catalog overhead rollover', near, rollover, windows, 'cohort'))

    # Existing16-window boundary rolls to a fresh group without changing the former control.
    full = dict(base)
    for i in range(16):
        indexed(full, 2, 10000 + i, raw_values(1, 991000 + i, lambda _i: b''))
    full['docs/history/stitchcad-g0-contract-evidence-part990002.md'] = b'original typed family\n'
    rolled, windows = append(full)
    assert rolled[ACTIVE] == full[ACTIVE]
    cases.append(('sixteen-window rollover', full, rolled, windows, 'cohort'))

    lines = dict(base)
    values = raw_values(30, 992000, lambda _i: b'')
    _path, additions = fixture['window'](10100, values, '0' * 40)
    available = 60000 - additions['docs/history/window10100.md'].count(b'\n')
    values = {name: b'\n' * min(2000, max(0, available - i * 2000)) for i, name in enumerate(values)}
    indexed(lines, 2, 10100, values)
    lines['docs/history/stitchcad-devnotes-part993000.md'] = b'z'
    rolled, windows = append(lines)
    assert rolled[ACTIVE] == lines[ACTIVE]
    cases.append(('decoded line rollover', lines, rolled, windows, 'cohort'))

    # A control budget may fill independently of decoded history; padded JSON remains valid source.
    controls = dict(full)
    controls.pop('docs/history/stitchcad-g0-contract-evidence-part990002.md')
    group = json.loads(controls[ACTIVE]); group['windows'].pop()
    removed = '.doctrine/history_archive/window10015.json'
    value = json.loads(controls.pop(removed))
    for name in (value['payload'], value['catalog']): controls.pop(name)
    controls[ACTIVE] = fixture['document'](group)
    paths = [ACTIVE, *group['windows']]
    missing = 262144 - sum(len(controls[name]) for name in paths)
    for name in paths[1:]:
        count = min(missing, 65536 - len(controls[name]))
        controls[name] += (b' ' * 399 + b'\n') * (count // 400) + b' ' * (count % 400)
        missing -= count
    assert missing == 0
    fixture['refresh'](controls, 2)
    controls['docs/history/stitchcad-devnotes-part993001.md'] = b'z'
    rolled, windows = append(controls)
    assert rolled[ACTIVE] == controls[ACTIVE]
    cases.append(('control budget rollover', controls, rolled, windows, 'cohort'))

    eight = dict(full)
    for index in range(3, 9):
        path = '.doctrine/history_archive/generation%d.json' % index
        eight[path] = fixture['document'](dict(version=2, windows=[], legacy_catalogs=[]))
        fixture['refresh'](eight, index)
        for i in range(16 if index == 8 else 1):
            indexed(eight, index, 11000 + index * 100 + i,
                    raw_values(1, 994000 + index * 100 + i, lambda _i: b''))
    refuse(lambda: append(eight), 'finite generation capacity exhausted')
    refuse(lambda: producer['seal_files'](eight, json.loads(eight[MASTER]), engine), 'finite generation capacity exhausted')
    cases.append(('eight full-window groups source', eight, eight, [], None))
    refuse(lambda: append(base), 'complete bounded committed raw snapshot required')
    refuse(lambda: producer['seal_files'](base, json.loads(base[MASTER]), engine), 'cannot seal an empty generation')

    checks = 0
    for label, before, after, windows, proof in cases:
        roots = []
        for kind, files in (('before', before), ('after', after)):
            target = work / str(checks) / kind
            outputs = [target / name for name in files]
            directories = {work, target}
            for path in outputs: directories.update(parent for parent in path.parents if parent.is_relative_to(work))
            plan = dict(sources=sources, directories=tuple(sorted(directories)), output_files=tuple(outputs))
            profile['enter_producer'](ROOT, **plan)
            for name, data in files.items():
                profile['plan_producer'](ROOT, **plan)
                with (target / name).open('xb') as stream: stream.write(data)
            roots.append(target)
        original, result = roots
        expected, _ = api['inventory'](original)
        actual, _ = api['inventory'](result)
        assert actual == expected, label
        assert api['backing_inventory'](result) == api['backing_inventory'](original), label
        old = json.loads(before[MASTER]); new = json.loads(after[MASTER])
        assert old['generations'][:-1] == new['generations'][:len(old['generations']) - 1]
        allowed = {MASTER, old['generations'][-1]['path']}
        assert all(after[name] == data for name, data in before.items()
                   if name not in allowed and not api['SEGMENT'].fullmatch(name)), label
        for window in windows:
            value = json.loads(after['.doctrine/history_archive/' + window + '.json'])
            profile['plan_producer'](ROOT, sources=(*sources, *(result / name for name in after)))
            assert subprocess.check_output(['tar', '-tzf', str(result / value['payload'])]).decode().splitlines() == [row['path'] for row in value['members']]
            profile['plan_producer'](ROOT, sources=(*sources, *(result / name for name in after)))
            assert subprocess.check_output(['tar', '-xOzf', str(result / value['payload'])]) == b''.join(expected[row['path']] for row in value['members'])
        if proof == 'git':
            api['guard_committed'](result)
            for window in windows: api['_v2_source_proof'](result, window)
            producer['append'](work / 'prepared', OWNER, source_root=original)
            audit = json.loads((work / 'prepared/source.json').read_bytes())
            assert {row['path'] for row in audit['candidate_files']} == set(after)
        elif proof == 'cohort':
            # Only metadata transport is simulated. Inventory and physical tar bytes stay real.
            paths, blob = api['_git_paths'], api['_git_blob']
            raw = {name for name in before if api['SEGMENT'].fullmatch(name)}
            api['_git_paths'] = lambda _root, _rev: raw
            def source_blob(_root, _rev, name, _bound):
                assert _rev == revision and name in {MASTER, *raw}
                return before[name]
            api['_git_blob'] = source_blob
            try:
                for window in windows: assert api['_v2_source_proof'](result, window) == (len(raw), 0, 0)
                api['_git_paths'] = lambda _root, _rev: raw | {'docs/history/stitchcad-devnotes-part999999.md'}
                try: api['_v2_source_proof'](result, windows[0])
                except api['Refusal'] as error: assert 'source snapshot membership mismatch' in str(error)
                else: raise AssertionError('incomplete cohort admitted')
            finally:
                api['_git_paths'], api['_git_blob'] = paths, blob
        checks += 1
    assert all(path.read_bytes() == data for path, data in frozen.items()), 'authoring changed source inputs'
    assert not (ROOT / 'target/history-authoring-red-0393').exists()
    print('generation authoring: %d physical source/candidate controls; %d early pure refusals; native tar/Git bytes exact; rc=0' % (checks, refusal_count))


if __name__ == '__main__':
    main()
