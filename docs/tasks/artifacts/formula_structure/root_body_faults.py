"""Compile isolated copies of actual native root faults against the independent root fixture.

Tracked production sources stay untouched. A complete repository-local workspace snapshot
keeps the original private crate scope and uses only ordinary published Cargo test commands.
This finite native check is not transitive compiler/locality or cross-platform signoff.
"""
from pathlib import Path
import argparse
import hashlib
import json
import os
import runpy
import signal
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[4]
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
SOURCE = ROOT / 'crates/sc-core/src/recipe/roots.rs'
FIXTURE = ROOT / 'crates/sc-core/tests/fixtures/formula_roots.tsv'
RUN = ROOT / 'target/root_body_faults/0625-workspace'
COPY = RUN / 'workspace'
REPORT = RUN / 'report.json'
FAULTS = (
    ('tie_toward_zero', 'if (numerator << 2_u8) >= denominator * &midpoint_twice * &midpoint_twice {',
     'if (numerator << 2_u8) > denominator * &midpoint_twice * &midpoint_twice {'),
    ('ratio_scale_lost', 'if ratio { 1_000_000_u128 } else { 1 }', 'if ratio { 1_u128 } else { 1 }'),
    ('hypot_right_lost', '&left_n * &left_n * &right_d * &right_d + &right_n * &right_n * &left_d * &left_d',
     '&left_n * &left_n * &right_d * &right_d + BigUint::from(0_u8)'),
    ('hypot_denominator_lost', 'let denominator = &left_d * &left_d * &right_d * &right_d;',
     'let denominator = &left_d * &left_d;'),
    ('round_after_quotient', 'if (numerator << 2_u8) >= denominator * &midpoint_twice * &midpoint_twice {',
     'if (quotient << 2_u8) >= denominator * &midpoint_twice * &midpoint_twice {'),
)
LABELS = ('baseline', *(label for label, _, _ in FAULTS))
TEST = 'coupled_root_rows_retain_real_upstream_calls_through_success_storage_and_original_refusals'


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def stop_group(process):
    try:
        os.killpg(process.pid, signal.SIGKILL)
    except ProcessLookupError:
        pass
    return process.communicate()


def run_group(arguments, *, timeout):
    """An interrupted compiler/test must lose its owned descendants before snapshot restoration."""
    process = subprocess.Popen(arguments, cwd=ROOT,
        stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, start_new_session=True)
    try:
        stdout, stderr = process.communicate(timeout=timeout)
    except BaseException:
        stop_group(process)
        raise
    return subprocess.CompletedProcess(arguments, process.returncode, stdout, stderr)


def timeout_control(profile, negative, positive):
    """Actual owned parent/child fixtures distinguish parent-only cleanup from group cleanup."""
    assert not positive.exists(), 'unexpected prior interrupted-child output'
    profile['plan_producer'](ROOT, output_files=(negative, positive))
    negative.write_text('pending\n')
    def command(marker):
        child = 'import time;from pathlib import Path;time.sleep(1);Path(%r).write_text("child ran\\n")' % (
            marker.relative_to(ROOT).as_posix(),)
        parent = ('import subprocess,sys,time;p=subprocess.Popen([sys.executable,"-I","-B","-c",%r],'
            'stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL);print(p.pid,flush=True);time.sleep(30)') % child
        return [sys.executable, '-I', '-B', '-c', parent]
    process = subprocess.Popen(command(negative), cwd=ROOT, stdout=subprocess.PIPE,
        stderr=subprocess.PIPE, text=True, start_new_session=True)
    try:
        try:
            process.communicate(timeout=0.5)
        except subprocess.TimeoutExpired:
            process.kill()
            stdout, _ = process.communicate()
            assert stdout.strip().isdigit(), 'actual descendant was started'
            deadline = time.monotonic() + 5
            while negative.read_text() != 'child ran\n' and time.monotonic() < deadline:
                time.sleep(0.05)
            assert negative.read_text() == 'child ran\n', 'parent-only kill must expose live child'
        else:
            raise AssertionError('negative timeout fixture did not time out')
    finally:
        stop_group(process)
    try:
        run_group(command(positive), timeout=0.5)
    except subprocess.TimeoutExpired as error:
        assert error.output and error.output.strip().isdigit(), 'actual guarded descendant was started'
    else:
        raise AssertionError('guarded timeout fixture did not time out')
    time.sleep(1.1)
    assert not positive.exists(), 'owned descendant escaped process-group cleanup'
    return {'parent_only_child_wrote': True, 'guarded_group_child_write_absent': True}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--plan-only', action='store_true')
    args = parser.parse_args()
    profile = runpy.run_path(str(PROFILE_SOURCE))
    initial = (Path(__file__), PROFILE_SOURCE, SOURCE, FIXTURE, ROOT / 'Cargo.toml',
        ROOT / 'Cargo.lock', ROOT / 'rust-toolchain.toml')
    logs = tuple(RUN / (label + '.log') for label in LABELS)
    negative, positive = RUN / 'timeout-parent-only.txt', RUN / 'timeout-group.txt'
    # Guard the fixed roots before even read-only inventory dispatch. The full file plan below
    # is derived without creation;all paths are validated again before any snapshot/build write.
    profile['plan_producer'](ROOT, sources=initial, directories=(RUN, COPY, ROOT / 'target'),
        output_files=(*logs, REPORT, negative, positive))
    inventory = subprocess.run(['git', 'ls-files', 'crates'], cwd=ROOT,
        check=True, capture_output=True, text=True).stdout.splitlines()
    snapshot = tuple(sorted({ROOT / name for name in inventory} | {
        ROOT / name for name in ('Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml')}))
    sources = tuple(sorted(set(initial) | set(snapshot)))
    destinations = tuple(COPY / path.relative_to(ROOT) for path in snapshot)
    outputs = (*destinations, *logs, REPORT, negative, positive)
    directories = tuple(sorted({RUN, COPY, ROOT / 'target', *(path.parent for path in destinations)}))
    profile['plan_producer'](ROOT, sources=sources, directories=directories, output_files=outputs)
    before_hashes = {path.relative_to(ROOT).as_posix(): digest(path) for path in sources}
    original, fixture = SOURCE.read_bytes(), FIXTURE.read_bytes()
    rows = fixture.decode().splitlines()[1:]
    assert len(rows) == 85
    text = original.decode()
    assert all(text.count(before) == 1 and before != after for _, before, after in FAULTS)
    if COPY.exists():
        existing = {path for path in COPY.rglob('*') if path.is_file()}
        assert existing <= set(destinations), 'unplanned existing copied workspace file'
    if args.plan_only:
        print('native root fault plan:6 variants/%d exact snapshot files/%d complete outputs;'
            'no writes or compiler dispatch;rc=0' % (len(snapshot), len(outputs)))
        return
    profile['enter_producer'](ROOT, sources=sources, directories=directories, output_files=outputs)
    timeout_result = timeout_control(profile, negative, positive)
    for source, destination in zip(snapshot, destinations):
        profile['plan_producer'](ROOT, sources=(source,), output_files=(destination,))
        destination.write_bytes(source.read_bytes())
    assert all(digest(destination) == before_hashes[source.relative_to(ROOT).as_posix()]
        for source, destination in zip(snapshot, destinations)), 'exact original workspace snapshot'
    body = COPY / SOURCE.relative_to(ROOT)
    compiler = run_group(['rustc', '-vV'], timeout=30)
    assert compiler.returncode == 0, 'actual compiler identity'
    outcomes = []
    try:
        for label, log in zip(LABELS, logs):
            variant = original
            if label != 'baseline':
                _, before, after = next(fault for fault in FAULTS if fault[0] == label)
                variant = text.replace(before, after, 1).encode()
            profile['plan_producer'](ROOT, sources=sources, output_files=(body, log))
            body.write_bytes(variant)
            profile['plan_producer'](ROOT, sources=destinations, output_files=(log,))
            result = run_group(['cargo', 'test', '--offline', '--locked', '--manifest-path',
                str(COPY / 'Cargo.toml'), '-p', 'sc-core', '--test', 'formula_root_value_contract',
                '--message-format=json', TEST, '--', '--nocapture'], timeout=180)
            transcript = result.stdout + result.stderr
            profile['plan_producer'](ROOT, output_files=(log,))
            log.write_text(transcript.replace(str(ROOT), '.'))
            messages = []
            for line in result.stdout.splitlines():
                try:
                    messages.append(json.loads(line))
                except json.JSONDecodeError:
                    pass
            builds = [item for item in messages if item.get('reason') == 'build-finished']
            assert len(builds) == 1 and builds[0]['success'], \
                'compiler failure is not an actual root fault rejection:' + label
            assert 'running 1 test' in transcript and TEST in transcript, 'actual watched native test'
            failed_rows = [index + 1 for index, row in enumerate(rows) if row in transcript]
            if label == 'baseline':
                assert result.returncode == 0 and '1 passed; 0 failed' in transcript
                assert digest(body) == before_hashes[SOURCE.relative_to(ROOT).as_posix()]
            else:
                assert result.returncode != 0 and '1 failed' in transcript and 'assertion' in transcript and failed_rows, \
                    'actual compiled root fault survived or unrelated refusal:' + label
            binaries = [Path(item['executable']) for item in messages
                if item.get('reason') == 'compiler-artifact' and item.get('target', {}).get('name') ==
                'formula_root_value_contract' and item.get('executable')]
            assert len(binaries) == 1, 'published Cargo test artifact identity'
            profile['plan_producer'](ROOT, sources=(binaries[0],))
            outcomes.append({'label': label, 'cargo_rc': result.returncode, 'compile_succeeded': True,
                'body_sha256': digest(body), 'failed_fixture_rows': failed_rows, 'binary': binaries[0].relative_to(ROOT).as_posix(),
                'binary_sha256': digest(binaries[0]), 'log': log.relative_to(ROOT).as_posix()})
            print('native root body:' + label + (' baseline passes' if label == 'baseline'
                else ' actual compiled test rejects fault'), flush=True)
    finally:
        profile['plan_producer'](ROOT, output_files=(body,))
        body.write_bytes(original)
    assert {path.relative_to(ROOT).as_posix(): digest(path) for path in sources} == before_hashes
    assert all(digest(destination) == before_hashes[source.relative_to(ROOT).as_posix()]
        for source, destination in zip(snapshot, destinations)), 'all snapshot bytes restored'
    assert {path for path in COPY.rglob('*') if path.is_file()} == set(destinations), \
        'complete actual snapshot output census'
    report = {'scope': 'actual copied native root bodies;finite85-row sourced/storage fault check only',
        'original_sources_unchanged': before_hashes, 'snapshot_restored': True,
        'compiler': compiler.stdout, 'fixture_sha256': digest(FIXTURE), 'outcomes': outcomes,
        'timeout_control': timeout_result,
        'planned_outputs': [path.relative_to(ROOT).as_posix() for path in outputs]}
    profile['plan_producer'](ROOT, output_files=(REPORT,))
    REPORT.write_text(json.dumps(report, indent=2) + '\n')
    print('native root body faults:baseline1/actual compiled faults5/all original and snapshot hashes unchanged;rc=0')


if __name__ == '__main__':
    main()
