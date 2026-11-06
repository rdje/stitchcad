#!/usr/bin/env bash
# scripts/check_licenses.sh — the license census ADR-0001 requires, and proof the solver builds where it must.
#
# WHY: G1's exit asks that a license-compatible solver builds on all targets. `MIT OR Apache-2.0` for this workspace
# is settled by ADR-0001, so the census's job is to see every dependency's own license field, including test ones,
# and refuse copyleft in what ships. It reads `cargo metadata` rather than a list kept by hand.
#
# WHAT IT REFUSES: a package with no license field; a license outside the permissive list below, read token by token,
# so a new one stops the census until somebody reads it; any workspace crate not under the workspace license.
#
# HONEST LIMITS: a license field is the crate's own claim, and this does not read LICENSE files or vendored code.
# "All targets" means this machine's native build plus wasm32-unknown-unknown here, and hosted CI adds Linux. Windows
# is not built anywhere yet.
#
# Usage:  bash scripts/check_licenses.sh
# Output: one line per third-party package, then `license census: <n> packages, <m> third-party, 0 refused`, and the
#         solver lines. Exit 0 when nothing is refused and the solver builds.
set -uo pipefail
export LC_ALL=C
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT" || exit 1
if ! python3 -I -B scripts/local_environment.py --verify >/dev/null 2>&1; then
  exec python3 -I -B scripts/local_environment.py -- bash scripts/check_licenses.sh "$@"
fi
mkdir -p target/license_census
cargo metadata --format-version 1 --locked > target/license_census/metadata.json 2> target/license_census/metadata.log \
  || { echo "license census: cargo metadata failed; see target/license_census/metadata.log" >&2; exit 1; }
python3 -I -B - target/license_census/metadata.json <<'PY' || exit 1
import json
import re
import sys

PERMISSIVE = {"MIT", "Apache-2.0", "BSD-2-Clause", "BSD-3-Clause", "ISC", "Unlicense", "Zlib", "Unicode-3.0"}
WORKSPACE_LICENSE = "MIT OR Apache-2.0"
metadata = json.load(open(sys.argv[1], encoding="utf-8"))
members = set(metadata["workspace_members"])
refused = []
third_party = 0
for package in sorted(metadata["packages"], key=lambda item: item["name"]):
    name, license_text = package["name"], package.get("license") or ""
    if package["id"] in members:
        if license_text != WORKSPACE_LICENSE:
            refused.append("%s is under %r, not the workspace license" % (name, license_text))
        continue
    third_party += 1
    print("%s %s: %s" % (name, package["version"], license_text or "(no license field)"))
    if not license_text:
        refused.append("%s declares no license" % name)
        continue
    # `A OR B` is permissive when one side is. `A AND B` needs both. Anything else stops here for a person to read.
    alternatives = [part.strip() for part in re.split(r"\bOR\b|/", license_text.strip("() "))]
    def allowed(alternative):
        return all(token.strip("() ") in PERMISSIVE for token in alternative.split(" AND "))
    if not any(allowed(alternative) for alternative in alternatives):
        refused.append("%s is under %r, which nobody has reviewed" % (name, license_text))
for reason in refused:
    print("  x " + reason, file=sys.stderr)
print("license census: %d packages, %d third-party, %d refused" % (len(metadata["packages"]), third_party, len(refused)))
sys.exit(1 if refused else 0)
PY

# The solver is in every profile's tree, and it builds for both targets with nothing beside it.
for package in sc-cli sc-viewer; do
  cargo tree -p "$package" -e normal --prefix none 2>/dev/null | grep -q '^sc-constraints ' \
    || { echo "license census: $package does not include the solver" >&2; exit 1; }
done
[ "$(cargo tree -p sc-constraints -e all --prefix none | wc -l)" -eq 1 ] \
  || { echo "license census: the solver has grown a dependency" >&2; exit 1; }
cargo build -p sc-constraints > target/license_census/native.log 2>&1 \
  || { echo "license census: the solver does not build natively" >&2; exit 1; }
cargo build --target wasm32-unknown-unknown -p sc-constraints > target/license_census/wasm.log 2>&1 \
  || { echo "license census: the solver does not build for the browser" >&2; exit 1; }
echo "solver: builds natively and for wasm32-unknown-unknown, alone, and is in the headless and browser profiles"
