"""Finite controls for the actual seam oracle driver;isolated scratch stays on this volume."""
from pathlib import Path
import hashlib
import os
import re
import shutil
import stat
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[4]
DIRECTORY = Path('docs/tasks/artifacts/seam_partition')
DRIVER = DIRECTORY / 'check_seam_oracles.sh'
RUNNER = DIRECTORY / 'run_seam_partition_probes.sh'
FIXTURES = Path('crates/sc-core/tests/fixtures')
NAMES = (
    'nonuniform_ease', 'weighted_domain_ease', 'notch_context_recovery',
    'notch_quantity_workspace', 'notch_quantities', 'original_comparison',
    'original_projection', 'projection_workspace', 'reference_recomposition',
    'range_progress', 'uniform_ease',
)
CI_LINE = '        run: bash docs/tasks/artifacts/seam_partition/run_seam_partition_probes.sh'
MAKE_DISCOVERY = '-name "run_*probe*.sh"'


def wiring(ci, make, runners):
    assert len(re.findall(r'^' + re.escape(CI_LINE) + r'$', ci, re.M)) == 1, 'actual CI invocation'
    assert MAKE_DISCOVERY in make, 'Make discovers the probe entry'
    assert runners.count(RUNNER) == 1, 'one discovered seam runner'


def main():
    ci = (ROOT / '.github/workflows/doctrines.yml').read_text()
    make = (ROOT / 'Makefile').read_text()
    runners = sorted(p.relative_to(ROOT) for p in (ROOT / 'docs/tasks/artifacts').rglob('run_*probe*.sh'))
    wiring(ci, make, runners)
    sources = [DRIVER] + sorted(p.relative_to(ROOT) for p in (ROOT / DIRECTORY).glob('*_oracle.py'))
    sources += [FIXTURES / (name + '.tsv') for name in NAMES]
    before = {p: hashlib.sha256((ROOT / p).read_bytes()).digest() for p in sources}
    scratch = ROOT
    for component in ('target', 'seam_oracle_watch'):
        scratch /= component
        scratch.mkdir(exist_ok=True)
        metadata = scratch.lstat()
        assert stat.S_ISDIR(metadata.st_mode) and metadata.st_dev == ROOT.stat().st_dev
    with tempfile.TemporaryDirectory(prefix='controls-', dir=scratch) as temporary:
        directory = Path(temporary)

        def scenario(name, mutate=None, expected=0, arguments=()):
            fixture = directory / name
            for source in sources:
                destination = fixture / source
                destination.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(ROOT / source, destination)
            if mutate is not None:
                mutate(fixture)
            result = subprocess.run(['bash', str(fixture / DRIVER), *arguments], cwd=fixture,
                                    stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=30,
                                    env={**os.environ, 'PYTHONDONTWRITEBYTECODE': '1'})
            assert (result.returncode == 0) == (expected == 0), (name, result.returncode, result.stdout, result.stderr)
            if expected == 0:
                assert b'9 exact producers/fixtures pass;rc=0' in result.stdout
            else:
                assert b'9 exact producers/fixtures pass;rc=0' not in result.stdout
            return fixture, result

        scenario('baseline')

        def mismatch(fixture):
            path = fixture / FIXTURES / 'range_progress.tsv'
            path.write_bytes(path.read_bytes() + b'# altered fixture\n')
        scenario('stdout-mismatch', mismatch, expected=1)

        def failed_child(fixture):
            (fixture / DIRECTORY / 'nonuniform_ease_oracle.py').write_text(
                "print('nonuniform oracle: plausible success;rc=0')\nraise SystemExit(7)\n")
        scenario('failed-child', failed_child, expected=1)

        def failed_stdout(fixture):
            (fixture / DIRECTORY / 'uniform_ease_oracle.py').write_text(
                "from pathlib import Path\nimport sys\n"
                "sys.stdout.buffer.write(Path('crates/sc-core/tests/fixtures/uniform_ease.tsv').read_bytes())\n"
                "raise SystemExit(7)\n")
        scenario('failed-stdout-child', failed_stdout, expected=1)
        scenario('missing-producer', lambda fixture:
                 (fixture / DIRECTORY / 'uniform_ease_oracle.py').unlink(), expected=1)

        def unknown(fixture):
            (fixture / DIRECTORY / 'unreviewed_oracle.py').write_text(
                "from pathlib import Path\nPath('unexpected-execution').write_text('ran')\n")
        fixture, result = scenario('unknown-producer', unknown, expected=1)
        assert b'unreviewed producer' in result.stderr
        assert not (fixture / 'unexpected-execution').exists()
        _, result = scenario('arguments', expected=1, arguments=('--skip',))
        assert result.returncode == 2 and b'no arguments accepted' in result.stderr

    for changed_ci, changed_make in ((ci.replace(CI_LINE, ''), make),
                                     (ci, make.replace(MAKE_DISCOVERY, '-name "other.sh"'))):
        try:
            wiring(changed_ci, changed_make, runners)
        except AssertionError:
            pass
        else:
            raise AssertionError('gate removal must refuse')
    assert {p: hashlib.sha256((ROOT / p).read_bytes()).digest() for p in sources} == before
    print(f'seam oracle watch:9 finite controls pass;9 producers/11 fixtures unchanged;'
          f'{len(runners)} discovered probe suites;actual CI/Make wiring checked;rc=0')


if __name__ == '__main__':
    main()
