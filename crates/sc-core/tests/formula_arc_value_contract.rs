//! Focused arc-length value boundary, independent of source-bearing recipe evaluation.
use sc_core::recipe::{
    FormulaBuiltin as F, FormulaKind as K, FormulaNumericError as Error, FormulaNumericValue as V,
    FormulaRational as R, FormulaRationalComponent as C, FormulaRationalError,
};

#[allow(clippy::expect_used)]
fn value(kind: K, numerator: &str, denominator: &str) -> V {
    let (negative, magnitude) = numerator
        .strip_prefix('-')
        .map_or((false, numerator), |n| (true, n));
    V::new(
        kind,
        R::new(
            magnitude.parse().expect("fixture magnitude"),
            denominator.parse().expect("fixture denominator"),
            negative,
        )
        .expect("fixture rational"),
    )
    .expect("admitted completed operand")
}

#[test]
#[allow(clippy::expect_used)]
fn arcs_match_independent_signed_turn_and_hard_midpoint_fixtures() {
    let mut rows = 0;
    for row in include_str!("fixtures/formula_arcs.tsv")
        .lines()
        .filter(|row| !row.starts_with('#'))
    {
        let cells: [&str; 5] = row
            .split('\t')
            .collect::<Vec<_>>()
            .try_into()
            .expect("five arc fixture columns");
        let [angle_n, angle_d, radius_n, radius_d, expected] = cells;
        let angle = value(K::Angle, angle_n, angle_d);
        let radius = value(K::Length, radius_n, radius_d);
        let actual = angle.checked_arc_length(radius);
        if let Some(integer) = expected.strip_prefix("integer:") {
            let result = actual.expect("admitted rounded arc");
            assert_eq!(result.kind(), K::Length);
            assert_eq!(
                result.rational(),
                R::from_signed_integer(integer.parse().expect("expected integer")),
                "{row}"
            );
        } else if let Some(integer) = expected.strip_prefix("domain:") {
            let error = actual.expect_err("rounded scalar excess");
            assert_eq!(
                error,
                Error::ScalarDomain {
                    operation: "arc_length",
                    kind: K::Length,
                    value: R::from_signed_integer(integer.parse().expect("rounded witness")),
                    minimum: -1_000_000_000,
                    maximum: Some(1_000_000_000)
                },
                "{row}"
            );
            assert_eq!(error.diagnostic_code(), "formula_domain");
        } else {
            let bits = expected
                .strip_prefix("width:")
                .expect("width fixture")
                .parse()
                .expect("measured bits");
            let error = actual.expect_err("rounded reduced-width excess");
            assert_eq!(
                error,
                Error::Rational(FormulaRationalError::RationalWidth {
                    component: C::Numerator,
                    measured_bits: bits
                }),
                "{row}"
            );
            assert_eq!(error.diagnostic_code(), "formula_domain");
        }
        rows += 1;
    }
    assert_eq!(rows, 23, "all authored arc cases must run");
}

#[test]
#[allow(clippy::expect_used)]
fn arc_kind_refusals_preserve_actual_argument_even_for_zero() {
    let angle = value(K::Angle, "0", "1");
    let radius = value(K::Length, "0", "1");
    for kind in [K::Length, K::Area, K::Angle, K::Ratio, K::Count] {
        let other = value(kind, "0", "1");
        if kind != K::Angle {
            let error = other
                .checked_arc_length(radius)
                .expect_err("first argument requires angle");
            assert_eq!(
                error,
                Error::BuiltinInputKind {
                    builtin: F::ArcLength,
                    argument: 0,
                    actual: kind
                }
            );
            assert_eq!(error.diagnostic_code(), "formula_dimension");
        }
        if kind != K::Length {
            let error = angle
                .checked_arc_length(other)
                .expect_err("second argument requires length");
            assert_eq!(
                error,
                Error::BuiltinInputKind {
                    builtin: F::ArcLength,
                    argument: 1,
                    actual: kind
                }
            );
        }
    }
}

#[test]
#[allow(clippy::expect_used)]
fn actual_sourced_arcs_keep_signed_turns_original_refusals_and_real_upstream_storage_flags() {
    use sc_core::{
        name::MachineToken,
        recipe::{FormulaBinaryOperator as B, FormulaEvaluatedScalar as S, FormulaToleranceName},
    };
    let zero_angle = S::from_numeric(V::new(K::Angle, R::ZERO).expect("actual zero Angle"));
    let cosine = zero_angle.checked_cos().expect("actual upstream Cos");
    let sine = zero_angle.checked_sin().expect("actual upstream Sin");
    let angle_zero = S::from_numeric(V::new(K::Angle, R::ZERO).expect("Angle zero"));
    let radius_zero = S::from_numeric(V::new(K::Length, R::ZERO).expect("Length zero"));
    let name = MachineToken::new("actual_arc_binding").expect("actual binding name");
    let mut counts = (0, 0, 0);
    for row in include_str!("fixtures/formula_arcs.tsv")
        .lines()
        .filter(|r| !r.starts_with('#'))
    {
        let fields: Vec<_> = row.split('\t').collect();
        let [angle_n, angle_d, radius_n, radius_d, expected] = fields.as_slice() else {
            unreachable!("five original independent arc columns")
        };
        let angle = S::from_numeric(value(K::Angle, angle_n, angle_d))
            .checked_binary(
                B::Add,
                cosine
                    .checked_binary(B::Multiply, angle_zero)
                    .expect("real sourced Angle zero"),
            )
            .expect("original full-width signed Angle");
        let radius = S::from_numeric(value(K::Length, radius_n, radius_d))
            .checked_binary(
                B::Add,
                sine.checked_binary(B::Multiply, radius_zero)
                    .expect("real sourced radius zero"),
            )
            .expect("original signed Length radius");
        let actual = angle.checked_arc_length(radius);
        counts.0 += 1;
        if let Some(integer) = expected.strip_prefix("integer:") {
            let expected =
                R::from_signed_integer(integer.parse().expect("independent signed integer"));
            let result = actual.expect("independent admitted arc");
            assert_eq!(result.kind(), K::Length);
            assert_eq!(
                result.numeric().expect("Length").rational(),
                expected,
                "{row}"
            );
            for source in [F::Cos, F::Sin, F::ArcLength] {
                assert!(result.sources().contains(source));
            }
            assert_eq!(result.sources().iter().count(), 3);
            assert!(
                result
                    .sources()
                    .check_tolerance(FormulaToleranceName::Numerical)
                    .is_err(),
                "zero and exact outcomes retain all actual successful call contributions"
            );
            let binding = result.bind(&name).expect("real admitted signed storage");
            assert_eq!(binding.read().expect("actual stored read"), result, "{row}");
        } else {
            let error = actual.expect_err("original completed mathematical failure");
            let expected = if let Some(integer) = expected.strip_prefix("domain:") {
                counts.1 += 1;
                Error::ScalarDomain {
                    operation: "arc_length",
                    kind: K::Length,
                    value: R::from_signed_integer(
                        integer.parse().expect("independent rounded witness"),
                    ),
                    minimum: -1_000_000_000,
                    maximum: Some(1_000_000_000),
                }
            } else {
                counts.2 += 1;
                Error::Rational(FormulaRationalError::RationalWidth {
                    component: C::Numerator,
                    measured_bits: expected
                        .strip_prefix("width:")
                        .expect("exact width witness")
                        .parse()
                        .expect("bits"),
                })
            };
            assert_eq!(error, expected, "original mathematical cause:{row}");
        }
    }
    assert_eq!(
        counts,
        (23, 3, 2),
        "all original independent arc outcome families"
    );
    println!(
        "actual sourced arcs:{} rows/{} final domain/{} original width refusals",
        counts.0, counts.1, counts.2
    );
}

#[test]
#[allow(clippy::expect_used)]
fn independent_fraction_pi_reference_regenerates_every_original_arc_fixture_byte() {
    let root = std::env::current_dir()
        .expect("Cargo package root")
        .join("../..");
    let output = std::process::Command::new("python3")
        .args([
            "-I",
            "-B",
            "docs/tasks/artifacts/formula_structure/arc_value_oracle.py",
        ])
        .current_dir(root)
        .output()
        .expect("pure independent signed arc oracle");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        output.stdout,
        include_bytes!("fixtures/formula_arcs.tsv"),
        "every original signed/full-turn/input/rounded-domain/width byte"
    );
}
