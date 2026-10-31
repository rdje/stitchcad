"""Actual modeled receipt publication binds its emitter; no native compiler/browser evidence."""
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


def validate(report):
    assert report['simulated_children'] and report['scope'].startswith('SIMULATED')
    assert report.get('fixture_instrument') == dict(
        path=ENTRY.relative_to(ROOT).as_posix(), sha256=hashlib.sha256(ENTRY.read_bytes()).hexdigest(),
        python=sys.version, implementation=sys.implementation.name), 'modeled fixture emitter identity mismatch'


def capture(label, *, script=SOURCE, drift=False, published_drift=False):
    scope = dict(__file__=str(ENTRY), __name__='numerical_fixture_provenance_control')
    saved = dict(os.environ)
    error = None
    try:
        exec(compile(script, str(ENTRY), 'exec'), scope)
        original = scope['FROZEN'][ENTRY]
        if drift:
            # Change only the controlled in-memory basis;no source file is amended.
            scope['FROZEN'][ENTRY] = b'controlled different fixture source basis\n'
        try:
            with contextlib.redirect_stdout(io.StringIO()):
                try:
                    scope['capture'](scope['DRIVER'], label)
                except AssertionError as caught:
                    error = caught
            assert len(scope['WORKSPACES']) == 1
            workspace = next(iter(scope['WORKSPACES']))
            report_path = workspace.parent / 'report.json'
            PROFILE['plan_producer'](ROOT, directories=(workspace.parent,), output_files=(report_path,))
            if drift:
                assert error is not None
                if published_drift:
                    assert report_path.is_file(), 'actual source-basis guard omission not exposed'
                else:
                    assert str(error) == 'fixture emitter source changed before publication'
                    assert not report_path.exists(), 'changed fixture basis published a receipt'
                return None
            assert error is None, error
            PROFILE['plan_producer'](ROOT, sources=(report_path,))
            return json.loads(report_path.read_text())
        finally:
            scope['FROZEN'][ENTRY] = original
    finally:
        os.environ.clear();os.environ.update(saved)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--diagnose', help='repository-relative old modeled report')
    args = parser.parse_args()
    if args.diagnose:
        name = PROFILE['producer_path'](ROOT, args.diagnose)
        path = ROOT / name
        PROFILE['plan_producer'](ROOT, sources=(path,))
        report = json.loads(path.read_text())
        assert report['simulated_children'] and 'fixture_instrument' not in report
        print('before:modeled receipt records original inputs but omits fixture emitter identity;no writes;rc=0')
        return
    report = capture('provenance-positive')
    validate(report)
    for field in ('path', 'sha256', 'python', 'implementation'):
        changed = dict(report, fixture_instrument=dict(report['fixture_instrument']))
        changed['fixture_instrument'][field] = 'controlled wrong value'
        try:
            validate(changed)
        except AssertionError as error:
            assert str(error) == 'modeled fixture emitter identity mismatch'
        else:
            raise AssertionError('fixture identity field drift accepted: ' + field)
    capture('provenance-drift', drift=True)
    guard = "        assert fixture_source.read_bytes() == FROZEN[fixture_source], 'fixture emitter source changed before publication'\n"
    assert SOURCE.count(guard) == 1
    capture('provenance-guard-omission', script=SOURCE.replace(guard, ''), drift=True, published_drift=True)
    field = 'fixture_instrument=instrument'
    assert SOURCE.count(field) == 1
    omitted = capture('provenance-field-omission', script=SOURCE.replace(field, 'fixture_instrument=None'))
    try:
        validate(omitted)
    except AssertionError as error:
        assert str(error) == 'modeled fixture emitter identity mismatch'
    else:
        raise AssertionError('actual fixture identity omission survived')
    print('modeled fixture provenance:actual emitter source/runtime identity;4 field refusals/'
          'changed-basis no publication/2 actual body omissions;source files unchanged;'
          'SIMULATED compiler only;rc=0')


if __name__ == '__main__':
    main()
