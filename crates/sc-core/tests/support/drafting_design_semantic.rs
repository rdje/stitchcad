//! Fresh actual Root/provider scope,original member loans and exact first failures.
use super::{
    limits, root_with_host_and_semantic, scope,
    semantic_root::{canonical, recipe},
    EmptyHost, Source, TestResult,
};
use sc_core::{construction::*, design::*, ontology::*, recipe::*, value::*};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn semantic_root() -> TestResult<DesignInputs<EmptyHost>> {
    root_with_host_and_semantic(
        "",
        vec![],
        EmptyHost,
        canonical()?,
        vec![],
        DesignSourceReferences {
            measurement_table: None,
            ease_set: None,
            size_set: None,
        },
        recipe(70)?,
    )
}
#[test]
fn fresh_design_semantic_scope_preserves_actual_root_snapshot_members_and_authored_state(
) -> TestResult {
    let root = semantic_root()?;
    let other = semantic_root()?;
    let source = Source::new(&root);
    let context = scope(&root)?
        .bind_drafting_context(&source, limits(1000, 1000, 1000)?)
        .map_err(|e| e.to_string())?;
    let text = 32 + source.frame.data().label.len() + source.snapshot.data().label.len();
    let policy = limits(11, text, 6)?;
    source.trace.set(0);
    let sources = context
        .prepare_design_semantic_sources(policy)
        .map_err(|e| e.to_string())?;
    assert_eq!(source.trace.get(), 2);
    assert_eq!(sources.limits(), policy);
    let demand = sources.demand();
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (11, text, 6)
    );
    assert!(std::ptr::eq(sources.context().scope().root(), &root));
    assert!(sources
        .context()
        .snapshot()
        .is_same_record(context.snapshot()));
    source.trace.set(0);
    let logical = sources
        .logical_source(LogicalInputTarget {
            declaration: id(90),
            kind: LogicalInputKind::PocketOpening,
        })
        .map_err(|e| e.to_string())?;
    assert!(std::ptr::eq(logical.scope(), &sources));
    assert!(std::ptr::eq(
        logical.binding().inputs(),
        root.inputs().logicals()
    ));
    let original = root
        .inputs()
        .logicals()
        .declaration(id(90))
        .ok_or("actual Root member")?;
    let foreign = other
        .inputs()
        .logicals()
        .declaration(id(90))
        .ok_or("other equal-data member")?;
    assert_eq!(logical.binding().index(), 0);
    assert!(std::ptr::eq(logical.declaration(), original));
    assert!(!std::ptr::eq(logical.declaration(), foreign));
    assert_eq!(logical.declaration(), foreign);
    assert!(
        matches!(logical.declaration().definition().state,LogicalInputState::Derived {operation} if operation==id(80))
    );
    let operation = sources
        .operation_source(id(80), SemanticOperationKind::LogicalDerivation)
        .map_err(|e| e.to_string())?;
    assert!(std::ptr::eq(operation.scope(), &sources));
    assert!(std::ptr::eq(
        operation.binding().recipe(),
        root.semantic_recipe()
    ));
    assert_eq!(operation.binding().index(), 1);
    assert!(std::ptr::eq(
        operation.operation(),
        root.semantic_recipe()
            .operation(id(80))
            .ok_or("actual Root operation")?
    ));
    let copied = logical;
    assert!(std::ptr::eq(copied.declaration(), original));
    let copied = operation;
    assert!(std::ptr::eq(copied.operation(), operation.operation()));
    for target in [
        CanonicalInputTarget::Length(id(10)),
        CanonicalInputTarget::Scalar {
            declaration: id(20),
            kind: ScalarInputKind::Boolean,
        },
    ] {
        let numeric = sources.numeric_source(target).map_err(|e| e.to_string())?;
        assert!(std::ptr::eq(numeric.scope(), &sources));
        assert!(std::ptr::eq(numeric.binding().inputs(), root.inputs()));
        assert!(!std::ptr::eq(numeric.binding().inputs(), other.inputs()));
        assert_eq!(numeric.binding().target(), target);
        match numeric.member() {
            CanonicalNumericInputMember::Length {
                index: 0,
                declaration,
            } => {
                assert!(std::ptr::eq(
                    declaration,
                    root.inputs().length(id(10)).ok_or("actual Root length")?
                ));
                assert!(!std::ptr::eq(
                    declaration,
                    other
                        .inputs()
                        .length(id(10))
                        .ok_or("foreign same-ID length")?
                ));
                assert_eq!(
                    declaration.state(),
                    &LengthState::Unknown {
                        observation: id(1001)
                    }
                );
            }
            CanonicalNumericInputMember::Scalar {
                index: 0,
                declaration,
            } => {
                assert!(std::ptr::eq(
                    declaration,
                    root.inputs().scalar(id(20)).ok_or("actual Root scalar")?
                ));
                assert!(!std::ptr::eq(
                    declaration,
                    other
                        .inputs()
                        .scalar(id(20))
                        .ok_or("foreign same-ID scalar")?
                ));
                assert_eq!(
                    declaration.state(),
                    &ScalarInputState::Unknown {
                        observation: id(1003)
                    }
                );
            }
            _ => return Err("wrong original numeric member/index".into()),
        }
        let copied = numeric;
        assert!(std::ptr::eq(copied.scope(), &sources));
        for private in ["Unknown", "option_a", "private empty"] {
            assert!(!format!("{numeric:?}").contains(private));
        }
    }
    assert_eq!(source.trace.get(), 0);
    for debug in [
        format!("{logical:?}"),
        format!("{operation:?}"),
        format!("{sources:?}"),
    ] {
        for private in ["option_a", "policy_a", "source_a", "private empty"] {
            assert!(!debug.contains(private));
        }
    }
    let mut meter = DesignResourceMeter::new(limits(12, text + 2, 7)?);
    let prior = DesignResourceLocation {
        scope: DesignResourceScope::CommandState,
        owner: Some(id(500)),
        field: "prior",
        index: None,
    };
    meter.charge_entries(1, prior)?;
    meter.charge_text("é", prior)?;
    meter.charge_counter_bits(1, prior)?;
    sources.charge_resources(&mut meter)?;
    let sum = meter.finish()?;
    assert_eq!(
        (
            sum.collection_entries(),
            sum.text_bytes(),
            sum.counter_bits()
        ),
        (12, text + 2, 7)
    );
    let mut meter = DesignResourceMeter::new(limits(1, 1, 1)?);
    let first = meter
        .charge_entries(2, prior)
        .err()
        .ok_or("prior first refusal")?;
    assert_eq!(sources.charge_resources(&mut meter), Err(first));
    assert_eq!(meter.finish(), Err(first));
    Ok(())
}
#[test]
fn design_numeric_sources_keep_exact_scope_target_and_original_lending_failures() -> TestResult {
    let root = semantic_root()?;
    let source = Source::new(&root);
    let context = scope(&root)?
        .bind_drafting_context(&source, limits(1000, 1000, 1000)?)
        .map_err(|e| e.to_string())?;
    let sources = context
        .prepare_design_semantic_sources(limits(1000, 1000, 1000)?)
        .map_err(|e| e.to_string())?;
    source.trace.set(0);
    for (case, target) in [
        CanonicalInputTarget::Length(id(20)),
        CanonicalInputTarget::Scalar {
            declaration: id(10),
            kind: ScalarInputKind::Boolean,
        },
        CanonicalInputTarget::Length(id(90)),
        CanonicalInputTarget::Scalar {
            declaration: id(20),
            kind: ScalarInputKind::Count,
        },
    ]
    .into_iter()
    .enumerate()
    {
        let error = sources
            .numeric_source(target)
            .expect_err("original numeric source refusal");
        assert!(std::ptr::eq(error.scope(), &sources));
        assert_eq!(
            error.request(),
            DraftingDesignSemanticRequest::Numeric(target)
        );
        let DraftingDesignSemanticLookupCause::Numeric(cause) = error.cause() else {
            return Err("numeric cause not retained".into());
        };
        assert!(std::ptr::eq(cause.inputs(), root.inputs()));
        assert_eq!(cause.target(), target);
        match (case, cause.refusal()) {
            (
                0,
                CanonicalNumericInputRefusal::MissingLength {
                    other: Some(CanonicalInputPosition::Scalar { index: 0 }),
                },
            )
            | (
                1,
                CanonicalNumericInputRefusal::MissingScalar {
                    other: Some(CanonicalInputPosition::Length { index: 0 }),
                },
            )
            | (
                2,
                CanonicalNumericInputRefusal::MissingLength {
                    other: Some(CanonicalInputPosition::Logical { index: 0 }),
                },
            ) => {}
            (
                3,
                CanonicalNumericInputRefusal::ScalarKind {
                    expected: ScalarInputKind::Count,
                    index: 0,
                    actual,
                },
            ) => {
                assert!(std::ptr::eq(
                    actual,
                    root.inputs()
                        .scalar(id(20))
                        .ok_or("actual mismatched Root scalar")?
                ));
                assert_eq!(actual.kind(), ScalarInputKind::Boolean);
            }
            _ => return Err(format!("wrong original numeric cause: {error}").into()),
        }
        for private in ["option_a", "Unknown", "private empty"] {
            assert!(!format!("{error:?} {error}").contains(private));
        }
    }
    assert_eq!(source.trace.get(), 0);
    Ok(())
}
#[test]
fn design_semantic_scope_fresh_resource_refusals_precede_lookup_and_preserve_ignored_sink_errors(
) -> TestResult {
    let root = semantic_root()?;
    let source = Source::new(&root);
    let context = scope(&root)?
        .bind_drafting_context(&source, limits(1000, 1000, 1000)?)
        .map_err(|e| e.to_string())?;
    let text = 32 + source.frame.data().label.len() + source.snapshot.data().label.len();
    let location = |scope, owner, field, index| DesignResourceLocation {
        scope,
        owner,
        field,
        index,
    };
    for (policy, trace, expected) in [
        (
            limits(7, 1000, 1000)?,
            0,
            DesignResourceLimitError::Count {
                unit: DesignResourceCount::CollectionEntries,
                maximum: 7,
                consumed: 7,
                requested: 1,
                location: location(
                    DesignResourceScope::Inputs,
                    Some(id(80)),
                    "semantic.logical.inputs",
                    None,
                ),
            },
        ),
        (
            limits(10, 1000, 1000)?,
            2,
            DesignResourceLimitError::Count {
                unit: DesignResourceCount::CollectionEntries,
                maximum: 10,
                consumed: 8,
                requested: 3,
                location: source.location("drafting.cache"),
            },
        ),
        (
            limits(11, text - 1, 6)?,
            2,
            DesignResourceLimitError::Count {
                unit: DesignResourceCount::TextBytes,
                maximum: text - 1,
                consumed: 32 + source.frame.data().label.len(),
                requested: source.snapshot.data().label.len(),
                location: source.location("drafting.snapshot.label"),
            },
        ),
        (
            limits(11, text, 5)?,
            2,
            DesignResourceLimitError::CounterBits {
                maximum: 5,
                consumed: 3,
                requested: 3,
                location: source.location("drafting.snapshot.revision"),
            },
        ),
    ] {
        source.trace.set(0);
        let error = context
            .prepare_design_semantic_sources(policy)
            .err()
            .ok_or("fresh cumulative source refusal")?;
        assert_eq!(*error.cause(), expected);
        assert_eq!(error.limits(), policy);
        assert!(std::ptr::eq(error.context().scope().root(), &root));
        assert!(error
            .context()
            .snapshot()
            .is_same_record(context.snapshot()));
        assert_eq!(source.trace.get(), trace);
        assert!(
            std::error::Error::source(&error).is_some_and(|e| e.is::<DesignResourceLimitError>())
        );
    }
    let mut source = Source::new(&root);
    source.ignore_refusal = true;
    let context = scope(&root)?
        .bind_drafting_context(&source, limits(1000, 1000, 1000)?)
        .map_err(|e| e.to_string())?;
    source.trace.set(0);
    let error = context
        .prepare_design_semantic_sources(limits(10, 1000, 1000)?)
        .err()
        .ok_or("ignored source failure retained")?;
    assert_eq!(
        *error.cause(),
        DesignResourceLimitError::Count {
            unit: DesignResourceCount::CollectionEntries,
            maximum: 10,
            consumed: 8,
            requested: 3,
            location: source.location("drafting.cache")
        }
    );
    assert_eq!(source.trace.get(), 2);
    Ok(())
}
#[test]
fn design_semantic_lookup_errors_keep_original_request_scope_and_lending_member_causes(
) -> TestResult {
    let root = semantic_root()?;
    let source = Source::new(&root);
    let context = scope(&root)?
        .bind_drafting_context(&source, limits(1000, 1000, 1000)?)
        .map_err(|e| e.to_string())?;
    let sources = context
        .prepare_design_semantic_sources(limits(1000, 1000, 1000)?)
        .map_err(|e| e.to_string())?;
    source.trace.set(0);
    let wrong = LogicalInputTarget {
        declaration: id(90),
        kind: LogicalInputKind::NotionSize,
    };
    let missing = LogicalInputTarget {
        declaration: id(10),
        kind: LogicalInputKind::PocketOpening,
    };
    for (error, request) in [
        (
            sources
                .logical_source(wrong)
                .err()
                .ok_or("original logical kind")?,
            DraftingDesignSemanticRequest::Logical(wrong),
        ),
        (
            sources
                .logical_source(missing)
                .err()
                .ok_or("numeric record isn't logical membership")?,
            DraftingDesignSemanticRequest::Logical(missing),
        ),
        (
            sources
                .operation_source(id(70), SemanticOperationKind::LogicalDerivation)
                .err()
                .ok_or("original operation kind")?,
            DraftingDesignSemanticRequest::Operation {
                operation: id(70),
                kind: SemanticOperationKind::LogicalDerivation,
            },
        ),
        (
            sources
                .operation_source(id(90), SemanticOperationKind::Buttonhole)
                .err()
                .ok_or("logical declaration isn't an operation")?,
            DraftingDesignSemanticRequest::Operation {
                operation: id(90),
                kind: SemanticOperationKind::Buttonhole,
            },
        ),
    ] {
        assert!(std::ptr::eq(error.scope(), &sources));
        assert_eq!(error.request(), request);
        match (request, error.cause()) {
            (
                DraftingDesignSemanticRequest::Logical(target),
                DraftingDesignSemanticLookupCause::Logical(LogicalInputLookupError::Kind {
                    expected,
                    actual,
                }),
            ) => {
                assert_eq!(expected, target.kind);
                assert!(std::ptr::eq(
                    actual,
                    root.inputs()
                        .logicals()
                        .declaration(id(90))
                        .ok_or("actual logical cause")?
                ));
            }
            (
                DraftingDesignSemanticRequest::Logical(target),
                DraftingDesignSemanticLookupCause::Logical(LogicalInputLookupError::Missing {
                    declaration,
                    expected,
                }),
            ) => {
                assert_eq!((declaration, expected), (target.declaration, target.kind));
            }
            (
                DraftingDesignSemanticRequest::Operation { kind, .. },
                DraftingDesignSemanticLookupCause::Operation(SemanticOperationLookupError::Kind {
                    expected,
                    actual,
                }),
            ) => {
                assert_eq!(expected, kind);
                assert!(std::ptr::eq(
                    actual,
                    root.semantic_recipe()
                        .operation(id(70))
                        .ok_or("actual operation cause")?
                ));
            }
            (
                DraftingDesignSemanticRequest::Operation { operation, kind },
                DraftingDesignSemanticLookupCause::Operation(
                    SemanticOperationLookupError::Missing {
                        operation: actual,
                        expected,
                    },
                ),
            ) => {
                assert_eq!((actual, expected), (operation, kind));
            }
            other => return Err(format!("original typed cause:{other:?}").into()),
        }
        assert!(!format!("{error:?}").contains("option_a"));
    }
    assert_eq!(source.trace.get(), 0);
    Ok(())
}
