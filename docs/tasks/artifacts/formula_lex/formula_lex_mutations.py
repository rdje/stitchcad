"""Actual scanner/name guard mutations; assertion reds and byte-identical restoration."""
from pathlib import Path
import runpy
import subprocess
ROOT = Path(__file__).resolve().parents[4]
WORK = ROOT / 'target/formula_lex_mutations'
LEXER = ROOT / 'crates/sc-core/src/recipe/lexer.rs'
NAME = ROOT / 'crates/sc-core/src/name.rs'
SOURCES = (LEXER, NAME)
runpy.run_path(str(ROOT / 'scripts/local_environment.py'))['enter_producer'](
    ROOT, directories=(WORK,), sources=SOURCES)
WORK.mkdir(exist_ok=True)
ORIGINALS = {path: path.read_bytes() for path in (LEXER, NAME)}
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
try:
    for number, (name, path, before, after, test) in enumerate(CASES, 1):
        text = ORIGINALS[path].decode()
        assert text.count(before) == 1, (name, 'nonunique mutation anchor')
        path.write_text(text.replace(before, after, 1))
        result = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test', 'formula_lex_contract', test, '--', '--exact'], cwd=ROOT, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('mutation-%d.log' % number)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output and b'assertion' in output and b'could not compile' not in output, (name, output.decode())
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
