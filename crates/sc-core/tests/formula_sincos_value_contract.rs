//! Native and actual sourced sin/cos boundaries against independent complete original fixtures.
use sc_core::recipe::{
    FormulaBuiltin as F, FormulaKind as K, FormulaNumericError as Error, FormulaNumericValue as V,
    FormulaRational as R,
};

#[test]
#[allow(clippy::expect_used)]
fn compiled_values_match_independent_full_turn_and_midpoint_neighbors() {
    let mut rows = 0;
    for row in include_str!("fixtures/formula_sincos.tsv")
        .lines()
        .filter(|row| !row.starts_with('#'))
    {
        let cells: [&str; 4] = row
            .split('\t')
            .collect::<Vec<_>>()
            .try_into()
            .expect("four fixture columns");
        let [numerator, denominator, sine, cosine] = cells;
        let (negative, magnitude) = numerator
            .strip_prefix('-')
            .map_or((false, numerator), |n| (true, n));
        let angle = V::new(
            K::Angle,
            R::new(
                magnitude.parse().expect("angle magnitude"),
                denominator.parse().expect("angle denominator"),
                negative,
            )
            .expect("admitted full-width angle"),
        )
        .expect("completed angle");
        for (actual, expected) in [(angle.checked_sin(), sine), (angle.checked_cos(), cosine)] {
            let result = actual.expect("certified rounded ratio");
            assert_eq!(result.kind(), K::Ratio);
            assert_eq!(
                result.rational(),
                R::from_signed_integer(expected.parse().expect("oracle ppm")),
                "{row}"
            );
        }
        rows += 1;
    }
    assert_eq!(rows, 44, "every retained authored case must run");
}

#[test]
#[allow(clippy::expect_used)]
fn retained_oracle_is_reproduced_without_native_folding_or_pi_input() {
    let root = std::env::current_dir()
        .expect("Cargo package working directory")
        .join("../..");
    let result = std::process::Command::new("python3")
        .args([
            "-I",
            "-B",
            "docs/tasks/artifacts/formula_structure/sincos_value_oracle.py",
        ])
        .current_dir(root)
        .output()
        .expect("independent direct-power/factorial producer");
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert_eq!(result.stdout, include_bytes!("fixtures/formula_sincos.tsv"));
}

#[test]
#[allow(clippy::expect_used)]
fn kind_refusals_preserve_actual_zero_operand_and_call() {
    for kind in [K::Length, K::Area, K::Ratio, K::Count] {
        let value = V::new(kind, R::ZERO).expect("actual numeric zero");
        for (result, builtin) in [(value.checked_sin(), F::Sin), (value.checked_cos(), F::Cos)] {
            let error = result.expect_err("only angle is admitted");
            assert_eq!(
                error,
                Error::BuiltinInputKind {
                    builtin,
                    argument: 0,
                    actual: kind
                }
            );
            assert_eq!(error.diagnostic_code(), "formula_dimension");
        }
    }
}

#[test]
#[allow(clippy::expect_used)]
fn actual_sourced_outputs_keep_upstream_calls_full_inputs_storage_and_t1_refusal() {
    use sc_core::{
        name::MachineToken,
        recipe::{FormulaBinaryOperator as B, FormulaEvaluatedScalar as S, FormulaToleranceName},
    };
    let ratio = |integer| {
        S::from_numeric(
            V::new(K::Ratio, R::from_signed_integer(integer)).expect("actual Ratio operand"),
        )
    };
    let root = ratio(1_000_000)
        .checked_sqrt()
        .expect("actual perfect Ratio root");
    let inverse = ratio(0).checked_atan().expect("actual zero inverse Angle");
    let upstream_zero = root
        .checked_binary(B::Multiply, inverse)
        .expect("real zero Angle with two actual call contributions");
    let name = MachineToken::new("actual_forward_binding").expect("binding name");
    let mut outputs = 0;
    for row in include_str!("fixtures/formula_sincos.tsv")
        .lines()
        .filter(|r| !r.starts_with('#'))
    {
        let fields: Vec<_> = row.split('\t').collect();
        let [numerator, denominator, sine, cosine] = fields.as_slice() else {
            unreachable!("four original independent forward columns")
        };
        let (negative, magnitude) = numerator
            .strip_prefix('-')
            .map_or((false, *numerator), |n| (true, n));
        let original = V::new(
            K::Angle,
            R::new(
                magnitude.parse().expect("original magnitude"),
                denominator.parse().expect("original denominator"),
                negative,
            )
            .expect("original full-width rational"),
        )
        .expect("actual Angle");
        let angle = S::from_numeric(original)
            .checked_binary(B::Add, upstream_zero)
            .expect("unbound full-width Angle with original value preserved");
        assert_eq!(
            angle.numeric(),
            Some(original),
            "no input storage/narrowing:{row}"
        );
        assert_eq!(angle.sources().iter().count(), 2);
        for (actual, expected, call) in [
            (angle.checked_sin(), sine, F::Sin),
            (angle.checked_cos(), cosine, F::Cos),
        ] {
            let result = actual.expect("complete certified forward result");
            assert_eq!(result.kind(), K::Ratio);
            assert_eq!(
                result.numeric().expect("Ratio").rational(),
                R::from_signed_integer(expected.parse().expect("independent ppm")),
                "{row}:{call:?}"
            );
            for source in [F::Sqrt, F::Atan, call] {
                assert!(result.sources().contains(source), "{row}:{source:?}");
            }
            assert_eq!(result.sources().iter().count(), 3);
            assert!(result
                .sources()
                .check_tolerance(FormulaToleranceName::Numerical)
                .is_err());
            let binding = result.bind(&name).expect("actual signed ppm storage");
            assert_eq!(binding.read().expect("actual stored read"), result, "{row}");
            outputs += 1;
        }
    }
    assert_eq!(
        outputs, 88,
        "every original forward outcome through actual sourced storage"
    );
}
