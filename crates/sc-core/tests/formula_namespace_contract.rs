//! Independent public initial-namespace sources and first-collision contracts.
#![allow(clippy::unwrap_used, clippy::expect_used)]
use sc_core::{
    name::MachineToken,
    ontology::{EdgeRef, EntityId, LocalTag, PointRef},
    recipe::{
        FormulaBindingKind as B, FormulaDeclaration as D, FormulaDeclarationSource as S,
        FormulaInitialDeclaration as I, FormulaInputOrigin as L, FormulaKind as K,
        FormulaNamespace as N, FormulaNamespaceError as E, FormulaOrigin as O, FormulaRecipe,
        FormulaReservedContext as C, FormulaReservedName as R, FormulaScalarInputOrigin as P,
        FormulaToleranceName as T,
    },
    value::{LengthDeclaration, LengthDeclarationDefinition, LengthState},
};
use sc_units::Length;

fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn admitted(d: D<'_>) -> I<'_> {
    let result = I::try_from(d);
    assert!(result.is_ok());
    let initial = result.unwrap();
    assert_eq!(initial.declaration().name(), d.name());
    assert_eq!(initial.declaration().kind(), d.kind());
    assert_eq!(initial.declaration().origin(), d.origin());
    initial
}
fn record(state: LengthState) -> LengthDeclaration {
    LengthDeclaration::new(LengthDeclarationDefinition {
        id: id(201),
        source: id(202),
        state,
    })
    .unwrap()
}
fn scalar<'a>(name: &'a MachineToken, origin: P, source: u128) -> D<'a> {
    D::input(name, origin, id(source), id(source + 100), B::Length)
}
fn find<'a>(ns: &N<'a>, name: &str) -> D<'a> {
    let declaration = ns.declarations().find(|d| d.name() == name);
    assert!(declaration.is_some());
    declaration.unwrap()
}
fn refused<'a>(inputs: impl IntoIterator<Item = I<'a>>) -> E<'a> {
    let result = N::new(inputs);
    assert!(result.is_err());
    result.unwrap_err()
}
const RESERVED: [(R, &str, K, O, C); 8] = [
    (
        R::Tolerance(T::Numerical),
        "eps_num",
        K::Length,
        O::Tolerance,
        C::Always,
    ),
    (
        R::Tolerance(T::Geometric),
        "eps_geo",
        K::Length,
        O::Tolerance,
        C::Always,
    ),
    (
        R::Tolerance(T::Format),
        "eps_fmt",
        K::Length,
        O::Tolerance,
        C::Export,
    ),
    (
        R::Tolerance(T::Importer),
        "eps_imp",
        K::Length,
        O::Tolerance,
        C::Export,
    ),
    (
        R::Tolerance(T::Physical),
        "eps_phys",
        K::Length,
        O::Tolerance,
        C::Profile,
    ),
    (R::SizeIndex, "size_index", K::Count, O::Size, C::Size),
    (R::SizeCount, "size_count", K::Count, O::Size, C::Size),
    (R::IsBaseSize, "is_base_size", K::Boolean, O::Size, C::Size),
];

#[test]
fn empty_inputs_still_declare_every_reserved_kind_without_context() {
    let ns = N::new([]).unwrap();
    assert_eq!(ns.declarations().len(), 8);
    assert_eq!(
        ns.declarations().map(|d| d.name()).collect::<Vec<_>>(),
        [
            "eps_fmt",
            "eps_geo",
            "eps_imp",
            "eps_num",
            "eps_phys",
            "is_base_size",
            "size_count",
            "size_index"
        ]
    );
    for (reserved, name, kind, origin, context) in RESERVED {
        let d = find(&ns, name);
        assert_eq!(d.kind(), kind);
        assert_eq!(d.origin(), origin);
        assert!(matches!(d.source(), S::Reserved(actual) if actual == reserved));
        assert_eq!(reserved.required_context(), context);
    }
}

#[test]
fn recipe_and_reserved_sources_cannot_seed_initial_inputs() {
    for (reserved, name, kind, origin, _) in RESERVED {
        let result = I::try_from(D::reserved(reserved));
        assert!(result.is_err());
        let refused = result.unwrap_err();
        assert_eq!(refused.name(), name);
        assert_eq!(refused.kind(), kind);
        assert_eq!(refused.origin(), origin);
        assert!(matches!(refused.source(), S::Reserved(actual) if actual == reserved));
    }
    let recipe = FormulaRecipe::parse(
        "let prior:length=1 cm assert c:eps_num=prior==prior let future:angle=missing",
    )
    .unwrap()
    .normalize_literals()
    .unwrap();
    for (index, name, kind) in [(1, "prior", K::Length), (3, "future", K::Angle)] {
        let result = I::try_from(D::recipe(&recipe, index).unwrap());
        assert!(result.is_err());
        let refused = result.unwrap_err();
        assert_eq!(refused.name(), name);
        assert_eq!(refused.kind(), kind);
        assert!(
            matches!(refused.source(), S::Recipe { statement_index, .. } if statement_index == index)
        );
    }
}

#[test]
fn admitted_scalar_metadata_keeps_three_domains_six_kinds_and_distinct_ids() {
    let name = MachineToken::new("customer_input").unwrap();
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
            let original = D::input(&name, source_origin, id(11), id(12), binding);
            let initial = admitted(original);
            let ns = N::new([initial]).unwrap();
            assert_eq!(initial.declaration().name(), "customer_input");
            let d = find(&ns, "customer_input");
            assert_eq!(d.kind(), kind);
            assert_eq!(d.origin(), origin);
            assert!(
                matches!(d.source(), S::Input { origin: actual, input, declaration, kind: annotation }
                if actual == source_origin && input == id(11) && declaration == id(12) && annotation == binding)
            );
            assert_eq!(ns.declarations().len(), 9);
        }
    }
}

#[test]
fn length_inputs_preserve_actual_borrow_for_all_origins_and_states() {
    let name = MachineToken::new("body_width").unwrap();
    for state in [
        LengthState::Known {
            value: Length::from_micrometres(317).unwrap(),
            evidence: vec![id(1)],
        },
        LengthState::Assumed {
            value: Length::from_micrometres(-911).unwrap(),
            assumption: id(2),
        },
        LengthState::Unknown { observation: id(3) },
        LengthState::Preference {
            value: Length::from_micrometres(553).unwrap(),
            provenance: id(4),
        },
        LengthState::Derived { formula: id(5) },
    ] {
        let canonical = record(state);
        for (source_origin, origin) in [
            (L::Measurement, O::Measurement),
            (L::Ease, O::Ease),
            (L::Parameter, O::Parameter),
            (L::Profile, O::Profile),
            (L::Material, O::Material),
        ] {
            let ns = N::new([admitted(D::length_input(
                &name,
                source_origin,
                id(21),
                &canonical,
            ))])
            .unwrap();
            let d = find(&ns, "body_width");
            assert_eq!(d.kind(), K::Length);
            assert_eq!(d.origin(), origin);
            let S::LengthInput {
                origin: actual,
                input,
                declaration,
            } = d.source()
            else {
                assert!(matches!(d.source(), S::LengthInput { .. }));
                return;
            };
            assert_eq!(actual, source_origin);
            assert_eq!(input, id(21));
            assert!(std::ptr::eq(declaration, &canonical));
            let cloned = ns.clone();
            assert!(
                matches!(find(&cloned, "body_width").source(), S::LengthInput { declaration, .. }
                if std::ptr::eq(declaration, &canonical))
            );
        }
    }
}

#[test]
fn point_and_edge_sources_keep_the_exact_creator_and_tag() {
    let point_name = MachineToken::new("corner").unwrap();
    let edge_name = MachineToken::new("waist_edge").unwrap();
    let point = PointRef::new(id(41), LocalTag::new(73));
    let edge = EdgeRef::new(id(42), LocalTag::new(74));
    let ns = N::new([
        admitted(D::point(&point_name, point)),
        admitted(D::edge(&edge_name, edge)),
    ])
    .unwrap();
    assert!(matches!(find(&ns, "corner").source(), S::Point(actual) if actual == point));
    assert!(matches!(find(&ns, "waist_edge").source(), S::Edge(actual) if actual == edge));
    assert_eq!(find(&ns, "corner").kind(), K::Point);
    assert_eq!(find(&ns, "waist_edge").kind(), K::Edge);
}

fn input_source<'a>(
    name: &'a MachineToken,
    source: usize,
    canonical: &'a LengthDeclaration,
) -> D<'a> {
    match source {
        0 => D::length_input(name, L::Measurement, id(51), canonical),
        1 => D::length_input(name, L::Ease, id(52), canonical),
        2 => scalar(name, P::Parameter, 53),
        3 => scalar(name, P::Profile, 54),
        4 => scalar(name, P::Material, 55),
        _ => D::point(name, PointRef::new(id(56), LocalTag::new(91))),
    }
}
fn identity(d: D<'_>) -> (O, u128) {
    match d.source() {
        S::LengthInput { input, .. } | S::Input { input, .. } => (d.origin(), input.as_bits()),
        S::Point(point) => (O::Geometry, point.creator().as_bits()),
        S::Edge(edge) => (O::Geometry, edge.creator().as_bits()),
        _ => {
            assert!(matches!(
                d.source(),
                S::LengthInput { .. } | S::Input { .. } | S::Point(_)
            ));
            (d.origin(), 0)
        }
    }
}

#[test]
fn every_ordered_origin_collision_retains_both_sources_including_equal_origins() {
    let first_name = MachineToken::new("collision").unwrap();
    let second_name = first_name.clone();
    let canonical = record(LengthState::Unknown { observation: id(3) });
    for first in 0..6 {
        for second in 0..6 {
            let a = input_source(&first_name, first, &canonical);
            let b = input_source(&second_name, second, &canonical);
            let sources = [admitted(a), admitted(b)];
            let error = refused(sources);
            assert_eq!(error.token(), "formula_ambiguous_name");
            assert_eq!(error.name(), "collision");
            assert!(matches!(error, E::AmbiguousName { .. }));
            let actual = error.binding_sources();
            assert_eq!(identity(actual[0]), identity(a));
            assert_eq!(identity(actual[1]), identity(b));
            assert!(std::ptr::eq(actual[0].name().as_ptr(), a.name().as_ptr()));
            assert!(std::ptr::eq(actual[1].name().as_ptr(), b.name().as_ptr()));
            assert_eq!(sources[0].declaration().name(), "collision");
        }
    }
}

#[test]
fn reserved_refusals_keep_fixed_metadata_and_attempted_sources_for_all_domains() {
    let canonical = record(LengthState::Derived { formula: id(9) });
    for (reserved, name, kind, origin, context) in RESERVED {
        let token = MachineToken::new(name).unwrap();
        let mut attempts = (0..6)
            .map(|source| input_source(&token, source, &canonical))
            .collect::<Vec<_>>();
        attempts.push(D::edge(&token, EdgeRef::new(id(57), LocalTag::new(93))));
        for source_origin in [P::Parameter, P::Profile, P::Material] {
            for kind in [B::Length, B::Angle, B::Area, B::Ratio, B::Count, B::Boolean] {
                attempts.push(D::input(&token, source_origin, id(61), id(62), kind));
            }
        }
        for attempt in attempts {
            let error = refused([admitted(attempt)]);
            assert_eq!(error.token(), "formula_rebinding");
            assert_eq!(error.name(), name);
            assert!(
                matches!(error, E::ReservedBinding { reserved: actual, .. } if actual == reserved)
            );
            let [prior, actual] = error.binding_sources();
            assert_eq!(prior.name(), name);
            assert_eq!(prior.kind(), kind);
            assert_eq!(prior.origin(), origin);
            assert!(
                matches!(prior.source(), S::Reserved(r) if r == reserved && r.required_context() == context)
            );
            assert_eq!(identity(actual), identity(attempt));
            assert_eq!(actual.kind(), attempt.kind());
        }
    }
}

#[test]
fn first_refusal_obeys_input_order_and_publishes_no_partial_namespace() {
    let name = MachineToken::new("z_collision").unwrap();
    let later = MachineToken::new("eps_num").unwrap();
    let a = admitted(scalar(&name, P::Parameter, 71));
    let b = admitted(scalar(&name, P::Material, 72));
    let c = admitted(scalar(&later, P::Profile, 73));
    let first = refused([a, b, c]);
    assert_eq!(first.token(), "formula_ambiguous_name");
    assert_eq!(first.name(), "z_collision");
    assert_eq!(identity(first.binding_sources()[0]), (O::Parameter, 71));
    assert_eq!(identity(first.binding_sources()[1]), (O::Material, 72));
    let reversed = refused([c, a, b]);
    assert_eq!(reversed.token(), "formula_rebinding");
    assert_eq!(reversed.name(), "eps_num");
    assert_eq!(identity(reversed.binding_sources()[1]), (O::Profile, 73));
}

#[test]
fn ordinary_names_have_exact_sorted_inspection_and_no_implicit_alias() {
    let names = [
        "z_name",
        "loop",
        "a2",
        "eps_geo_value",
        "fn",
        "macro",
        "repeat",
        "while",
    ]
    .map(|s| MachineToken::new(s).unwrap());
    let inputs = names
        .iter()
        .enumerate()
        .map(|(i, n)| admitted(scalar(n, P::Parameter, i as u128 + 11)));
    let ns = N::new(inputs).unwrap();
    assert_eq!(ns.declarations().len(), 16);
    let mut actual = ns.declarations().map(|d| d.name()).collect::<Vec<_>>();
    let mut expected = names.iter().map(MachineToken::as_str).collect::<Vec<_>>();
    expected.extend(RESERVED.iter().map(|(_, name, _, _, _)| *name));
    expected.sort_unstable();
    assert_eq!(actual, expected);
    actual.dedup();
    assert_eq!(actual.len(), 16);
}

#[test]
fn default_namespace_and_error_formatting_omit_authored_payloads() {
    let name = MachineToken::new("secret_customer_input").unwrap();
    let source = id(0xfacefeed);
    let d = D::input(&name, P::Parameter, source, id(0xbeefdeed), B::Length);
    let initial = admitted(d);
    let ns = N::new([initial]).unwrap();
    let error = refused([initial, initial]);
    for text in [
        format!("{ns:?}"),
        format!("{error:?}"),
        format!("{initial:?}"),
    ] {
        assert!(!text.contains("secret_customer_input"));
        assert!(!text.contains(&source.to_string()));
        assert!(!text.contains("facefeed"));
        assert!(!text.contains("beefdeed"));
    }
    assert_eq!(error.to_string(), "formula_ambiguous_name");
    let reserved_name = MachineToken::new("eps_phys").unwrap();
    let reserved_error = refused([admitted(scalar(&reserved_name, P::Material, 99))]);
    assert_eq!(reserved_error.to_string(), "formula_rebinding");
}
