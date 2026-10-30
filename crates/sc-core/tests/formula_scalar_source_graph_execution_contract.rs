//! Actual bounded lazy source execution,earned unbound cache and original read/failure witnesses.
#![allow(clippy::expect_used)]
use sc_core::{name::MachineToken, ontology::EntityId, recipe::*, value::*};
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn length(target: u128, state: LengthState) -> LengthDeclaration {
    LengthDeclaration::new(LengthDeclarationDefinition {
        id: id(target),
        source: id(4000),
        state,
    })
    .expect("actual length")
}
fn scalar(target: u128, kind: ScalarInputKind, formula: u128) -> ScalarInputDeclaration {
    ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
        id: id(target),
        source: id(4001),
        kind,
        state: ScalarInputState::Derived {
            formula: id(formula),
        },
    })
    .expect("actual scalar")
}
struct Fixture {
    inputs: CanonicalInputs,
    sources: FormulaSources,
    names: Vec<(MachineToken, EntityId)>,
}
impl Fixture {
    fn new(inputs: CanonicalInputs, rows: Vec<(u128, String)>, names: Vec<(String, u128)>) -> Self {
        let sources: Vec<_> = rows
            .into_iter()
            .map(|(identity, body)| {
                FormulaSource::new(
                    id(identity),
                    FormulaExpressionDocument::new(body).expect("actual original body"),
                )
            })
            .collect();
        let limits = FormulaSourceLimits::new(
            sources.len(),
            sources.iter().map(|s| s.expression().source().len()).sum(),
            sources
                .iter()
                .map(|s| s.expression().canonical().as_str().len())
                .sum(),
        );
        Self {
            inputs,
            sources: FormulaSources::new(sources, limits).expect("complete sources"),
            names: names
                .into_iter()
                .map(|(name, target)| (MachineToken::new(name).expect("actual name"), id(target)))
                .collect(),
        }
    }
    fn with_plan(&self, inspect: impl FnOnce(FormulaScalarSourceGraphPlan<'_>)) {
        let s = self.sources.sources().len();
        let n = self
            .sources
            .sources()
            .iter()
            .map(|s| s.expression().node_count())
            .sum();
        let normalized = FormulaSourceInputs::new(&self.inputs, &self.sources)
            .expect("actual source joins")
            .normalize_sources(FormulaSourceNormalizationLimits::new(s, n, s + 2 * n))
            .expect("all original arenas");
        let namespace = FormulaNamespace::new(self.names.iter().enumerate().map(
            |(index, (name, target))| {
                let metadata = if let Some(record) = self.inputs.length(*target) {
                    FormulaDeclaration::length_input(
                        name,
                        FormulaInputOrigin::Parameter,
                        id(5000 + index as u128),
                        record,
                    )
                } else {
                    let record = self.inputs.scalar(*target).expect("actual general target");
                    FormulaDeclaration::input(
                        name,
                        FormulaScalarInputOrigin::Parameter,
                        id(5000 + index as u128),
                        *target,
                        record.kind().into(),
                    )
                };
                FormulaInitialDeclaration::try_from(metadata).expect("actual initial metadata")
            },
        ))
        .expect("actual initial namespace");
        let checked = normalized
            .check_sources(
                &namespace,
                FormulaSourceCheckLimits::new(s, n, usize::MAX, s + 2 * n),
            )
            .expect("complete source proof");
        let d = checked
            .sources()
            .iter()
            .map(|s| s.proof().dependencies().len())
            .sum();
        let deps = checked
            .prepare_dependencies(FormulaSourceDependencyLimits::new(s, d, s + d + 1))
            .expect("complete actual relation");
        let graph = deps
            .admit_acyclic(FormulaSourceCycleLimits::new(s, 2 * s))
            .expect("actual complete acyclic proof");
        let plan = graph
            .prepare_scalar_sources(FormulaSourceGraphExecutionLimits::new(
                s,
                n,
                d,
                2 * s + 2 * n + d,
                s,
            ))
            .expect("complete prepared sources");
        inspect(plan);
    }
}
fn context() -> FormulaScalarExecutionContext<'static> {
    FormulaScalarExecutionContext::new(
        FormulaToleranceContext::empty(),
        FormulaSizeContext::empty(),
    )
}
fn rows(rows: &[(u128, &str)]) -> Vec<(u128, String)> {
    rows.iter()
        .map(|(id, body)| (*id, (*body).into()))
        .collect()
}
fn names(rows: &[(&str, u128)]) -> Vec<(String, u128)> {
    rows.iter()
        .map(|(name, id)| ((*name).into(), *id))
        .collect()
}
fn rational(value: FormulaEvaluatedScalar, n: u128, d: u128) {
    assert_eq!(
        value.numeric().expect("actual numeric").rational(),
        FormulaRational::new(n, d, false).expect("exact rational")
    );
}
#[test]
fn shared_actual_suppliers_finish_once_and_repeated_mixed_family_reads_keep_original_sources() {
    let fixture = Fixture::new(
        CanonicalInputs::new(
            vec![
                length(10, LengthState::Derived { formula: id(9001) }),
                length(
                    12,
                    LengthState::Unknown {
                        observation: id(42),
                    },
                ),
            ],
            vec![scalar(11, ScalarInputKind::Length, 9003)],
        )
        .expect("actual inventory"),
        rows(&[
            (9004, "a+b+a"),
            (9003, "a*6"),
            (9002, "eps_phys"),
            (9001, "1 mm/3"),
            (9005, "missing"),
        ]),
        names(&[("a", 10), ("b", 11), ("missing", 12)]),
    );
    let before = fixture.inputs.clone();
    fixture.with_plan(|plan| {
        let graph = plan.graph();
        let limits = plan.limits();
        let result = plan
            .evaluate_source(id(9004), context())
            .expect("only actual demanded sources");
        assert!(std::ptr::eq(result.plan().graph(), graph));
        assert_eq!(result.plan().limits(), limits);
        assert_eq!(result.entry().source().normalized().source().id(), id(9004));
        rational(result.entry().value(), 8000, 3);
        assert!(result.entry().value().sources().is_empty());
        let completed: Vec<_> = result
            .sources()
            .map(|s| {
                (
                    s.source().normalized().source().id(),
                    s.completion_ordinal(),
                )
            })
            .collect();
        assert_eq!(completed, vec![(id(9004), 3), (id(9003), 2), (id(9001), 1)]);
        assert!(result.source(id(9002)).is_none() && result.source(id(9005)).is_none());
        assert!(result.source(id(10)).is_none() && result.source(id(5000)).is_none());
        rational(
            result
                .source(id(9001))
                .expect("actual shared supplier")
                .value(),
            1000,
            3,
        );
        rational(
            result
                .source(id(9003))
                .expect("actual second supplier")
                .value(),
            2000,
            1,
        );
        let reads: Vec<_> = result.entry().reads().collect();
        assert_eq!(reads.len(), 3);
        for (read, supplier_id) in reads.iter().zip([9001, 9003, 9001]) {
            let FormulaScalarSourceGraphReadKind::Derived(witness) = read.source() else {
                unreachable!("actual Derived read");
            };
            assert!(std::ptr::eq(witness.input().inputs(), &fixture.inputs));
            assert!(std::ptr::eq(
                witness.supplier(),
                graph
                    .dependencies()
                    .checked()
                    .source(id(supplier_id))
                    .expect("actual original supplier proof")
            ));
            assert_eq!(
                read.value(),
                result
                    .source(id(supplier_id))
                    .expect("actual completed root")
                    .value()
            );
            assert!(std::ptr::eq(
                read.node().expression(),
                result.entry().source().proof().expression()
            ));
        }
        assert_ne!(
            reads.first().expect("first a").node().span(),
            reads.last().expect("repeated a").node().span()
        );
        let FormulaScalarSourceGraphReadKind::Derived(first) =
            reads.first().expect("first").source()
        else {
            unreachable!();
        };
        let FormulaCanonicalInputAdapter::Length(actual) = first.input().adapter() else {
            unreachable!();
        };
        assert!(std::ptr::eq(
            actual.canonical_declaration(),
            fixture.inputs.length(id(10)).expect("actual length")
        ));
        let FormulaScalarSourceGraphReadKind::Derived(second) = reads.get(1).expect("b").source()
        else {
            unreachable!();
        };
        let FormulaCanonicalInputAdapter::Scalar(actual) = second.input().adapter() else {
            unreachable!();
        };
        assert!(std::ptr::eq(
            actual.canonical_declaration(),
            fixture
                .inputs
                .scalar(id(11))
                .expect("actual general Length")
        ));
    });
    assert_eq!(fixture.inputs, before);
}
#[test]
fn actual_false_derived_condition_keeps_call_contributions_and_unused_count_stays_unexecuted() {
    let fixture = Fixture::new(
        CanonicalInputs::new(
            vec![],
            vec![
                scalar(10, ScalarInputKind::Count, 9001),
                scalar(11, ScalarInputKind::Boolean, 9002),
            ],
        )
        .expect("actual general owners"),
        rows(&[
            (9003, "if(flag,n+n,3/2.0)"),
            (9001, "1/2.0"),
            (9002, "cos(0 deg)==0.0"),
        ]),
        names(&[("n", 10), ("flag", 11)]),
    );
    let before = fixture.inputs.clone();
    fixture.with_plan(|plan| {
        let result = plan
            .evaluate_source(id(9003), context())
            .expect("actual false branch");
        rational(result.entry().value(), 3, 2);
        assert_eq!(
            result.entry().value().sources().iter().collect::<Vec<_>>(),
            vec![FormulaBuiltin::Cos]
        );
        assert!(result.source(id(9001)).is_none());
        let condition = result.source(id(9002)).expect("actual computed Boolean");
        assert_eq!(condition.value().boolean(), Some(false));
        assert_eq!(condition.completion_ordinal(), 1);
        let read = result
            .entry()
            .reads()
            .next()
            .expect("actual condition read");
        assert_eq!(read.value(), condition.value());
        assert_eq!(result.entry().reads().count(), 1);
    });
    fixture.with_plan(|plan| {
        let result = plan
            .evaluate_source(id(9001), context())
            .expect("bare Count root,no storage");
        rational(result.entry().value(), 1, 2);
        assert_eq!(result.sources().count(), 1);
        assert!(result.entry().reads().next().is_none());
    });
    assert_eq!(fixture.inputs, before);
}
#[test]
fn direct_reads_and_nested_supplier_failures_keep_actual_nodes_and_canonical_requirements() {
    for body in ["eps_num+1 mm/3", "eps_phys", "missing", "1 mm/0"] {
        let fixture = Fixture::new(
            CanonicalInputs::new(
                vec![
                    length(10, LengthState::Derived { formula: id(9001) }),
                    length(
                        11,
                        LengthState::Unknown {
                            observation: id(42),
                        },
                    ),
                ],
                vec![],
            )
            .expect("actual records"),
            rows(&[(9002, "a+1 mm"), (9001, body)]),
            names(&[("a", 10), ("missing", 11)]),
        );
        let before = fixture.inputs.clone();
        fixture.with_plan(|plan| {
            let graph = plan.graph();
            let execution = plan.evaluate_source(id(9002), context());
            if body == "eps_num+1 mm/3" {
                let result = execution.expect("actual intrinsic read");
                rational(result.entry().value(), 4003, 3);
                let read = result
                    .source(id(9001))
                    .expect("supplier")
                    .reads()
                    .next()
                    .expect("eps_num");
                assert!(matches!(
                    read.source(),
                    FormulaScalarSourceGraphReadKind::Direct(
                        FormulaScalarExecutionReadKind::Tolerance(_)
                    )
                ));
                return;
            }
            let error = execution.expect_err("whole graph refuses without accepted cache/result");
            assert!(std::ptr::eq(error.graph(), graph));
            assert_eq!(error.source_id(), id(9002));
            let FormulaScalarSourceGraphExecutionCause::Evaluation {
                source_index,
                error,
            } = error.cause()
            else {
                unreachable!("original scalar cause");
            };
            assert_eq!(*source_index, 1);
            assert!(std::ptr::eq(
                error.node().expression(),
                graph
                    .dependencies()
                    .checked()
                    .source(id(9001))
                    .expect("actual supplier")
                    .proof()
                    .expression()
            ));
            let FormulaScalarExecutionErrorCause::Evaluation(cause) = error.cause() else {
                unreachable!("selected node failure");
            };
            match body {
                "eps_phys" => assert!(matches!(
                    cause.as_ref(),
                    FormulaScalarExecutionFailure::ToleranceRead(_)
                )),
                "missing" => {
                    let FormulaScalarExecutionFailure::LengthInput(error) = cause.as_ref() else {
                        unreachable!("actual unknown");
                    };
                    assert_eq!(
                        error.reason(),
                        LengthValueError::RequiresObservation {
                            declaration: id(11),
                            observation: id(42)
                        }
                    );
                    assert!(std::ptr::eq(
                        error.input().canonical_declaration(),
                        fixture.inputs.length(id(11)).expect("actual record")
                    ));
                }
                _ => assert!(matches!(
                    cause.as_ref(),
                    FormulaScalarExecutionFailure::Numeric(_)
                )),
            }
        });
        assert_eq!(fixture.inputs, before);
    }
}
#[test]
fn absent_exact_entry_and_empty_graph_refuse_without_identity_fallback() {
    for empty in [false, true] {
        let fixture = Fixture::new(
            CanonicalInputs::new(vec![], vec![]).expect("empty inventory"),
            if empty {
                vec![]
            } else {
                rows(&[(9001, "1 mm")])
            },
            vec![],
        );
        for entry in [10, 5000, 9999] {
            fixture.with_plan(|plan| {
                let graph = plan.graph();
                let limits = plan.limits();
                let error = plan
                    .evaluate_source(id(entry), context())
                    .expect_err("only actual source identity");
                assert!(std::ptr::eq(error.graph(), graph));
                assert_eq!(error.limits(), limits);
                assert_eq!(error.source_id(), id(entry));
                assert!(matches!(
                    error.cause(),
                    FormulaScalarSourceGraphExecutionCause::MissingSource
                ));
            });
        }
    }
}
#[test]
fn actual_512_source_chain_executes_iteratively_on_a_small_requested_thread_stack() {
    const S: usize = 512;
    let inputs = CanonicalInputs::new(
        (0..S)
            .map(|i| {
                length(
                    1000 + i as u128,
                    LengthState::Derived {
                        formula: id(9000 + i as u128),
                    },
                )
            })
            .collect(),
        vec![],
    )
    .expect("actual canonical chain");
    let fixture = Fixture::new(
        inputs,
        (0..S)
            .map(|i| {
                (
                    9000 + i as u128,
                    if i == 0 {
                        "1 mm/3".into()
                    } else {
                        format!("v{}*1", i - 1)
                    },
                )
            })
            .collect(),
        (0..S)
            .map(|i| (format!("v{i}"), 1000 + i as u128))
            .collect(),
    );
    fixture.with_plan(|plan| {
        std::thread::scope(|scope| {
            std::thread::Builder::new()
                .stack_size(64 * 1024)
                .spawn_scoped(scope, move || {
                    let result = plan
                        .evaluate_source(id(9000 + (S - 1) as u128), context())
                        .expect("actual iterative chain");
                    assert_eq!(result.sources().count(), S);
                    rational(result.entry().value(), 1000, 3);
                    for (index, source) in result.sources().enumerate() {
                        assert_eq!(source.completion_ordinal(), index + 1);
                        rational(source.value(), 1000, 3);
                        assert_eq!(source.reads().count(), usize::from(index != 0));
                    }
                })
                .expect("requested small stack thread")
                .join()
                .expect("actual worker completed");
        });
    });
}

#[test]
fn actual_derived_within_operands_keep_direct_class_and_symbolic_approximation_guard() {
    for supplier in ["1 mm", "1 mm+cos(0 deg)*0 mm"] {
        let fixture = Fixture::new(
            CanonicalInputs::new(
                vec![length(10, LengthState::Derived { formula: id(9001) })],
                vec![],
            )
            .expect("actual target"),
            rows(&[
                (9002, "if(within(a,1 mm,eps_num),2 mm,eps_phys)"),
                (9001, supplier),
            ]),
            names(&[("a", 10)]),
        );
        fixture.with_plan(|plan| {
            let execution = plan.evaluate_source(id(9002), context());
            if supplier == "1 mm" {
                let result = execution.expect("actual T1 class,untaken physical stays unread");
                rational(result.entry().value(), 2000, 1);
                let reads: Vec<_> = result.entry().reads().collect();
                assert_eq!(reads.len(), 2);
                assert!(matches!(
                    reads.first().expect("actual a").source(),
                    FormulaScalarSourceGraphReadKind::Derived(_)
                ));
                assert!(matches!(
                    reads.last().expect("actual class").source(),
                    FormulaScalarSourceGraphReadKind::Direct(
                        FormulaScalarExecutionReadKind::Tolerance(_)
                    )
                ));
            } else {
                let error = execution
                    .expect_err("actual cos contribution forbids T1 despite exact zero product");
                let FormulaScalarSourceGraphExecutionCause::Evaluation {
                    source_index,
                    error,
                } = error.cause()
                else {
                    unreachable!();
                };
                assert_eq!(*source_index, 0);
                assert!(matches!(
                    error.node().form(),
                    FormulaCheckedNodeForm::Call {
                        builtin: FormulaBuiltin::Within,
                        ..
                    }
                ));
                assert!(
                    matches!(error.cause(),FormulaScalarExecutionErrorCause::Evaluation(failure)
                    if matches!(failure.as_ref(),FormulaScalarExecutionFailure::Comparison(_)))
                );
            }
        });
    }
}

#[path = "support/formula_scalar_source_graph_execution/oracle.rs"]
mod independent_source_oracle;
