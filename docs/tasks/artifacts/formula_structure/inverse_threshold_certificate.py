"""Source-locked resumable inverse certificate controller; native branch admission is separate."""
from contextlib import contextmanager
from pathlib import Path
import argparse
import json
import os
import runpy
import shutil
import stat
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[4]
HERE = Path(__file__)
PROFILE = ROOT / 'scripts/local_environment.py'
READER = HERE.with_name('verify_inverse_certificate.py')
NATIVE = ROOT / 'crates/sc-core/examples/inverse_threshold_chunk.rs'
PI = ROOT / 'crates/sc-core/tests/fixtures/pi640_certificate.tsv'
RUN = ROOT / 'target/inverse_threshold_certificate/0284'
MANIFEST = RUN / 'manifest.json'
PENDING = RUN / 'manifest.pending'
LOCK = RUN / 'active.lock'
ACTIVE = RUN / 'active.json'
PROTOCOL = RUN / 'pi-protocol.txt'
BUILD_LOG = RUN / 'build.log'
PRODUCER = RUN / ('producer.exe' if os.name == 'nt' else 'producer')


def paths(index):
    return (RUN / ('chunk-%03d.part' % index), RUN / ('chunk-%03d.bin' % index),
            RUN / ('chunk-%03d.stderr' % index))


@contextmanager
def exclusive_run():
    with LOCK.open('x') as stream:
        stream.write(str(os.getpid()) + '\n')
    try:
        yield
    finally:
        LOCK.unlink()


def stop_children(processes):
    """Terminate only processes started and retained by this controller; reap every owned child."""
    processes = list(processes)
    for process in processes:
        if process.poll() is None:
            process.terminate()
    for process in processes:
        try:
            process.wait(timeout=5)
        except subprocess.TimeoutExpired:
            process.kill()
            process.wait()


def write_json(profile, path, value):
    profile['plan_producer'](ROOT, output_files=(path,))
    path.write_text(json.dumps(value, indent=2) + '\n')


def publish(profile, manifest):
    write_json(profile, PENDING, manifest)
    profile['plan_producer'](ROOT, output_files=(PENDING, MANIFEST))
    PENDING.replace(MANIFEST)


def capture_and_build(profile, reader):
    inventory = subprocess.run(['git', 'ls-files', '-z', '*.rs', '**/Cargo.toml'], cwd=ROOT,
                               check=True, capture_output=True).stdout.decode().split('\0')
    sources = {ROOT / name for name in inventory if name}
    sources.update((HERE, READER, NATIVE, PROFILE, PI, ROOT / 'Cargo.toml', ROOT / 'Cargo.lock',
                    ROOT / 'rust-toolchain.toml', HERE.with_name('pi_rounding_certificate.py')))
    config = ROOT / '.cargo/config.toml'
    if config.exists():
        sources.add(config)
    sources = sorted(sources)
    frozen_paths = [RUN / 'sources' / path.relative_to(ROOT) for path in sources]
    profile['plan_producer'](ROOT, sources=sources, directories=tuple(p.parent for p in frozen_paths),
                              output_files=(*frozen_paths, PRODUCER, BUILD_LOG))
    hashes = {p.relative_to(ROOT).as_posix(): reader['digest'](p) for p in sources}
    profile['enter_producer'](ROOT, sources=sources, directories=tuple(p.parent for p in frozen_paths),
                               output_files=(*frozen_paths, PRODUCER, BUILD_LOG))
    for source, frozen in zip(sources, frozen_paths):
        profile['plan_producer'](ROOT, output_files=(frozen,))
        if frozen.exists():
            assert reader['digest'](frozen) == hashes[source.relative_to(ROOT).as_posix()], 'unfinished snapshot differs'
        else:
            shutil.copyfile(source, frozen)
        assert reader['digest'](frozen) == hashes[source.relative_to(ROOT).as_posix()]
    build = subprocess.run(['cargo', 'build', '--offline', '-p', 'sc-core', '--release',
                            '--example', 'inverse_threshold_chunk', '--message-format=json'],
                           cwd=ROOT, capture_output=True, text=True)
    profile['plan_producer'](ROOT, output_files=(BUILD_LOG,))
    BUILD_LOG.write_text((build.stdout + build.stderr).replace(str(ROOT), '.'))
    assert build.returncode == 0, 'native build failed; see project-local build.log'
    binaries = [Path(item['executable']) for line in build.stdout.splitlines()
                if (item := json.loads(line)).get('reason') == 'compiler-artifact'
                and item.get('target', {}).get('name') == 'inverse_threshold_chunk' and item.get('executable')]
    assert len(binaries) == 1, 'actual Cargo artifact identity'
    binary = binaries[0]
    assert binary.relative_to(ROOT).parts[0] == 'target'
    profile['plan_producer'](ROOT, sources=(binary,), output_files=(PRODUCER,))
    shutil.copyfile(binary, PRODUCER)
    PRODUCER.chmod(stat.S_IMODE(binary.stat().st_mode) & 0o755)
    assert reader['digest'](PRODUCER) == reader['digest'](binary), 'producer copy mismatch'
    assert {p.relative_to(ROOT).as_posix(): reader['digest'](p) for p in sources} == hashes, 'build source drift'
    return {'schema': 1, 'scope': 'inverse_threshold_population', 'complete': False,
            'parameters': reader['PARAMETERS'], 'producer_file': PRODUCER.name,
            'producer_sha256': reader['digest'](PRODUCER), 'frozen_sources': hashes,
            'native_platform': sys.platform, 'chunks': []}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--chunks', type=int, default=450, help='maximum new chunks; partial never means complete')
    parser.add_argument('--workers', type=int, default=min(4, os.cpu_count() or 1))
    args = parser.parse_args()
    assert 1 <= args.chunks <= 450 and 1 <= args.workers <= 4
    profile = runpy.run_path(str(PROFILE))
    initial = (HERE, PROFILE, READER, NATIVE, PI)
    outputs = (MANIFEST, PENDING, LOCK, ACTIVE, PROTOCOL, BUILD_LOG, PRODUCER,
               *(path for index in range(450) for path in paths(index)))
    profile['plan_producer'](ROOT, sources=initial, directories=(RUN, ROOT / 'target'), output_files=outputs)
    reader = runpy.run_path(str(READER))
    profile['enter_producer'](ROOT, sources=initial, directories=(RUN, ROOT / 'target'), output_files=outputs)
    with exclusive_run():
        if MANIFEST.exists():
            assert MANIFEST.stat().st_size <= 1_000_000
            manifest = json.loads(MANIFEST.read_text())
            reader['verify_manifest'](manifest)
            reader['verify_sources'](manifest, profile)
        else:
            manifest = capture_and_build(profile, reader)
            reader['verify_manifest'](manifest)
            publish(profile, manifest)
        assert manifest['native_platform'] == sys.platform, 'partial native resume requires its original platform'
        # Recheck every published chunk before any new dispatch; no checkpoint is trusted by name.
        for chunk in manifest['chunks']:
            path = RUN / chunk['file']
            profile['plan_producer'](ROOT, sources=(path,))
            actual, _, _ = reader['inspect_chunk'](path)
            assert all(actual[key] == chunk[key] for key in actual), 'published chunk drift'
        print('inverse resume: %d verified chunks' % len(manifest['chunks']), flush=True)
        pi_rows = dict(line.split('\t') for line in PI.read_text().splitlines() if line and not line.startswith('#'))
        assert pi_rows['fraction_bits'] == '640'
        protocol = (pi_rows['pi_lower_hex'] + '\n' + pi_rows['pi_upper_hex'] + '\n').encode()
        profile['plan_producer'](ROOT, output_files=(PROTOCOL,))
        PROTOCOL.write_bytes(protocol)
        done = {chunk['index'] for chunk in manifest['chunks']}
        remaining = iter([index for index in range(450) if index not in done][:args.chunks])
        active = {}
        exhausted = False
        try:
            while active or not exhausted:
                while len(active) < args.workers and not exhausted:
                    index = next(remaining, None)
                    if index is None:
                        exhausted = True
                        break
                    reader['verify_sources'](manifest, profile)
                    partial, final, stderr = paths(index)
                    profile['plan_producer'](ROOT, sources=(PRODUCER, PROTOCOL), output_files=(partial, final, stderr))
                    started = time.perf_counter_ns()
                    with partial.open('wb') as out, stderr.open('wb') as err, PROTOCOL.open('rb') as inp:
                        process = subprocess.Popen([str(PRODUCER), str(index * 100_000), '100000'],
                                                   stdin=inp, stdout=out, stderr=err, cwd=ROOT)
                    active[index] = (process, started)
                    write_json(profile, ACTIVE, {'controller_pid': os.getpid(),
                               'children': {str(i): entry[0].pid for i, entry in active.items()}})
                finished = [index for index, (process, _) in active.items() if process.poll() is not None]
                if not finished:
                    if active:
                        time.sleep(0.2)
                    continue
                for index in finished:
                    process, started = active[index]
                    assert process.wait() == 0, ('native chunk failed', index)
                    partial, final, stderr = paths(index)
                    profile['plan_producer'](ROOT, sources=(partial,), output_files=(partial, final))
                    actual, _, _ = reader['inspect_chunk'](partial)
                    reader['verify_sources'](manifest, profile)
                    if final.exists():
                        assert reader['digest'](final) == actual['sha256'], 'unpublished prior output differs; preserved'
                        partial.unlink()
                    else:
                        partial.replace(final)
                    manifest['chunks'].append({'index': index, 'file': final.name, 'start': index * 100_000,
                                               'count': 100_000, 'elapsed_ns': time.perf_counter_ns() - started, **actual})
                    manifest['chunks'].sort(key=lambda chunk: chunk['index'])
                    manifest['complete'] = len(manifest['chunks']) == 450
                    reader['verify_manifest'](manifest)
                    publish(profile, manifest)
                    del active[index]
                    write_json(profile, ACTIVE, {'controller_pid': os.getpid(),
                               'children': {str(i): entry[0].pid for i, entry in active.items()}})
                    print('inverse coverage: %d/45000000; chunks%d; complete=%s' %
                          (len(manifest['chunks']) * 100_000, len(manifest['chunks']), manifest['complete']), flush=True)
        finally:
            stop_children(entry[0] for entry in active.values())
            write_json(profile, ACTIVE, {'controller_pid': os.getpid(), 'children': {}})
        reader['verify_sources'](manifest, profile)
        print('inverse result: %s; rows%d; rc=0' %
              ('COMPLETE' if manifest['complete'] else 'PARTIAL', len(manifest['chunks']) * 100_000), flush=True)


if __name__ == '__main__':
    main()
