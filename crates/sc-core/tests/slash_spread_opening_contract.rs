//! Explicit opening modes and actual scalar proofs,without invented cut/region/geometry inputs.
#![allow(clippy::expect_used, clippy::panic)]
use sc_core::{
    construction::{
        SlashSpreadCheckRefusal as Check, SlashSpreadNormalizationLimits as Limits,
        SlashSpreadNormalizationRefusal as Normal, SlashSpreadOpening as Opening,
        SlashSpreadOpeningMode as Mode,
    },
    name::MachineToken,
    ontology::EntityId,
    recipe::{
        FormulaCanonicalInput, FormulaDeclaration, FormulaExpressionCheckRefusal,
        FormulaExpressionDocument as Doc, FormulaInitialDeclaration, FormulaInputOrigin,
        FormulaKind, FormulaNamespace,
    },
    value::{
        CanonicalInputs, LengthDeclaration, LengthDeclarationDefinition, LengthState,
        LengthValueError,
    },
};
fn opening(mode: Mode, source: &str) -> Opening {
    let document = Doc::new(source.into()).expect("original syntax/input validated amount");
    match mode {
        Mode::Rotation => Opening::rotation(document),
        Mode::Separation => Opening::separation(document),
    }
}
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}

#[test]
fn explicit_modes_keep_original_documents_and_proofs_without_angle_reduction_or_division() {
    for (mode, source, kind) in [
        (Mode::Rotation, " (-720 deg) ", FormulaKind::Angle),
        (Mode::Separation, "1 mm/0", FormulaKind::Length),
    ] {
        let opening = opening(mode, source);
        assert_eq!(opening.mode(), mode);
        assert_eq!(opening.document().source(), source);
        let normalized = opening
            .normalized(Limits::new(8, 17))
            .expect("actual complete shape");
        let checked = normalized
            .check_argument(&FormulaNamespace::new([]).expect("namespace"))
            .expect("kind proof without runtime execution");
        assert!(std::ptr::eq(checked.normalized(), &normalized));
        assert!(std::ptr::eq(normalized.opening(), &opening));
        assert!(std::ptr::eq(
            checked.expression().expression(),
            normalized.expression()
        ));
        assert_eq!(checked.expression().kind(), kind);
        assert_eq!(
            &checked.expression().canonical_expression(),
            opening.document().canonical()
        );
        let span = checked.expression().root().span();
        assert_eq!(
            opening.document().source().get(span.start()..span.end()),
            Some(source.trim())
        );
        if mode == Mode::Rotation {
            assert_eq!(
                checked.expression().canonical_expression().as_str(),
                "(- angle:720000000)"
            );
        }
    }
}

#[test]
fn wrong_root_kind_never_reinterprets_the_explicit_opening_mode_or_original_span() {
    for (mode, source, actual, required) in [
        (
            Mode::Rotation,
            " (1 mm) ",
            FormulaKind::Length,
            FormulaKind::Angle,
        ),
        (
            Mode::Separation,
            " (90 deg) ",
            FormulaKind::Angle,
            FormulaKind::Length,
        ),
        (
            Mode::Rotation,
            " (1) ",
            FormulaKind::Count,
            FormulaKind::Angle,
        ),
    ] {
        let opening = opening(mode, source);
        let before = opening.clone();
        let normalized = opening
            .normalized(Limits::new(1, 3))
            .expect("single node shape");
        let error = normalized
            .check_argument(&FormulaNamespace::new([]).expect("namespace"))
            .expect_err("actual explicit mode requires another kind");
        assert!(std::ptr::eq(error.normalized(), &normalized));
        assert_eq!(error.mode(), mode);
        assert_eq!(error.required_kind(), required);
        assert_eq!(error.token(), "formula_dimension");
        assert!(matches!(error.refusal(),Check::Kind { actual:value } if *value==actual));
        let span = error.span();
        assert_eq!(source.get(span.start()..span.end()), Some(source.trim()));
        assert_eq!(opening, before);
    }
}

#[test]
fn original_unbound_and_untaken_branch_causes_keep_the_real_arena_without_reading_values() {
    for (source, token) in [
        (" private_secret ", "formula_unbound_name"),
        ("if(1==1,1 mm,1 deg)", "formula_dimension"),
    ] {
        let opening = opening(Mode::Separation, source);
        let normalized = opening
            .normalized(Limits::new(32, 65))
            .expect("all branches admitted");
        let error = normalized
            .check_argument(&FormulaNamespace::new([]).expect("namespace"))
            .expect_err("actual complete static cause");
        assert_eq!(error.token(), token);
        assert_eq!(error.mode(), Mode::Separation);
        let Check::Expression(original) = error.refusal() else {
            panic!("original expression cause required")
        };
        assert!(std::ptr::eq(original.expression(), normalized.expression()));
        assert_eq!(original.span(), error.span());
        if token == "formula_unbound_name" {
            assert!(matches!(
                original.refusal(),
                FormulaExpressionCheckRefusal::UnboundName(_)
            ));
            assert!(!format!("{error:?}").contains("private_secret"));
        } else {
            assert!(matches!(
                original.refusal(),
                FormulaExpressionCheckRefusal::Dimension(_)
            ));
        }
    }
}

#[test]
fn complete_one_owner_quota_precedes_normalization_and_preserves_the_original_document() {
    let opening = opening(Mode::Separation, "1 mm+2 mm/0");
    let before = opening.clone();
    let limits = Limits::new(4, 0);
    let error = opening
        .normalized(limits)
        .expect_err("five nodes before any retained check");
    assert!(std::ptr::eq(error.opening(), &opening));
    assert_eq!(error.limits(), limits);
    assert!(matches!(
        error.refusal(),
        Normal::Nodes {
            allowed: 4,
            required: 5
        }
    ));
    let error = opening
        .normalized(Limits::new(5, 10))
        .expect_err("one owner plus twice five nodes");
    assert!(matches!(
        error.refusal(),
        Normal::RetainedRecords {
            allowed: 10,
            required: 11
        }
    ));
    let limits = Limits::new(5, 11);
    let normalized = opening.normalized(limits).expect("exact complete quota");
    assert_eq!(normalized.node_count(), 5);
    assert_eq!(normalized.retained_records(), 11);
    assert_eq!(normalized.limits(), limits);
    assert_eq!(limits.nodes(), 5);
    assert_eq!(limits.retained_records(), 11);
    assert_eq!(opening, before);
}

#[test]
fn real_unknown_separation_source_stays_unknown_and_preserves_the_actual_canonical_supplier() {
    let inputs = CanonicalInputs::new(
        vec![LengthDeclaration::new(LengthDeclarationDefinition {
            id: id(40),
            source: id(30),
            state: LengthState::Unknown {
                observation: id(99),
            },
        })
        .expect("actual unknown length")],
        vec![],
    )
    .expect("canonical inventory");
    let before = inputs.clone();
    let record = inputs.length(id(40)).expect("actual original record");
    let name = MachineToken::new("unknown_gap").expect("source name");
    let namespace = FormulaNamespace::new([FormulaInitialDeclaration::try_from(
        FormulaDeclaration::length_input(&name, FormulaInputOrigin::Measurement, id(20), record),
    )
    .expect("actual source metadata")])
    .expect("actual namespace");
    let opening = opening(Mode::Separation, "unknown_gap");
    let normalized = opening
        .normalized(Limits::new(1, 3))
        .expect("complete shape");
    let checked = normalized
        .check_argument(&namespace)
        .expect("Unknown is statically Length");
    let dependency = checked
        .expression()
        .dependencies()
        .first()
        .expect("actual source use");
    let joined = FormulaCanonicalInput::resolve(dependency.declaration(), &inputs)
        .expect("actual canonical source");
    assert!(std::ptr::eq(joined.inputs(), &inputs));
    assert_eq!(inputs, before);
    assert_eq!(
        record.authored_value(),
        Err(LengthValueError::RequiresObservation {
            declaration: id(40),
            observation: id(99)
        })
    );
}
