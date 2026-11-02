#!/usr/bin/env python3
"""Reproduce the scoped native-family census and actual Garment/input loan controls.

Run through scripts/local_environment.py after consuming all Cargo producers. A focused
library build makes the compiler controls use the current public API. Outputs are
regenerable compiler scratch under target;this grants no new dependency/programme proof.
"""
from pathlib import Path
import re
import subprocess
import sys


def require(condition, message):
    if not condition:
        raise RuntimeError(message)


def main():
    require(len(sys.argv) == 1, "this focused probe accepts no arguments")
    root = Path(__file__).resolve().parents[4]
    source = (root / "crates/sc-core/src/construction/operation_input.rs").read_text()
    family = source.split("pub enum DraftingOperationFamily {", 1)[1].split("\n}", 1)[0]
    names = re.findall(r"^    ([A-Za-z]+),$", family, re.M)
    require(len(names) == 13, "closed v1 native-family population changed;review the plan")
    out = root / "target/drafting_dependency_plan"
    for directory in (out.parent, out):
        require(not directory.is_symlink(), "compiler scratch traverses a symlink")
        require(not (directory / ".git").exists(), "compiler scratch crosses a nested repository")
    require(out.parent.stat().st_dev == root.stat().st_dev, "compiler scratch is off-volume")
    out.mkdir(exist_ok=True)
    for label in ("borrow", "escape"):
        for suffix in (".rs", ".rmeta"):
            require(not (out / (label + suffix)).is_symlink(), "compiler output is a symlink")
    built = subprocess.run(["cargo", "build", "-p", "sc-core", "--lib"], cwd=root,
                           capture_output=True, text=True, check=False)
    require(built.returncode == 0, built.stdout + built.stderr)
    print("focused current sc-core library build: rc=0", flush=True)
    libraries = list((root / "target/debug/deps").glob("libsc_core-*.rlib"))
    require(libraries, "build the actual sc-core library before this focused probe")
    library = max(libraries, key=lambda p: p.stat().st_mtime_ns)

    def fingerprint():
        metadata = library.stat()
        return metadata.st_ino, metadata.st_size, metadata.st_mtime_ns

    signature = fingerprint()
    for label, lifetime, expected in (("borrow", "'a", 0), ("escape", "'static", 1)):
        probe = out / (label + ".rs")
        probe.write_text(
            "#![allow(dead_code)]\n"
            "use sc_core::design::{DesignGarment,DesignInputs,DesignReferenceHost};\n"
            "fn root_source<'a,H:DesignReferenceHost+'static>(garment:&'a DesignGarment<H>)"
            "->&" + lifetime + " DesignInputs<H> {garment.inputs()}\n"
        )
        result = subprocess.run(
            ["rustc", "--edition=2021", "--crate-type=lib", "--emit=metadata",
             "--crate-name", label, str(probe), "--extern", "sc_core=" + str(library),
             "-L", "dependency=" + str(root / "target/debug/deps"),
             "-o", str(out / (label + ".rmeta"))],
            cwd=root, capture_output=True, text=True, check=False,
        )
        require(fingerprint() == signature,
                "actual library changed during probe;discard the result")
        require(result.returncode == expected, label + ": " + result.stderr)
        if expected:
            require("lifetime may not live long enough" in result.stderr, result.stderr)
        print("Garment/input " + label + ": expected rustc rc=" + str(expected) + ";matched")
    print("closed native families=" + str(len(names)) + ";public-owner controls matched;rc=0")


if __name__ == "__main__":
    main()
