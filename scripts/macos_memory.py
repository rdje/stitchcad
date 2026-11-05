#!/usr/bin/env python3
"""Read-only cache-adjusted macOS RAM sample for trend watching; never a process launcher or UI replica."""
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
# Director 2026-11-06: above this line and still rising is worth flagging; nothing stops at it.
FLAG_PERCENT = 90


def estimate(text, physical, pressure):
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
    if not 0 < physical < 2**64 or physical % page_size or pressure not in (1, 2, 4):
        raise ValueError('invalid physical size or pressure')
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
    return {
        'metric': 'macos-cache-adjusted-estimate', 'physical_bytes': physical,
        'used_bytes': used, 'used_percent': round(100 * used / physical, 3),
        'cached_bytes': cached, 'occupied_bytes': occupied, 'page_size': page_size,
        'pressure_level': pressure, 'above_flag_line': used * 100 >= physical * FLAG_PERCENT,
    }


def _read_command(argv):
    result = subprocess.run(argv, capture_output=True, text=True, timeout=5, check=True)
    if result.stderr.strip():
        raise ValueError('unexpected memory-command diagnostic')
    return result.stdout


def sample():
    if sys.platform != 'darwin':
        raise ValueError('macOS host required')
    physical = int(_read_command(['/usr/sbin/sysctl', '-n', 'hw.memsize']).strip())
    pressure = int(_read_command([
        '/usr/sbin/sysctl', '-n', 'kern.memorystatus_vm_pressure_level']).strip())
    result = estimate(_read_command(['/usr/bin/vm_stat']), physical, pressure)
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
    assert result['above_flag_line'] is False
    # Pressure is reported context; it never turns a reading into a verdict.
    assert estimate(raw, total, 4)['above_flag_line'] is False
    assert estimate(raw, total, 4)['pressure_level'] == 4
    def boundary(pages):
        return 'Mach Virtual Memory Statistics: (page size of 4096 bytes)\n' + '\n'.join(
            name + ': ' + str(pages if i == 0 else 0) + '.' for i, name in enumerate(FIELDS))
    assert estimate(boundary(89), 409600, 1)['above_flag_line'] is False
    assert estimate(boundary(90), 409600, 1)['above_flag_line'] is True
    for index, (text, size, pressure) in enumerate((
        ('', total, 1), (raw + '\nPages active: 1.', total, 1),
        (raw.replace('332861.', '-1.'), total, 1),
        (raw.replace('16384 bytes', '16383 bytes'), total, 1),
        (raw.replace('332861.', '999999999.'), total, 1),
        (raw.replace('245403.', '1550000.'), total, 1),
        (raw, 0, 1), (raw, total, 0),
    )):
        try:
            estimate(text, size, pressure)
        except ValueError:
            continue
        raise AssertionError(('invalid memory observation accepted', index))
    print('macos-memory: frozen cache discrimination, pressure context, exact flag line and refusals OK')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--self-test', action='store_true')
    args = parser.parse_args()
    if args.self_test:
        self_test()
        return 0
    try:
        result = sample()
    except (ValueError, OSError, subprocess.SubprocessError) as error:
        print('macos-memory: measurement refused: ' + str(error), file=sys.stderr)
        return 2
    print(json.dumps(result, sort_keys=True))
    return 0


if __name__ == '__main__':
    sys.exit(main())
