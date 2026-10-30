//! Scalar preparation binds complete actual sources without executing values or missing backends.
#![allow(clippy::expect_used)]
use sc_core::{
    name::MachineToken,
    ontology::{EntityId, LocalTag, PointRef},
    recipe::{
        FormulaBindingKind, FormulaCanonicalInputAdapter as Adapter,
        FormulaCanonicalInputRefusal as JoinRefusal, FormulaCheckedNodeForm as Form,
        FormulaCheckedStatementKind as Statement, FormulaDeclaration as D,
        FormulaDeclarationSource as Source, FormulaExecutionLimits as Limits, FormulaExpression,
        FormulaInitialDeclaration, FormulaInputOrigin, FormulaNamespace, FormulaRecipe,
        FormulaReservedName, FormulaScalarExecutionPlanRefusal as Refusal,
        FormulaScalarExecutionSourceKind as SourceKind, FormulaScalarInputOrigin,
        FormulaToleranceName,
    },
    value::{CanonicalInputs, LengthDeclaration, LengthDeclarationDefinition, LengthState},
};

fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn inputs() -> CanonicalInputs {
    CanonicalInputs::new(
        vec![LengthDeclaration::new(LengthDeclarationDefinition {
            id: id(1),
            source: id(2),
            state: LengthState::Unknown { observation: id(3) },
        })
        .expect("actual unknown")],
        vec![],
    )
    .expect("actual canonical inventory")
}

#[test]
fn implemented_scalar_shapes_prepare_without_computing_calls_or_reading_optional_providers() {
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("empty inventory");
    let namespace = FormulaNamespace::new([]).expect("reserved namespace");
    for source in [
        "1 mm",
        "-((2 mm)^2)",
        "1 mm+2 mm",
        "1 mm/2 mm",
        "1 mm<2 mm",
        "sqrt((2 mm)^2)",
        "hypot(3 mm,4 mm)",
        "abs(-1 mm)",
        "min(1 mm,2 mm)",
        "max(1 mm,2 mm)",
        "clamp(1 mm,0 mm,2 mm)",
        "round_to(1 mm,1 um)",
        "sin(90 deg)",
        "cos(90 deg)",
        "atan(100 pct)",
        "atan2(1 mm,2 mm)",
        "arc_length(90 deg,2 mm)",
        "within(1 mm,2 mm,eps_phys)",
        "if(1 mm<2 mm,1 mm,eps_fmt)",
    ] {
        let expression = FormulaExpression::parse(source)
            .expect("syntax")
            .normalize_literals()
            .expect("literal inputs");
        let proof = expression
            .check_kinds(&namespace)
            .expect("complete static proof");
        let admission = proof
            .admit_execution(Limits::new(256, 768))
            .expect("explicit bounds");
        let plan = admission
            .prepare_scalar(&inputs)
            .expect("actual available scalar scope");
        assert!(std::ptr::eq(plan.inputs(), &inputs));
        assert!(std::ptr::eq(
            plan.admission().root().expression(),
            &expression
        ));
        assert_eq!(plan.sources().len(), proof.dependencies().len());
        for (source, dependency) in plan.sources().zip(proof.dependencies()) {
            assert_eq!(source.node().span(), dependency.span());
            assert!(std::ptr::eq(source.node().expression(), &expression));
            assert!(matches!(source.kind(), SourceKind::Reserved(_)));
        }
    }
}

#[test]
fn all_repeated_and_untaken_sources_bind_to_actual_unknown_records_and_original_reserved_roles() {
    let name = MachineToken::new("private_width").expect("name");
    let inputs = inputs();
    let record = inputs.length(id(1)).expect("actual record");
    let metadata = D::length_input(&name, FormulaInputOrigin::Measurement, id(4), record);
    let namespace = FormulaNamespace::new([
        FormulaInitialDeclaration::try_from(metadata).expect("initial declaration")
    ])
    .expect("namespace");
    let expression =
        FormulaExpression::parse("if(is_base_size,private_width+private_width,eps_phys)")
            .expect("syntax")
            .normalize_literals()
            .expect("inputs");
    let proof = expression
        .check_kinds(&namespace)
        .expect("both static branches");
    let plan = proof
        .admit_execution(Limits::new(6, 16))
        .expect("exact full bound")
        .prepare_scalar(&inputs)
        .expect("no value or context read");
    assert_eq!(plan.sources().len(), 4);
    let mut canonical = 0;
    let mut roles = Vec::new();
    for (source, dependency) in plan.sources().zip(proof.dependencies()) {
        assert_eq!(source.node().span(), dependency.span());
        match source.kind() {
            SourceKind::Canonical(joined) => {
                canonical += 1;
                assert!(std::ptr::eq(joined.inputs(), &inputs));
                let Adapter::Length(input) = joined.adapter() else {
                    unreachable!("actual dedicated length fixture");
                };
                assert!(std::ptr::eq(input.canonical_declaration(), record));
                assert!(std::ptr::eq(input.state(), record.state()));
            }
            SourceKind::Reserved(name) => roles.push(name),
        }
    }
    assert_eq!(canonical, 2);
    assert_eq!(
        roles,
        [
            FormulaReservedName::IsBaseSize,
            FormulaReservedName::Tolerance(FormulaToleranceName::Physical)
        ]
    );
    assert_eq!(
        plan.sources()
            .rev()
            .map(|source| source.node().span())
            .collect::<Vec<_>>(),
        proof
            .dependencies()
            .iter()
            .rev()
            .map(|dependency| dependency.span())
            .collect::<Vec<_>>()
    );
    assert!(!format!("{plan:?}").contains("private_width"));
    assert!(
        matches!(record.state(), LengthState::Unknown { observation } if *observation == id(3))
    );
}

#[test]
fn a_late_untaken_missing_target_has_actual_node_owner_and_full_join_cause_without_partial_plan() {
    let width = MachineToken::new("width").expect("name");
    let missing = MachineToken::new("missing").expect("name");
    let inputs = inputs();
    let namespace = FormulaNamespace::new(
        [
            D::length_input(
                &width,
                FormulaInputOrigin::Measurement,
                id(4),
                inputs.length(id(1)).expect("owned"),
            ),
            D::input(
                &missing,
                FormulaScalarInputOrigin::Parameter,
                id(5),
                id(99),
                FormulaBindingKind::Length,
            ),
        ]
        .map(|metadata| FormulaInitialDeclaration::try_from(metadata).expect("initial")),
    )
    .expect("namespace");
    let source = "if(is_base_size,width,missing)";
    let expression = FormulaExpression::parse(source)
        .expect("syntax")
        .normalize_literals()
        .expect("inputs");
    let proof = expression
        .check_kinds(&namespace)
        .expect("complete kind proof does not read registry");
    let error = proof
        .admit_execution(Limits::new(4, 11))
        .expect("explicit bound")
        .prepare_scalar(&inputs)
        .expect_err("complete actual source binding,even untaken");
    assert!(std::ptr::eq(error.node().expression(), &expression));
    assert!(std::ptr::eq(error.inputs(), &inputs));
    assert_eq!(
        source.get(error.node().span().start()..error.node().span().end()),
        Some("missing")
    );
    let Refusal::CanonicalInput(cause) = error.refusal() else {
        unreachable!("actual canonical target refusal");
    };
    assert!(
        matches!(cause.refusal(), JoinRefusal::MissingScalar { declaration, length_index: None }
        if declaration == id(99))
    );
    assert_eq!(cause.metadata().name(), "missing");
    assert!(std::ptr::eq(cause.inputs(), &inputs));
    assert!(error.allocation_error().is_none());
    assert!(!format!("{error:?}").contains("missing"));
}

#[test]
fn unavailable_implementations_remain_preparation_gaps_before_registry_binding_or_values() {
    let missing = MachineToken::new("missing").expect("name");
    let point = MachineToken::new("point_value").expect("name");
    let inputs = inputs();
    let namespace = FormulaNamespace::new(
        [
            D::input(
                &missing,
                FormulaScalarInputOrigin::Parameter,
                id(4),
                id(99),
                FormulaBindingKind::Length,
            ),
            D::point(&point, PointRef::new(id(6), LocalTag::FIRST)),
        ]
        .map(|metadata| FormulaInitialDeclaration::try_from(metadata).expect("initial")),
    )
    .expect("namespace");
    for source in [
        "tan(0 deg)",
        "missing+tan(0 deg)*1 mm",
        "if(1 mm<2 mm,1 mm,tan(0 deg)*1 mm)",
    ] {
        let expression = FormulaExpression::parse(source)
            .expect("syntax")
            .normalize_literals()
            .expect("inputs");
        let proof = expression
            .check_kinds(&namespace)
            .expect("tan remains admitted language");
        let error = proof
            .admit_execution(Limits::new(256, 768))
            .expect("bound")
            .prepare_scalar(&inputs)
            .expect_err("no selected tangent policy");
        assert!(matches!(error.refusal(), Refusal::MissingTangentPolicy));
        assert_eq!(
            source.get(error.node().span().start()..error.node().span().end()),
            Some("tan(0 deg)")
        );
        assert!(error.allocation_error().is_none());
    }
    let expression = FormulaExpression::parse("point_value")
        .expect("syntax")
        .normalize_literals()
        .expect("inputs");
    let proof = expression
        .check_kinds(&namespace)
        .expect("actual geometry name proof");
    let error = proof
        .admit_execution(Limits::new(1, 3))
        .expect("bound")
        .prepare_scalar(&inputs)
        .expect_err("real geometry integration remains owned");
    assert!(
        matches!(error.refusal(), Refusal::UnavailableSource(Source::Point(reference))
        if reference == PointRef::new(id(6), LocalTag::FIRST))
    );

    let recipe = FormulaRecipe::parse("let first:length=1 mm\nlet second:length=first")
        .expect("recipe")
        .normalize_literals()
        .expect("inputs");
    let proof = recipe
        .check_kinds(FormulaNamespace::new([]).expect("namespace"))
        .expect("whole proof");
    let second = proof.statements().get(1).expect("actual statement");
    let Statement::Let { expression } = second.kind() else {
        unreachable!("actual let");
    };
    let error = expression
        .admit_execution(Limits::new(1, 3))
        .expect("bound")
        .prepare_scalar(&inputs)
        .expect_err("no caller-injected prior binding");
    assert!(matches!(
        error.refusal(),
        Refusal::UnavailableSource(Source::Recipe {
            statement_index: 1,
            ..
        })
    ));
    assert!(matches!(
        error.node().form(),
        Form::Name { name: "first", .. }
    ));
}

#[test]
fn the_full_256_node_arena_binds_255_distinct_original_occurrences_without_deduplication() {
    let source = format!("min({})", vec!["eps_num"; 255].join(","));
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
    let expression = FormulaExpression::parse(&source)
        .expect("max legal syntax")
        .normalize_literals()
        .expect("inputs");
    let proof = expression
        .check_kinds(&FormulaNamespace::new([]).expect("namespace"))
        .expect("whole proof");
    let plan = proof
        .admit_execution(Limits::new(256, 767))
        .expect("exact full reservation")
        .prepare_scalar(&inputs)
        .expect("complete source vector");
    assert_eq!(plan.admission().node_count(), 256);
    assert_eq!(plan.admission().workspace_records(), 767);
    assert_eq!(plan.sources().len(), 255);
    for (source, dependency) in plan.sources().zip(proof.dependencies()) {
        assert_eq!(source.node().span(), dependency.span());
        assert!(matches!(
            source.kind(),
            SourceKind::Reserved(FormulaReservedName::Tolerance(
                FormulaToleranceName::Numerical
            ))
        ));
    }
}
