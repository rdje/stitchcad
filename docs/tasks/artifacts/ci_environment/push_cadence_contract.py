"""Exercise the actual read-only push checker, including Git failures and body faults."""
from pathlib import Path
import os
import runpy
import subprocess

ROOT = Path(__file__).resolve().parents[4]
ENTRY = ROOT / 'scripts/check_push_due.sh'
SOURCES = (ENTRY, ROOT / 'COMMIT.md', ROOT / 'Makefile',
           ROOT / 'docs/book/src/build-and-checks.md')
runpy.run_path(str(ROOT / 'scripts/local_environment.py'))['enter_producer'](
    ROOT, sources=SOURCES)
ORIGINAL = ENTRY.read_bytes()

# Only specified Git observations are simulated. Every other Git call is real and read-only;
# no fixture repository is created and no source or Git metadata is modified.
WRAPPER = r'''
git() {
  if [ "$1" = rev-list ] && [ "$2" = --count ]; then
    [ "$PUSH_CADENCE_MODE" != count-failure ] || return 27
    printf '%s\n' "$PUSH_CADENCE_COUNT"
    return 0
  fi
  if [ "$1" = rev-parse ]; then
    [ "$PUSH_CADENCE_MODE:$2" != root-failure:--show-toplevel ] || return 27
    [ "$PUSH_CADENCE_MODE:${3:-}" != branch-failure:HEAD ] || return 27
    [ "$PUSH_CADENCE_MODE:${3:-}" != upstream-failure:--symbolic-full-name ] || return 27
  fi
  command git "$@"
}
export -f git
exec bash -s
'''


def observe(text, count='0', mode='count', base=None):
    environment = dict(os.environ)
    environment.pop('PUSH_DUE_BASE', None)
    environment.update(PUSH_CADENCE_COUNT=count, PUSH_CADENCE_MODE=mode)
    if base is not None:
        environment['PUSH_DUE_BASE'] = base
    return subprocess.run(['bash', '-c', WRAPPER], cwd=ROOT, env=environment,
                          input=text, capture_output=True)


def controls(text):
    checks = 0
    for count, expected in [('0', 0), ('1', 0), ('20', 0), ('399', 0),
                            ('400', 1), ('401', 1), ('9' * 40, 1)]:
        result = observe(text, count)
        assert result.returncode == expected, ('cadence boundary', count, result)
        assert (b'PUSH DUE' in result.stdout) == (expected == 1), ('cadence verdict', count)
        checks += 1
    for count, mode, reason, base in [
        ('0', 'count-failure', b'cannot count unpushed commits', None),
        ('', 'count', b'invalid commit count', None),
        ('broken', 'count', b'invalid commit count', None),
        ('-1', 'count', b'invalid commit count', None),
        ('0400', 'count', b'invalid commit count', None),
        ('400\n400', 'count', b'invalid commit count', None),
        ('0', 'root-failure', b'not inside a git repository', None),
        ('0', 'branch-failure', b'cannot identify current branch', None),
        ('0', 'upstream-failure', b'no upstream', None),
        ('0', 'count', b'does not resolve to a commit', 'refs/heads/stitchcad-absent-contract-base'),
    ]:
        result = observe(text, count, mode, base)
        assert result.returncode == 2 and reason in result.stderr, ('Git refusal', mode, count, result)
        assert b'nothing to push' not in result.stdout and b'PUSH DUE' not in result.stdout
        checks += 1
    return checks


def main():
    checks = controls(ORIGINAL)
    faults = (
        ('threshold', b'[ "$ahead" -ge "$cadence" ]', b'[ "$ahead" -gt "$cadence" ]'),
        ('early push', b'[ "$ahead" -ge "$cadence" ]', b'[ "$ahead" -ge 1 ]'),
        ('count failure', b"|| refuse 'cannot count unpushed commits'", b'|| ahead=0'),
        ('count validation', b'[[ "$ahead" =~ ^(0|[1-9][0-9]*)$ ]]', b'true'),
    )
    for label, before, after in faults:
        assert ORIGINAL.count(before) == 1, ('body fault anchor', label)
        try:
            controls(ORIGINAL.replace(before, after, 1))
        except AssertionError as error:
            assert error.args[0][0] in ('cadence boundary', 'Git refusal'), (label, error)
        else:
            raise AssertionError(('body fault accepted', label))
        print('  actual push checker body red:', label)
    # These integrations use real Git counts; one base includes the former exceptional paths.
    parents = subprocess.check_output(['git', 'log', '-1', '--format=%P', '--',
                                       '.doctrine/history_archive/windows.json'], cwd=ROOT).decode().split()
    ancestor = parents[0] if parents else ''
    assert ancestor, 'historical integration base absent'
    changed = subprocess.check_output(['git', 'diff', '--name-only', ancestor + '..HEAD', '--',
                                       '.doctrine', '.github/workflows', '.githooks'], cwd=ROOT)
    assert changed, 'historical comparison no longer covers formerly exceptional paths'
    for base in ['HEAD', ancestor]:
        count = int(subprocess.check_output(['git', 'rev-list', '--count', base + '..HEAD'], cwd=ROOT))
        environment = dict(os.environ, PUSH_DUE_BASE=base)
        result = subprocess.run(['bash', str(ENTRY)], cwd=ROOT, env=environment, capture_output=True)
        assert result.returncode == (1 if count >= 400 else 0), ('real Git verdict', base, result)
        assert ('%d unpushed commit(s)' % count).encode() in result.stdout
        checks += 1
    assert ENTRY.read_bytes() == ORIGINAL, 'push checker source changed'
    assert 'No automatic exceptions.' in SOURCES[1].read_text()
    assert '400-commit push cadence' in SOURCES[2].read_text()
    assert 'CI/doctrine/hook changes have no automatic exception.' in SOURCES[3].read_text()
    print('push cadence: %d controls / four actual body reds / real historical Git counts / 0 fail' % checks)


if __name__ == '__main__':
    main()
