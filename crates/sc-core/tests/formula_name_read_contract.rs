//! Exact metadata reads and absent-name diagnostics through the public namespace boundary.
#![allow(clippy::unwrap_used, clippy::expect_used)]
use sc_core::{
    name::MachineToken,
    ontology::{EdgeRef, EntityId, LocalTag, PointRef},
    recipe::{
        FormulaBindingKind as B, FormulaDeclaration as D, FormulaDeclarationSource as S,
        FormulaInitialDeclaration as I, FormulaInputOrigin as L, FormulaKind as K,
        FormulaNamespace as N, FormulaOrigin as O, FormulaReservedContext as C,
        FormulaReservedName as R, FormulaScalarInputOrigin as P, FormulaToleranceName as T,
        FormulaUnboundName,
    },
    value::{LengthDeclaration, LengthDeclarationDefinition, LengthState},
};
use sc_units::Length;

fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn initial(d: D<'_>) -> I<'_> {
    I::try_from(d).unwrap()
}
fn resolved<'a>(ns: &N<'a>, name: &MachineToken) -> D<'a> {
    let result = ns.resolve(name);
    assert!(result.is_ok());
    result.unwrap()
}
fn missing<'n>(ns: &N<'_>, name: &'n MachineToken) -> FormulaUnboundName<'n> {
    let result = ns.resolve(name);
    assert!(result.is_err());
    result.unwrap_err()
}

#[test]
fn reserved_reads_keep_closed_metadata_without_value_providers() {
    let ns = N::new([]).unwrap();
    for (name, source, kind, origin, context) in [
        (
            "eps_num",
            R::Tolerance(T::Numerical),
            K::Length,
            O::Tolerance,
            C::Always,
        ),
        (
            "eps_geo",
            R::Tolerance(T::Geometric),
            K::Length,
            O::Tolerance,
            C::Always,
        ),
        (
            "eps_fmt",
            R::Tolerance(T::Format),
            K::Length,
            O::Tolerance,
            C::Export,
        ),
        (
            "eps_imp",
            R::Tolerance(T::Importer),
            K::Length,
            O::Tolerance,
            C::Export,
        ),
        (
            "eps_phys",
            R::Tolerance(T::Physical),
            K::Length,
            O::Tolerance,
            C::Profile,
        ),
        ("size_index", R::SizeIndex, K::Count, O::Size, C::Size),
        ("size_count", R::SizeCount, K::Count, O::Size, C::Size),
        ("is_base_size", R::IsBaseSize, K::Boolean, O::Size, C::Size),
    ] {
        let query = MachineToken::new(name).unwrap();
        let d = resolved(&ns, &query);
        assert_eq!(d.name(), name);
        assert_eq!(d.kind(), kind);
        assert_eq!(d.origin(), origin);
        assert!(matches!(d.source(), S::Reserved(actual) if actual == source
            && actual.required_context() == context));
    }
    assert_eq!(ns.declarations().len(), 8);
}

#[test]
fn scalar_reads_preserve_every_annotation_and_origin_and_exact_identity() {
    let original = MachineToken::new("width").unwrap();
    let other = MachineToken::new("other_width").unwrap();
    for (source_origin, origin) in [
        (P::Parameter, O::Parameter),
        (P::Profile, O::Profile),
        (P::Material, O::Material),
    ] {
        for (binding, kind) in [
            (B::Length, K::Length),
            (B::Angle, K::Angle),
            (B::Area, K::Area),
            (B::Ratio, K::Ratio),
            (B::Count, K::Count),
            (B::Boolean, K::Boolean),
        ] {
            let ns = N::new([
                initial(D::input(&other, source_origin, id(31), id(32), binding)),
                initial(D::input(&original, source_origin, id(11), id(12), binding)),
            ])
            .unwrap();
            // A different validated token of the same spelling is a lookup, not a new binding.
            let query = MachineToken::new("width").unwrap();
            let d = resolved(&ns, &query);
            assert!(std::ptr::eq(d.name(), original.as_str()));
            assert_eq!(d.kind(), kind);
            assert_eq!(d.origin(), origin);
            assert!(
                matches!(d.source(), S::Input { origin: actual, input, declaration, kind: annotation }
                if actual == source_origin && input == id(11) && declaration == id(12) && annotation == binding)
            );
            let other_d = resolved(&ns, &other);
            assert!(
                matches!(other_d.source(), S::Input { input, declaration, .. }
                if input == id(31) && declaration == id(32))
            );
        }
    }
}

#[test]
fn canonical_length_reads_keep_the_same_record_for_all_states_and_origins() {
    let name = MachineToken::new("body_width").unwrap();
    for state in [
        LengthState::Known {
            value: Length::from_micrometres(331).unwrap(),
            evidence: vec![id(1)],
        },
        LengthState::Assumed {
            value: Length::from_micrometres(-719).unwrap(),
            assumption: id(2),
        },
        LengthState::Unknown { observation: id(3) },
        LengthState::Preference {
            value: Length::from_micrometres(557).unwrap(),
            provenance: id(4),
        },
        LengthState::Derived { formula: id(5) },
    ] {
        let record = LengthDeclaration::new(LengthDeclarationDefinition {
            id: id(101),
            source: id(102),
            state,
        })
        .unwrap();
        for (source_origin, origin) in [
            (L::Measurement, O::Measurement),
            (L::Ease, O::Ease),
            (L::Parameter, O::Parameter),
            (L::Profile, O::Profile),
            (L::Material, O::Material),
        ] {
            let ns = N::new([initial(D::length_input(
                &name,
                source_origin,
                id(103),
                &record,
            ))])
            .unwrap();
            let d = resolved(&ns, &name);
            assert_eq!(d.kind(), K::Length);
            assert_eq!(d.origin(), origin);
            assert!(
                matches!(d.source(), S::LengthInput { origin: actual, input, declaration }
                if actual == source_origin && input == id(103) && std::ptr::eq(declaration, &record))
            );
            let copy = ns.clone();
            assert!(
                matches!(resolved(&copy, &name).source(), S::LengthInput { declaration, .. }
                if std::ptr::eq(declaration, &record))
            );
        }
    }
}

#[test]
fn geometry_reads_retain_exact_existing_refs() {
    let point_name = MachineToken::new("corner").unwrap();
    let edge_name = MachineToken::new("waist_edge").unwrap();
    let point = PointRef::new(id(211), LocalTag::new(47));
    let edge = EdgeRef::new(id(221), LocalTag::new(53));
    let ns = N::new([
        initial(D::point(&point_name, point)),
        initial(D::edge(&edge_name, edge)),
    ])
    .unwrap();
    let d = resolved(&ns, &point_name);
    assert_eq!(d.kind(), K::Point);
    assert_eq!(d.origin(), O::Geometry);
    assert!(matches!(d.source(), S::Point(actual) if actual == point));
    let d = resolved(&ns, &edge_name);
    assert_eq!(d.kind(), K::Edge);
    assert_eq!(d.origin(), O::Geometry);
    assert!(matches!(d.source(), S::Edge(actual) if actual == edge));
}

#[test]
fn absent_queries_have_exact_name_and_every_searched_origin_without_aliasing() {
    let name = MachineToken::new("width").unwrap();
    let ns = N::new([initial(D::input(
        &name,
        P::Parameter,
        id(41),
        id(42),
        B::Length,
    ))])
    .unwrap();
    let before = ns.declarations().map(|d| d.name()).collect::<Vec<_>>();
    for name in [
        "missing_width",
        "width2",
        "width_2",
        "widt",
        "widthx",
        "width_width",
        "eps_num2",
        "eps_geo2",
        "eps_fmt2",
        "eps_imp2",
        "eps_phys2",
        "size_index2",
        "size_count2",
        "is_base_size2",
        "loop",
        "repeat",
        "while",
        "fn",
        "macro",
    ] {
        let query = MachineToken::new(name).unwrap();
        let error = missing(&ns, &query);
        assert!(std::ptr::eq(error.name(), query.as_str()));
        assert_eq!(error.name(), name);
        assert_eq!(error.token(), "formula_unbound_name");
        assert_eq!(
            error.origins_searched(),
            &[
                O::Measurement,
                O::Ease,
                O::Parameter,
                O::Profile,
                O::Material,
                O::Geometry,
                O::Recipe,
                O::Size,
                O::Tolerance
            ]
        );
        assert_eq!(
            ns.declarations().map(|d| d.name()).collect::<Vec<_>>(),
            before
        );
    }
}

#[test]
fn returned_declaration_and_error_retain_their_independent_source_lifetimes() {
    let original = MachineToken::new("original").unwrap();
    let d = {
        let ns = N::new([initial(D::input(
            &original,
            P::Material,
            id(51),
            id(52),
            B::Ratio,
        ))])
        .unwrap();
        let query = MachineToken::new("original").unwrap();
        resolved(&ns, &query)
    };
    assert!(std::ptr::eq(d.name(), original.as_str()));
    assert!(
        matches!(d.source(), S::Input { input, declaration, kind, .. }
        if input == id(51) && declaration == id(52) && kind == B::Ratio)
    );
    let query = MachineToken::new("private_missing_customer_name").unwrap();
    let error = {
        let ns = N::new([]).unwrap();
        missing(&ns, &query)
    };
    assert!(std::ptr::eq(error.name(), query.as_str()));
    let cloned = error;
    assert!(std::ptr::eq(cloned.name(), query.as_str()));
}

#[test]
fn default_error_formatting_omits_authored_query_and_exposes_only_the_token() {
    let query = MachineToken::new("private_missing_customer_name").unwrap();
    let error = missing(&N::new([]).unwrap(), &query);
    for debug in [format!("{error:?}"), format!("{error:#?}")] {
        assert!(debug.contains("FormulaUnboundName"));
        assert!(!debug.contains(query.as_str()));
    }
    assert_eq!(error.to_string(), "formula_unbound_name");
    assert!(std::error::Error::source(&error).is_none());
}
