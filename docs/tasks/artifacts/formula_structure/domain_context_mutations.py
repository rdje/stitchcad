"""D90/D92: actual context/rendering mutations compile, fail assertions and restore all sources."""
from pathlib import Path
import subprocess
ROOT = Path(__file__).resolve().parents[4]
WORK = ROOT / 'target/domain_context_mutations'
WORK.mkdir(exist_ok=True)
CASES = [
 ('display operation', 'crates/sc-units/src/error.rs', '"{operation}: {kind} {value} is outside the declared domain (limit {limit})"', '"{kind} {value} is outside the declared domain (limit {limit})"', 'units'),
 ('display invented cause', 'crates/sc-units/src/error.rs', '"{operation}: {kind} {value} is outside the declared domain (limit {limit})"', '"{operation}: {kind} {value} is outside the declared domain (limit {limit}); unit-conversion bug"', 'units'),
 ('display false relation', 'crates/sc-units/src/error.rs', 'is outside the declared domain', 'exceeds the declared domain', 'units'),
 ('direct length context', 'crates/sc-units/src/length.rs', 'Self::from_micrometres_for(micrometres, "Length::from_micrometres")', 'Self::from_micrometres_for(micrometres, "wrong")', 'units'),
 ('direct area context', 'crates/sc-units/src/length.rs', 'Self::from_square_micrometres_for(value, "Area::from_square_micrometres")', 'Self::from_square_micrometres_for(value, "wrong")', 'units'),
 ('rational input context', 'crates/sc-units/src/length.rs', 'Self::from_micrometres_for(um, "Length::from_rational")', 'Self::from_micrometres_for(um, "Length::from_micrometres")', 'units'),
 ('float input context', 'crates/sc-units/src/length.rs', 'Self::from_micrometres_for(um, "Length::from_f64_in")', 'Self::from_micrometres_for(um, "Length::from_micrometres")', 'units'),
 ('addition context', 'crates/sc-units/src/length.rs', 'Self::from_micrometres_for(truncated, "Length::checked_add")', 'Self::from_micrometres_for(truncated, "Length::from_micrometres")', 'units'),
 ('subtraction context', 'crates/sc-units/src/length.rs', 'Self::from_micrometres_for(truncated, "Length::checked_sub")', 'Self::from_micrometres_for(truncated, "Length::from_micrometres")', 'units'),
 ('multiplication context', 'crates/sc-units/src/length.rs', 'Self::from_micrometres_for(truncated, "Length::checked_mul_i64")', 'Self::from_micrometres_for(truncated, "Length::from_micrometres")', 'units'),
 ('ratio scale context', 'crates/sc-units/src/ratio.rs', 'Length::from_micrometres_for(um, "Ratio::scale")', 'Length::from_micrometres_for(um, "Length::from_micrometres")', 'units'),
 ('range parameter context', 'crates/sc-core/src/ontology/range.rs', 'operation: "IdentityLedger::resolve_range",\n        kind: "range parameter",', 'operation: "wrong",\n        kind: "range parameter",', 'core'),
 ('range width context', 'crates/sc-core/src/ontology/range.rs', 'operation: "IdentityLedger::resolve_range",\n        kind: "positive range width",', 'operation: "wrong",\n        kind: "positive range width",', 'core'),
 ('edge parameter context', 'crates/sc-core/src/ontology/topology.rs', 'operation: "IdentityLedger::resolve",\n        kind: "edge parameter",', 'operation: "wrong",\n        kind: "edge parameter",', 'core'),
]
ORIGINAL = {path: (ROOT / path).read_bytes() for _, path, _, _, _ in CASES}
try:
 for number, (name, path, before, after, family) in enumerate(CASES, 1):
    text = ORIGINAL[path].decode()
    assert text.count(before) == 1, (name, 'nonunique anchor')
    (ROOT / path).write_text(text.replace(before, after, 1))
    args = ['cargo', 'test', '-p', 'sc-units', '--test', 'domain_context_contract'] if family == 'units' else ['cargo', 'test', '-p', 'sc-core', '--lib', 'domain_context_contracts']
    result = subprocess.run(args, cwd=ROOT, capture_output=True)
    output = result.stdout + result.stderr
    (WORK / ('mutation-%d.log' % number)).write_bytes(output)
    assert result.returncode == 101 and b'test result: FAILED.' in output and b'assertion' in output and b'could not compile' not in output, (name, output.decode())
    print('domain context mutation %d %s: rc=101, actual contract assertion failed' % (number, name))
    (ROOT / path).write_bytes(ORIGINAL[path])
finally:
 for path, data in ORIGINAL.items(): (ROOT / path).write_bytes(data)
assert all((ROOT / path).read_bytes() == data for path, data in ORIGINAL.items())
print('Domain context mutations: fourteen actual reds; all production sources restored byte-identically')
