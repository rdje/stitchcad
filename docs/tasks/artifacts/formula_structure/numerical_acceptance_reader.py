"""Read fresh tangent/resource native evidence independently; no compiler/browser or writes."""
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
    value = copy.deepcopy(original);value['scope'] = 'SIMULATED';cases.append(value)
    value = copy.deepcopy(original);value['simulated_children'] = True;cases.append(value)
    value = copy.deepcopy(original);value['outcomes'][1]['compile_succeeded'] = False;cases.append(value)
    value = copy.deepcopy(original);value['outcomes'][1]['cargo_rc'] = 0;cases.append(value)
    value = copy.deepcopy(original);value['outcomes'][1]['failed_fixture_rows'] = [0];cases.append(value)
    value = copy.deepcopy(original);value['outcomes'][1]['independent_failure'] = 'unrelated';cases.append(value)
    value = copy.deepcopy(original);value['snapshot_restored'] = False;cases.append(value)
    value = copy.deepcopy(original);value['outcomes'] = value['outcomes'][:1];cases.append(value)
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


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--self-test', action='store_true')
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
    print(json.dumps(dict(scope='fresh actual tangent/resource native acceptance;platform pending',
        restored_artifacts=restored_artifacts,
        source_files=len(before['sources']), legacy_files_unchanged=len(before['legacy']),
        records=records, baselines=sum(item['baselines'] for item in records),
        compiled_fault_rejections=sum(item['compiled_fault_rejections'] for item in records),
        restored_snapshot_files=sum(item['restored_snapshot_files'] for item in records), rc=0), indent=2))


if __name__ == '__main__':
    main()
