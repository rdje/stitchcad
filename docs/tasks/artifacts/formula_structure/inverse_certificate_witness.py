"""Pure stdout compact inverse proof witness from complete metadata; never replaces full reading."""
from pathlib import Path
import hashlib
import json
import runpy

ROOT = Path(__file__).resolve().parents[4]
PROFILE = ROOT / 'scripts/local_environment.py'
READER = Path(__file__).with_name('verify_inverse_certificate.py')
MANIFEST = ROOT / 'target/inverse_threshold_certificate/0284/manifest.json'


def witness():
    profile = runpy.run_path(str(PROFILE))
    profile['plan_producer'](ROOT, sources=(Path(__file__), PROFILE, READER, MANIFEST))
    reader = runpy.run_path(str(READER))
    assert MANIFEST.stat().st_size <= 1_000_000
    manifest = json.loads(MANIFEST.read_text())
    reader['verify_manifest'](manifest)
    assert manifest['complete'], 'partial coverage is not a complete witness'
    reader['verify_sources'](manifest, profile)
    params = manifest['parameters']
    minimum = min(chunk['minimum_margin_scaled'] for chunk in manifest['chunks'])
    assert minimum > 0
    # For slopes in[0,1], d(atan(r)/(2*pi))/dr>=1/(4*pi)>1/16.
    angular_margin_bits = params['fraction_bits'] - (minimum.bit_length() - 1) + 4
    rows = [('schema', 1), ('population', params['population']), ('chunk_count', params['chunk_count']),
            ('row_bytes', params['row_bytes']), ('fraction_bits', params['fraction_bits']),
            ('turn_band_bits', params['turn_band_bits']), ('slope_denominator_bound', reader['Q']),
            ('minimum_slope_margin_scaled', minimum), ('angular_margin_bits', angular_margin_bits),
            ('combined_chunk_sha256', hashlib.sha256(b''.join(bytes.fromhex(chunk['sha256'])
                for chunk in manifest['chunks'])).hexdigest()), ('producer_sha256', manifest['producer_sha256'])]
    for name in (*reader['CRITICAL'], Path(__file__).relative_to(ROOT).as_posix()):
        expected = reader['digest'](ROOT / name)
        if name in manifest['frozen_sources']:
            assert expected == manifest['frozen_sources'][name]
        rows.append(('source:' + name, expected))
    return rows


if __name__ == '__main__':
    print('# Complete inverse witness; full producer and independent reader passed in0286.')
    print('# Metadata extraction supplements full reading; phase margin is not a native runtime budget.')
    for key, value in witness():
        print(str(key) + '\t' + str(value))
