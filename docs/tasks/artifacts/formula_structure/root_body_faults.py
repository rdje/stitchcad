"""Compile isolated original-scope native kernel faults against independent fixtures.

Tracked production sources stay untouched. A complete repository-local workspace snapshot
keeps the original private crate scope and uses only ordinary published Cargo test commands.
This finite native check is not transitive compiler/locality or cross-platform signoff.
"""
from pathlib import Path
from datetime import datetime, timezone
import argparse
import ast
import hashlib
import json
import os
import re
import runpy
import signal
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[4]
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
SOURCE = ROOT / 'crates/sc-core/src/recipe/roots.rs'
FIXTURE = ROOT / 'crates/sc-core/tests/fixtures/formula_roots.tsv'
RUN = ROOT / 'target/root_body_faults/0632-exact-root'
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


def reserve_capture(path):
    """Only an atomically created new directory can own a replay's writes."""
    path.mkdir()
    identity = path.stat()
    return identity.st_dev, identity.st_ino


def write_report(path, report, owner):
    identity = path.parent.lstat()
    assert not path.parent.is_symlink() and (identity.st_dev, identity.st_ino) == owner, \
        'capture ownership changed before publication'
    with path.open('x') as stream:
        stream.write(json.dumps(report, indent=2) + '\n')


def capture_controls(profile, directory):
    """Exercise actual sinks and actual guard omissions in separate owned control directories."""
    bases = tuple(directory / label for label in ('guarded', 'reuse-omitted', 'exclusive-omitted'))
    outputs = tuple(base / 'report.json' for base in bases)
    profile['plan_producer'](ROOT, directories=(directory, *bases), output_files=outputs)
    directory.mkdir()
    source = ast.parse(Path(__file__).read_text())
    def omitted(name, old, new):
        node = next(node for node in source.body if isinstance(node, ast.FunctionDef) and node.name == name)
        text = ast.unparse(node)
        assert text.count(old) == 1, 'one exact capture guard anchor'
        namespace = {'json': json}
        exec(compile(text.replace(old, new, 1), '<actual capture guard omission>', 'exec'), namespace)
        return namespace[name]
    unsafe_reserve = omitted('reserve_capture', 'path.mkdir()', 'path.mkdir(exist_ok=True)')
    unsafe_write = omitted('write_report', "path.open('x')", "path.open('w')")
    def exercise(base, reserve, writer):
        owner = reserve(base)
        writer(base / 'report.json', {'sentinel': 'original receipt'}, owner)
        original = (base / 'report.json').read_bytes()
        for action in (lambda: reserve(base),
                       lambda: writer(base / 'report.json', {'replacement': True}, owner)):
            try:
                action()
            except FileExistsError:
                pass
            else:
                raise AssertionError('existing capture/report was admitted')
        try:
            writer(base / 'report.json', {'replacement': True}, (owner[0], owner[1] + 1))
        except AssertionError:
            pass
        else:
            raise AssertionError('foreign capture identity was admitted')
        assert (base / 'report.json').read_bytes() == original
    exercise(bases[0], reserve_capture, write_report)
    for base, reserve, writer in ((bases[1], unsafe_reserve, write_report),
                                  (bases[2], reserve_capture, unsafe_write)):
        try:
            exercise(base, reserve, writer)
        except AssertionError:
            pass
        else:
            raise AssertionError('actual capture guard omission survived')
    result = {'fresh_receipt': True, 'existing_capture_refused': True,
              'existing_report_refused': True, 'foreign_identity_refused': True,
              'original_guarded_receipt_unchanged': True, 'actual_guard_omissions_rejected': 2}
    print('capture controls:fresh1/refusals3/original unchanged/actual guard omissions2;rc=0', flush=True)
    return result


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


def classify_failure(stderr, test_name, rows):
    """Bind one actual watched-test panic to its explicit ordinal and complete original row."""
    headers = [line for line in stderr.splitlines()
               if line.startswith("thread '") and ' panicked at ' in line]
    assert len(headers) == 1, 'one actual watched-test panic required'
    pattern = (r"^thread '" + re.escape(test_name)
               + r"'(?: \([0-9]+\))? panicked at [^\n]+\n([^\n]+)$")
    payloads = re.findall(pattern, stderr, re.MULTILINE)
    assert len(payloads) == 1, 'actual panic belongs to a different test'
    payload = payloads[0]
    markers = list(re.finditer(r'fixture-row=([1-9][0-9]*);', payload))
    assert len(markers) == 1, 'independent ordinal assertion marker required'
    marker = markers[0]
    ordinal = int(marker[1])
    assert 1 <= ordinal <= len(rows), 'fixture ordinal outside original population'
    tail = payload[marker.end():]
    row = rows[ordinal - 1]
    assert tail == row or tail.startswith(row + ':'), 'complete original fixture row required'
    return ordinal, payload


def classifier_controls():
    negative = '-720000000\t1\t0\t1000000'
    zero = '0\t1\t0\t1000000'
    positive = '720000000\t1\t0\t1000000'
    duplicate = 'sqrt\tarea\t9\t4\t0\t1\tlength\tinteger:2'
    rows = [negative, zero, positive, duplicate, duplicate]
    header = "thread 'watched' (123) panicked at original.rs:1:1:\n"
    def panic(ordinal, row):
        return header + f'assertion `left == right` failed: fixture-row={ordinal};{row}:Cos\n'
    for ordinal in (1, 2, 5):
        assert classify_failure(panic(ordinal, rows[ordinal - 1]), 'watched', rows)[0] == ordinal
    legacy = header + 'assertion `left == right` failed: ' + negative + '\n'
    assert [i + 1 for i, row in enumerate(rows) if row in legacy] == [1, 2, 3]
    assert [i + 1 for i, row in enumerate(rows) if row in duplicate] == [4, 5]
    refused = [
        legacy,
        '{"debug_assertions":true}\nunrelated assertion failure\n',
        panic(1, negative).replace("'watched'", "'unrelated'"),
        panic(1, negative) + panic(2, zero),
        panic(0, negative),
        panic(6, negative),
        panic(1, positive),
        panic(1, negative[:-1]),
        panic(1, 'extra' + negative),
        panic(1, negative + '\textra'),
        header + 'unrelated panic\nfixture-row=1;' + negative + '\n',
        panic(1, negative) + header,
        panic(1, negative).replace(':Cos', ':fixture-row=1;' + negative),
    ]
    for transcript in refused:
        try:
            classify_failure(transcript, 'watched', rows)
        except AssertionError:
            pass
        else:
            raise AssertionError('fault classifier accepted unrelated/ambiguous fixture evidence')
    print('fault classifier:3 exact ordinal controls/2 legacy false-admissions/%d refusals;rc=0'
          % len(refused), flush=True)


def main(config=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--plan-only', action='store_true')
    args = parser.parse_args()
    config = dict(entry=Path(__file__), source=SOURCE, fixture=FIXTURE, run=RUN,
        faults=FAULTS, test=TEST, target='formula_root_value_contract', rows=85, family='root') if config is None else config
    assert set(config) == {'entry','source','fixture','run','faults','test','target','rows','family'}
    kernel_source, fixture_path, template = config['source'], config['fixture'], config['run']
    profile = runpy.run_path(str(PROFILE_SOURCE))
    # Legacy paths are provenance labels only; no replay writes into an earlier capture.
    relative_template = Path(profile['producer_output_path'](ROOT, template)).relative_to('target')
    stamp = datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%S%fZ') + '-' + str(os.getpid())
    run_dir = ROOT / 'target/irrational_platform_certificate/native-body-faults' / \
        relative_template.parent / (relative_template.name + '-' + stamp)
    faults, test_name, test_target = config['faults'], config['test'], config['target']
    labels = ('baseline', *(label for label, _, _ in faults))
    copy_root, report_path = run_dir / 'workspace', run_dir / 'report.json'
    initial = (Path(__file__), config['entry'], PROFILE_SOURCE, kernel_source, fixture_path, ROOT / 'Cargo.toml',
        ROOT / 'Cargo.lock', ROOT / 'rust-toolchain.toml')
    logs = tuple(run_dir / (label + '.log') for label in labels)
    negative, positive = run_dir / 'timeout-parent-only.txt', run_dir / 'timeout-group.txt'
    control_dir = run_dir / 'capture-controls'
    control_bases = tuple(control_dir / label for label in ('guarded', 'reuse-omitted', 'exclusive-omitted'))
    control_outputs = tuple(base / 'report.json' for base in control_bases)
    # Guard the fresh roots before even read-only inventory dispatch. The full file plan below
    # is derived without creation;all paths are validated again before any snapshot/build write.
    profile['plan_producer'](ROOT, sources=initial, directories=(run_dir, copy_root, ROOT / 'target'),
        output_files=(*logs, report_path, negative, positive, *control_outputs))
    inventory = subprocess.run(['git', 'ls-files', 'crates'], cwd=ROOT,
        check=True, capture_output=True, text=True).stdout.splitlines()
    snapshot = tuple(sorted({ROOT / name for name in inventory} | {
        ROOT / name for name in ('Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml')}))
    sources = tuple(sorted(set(initial) | set(snapshot)))
    destinations = tuple(copy_root / path.relative_to(ROOT) for path in snapshot)
    outputs = (*destinations, *logs, report_path, negative, positive, *control_outputs)
    directories = tuple(sorted({run_dir, copy_root, ROOT / 'target', *(path.parent for path in destinations)}))
    profile['plan_producer'](ROOT, sources=sources,
        directories=(*directories, control_dir, *control_bases), output_files=outputs)
    before_hashes = {path.relative_to(ROOT).as_posix(): digest(path) for path in sources}
    original, fixture = kernel_source.read_bytes(), fixture_path.read_bytes()
    rows = fixture.decode().splitlines()[1:]
    assert len(rows) == config['rows']
    text = original.decode()
    assert all(text.count(before) == 1 and before != after for _, before, after in faults)
    classifier_controls()
    assert not run_dir.exists(), 'fresh capture required;existing captures are immutable'
    if args.plan_only:
        print('native %s fault plan:%d variants/%d exact snapshot files/%d complete outputs;'
            'no writes or compiler dispatch;rc=0' % (config['family'], len(labels), len(snapshot), len(outputs)))
        return
    profile['enter_producer'](ROOT, sources=sources, directories=(run_dir.parent,), output_files=outputs)
    owner = reserve_capture(run_dir)
    profile['enter_producer'](ROOT, sources=sources, directories=directories, output_files=outputs)
    capture_result = capture_controls(profile, control_dir)
    timeout_result = timeout_control(profile, negative, positive)
    for source, destination in zip(snapshot, destinations):
        profile['plan_producer'](ROOT, sources=(source,), output_files=(destination,))
        destination.write_bytes(source.read_bytes())
    assert all(digest(destination) == before_hashes[source.relative_to(ROOT).as_posix()]
        for source, destination in zip(snapshot, destinations)), 'exact original workspace snapshot'
    body = copy_root / kernel_source.relative_to(ROOT)
    compiler = run_group(['rustc', '-vV'], timeout=30)
    assert compiler.returncode == 0, 'actual compiler identity'
    outcomes = []
    try:
        for label, log in zip(labels, logs):
            variant = original
            if label != 'baseline':
                _, before, after = next(fault for fault in faults if fault[0] == label)
                variant = text.replace(before, after, 1).encode()
            profile['plan_producer'](ROOT, sources=sources, output_files=(body, log))
            body.write_bytes(variant)
            profile['plan_producer'](ROOT, sources=destinations, output_files=(log,))
            result = run_group(['cargo', 'test', '--offline', '--locked', '--manifest-path',
                str(copy_root / 'Cargo.toml'), '-p', 'sc-core', '--test', test_target,
                '--message-format=json', test_name, '--', '--nocapture'], timeout=180)
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
            assert 'running 1 test' in transcript and test_name in transcript, 'actual watched native test'
            failed_rows, independent_failure = [], None
            if label == 'baseline':
                assert result.returncode == 0 and '1 passed; 0 failed' in transcript
                assert digest(body) == before_hashes[kernel_source.relative_to(ROOT).as_posix()]
            else:
                assert result.returncode != 0 and '1 failed' in transcript, \
                    'actual compiled native fault survived:' + label
                ordinal, independent_failure = classify_failure(result.stderr, test_name, rows)
                failed_rows = [ordinal]
            binaries = [Path(item['executable']) for item in messages
                if item.get('reason') == 'compiler-artifact' and item.get('target', {}).get('name') ==
                test_target and item.get('executable')]
            assert len(binaries) == 1, 'published Cargo test artifact identity'
            profile['plan_producer'](ROOT, sources=(binaries[0],))
            outcomes.append({'label': label, 'cargo_rc': result.returncode, 'compile_succeeded': True,
                'body_sha256': digest(body), 'failed_fixture_rows': failed_rows,
                'independent_failure': independent_failure, 'binary': binaries[0].relative_to(ROOT).as_posix(),
                'binary_sha256': digest(binaries[0]), 'log': log.relative_to(ROOT).as_posix()})
            print('native ' + config['family'] + ' body:' + label + (' baseline passes' if label == 'baseline'
                else ' actual compiled test rejects fault'), flush=True)
    finally:
        profile['plan_producer'](ROOT, output_files=(body,))
        body.write_bytes(original)
    assert {path.relative_to(ROOT).as_posix(): digest(path) for path in sources} == before_hashes
    assert all(digest(destination) == before_hashes[source.relative_to(ROOT).as_posix()]
        for source, destination in zip(snapshot, destinations)), 'all snapshot bytes restored'
    assert {path for path in copy_root.rglob('*') if path.is_file()} == set(destinations), \
        'complete actual snapshot output census'
    report = {'scope': 'actual copied native ' + config['family'] + ' bodies;finite sourced/storage fault check only',
        'rows': config['rows'],
        'original_sources_unchanged': before_hashes, 'snapshot_restored': True,
        'compiler': compiler.stdout, 'fixture_sha256': digest(fixture_path), 'outcomes': outcomes,
        'timeout_control': timeout_result,
        'capture_controls': capture_result,
        'legacy_capture_label': relative_template.as_posix(),
        'capture': run_dir.relative_to(ROOT).as_posix(),
        'planned_outputs': [path.relative_to(ROOT).as_posix() for path in outputs]}
    profile['plan_producer'](ROOT, output_files=(report_path,))
    write_report(report_path, report, owner)
    print('retained native capture:' + report_path.relative_to(ROOT).as_posix(), flush=True)
    print('native %s body faults:baseline1/actual compiled faults%d/all original and snapshot hashes unchanged;rc=0' % (config['family'], len(faults)))


if __name__ == '__main__':
    main()
