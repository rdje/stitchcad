"""Exercise the actual CI store helper with isolated inputs, then disable individual guards."""
from pathlib import Path
import shutil
import types

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / "scripts/ci_environment.py"
WORK = ROOT / "target/scratch/ci_environment_contract"
EXPECTED = {
    "CARGO_HOME": "target/cargo-home",
    "RUSTUP_HOME": "target/cargo-home/rustup-ci",
    "CARGO_TARGET_DIR": "target",
    "TMPDIR": "target/scratch",
}


def safe_remove(path):
    if path.exists():
        assert not path.is_symlink() and path.stat().st_dev == ROOT.stat().st_dev
        assert not list(path.rglob(".git")), "probe would delete a Git boundary"
        shutil.rmtree(path)


def load(source):
    scope = {"__file__": str(SOURCE), "__name__": "ci_environment_probe"}
    exec(compile(source, str(SOURCE), "exec"), scope)
    return scope


def controls(scope):
    safe_remove(WORK)
    WORK.mkdir(parents=True)
    fixture = WORK / "workspace"
    fixture.mkdir()
    export = WORK / "environment"
    export.write_text("EXISTING=retained\n")
    good = {name: str(fixture / relative) for name, relative in EXPECTED.items()}
    prepare, verify, refusal = scope["prepare"], scope["verify"], scope["Refusal"]
    prepare(fixture, {"GITHUB_ENV": str(export)})
    assert export.read_text() == "EXISTING=retained\n" + "".join(
        name + "=" + value + "\n" for name, value in good.items()), "export identity/order or prior data lost"
    assert verify(fixture, good) == EXPECTED, "effective store catalog differs"
    count = 2

    def reject(action, message):
        nonlocal count
        try:
            action()
        except refusal as error:
            assert str(error) == message, (message, str(error))
        else:
            raise AssertionError("actual guard accepted: " + message)
        count += 1

    for name in EXPECTED:
        bad = dict(good)
        del bad[name]
        reject(lambda: verify(fixture, bad), "effective " + name + " is not checkout-local")
        bad = dict(good)
        bad[name] = str(ROOT / "unowned-store")
        reject(lambda: verify(fixture, bad), "effective " + name + " is not checkout-local")
    reject(lambda: prepare(fixture, {}), "GitHub environment command file is unavailable")
    reject(lambda: prepare(fixture, {"GITHUB_ENV": str(fixture)}), "environment command file is not regular")
    link = WORK / "export-link"
    link.symlink_to(export)
    reject(lambda: prepare(fixture, {"GITHUB_ENV": str(link)}), "invalid environment command file")
    link.unlink()
    # Exact directory guards act on actual fixture entries, not guessed path text.
    scratch = fixture / "target/scratch"
    scratch.rmdir()
    reject(lambda: verify(fixture, good), "missing store directory")
    scratch.write_text("ordinary file")
    reject(lambda: verify(fixture, good), "store component is not a directory")
    scratch.unlink()
    scratch.symlink_to(WORK)
    reject(lambda: verify(fixture, good), "symlink store component")
    scratch.unlink()
    scratch.mkdir()
    marker = scratch / ".git"
    marker.write_text("probe-only boundary marker; no Git repository")
    try:
        reject(lambda: verify(fixture, good), "store crosses another Git repository")
    finally:
        marker.unlink()
    # Device metadata is simulated explicitly; no off-volume file is written for this control.
    original_stat = Path.stat

    def changed_device(path, *args, **kwargs):
        result = original_stat(path, *args, **kwargs)
        if path == scratch:
            return types.SimpleNamespace(st_mode=result.st_mode, st_dev=result.st_dev + 1)
        return result

    Path.stat = changed_device
    try:
        reject(lambda: verify(fixture, good), "store is on another volume")
    finally:
        Path.stat = original_stat
    # A malformed destination must refuse before adding exports or store paths.
    before = export.read_bytes()
    reject(lambda: scope["directory"](fixture, "../escape", create=True), "escaped store path")
    assert export.read_bytes() == before and not (WORK / "escape").exists(), "refusal leaked data"
    return count


original = SOURCE.read_bytes()
text = original.decode()
try:
    base = controls(load(text))
    # Actual producer source is compiled with each changed guard, while on-disk bytes stay intact.
    faults = {
        "effective environment": ('environment.get(name) == str(root / relative)', 'True'),
        "symlink": ('not current.is_symlink()', 'True'),
        "Git boundary": ('not (current / ".git").exists()', 'True'),
        "device": ('current.stat().st_dev == device', 'True'),
        "component type": ('current.is_dir()', 'True'),
        "export identity": ('name + "=" + str(path) + "\\n"', 'name + "=" + "unowned" + "\\n"'),
    }
    for label, (needle, replacement) in faults.items():
        assert text.count(needle) == 1, (label, "changed fault anchor")
        try:
            controls(load(text.replace(needle, replacement)))
        except AssertionError as error:
            assert str(error).startswith(("actual guard accepted:", "export identity/order")), (label, error)
        else:
            raise AssertionError("compiled guard fault passed: " + label)
        print("  actual compiled CI assertion red:", label)
    workflow = (ROOT / ".github/workflows/rust.yml").read_text()
    prepare_line = "        run: python3 -I -B scripts/ci_environment.py prepare\n"
    verify_line = "        run: python3 -I -B scripts/ci_environment.py verify\n"
    action = "        run: rustup toolchain install stable --profile minimal --component rustfmt --component clippy --target wasm32-unknown-unknown --no-self-update\n"
    format_step = "      - name: Format\n"
    assert all(workflow.count(line) == 1 for line in (prepare_line, verify_line, action, format_step))
    assert workflow.index(prepare_line) < workflow.index(action) < workflow.index(verify_line) < workflow.index(format_step)
    assert "      RUSTUP_TOOLCHAIN: stable\n" in workflow
    assert SOURCE.read_bytes() == original
    print("ci-environment probes: %d independent runtime controls / 6 actual compiled reds / workflow order / 0 fail" % base)
finally:
    safe_remove(WORK)
