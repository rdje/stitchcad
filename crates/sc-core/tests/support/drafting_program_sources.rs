//! Existing complete Root source factory under actual extra initial names,without borrowing its map.
use super::{limits, root_with_host, scope, EmptyHost, Fault, Provider, TestResult};
use sc_core::{
    construction::*,
    design::{DesignInputs, DesignSourceReferences},
    ontology::EntityId,
    recipe::{
        FormulaExpressionDocument, FormulaSource, FormulaSourceCheckLimits,
        FormulaSourceCheckRefusal,
    },
    value::*,
};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn root(mode: usize) -> TestResult<DesignInputs<EmptyHost>> {
    let canonical = CanonicalInputs::new(
        vec![LengthDeclaration::new(LengthDeclarationDefinition {
            id: id(101),
            source: id(201),
            state: LengthState::Unknown {
                observation: id(301),
            },
        })?],
        vec![
            ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
                id: id(102),
                source: id(202),
                kind: ScalarInputKind::Count,
                state: ScalarInputState::Unknown {
                    observation: id(302),
                },
            })?,
            ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
                id: id(103),
                source: id(203),
                kind: ScalarInputKind::Count,
                state: ScalarInputState::Derived { formula: id(9003) },
            })?,
        ],
    )?;
    let sources = [
        (
            9003,
            if mode == 2 {
                "private_length"
            } else {
                "private_count+1"
            },
        ),
        (
            9004,
            if mode == 1 {
                "private_missing"
            } else {
                "private_length"
            },
        ),
    ]
    .into_iter()
    .map(|(n, source)| {
        Ok(FormulaSource::new(
            id(n),
            FormulaExpressionDocument::new(source.into())?,
        ))
    })
    .collect::<TestResult<Vec<_>>>()?;
    root_with_host(
        "let untouched:length=1 mm/0",
        sources,
        EmptyHost,
        canonical,
        vec![],
        DesignSourceReferences {
            measurement_table: None,
            ease_set: None,
            size_set: None,
        },
    )
}
#[test]
fn root_sources_keep_actual_extra_metadata_consumers_and_original_causes_after_map_consumption(
) -> TestResult {
    for mode in 0..3 {
        let root = root(mode)?;
        let other = super::root()?;
        let provider = Provider::new(&root, &other, Fault::None)?;
        let context = scope(&root)?
            .bind_drafting_context(&provider, limits(10000, 10000, 10000)?)
            .map_err(|e| e.to_string())?;
        let raw =
            DraftingProgramInputs::new(context, vec![DraftingProgramStep::Scalar { ordinal: 1 }]);
        let pp = DraftingProgramLimits::new(limits(10000, 10000, 10000)?, 100, 1000, 10000);
        let program = raw
            .admit(&pp)
            .map_err(|e| e.to_string())?
            .normalized()
            .map_err(|e| e.to_string())?;
        let names = DraftingProgramNameInputs::new(
            &program,
            vec![provider.initial(0)?, provider.initial(1)?],
            vec![],
        );
        let np = DraftingProgramNameLimits::new(limits(10000, 10000, 10000)?, 64, 128);
        let work = names.admit_name_work(&np).map_err(|e| e.to_string())?;
        let positions = work.bind_output_positions().map_err(|e| e.to_string())?;
        let roles = positions.bind_role_sources().map_err(|e| e.to_string())?;
        let initial = roles.bind_initial_sources().map_err(|e| e.to_string())?;
        let namespace = initial
            .bind_initial_namespace()
            .map_err(|e| e.to_string())?;
        let policy = FormulaSourceCheckLimits::new(2, 100, 1, 1000);
        provider.calls.set(0);
        provider.base.trace.set(0);
        if mode == 0 {
            let sources = namespace
                .check_root_sources(policy)
                .map_err(|e| e.to_string())?;
            assert!(std::ptr::eq(sources.inputs(), &initial));
            assert!(std::ptr::eq(sources.normalized(), program.sources()));
            assert!(std::ptr::eq(
                sources.proof().normalized(),
                program.sources().normalized()
            ));
            assert_eq!(sources.proof().limits(), policy);
            assert_eq!(sources.proof().sources().len(), 2);
            assert_eq!(sources.proof().target_count(), 1);
            assert_eq!(sources.proof().retained_records(), 10);
            let source = sources
                .proof()
                .source(id(9003))
                .ok_or("actual selected source proof")?;
            let [_, count_metadata] = &provider.metadata;
            let count_name = count_metadata.data().name.as_str();
            assert!(source
                .proof()
                .dependencies()
                .iter()
                .any(|dependency| std::ptr::eq(dependency.declaration().name(), count_name)));
            // Source proof owns no map borrow:move the namespace and retain both original proofs.
            let op = DraftingProgramOrderLimits::new(1, 3, 9);
            let order = namespace.check_order(&op).map_err(|e| e.to_string())?;
            assert!(std::ptr::eq(order.inputs(), sources.inputs()));
            assert_eq!(order.steps().len(), 1);
            assert_eq!(sources.proof().sources().len(), 2);
            assert!(matches!(
                root.inputs()
                    .scalar(id(102))
                    .ok_or("Unknown Count")?
                    .state(),
                ScalarInputState::Unknown { .. }
            ));
            assert!(
                matches!(root.inputs().scalar(id(103)).ok_or("Derived consumer")?.state(),ScalarInputState::Derived {formula} if *formula==id(9003))
            );
            assert!(!format!("{sources:?}").contains("private"));
        } else {
            let error = namespace
                .check_root_sources(policy)
                .expect_err("original complete source refusal,no prefix");
            assert!(std::ptr::eq(error.inputs(), &initial));
            assert!(std::ptr::eq(error.normalized(), program.sources()));
            assert_eq!(error.limits(), policy);
            assert!(std::ptr::eq(
                error.cause().normalized(),
                program.sources().normalized()
            ));
            assert!(!format!("{error:?} {error}").contains("private_missing"));
            if mode == 1 {
                assert!(matches!(
                    error.cause().refusal(),
                    FormulaSourceCheckRefusal::Expression {
                        source_index: 1,
                        ..
                    }
                ));
            } else {
                assert!(matches!(
                    error.cause().refusal(),
                    FormulaSourceCheckRefusal::TargetKind {
                        source_index: 0,
                        expected: sc_core::recipe::FormulaKind::Count,
                        actual: sc_core::recipe::FormulaKind::Length,
                        ..
                    }
                ));
            }
        }
        assert_eq!(provider.calls.get(), 0);
        assert_eq!(provider.base.trace.get(), 0);
    }
    Ok(())
}
#[test]
fn root_source_quota_order_uses_complete_existing_populations() -> TestResult {
    let root = root(0)?;
    let other = super::root()?;
    let provider = Provider::new(&root, &other, Fault::None)?;
    let context = scope(&root)?
        .bind_drafting_context(&provider, limits(10000, 10000, 10000)?)
        .map_err(|e| e.to_string())?;
    let raw = DraftingProgramInputs::new(context, vec![DraftingProgramStep::Scalar { ordinal: 1 }]);
    let pp = DraftingProgramLimits::new(limits(10000, 10000, 10000)?, 100, 1000, 10000);
    let program = raw
        .admit(&pp)
        .map_err(|e| e.to_string())?
        .normalized()
        .map_err(|e| e.to_string())?;
    let names = DraftingProgramNameInputs::new(
        &program,
        vec![provider.initial(0)?, provider.initial(1)?],
        vec![],
    );
    let np = DraftingProgramNameLimits::new(limits(10000, 10000, 10000)?, 64, 128);
    let work = names.admit_name_work(&np).map_err(|e| e.to_string())?;
    let positions = work.bind_output_positions().map_err(|e| e.to_string())?;
    let roles = positions.bind_role_sources().map_err(|e| e.to_string())?;
    let initial = roles.bind_initial_sources().map_err(|e| e.to_string())?;
    let namespace = initial
        .bind_initial_namespace()
        .map_err(|e| e.to_string())?;
    provider.calls.set(0);
    provider.base.trace.set(0);
    for (index, policy) in [
        FormulaSourceCheckLimits::new(1, 0, 0, 0),
        FormulaSourceCheckLimits::new(2, 3, 0, 0),
        FormulaSourceCheckLimits::new(2, 4, 0, 0),
        FormulaSourceCheckLimits::new(2, 4, 1, 9),
    ]
    .into_iter()
    .enumerate()
    {
        let error = namespace
            .check_root_sources(policy)
            .expect_err("original complete quota precedence");
        assert_eq!(error.limits(), policy);
        assert!(std::ptr::eq(error.inputs(), &initial));
        assert!(matches!(
            (index, error.cause().refusal()),
            (
                0,
                FormulaSourceCheckRefusal::Sources {
                    allowed: 1,
                    required: 2
                }
            ) | (
                1,
                FormulaSourceCheckRefusal::Nodes {
                    allowed: 3,
                    required: 4
                }
            ) | (
                2,
                FormulaSourceCheckRefusal::Targets {
                    allowed: 0,
                    required: 1
                }
            ) | (
                3,
                FormulaSourceCheckRefusal::RetainedRecords {
                    allowed: 9,
                    required: 10
                }
            )
        ));
    }
    let sources = namespace
        .check_root_sources(FormulaSourceCheckLimits::new(2, 4, 1, 10))
        .map_err(|e| e.to_string())?;
    assert_eq!(sources.proof().retained_records(), 10);
    assert_eq!(provider.calls.get(), 0);
    assert_eq!(provider.base.trace.get(), 0);
    Ok(())
}

#[test]
fn empty_root_sources_accept_zero_native_limits_and_keep_actual_root_after_map_consumption(
) -> TestResult {
    let root = super::root()?;
    let other = super::root()?;
    let provider = Provider::new(&root, &other, Fault::None)?;
    let context = scope(&root)?
        .bind_drafting_context(&provider, limits(10000, 10000, 10000)?)
        .map_err(|e| e.to_string())?;
    let raw = DraftingProgramInputs::new(context, vec![DraftingProgramStep::Scalar { ordinal: 1 }]);
    let pp = DraftingProgramLimits::new(limits(10000, 10000, 10000)?, 10, 100, 1000);
    let program = raw
        .admit(&pp)
        .map_err(|e| e.to_string())?
        .normalized()
        .map_err(|e| e.to_string())?;
    let names = DraftingProgramNameInputs::new(&program, vec![], vec![]);
    let np = DraftingProgramNameLimits::new(limits(10000, 10000, 10000)?, 16, 32);
    let work = names.admit_name_work(&np).map_err(|e| e.to_string())?;
    let positions = work.bind_output_positions().map_err(|e| e.to_string())?;
    let roles = positions.bind_role_sources().map_err(|e| e.to_string())?;
    let initial = roles.bind_initial_sources().map_err(|e| e.to_string())?;
    let namespace = initial
        .bind_initial_namespace()
        .map_err(|e| e.to_string())?;
    provider.calls.set(0);
    provider.base.trace.set(0);
    let sources = namespace
        .check_root_sources(FormulaSourceCheckLimits::new(0, 0, 0, 0))
        .map_err(|e| e.to_string())?;
    let op = DraftingProgramOrderLimits::new(1, 3, 9);
    let order = namespace.check_order(&op).map_err(|e| e.to_string())?;
    assert!(std::ptr::eq(sources.inputs(), order.inputs()));
    assert!(std::ptr::eq(sources.normalized().scope().root(), &root));
    assert!(sources.proof().sources().is_empty());
    assert_eq!(sources.proof().target_count(), 0);
    assert_eq!(sources.proof().retained_records(), 0);
    assert_eq!(provider.calls.get(), 0);
    assert_eq!(provider.base.trace.get(), 0);
    Ok(())
}
