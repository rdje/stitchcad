#!/usr/bin/env bash
# docs/tasks/artifacts/canvas_spike/run_spike_verdict.sh
# G0-CONTRACT.11 — the instrument that applies ADR-0002's decision rule to the spike's measurements.
#
# WHY A TOOL WRITTEN BEFORE THE DATA EXISTS: the leaf's acceptance is that the spike's measurements,
# pass/fail criteria and decision rule are written at G0 "so the G1 outcome cannot be argued after the
# fact". A rule in prose is argued; a rule in an instrument that reads a data plane is executed. So the
# gates, the applicability table and the rule's parameters are TSVs beside this script, and the verdict is
# derived from them — which also means tightening a threshold is a diff a reviewer sees, not an edit
# inside a program.
#
# WHAT IT DOES, mechanically:
#   1. reads spike_gates.tsv (metric, comparator, threshold, hard), spike_topologies.tsv (which topology
#      is a candidate in which profile), spike_rule.tsv (the margin, the tiebreak order, the fallback) and
#      results.tsv (the measurements);
#   2. REFUSES a data set that cannot produce a verdict — an applicable pair with no row, a row for a
#      topology the table does not declare, a gated metric left blank or `-`, a non-numeric where a number
#      is owed, a threshold that does not parse;
#   3. prints PENDING (exit 0) when no measurements exist yet, which is the honest state at G0 and the
#      reason ADR-0002's canvas half is recorded `proposed`;
#   4. otherwise applies R1 eligibility, R2 correctness gates, R3 scoring with the declared margin and
#      tiebreaks, R4 the one-renderer preference, and R5 escalation with the bounded fallback, printing
#      the rule that decided each profile.
#
# HONEST LIMITS: it applies the rule to numbers it did not measure. It cannot tell whether a frame time is
# real, whether the corpus was the declared one, or whether the hardware recorded is the hardware used —
# those are the spike's own evidence obligations (R6, and `G1-SLICE.13`'s acceptance). It also does not
# decide anything the rule does not: a tie the declared tiebreaks cannot separate is refused as such
# rather than settled by an opinion, and the alphabetical last resort is what makes the rule total.
#
# Usage:  bash docs/tasks/artifacts/canvas_spike/run_spike_verdict.sh
#         SPIKE_DIR=<dir> bash …      another copy of the data plane (the probe suites use this)
#         SPIKE_RESULTS=<path> bash …  a results file on its own
# Output: the rule trace, then
#         `spike verdict: <rows> row(s) / <profiles> profile(s) / <outcome> / <n> refusal(s)`
#         exit 1 on a data set that cannot produce a verdict, exit 2 when an input is missing.
set -uo pipefail
export LC_ALL=C

ROOT="${SPIKE_ROOT:-$(git rev-parse --show-toplevel 2>/dev/null || pwd)}"
DIR="${SPIKE_DIR:-$ROOT/docs/tasks/artifacts/canvas_spike}"
GATES="${SPIKE_GATES:-$DIR/spike_gates.tsv}"
TOPOS="${SPIKE_TOPOLOGIES:-$DIR/spike_topologies.tsv}"
RULE="${SPIKE_RULE:-$DIR/spike_rule.tsv}"
RESULTS="${SPIKE_RESULTS:-$DIR/results.tsv}"

for f in "$GATES" "$TOPOS" "$RULE" "$RESULTS"; do
  [ -f "$f" ] || { echo "spike verdict: REFUSED — $f not found" >&2; exit 2; }
done
command -v python3 >/dev/null 2>&1 || { echo "spike verdict: REFUSED — python3 not found" >&2; exit 2; }

python3 - "$GATES" "$TOPOS" "$RULE" "$RESULTS" <<'PY'
import pathlib, sys

GATES, TOPOS, RULE, RESULTS = sys.argv[1:5]
fails = 0
def bad(msg):
    global fails
    fails += 1
    print("  x %s" % msg)

def read_tsv(path, columns):
    """Comment-and-header-stripped rows as dicts; refuses a row whose cell count disagrees."""
    out = []
    for n, line in enumerate(pathlib.Path(path).read_text(encoding="utf-8").splitlines(), 1):
        if not line.strip() or line.lstrip().startswith("#"):
            continue
        cells = line.split("\t")
        if cells[0] == columns[0]:            # the header row, whose first cell is the column name
            continue
        if len(cells) != len(columns):
            bad("%s:%d has %d cells, the schema declares %d" % (pathlib.Path(path).name, n, len(cells),
                                                                len(columns)))
            continue
        out.append(dict(zip(columns, cells)))
    return out

gates = read_tsv(GATES, ["metric", "comparator", "threshold", "hard", "why"])
topos = read_tsv(TOPOS, ["topology", "profile", "applies", "note"])
rule = {r["parameter"]: r["value"] for r in read_tsv(RULE, ["parameter", "value", "why"])}
results = read_tsv(RESULTS, ["topology", "profile", "capable", "pick_correct", "snap_exact",
                             "fidelity_max_px", "fidelity_drift_px", "frame_p95_ms", "peak_mb",
                             "unproven_deps", "frame_median_ms", "cold_start_ms", "bundle_kb"])

# ── the data plane must parse before it can decide anything ────────────────────────────────
hard, recorded, threshold = [], [], {}
for g in gates:
    if g["hard"] not in ("yes", "no"):
        bad("gate `%s` declares hard=%r, which is neither yes nor no" % (g["metric"], g["hard"]))
        continue
    (hard if g["hard"] == "yes" else recorded).append(g["metric"])
    if g["hard"] == "yes":
        if g["comparator"] not in ("eq", "le", "ge", "lt", "gt"):
            bad("gate `%s` declares comparator %r, which this instrument does not apply"
                % (g["metric"], g["comparator"]))
            continue
        if g["threshold"] not in ("yes", "no"):
            try:
                threshold[g["metric"]] = float(g["threshold"])
            except ValueError:
                bad("gate `%s` declares threshold %r, which is neither a number nor yes/no"
                    % (g["metric"], g["threshold"]))
        else:
            threshold[g["metric"]] = g["threshold"]
for need in ("margin_pct", "tiebreak", "fallback"):
    if need not in rule:
        bad("the rule file declares no `%s`" % need)
try:
    margin = float(rule.get("margin_pct", "nan"))
except ValueError:
    bad("`margin_pct` is %r, which is not a number" % rule.get("margin_pct")); margin = float("nan")
tiebreak = [t.strip() for t in rule.get("tiebreak", "").split(",") if t.strip()]

print("=== ADR-0002 canvas spike verdict ===")
print("-- the data plane as read")
print("  gates: %d hard (%s)" % (len(hard), ", ".join(hard)))
print("  recorded, not gated: %s" % (", ".join(recorded) or "none"))
print("  candidates: %d (topology, profile) pairs · margin %s%% · tiebreak %s · fallback %s"
      % (len([t for t in topos if t["applies"] == "yes"]), rule.get("margin_pct"),
         " > ".join(tiebreak), rule.get("fallback")))

# ── PENDING is a verdict about the data, not a failure ────────────────────────────────────
if not results and fails == 0:
    print("-- no measurements recorded")
    print("spike verdict: 0 rows / 0 profiles / PENDING — ADR-0002's canvas half stays `proposed` "
          "until G1-SLICE.13 records measurements / 0 refusal(s)")
    sys.exit(0)

# ── completeness: an applicable pair needs exactly one row, a row needs a declared pair ────
applicable = {(t["topology"], t["profile"]) for t in topos if t["applies"] == "yes"}
declared = {(t["topology"], t["profile"]) for t in topos}
seen = {}
for r in results:
    key = (r["topology"], r["profile"])
    if key not in declared:
        bad("results carry %s / %s, which the topology table does not declare — an ineligible "
            "candidate is not a measurement" % key)
        continue
    if key in seen:
        bad("results carry two rows for %s / %s" % key)
        continue
    seen[key] = r
    for metric in hard + recorded:
        if metric not in r:
            continue
        value = r[metric].strip()
        if metric in hard and value in ("", "-"):
            bad("%s / %s leaves the gated metric `%s` unmeasured — a verdict on partial data is what "
                "the protocol exists to prevent" % (key[0], key[1], metric))
        if value in ("", "-", "yes", "no"):
            continue
        try:
            float(value)
        except ValueError:
            bad("%s / %s records `%s` = %r, which is neither a number nor yes/no"
                % (key[0], key[1], metric, value))
for key in sorted(applicable - set(seen)):
    bad("the topology table makes %s / %s a candidate and the results carry no row for it" % key)

# ── R1 eligibility, R2 correctness, R3 scoring ────────────────────────────────────────────
def passes(metric, value):
    want = threshold.get(metric)
    if want in ("yes", "no"):
        return value.strip().lower() == want
    try:
        got = float(value)
    except ValueError:
        return False
    return {"eq": got == want, "le": got <= want, "ge": got >= want,
            "lt": got < want, "gt": got > want}[gates_comparator[metric]]

gates_comparator = {g["metric"]: g["comparator"] for g in gates}
SCORED = "frame_p95_ms"
profiles = sorted({p for _, p in applicable})
winners, traces, eligible_by_profile, best_by_profile = {}, {}, {}, {}
for profile in profiles:
    print("-- profile `%s`" % profile)
    rows = [seen[k] for k in sorted(seen) if k[1] == profile]
    eligible, disqualified = [], []
    for r in rows:
        name = r["topology"]
        if r["capable"].strip().lower() != "yes":
            disqualified.append((name, "capable", r["capable"]))
            continue
        failed = [(m, r[m]) for m in hard if m != "capable" and not passes(m, r[m])]
        if failed:
            for m, v in failed:
                disqualified.append((name, m, v))
            continue
        eligible.append(r)
    for name, metric, value in disqualified:
        print("  R2 disqualified %-24s %s = %s (gate: %s %s %s)"
              % (name, metric, value, metric, gates_comparator.get(metric, "?"),
                 threshold.get(metric, "?")))
    eligible_by_profile[profile] = eligible
    if not eligible:
        print("  R5 no survivor: escalate to the director with these measurements; the bounded fallback "
              "is `%s` (egui/iced hosts the canvas, G1-SLICE.14) until a re-spike" % rule.get("fallback"))
        winners[profile] = None
        traces[profile] = "R5 escalation"
        continue
    best = min(float(r[SCORED]) for r in eligible)
    best_by_profile[profile] = best
    within = [r for r in eligible if float(r[SCORED]) <= best * (1 + margin / 100.0)]
    for r in eligible:
        print("  R3 %-24s %s = %s ms%s" % (r["topology"], SCORED, r[SCORED],
                                           "   (inside the %s%% margin)" % rule.get("margin_pct")
                                           if r in within else ""))
    order = []
    for key in tiebreak:
        if key == "topology":
            order.append((lambda r: r["topology"],))
        elif key in ("peak_mb", "unproven_deps", SCORED):
            order.append((lambda r, k=key: float(r[k]),))
        else:
            bad("the tiebreak names `%s`, which is not a measured column" % key)
            order.append((lambda r: r["topology"],))
    chosen = sorted(within, key=lambda r: tuple(f(r) for (f,) in order))[0]
    winners[profile] = chosen["topology"]
    traces[profile] = "R3 (%d inside the margin, decided by %s)" % (len(within), " > ".join(tiebreak))
    print("  → winner: %s" % chosen["topology"])

# ── R4 one renderer where it is free ──────────────────────────────────────────────────────
outcome = "decided"
if any(w is None for w in winners.values()):
    outcome = "ESCALATE (%s)" % ", ".join(p for p, w in winners.items() if w is None)
else:
    distinct = set(winners.values())
    if len(distinct) == 1:
        outcome = "one topology for every profile: %s" % distinct.pop()
    else:
        # R4: a topology that survived in EVERY profile and stayed inside the margin of each profile's
        # best is preferred for both, because one renderer is one code path and one conformance surface.
        everywhere = [t for t in sorted({r["topology"] for r in results})
                      if all(any(e["topology"] == t for e in eligible_by_profile[p]) for p in profiles)]
        r4 = next((t for t in everywhere
                   if all(float(next(e for e in eligible_by_profile[p] if e["topology"] == t)[SCORED])
                          <= best_by_profile[p] * (1 + margin / 100.0) for p in profiles)), None)
        if r4:
            outcome = "R4 prefers one renderer: %s (inside the margin in every profile)" % r4
            for p in profiles:
                winners[p] = r4
                traces[p] = "R4 one-renderer preference over %s" % traces[p]
        else:
            outcome = "two renderers: %s" % ", ".join("%s=%s" % (p, w)
                                                     for p, w in sorted(winners.items()))
            print("-- R4 does not fire: no topology is inside the margin in every profile, so the record "
                  "that closes ADR-0002 must state the cost — two renderers, two conformance surfaces")

print("-- verdict")
for p in profiles:
    print("  · %-10s %s   [%s]" % (p, winners[p] or "no survivor — escalate", traces.get(p, "")))
print("spike verdict: %d row(s) / %d profile(s) / %s / %d refusal(s)"
      % (len(results), len(profiles), outcome, fails))
sys.exit(1 if fails else 0)
PY
rc=$?
exit "$rc"
