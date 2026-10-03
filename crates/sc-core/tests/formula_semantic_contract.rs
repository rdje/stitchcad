//! Independently authored semantic populations and canonical table correspondence.
#![allow(clippy::unwrap_used, clippy::expect_used)]
use sc_core::name::MachineToken;
use sc_core::recipe::{
    FormulaBindingKind as B, FormulaKind as K, FormulaOrigin as O, FormulaReservedContext as C,
    FormulaReservedName as R, FormulaToleranceName as T,
};

const KINDS: [(K, &str, Option<B>); 8] = [
    (K::Length, "length", Some(B::Length)),
    (K::Angle, "angle", Some(B::Angle)),
    (K::Area, "area", Some(B::Area)),
    (K::Ratio, "ratio", Some(B::Ratio)),
    (K::Count, "count", Some(B::Count)),
    (K::Boolean, "boolean", Some(B::Boolean)),
    (K::Point, "point", None),
    (K::Edge, "edge", None),
];
const ORIGINS: [(O, &str); 9] = [
    (O::Measurement, "measurement"),
    (O::Ease, "ease"),
    (O::Parameter, "parameter"),
    (O::Profile, "profile"),
    (O::Material, "material"),
    (O::Geometry, "geometry"),
    (O::Recipe, "recipe"),
    (O::Size, "size"),
    (O::Tolerance, "tolerance"),
];
#[derive(Clone, Copy)]
struct ReservedCase {
    name: R,
    token: &'static str,
    kind: K,
    origin: O,
    context: C,
    tolerance: Option<T>,
}
const RESERVED: [ReservedCase; 8] = [
    ReservedCase {
        name: R::Tolerance(T::Numerical),
        token: "eps_num",
        kind: K::Length,
        origin: O::Tolerance,
        context: C::Always,
        tolerance: Some(T::Numerical),
    },
    ReservedCase {
        name: R::Tolerance(T::Geometric),
        token: "eps_geo",
        kind: K::Length,
        origin: O::Tolerance,
        context: C::Always,
        tolerance: Some(T::Geometric),
    },
    ReservedCase {
        name: R::Tolerance(T::Format),
        token: "eps_fmt",
        kind: K::Length,
        origin: O::Tolerance,
        context: C::Export,
        tolerance: Some(T::Format),
    },
    ReservedCase {
        name: R::Tolerance(T::Importer),
        token: "eps_imp",
        kind: K::Length,
        origin: O::Tolerance,
        context: C::Export,
        tolerance: Some(T::Importer),
    },
    ReservedCase {
        name: R::Tolerance(T::Physical),
        token: "eps_phys",
        kind: K::Length,
        origin: O::Tolerance,
        context: C::Profile,
        tolerance: Some(T::Physical),
    },
    ReservedCase {
        name: R::SizeIndex,
        token: "size_index",
        kind: K::Count,
        origin: O::Size,
        context: C::Size,
        tolerance: None,
    },
    ReservedCase {
        name: R::SizeCount,
        token: "size_count",
        kind: K::Count,
        origin: O::Size,
        context: C::Size,
        tolerance: None,
    },
    ReservedCase {
        name: R::IsBaseSize,
        token: "is_base_size",
        kind: K::Boolean,
        origin: O::Size,
        context: C::Size,
        tolerance: None,
    },
];

#[test]
fn all_operand_kinds_and_binding_conversions_are_closed() {
    assert_eq!(K::ALL, KINDS.map(|row| row.0));
    for (kind, token, binding) in KINDS {
        assert_eq!(kind.token(), token);
        assert_eq!(K::from_token(token), Some(kind));
        assert_eq!(kind.binding_kind(), binding);
        if let Some(binding) = binding {
            assert_eq!(K::from(binding), kind);
            assert_eq!(binding.token(), token);
        }
    }
}

#[test]
fn all_source_origins_have_distinct_exact_tokens() {
    assert_eq!(O::ALL, ORIGINS.map(|row| row.0));
    for (origin, token) in ORIGINS {
        assert_eq!(origin.token(), token);
        assert_eq!(O::from_token(token), Some(origin));
    }
}

#[test]
fn reserved_metadata_preserves_kinds_origins_contexts_and_tolerance_roles() {
    assert_eq!(R::ALL, RESERVED.map(|row| row.name));
    for ReservedCase {
        name,
        token,
        kind,
        origin,
        context,
        tolerance,
    } in RESERVED
    {
        assert_eq!(name.token(), token);
        assert_eq!(R::from_token(token), Some(name));
        assert_eq!(name.kind(), kind);
        assert_eq!(name.origin(), origin);
        assert_eq!(name.required_context(), context);
        assert_eq!(name.tolerance_name(), tolerance);
        assert!(MachineToken::new(token).unwrap().is_reserved_input_name());
    }
}

#[test]
fn lookups_refuse_repairs_aliases_and_new_excluded_form_reservations() {
    for token in [
        "", "Length", " length", "length ", "length\0", "scalar", "text", "lеngth",
    ] {
        assert_eq!(K::from_token(token), None, "{token:?}");
    }
    for token in [
        "",
        "Measurement",
        " measurement",
        "measurement ",
        "measurement\0",
        "input",
        "unknown",
    ] {
        assert_eq!(O::from_token(token), None, "{token:?}");
    }
    for token in [
        "",
        "EPS_NUM",
        " eps_num",
        "eps_num ",
        "eps_num\0",
        "eps_chord",
        "eps_num_2",
    ] {
        assert_eq!(R::from_token(token), None, "{token:?}");
    }
    for token in [
        "loop",
        "repeat",
        "while",
        "fn",
        "macro",
        "width",
        "length",
        "eps_geo_input",
    ] {
        assert_eq!(R::from_token(token), None);
        assert!(!MachineToken::new(token).unwrap().is_reserved_input_name());
    }
}

fn table(header: &str) -> Vec<Vec<&'static str>> {
    let contract = include_str!("../../../docs/book/src/spec/formula-language.md");
    let rows: Vec<_> = contract
        .lines()
        .skip_while(|line| *line != header)
        .skip(2)
        .take_while(|line| line.starts_with('|'))
        .map(|line| {
            line.trim_matches('|')
                .split('|')
                .map(str::trim)
                .collect::<Vec<_>>()
        })
        .collect();
    assert!(
        !rows.is_empty(),
        "canonical table header must resolve: {header}"
    );
    rows
}
fn token(cell: &str) -> &str {
    cell.trim_matches('`')
}

#[test]
fn independently_authored_populations_match_all_canonical_rows_both_directions() {
    let kinds = table("| Kind | Bound form | Holds | Bindable by `let` |");
    assert_eq!(kinds.len(), KINDS.len());
    for (row, (_, expected, binding)) in kinds.iter().zip(KINDS) {
        assert_eq!(row.len(), 4);
        assert_eq!(token(row.first().unwrap()), expected);
        assert_eq!(
            row.last().copied(),
            Some(if binding.is_some() { "yes" } else { "no" })
        );
    }
    let origins = table("| Origin | Supplied by | If there is no value |");
    assert_eq!(origins.len(), ORIGINS.len());
    for (row, (_, expected)) in origins.iter().zip(ORIGINS) {
        assert_eq!(row.len(), 3);
        assert_eq!(token(row.first().unwrap()), expected);
    }
    let names = table("| Name | Kind | Value | Bound where |");
    assert_eq!(names.len(), RESERVED.len());
    for (
        row,
        ReservedCase {
            token: expected,
            kind,
            context,
            ..
        },
    ) in names.iter().zip(RESERVED)
    {
        assert_eq!(row.len(), 4);
        assert_eq!(token(row.first().unwrap()), expected);
        assert_eq!(row.get(1).copied(), Some(kind.token()));
        let expected_context = match context {
            C::Always => "always",
            C::Export => "an export context only",
            C::Profile => "a profile that supplies one",
            C::Size => "a size context",
        };
        assert_eq!(row.last().copied(), Some(expected_context));
    }
}
