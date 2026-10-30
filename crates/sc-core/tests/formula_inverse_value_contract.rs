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

#[test]
#[allow(clippy::expect_used)]
fn actual_sourced_inverse_rows_preserve_whole_inputs_signed_storage_and_original_refusals() {
    use sc_core::{
        name::MachineToken,
        recipe::{FormulaBinaryOperator as B, FormulaEvaluatedScalar as S, FormulaToleranceName},
    };
    let exact = |kind, value| S::from_numeric(V::new(kind, value).expect("actual numeric operand"));
    let root = exact(K::Ratio, R::from_signed_integer(1_000_000))
        .checked_sqrt()
        .expect("actual perfect Ratio root");
    let cosine = exact(K::Angle, R::ZERO)
        .checked_cos()
        .expect("actual upstream cosine");
    let zero = |kind, source: S| {
        source
            .checked_binary(B::Multiply, exact(kind, R::ZERO))
            .expect("real same-kind zero with actual contribution")
    };
    let name = MachineToken::new("actual_inverse_binding").expect("actual binding name");
    let mut counts = (0, 0);
    let mut negative_endpoint = false;
    for (index, row) in include_str!("fixtures/formula_inverse.tsv")
        .lines()
        .filter(|r| !r.starts_with('#'))
        .enumerate()
    {
        let row_number = index + 1;
        let [call, kind, yn, yd, xn, xd, expected]: [&str; 7] = row
            .split('\t')
            .collect::<Vec<_>>()
            .try_into()
            .expect("seven original independent inverse columns");
        let kind = match kind {
            "Length" => Some(K::Length),
            "Ratio" => Some(K::Ratio),
            _ => None,
        }
        .expect("original capitalized fixture kind");
        let original_y = V::new(kind, rational(yn, yd)).expect("original full-width y");
        let original_x = V::new(kind, rational(xn, xd)).expect("original full-width x");
        let y = S::from_numeric(original_y)
            .checked_binary(B::Add, zero(kind, root))
            .expect("original y plus actual sourced zero");
        let x = S::from_numeric(original_x)
            .checked_binary(B::Add, zero(kind, cosine))
            .expect("original x plus distinct sourced zero");
        assert_eq!(y.numeric(), Some(original_y));
        assert_eq!(x.numeric(), Some(original_x));
        let (result, source, source_count) = if call == "atan" {
            counts.0 += 1;
            (y.checked_atan(), F::Atan, 2)
        } else {
            assert_eq!(call, "atan2");
            counts.1 += 1;
            (y.checked_atan2(x), F::Atan2, 3)
        };
        let result = result.expect("complete certified inverse result");
        assert_eq!(result.kind(), K::Angle);
        let expected: i128 = expected.parse().expect("independent signed microdegrees");
        assert_eq!(
            result.numeric().expect("actual Angle").rational(),
            R::from_signed_integer(expected),
            "fixture-row={row_number};{row}"
        );
        assert!(result.sources().contains(F::Sqrt));
        assert!(result.sources().contains(source));
        if call == "atan2" {
            assert!(result.sources().contains(F::Cos));
        }
        assert_eq!(result.sources().iter().count(), source_count);
        assert!(result
            .sources()
            .check_tolerance(FormulaToleranceName::Numerical)
            .is_err());
        let binding = result.bind(&name).expect("actual signed Angle storage");
        assert_eq!(binding.read().expect("actual stored read"), result);
        negative_endpoint |= expected == -180_000_000;
    }
    assert_eq!(counts, (27, 54));
    assert!(
        negative_endpoint,
        "signed -180 survives actual storage without direction modulo"
    );
    for kind in [K::Length, K::Ratio] {
        let y = zero(kind, root);
        let x = zero(kind, cosine);
        assert_eq!(
            y.checked_atan2(x),
            Err(Error::Atan2ZeroVector {
                kind,
                y: R::ZERO,
                x: R::ZERO
            })
        );
        assert!(!y.sources().contains(F::Atan2));
        assert!(!x.sources().contains(F::Atan2));
    }
    let length = zero(K::Length, root);
    let ratio = zero(K::Ratio, cosine);
    assert_eq!(
        length.checked_atan(),
        Err(Error::BuiltinInputKind {
            builtin: F::Atan,
            argument: 0,
            actual: K::Length
        })
    );
    assert_eq!(
        length.checked_atan2(ratio),
        Err(Error::BuiltinDimension {
            builtin: F::Atan2,
            argument: 1,
            wanted: K::Length,
            actual: K::Ratio
        })
    );
    assert_eq!(
        exact(K::Angle, R::ZERO).checked_atan2(length),
        Err(Error::BuiltinInputKind {
            builtin: F::Atan2,
            argument: 0,
            actual: K::Angle
        })
    );
}
