//! Native inverse value boundary; the oracle uses addition reduction without native roots.
use sc_core::recipe::{
    FormulaBuiltin as F, FormulaKind as K, FormulaNumericError as Error, FormulaNumericValue as V,
    FormulaRational as R,
};

#[allow(clippy::expect_used)]
fn rational(numerator: &str, denominator: &str) -> R {
    let (negative, magnitude) = numerator
        .strip_prefix('-')
        .map_or((false, numerator), |n| (true, n));
    R::new(
        magnitude.parse().expect("magnitude"),
        denominator.parse().expect("denominator"),
        negative,
    )
    .expect("admitted exact fraction")
}

#[test]
#[allow(clippy::expect_used)]
fn native_inverse_matches_independent_midpoint_and_full_width_cases() {
    let mut atan_rows = 0;
    let mut atan2_rows = 0;
    let mut negative_endpoint = false;
    for row in include_str!("fixtures/formula_inverse.tsv")
        .lines()
        .filter(|r| !r.starts_with('#'))
    {
        let [call, kind, yn, yd, xn, xd, expected]: [&str; 7] = row
            .split('\t')
            .collect::<Vec<_>>()
            .try_into()
            .expect("seven columns");
        let kind = match kind {
            "Length" => Some(K::Length),
            "Ratio" => Some(K::Ratio),
            _ => None,
        }
        .expect("fixture numeric kind");
        let y = V::new(kind, rational(yn, yd)).expect("completed y");
        let actual = if call == "atan" {
            atan_rows += 1;
            y.checked_atan()
        } else {
            assert_eq!(call, "atan2", "fixture call");
            atan2_rows += 1;
            y.checked_atan2(V::new(kind, rational(xn, xd)).expect("completed x"))
        }
        .expect("certified inverse answer");
        assert_eq!(actual.kind(), K::Angle);
        let expected: i128 = expected.parse().expect("oracle microdegrees");
        assert_eq!(actual.rational(), R::from_signed_integer(expected), "{row}");
        negative_endpoint |= expected == -180_000_000;
    }
    assert_eq!((atan_rows, atan2_rows), (27, 54));
    assert!(
        negative_endpoint,
        "signed rounded negative branch endpoint is exercised"
    );
}

#[test]
#[allow(clippy::expect_used)]
fn independent_oracle_reproduces_the_retained_fixture() {
    let root = std::env::current_dir()
        .expect("Cargo package root")
        .join("../..");
    let output = std::process::Command::new("python3")
        .args([
            "-I",
            "-B",
            "docs/tasks/artifacts/formula_structure/inverse_value_oracle.py",
        ])
        .current_dir(root)
        .output()
        .expect("independent addition oracle");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        output.stdout,
        include_bytes!("fixtures/formula_inverse.tsv")
    );
}

#[test]
#[allow(clippy::expect_used)]
fn kind_priority_precedes_zero_vector_and_preserves_actual_operands() {
    let kinds = [K::Length, K::Area, K::Angle, K::Ratio, K::Count];
    for ykind in kinds {
        let y = V::new(ykind, R::ZERO).expect("actual zero");
        if ykind != K::Ratio {
            assert_eq!(
                y.checked_atan(),
                Err(Error::BuiltinInputKind {
                    builtin: F::Atan,
                    argument: 0,
                    actual: ykind,
                })
            );
        }
        for xkind in kinds {
            let x = V::new(xkind, R::ZERO).expect("actual zero");
            let error = y.checked_atan2(x).expect_err("zero or incompatible inputs");
            let expected = if !matches!(ykind, K::Length | K::Ratio) {
                Error::BuiltinInputKind {
                    builtin: F::Atan2,
                    argument: 0,
                    actual: ykind,
                }
            } else if xkind != ykind {
                Error::BuiltinDimension {
                    builtin: F::Atan2,
                    argument: 1,
                    wanted: ykind,
                    actual: xkind,
                }
            } else {
                Error::Atan2ZeroVector {
                    kind: ykind,
                    y: R::ZERO,
                    x: R::ZERO,
                }
            };
            assert_eq!(error, expected);
            assert_eq!(
                error.diagnostic_code(),
                if ykind == xkind && matches!(ykind, K::Length | K::Ratio) {
                    "formula_domain"
                } else {
                    "formula_dimension"
                }
            );
            assert!(error.to_string().contains(error.diagnostic_code()));
        }
    }
}
