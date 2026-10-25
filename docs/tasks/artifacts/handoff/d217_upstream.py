"""Verify, calibrate and package D217's exact upstream patch in private state.

Activation is interactive and refuses inside the active harness's ancestry.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import runpy
import shutil
import stat
import subprocess
import tarfile

ROOT = Path(__file__).resolve().parents[4]
PROFILE = ROOT / 'scripts/local_environment.py'
PATCH = Path(__file__).with_name('d217_daemon_fix.patch')
REVISION = '6aaac694803004a63c6f393d153c96d22521a6b9'
ARCHIVE_SHA = '07197c09e577d0a4634bbfa3f451e012e4743430afe61b5925f6501c72b32f90'
WORK = ROOT / 'target/artifact_cleanup_audit/d217-upstream-0164'
SOURCE = WORK / 'source' / ('codex-' + REVISION)
FILES = ('codex-rs/app-server-daemon/Cargo.toml',
         'codex-rs/app-server-daemon/src/backend/pid_start.rs')
INSTALLED = Path.home() / '.codex/packages/app-server-daemon/releases/0.164.0-aarch64-apple-darwin'
CANDIDATE = WORK / 'candidate-package'


def file_digest(path):
    value = hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b''):
            value.update(block)
    return value.hexdigest()


def patch_paths(text):
    pairs = [line for line in text.splitlines() if line.startswith(('--- ', '+++ '))]
    expected = [prefix + side + name for name in FILES
                for prefix, side in [('--- ', 'a/'), ('+++ ', 'b/')]]
    if pairs != expected or len(text.encode()) > 16384:
        raise ValueError('patch must select only the two declared upstream inputs')


def setup():
    for path in (ROOT / 'scripts', PROFILE, Path(__file__), PATCH):
        if path.is_symlink() or path.stat().st_dev != ROOT.stat().st_dev:
            raise RuntimeError('unsafe upstream diagnostic bootstrap')
    p = runpy.run_path(str(PROFILE))
    sources = (Path(__file__), PROFILE, PATCH, WORK / 'source.tar.gz',
               *(SOURCE / name for name in FILES))
    p['plan_producer'](ROOT, sources=sources)
    return p, sources


def verify(profile, sources):
    patch = PATCH.read_text()
    patch_paths(patch)
    archive = WORK / 'source.tar.gz'
    assert hashlib.sha256(archive.read_bytes()).hexdigest() == ARCHIVE_SHA
    mirror = WORK / ('patch-proof-' + str(os.getpid()))
    if mirror.exists() or mirror.is_symlink():
        raise RuntimeError('patch proof fixture must be absent')
    outputs = tuple(mirror / name for name in FILES)
    profile['prepare_producer'](ROOT, sources=sources,
        directories=tuple(p.parent for p in outputs), output_files=outputs)
    with tarfile.open(archive) as reader:
        for name, output in zip(FILES, outputs):
            with output.open('xb') as stream:
                stream.write(reader.extractfile('codex-' + REVISION + '/' + name).read())
        unchanged = 0
        for member in reader.getmembers():
            if not member.isfile():
                continue
            name = member.name.split('/', 1)[1]
            if name in (*FILES, 'codex-rs/Cargo.lock'):
                continue
            actual = SOURCE / name
            assert actual.is_file() and not actual.is_symlink(), name
            assert hashlib.sha256(reader.extractfile(member).read()).hexdigest() == file_digest(actual), name
            unchanged += 1
    env = profile['environment_for'](ROOT, os.environ, create=False)
    argv = ['git', 'apply', '--no-index', str(PATCH)]
    subprocess.run([*argv[:2], '--check', *argv[2:]], cwd=mirror, env=env, check=True)
    subprocess.run(argv, cwd=mirror, env=env, check=True)
    for name, output in zip(FILES, outputs):
        assert output.read_bytes() == (SOURCE / name).read_bytes(), name
    return dict(revision=REVISION, archive_sha256=ARCHIVE_SHA,
                patch_sha256=hashlib.sha256(PATCH.read_bytes()).hexdigest(),
                dependency_lock_sha256=file_digest(SOURCE / 'codex-rs/Cargo.lock'),
                unchanged_release_files=unchanged,
                complete_upstream_inputs=len(FILES), independent_git_apply=True)


def package(profile, sources):
    binary = ROOT / 'target/d217-codex-build/release/codex'
    if CANDIDATE.exists() or CANDIDATE.is_symlink():
        raise RuntimeError('candidate package must be absent')
    assert file_digest(INSTALLED / 'bin/codex') == (
        'd8990e3ee7acfbc26f91265b12a1a83f0fee40d0e0f8f1f40a37d4ec21f499cf')
    selected = {}
    for path in sorted(INSTALLED.rglob('*')):
        name = path.relative_to(INSTALLED)
        if name.as_posix() == 'codex' and path.is_symlink() and os.readlink(path) == 'bin/codex':
            continue
        if path.is_symlink():
            raise RuntimeError('unreviewed source package symlink')
        if path.is_file():
            selected[name] = binary if name.as_posix() == 'bin/codex' else path
        elif not path.is_dir():
            raise RuntimeError('nonregular source package')
    outputs = tuple(CANDIDATE / name for name in selected)
    provenance = CANDIDATE / 'd217-build.json'
    profile['prepare_producer'](ROOT, sources=(*sources, binary),
        directories=tuple(p.parent for p in (*outputs, provenance)),
        output_files=(*outputs, provenance))
    copied = {}
    for name, source in selected.items():
        dest = CANDIDATE / name
        before = file_digest(source)
        with source.open('rb') as src, dest.open('xb') as dst:
            shutil.copyfileobj(src, dst, 1024 * 1024)
        dest.chmod(stat.S_IMODE(source.stat().st_mode))
        assert file_digest(dest) == before and file_digest(source) == before
        copied[name.as_posix()] = before
    voice_name = 'codex-resources/voice/manifest.json'
    voice = CANDIDATE / voice_name
    metadata = json.loads(voice.read_text())
    assert metadata['sha256']['bin/codex'] == file_digest(INSTALLED / 'bin/codex')
    metadata['sha256']['bin/codex'] = copied['bin/codex']
    profile['plan_producer'](ROOT, output_files=(voice,))
    voice.write_text(json.dumps(metadata, indent=2) + '\n')
    copied[voice_name] = file_digest(voice)
    for name, expected in metadata['sha256'].items():
        assert '..' not in Path(name).parts and not Path(name).is_absolute()
        assert file_digest(CANDIDATE / name) == expected
    record = dict(revision=REVISION, patch_sha256=file_digest(PATCH), files=copied,
                  package_kind='locally patched diagnostic candidate')
    with provenance.open('x') as stream:
        json.dump(record, stream, indent=2, sort_keys=True)
        stream.write('\n')
    return dict(package=CANDIDATE.relative_to(ROOT).as_posix(), files=len(copied),
                binary_sha256=copied['bin/codex'], metadata=provenance.relative_to(ROOT).as_posix())


def verify_package(profile):
    record = CANDIDATE / 'd217-build.json'
    profile['plan_producer'](ROOT, sources=(Path(__file__), PATCH, record))
    expected = json.loads(record.read_text())
    if expected['revision'] != REVISION or expected['patch_sha256'] != file_digest(PATCH):
        raise RuntimeError('candidate source/patch provenance changed')
    files = expected['files']
    actual = {p.relative_to(CANDIDATE).as_posix() for p in CANDIDATE.rglob('*')
              if p.is_file() and p != record}
    if set(files) != actual:
        raise RuntimeError('candidate package membership changed')
    for name, value in files.items():
        if Path(name).is_absolute() or '..' in Path(name).parts:
            raise RuntimeError('candidate file escapes package')
        path = CANDIDATE / name
        profile['plan_producer'](ROOT, sources=(path,))
        if file_digest(path) != value:
            raise RuntimeError('candidate file bytes changed: ' + name)
    if files['bin/codex'] != file_digest(ROOT / 'target/d217-codex-build/release/codex'):
        raise RuntimeError('candidate differs from tested release executable')
    return expected


def require_external_terminal():
    # This shared-service replacement must originate outside the active harness.
    state = Path.home() / '.codex/app-server-daemon/daemon.pid'
    current = json.loads(state.read_text())['pid'] if state.is_file() else None
    if current is not None and (type(current) is not int or current <= 1):
        raise RuntimeError('invalid managed daemon identity')
    pid, ancestry = os.getpid(), set()
    while pid > 1:
        if pid in ancestry:
            raise RuntimeError('cyclic caller ancestry')
        ancestry.add(pid)
        pid = int(subprocess.check_output(['ps', '-p', str(pid), '-o', 'ppid='], text=True))
    if current in ancestry:
        raise RuntimeError('activation requires a separate Terminal; active harness would be stopped')


def activate(profile):
    require_external_terminal()
    record = verify_package(profile)
    expected = 'd8990e3ee7acfbc26f91265b12a1a83f0fee40d0e0f8f1f40a37d4ec21f499cf'
    if file_digest(INSTALLED / 'bin/codex') != expected:
        raise RuntimeError('original vendor executable changed')
    current = INSTALLED.parent.parent / 'current/bin/codex'
    if file_digest(current) != expected:
        raise RuntimeError('selected vendor package changed; rebuild and review the activation plan')
    print('Verified local D217 patch:', record['files']['bin/codex'], flush=True)
    print('The native installer will ask to replace and pin the shared daemon package.', flush=True)
    print('This interrupts Codex sessions and pauses production automatic updates until daemon update.',
          flush=True)
    # Native confirmation stays interactive. No automatic --yes for the real home.
    subprocess.run([str(CANDIDATE / 'bin/codex'), 'app-server', 'daemon', 'update', '--from-cli'],
                   close_fds=True, check=True)


def rehearse(profile, sources):
    probe_source = Path(__file__).with_name('d217_probe.py')
    daemon_source = Path(__file__).with_name('d217_daemon.py')
    profile['plan_producer'](ROOT, sources=(*sources, probe_source, daemon_source))
    verify_package(profile)
    probe, daemon = runpy.run_path(str(probe_source)), runpy.run_path(str(daemon_source))
    base = WORK / ('installation-rehearsal-' + str(os.getpid()))
    if base.exists() or base.is_symlink():
        raise RuntimeError('installation rehearsal must be absent')
    home = base / 'private-home'
    package_root = home / 'packages/app-server-daemon'
    release = package_root / 'releases/0.164.0-aarch64-apple-darwin'
    old_binary = release / 'bin/codex'
    state = home / 'app-server-daemon'
    settings, marker = state / 'settings.json', package_root / 'auto-update-version'
    watched = (base / 'original-watched', base / 'replacement-watched')
    receipt = base / 'receipt.json'
    plan = dict(sources=(*sources, probe_source, daemon_source, CANDIDATE / 'd217-build.json'),
                directories=(home, release / 'bin', state, *watched),
                output_files=(old_binary, settings, marker, receipt))
    env = profile['prepare_producer'](ROOT, **plan)
    with (INSTALLED / 'bin/codex').open('rb') as src, old_binary.open('xb') as dst:
        shutil.copyfileobj(src, dst, 1024 * 1024)
    old_binary.chmod(0o755)
    assert file_digest(old_binary) == file_digest(INSTALLED / 'bin/codex')
    (package_root / 'current').symlink_to(release, target_is_directory=True)
    marker.write_text(release.name)
    settings.write_text(json.dumps(dict(shutdownGraceSeconds=1)))
    result, records, descriptors = dict(complete=False), [], []
    lib = probe['library']()
    import fcntl
    try:
        fd = os.open(watched[0], os.O_EVTONLY)
        descriptors.append(fd)
        descriptor = fcntl.fcntl(fd, fcntl.F_DUPFD_CLOEXEC, 64)
        descriptors.append(descriptor)
        daemon['cli'](old_binary, home, ['start'], env, result, descriptor)
        old = [daemon['read_record'](home, name) for name in ('daemon', 'daemon-updater')]
        records.extend((r, old_binary) for r in old)
        daemon['wait_updater_ready'](home)
        for fd in descriptors:
            os.close(fd)
        descriptors.clear()
        profile['plan_producer'](ROOT, directories=(watched[0],))
        watched[0].rmdir()
        result['before'] = [probe['observe'](lib, r['pid'], descriptor) for r in old]
        assert all(daemon['revoked'](r) for r in result['before'])
        fd = os.open(watched[1], os.O_EVTONLY)
        descriptors.append(fd)
        descriptor = fcntl.fcntl(fd, fcntl.F_DUPFD_CLOEXEC, 64)
        descriptors.append(descriptor)
        result['install'] = daemon['cli'](CANDIDATE / 'bin/codex', home,
            ['update', '--from-cli', '--yes'], env, result, descriptor)
        selected = (package_root / 'current').resolve()
        assert selected.parent == (package_root / 'releases').resolve()
        assert selected.name.startswith('local-') and not marker.exists()
        new_binary = selected / 'bin/codex'
        assert file_digest(new_binary) == file_digest(CANDIDATE / 'bin/codex')
        server = daemon['read_record'](home, 'daemon')
        records.append((server, new_binary))
        assert all(daemon['stopped'](r) for r in old)
        assert not daemon['metadata_ok'](probe['observe'](lib, server['pid'], descriptor))
        for fd in descriptors:
            os.close(fd)
        descriptors.clear()
        profile['plan_producer'](ROOT, directories=(watched[1],))
        watched[1].rmdir()
        result['after'] = daemon['complete_snapshot'](probe, lib, [server['pid']], result)
        result['local_pin'] = selected.name
        result['version'] = daemon['cli'](new_binary, home, ['version'], env, result)
        result['rollback'] = daemon['cli'](INSTALLED / 'bin/codex', home,
            ['update', '--from-cli', '--yes'], env, result)
        rollback_binary = (package_root / 'current/bin/codex').resolve()
        rollback = daemon['read_record'](home, 'daemon')
        records.append((rollback, rollback_binary))
        assert daemon['stopped'](server)
        assert file_digest(rollback_binary) == file_digest(INSTALLED / 'bin/codex')
        result['rollback_exact'] = True
        result['complete'] = True
    finally:
        for fd in descriptors:
            os.close(fd)
        current = state / 'daemon.pid'
        if current.is_file():
            record = json.loads(current.read_text())
            binary = (package_root / 'current/bin/codex').resolve()
            if not any(r == record for r, _ in records):
                records.append((record, binary))
        for record, binary in reversed(records):
            daemon['terminate_owned'](record, binary)
        result['all_owned_stopped'] = all(daemon['stopped'](r) for r, _ in records)
        profile['plan_producer'](ROOT, output_files=(receipt,))
        with receipt.open('x') as output:
            json.dump(result, output, indent=2, sort_keys=True)
            output.write('\n')
    assert result['all_owned_stopped']
    return dict(rehearsal=receipt.relative_to(ROOT).as_posix(), complete=result['complete'],
                rollback_exact=result['rollback_exact'], all_owned_stopped=True)


def calibrate(profile, sources):
    source = SOURCE / FILES[1]
    original = source.read_text()
    target = '    close_fds::set_fds_cloexec(3, &[]);'
    assert original.count(target) == 1
    logbase = WORK / ('guard-proof-' + str(os.getpid()))
    logs = (logbase.with_suffix('.omitted.log'), logbase.with_suffix('.restored.log'))
    plan = dict(sources=sources, mutable_sources=(source,), output_files=logs)
    env = profile['prepare_producer'](ROOT, **plan)
    env.update(RUSTUP_TOOLCHAIN='1.99.1', CARGO_TARGET_DIR=str(ROOT / 'target/d217-codex-build'))
    argv = ['cargo', 'test', '--manifest-path', str(SOURCE / 'codex-rs/Cargo.toml'),
            '-p', 'codex-app-server-daemon', 'inherited_fd_tests']
    try:
        profile['plan_producer'](ROOT, **plan)
        source.write_text(original.replace(target, '    // Actual containment omitted for calibration.'))
        with logs[0].open('x') as output:
            red = subprocess.run(argv, cwd=SOURCE / 'codex-rs', env=env,
                                 stdout=output, stderr=subprocess.STDOUT)
        text = logs[0].read_text()
        assert red.returncode == 101 and 'left != right' in text and 'FAILED' in text, text[-4000:]
    finally:
        profile['plan_producer'](ROOT, **plan)
        source.write_text(original)
    with logs[1].open('x') as output:
        green = subprocess.run(argv, cwd=SOURCE / 'codex-rs', env=env,
                               stdout=output, stderr=subprocess.STDOUT)
    assert green.returncode == 0 and '3 passed; 0 failed' in logs[1].read_text()
    assert source.read_text() == original
    return dict(actual_compiled_guard_red=True, restored_tests=3, source_unchanged=True,
                logs=[p.relative_to(ROOT).as_posix() for p in logs])


def self_test():
    valid = '\n'.join(prefix + side + name for name in FILES
                      for prefix, side in [('--- ', 'a/'), ('+++ ', 'b/')])
    patch_paths(valid)
    for bad in (valid.replace('a/codex-rs/', 'a/../'), valid + '\n+++ b/extra',
                valid.replace('pid_start.rs', 'pid.rs'), valid + 'x' * 16385):
        try:
            patch_paths(bad)
        except ValueError:
            continue
        raise AssertionError('unsafe patch selector accepted')
    print(json.dumps(dict(patch_selector_refusals=4)))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('mode', choices=('verify', 'calibrate', 'package', 'rehearse',
                                        'activate', 'activation-guard', 'self-test'))
    args = parser.parse_args()
    if args.mode == 'self-test':
        self_test()
    elif args.mode == 'activation-guard':
        require_external_terminal()
        print('activation guard: external caller verified')
    else:
        profile, sources = setup()
        if args.mode == 'activate':
            activate(profile)
            raise SystemExit(0)
        result = verify(profile, sources)
        if args.mode == 'calibrate':
            result.update(calibrate(profile, sources))
        elif args.mode == 'package':
            result.update(package(profile, sources))
        elif args.mode == 'rehearse':
            result.update(rehearse(profile, sources))
        print(json.dumps(result, sort_keys=True))
