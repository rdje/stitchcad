#!/usr/bin/env bash
# WHY: a file only this project can read is no evidence. No independent engine is available, so this is the nearest thing that is: a reader written from the format's description by a separate agent that never saw this code.
# WHERE: everything it writes is under target/, because the reader writes beside itself.
# A comparison that cannot fail proves nothing, so each garment is also compared with one row tampered with, and that has to be caught.
set -u
cd "$(dirname "$0")/../../../../.." || exit 2
HERE=docs/tasks/artifacts/grading/independent_reader
OUT=target/independent-reader
L="python3 -I -B scripts/local_environment.py --"
rm -rf "$OUT"
mkdir -p "$OUT"
$L cargo build -p sc-cli > "$OUT/build.log" 2>&1 || { echo "the program did not build"; exit 2; }
for garment in skirt bodice trousers; do
  DIR="$OUT/$garment"
  mkdir -p "$DIR"
  cp "$HERE/read_rul_independently.py" "$DIR/read_rul_independently.py"
  python3 -I -B - "$garment" "$DIR/session.json" <<'PY'
import json
import sys
garment, target = sys.argv[1:3]
session = json.load(open(f'conformance/reference-{garment}/session.json', encoding='utf-8'))
steps = [step for step in session['steps'] if step['do'] != 'save']
steps += [
    {"do": "regenerate"},
    {"do": "grade", "rules_path": "exports/rules.json"},
    {"do": "export_graded", "mode": "embedded_rules", "semantics": "sew_as_1", "path": "exports/graded"},
    {"do": "save"},
]
session['steps'] = steps
json.dump(session, open(target, 'w', encoding='utf-8'))
PY
  ./target/debug/stitchcad replay "$DIR/session.json" "$DIR/project" > "$DIR/transcript.json" || { echo "$garment did not replay"; exit 2; }
  python3 -I -B "$DIR/read_rul_independently.py" "$DIR/project/exports/graded.rul" "$DIR/project/exports/graded.dxf" > "$DIR/reader.log" || { echo "$garment: the reader refused"; cat "$DIR/reader.log"; exit 1; }
  printf '%s: ' "$garment"
  python3 -I -B "$HERE/compare_with_recipe.py" "$DIR/independent_reader_output.tsv" "$DIR/project/exports/rules.json" "$DIR/transcript.json" || exit 1
  python3 -I -B - "$DIR/independent_reader_output.tsv" "$DIR/tampered.tsv" <<'PY'
import sys
rows = open(sys.argv[1], encoding='ascii').read().splitlines()
fields = rows[-1].split('\t')
fields[3] = '%.2f' % (float(fields[3]) + 0.02)
rows[-1] = '\t'.join(fields)
open(sys.argv[2], 'w', encoding='ascii').write('\n'.join(rows) + '\n')
PY
  if python3 -I -B "$HERE/compare_with_recipe.py" "$DIR/tampered.tsv" "$DIR/project/exports/rules.json" "$DIR/transcript.json" > "$DIR/tampered.log"; then
    echo "$garment: a row 20 um out was not caught, so this comparison proves nothing"
    exit 1
  fi
done
echo "independent reader: agrees, and a tampered row is caught"
