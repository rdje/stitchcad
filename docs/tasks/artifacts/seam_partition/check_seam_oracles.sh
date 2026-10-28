#!/usr/bin/env bash
# Exact immutable fixture checks,without native producers or scratch writes.
set -euo pipefail
if [ "$#" -ne 0 ]; then
  printf '%s\n' 'seam-oracles: no arguments accepted' >&2
  exit 2
fi
seam_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
seam_root="$(cd "$seam_dir/../../../.." && pwd)"
cd "$seam_root"
seam_count=0
for seam_oracle in "$seam_dir"/*_oracle.py; do
  test -f "$seam_oracle"
  case "${seam_oracle##*/}" in
    range_progress_oracle.py)
      python3 -I -B "$seam_oracle" | cmp - crates/sc-core/tests/fixtures/range_progress.tsv
      ;;
    uniform_ease_oracle.py)
      python3 -I -B "$seam_oracle" | cmp - crates/sc-core/tests/fixtures/uniform_ease.tsv
      ;;
    nonuniform_ease_oracle.py|notch_context_oracle.py|notch_quantity_oracle.py|original_comparison_oracle.py|original_projection_oracle.py|projection_workspace_oracle.py|reference_recomposition_oracle.py)
      python3 -I -B "$seam_oracle" --check
      ;;
    *)
      printf 'seam-oracles: unreviewed producer %s\n' "${seam_oracle##*/}" >&2
      exit 2
      ;;
  esac
  seam_count=$((seam_count + 1))
done
if [ "$seam_count" -ne 9 ]; then
  printf 'seam-oracles: expected9 complete producers,found%s\n' "$seam_count" >&2
  exit 2
fi
printf 'seam-oracles: %s exact producers/fixtures pass;rc=0\n' "$seam_count"
