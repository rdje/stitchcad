#!/usr/bin/env bash
# docs/tasks/artifacts/canvas_spike/run_spike_verdict_probes.sh
# G0-CONTRACT.11 — probes for the ADR-0002 spike verdict instrument.
#
# WHY: the leaf's acceptance is that the decision rule is written BEFORE the measurement exists, so the
# G1 outcome cannot be argued after the fact. A rule nobody has seen applied to a losing data set is a
# rule that will be argued, so every arm below feeds the instrument a synthetic result set whose verdict
# is known in advance and requires exactly that verdict. The data sets are the interesting part: a
# tie broken by memory, a fast topology disqualified for imprecise snapping, a native-only winner, and a
# profile where nothing survives.
#
# ⚠ Each arm writes its own results file and REFUSES (exit 3) if the instrument's output does not carry
#   the expected verdict — an arm that accepted any output would be a report, not a probe.
# ⚠ GATES-READ is the arm that proves the thresholds live in the data plane: it tightens one gate in a
#   COPY of spike_gates.tsv and requires the verdict to change, so a script with hardcoded numbers fails
#   here by name.
#   R4-FREE (a faster native-only winner gives way to one renderer for both profiles)
# ⚠ CONTROL feeds the same measurements with an extra comment line and requires the identical verdict.
#
# Usage:  bash docs/tasks/artifacts/canvas_spike/run_spike_verdict_probes.sh
#         TMPDIR=<dir> bash …   (`make probes` pins it to the repository volume)
# Output: per-arm lines plus `probes: N pass / M fail` (exit nonzero if M > 0).
set -uo pipefail
export LC_ALL=C
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
TOOL="$ROOT/docs/tasks/artifacts/canvas_spike/run_spike_verdict.sh"
PLANE="$ROOT/docs/tasks/artifacts/canvas_spike"
for f in "$TOOL" "$PLANE/spike_gates.tsv" "$PLANE/spike_topologies.tsv" "$PLANE/spike_rule.tsv" \
         "$PLANE/results.tsv"; do
  [ -f "$f" ] || { echo "probe: REFUSED — $f not found" >&2; exit 2; }
done
command -v python3 >/dev/null 2>&1 || { echo "probe: REFUSED — python3 not found" >&2; exit 2; }

WORK="${TMPDIR:-$ROOT/target/scratch}/canvas_spike_probes"
rm -rf "$WORK"; mkdir -p "$WORK"; trap 'rm -rf "$WORK"' EXIT

pass=0; fail=0
ok()  { printf '  ✓ %-14s %s\n' "$1" "$2"; pass=$((pass+1)); }
bad() { printf '  ✗ %-14s %s\n' "$1" "$2"; fail=$((fail+1))
        [ -n "${3:-}" ] && printf '%s\n' "$3" | grep -E '^  x |spike verdict:|REFUSED|R5|→ winner' | head -6 | sed 's/^/               /'; }

# plane <arm>: a copy of the data plane, so one arm's results or gates never reach another's.
plane() {
  local d="$WORK/$1"; rm -rf "$d"; mkdir -p "$d"
  cp "$PLANE"/spike_gates.tsv "$PLANE"/spike_topologies.tsv "$PLANE"/spike_rule.tsv "$d/"
  printf '%s' "$d"
}
run() { SPIKE_DIR="$1" bash "$TOOL" 2>&1; }

HDR='#topology	profile	capable	pick_correct	snap_exact	fidelity_max_px	fidelity_drift_px	frame_p95_ms	peak_mb	unproven_deps	frame_median_ms	cold_start_ms	bundle_kb'
# row <topology> <profile> <capable> <pick> <snap> <fidelity> <drift> <p95> <mb> <deps>
row() { printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t-\t-\t-\n' \
        "$1" "$2" "$3" "$4" "$5" "$6" "$7" "$8" "$9" "${10}"; }

echo "canvas-spike probes — docs/tasks/artifacts/canvas_spike/run_spike_verdict.sh"

# ---------------------------------------------------------------- PENDING (the real plane)
out="$(run "$PLANE")"; rc=$?
if [ "$rc" -eq 0 ] && grep -q 'PENDING' <<<"$out" && grep -q '0 refusal(s)' <<<"$out"; then
  ok PENDING "no measurements yet is a stated verdict, not a failure: $(grep -o 'spike verdict: .*' <<<"$out" | cut -c1-70)"
else
  bad PENDING "an empty results file did not report PENDING (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- CLEAR-WINNER (R4 fires)
D="$(plane clear)"; { printf '%s\n' "$HDR"
  row native-wgpu-composited native yes yes yes 0.1 0 12.0 260 1
  row webgpu-webview native yes yes yes 0.2 0 8.0 300 2
  row webgpu-webview browser yes yes yes 0.2 0 9.0 280 2
  row dom-canvas native yes yes yes 0.4 0 14.0 190 0
  row dom-canvas browser yes yes yes 0.4 0 15.0 170 0; } > "$D/results.tsv"
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 0 ] && grep -q 'one topology for every profile: webgpu-webview' <<<"$out"; then
  ok CLEAR-WINNER "one topology winning both profiles outright is the verdict, with no tiebreak needed"
else
  bad CLEAR-WINNER "a clear winner was not reported as one (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- R4-FREE (one renderer where it is free)
D="$(plane r4free)"; { printf '%s\n' "$HDR"
  row native-wgpu-composited native yes yes yes 0.1 0 8.0 260 1
  row webgpu-webview native yes yes yes 0.2 0 8.5 300 2
  row webgpu-webview browser yes yes yes 0.2 0 9.0 280 2
  row dom-canvas native yes yes yes 0.4 0 14.0 190 0
  row dom-canvas browser yes yes yes 0.4 0 15.0 170 0; } > "$D/results.tsv"
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 0 ] && grep -q 'R4 prefers one renderer: webgpu-webview' <<<"$out" \
   && grep -q '→ winner: native-wgpu-composited' <<<"$out"; then
  ok R4-FREE "a faster native-only winner gives way to one renderer for both profiles, and the trace shows both"
else
  bad R4-FREE "R4 did not prefer the single renderer (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- TIE-MEMORY (R3 tiebreak)
D="$(plane tie)"; { printf '%s\n' "$HDR"
  row native-wgpu-composited native yes yes yes 0.1 0 12.0 260 1
  row webgpu-webview native yes yes yes 0.2 0 9.0 480 2
  row webgpu-webview browser yes yes yes 0.2 0 9.0 480 2
  row dom-canvas native yes yes yes 0.4 0 9.2 190 0
  row dom-canvas browser yes yes yes 0.4 0 9.2 190 0; } > "$D/results.tsv"
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 0 ] && grep -q 'inside the 10% margin' <<<"$out" && grep -q '2 inside the margin' <<<"$out"; then
  ok TIE-MEMORY "a tie inside the margin goes to the declared tiebreak, and the trace says so"
else
  bad TIE-MEMORY "a tie was not broken by the declared order (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- CORRECTNESS (R2 outranks R3)
D="$(plane correct)"; { printf '%s\n' "$HDR"
  row native-wgpu-composited native yes yes yes 0.1 0 14.0 260 1
  row webgpu-webview native yes yes yes 0.2 0 12.0 300 2
  row webgpu-webview browser yes yes yes 0.2 0 12.0 300 2
  row dom-canvas native yes yes no 0.4 0 4.0 190 0
  row dom-canvas browser yes yes yes 0.4 0 13.0 170 0; } > "$D/results.tsv"
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 0 ] && grep -q 'R2 disqualified dom-canvas *snap_exact = no' <<<"$out" \
   && grep -q '→ winner: webgpu-webview' <<<"$out"; then
  ok CORRECTNESS "the fastest topology loses to a correctness gate, and the gate is named"
else
  bad CORRECTNESS "speed outranked correctness (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- NATIVE-ONLY (R1 + R4 cost)
D="$(plane nativeonly)"; { printf '%s\n' "$HDR"
  row webgpu-webview native yes yes yes 0.2 0 9.0 300 2
  row webgpu-webview browser yes yes yes 0.2 0 10.0 280 2
  row native-wgpu-composited native yes yes yes 0.1 0 5.0 260 1
  row dom-canvas native yes yes yes 0.4 0 14.0 190 0
  row dom-canvas browser yes yes yes 0.4 0 15.0 170 0; } > "$D/results.tsv"
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 0 ] && grep -q 'two renderers: browser=webgpu-webview, native=native-wgpu-composited' <<<"$out" \
   && grep -q 'R4 does not fire' <<<"$out"; then
  ok NATIVE-ONLY "a native-only winner is a real outcome, and the two-renderer cost is stated"
else
  bad NATIVE-ONLY "an ineligible topology was scored, or the cost was not stated (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- NO-SURVIVOR (R5)
D="$(plane nosurvivor)"; { printf '%s\n' "$HDR"
  row native-wgpu-composited native yes yes yes 0.1 0 11.0 260 1
  row webgpu-webview native yes yes yes 0.2 0 9.0 300 2
  row webgpu-webview browser no yes yes 0.2 0 9.0 300 2
  row dom-canvas native yes yes yes 0.4 0 14.0 190 0
  row dom-canvas browser yes yes yes 3.0 0 15.0 170 0; } > "$D/results.tsv"
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 0 ] && grep -q 'R5 no survivor' <<<"$out" && grep -q 'ESCALATE (browser)' <<<"$out" \
   && grep -q 'dev-shell' <<<"$out"; then
  ok NO-SURVIVOR "a profile with nothing left escalates and names the bounded fallback"
else
  bad NO-SURVIVOR "a profile with no survivor did not escalate (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- INCOMPLETE (refusal)
D="$(plane incomplete)"; { printf '%s\n' "$HDR"
  row native-wgpu-composited native yes yes yes 0.1 0 12.0 260 1
  row webgpu-webview native yes yes yes 0.2 0 9.0 300 2
  row webgpu-webview browser yes yes yes 0.2 0 - 300 2
  row dom-canvas native yes yes yes 0.4 0 14.0 190 0
  row dom-canvas browser yes yes yes 0.4 0 15.0 170 0; } > "$D/results.tsv"
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'leaves the gated metric `frame_p95_ms` unmeasured' <<<"$out"; then
  ok INCOMPLETE "a gated metric left unmeasured is refused rather than averaged away"
else
  bad INCOMPLETE "a partial data set produced a verdict (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- UNDECLARED (refusal)
D="$(plane undeclared)"; { printf '%s\n' "$HDR"
  row native-wgpu-composited native yes yes yes 0.1 0 12.0 260 1
  row webgpu-webview native yes yes yes 0.2 0 9.0 300 2
  row webgpu-webview browser yes yes yes 0.2 0 10.0 280 2
  row svg-canvas browser yes yes yes 0.4 0 11.0 170 0
  row dom-canvas native yes yes yes 0.4 0 14.0 190 0
  row dom-canvas browser yes yes yes 0.4 0 15.0 170 0; } > "$D/results.tsv"
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'svg-canvas / browser, which the topology table does not declare' <<<"$out"; then
  ok UNDECLARED "a fourth topology measured but never declared is refused by name"
else
  bad UNDECLARED "an undeclared topology was scored (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- MISSING-ROW (refusal)
D="$(plane missingrow)"; { printf '%s\n' "$HDR"
  row native-wgpu-composited native yes yes yes 0.1 0 12.0 260 1
  row webgpu-webview native yes yes yes 0.2 0 9.0 300 2
  row dom-canvas native yes yes yes 0.4 0 14.0 190 0
  row dom-canvas browser yes yes yes 0.4 0 15.0 170 0; } > "$D/results.tsv"
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'makes webgpu-webview / browser a candidate and the results carry no row' <<<"$out"; then
  ok MISSING-ROW "an applicable pair nobody measured is refused, so a verdict cannot rest on absence"
else
  bad MISSING-ROW "a missing measurement was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- GATES-READ (the data plane is real)
D="$(plane gatesread)"; { printf '%s\n' "$HDR"
  row native-wgpu-composited native yes yes yes 0.4 0 12.0 260 1
  row webgpu-webview native yes yes yes 0.1 0 9.0 300 2
  row webgpu-webview browser yes yes yes 0.1 0 10.0 280 2
  row dom-canvas native yes yes yes 0.4 0 14.0 190 0
  row dom-canvas browser yes yes yes 0.4 0 15.0 170 0; } > "$D/results.tsv"
python3 - "$D/spike_gates.tsv" <<'PY'
import pathlib, sys
p = pathlib.Path(sys.argv[1]); t = p.read_text(encoding="utf-8")
old = "fidelity_max_px\tle\t0.5\tyes"
if old not in t:
    print("mutation REFUSED — pattern absent", file=sys.stderr); sys.exit(3)
p.write_text(t.replace(old, "fidelity_max_px\tle\t0.2\tyes", 1), encoding="utf-8")
PY
[ $? -eq 0 ] || bad GATES-READ "the mutation did not apply" ""
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 0 ] && grep -q 'R2 disqualified .*fidelity_max_px = 0.4' <<<"$out"; then
  ok GATES-READ "tightening a threshold in the TSV changes the verdict, so the gates are read, not hardcoded"
else
  bad GATES-READ "a threshold in the data plane did not bind (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- CONTROL
D="$(plane control)"; { printf '%s\n' "$HDR"
  row native-wgpu-composited native yes yes yes 0.1 0 12.0 260 1
  printf '# a comment line that changes no measurement\n'
  row webgpu-webview native yes yes yes 0.2 0 9.0 300 2
  row webgpu-webview browser yes yes yes 0.2 0 10.0 280 2
  row dom-canvas native yes yes yes 0.4 0 14.0 190 0
  row dom-canvas browser yes yes yes 0.4 0 15.0 170 0; } > "$D/results.tsv"
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 0 ] && grep -q 'one topology for every profile: webgpu-webview' <<<"$out"; then
  ok CONTROL "a comment in the results file leaves the verdict identical, so the arms are not vacuous"
else
  bad CONTROL "an unrelated comment changed the verdict (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- MISSING
D="$(plane missing)"; rm -f "$D/spike_gates.tsv"
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 2 ] && grep -q 'REFUSED' <<<"$out"; then
  ok MISSING "an absent data plane refuses with exit=2 instead of reporting a verdict"
else
  bad MISSING "a missing gates file did not refuse (exit=$rc)" "$out"
fi

echo
printf 'probes: %d pass / %d fail\n' "$pass" "$fail"
[ "$fail" -eq 0 ] || exit 1
exit 0
