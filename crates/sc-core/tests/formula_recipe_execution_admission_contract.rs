//! Whole admission uses complete actual owners, not supplied populations or runtime annotations.
#![allow(clippy::expect_used)]
use sc_core::{
    name::MachineToken,
    ontology::EntityId,
    recipe::{
        FormulaDeclaration, FormulaDeclarationSource as Source, FormulaInitialDeclaration,
        FormulaInputOrigin, FormulaNamespace, FormulaRecipe,
        FormulaRecipeExecutionAdmissionRefusal as Refusal, FormulaRecipeExecutionLimits as Limits,
    },
    value::{LengthDeclaration, LengthDeclarationDefinition, LengthState},
};

#[test]
fn complete_counts_keep_actual_unknown_record_headers_prior_lets_and_global_ordinals() {
    let name = MachineToken::new("body_width").expect("name");
    let record = LengthDeclaration::new(LengthDeclarationDefinition {
        id: EntityId::from_bits(1),
        source: EntityId::from_bits(2),
        state: LengthState::Unknown {
            observation: EntityId::from_bits(3),
        },
    })
    .expect("actual unknown");
    let original = record.clone();
    let declaration = FormulaDeclaration::length_input(
        &name,
        FormulaInputOrigin::Measurement,
        EntityId::from_bits(4),
        &record,
    );
    let namespace =
        FormulaNamespace::new([FormulaInitialDeclaration::try_from(declaration).expect("initial")])
            .expect("namespace");
    let source = "let first:length=if(is_base_size,body_width+body_width,body_width)\nassert check:eps_phys=first==body_width\nlet zero:length=first-first";
    let recipe = FormulaRecipe::parse(source)
        .expect("syntax")
        .normalize_literals()
        .expect("literals");
    let proof = recipe
        .check_kinds(namespace)
        .expect("complete static proof");
    let limits = Limits::new(3, 11, 31, 3);
    let admission = proof
        .admit_execution(limits)
        .expect("unread unknown and physical context");
    assert!(std::ptr::eq(admission.proof(), &proof));
    assert!(std::ptr::eq(admission.proof().recipe(), &recipe));
    assert_eq!(admission.limits(), limits);
    assert_eq!(
        (
            limits.statements(),
            limits.nodes(),
            limits.workspace_records(),
            limits.output_records()
        ),
        (3, 11, 31, 3)
    );
    assert_eq!(
        (
            admission.statement_count(),
            admission.binding_count(),
            admission.assertion_count()
        ),
        (3, 2, 1)
    );
    assert_eq!(
        (
            admission.node_count(),
            admission.name_use_count(),
            admission.workspace_records(),
            admission.output_records()
        ),
        (11, 9, 31, 3)
    );
    let mut actual_width_uses = 0;
    for dependency in admission.proof().dependencies() {
        let span = dependency.span();
        assert_eq!(
            source.get(span.start()..span.end()),
            Some(dependency.declaration().name())
        );
        match dependency.declaration().source() {
            Source::LengthInput { declaration, .. } => {
                actual_width_uses += 1;
                assert!(std::ptr::eq(declaration, &record));
            }
            Source::Recipe {
                statement_index, ..
            } => {
                assert_eq!(statement_index, 1);
                assert!(dependency.statement_index() > statement_index);
            }
            _ => {}
        }
    }
    assert_eq!(actual_width_uses, 4);
    for (position, statement) in admission.proof().statements().iter().enumerate() {
        assert_eq!(statement.statement_index(), position + 1);
        assert!(std::ptr::eq(
            statement.statement(),
            recipe
                .statements()
                .get(position)
                .expect("actual original statement")
        ));
    }
    assert_eq!(record, original);
    assert!(!format!("{admission:?}").contains("body_width"));
}

#[test]
fn independent_allowances_refuse_complete_population_in_documented_priority() {
    let recipe =
        FormulaRecipe::parse("let a:length=eps_num+eps_num\nassert check:eps_phys=a==eps_num")
            .expect("syntax")
            .normalize_literals()
            .expect("literals");
    let proof = recipe
        .check_kinds(FormulaNamespace::new([]).expect("namespace"))
        .expect("whole proof");
    // S=2, N=3+1+1=5, D=2+(header+left+right)=5, W=15, O=2.
    for (limits, wanted) in [
        (
            Limits::new(1, 0, 0, 0),
            Refusal::Statements {
                allowed: 1,
                actual: 2,
            },
        ),
        (
            Limits::new(2, 4, 0, 0),
            Refusal::Nodes {
                allowed: 4,
                actual: 5,
            },
        ),
        (
            Limits::new(2, 5, 0, 1),
            Refusal::OutputRecords {
                allowed: 1,
                required: 2,
            },
        ),
        (
            Limits::new(2, 5, 14, 2),
            Refusal::WorkspaceRecords {
                allowed: 14,
                required: 15,
            },
        ),
    ] {
        let error = proof
            .admit_execution(limits)
            .expect_err("independent complete quota");
        assert_eq!(error.refusal(), wanted);
        assert_eq!(error.limits(), limits);
        assert!(std::ptr::eq(error.proof(), &proof));
        assert!(!format!("{error:?} {error}").contains("check"));
    }
    let admitted = proof
        .admit_execution(Limits::new(2, 5, 15, 2))
        .expect("exact");
    assert_eq!(
        (
            admitted.node_count(),
            admitted.name_use_count(),
            admitted.workspace_records()
        ),
        (5, 5, 15)
    );
    let equal_recipe = recipe.clone();
    let equal_proof = equal_recipe
        .check_kinds(FormulaNamespace::new([]).expect("namespace"))
        .expect("same text");
    assert!(!std::ptr::eq(admitted.proof(), &equal_proof));
    assert_eq!(
        admitted.proof().canonical_recipe(),
        equal_proof.canonical_recipe()
    );
}

#[test]
fn empty_and_maximum_statement_population_have_exact_output_and_header_reservations() {
    let empty = FormulaRecipe::parse("")
        .expect("empty syntax")
        .normalize_literals()
        .expect("inputs");
    let proof = empty
        .check_kinds(FormulaNamespace::new([]).expect("namespace"))
        .expect("proof");
    let admission = proof
        .admit_execution(Limits::new(0, 0, 0, 0))
        .expect("zero empty capacities");
    assert_eq!(
        (
            admission.statement_count(),
            admission.binding_count(),
            admission.assertion_count(),
            admission.node_count(),
            admission.name_use_count(),
            admission.workspace_records(),
            admission.output_records()
        ),
        (0, 0, 0, 0, 0, 0, 0)
    );
    let source = (0..4096)
        .map(|n| format!("assert check_{n}:eps_phys=1==1"))
        .collect::<Vec<_>>()
        .join("\n");
    let maximum = FormulaRecipe::parse(&source)
        .expect("4096 actual assertions")
        .normalize_literals()
        .expect("inputs");
    let proof = maximum
        .check_kinds(FormulaNamespace::new([]).expect("namespace"))
        .expect("all statements");
    assert_eq!(
        proof
            .admit_execution(Limits::new(4096, 8192, 20480, 4095))
            .expect_err("all assertion outputs count")
            .refusal(),
        Refusal::OutputRecords {
            allowed: 4095,
            required: 4096
        }
    );
    assert_eq!(
        proof
            .admit_execution(Limits::new(4096, 8192, 20479, 4096))
            .expect_err("every real header needs a source record")
            .refusal(),
        Refusal::WorkspaceRecords {
            allowed: 20479,
            required: 20480
        }
    );
    let admission = proof
        .admit_execution(Limits::new(4096, 8192, 20480, 4096))
        .expect("exact maximum");
    assert_eq!(
        (
            admission.binding_count(),
            admission.assertion_count(),
            admission.node_count(),
            admission.name_use_count(),
            admission.output_records()
        ),
        (0, 4096, 8192, 4096, 4096)
    );
    assert_eq!(
        admission
            .proof()
            .statements()
            .last()
            .expect("actual last")
            .statement_index(),
        4096
    );
}

#[test]
fn static_late_failure_exposes_no_admission_and_runtime_failures_remain_unevaluated() {
    let invalid = FormulaRecipe::parse("let prior:length=1 mm/0\nassert later:eps_phys=1 mm==1.0")
        .expect("syntax")
        .normalize_literals()
        .expect("inputs");
    let error = invalid
        .check_kinds(FormulaNamespace::new([]).expect("namespace"))
        .expect_err("whole static failure precedes runtime division");
    assert_eq!(error.statement_index(), 2);
    assert_eq!(error.token(), "formula_dimension");
    let valid = FormulaRecipe::parse("let prior:length=1 mm/0\nassert later:eps_phys=1==2")
        .expect("syntax")
        .normalize_literals()
        .expect("inputs");
    let proof = valid
        .check_kinds(FormulaNamespace::new([]).expect("namespace"))
        .expect("static only");
    let admission = proof
        .admit_execution(Limits::new(2, 5, 11, 2))
        .expect("no runtime verdict claimed");
    assert_eq!(
        (
            admission.node_count(),
            admission.name_use_count(),
            admission.workspace_records()
        ),
        (5, 1, 11)
    );
}

#[test]
fn complete_populations_match_the_independent_reference_parser_without_evaluation() {
    let root = std::env::current_dir()
        .expect("Cargo package root")
        .join("../..");
    let output = std::process::Command::new("python3")
        .args([
            "-I",
            "-B",
            "docs/tasks/artifacts/formula_structure/recipe_execution_admission_oracle.py",
        ])
        .current_dir(root)
        .output()
        .expect("guarded independent parser");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let rows = String::from_utf8(output.stdout).expect("UTF8 reference rows");
    let mut seen = 0;
    for row in rows.lines() {
        let (source, populations) = row.split_once('\t').expect("source and populations");
        let source = source.replace("\\n", "\n");
        let counts: [usize; 7] = populations
            .split('\t')
            .map(|n| n.parse::<usize>().expect("actual population"))
            .collect::<Vec<_>>()
            .try_into()
            .expect("seven complete populations");
        let [statements, _, _, nodes, _, workspace, outputs] = counts;
        let recipe = FormulaRecipe::parse(&source)
            .expect("reference source")
            .normalize_literals()
            .expect("inputs");
        let proof = recipe
            .check_kinds(FormulaNamespace::new([]).expect("namespace"))
            .expect("static whole recipe");
        let admitted = proof
            .admit_execution(Limits::new(statements, nodes, workspace, outputs))
            .expect("independently measured exact limits");
        assert_eq!(
            [
                admitted.statement_count(),
                admitted.binding_count(),
                admitted.assertion_count(),
                admitted.node_count(),
                admitted.name_use_count(),
                admitted.workspace_records(),
                admitted.output_records()
            ],
            counts,
            "actual independent recipe {source}"
        );
        seen += 1;
    }
    assert_eq!(seen, 4, "finite original reference population");
}
