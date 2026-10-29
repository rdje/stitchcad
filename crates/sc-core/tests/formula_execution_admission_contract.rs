//! Admission measures the complete actual proof before any execution or source read.
#![allow(clippy::expect_used)]
use sc_core::{
    name::MachineToken,
    ontology::EntityId,
    recipe::{
        FormulaCheckedNodeForm as Form, FormulaCheckedStatementKind as Statement,
        FormulaDeclaration, FormulaDeclarationSource as Source,
        FormulaExecutionAdmissionRefusal as Refusal, FormulaExecutionLimits as Limits,
        FormulaExpression, FormulaInitialDeclaration, FormulaInputOrigin, FormulaNamespace,
        FormulaRecipe,
    },
    value::{LengthDeclaration, LengthDeclarationDefinition, LengthState},
};

#[test]
fn actual_complete_node_counts_and_independent_quota_priority_keep_original_owners() {
    let namespace = FormulaNamespace::new([]).expect("namespace");
    for (source, nodes, names, records) in [
        ("1 mm", 1, 0, 2),
        ("(eps_num)", 1, 1, 3),
        ("eps_num+eps_num", 3, 2, 8),
        ("if(is_base_size,1 mm,2 mm)", 4, 1, 9),
        ("-((1 mm)^2)/(2 mm)", 5, 0, 10),
        ("clamp(1 mm,0 mm,2 mm)", 4, 0, 8),
    ] {
        let expression = FormulaExpression::parse(source)
            .expect("syntax")
            .normalize_literals()
            .expect("inputs");
        let proof = expression.check_kinds(&namespace).expect("complete proof");
        assert_eq!(expression.node_count(), nodes);
        let node_error = proof
            .admit_execution(Limits::new(nodes - 1, 0))
            .expect_err("node quota precedes workspace");
        assert_eq!(
            node_error.refusal(),
            Refusal::Nodes {
                allowed: nodes - 1,
                actual: nodes
            }
        );
        assert!(std::ptr::eq(node_error.root().expression(), &expression));
        assert_eq!(node_error.root().span(), proof.root().span());
        let error = proof
            .admit_execution(Limits::new(nodes, records - 1))
            .expect_err("complete workspace cannot fit");
        assert_eq!(
            error.refusal(),
            Refusal::WorkspaceRecords {
                allowed: records - 1,
                required: records
            }
        );
        assert!(std::ptr::eq(error.root().expression(), &expression));
        let limits = Limits::new(nodes, records);
        let admitted = proof.admit_execution(limits).expect("exact allowances");
        assert_eq!(admitted.limits(), limits);
        assert_eq!(admitted.limits().nodes(), nodes);
        assert_eq!(admitted.limits().workspace_records(), records);
        assert_eq!(admitted.node_count(), nodes);
        assert_eq!(admitted.name_use_count(), names);
        assert_eq!(admitted.workspace_records(), records);
        assert!(std::ptr::eq(admitted.root().expression(), &expression));
        assert!(std::ptr::eq(admitted.dependencies(), proof.dependencies()));
        assert_eq!(admitted.root().span(), proof.root().span());
        assert_eq!(admitted.root().kind(), proof.kind());
    }
    // Equal source/kind/populations do not substitute a different actual normalized owner.
    let first = FormulaExpression::parse("1 mm")
        .expect("syntax")
        .normalize_literals()
        .expect("inputs");
    let second = FormulaExpression::parse("1 mm")
        .expect("syntax")
        .normalize_literals()
        .expect("inputs");
    let proof = first.check_kinds(&namespace).expect("proof");
    let admitted = proof.admit_execution(Limits::new(1, 2)).expect("admit");
    assert!(!std::ptr::eq(admitted.root().expression(), &second));
}

#[test]
fn repeated_and_untaken_names_reserve_distinct_uses_without_reading_actual_unknown_state() {
    let name = MachineToken::new("body_width").expect("name");
    let record = LengthDeclaration::new(LengthDeclarationDefinition {
        id: EntityId::from_bits(1),
        source: EntityId::from_bits(2),
        state: LengthState::Unknown {
            observation: EntityId::from_bits(3),
        },
    })
    .expect("canonical unknown");
    let metadata = FormulaDeclaration::length_input(
        &name,
        FormulaInputOrigin::Measurement,
        EntityId::from_bits(4),
        &record,
    );
    let namespace =
        FormulaNamespace::new([FormulaInitialDeclaration::try_from(metadata).expect("initial")])
            .expect("namespace");
    let source = "if(is_base_size,body_width+body_width,body_width)";
    let expression = FormulaExpression::parse(source)
        .expect("syntax")
        .normalize_literals()
        .expect("inputs");
    let proof = expression.check_kinds(&namespace).expect("both branches");
    let admitted = proof.admit_execution(Limits::new(6, 16)).expect("unread");
    assert_eq!(admitted.node_count(), 6);
    assert_eq!(admitted.name_use_count(), 4);
    assert_eq!(admitted.workspace_records(), 16);
    let mut widths = 0;
    let mut prior_end = 0;
    for dependency in admitted.dependencies() {
        assert!(dependency.span().start() >= prior_end);
        prior_end = dependency.span().end();
        let declaration = dependency.declaration();
        if let Source::LengthInput {
            declaration: actual,
            ..
        } = declaration.source()
        {
            widths += 1;
            assert!(std::ptr::eq(actual, &record));
            assert_eq!(
                source.get(dependency.span().start()..dependency.span().end()),
                Some("body_width")
            );
        }
    }
    assert_eq!(widths, 3);
    assert_eq!(
        proof
            .admit_execution(Limits::new(6, 15))
            .expect_err("no name deduplication or branch discount")
            .refusal(),
        Refusal::WorkspaceRecords {
            allowed: 15,
            required: 16
        }
    );
    assert!(
        matches!(record.state(), LengthState::Unknown { observation }
        if *observation == EntityId::from_bits(3))
    );
    for debug in [format!("{admitted:?}"), format!("{:?}", proof.root())] {
        assert!(!debug.contains("body_width"));
    }
}

#[test]
fn maximum_legal_arena_reserves_all_name_occurrences_including_full_call_arguments() {
    let source = format!("min({})", vec!["eps_num"; 255].join(","));
    let expression = FormulaExpression::parse(&source)
        .expect("full legal arena")
        .normalize_literals()
        .expect("inputs");
    let proof = expression
        .check_kinds(&FormulaNamespace::new([]).expect("namespace"))
        .expect("all arguments checked");
    assert_eq!(expression.node_count(), 256);
    assert_eq!(proof.node_count(), 256);
    assert_eq!(proof.dependencies().len(), 255);
    assert_eq!(
        proof
            .admit_execution(Limits::new(255, usize::MAX))
            .expect_err("syntax bound does not waive explicit quota")
            .refusal(),
        Refusal::Nodes {
            allowed: 255,
            actual: 256
        }
    );
    assert_eq!(
        proof
            .admit_execution(Limits::new(256, 766))
            .expect_err("complete reservation is 767")
            .refusal(),
        Refusal::WorkspaceRecords {
            allowed: 766,
            required: 767
        }
    );
    let admitted = proof.admit_execution(Limits::new(256, 767)).expect("exact");
    assert_eq!(admitted.workspace_records(), 767);
    let Form::Call { arguments, .. } = admitted.root().form() else {
        unreachable!("actual call fixture");
    };
    assert_eq!(arguments.len(), 255);
    for (argument, dependency) in arguments.iter().zip(admitted.dependencies()) {
        assert_eq!(argument.span(), dependency.span());
        assert!(std::ptr::eq(argument.expression(), &expression));
    }
}

#[test]
fn actual_whole_recipe_expression_keeps_prior_let_metadata_and_global_spans() {
    let source = "let first:length=1 mm\nlet second:length=first+first";
    let recipe = FormulaRecipe::parse(source)
        .expect("recipe")
        .normalize_literals()
        .expect("inputs");
    let proof = recipe
        .check_kinds(FormulaNamespace::new([]).expect("namespace"))
        .expect("complete static recipe");
    let second = proof.statements().get(1).expect("actual second statement");
    let Statement::Let { expression } = second.kind() else {
        unreachable!("actual let fixture");
    };
    let admitted = expression
        .admit_execution(Limits::new(3, 8))
        .expect("actual expression only");
    assert!(std::ptr::eq(
        admitted.root().expression(),
        expression.expression()
    ));
    assert_eq!(admitted.workspace_records(), 8);
    let span = admitted.root().span();
    assert_eq!(source.get(span.start()..span.end()), Some("first+first"));
    for dependency in admitted.dependencies() {
        assert!(dependency.span().start() > 20);
        assert!(matches!(
            dependency.declaration().source(),
            Source::Recipe {
                statement_index: 1,
                ..
            }
        ));
    }
}
