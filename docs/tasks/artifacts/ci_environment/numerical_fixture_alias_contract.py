"""Fixture-owned physical alias refusals and optional complete modeled replay; no numeric proof."""
from pathlib import Path
import argparse
import contextlib
import hashlib
import io
import json
import os
import runpy
import sys

ROOT = Path(__file__).resolve().parents[4]
ENTRY = Path(__file__).with_name('python_numerical_runtime_contract.py')
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
PROFILE['plan_producer'](ROOT, sources=(Path(__file__), ENTRY, PROFILE_SOURCE))
SOURCE = ENTRY.read_text()


def module(script):
    scope = dict(__file__=str(ENTRY), __name__='numerical_fixture_alias_control')
    exec(compile(script, str(ENTRY), 'exec'), scope)
    return scope


def restored(scope):
    assert all(path.read_bytes() == data for path,data in scope['FROZEN'].items())
    for workspace in scope['WORKSPACES']:
        actual = {path.relative_to(workspace) for path in workspace.rglob('*') if path.is_file()}
        expected = {path.relative_to(ROOT) for path in scope['SNAPSHOT']}
        if actual != expected:
            # The deliberate session-guard omission refuses its real timeout child before copying.
            assert workspace.parent.parent.name == 'fault-session'
            assert not actual and not (workspace.parent / 'report.json').exists()
            continue
        assert all((workspace / path.relative_to(ROOT)).read_bytes() == scope['FROZEN'][path]
                   for path in scope['SNAPSHOT'])


def alias_case(label, *, script=SOURCE):
    saved = dict(os.environ)
    error = None
    try:
        scope = module(script)
        sentinel = scope['WORK'] / 'controlled-sentinel'
        if label.startswith('artifact'):
            _, config = scope['configuration'](scope['DRIVER'], 'alias-control')
            variant = 'baseline' if label == 'artifact-first-symlink' else config['faults'][-1][0]
            target = scope['WORK'] / 'caller-stores/cargo_target_dir/SIMULATED-artifacts/alias-control' / ('root-' + variant)
            counterpart = None
        else:
            case = 'source-link' if label == 'kernel-symlink' else 'source-missing'
            folder = scope['WORK'] / 'preflight-fixtures' / case
            target = folder / ('kernel.rs' if label == 'kernel-symlink' else 'fixture.tsv')
            counterpart = folder / ('fixture.tsv' if label == 'kernel-symlink' else 'kernel.rs')
        PROFILE['prepare_producer'](ROOT, sources=(ENTRY, Path(__file__)), directories=(target.parent,),
                                   output_files=(target, sentinel))
        sentinel.write_bytes(b'controlled prior artifact bytes\n')
        if label.endswith('symlink'):
            target.symlink_to(sentinel)
        else:
            os.link(sentinel, target)
        configured = dict(os.environ)
        with contextlib.redirect_stdout(io.StringIO()):
            try:
                if label.startswith('artifact'):
                    scope['capture'](scope['DRIVER'], 'alias-control')
                else:
                    scope['preflight'](case)
            except (AssertionError, ValueError) as caught:
                error = caught
        assert sentinel.read_bytes() == b'controlled prior artifact bytes\n', 'fixture output alias changed sentinel'
        assert error is not None, 'fixture output alias accepted'
        expected = 'symlink producer output' if label.endswith('symlink') else 'hard-linked producer output'
        assert expected in str(error), (label, error)
        if counterpart is not None:
            assert not counterpart.exists(), 'fixture wrote another output before complete alias refusal'
        assert dict(os.environ) == configured, 'fixture alias refusal changed caller environment'
        assert not any((path.parent / 'report.json').exists() for path in scope['WORKSPACES'])
        restored(scope)
    finally:
        os.environ.clear();os.environ.update(saved)


def full_replay():
    saved, argv = dict(os.environ), sys.argv
    try:
        scope = module(SOURCE)
        sys.argv = [str(ENTRY)]
        with contextlib.redirect_stdout(io.StringIO()) as output:
            scope['main']()
        restored(scope)
        paths = [workspace.parent / 'report.json' for workspace in scope['WORKSPACES']
                 if (workspace.parent / 'report.json').is_file()]
        assert len(paths) == 9, 'complete original modeled report population changed'
        expected = dict(path=ENTRY.relative_to(ROOT).as_posix(),
                        sha256=hashlib.sha256(ENTRY.read_bytes()).hexdigest(),
                        python=sys.version, implementation=sys.implementation.name)
        for path in paths:
            PROFILE['plan_producer'](ROOT, sources=(path,))
            report = json.loads(path.read_text())
            assert report['simulated_children'] and report['scope'].startswith('SIMULATED')
            assert report['fixture_instrument'] == expected
        assert '30 simulated variants' in output.getvalue() and '27 simulated variants' in output.getvalue()
        print(output.getvalue(), end='')
        print('current full fixture:9 accepted modeled receipts bind emitter;all failed/surviving copies restored;rc=0')
    finally:
        sys.argv = argv
        os.environ.clear();os.environ.update(saved)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--full', action='store_true')
    args = parser.parse_args()
    for label in ('artifact-first-symlink', 'artifact-last-hardlink', 'kernel-symlink', 'fixture-hardlink'):
        alias_case(label)
    guards = (
        ('artifact-first-symlink', "            PROFILE['prepare_producer'](ROOT, sources=CORE, directories=(artifact.parent,), output_files=(artifact,))\n"),
        ('kernel-symlink', "    PROFILE['prepare_producer'](ROOT, sources=CORE, directories=(folder,), output_files=(kernel, fixture))\n"),
    )
    for label, guard in guards:
        assert SOURCE.count(guard) == 1
        try:
            alias_case(label, script=SOURCE.replace(guard, ''))
        except AssertionError as error:
            assert str(error) == 'fixture output alias changed sentinel', error
        else:
            raise AssertionError('actual fixture-owned output omission survived')
    print('fixture-own aliases:4 physical first/last/kernel/fixture refusals;2 actual guard omissions;'
          'sentinels/caller/source/copy bytes exact;no accepted failed receipt;SIMULATED compiler;rc=0')
    if args.full:
        full_replay()


if __name__ == '__main__':
    main()
