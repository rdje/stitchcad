#!/usr/bin/env python3
"""Read-only cache-adjusted macOS RAM estimate; never a process launcher or exact UI replica."""
import argparse
from datetime import datetime, timezone
import json
import re
import subprocess
import sys


FIELDS = (
    'Pages active', 'Pages inactive', 'Pages speculative', 'Pages wired down',
    'Pages occupied by compressor', 'Pages purgeable', 'File-backed pages',
)


def estimate(text, physical, pressure, reserve_mib=0):
    """Use Stats' published category arithmetic, not top's cache-inclusive occupancy.

    Independent source: https://github.com/exelban/stats/blob/master/Modules/RAM/readers.swift
    Counters are one vm_stat observation; pressure/physical size are separate observations.
    """
    if len(text) > 65536:
        raise ValueError('oversized vm_stat observation')
    header = re.search(r'^Mach Virtual Memory Statistics: \(page size of ([0-9]+) bytes\)', text)
    if header is None:
        raise ValueError('missing page size')
    page_size = int(header[1])
    if page_size <= 0 or page_size & (page_size - 1):
        raise ValueError('invalid page size')
    if (not 0 < physical < 2**64 or physical % page_size or pressure not in (1, 2, 4)
            or not 0 <= reserve_mib <= physical // (1024 * 1024)):
        raise ValueError('invalid physical size, pressure or reserve')
    counts = {}
    for field in FIELDS:
        matches = re.findall(r'^' + re.escape(field) + r':\s*([0-9]+)\.\s*$', text, re.M)
        if len(matches) != 1:
            raise ValueError('missing, duplicate or malformed counter: ' + field)
        counts[field] = int(matches[0])
    if any(count * page_size > physical for count in counts.values()):
        raise ValueError('counter exceeds physical RAM')
    occupied = sum(counts[field] for field in FIELDS[:5]) * page_size
    cached = sum(counts[field] for field in FIELDS[5:]) * page_size
    used = occupied - cached
    if not 0 <= cached <= occupied <= physical or not 0 <= used <= physical:
        raise ValueError('inconsistent memory observation')
    projected = used + reserve_mib * 1024 * 1024
    if pressure != 1:
        admission = 'defer-pressure'
    elif used * 100 >= physical * 88:
        admission = 'hard-ceiling'
    elif projected * 100 >= physical * 86:
        admission = 'defer-headroom'
    else:
        admission = 'within-budget'
    return {
        'metric': 'macos-cache-adjusted-estimate', 'physical_bytes': physical,
        'used_bytes': used, 'used_percent': round(100 * used / physical, 3),
        'cached_bytes': cached, 'occupied_bytes': occupied, 'page_size': page_size,
        'pressure_level': pressure, 'reserve_mib': reserve_mib,
        'projected_percent': round(100 * projected / physical, 3),
        'admission': admission,
    }


def _read_command(argv):
    result = subprocess.run(argv, capture_output=True, text=True, timeout=5, check=True)
    if result.stderr.strip():
        raise ValueError('unexpected memory-command diagnostic')
    return result.stdout


def sample(reserve_mib):
    if sys.platform != 'darwin':
        raise ValueError('macOS host required')
    physical = int(_read_command(['/usr/sbin/sysctl', '-n', 'hw.memsize']).strip())
    pressure = int(_read_command([
        '/usr/sbin/sysctl', '-n', 'kern.memorystatus_vm_pressure_level']).strip())
    result = estimate(_read_command(['/usr/bin/vm_stat']), physical, pressure, reserve_mib)
    result['observed_utc'] = datetime.now(timezone.utc).isoformat()
    return result


def self_test():
    # Frozen independent OS observation, 2026-11-05 21:27 UTC. Not simultaneous UI parity.
    raw = 'Mach Virtual Memory Statistics: (page size of 16384 bytes)\n' + '\n'.join(
        name + ': ' + str(value) + '.' for name, value in zip(FIELDS, (
            332861, 330183, 1593, 290226, 578765, 28, 245403)))
    total = 25769803776
    result = estimate(raw, total, 1)
    # Independent subtraction: 1533628 occupied pages minus 245431 cache pages.
    assert result['used_bytes'] == 1288197 * 16384
    assert result['occupied_bytes'] > total * 97 // 100
    assert result['used_bytes'] < total * 82 // 100
    assert result['admission'] == 'within-budget'
    assert estimate(raw, total, 2)['admission'] == 'defer-pressure'
    assert estimate(raw, total, 4)['admission'] == 'defer-pressure'
    assert estimate(raw, total, 1, 1024)['admission'] == 'defer-headroom'
    def boundary(pages):
        return 'Mach Virtual Memory Statistics: (page size of 4096 bytes)\n' + '\n'.join(
            name + ': ' + str(pages if i == 0 else 0) + '.' for i, name in enumerate(FIELDS))
    assert estimate(boundary(85), 409600, 1)['admission'] == 'within-budget'
    assert estimate(boundary(86), 409600, 1)['admission'] == 'defer-headroom'
    assert estimate(boundary(88), 409600, 1)['admission'] == 'hard-ceiling'
    for index, (text, size, pressure, reserve) in enumerate((
        ('', total, 1, 0), (raw + '\nPages active: 1.', total, 1, 0),
        (raw.replace('332861.', '-1.'), total, 1, 0),
        (raw.replace('16384 bytes', '16383 bytes'), total, 1, 0),
        (raw.replace('332861.', '999999999.'), total, 1, 0),
        (raw.replace('245403.', '1550000.'), total, 1, 0),
        (raw, 0, 1, 0), (raw, total, 0, 0), (raw, total, 1, -1),
        (raw, total, 1, 10**100),
    )):
        try:
            estimate(text, size, pressure, reserve)
        except ValueError:
            continue
        raise AssertionError(('invalid memory observation accepted', index))
    print('macos-memory: frozen cache discrimination, pressure, exact thresholds and refusals OK')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--self-test', action='store_true')
    parser.add_argument('--reserve-mib', type=int, default=0,
                        help='expected additional job peak; projected usage must stay below 86%%')
    args = parser.parse_args()
    if args.self_test:
        self_test()
        return 0
    try:
        result = sample(args.reserve_mib)
    except (ValueError, OSError, subprocess.SubprocessError) as error:
        print('macos-memory: measurement refused: ' + str(error), file=sys.stderr)
        return 2
    print(json.dumps(result, sort_keys=True))
    return 0 if result['admission'] == 'within-budget' else 1


if __name__ == '__main__':
    sys.exit(main())
