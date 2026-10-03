//! Public metadata locators preserve authored sources without numeric availability or acceptance.
#![allow(clippy::unwrap_used, clippy::expect_used)]
use sc_core::{
    name::MachineToken,
    ontology::{EdgeRef, EntityId, LocalTag, PointRef},
    recipe::{
        FormulaBindingKind as B, FormulaDeclaration as D, FormulaDeclarationSource as S,
        FormulaInputOrigin as I, FormulaKind as K, FormulaOrigin as O, FormulaRecipe,
        FormulaReservedName as R, FormulaScalarInputOrigin as P,
    },
    value::{LengthDeclaration, LengthDeclarationDefinition, LengthState},
};
use sc_units::Length;

const ORIGINS: [(I, O); 5] = [
    (I::Measurement, O::Measurement),
    (I::Ease, O::Ease),
    (I::Parameter, O::Parameter),
    (I::Profile, O::Profile),
    (I::Material, O::Material),
];
const KINDS: [(B, K); 6] = [
    (B::Length, K::Length),
    (B::Angle, K::Angle),
    (B::Area, K::Area),
    (B::Ratio, K::Ratio),
    (B::Count, K::Count),
    (B::Boolean, K::Boolean),
];
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn length(state: LengthState) -> LengthDeclaration {
    LengthDeclaration::new(LengthDeclarationDefinition {
        id: id(301),
        source: id(302),
        state,
    })
    .unwrap()
}

#[test]
fn input_metadata_keeps_general_scalar_origins_kinds_and_distinct_canonical_ids() {
    let name = MachineToken::new("customer_input_2").unwrap();
    for (input_origin, origin) in [
        (P::Parameter, O::Parameter),
        (P::Profile, O::Profile),
        (P::Material, O::Material),
    ] {
        for (binding, kind) in KINDS {
            let declaration = D::input(&name, input_origin, id(11), id(12), binding);
            assert_eq!(declaration.name(), "customer_input_2");
            assert_eq!(declaration.origin(), origin);
            assert_eq!(declaration.kind(), kind);
            let S::Input {
                origin: actual_origin,
                input,
                declaration: canonical,
                kind: actual_kind,
            } = declaration.source()
            else {
                assert!(matches!(declaration.source(), S::Input { .. }));
                return;
            };
            assert_eq!(actual_origin, input_origin);
            assert_eq!(input, id(11));
            assert_eq!(canonical, id(12));
            assert_eq!(actual_kind, binding);
        }
    }
}

#[test]
fn canonical_length_adapter_borrows_every_state_without_fetching_a_value() {
    let name = MachineToken::new("body_width").unwrap();
    for state in [
        LengthState::Known {
            value: Length::from_micrometres(831).unwrap(),
            evidence: vec![id(1)],
        },
        LengthState::Assumed {
            value: Length::from_micrometres(-417).unwrap(),
            assumption: id(2),
        },
        LengthState::Unknown { observation: id(3) },
        LengthState::Preference {
            value: Length::from_micrometres(552).unwrap(),
            provenance: id(4),
        },
        LengthState::Derived { formula: id(5) },
    ] {
        let record = length(state);
        for (input_origin, origin) in ORIGINS {
            let declaration = D::length_input(&name, input_origin, id(21), &record);
            assert_eq!(declaration.name(), "body_width");
            assert_eq!(declaration.kind(), K::Length);
            assert_eq!(declaration.origin(), origin);
            let S::LengthInput {
                origin: actual_origin,
                input,
                declaration: borrowed,
            } = declaration.source()
            else {
                assert!(matches!(declaration.source(), S::LengthInput { .. }));
                return;
            };
            assert!(std::ptr::eq(borrowed, &record));
            assert_eq!(borrowed.id(), id(301));
            assert_eq!(borrowed.source(), id(302));
            assert_eq!(input, id(21));
            assert_eq!(actual_origin, input_origin);
        }
    }
}

#[test]
fn geometry_declarations_keep_exact_creator_tags_and_forced_kinds() {
    let name = MachineToken::new("constructed_2").unwrap();
    for creator in [id(44), id(45)] {
        for tag in [LocalTag::FIRST, LocalTag::new(97)] {
            let point = PointRef::new(creator, tag);
            let edge = EdgeRef::new(creator, tag);
            let p = D::point(&name, point);
            let e = D::edge(&name, edge);
            assert_eq!(p.kind(), K::Point);
            assert_eq!(e.kind(), K::Edge);
            assert_eq!(p.origin(), O::Geometry);
            assert_eq!(e.origin(), O::Geometry);
            assert_eq!(p.name(), "constructed_2");
            assert_eq!(e.name(), "constructed_2");
            assert!(matches!(p.source(),S::Point(actual) if actual==point));
            assert!(matches!(e.source(),S::Edge(actual) if actual==edge));
            assert_eq!(p.kind().binding_kind(), None);
            assert_eq!(e.kind().binding_kind(), None);
        }
    }
}

#[test]
fn recipe_sources_use_actual_positions_labels_annotations_and_global_spans() {
    let source="  let first:length=missing\nassert check:eps_num=first==first\nlet third:count=1 let fourth:boolean=1<2  ";
    let recipe = FormulaRecipe::parse(source)
        .unwrap()
        .normalize_literals()
        .unwrap();
    for (index, name, kind, statement_text) in [
        (1, "first", K::Length, "let first:length=missing"),
        (3, "third", K::Count, "let third:count=1"),
        (4, "fourth", K::Boolean, "let fourth:boolean=1<2"),
    ] {
        let declaration = D::recipe(&recipe, index).unwrap();
        assert_eq!(declaration.name(), name);
        assert_eq!(declaration.kind(), kind);
        assert_eq!(declaration.origin(), O::Recipe);
        let S::Recipe {
            statement_index,
            kind: binding,
            span,
            name_span,
        } = declaration.source()
        else {
            assert!(matches!(declaration.source(), S::Recipe { .. }));
            return;
        };
        assert_eq!(statement_index, index);
        assert_eq!(K::from(binding), kind);
        let start = source.find(statement_text).unwrap();
        assert_eq!(
            (span.start(), span.end()),
            (start, start + statement_text.len())
        );
        let name_start = start + 4;
        assert_eq!(
            (name_span.start(), name_span.end()),
            (name_start, name_start + name.len())
        );
    }
    for index in [0, 2, 5, usize::MAX] {
        assert!(D::recipe(&recipe, index).is_none());
    }
    let empty = FormulaRecipe::parse("")
        .unwrap()
        .normalize_literals()
        .unwrap();
    assert!(D::recipe(&empty, 1).is_none());
    // Missing expression names and reserved let names remain syntax metadata, not accepted bindings.
    let reserved = FormulaRecipe::parse("let eps_geo:angle=missing")
        .unwrap()
        .normalize_literals()
        .unwrap();
    let declaration = D::recipe(&reserved, 1).unwrap();
    assert_eq!(declaration.name(), "eps_geo");
    assert_eq!(declaration.kind(), K::Angle);
}

#[test]
fn all_six_recipe_annotations_survive_without_expression_type_inference() {
    for (binding, kind) in KINDS {
        let source = format!("let independent:{}=missing", binding.token());
        let recipe = FormulaRecipe::parse(&source)
            .unwrap()
            .normalize_literals()
            .unwrap();
        let declaration = D::recipe(&recipe, 1).unwrap();
        assert_eq!(declaration.kind(), kind);
        assert_eq!(declaration.origin(), O::Recipe);
        assert_eq!(declaration.name(), "independent");
    }
    let mut source = String::new();
    for index in 1..=4096 {
        source.push_str(&format!("let n{index}:count=1 "));
    }
    let recipe = FormulaRecipe::parse(&source)
        .unwrap()
        .normalize_literals()
        .unwrap();
    assert_eq!(D::recipe(&recipe, 4096).unwrap().name(), "n4096");
    assert!(matches!(
        D::recipe(&recipe, 4096).unwrap().source(),
        S::Recipe {
            statement_index: 4096,
            ..
        }
    ));
    assert!(D::recipe(&recipe, 4097).is_none());
}

#[test]
fn reserved_sources_derive_fixed_metadata_without_value_contexts() {
    let expected = [
        ("eps_num", K::Length, O::Tolerance),
        ("eps_geo", K::Length, O::Tolerance),
        ("eps_fmt", K::Length, O::Tolerance),
        ("eps_imp", K::Length, O::Tolerance),
        ("eps_phys", K::Length, O::Tolerance),
        ("size_index", K::Count, O::Size),
        ("size_count", K::Count, O::Size),
        ("is_base_size", K::Boolean, O::Size),
    ];
    assert_eq!(R::ALL.len(), expected.len());
    for (reserved, (name, kind, origin)) in R::ALL.into_iter().zip(expected) {
        let declaration = D::reserved(reserved);
        assert_eq!(declaration.name(), name);
        assert_eq!(declaration.kind(), kind);
        assert_eq!(declaration.origin(), origin);
        assert!(matches!(declaration.source(),S::Reserved(actual) if actual==reserved));
    }
}

#[test]
fn default_debug_omits_names_identities_states_and_numeric_values_even_for_source_views() {
    let name = MachineToken::new("private_customer_body").unwrap();
    let record = length(LengthState::Known {
        value: Length::from_micrometres(984213).unwrap(),
        evidence: vec![id(88)],
    });
    let recipe = FormulaRecipe::parse("let private_customer_body:length=1 um")
        .unwrap()
        .normalize_literals()
        .unwrap();
    let declarations = [
        D::length_input(&name, I::Measurement, id(9), &record),
        D::input(&name, P::Parameter, id(9), id(10), B::Angle),
        D::point(&name, PointRef::new(id(9), LocalTag::new(8))),
        D::edge(&name, EdgeRef::new(id(9), LocalTag::new(8))),
        D::recipe(&recipe, 1).unwrap(),
        D::reserved(R::SizeIndex),
    ];
    for declaration in declarations {
        let copy = declaration;
        assert_eq!(copy.name(), declaration.name());
        for rendered in [
            format!("{declaration:?}"),
            format!("{:?}", declaration.source()),
        ] {
            for private in [
                "private_customer_body",
                "984213",
                "Known",
                "LengthDeclaration",
                &id(9).to_string(),
                &id(301).to_string(),
            ] {
                assert!(!rendered.contains(private));
            }
            assert!(rendered.contains("kind"));
            assert!(rendered.contains("origin"));
        }
    }
}
