#!/usr/bin/env bash
# scripts/check_build_fingerprints.sh — refuses a build directory whose fingerprints watch something other than the real crates.
#
# WHY: cargo hashes a workspace package by its path relative to the workspace, so a harness that copies the workspace and builds the copy with the same
# target directory writes over the real crate's library and fingerprint. Cargo then watches the copy's source. Edits to the real crate rebuild nothing,
# and everything links the copy's library, with no warning. D400 found this: `sc-units` watched a fault run's copy for eight days. A check that passes on
# such a library says nothing about the source in front of you. Hosted CI builds clean, so this can only happen here.
#
# Usage:  bash scripts/check_build_fingerprints.sh [--clear]
# --clear removes those fingerprints and their libraries, inside target/ only, so the next build rebuilds them from the real source.
# Exit 0 when none is found or all were cleared.
set -uo pipefail
export LC_ALL=C
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT" || exit 1
exec python3 -I -B - "$@" <<'PY'
import pathlib
import re
import shutil
import sys

CLEAR = "--clear" in sys.argv[1:]
ROOT = pathlib.Path("target")
PROFILES = ("debug", "release", "wasm32-unknown-unknown/debug", "wasm32-unknown-unknown/release")
COPY = re.compile(rb"(/workspace/|_faults/|certificate/)")


def found():
    for profile in PROFILES:
        directory = ROOT / profile / ".fingerprint"
        if not directory.is_dir():
            continue
        for unit in directory.iterdir():
            if re.match(r"sc[-_]", unit.name) and any(COPY.search(dep.read_bytes()) for dep in unit.glob("dep-*")):
                yield profile, unit


bad = list(found())
for profile, unit in bad:
    print("build fingerprints: %s/%s watches a copy of the source" % (profile, unit.name))
    if CLEAR:
        crate, code = unit.name.rsplit("-", 1)
        for artifact in (ROOT / profile / "deps").glob("*%s-%s*" % (crate.replace("-", "_"), code)):
            if artifact.is_file():
                artifact.unlink()
            else:
                shutil.rmtree(artifact)
        shutil.rmtree(unit)
if bad and not CLEAR:
    print("build fingerprints: run this with --clear, then build again", file=sys.stderr)
    sys.exit(1)
print("build fingerprints: %s" % ("%d cleared" % len(bad) if bad else "every unit watches the real crates"))
PY
