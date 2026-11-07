#!/usr/bin/env bash
# WHY: a golden suite is worth only what it catches, and G3's exit asks for that proved by an injected mutation in each of the two things it guards.
# WHERE: a copy of the tree under target/, with its own build directory, so nothing here touches the checkout or its fingerprints.
# A mutated run only means something if the same suite passes unmutated, so the first row has to be clean, or this stops and says so.
set -u
cd "$(dirname "$0")/../../../.." || exit 2
ROOT=$PWD
HERE=target/mutation-proof
TREE="$HERE/tree"
rm -rf "$TREE"
mkdir -p "$TREE"
git ls-files -co --exclude-standard -z | tar --null -T - -cf - | tar -x -C "$TREE"
cd "$TREE" || exit 2
export CARGO_HOME="$ROOT/target/cargo-home"
export RUSTUP_HOME="$ROOT/target/cargo-home/rustup-ci"
export CARGO_TARGET_DIR="$ROOT/$TREE/build"
export TMPDIR="$ROOT/target/scratch"
mkdir -p "$TMPDIR"
suite() {
  cargo test --offline -p sc-geometry --test offset --test offset_corpus --test metamorphic > ../suite-geometry.log 2>&1
  local a=$?
  cargo test --offline -p sc-cli --test graded_goldens --test modes --test skirt > ../suite-cli.log 2>&1
  local b=$?
  echo "$a $b"
}
run() {
  # $1 is the name, $2 the file, $3 the text, $4 what it becomes.
  python3 -I -B - "$2" "$3" "$4" <<'PY'
import sys
path, was, now = sys.argv[1:4]
text = open(path, encoding='utf-8').read()
assert text.count(was) >= 1, was
open(path, 'w', encoding='utf-8').write(text.replace(was, now, 1))
PY
  local result
  result=$(suite)
  printf '%s\t%s\t%s\n' "$1" "$result" "$(grep -hE '^test .*FAILED' ../suite-geometry.log ../suite-cli.log | grep -v 'test result' | sed -E 's/^test //; s/ \.\.\. FAILED//' | head -3 | tr '\n' ' ')"
  python3 -I -B - "$2" "$3" "$4" <<'PY'
import sys
path, was, now = sys.argv[1:4]
text = open(path, encoding='utf-8').read()
open(path, 'w', encoding='utf-8').write(text.replace(now, was, 1))
PY
}
printf 'mutation\tgeometry_suite_and_cli_suite_exit\tfirst_failures\n'
baseline=$(suite)
printf 'none\t%s\t\n' "$baseline"
if [ "$baseline" != "0 0" ]; then
  echo "the suite does not pass unmutated, so nothing below would mean anything"
  exit 1
fi
run offset_engine crates/sc-geometry/src/offset.rs "fn offset_polyline(net: &[Point], width: i64, tolerance: i64) -> Vec<Point> {" "fn offset_polyline(net: &[Point], width: i64, tolerance: i64) -> Vec<Point> {
    let width = width + 1;"
run grading_rules crates/sc-grading/src/table.rs "Convention::Cumulative => Ok(rule.deltas.get(at).copied().unwrap_or((0, 0)))," "Convention::Cumulative => Ok(rule.deltas.get(at).copied().map_or((0, 0), |(x, y)| (x, y + i64::from(at != sample_at)))),"
