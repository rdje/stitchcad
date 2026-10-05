"""Actual scanner/name guard mutations; assertion reds and byte-identical restoration."""
from pathlib import Path
import re
import runpy
import subprocess
ROOT = Path(__file__).resolve().parents[4]
WORK = ROOT / 'target/formula_lex_mutations'
LEXER = ROOT / 'crates/sc-core/src/recipe/lexer.rs'
NAME = ROOT / 'crates/sc-core/src/name.rs'
TEST_SOURCE = ROOT / 'crates/sc-core/tests/formula_lex_contract.rs'
EXAMPLES = ROOT / 'docs/book/src/spec/formula-language/examples.md'
MUTABLE_SOURCES = (LEXER, NAME)
SOURCES = (*MUTABLE_SOURCES, TEST_SOURCE, EXAMPLES)
CASES = [
    ('ASCII preflight', LEXER, '!c.is_ascii()', 'false && !c.is_ascii()', 'unicode_preflight_refuses_before_any_valid_prefix_token'),
    ('identifier spelling', LEXER, 'if !valid_spelling(text) {', 'if false && !valid_spelling(text) {', 'only_three_keywords_are_reserved_and_spelling_matches_machine_tokens'),
    ('keyword role', NAME, '"if" => Some(GrammarKeyword::If),', '"if_else" => Some(GrammarKeyword::If),', 'only_three_keywords_are_reserved_and_spelling_matches_machine_tokens'),
    ('longest comparison', LEXER, "'<' if self.consume_equals()", "'<' if false && self.consume_equals()", 'paired_comparisons_use_longest_match'),
    ('decimal fraction', LEXER, 'if !self.chars.peek().is_some_and(|(_, c)| c.is_ascii_digit()) {', 'if false && !self.chars.peek().is_some_and(|(_, c)| c.is_ascii_digit()) {', 'decimals_require_nonempty_fraction'),
    ('source span', LEXER, '.map_or(self.source.len(), |(position, _)| *position);', '.map_or(self.source.len(), |(position, _)| position.saturating_sub(1));', 'binding_keeps_exact_roles_text_and_byte_positions'),
    ('first-error termination', LEXER, 'Err(error) => {\n                self.stopped = true;', 'Err(error) => {\n                self.stopped = false;', 'end_and_first_error_are_permanently_fused'),
    ('debug privacy', LEXER, '.field("source_bytes", &self.source.len())', '.field("source", &self.source)', 'debug_and_errors_do_not_dump_customer_source'),
    ('ASCII vertical tab', LEXER, "c.is_ascii_whitespace() || c == '\\x0b'", 'c.is_ascii_whitespace()', 'whitespace_gaps_preserve_later_unit_separator_authority'),
]
OUTPUT_FILES = tuple(WORK / ('mutation-%d.log' % index) for index in range(1, len(CASES) + 1)) + \
    (WORK / 'restored.log',)


def assertion_sites(source, path):
    """Current curated test macros, with Rust panic byte columns."""
    sites = set()
    for number, line in enumerate(source.splitlines(), 1):
        match = re.match(r'^(\s*)assert(?:_eq|_ne)?!\s*\(', line)
        if match is not None:
            sites.add((path, number, len(match.group(1).encode()) + 1))
    return sites


def assertion_failure(output):
    """A failed-body panic must name a guarded current assertion macro location."""
    parts = output.split(b'\nfailures:\n', 2)
    if len(parts) != 3:
        return False
    for match in re.finditer(
            rb'(?m)^thread\b[^\r\n]*? panicked at ([^:\r\n]+):([0-9]+):([0-9]+):\r?$', parts[1]):
        try:
            path = match.group(1).decode('utf-8')
        except UnicodeDecodeError:
            continue
        if (path, int(match.group(2)), int(match.group(3))) in ASSERTION_SITES:
            return True
    return False


runpy.run_path(str(ROOT / 'scripts/local_environment.py'))['enter_producer'](
    ROOT, directories=(WORK,), output_files=OUTPUT_FILES, sources=SOURCES, mutable_sources=MUTABLE_SOURCES)
WORK.mkdir(exist_ok=True)
ORIGINALS = {path: path.read_bytes() for path in (LEXER, NAME)}
ASSERTION_SITES = assertion_sites(TEST_SOURCE.read_text(), TEST_SOURCE.relative_to(ROOT).as_posix())
assert ASSERTION_SITES, 'current lexer test has no assertion macros'
try:
    for number, (name, path, before, after, test) in enumerate(CASES, 1):
        text = ORIGINALS[path].decode()
        assert text.count(before) == 1, (name, 'nonunique mutation anchor')
        path.write_text(text.replace(before, after, 1))
        result = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test', 'formula_lex_contract', test, '--', '--exact'], cwd=ROOT, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('mutation-%d.log' % number)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output and assertion_failure(output) and b'could not compile' not in output, (name, output.decode())
        print('mutation %d %s: rc=101, real contract assertion failed' % (number, name))
        path.write_bytes(ORIGINALS[path])
finally:
    for path, original in ORIGINALS.items():
        path.write_bytes(original)
    restored = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test', 'formula_lex_contract'],
                              cwd=ROOT, capture_output=True)
    (WORK / 'restored.log').write_bytes(restored.stdout + restored.stderr)
    assert restored.returncode == 0, ('restored lexer artifact failed', restored.stderr.decode())
assert all(path.read_bytes() == original for path, original in ORIGINALS.items())
print('Formula lexer mutations: 9 actual reds; both sources and actual compiled artifact restored')
