//! Whole source preparation preserves actual suppliers without manufacturing runtime values.
#![allow(clippy::expect_used)]
use sc_core::{
    name::MachineToken,
    ontology::EntityId,
    recipe::{
        FormulaBindingKind, FormulaBuiltin, FormulaCanonicalInputAdapter as Adapter,
        FormulaCanonicalInputRefusal as JoinRefusal, FormulaCheckedStatementKind as Statement,
        FormulaDeclaration as D, FormulaInitialDeclaration, FormulaInputOrigin, FormulaNamespace,
        FormulaRecipe, FormulaRecipeDependencyRole as Role, FormulaRecipeExecutionLimits as Limits,
        FormulaReservedName, FormulaScalarInputOrigin, FormulaScalarRecipePlanRefusal as Refusal,
        FormulaScalarRecipeSourceKind as Kind, FormulaStatementExpression, FormulaToleranceName,
    },
    value::{CanonicalInputs, LengthDeclaration, LengthDeclarationDefinition, LengthState},
};
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn inputs() -> CanonicalInputs {
    CanonicalInputs::new(
        vec![
            LengthDeclaration::new(LengthDeclarationDefinition {
                id: id(1),
                source: id(2),
                state: LengthState::Unknown { observation: id(3) },
            })
            .expect("actual Unknown"),
            LengthDeclaration::new(LengthDeclarationDefinition {
                id: id(10),
                source: id(11),
                state: LengthState::Derived { formula: id(12) },
            })
            .expect("actual Derived"),
        ],
        vec![],
    )
    .expect("inventory")
}

#[test]
fn complete_actual_unknown_derived_and_reserved_sources_join_before_any_value_or_provider_read() {
    let inputs = inputs();
    let before = inputs.clone();
    let width = MachineToken::new("private_width").expect("name");
    let derived = MachineToken::new("derived_width").expect("name");
    let namespace = FormulaNamespace::new(
        [
            D::length_input(
                &width,
                FormulaInputOrigin::Measurement,
                id(4),
                inputs.length(id(1)).expect("actual"),
            ),
            D::length_input(
                &derived,
                FormulaInputOrigin::Parameter,
                id(5),
                inputs.length(id(10)).expect("actual"),
            ),
        ]
        .map(|d| FormulaInitialDeclaration::try_from(d).expect("initial")),
    )
    .expect("namespace");
    let text = "let first:length=if(is_base_size,private_width+private_width,private_width)\nassert check:eps_phys=first==private_width\nlet last:length=first+derived_width";
    let recipe = FormulaRecipe::parse(text)
        .expect("syntax")
        .normalize_literals()
        .expect("inputs");
    let proof = recipe
        .check_kinds(namespace)
        .expect("complete static proof");
    let plan = proof
        .admit_execution(Limits::new(3, 11, 31, 3))
        .expect("exact allowances")
        .prepare_scalar(&inputs)
        .expect("unread actual requirements/optional contexts");
    assert!(std::ptr::eq(plan.admission().proof(), &proof));
    assert!(std::ptr::eq(plan.inputs(), &inputs));
    assert_eq!(plan.sources().len(), 9);
    let mut counts = (0, 0, 0);
    for (source, original) in plan.sources().iter().zip(proof.dependencies()) {
        let dependency = source.dependency();
        assert_eq!(
            (
                dependency.statement_index(),
                dependency.role(),
                dependency.span()
            ),
            (original.statement_index(), original.role(), original.span())
        );
        let span = dependency.span();
        assert_eq!(
            text.get(span.start()..span.end()),
            Some(dependency.declaration().name())
        );
        assert!(span.start() < span.end());
        match source.kind() {
            Kind::Canonical(joined) => {
                counts.0 += 1;
                let Adapter::Length(actual) = joined.adapter() else {
                    unreachable!("length fixture");
                };
                let id = if dependency.declaration().name() == "private_width" {
                    id(1)
                } else {
                    id(10)
                };
                let record = inputs.length(id).expect("actual record");
                assert!(std::ptr::eq(joined.inputs(), &inputs));
                assert!(std::ptr::eq(actual.canonical_declaration(), record));
                assert!(std::ptr::eq(actual.state(), record.state()));
            }
            Kind::Reserved(name) => {
                counts.1 += 1;
                if dependency.role() == Role::AssertionTolerance {
                    assert_eq!(
                        name,
                        FormulaReservedName::Tolerance(FormulaToleranceName::Physical)
                    );
                    assert_eq!(dependency.statement_index(), 2);
                }
            }
            Kind::PriorLet(supplier) => {
                counts.2 += 1;
                assert!(std::ptr::eq(
                    supplier,
                    proof.statements().first().expect("actual first")
                ));
                assert!(supplier.statement_index() < dependency.statement_index());
            }
        }
    }
    assert_eq!(counts, (5, 2, 2));
    assert_eq!(inputs, before);
    assert!(!format!("{plan:?}").contains("private_width"));
}

#[test]
fn prior_let_links_use_actual_checked_supplier_positions_including_interleaved_assertions() {
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
    let recipe = FormulaRecipe::parse("let first:count=1\nassert check:eps_phys=first==first\nlet last:count=first+1\nassert again:eps_num=last==last")
        .expect("syntax").normalize_literals().expect("inputs");
    let proof = recipe
        .check_kinds(FormulaNamespace::new([]).expect("namespace"))
        .expect("whole proof");
    let plan = proof
        .admit_execution(Limits::new(4, 8, 23, 4))
        .expect("exact")
        .prepare_scalar(&inputs)
        .expect("actual metadata suppliers only");
    let mut supplier_positions = Vec::new();
    for source in plan.sources() {
        if let Kind::PriorLet(supplier) = source.kind() {
            assert!(matches!(supplier.kind(), Statement::Let { .. }));
            let actual = proof
                .statements()
                .get(supplier.statement_index() - 1)
                .expect("real position");
            assert!(std::ptr::eq(supplier, actual));
            assert!(supplier.statement_index() < source.dependency().statement_index());
            supplier_positions.push(supplier.statement_index());
        }
    }
    assert_eq!(supplier_positions, [1, 1, 1, 3, 3]);
    assert_eq!(plan.sources().len(), 7);
}

#[test]
fn late_capability_gap_precedes_allocation_and_earlier_missing_inventory_join() {
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
    let missing = MachineToken::new("private_missing").expect("name");
    let declaration = D::input(
        &missing,
        FormulaScalarInputOrigin::Parameter,
        id(7),
        id(99),
        FormulaBindingKind::Length,
    );
    let namespace =
        FormulaNamespace::new([FormulaInitialDeclaration::try_from(declaration).expect("initial")])
            .expect("namespace");
    let text = "let first:length=private_missing\nassert later:eps_phys=1.0==tan(0 deg)";
    let recipe = FormulaRecipe::parse(text)
        .expect("syntax")
        .normalize_literals()
        .expect("inputs");
    let proof = recipe
        .check_kinds(namespace)
        .expect("complete actual static proof");
    let error = proof
        .admit_execution(Limits::new(2, 4, 10, 2))
        .expect("exact")
        .prepare_scalar(&inputs)
        .expect_err("whole capability scan before any earlier join");
    assert!(matches!(
        error.refusal(),
        Refusal::UnavailableBuiltin(FormulaBuiltin::Tan)
    ));
    let statement = error.statement().expect("actual failed operand owner");
    assert_eq!(statement.statement_index(), 2);
    assert!(std::ptr::eq(
        statement,
        proof.statements().get(1).expect("actual second")
    ));
    assert_eq!(
        error.expression_part(),
        Some(FormulaStatementExpression::AssertionRight)
    );
    let node = error.node().expect("actual tangent node");
    let Statement::Assert { right, .. } = statement.kind() else {
        unreachable!("assert fixture");
    };
    assert!(std::ptr::eq(node.expression(), right.expression()));
    let span = node.span();
    assert_eq!(text.get(span.start()..span.end()), Some("tan(0 deg)"));
    assert!(error.dependency().is_none());
    assert!(error.allocation_error().is_none());
    assert!(std::ptr::eq(error.inputs(), &inputs));
    assert!(std::ptr::eq(error.proof(), &proof));
    assert!(!format!("{error:?} {error}").contains("private_missing"));
}

#[test]
fn missing_inventory_target_in_untaken_branch_keeps_exact_consumer_source_and_original_cause() {
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
    let missing = MachineToken::new("private_missing").expect("name");
    let other = MachineToken::new("other_missing").expect("name");
    let namespace = FormulaNamespace::new(
        [
            D::input(
                &missing,
                FormulaScalarInputOrigin::Parameter,
                id(7),
                id(99),
                FormulaBindingKind::Length,
            ),
            D::input(
                &other,
                FormulaScalarInputOrigin::Parameter,
                id(8),
                id(100),
                FormulaBindingKind::Length,
            ),
        ]
        .map(|d| FormulaInitialDeclaration::try_from(d).expect("initial")),
    )
    .expect("namespace");
    let text = "let first:length=1 mm\nassert later:eps_phys=first==if(1==1,1 mm,private_missing+other_missing)";
    let recipe = FormulaRecipe::parse(text)
        .expect("syntax")
        .normalize_literals()
        .expect("inputs");
    let proof = recipe
        .check_kinds(namespace)
        .expect("complete actual static proof");
    let error = proof
        .admit_execution(Limits::new(2, 10, 24, 2))
        .expect("exact")
        .prepare_scalar(&inputs)
        .expect_err("all static occurrences bind,not only selected ones");
    let dependency = error.dependency().expect("actual refused occurrence");
    assert_eq!(dependency.statement_index(), 2);
    assert_eq!(dependency.role(), Role::AssertionRight);
    assert_eq!(dependency.declaration().name(), "private_missing");
    assert_eq!(
        error
            .statement()
            .expect("actual consumer")
            .statement_index(),
        2
    );
    assert!(error.node().is_none() && error.expression_part().is_none());
    let Refusal::CanonicalInput(cause) = error.refusal() else {
        unreachable!("canonical join fixture");
    };
    assert!(
        matches!(cause.refusal(), JoinRefusal::MissingScalar { declaration, length_index: None } if declaration == id(99))
    );
    assert_eq!(cause.metadata().name(), "private_missing");
    assert!(std::ptr::eq(cause.inputs(), &inputs));
    assert!(error.allocation_error().is_none());
    assert!(!format!("{error:?} {error}").contains("private_missing"));
}

#[test]
fn empty_and_maximum_statement_plans_retain_complete_header_populations_without_context_values() {
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
    for (text, statements, nodes, names, workspace) in [
        (String::new(), 0, 0, 0, 0),
        (
            (0..4096)
                .map(|n| format!("assert check_{n}:eps_phys=1==1"))
                .collect::<Vec<_>>()
                .join("\n"),
            4096,
            8192,
            4096,
            20480,
        ),
    ] {
        let recipe = FormulaRecipe::parse(&text)
            .expect("syntax")
            .normalize_literals()
            .expect("inputs");
        let proof = recipe
            .check_kinds(FormulaNamespace::new([]).expect("namespace"))
            .expect("whole proof");
        let plan = proof
            .admit_execution(Limits::new(statements, nodes, workspace, statements))
            .expect("exact")
            .prepare_scalar(&inputs)
            .expect("no physical provider read");
        assert_eq!(plan.sources().len(), names);
        for (position, source) in plan.sources().iter().enumerate() {
            assert_eq!(source.dependency().statement_index(), position + 1);
            assert_eq!(source.dependency().role(), Role::AssertionTolerance);
            assert!(matches!(
                source.kind(),
                Kind::Reserved(FormulaReservedName::Tolerance(
                    FormulaToleranceName::Physical
                ))
            ));
        }
    }
}
