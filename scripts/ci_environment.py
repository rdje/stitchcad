"""Prepare and verify checkout-local Rust CI stores through documented environment variables."""
from pathlib import Path
import argparse
import os
import stat
import sys

ROOT = Path(__file__).resolve().parent.parent
STORES = {
    "CARGO_HOME": "target/cargo-home",
    "RUSTUP_HOME": "target/cargo-home/rustup-ci",
    "CARGO_TARGET_DIR": "target",
    "TMPDIR": "target/scratch",
}


class Refusal(ValueError):
    """The effective CI environment cannot satisfy the locality contract."""


def require(condition, message):
    if not condition:
        raise Refusal(message)


def directory(root, relative, create=False):
    """Check every existing component before creating only the fixed local store directories."""
    device = root.stat().st_dev
    current = root
    for part in Path(relative).parts:
        require(part not in ("", ".", "..") and not Path(relative).is_absolute(), "escaped store path")
        current = current / part
        require(not current.is_symlink(), "symlink store component")
        if not current.exists():
            require(create, "missing store directory")
            current.mkdir()
        require(current.is_dir(), "store component is not a directory")
        require(current.stat().st_dev == device, "store is on another volume")
        require(not (current / ".git").exists(), "store crosses another Git repository")
    return current


def workspace(root):
    """Production root or bounded local fixture; never a foreign repository or volume."""
    require(root.is_absolute(), "workspace must be absolute at runtime")
    require(root == ROOT or root.is_relative_to(ROOT / "target"), "workspace outside project target")
    if root != ROOT:
        directory(ROOT, root.relative_to(ROOT).as_posix())
    require("\n" not in str(root) and "\r" not in str(root), "workspace contains command-file newline")


def prepare(root, environment):
    workspace(root)
    export_name = environment.get("GITHUB_ENV")
    require(bool(export_name), "GitHub environment command file is unavailable")
    export = Path(export_name)
    require(export.is_absolute() and not export.is_symlink(), "invalid environment command file")
    require(export.exists() and stat.S_ISREG(export.stat().st_mode), "environment command file is not regular")
    require(".git" not in export.parts, "environment command file is Git metadata")
    # The runner-supplied command file is a required platform protocol, not a build store.
    # Local probes use an existing file below target/; foreign-repository writes always refuse.
    for parent in export.parents:
        require(not parent.is_symlink(), "symlink command-file ancestor")
        require(parent == ROOT or not (parent / ".git").exists(), "command file crosses another Git repository")
    if export.is_relative_to(ROOT / "target"):
        require(export.stat().st_dev == ROOT.stat().st_dev, "local command file is on another volume")
    else:
        require(environment.get("GITHUB_ACTIONS") == "true", "external command file outside GitHub runner")
        runner_temp = Path(environment.get("RUNNER_TEMP", ""))
        require(runner_temp.is_absolute() and export.is_relative_to(runner_temp),
                "command file outside declared runner temporary directory")
    values = {name: directory(root, relative, create=True) for name, relative in STORES.items()}
    with export.open("a", encoding="utf-8") as stream:
        stream.write("".join(name + "=" + str(path) + "\n" for name, path in values.items()))
    return values


def verify(root, environment):
    workspace(root)
    for name, relative in STORES.items():
        require(environment.get(name) == str(root / relative), "effective " + name + " is not checkout-local")
        directory(root, relative)
    return dict(STORES)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("operation", choices=("prepare", "verify"))
    args = parser.parse_args()
    if args.operation == "prepare":
        prepare(ROOT, os.environ)
    else:
        verify(ROOT, os.environ)
    print("ci-environment: " + args.operation + " OK; all four stores on checkout volume")
    for name, relative in STORES.items():
        print(name + "=" + relative)


if __name__ == "__main__":
    try:
        main()
    except (Refusal, OSError) as error:
        print("ci-environment: REFUSED — " + str(error), file=sys.stderr)
        sys.exit(2)
