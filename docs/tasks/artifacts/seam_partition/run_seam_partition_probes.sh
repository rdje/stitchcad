#!/usr/bin/env bash
# Make's discovered probe entry and the actual CI step use this same contract.
set -euo pipefail
seam_probe_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
bash "$seam_probe_dir/check_seam_oracles.sh" "$@"
python3 -I -B "$seam_probe_dir/oracle_watch_contract.py"
