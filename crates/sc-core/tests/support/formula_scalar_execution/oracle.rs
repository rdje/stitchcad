//! Independent book-reference values/contributions/read order watch actual public AST execution.
use super::{
    context, execute, failure, id, normalized, Cause, Failure, FormulaBindingKind,
    FormulaInitialDeclaration, FormulaInputOrigin, FormulaNamespace, FormulaScalarInputOrigin,
    LengthValueError, D,
};
use sc_core::{
    name::MachineToken,
    recipe::{
        FormulaCheckedNodeForm as Form, FormulaExecutedScalarExpression,
        FormulaScalarExecutionError,
    },
    value::ScalarInputReadError,
};

const FIXTURE: &str = include_str!("../../fixtures/formula_scalar_execution.tsv");

fn outcome(
    result: Result<FormulaExecutedScalarExpression<'_>, FormulaScalarExecutionError<'_>>,
) -> String {
    match result {
        Ok(result) => {
            let value = result.value();
            let (payload, denominator) = if let Some(boolean) = value.boolean() {
                (boolean.to_string(), "-".to_owned())
            } else {
                let value = value.numeric().expect("actual numeric").rational();
                (
                    format!(
                        "{}{}",
                        if value.is_negative() { "-" } else { "" },
                        value.numerator_magnitude()
                    ),
                    value.denominator().to_string(),
                )
            };
            let calls = value
                .sources()
                .iter()
                .map(|call| call.token())
                .collect::<Vec<_>>();
            let reads = result
                .reads()
                .map(|read| {
                    let Form::Name { name, .. } = read.node().form() else {
                        unreachable!("actual read name");
                    };
                    name
                })
                .collect::<Vec<_>>();
            [
                value.kind().token().to_owned(),
                payload,
                denominator,
                if calls.is_empty() {
                    "-".to_owned()
                } else {
                    calls.join(",")
                },
                if reads.is_empty() {
                    "-".to_owned()
                } else {
                    reads.join(",")
                },
            ]
            .join("\t")
        }
        Err(error) => {
            assert!(
                matches!(error.cause(), Cause::Evaluation(_)),
                "no synthetic allocation fixture"
            );
            let token = match failure(&error) {
                Failure::Numeric(cause) => cause.diagnostic_code(),
                Failure::LengthInput(cause) => match cause.reason() {
                    LengthValueError::RequiresObservation { .. } => "formula_unknown",
                    LengthValueError::RequiresEvaluation { .. } => {
                        unreachable!("D338 reference Derived verdict is outside this fixture")
                    }
                },
                Failure::ScalarInput(cause) => match cause.reason() {
                    ScalarInputReadError::RequiresObservation { .. } => "formula_unknown",
                    ScalarInputReadError::RequiresEvaluation { .. } => {
                        unreachable!("D338 reference Derived verdict is outside this fixture")
                    }
                },
                Failure::ToleranceRead(cause) => cause.diagnostic_code(),
                Failure::SizeRead(cause) => cause.diagnostic_code(),
                Failure::Comparison(cause) => cause.diagnostic_code(),
                Failure::WorkspaceReservation { .. } => {
                    unreachable!("valid actual proof fits its admitted logical buffers")
                }
            };
            format!("error\t{token}\t-\t-\t-")
        }
    }
}

#[test]
fn actual_worker_matches_every_independent_reference_value_source_trace_and_refusal_row() {
    let inputs = super::inputs(true);
    let names = [
        "private_width",
        "known_zero",
        "assumed",
        "preferred",
        "use_fixed",
    ]
    .map(|name| MachineToken::new(name).expect("name"));
    let [width, zero, assumed, preferred, flag] = &names;
    let namespace = FormulaNamespace::new(
        [
            D::length_input(
                width,
                FormulaInputOrigin::Measurement,
                id(101),
                inputs.length(id(1)).expect("actual unknown"),
            ),
            D::length_input(
                zero,
                FormulaInputOrigin::Parameter,
                id(104),
                inputs.length(id(4)).expect("actual known zero"),
            ),
            D::length_input(
                assumed,
                FormulaInputOrigin::Parameter,
                id(105),
                inputs.length(id(5)).expect("actual assumption"),
            ),
            D::length_input(
                preferred,
                FormulaInputOrigin::Parameter,
                id(106),
                inputs.length(id(6)).expect("actual preference"),
            ),
            D::input(
                flag,
                FormulaScalarInputOrigin::Parameter,
                id(103),
                id(3),
                FormulaBindingKind::Boolean,
            ),
        ]
        .map(|metadata| FormulaInitialDeclaration::try_from(metadata).expect("initial")),
    )
    .expect("namespace");
    let rows = FIXTURE
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .collect::<Vec<_>>();
    assert!(!rows.is_empty());
    for line in &rows {
        let fields = line.split('\t').collect::<Vec<_>>();
        assert_eq!(fields.len(), 6, "complete frozen row");
        let source = fields.first().copied().expect("source");
        let expression = normalized(source);
        let proof = expression
            .check_kinds(&namespace)
            .expect("both complete branches statically accepted");
        let actual = outcome(execute(&proof, &inputs, context()));
        let expected = fields
            .iter()
            .skip(1)
            .copied()
            .collect::<Vec<_>>()
            .join("\t");
        assert_eq!(actual, expected, "independent actual-AST row: {source}");
    }
    println!(
        "actual scalar/reference: {} exact value/source/read/refusal rows",
        rows.len()
    );
}

#[test]
fn independent_reference_reproduces_the_complete_frozen_scalar_fixture() {
    let root = std::env::current_dir()
        .expect("Cargo package root")
        .join("../..");
    let output = std::process::Command::new("python3")
        .args([
            "-I",
            "-B",
            "docs/tasks/artifacts/formula_structure/scalar_execution_oracle.py",
        ])
        .current_dir(root)
        .output()
        .expect("independent guarded reference oracle");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        output.stdout,
        FIXTURE.as_bytes(),
        "complete byte-exact independent fixture"
    );
}
