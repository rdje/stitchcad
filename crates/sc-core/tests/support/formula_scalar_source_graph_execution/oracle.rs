//! Whole original source collections compared with independent recursive book-reference results.
use super::*;
use sc_units::Length;
use std::collections::BTreeMap;
const FIXTURE: &str = include_str!("../../fixtures/formula_scalar_source_graph_execution.tsv");
fn fixture(declarations: &str, sources: &str) -> Fixture {
    let mut dedicated = vec![];
    let mut general = vec![];
    let mut names = vec![];
    let mut canonical = BTreeMap::new();
    for declaration in declarations.split(';') {
        let [name, family, kind, identity, state, payload]: [&str; 6] = declaration
            .split(':')
            .collect::<Vec<_>>()
            .try_into()
            .expect("complete original declaration");
        let identity: u128 = identity.parse().expect("actual canonical id");
        names.push((name.to_owned(), identity));
        let fact = (family, kind, state, payload);
        if let Some(actual) = canonical.insert(identity, fact) {
            assert_eq!(actual, fact);
            continue;
        }
        if family == "L" {
            assert_eq!(kind, "length");
            let value = || {
                Length::from_micrometres(payload.parse().expect("actual authored Length"))
                    .expect("actual Length domain")
            };
            let state = match state {
                "derived" => LengthState::Derived {
                    formula: id(payload.parse().expect("actual formula id")),
                },
                "unknown" => LengthState::Unknown {
                    observation: id(payload.parse().expect("actual observation")),
                },
                "known" => LengthState::Known {
                    value: value(),
                    evidence: vec![id(7000)],
                },
                "assumed" => LengthState::Assumed {
                    value: value(),
                    assumption: id(7001),
                },
                "preference" => LengthState::Preference {
                    value: value(),
                    provenance: id(7002),
                },
                _ => unreachable!("finite authored state"),
            };
            dedicated.push(length(identity, state));
        } else {
            assert_eq!(family, "S");
            let kind = match kind {
                "length" => ScalarInputKind::Length,
                "angle" => ScalarInputKind::Angle,
                "area" => ScalarInputKind::Area,
                "ratio" => ScalarInputKind::Ratio,
                "count" => ScalarInputKind::Count,
                "boolean" => ScalarInputKind::Boolean,
                _ => unreachable!("finite scalar kind"),
            };
            let state = match state {
                "derived" => ScalarInputState::Derived {
                    formula: id(payload.parse().expect("actual sole formula")),
                },
                "unknown" => ScalarInputState::Unknown {
                    observation: id(payload.parse().expect("actual observation")),
                },
                "preference" => {
                    assert_eq!(kind, ScalarInputKind::Boolean);
                    assert!(matches!(payload, "true" | "false"));
                    ScalarInputState::Preference {
                        value: ScalarInputValue::from_boolean(payload == "true"),
                        provenance: id(7002),
                    }
                }
                _ => unreachable!("finite general scalar authored-state population"),
            };
            general.push(
                ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
                    id: id(identity),
                    source: id(4001),
                    kind,
                    state,
                })
                .expect("actual general owner"),
            );
        }
    }
    Fixture::new(
        CanonicalInputs::new(dedicated, general).expect("actual original inventory"),
        sources
            .split(';')
            .map(|source| {
                let (identity, body) = source.split_once('=').expect("actual original source");
                (identity.parse().expect("actual source id"), body.to_owned())
            })
            .collect(),
        names,
    )
}
fn scalar_text(value: FormulaEvaluatedScalar) -> String {
    let payload = if let Some(value) = value.boolean() {
        value.to_string()
    } else {
        let value = value.numeric().expect("actual numeric").rational();
        format!(
            "{}{}/{}",
            if value.is_negative() { "-" } else { "" },
            value.numerator_magnitude(),
            value.denominator()
        )
    };
    let mut calls = value
        .sources()
        .iter()
        .map(|call| call.token())
        .collect::<Vec<_>>();
    calls.sort_unstable();
    format!(
        "{}:{}:{}",
        value.kind().token(),
        payload,
        if calls.is_empty() {
            "-".to_owned()
        } else {
            calls.join(",")
        }
    )
}
fn canonical_id(joined: FormulaCanonicalInput<'_>) -> EntityId {
    match joined.adapter() {
        FormulaCanonicalInputAdapter::Length(input) => input.canonical_declaration().id(),
        FormulaCanonicalInputAdapter::Scalar(input) => input.canonical_declaration().id(),
    }
}
fn scalar_token(error: &FormulaScalarExecutionError<'_>) -> &'static str {
    let FormulaScalarExecutionErrorCause::Evaluation(cause) = error.cause() else {
        unreachable!("no allocation fault in reference population");
    };
    match cause.as_ref() {
        FormulaScalarExecutionFailure::Numeric(error) => error.diagnostic_code(),
        FormulaScalarExecutionFailure::ToleranceRead(error) => error.diagnostic_code(),
        FormulaScalarExecutionFailure::SizeRead(error) => error.diagnostic_code(),
        FormulaScalarExecutionFailure::Comparison(error) => error.diagnostic_code(),
        FormulaScalarExecutionFailure::LengthInput(error) => {
            assert!(
                matches!(error.reason(), LengthValueError::RequiresObservation { .. }),
                "unexecuted Derived cannot masquerade as Unknown"
            );
            "formula_unknown"
        }
        FormulaScalarExecutionFailure::ScalarInput(error) => {
            assert!(
                matches!(
                    error.reason(),
                    ScalarInputReadError::RequiresObservation { .. }
                ),
                "unexecuted Derived cannot masquerade as Unknown"
            );
            "formula_unknown"
        }
        FormulaScalarExecutionFailure::Tangent(_)
        | FormulaScalarExecutionFailure::MissingTangentPolicy { .. } => {
            unreachable!("this original independent fixture contains no tangent calls")
        }
        FormulaScalarExecutionFailure::WorkspaceReservation { .. } => {
            unreachable!("actual complete allowances fit")
        }
    }
}
fn render(
    execution: Result<
        FormulaExecutedScalarSourceGraph<'_>,
        FormulaScalarSourceGraphExecutionError<'_>,
    >,
) -> String {
    let result = match execution {
        Ok(result) => result,
        Err(error) => {
            let FormulaScalarSourceGraphExecutionCause::Evaluation {
                source_index,
                error: scalar,
            } = error.cause()
            else {
                unreachable!(
                    "finite original entry/populations have no selection/allocation/frame failure"
                );
            };
            let source = error
                .graph()
                .dependencies()
                .checked()
                .sources()
                .get(*source_index)
                .expect("actual failing source");
            return format!(
                "error:{}:{}\t-",
                source.normalized().source().id().as_bits(),
                scalar_token(scalar)
            );
        }
    };
    let mut outcomes = vec!["ok".to_owned()];
    let mut reads = vec![];
    for source in result.sources() {
        let source_id = source.source().normalized().source().id().as_bits();
        outcomes.push(format!(
            "{source_id}@{}:{}",
            source.completion_ordinal(),
            scalar_text(source.value())
        ));
        for read in source.reads() {
            let FormulaCheckedNodeForm::Name { name, .. } = read.node().form() else {
                unreachable!("actual selected name");
            };
            let (target, role) = match read.source() {
                FormulaScalarSourceGraphReadKind::Derived(witness) => (
                    canonical_id(witness.input()).as_bits().to_string(),
                    format!(
                        "derived_{}",
                        witness.supplier().normalized().source().id().as_bits()
                    ),
                ),
                FormulaScalarSourceGraphReadKind::Direct(direct) => {
                    let target = match direct {
                        FormulaScalarExecutionReadKind::Length(read) => read
                            .input()
                            .canonical_declaration()
                            .id()
                            .as_bits()
                            .to_string(),
                        FormulaScalarExecutionReadKind::Scalar(read) => read
                            .input()
                            .canonical_declaration()
                            .id()
                            .as_bits()
                            .to_string(),
                        FormulaScalarExecutionReadKind::Tolerance(_)
                        | FormulaScalarExecutionReadKind::Size(_) => "-".into(),
                    };
                    (target, "direct".into())
                }
            };
            reads.push(format!(
                "{source_id}:{name}:{target}:{role}:{}",
                scalar_text(read.value())
            ));
        }
    }
    format!(
        "{}\t{}",
        outcomes.join(";"),
        if reads.is_empty() {
            "-".to_owned()
        } else {
            reads.join(";")
        }
    )
}
#[test]
fn actual_source_worker_matches_every_independent_root_ordinal_read_and_original_refusal() {
    let mut seen = 0;
    let mut refusals = 0;
    for row in FIXTURE.lines().filter(|row| !row.starts_with('#')) {
        let mut fields = row.splitn(5, '\t');
        let label = fields.next().expect("case");
        let declarations = fields.next().expect("original declarations");
        let sources = fields.next().expect("original source entities");
        let entry = fields
            .next()
            .expect("actual source selection")
            .parse()
            .expect("entry id");
        let expected = fields.next().expect("independent full outcomes/reads");
        let fixture = fixture(declarations, sources);
        let before = fixture.inputs.clone();
        let mut actual = None;
        fixture.with_plan(|plan| actual = Some(render(plan.evaluate_source(id(entry), context()))));
        assert_eq!(
            actual.as_deref(),
            Some(expected),
            "actual complete source case {label}"
        );
        assert_eq!(
            fixture.inputs, before,
            "canonical original state/provenance unchanged: {label}"
        );
        seen += 1;
        refusals += usize::from(expected.starts_with("error:"));
    }
    assert_eq!(
        (seen, refusals),
        (27, 8),
        "complete finite original source population"
    );
    println!("actual source/reference:{seen} complete rows/{refusals} refusals/roots,ordinals,reads,states");
}
#[test]
fn independent_source_reference_reproduces_every_frozen_original_case_and_outcome_byte() {
    let root = std::env::current_dir()
        .expect("Cargo package root")
        .join("../..");
    let output = std::process::Command::new("python3")
        .args([
            "-I",
            "-B",
            "docs/tasks/artifacts/formula_structure/scalar_source_execution_oracle.py",
        ])
        .current_dir(root)
        .output()
        .expect("guarded independent recursive reference");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        output.stdout,
        FIXTURE.as_bytes(),
        "complete original case/outcome/read/refusal fixture bytes"
    );
}
