"""Check sealed payload identities without normalizing retained source bytes.

This is descriptor integrity, not proof of the separately named historical Git
intervals. Coverage and source-receipt checks remain independent.
"""
import base64
import binascii
import collections
import hashlib
from pathlib import Path
import re
import sys


ENCODING = (b'- **Encoding:** base64, wrapped at100 characters; '
            b'decoded source intervals concatenate exactly.')
IDENTITY = re.compile(
    rb'^- \*\*(?:Sealed identity|Identity):\*\* ([0-9]+) lines'
    rb'(?:, ([0-9]+) bytes)?,\s+`?sha256:([0-9a-f]{64})`?', re.M)
INTERVAL = re.compile(
    rb'^- Whole source interval: [A-Za-z0-9_./-]+ bytes [0-9]+'
    + '–'.encode() + rb'[0-9]+, [0-9]+ bytes, sha256:[0-9a-f]{64}$', re.M)


def check(data):
    """Return (profile, payload lines, payload bytes), or a named refusal."""
    header, separator, body = data.partition(b'\n---\n\n')
    if not separator:
        raise ValueError('no --- rule and blank line separating descriptor from content')
    interval_lines = [line for line in header.splitlines()
                      if line.startswith(b'- Whole source interval:')]
    if interval_lines and len(INTERVAL.findall(header)) != len(interval_lines):
        raise ValueError('malformed whole-source interval profile')
    encodings = [line for line in header.splitlines() if line.startswith(b'- **Encoding:**')]
    if encodings and (encodings != [ENCODING] or not interval_lines):
        raise ValueError('unsupported encoding or missing whole-source interval profile')
    profile = 'whole-raw' if interval_lines else 'legacy'
    if encodings:
        profile = 'whole-base64'
        match = re.fullmatch(rb'```base64\n([A-Za-z0-9+/=\n]+)```\n', body)
        if not match or any(not 0 < len(line) <= 100 for line in match[1].splitlines()):
            raise ValueError('malformed base64 fence or line width')
        try:
            body = base64.b64decode(b''.join(match[1].splitlines()), validate=True)
        except binascii.Error as error:
            raise ValueError('malformed base64 payload') from error
        if len(body) > 160000:
            raise ValueError('decoded payload exceeds archive record bound')
    elif profile == 'legacy':
        if body.endswith(b'\n') and body.splitlines()[-1] == b'':
            raise ValueError('ends with a blank line: legacy sealed content requires exactly one terminal newline')
        if not body.endswith(b'\n'):
            raise ValueError('legacy sealed content requires exactly one terminal newline')
    identities = list(IDENTITY.finditer(header))
    if len(identities) != 1:
        raise ValueError('expected one payload identity with line count and sha256')
    lines, size, digest = identities[0].groups()
    if profile != 'legacy' and size is None:
        raise ValueError('whole-source payload identity requires byte count')
    if hashlib.sha256(body).hexdigest().encode() != digest:
        raise ValueError('payload sha256 does not match descriptor')
    if len(body.splitlines()) != int(lines):
        raise ValueError('payload line count does not match descriptor')
    if size is not None and len(body) != int(size):
        raise ValueError('payload byte count does not match descriptor')
    return profile, len(body.splitlines()), len(body)


def main():
    profiles = collections.Counter()
    failures = 0
    for path in sorted(Path(sys.argv[1]).glob('*.md')):
        try:
            profile, lines, size = check(path.read_bytes())
        except (OSError, ValueError) as error:
            failures += 1
            print('DESCRIPTOR FAIL %s %s' % (path.name, error))
        else:
            profiles[profile] += 1
            print('DESCRIPTOR PASS %s: %s, %d lines / %d bytes reproduce identity' %
                  (path.name, profile, lines, size))
    print('DESCRIPTOR-PROFILES PASS ' + ', '.join('%s=%d' % row for row in sorted(profiles.items())))
    return int(bool(failures))


if __name__ == '__main__':
    sys.exit(main())
