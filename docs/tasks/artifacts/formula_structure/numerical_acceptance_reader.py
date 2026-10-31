"""Read fresh native evidence and optional Chrome captures; no compiler/browser or writes."""
from pathlib import Path
import argparse
import hashlib
import json
import os
import re
import runpy
import subprocess

ROOT = Path(__file__).resolve().parents[4]
PROFILE = runpy.run_path(str(ROOT / 'scripts/local_environment.py'))


def read(path):
    PROFILE['plan_producer'](ROOT, sources=(path,))
    return path.read_bytes()


def digest(path):
    return hashlib.sha256(read(path)).hexdigest()


def recorded_path(name):
    relative = PROFILE['producer_path'](ROOT, name)
    return ROOT / relative


def verify_hashes(values):
    for name, expected in values.items():
        assert re.fullmatch('[0-9a-f]{64}', expected)
        assert digest(recorded_path(name)) == expected, 'recorded source/artifact digest drift: ' + name


def native_report(path, family, faults, rows, fixture, snapshot):
    report = json.loads(read(path))
    assert report['scope'] == 'actual copied native ' + family + ' bodies;finite sourced/storage fault check only'
    assert not report.get('simulated_children') and 'SIMULATED' not in report['compiler']
    assert report['rows'] == rows and report['snapshot_restored'] is True
    fixture = ROOT / 'crates/sc-core/tests/fixtures' / fixture
    assert digest(fixture) == report['fixture_sha256']
    resource = family.startswith('tangent-resource')
    watched = ('independent_resource_rows_preserve_actual_stages_limits_demands_and_prior_progress' if resource else
               'actual_upstream_sources_preserve_all_original_tangent_outcomes_through_signed_storage')
    target = 'formula_tangent_resource_contract' if resource else 'formula_tangent_oracle_contract'
    original_rows = read(fixture).decode().splitlines()[1:]
    assert len(original_rows) == rows
    outcomes = report['outcomes']
    assert len(outcomes) == faults + 1 and outcomes[0]['label'] == 'baseline'
    assert len({outcome['label'] for outcome in outcomes}) == len(outcomes)
    for index, outcome in enumerate(outcomes):
        assert outcome['compile_succeeded'] is True
        assert outcome['cargo_rc'] == (0 if index == 0 else 101)
        text = read(recorded_path(outcome['log'])).decode()
        assert 'SIMULATED' not in text and 'running 1 test' in text and watched in text
        messages, artifacts = [], []
        for line in text.splitlines():
            try:
                item = json.loads(line)
            except json.JSONDecodeError:
                continue
            if isinstance(item, dict) and item.get('reason') == 'build-finished':
                messages.append(item)
            if (isinstance(item, dict) and item.get('reason') == 'compiler-artifact'
                    and item.get('target', {}).get('name') == target and item.get('executable')):
                artifacts.append(item)
        assert len(artifacts) == 1, 'actual named consumer artifact required'
        assert len(messages) == 1 and messages[0]['success'] is True
        if index == 0:
            assert outcome['failed_fixture_rows'] == [] and '1 passed; 0 failed' in text
        else:
            assert '1 failed' in text and len(outcome['failed_fixture_rows']) == 1
            ordinal = outcome['failed_fixture_rows'][0]
            assert 1 <= ordinal <= rows
            marker = 'fixture-row=' + str(ordinal) + ';' + original_rows[ordinal - 1]
            assert marker in text and marker in outcome['independent_failure']
            assert re.search(r"(?m)^thread '" + re.escape(watched) + r"'(?: \([0-9]+\))? panicked at ", text)
    expected_inputs = snapshot | {'scripts/local_environment.py',
        'docs/tasks/artifacts/formula_structure/root_body_faults.py',
        'docs/tasks/artifacts/formula_structure/' + ('tangent_resource_body_faults.py' if resource else 'tangent_body_faults.py')}
    assert set(report['original_sources_unchanged']) == expected_inputs, 'complete native input population drift'
    verify_hashes(report['original_sources_unchanged'])
    workspace = path.parent / 'workspace'
    actual = {file.relative_to(workspace).as_posix() for file in workspace.rglob('*') if file.is_file()}
    assert actual == snapshot, 'complete restored native snapshot population drift'
    for name in sorted(snapshot):
        assert read(workspace / name) == read(ROOT / name), 'restored native snapshot bytes drift'
    assert report['timeout_control'] == dict(parent_only_child_wrote=True, guarded_group_child_write_absent=True)
    assert report['capture_controls']['actual_guard_omissions_rejected'] == 2
    return dict(report=path.relative_to(ROOT).as_posix(), sha256=digest(path), baselines=1,
                compiled_fault_rejections=faults, rows=rows, restored_snapshot_files=len(snapshot),
                compiler=report['compiler'])


def refusal_controls(run, snapshot):
    import copy
    paths = re.findall(r'(?m)^retained native capture:(.+/report\.json)$', read(run / 'tangent.log').decode())
    path = recorded_path(paths[0])
    original = json.loads(read(path))
    cases = []
    value = copy.deepcopy(original)
    value['scope'] = 'SIMULATED'
    cases.append(value)
    value = copy.deepcopy(original)
    value['simulated_children'] = True
    cases.append(value)
    value = copy.deepcopy(original)
    value['outcomes'][1]['compile_succeeded'] = False
    cases.append(value)
    value = copy.deepcopy(original)
    value['outcomes'][1]['cargo_rc'] = 0
    cases.append(value)
    value = copy.deepcopy(original)
    value['outcomes'][1]['failed_fixture_rows'] = [0]
    cases.append(value)
    value = copy.deepcopy(original)
    value['outcomes'][1]['independent_failure'] = 'unrelated'
    cases.append(value)
    value = copy.deepcopy(original)
    value['snapshot_restored'] = False
    cases.append(value)
    value = copy.deepcopy(original)
    value['outcomes'] = value['outcomes'][:1]
    cases.append(value)
    reader = globals()['read']
    try:
        for value in cases:
            globals()['read'] = lambda selected, value=value: json.dumps(value).encode() if selected == path else reader(selected)
            try:
                native_report(path, 'tangent-fold', 3, 139, 'formula_tangent.tsv', snapshot)
            except AssertionError:
                pass
            else:
                raise AssertionError('independent acceptance reader admitted a defective report')
    finally:
        globals()['read'] = reader
    print('acceptance reader:8 defective native reports refused;no writes;rc=0')


def platform_report(run, snapshot):
    """Bind retained DOMs, binaries and complete copied consumers to the actual platform report."""
    from html.parser import HTMLParser
    text = read(run / 'platform.log').decode()
    paths = re.findall(r'(?m)^irrational platform:31 checks/native\+actual Chrome/'
                       r'bad expectation rejected/originals restored;report=(.+/report\.json);rc=0$', text)
    assert len(paths) == 1, 'one actual completed platform report required'
    path = recorded_path(paths[0])
    platform = json.loads(read(path))
    assert not platform.get('simulated_children') and 'SIMULATED' not in json.dumps(platform)
    assert platform['selected_checks'] == 31 and platform['original_fixture_rows'] == 405
    assert platform['workspace_restored'] and not platform['new_ambient_chrome_entries']
    expected_sources = snapshot | {'docs/tasks/artifacts/formula_structure/irrational_platform.py',
        'scripts/local_environment.py', 'docs/tasks/artifacts/artifact_cleanup/cleanup.py',
        'scripts/check_handoff.sh', 'scripts/handoff_census.py'}
    assert set(platform['original_sources_unchanged']) == expected_sources
    verify_hashes(platform['original_sources_unchanged'])
    verify_hashes(platform['fixtures'])
    baseline, wrong = platform['outcomes']
    assert baseline['label'] == 'baseline' and baseline['native_rc'] == 104
    assert baseline['browser']['status'] == 'pass' and baseline['browser']['marker'] == 104
    assert baseline['browser']['phase'] == 'completed' and baseline['browser']['imports'] == []
    assert wrong['label'] == 'wrong-fixture' and wrong['native_rc'] == 101
    assert wrong['browser']['status'] == 'trap' and wrong['browser']['phase'] == 'executing'
    assert wrong['browser']['name'] == 'RuntimeError'

    class Result(HTMLParser):
        def __init__(self):
            super().__init__()
            self.active, self.count, self.parts = False, 0, []

        def handle_starttag(self, tag, attributes):
            if tag == 'pre' and dict(attributes).get('id') == 'result':
                self.active = True
                self.count += 1

        def handle_endtag(self, tag):
            if tag == 'pre':
                self.active = False

        def handle_data(self, value):
            if self.active:
                self.parts.append(value)

    for outcome in (baseline, wrong):
        assert set(outcome['binaries']) == {'native', 'wasm'}
        for artifact in outcome['binaries'].values():
            assert digest(recorded_path(artifact['path'])) == artifact['sha256']
        parsed = Result()
        parsed.feed(read(path.parent / (outcome['label'] + '-dom.html')).decode())
        assert parsed.count == 1 and json.loads(''.join(parsed.parts)) == outcome['browser']
        for kind in ('native', 'wasm'):
            log = read(path.parent / (outcome['label'] + '-' + kind + '-build.log')).decode()
            messages = [json.loads(line) for line in log.splitlines() if line.startswith('{')]
            finished = [item for item in messages if item.get('reason') == 'build-finished']
            assert len(finished) == 1 and finished[0]['success'] is True
    assert 'degree_-720' in read(path.parent / 'wrong-fixture-native-run.log').decode()
    assert 'handoff: OK' in read(path.parent / 'handoff.log').decode()
    for name, expected in (('rustc', platform['rustc']), ('chrome-version', platform['chrome'])):
        for suffix in ('.log', '-final.log'):
            assert expected.strip() in read(path.parent / (name + suffix)).decode()

    excluded = {
        'independent_isqrt_fraction_reference_regenerates_the_entire_root_fixture',
        'independent_fraction_pi_reference_regenerates_every_original_arc_fixture_byte',
        'retained_oracle_is_reproduced_without_native_folding_or_pi_input',
        'independent_oracle_reproduces_the_retained_fixture',
        'independent_fraction_producer_regenerates_every_frozen_byte',
        'independent_resource_producer_regenerates_every_frozen_byte',
    }
    consumers = ('formula_root_value_contract.rs', 'formula_arc_value_contract.rs',
        'formula_sincos_value_contract.rs', 'formula_inverse_value_contract.rs',
        'formula_tangent_oracle_contract.rs', 'formula_tangent_value_contract.rs',
        'formula_evaluated_tangent_contract.rs', 'formula_tangent_resource_contract.rs',
        'formula_tangent_prepare_contract.rs')
    assert set(platform['consumers']) == set(consumers) and set(platform['excluded_host_only']) == excluded
    copied = path.parent / 'workspace/consumers'
    expected_files, fixtures = set(consumers), set()
    for name in consumers:
        original = read(ROOT / 'crates/sc-core/tests' / name).decode()
        lines = original.splitlines()
        functions = []
        for index, line in enumerate(lines):
            if line != '#[test]':
                continue
            following = index + 1
            while lines[following].startswith('#['):
                following += 1
            match = re.fullmatch(r'fn ([a-z0-9_]+)\(\) \{', lines[following])
            assert match, 'original test syntax changed;review reader'
            if match[1] not in excluded:
                functions.append(match[1])
        assert platform['consumers'][name] == functions
        retained = read(copied / name).decode()
        body, wrapper = retained.removeprefix('#![allow(dead_code)]\n').rsplit('\npub fn platform_check()', 1)
        assert body == original.replace('#[test]\n', ''), 'original assertion bodies changed'
        assert re.findall(r'(?m)^    ([a-z0-9_]+)\(\);$', wrapper) == functions
        for fixture in re.findall(r'include_str!\("([^"\n]+)"\)', original):
            fixtures.add(fixture)
            assert read(copied / fixture) == read(ROOT / 'crates/sc-core/tests' / fixture)
    expected_files.update(fixtures)
    assert {item.relative_to(copied).as_posix() for item in copied.rglob('*') if item.is_file()} == expected_files
    assert len(fixtures) == 6 and set(platform['fixtures']) == {
        'crates/sc-core/tests/' + fixture for fixture in fixtures}
    assert not recorded_path(platform['temporary_root']).exists()
    sealed = recorded_path(platform['temporary_sealed_at'])
    assert sealed.is_dir() and sealed.stat().st_dev == ROOT.stat().st_dev
    actual_temp = []
    for item in sorted(sealed.rglob('*')):
        assert '.git' not in item.relative_to(sealed).parts
        info = item.lstat()
        assert info.st_dev == ROOT.stat().st_dev
        actual_temp.append(dict(path=item.relative_to(sealed).as_posix(), bytes=info.st_size,
            kind='link' if item.is_symlink() else 'directory' if item.is_dir() else 'file',
            sha256=digest(item) if not item.is_symlink() and item.is_file() else None,
            link=os.readlink(item) if item.is_symlink() else None))
    assert actual_temp == platform['temporary_entries']
    return dict(platform_report=paths[0], platform_sha256=digest(path),
                platform_source_files=len(platform['original_sources_unchanged']), platform_checks=31,
                platform_rows=405, rustc=platform['rustc'], chrome=platform['chrome'], host=platform['host'])


def platform_refusal_controls(run, snapshot):
    import copy
    paths = re.findall(r'(?m)^irrational platform:.*;report=(.+/report\.json);rc=0$',
                       read(run / 'platform.log').decode())
    assert len(paths) == 1
    path = recorded_path(paths[0])
    original = json.loads(read(path))
    cases = []
    value = copy.deepcopy(original)
    value['simulated_children'] = True
    cases.append(value)
    value = copy.deepcopy(original)
    value['outcomes'][0]['browser']['marker'] = 0
    cases.append(value)
    value = copy.deepcopy(original)
    value['outcomes'][1]['browser']['phase'] = 'compiled'
    cases.append(value)
    value = copy.deepcopy(original)
    value['outcomes'][1]['native_rc'] = 0
    cases.append(value)
    value = copy.deepcopy(original)
    value['original_sources_unchanged'].pop(next(iter(value['original_sources_unchanged'])))
    cases.append(value)
    value = copy.deepcopy(original)
    value['workspace_restored'] = False
    cases.append(value)
    value = copy.deepcopy(original)
    value['outcomes'][0]['binaries']['wasm']['sha256'] = '0' * 64
    cases.append(value)
    value = copy.deepcopy(original)
    value['temporary_entries'] = [dict(unexpected=True)]
    cases.append(value)
    reader = globals()['read']
    try:
        for value in cases:
            globals()['read'] = lambda selected, value=value: json.dumps(value).encode() if selected == path else reader(selected)
            try:
                platform_report(run, snapshot)
            except AssertionError:
                pass
            else:
                raise AssertionError('independent platform reader admitted defective evidence')
    finally:
        globals()['read'] = reader
    print('platform reader:8 defective platform reports refused;no writes;rc=0')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--self-test', action='store_true')
    parser.add_argument('--platform', action='store_true')
    parser.add_argument('run', help='repository-relative owned acceptance directory')
    args = parser.parse_args()
    run = recorded_path(args.run)
    assert run.is_relative_to(ROOT / 'target/irrational_platform_certificate')
    before = json.loads(read(run / 'before.json'))
    verify_hashes(before['sources'])
    verify_hashes(before['legacy'])
    current_legacy = {file.relative_to(ROOT).as_posix() for name in (
        'root', 'arc', 'forward', 'inverse', 'tangent', 'tangent_resource')
        for file in (ROOT / 'target' / (name + '_body_faults')).rglob('*') if file.is_file()}
    assert current_legacy == set(before['legacy']), 'complete legacy evidence population drift'
    environment = {key:value for key,value in os.environ.items() if not key.startswith('GIT_')}
    environment.update(GIT_CONFIG_NOSYSTEM='1', GIT_CONFIG_GLOBAL=os.devnull, GIT_OPTIONAL_LOCKS='0')
    snapshot = set(subprocess.check_output(['git', 'ls-files', 'crates'], cwd=ROOT,
        env=environment, text=True).splitlines()) | {'Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml'}
    expected_inputs = snapshot | {'scripts/local_environment.py',
        *("docs/tasks/artifacts/formula_structure/" + name + '.py' for name in
          ('root_body_faults', 'tangent_body_faults', 'tangent_resource_body_faults', 'irrational_platform'))}
    assert set(before['sources']) == expected_inputs, 'complete original input population drift'
    records = []
    for log, expected in (
        ('tangent.log', (('tangent-fold', 3), ('tangent-interval', 3), ('tangent-execution', 3))),
        ('resource.log', (('tangent-resource-prepare', 6), ('tangent-resource-execute', 7)))):
        paths = re.findall(r'(?m)^retained native capture:(.+/report\.json)$', read(run / log).decode())
        assert len(paths) == len(expected), 'native public wrapper report count drift'
        for name, (family, faults) in zip(paths, expected):
            resource = family.startswith('tangent-resource')
            records.append(native_report(recorded_path(name), family, faults, 33 if resource else 139,
                'formula_tangent_resources.tsv' if resource else 'formula_tangent.tsv', snapshot))
    if args.self_test:
        refusal_controls(run, snapshot)
    restored = read(run / 'restored.log').decode()
    assert restored.count('test result: ok.') == 2 and 'test result: FAILED' not in restored
    assert 'SIMULATED' not in restored
    assert all(target + '-' in restored for target in ('formula_tangent_oracle_contract', 'formula_tangent_resource_contract'))
    restored_artifacts = {}
    for target in ('formula_tangent_oracle_contract', 'formula_tangent_resource_contract'):
        artifact = run / ('restored-' + target + '.native')
        paths = re.findall(r'Running tests/' + target + r'\.rs \(([^)]+)\)', restored)
        assert len(paths) == 1 and digest(artifact) == digest(recorded_path(paths[0]))
        restored_artifacts[target] = dict(path=artifact.relative_to(ROOT).as_posix(), sha256=digest(artifact))
    result = dict(scope='fresh actual tangent/resource native acceptance;platform outside scope',
        restored_artifacts=restored_artifacts,
        source_files=len(before['sources']), legacy_files_unchanged=len(before['legacy']),
        records=records, baselines=sum(item['baselines'] for item in records),
        compiled_fault_rejections=sum(item['compiled_fault_rejections'] for item in records),
        restored_snapshot_files=sum(item['restored_snapshot_files'] for item in records), rc=0)
    if args.platform:
        result.update(platform_report(run, snapshot), scope='fresh actual tangent/resource/native+Chrome acceptance')
        result['native_source_files'] = result.pop('source_files')
        if args.self_test:
            platform_refusal_controls(run, snapshot)
    print(json.dumps(result, indent=2))


if __name__ == '__main__':
    main()
