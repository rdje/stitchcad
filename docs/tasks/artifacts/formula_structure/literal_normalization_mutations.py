"""Compiled faults in the actual product normalizer, with public assertion verdicts."""
from pathlib import Path
import subprocess
ROOT=Path(__file__).resolve().parents[4]
SOURCE=ROOT/'crates/sc-core/src/recipe/literal.rs'
ORIGINAL=SOURCE.read_bytes()
WORK=ROOT/'target/formula_literal_mutations'
WORK.mkdir(exist_ok=True)
FIXTURES='independent_fraction_fixtures_cover_conversion_width_and_domain'
KINDS='kinds_quanta_and_angle_turns_are_preserved'
CASES=[
 ('inch conversion', '(FormulaLiteralKind::Length, 25400)', '(FormulaLiteralKind::Length, 25000)',FIXTURES),
 ('percent meaning', '(FormulaLiteralKind::Ratio, 10000)', '(FormulaLiteralKind::Ratio, 1000000)',KINDS),
 ('decimal kind', "None if number.contains('.') => (FormulaLiteralKind::Ratio, 1000000)", "None if number.contains('.') => (FormulaLiteralKind::Count, 1)",KINDS),
 ('raw angle direction modulo', '        magnitude,\n', '        magnitude: if kind == FormulaLiteralKind::Angle { magnitude % 360000000 } else { magnitude },\n',KINDS),
 ('literal narrowed', '        magnitude,\n', '        magnitude: u128::from(magnitude as u64),\n',KINDS),
 ('raw mantissa width', 'if significant > scale + 39 {', 'if significant > 39 {',FIXTURES),
 ('fraction zero cancellation', "fraction.trim_end_matches('0')", 'fraction', 'huge_spelling_refuses_or_reduces_without_new_language_caps'),
 ('mantissa reduction', 'divide_decimal(&mut decimal, prime);', 'divide_decimal(&mut decimal, 1);',FIXTURES),
 ('unit denominator cancellation', 'multiplier /= prime;', 'multiplier /= 1;',FIXTURES),
 ('denominator width hidden', '.checked_mul(prime)\n', '.checked_mul(prime).or(Some(1))\n','width_checks_precede_rounding_and_length_checks_follow_it'),
 ('length guard bypassed', 'magnitude > maximum {', 'false && magnitude > maximum {','width_checks_precede_rounding_and_length_checks_follow_it'),
 ('diagnostic family', '        "formula_domain"', '        "formula_parse"',FIXTURES),
 ('source identity', '        self.number\n', '        "0"\n','source_location_privacy_and_unary_identity_survive_conversion'),
]
try:
 for i,(name,before,after,test) in enumerate(CASES,1):
  text=ORIGINAL.decode()
  assert text.count(before)==1,(name,'actual anchor not unique',text.count(before))
  SOURCE.write_text(text.replace(before,after,1))
  result=subprocess.run(['cargo','test','-p','sc-core','--test','formula_literal_contract',test,'--','--exact'],cwd=ROOT,capture_output=True)
  output=result.stdout+result.stderr
  (WORK/('mutation-%d.log'%i)).write_bytes(output)
  assert result.returncode==101 and b'test result: FAILED.' in output and b'assertion' in output and b'could not compile' not in output,(name,output.decode())
  print('literal mutation %d %s: rc=101, actual compiled public assertion red'%(i,name))
  SOURCE.write_bytes(ORIGINAL)
finally:
 SOURCE.write_bytes(ORIGINAL)
assert SOURCE.read_bytes()==ORIGINAL
print('literal normalization mutations: thirteen compiled actual assertion reds; source restored byte-identically')
