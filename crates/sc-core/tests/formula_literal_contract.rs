//! Independent input-normalization contracts; no recipe/binding/evaluation certificate.
use sc_core::recipe::{
    FormulaExpression as E, FormulaLiteralError, FormulaLiteralKind as K, FormulaLiteralRule as R,
    FormulaNodeKind, FormulaRationalComponent as C, FormulaUnit as U,
};

#[allow(clippy::expect_used)] // A fixture syntax refusal or unwind is a failed public contract.
fn value(source: &str) -> Result<(K, u128), FormulaLiteralError> {
    let tree = E::parse(source).expect("valid fixture syntax");
    std::panic::catch_unwind(|| tree.root().normalized_literal())
        .expect("assertion: numeric normalization must not unwind")
        .map(|literal| {
            let literal = literal.expect("fixture root is a literal");
            (literal.kind(), literal.magnitude())
        })
}

#[test]
#[allow(clippy::expect_used)]
fn independent_fraction_fixtures_cover_conversion_width_and_domain() {
    let rows = include_str!(
        "../../../docs/tasks/artifacts/formula_structure/literal_normalization_cases.tsv"
    );
    let mut count = 0;
    for row in rows
        .lines()
        .filter(|row| !row.starts_with('#') && !row.is_empty())
    {
        let cells: Vec<_> = row.split('\t').collect();
        let source = cells.first().expect("source");
        let kind = *cells.get(1).expect("kind");
        let want = *cells.get(2).expect("verdict");
        let actual = value(source);
        match want {
            "width" => {
                let error = actual.expect_err("assertion: exact pre-round width must refuse");
                assert!(
                    matches!(
                        error.rule(),
                        R::RationalWidth {
                            measured_bits_at_least: 129,
                            ..
                        }
                    ),
                    "{source}: {error:?}"
                );
                assert_eq!(error.rational_bit_bound(), Some(128));
                assert_eq!(error.limit_token(), Some("max_rational_bits"));
                assert_eq!(error.diagnostic_code(), "formula_domain");
            }
            "length" => assert!(
                matches!(
                    actual
                        .expect_err("assertion: scalar length must refuse")
                        .rule(),
                    R::LengthDomain {
                        maximum: 1000000000,
                        ..
                    }
                ),
                "{source}"
            ),
            magnitude => {
                assert!(
                    actual.is_ok(),
                    "assertion: valid exact input {source}: {actual:?}"
                );
                let (got_kind, got_value) = actual.expect("valid exact input");
                assert_eq!(got_kind.token(), kind, "{source}");
                assert_eq!(
                    got_value,
                    magnitude.parse::<u128>().expect("fixture integer"),
                    "{source}"
                );
            }
        }
        count += 1;
    }
    assert_eq!(count, 100);
}

#[test]
fn kinds_quanta_and_angle_turns_are_preserved() {
    for (source, kind, magnitude) in [
        ("4", K::Count, 4),
        ("4.0", K::Ratio, 4000000),
        ("150 pct", K::Ratio, 1500000),
        ("2.5 cm", K::Length, 25000),
        ("25 mm", K::Length, 25000),
        ("0.00004 cm", K::Length, 0),
        ("0.00005 cm", K::Length, 1),
        ("360 deg", K::Angle, 360000000),
        ("720 deg", K::Angle, 720000000),
        ("9223372036854.775808 deg", K::Angle, 1_u128 << 63),
    ] {
        assert_eq!(value(source), Ok((kind, magnitude)), "{source}");
    }
    assert_eq!(value(&u128::MAX.to_string()), Ok((K::Count, u128::MAX)));
}

#[test]
#[allow(clippy::expect_used)]
fn width_checks_precede_rounding_and_length_checks_follow_it() {
    let error = value("0.0000000000000000000000000000000000000001 um")
        .expect_err("assertion: rounding to zero cannot rescue denominator width");
    assert_eq!(
        error.rule(),
        R::RationalWidth {
            component: C::Denominator,
            measured_bits_at_least: 129
        }
    );
    assert_eq!(value("1000000000.4 um"), Ok((K::Length, 1000000000)));
    let error = value("1000000000.5 um").expect_err("assertion: rounded length must refuse");
    assert_eq!(
        error.rule(),
        R::LengthDomain {
            maximum: 1000000000,
            measured: 1000000001
        }
    );
    assert_eq!((error.span().start(), error.span().end()), (0, 15));
    assert!(error.to_string().contains("formula_domain"));
}

#[test]
#[allow(clippy::expect_used)]
fn source_location_privacy_and_unary_identity_survive_conversion() {
    let source = String::from("  -(0009223372036854.775808 deg)  ");
    let syntax = E::parse(&source).expect("valid unary expression");
    assert!(syntax
        .root()
        .normalized_literal()
        .expect("nonliteral")
        .is_none());
    let child = match syntax.root().kind() {
        FormulaNodeKind::Negate(child) => Some(child),
        _ => None,
    }
    .expect("assertion: unary node retained");
    let literal = child
        .normalized_literal()
        .expect("conversion")
        .expect("literal child");
    assert_eq!(literal.kind(), K::Angle);
    assert_eq!(literal.magnitude(), 1_u128 << 63);
    assert_eq!(literal.unit(), Some(U::Degree));
    assert_eq!(literal.number(), "0009223372036854.775808");
    assert_eq!(
        literal.number().as_ptr(),
        source.as_bytes().get(4..).expect("source slice").as_ptr()
    );
    assert_eq!((literal.span().start(), literal.span().end()), (3, 32));
    let debug = format!("{literal:?}");
    assert!(!debug.contains(literal.number()));
    assert!(!debug.contains(&literal.magnitude().to_string()));
    let name = E::parse("customer_name").expect("name");
    assert!(name
        .root()
        .normalized_literal()
        .expect("nonliteral")
        .is_none());
}

#[test]
fn huge_spelling_refuses_or_reduces_without_new_language_caps() {
    let zero = format!("0.{} deg", "0".repeat(100000));
    assert_eq!(value(&zero), Ok((K::Angle, 0)));
    let padded = format!("{}1.0{} pct", "0".repeat(100000), "0".repeat(100000));
    assert_eq!(value(&padded), Ok((K::Ratio, 10000)));
    let tiny = format!("0.{}1 um", "0".repeat(100000));
    assert!(
        matches!(value(&tiny),Err(error) if matches!(error.rule(),R::RationalWidth{component:C::Denominator,..}))
    );
    let huge = format!("{} um", "9".repeat(100000));
    assert!(
        matches!(value(&huge),Err(error) if matches!(error.rule(),R::RationalWidth{component:C::Numerator,..}))
    );
}
