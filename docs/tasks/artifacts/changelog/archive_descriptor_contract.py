"""Independent frozen payloads and copied-descriptor faults for D310."""
from pathlib import Path
import runpy

CHECK = runpy.run_path(str(Path(__file__).with_name('archive_descriptors.py')))['check']
# SHA-256 values independently obtained with native shasum, not the checker.
SINGLE = b'b6a98d9ce9a2d9149288fa3df42d377c3e42737afdcdaf714e33c0a100b51060'
BLANK = b'1d9e9bdc4ffd505aa2ee64c024f8bece1a280d14e5b47807529fe12e8668effe'
NO_LF = b'8ed3f6ad685b959ead7022518e1af76cd816f8e8ec7ccdda1ed4018e8f2223f8'
INTERVAL = (b'- Whole source interval: CHANGELOG.md bytes 0' + '–'.encode()
            + b'7, 7 bytes, sha256:' + BLANK + b'\n')
ENCODING = (b'- **Encoding:** base64, wrapped at100 characters; '
            b'decoded source intervals concatenate exactly.\n')


def record(body, lines, size, checksum, extra=b'', label=b'Sealed identity'):
    return (b'# Frozen descriptor control\n\n- **' + label + b':** '
            + str(lines).encode() + b' lines, ' + str(size).encode() + b' bytes, sha256:'
            + checksum + b'\n' + extra + b'\n---\n\n' + body)


legacy = record(b'alpha\n', 1, 6, SINGLE)
raw_blank = record(b'alpha\n\n', 2, 7, BLANK, INTERVAL)
raw_no_lf = record(b'alpha', 1, 5, NO_LF,
                   INTERVAL.replace(b'7, 7 bytes', b'5, 5 bytes').replace(BLANK, NO_LF))
encoded = record(b'```base64\nYWxwaGEKCg==\n```\n', 2, 7, BLANK,
                 INTERVAL + ENCODING, b'Identity')
positives = [(legacy, ('legacy', 1, 6)), (raw_blank, ('whole-raw', 2, 7)),
             (raw_no_lf, ('whole-raw', 1, 5)), (encoded, ('whole-base64', 2, 7))]
for data, expected in positives:
    assert CHECK(data) == expected, expected

faults = [
    (record(b'alpha\n\n', 2, 7, BLANK), 'blank line'),
    (record(b'alpha', 1, 5, NO_LF), 'terminal newline'),
    (legacy.replace(b'alpha', b'Alpha'), 'sha256'),
    (raw_blank.replace(b'alpha', b'Alpha'), 'sha256'),
    (raw_no_lf + b'\n', 'sha256'),
    (encoded.replace(b'YWxwaGEKCg==', b'QWxwaGEKCg=='), 'sha256'),
    (legacy.replace(b'1 lines', b'2 lines'), 'line count'),
    (raw_blank.replace(b'7 bytes, sha256:', b'8 bytes, sha256:', 1), 'byte count'),
    (encoded.replace(b'Identity:**', b'Absent:**'), 'one payload identity'),
    (legacy.replace(b'\n---\n\n', b'\n----\n\n'), 'separating'),
    (raw_blank.replace(b'0' + '–'.encode() + b'7', b'invalid'), 'malformed whole-source'),
    (encoded.replace(b'base64, wrapped', b'base32, wrapped'), 'unsupported encoding'),
    (encoded.replace(INTERVAL, b''), 'missing whole-source'),
    (encoded.replace(b'```base64', b'```text'), 'base64 fence'),
    (encoded.replace(b'YWxwaGEKCg==', b'YWxwaGEKCg='), 'malformed base64'),
    (encoded.replace(b'YWxwaGEKCg==', b'A' * 104), 'line width'),
    (raw_blank.replace(b'7 bytes, sha256:', b'sha256:', 1), 'byte count'),
    (legacy.replace(b'\n---', b'\n- **Identity:** 1 lines, 6 bytes, sha256:' + SINGLE + b'\n---'),
     'one payload identity'),
]
for data, reason in faults:
    try:
        CHECK(data)
    except ValueError as error:
        assert reason in str(error), (reason, str(error))
    else:
        raise AssertionError('accepted descriptor fault: ' + reason)

# Changing only coverage cannot affect payload identity: the ledger judges that
# independently, and its COVERAGE-RED arm must remain the sole refusal.
assert CHECK(raw_blank.replace(b'# Frozen descriptor control', b'- **Coverage:** WRONG')) == positives[1][1]
print('archive descriptor contract: 4 frozen profiles / %d refusals / independent coverage pass; rc=0' % len(faults))
