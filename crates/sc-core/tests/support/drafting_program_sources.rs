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
            // A recomputed proof over identical actual inputs/corpus is still a different owner.
            let repeated_sources = namespace
                .check_root_sources(policy)
                .map_err(|e| e.to_string())?;
            let dp = sc_core::recipe::FormulaSourceDependencyLimits::new(2, 0, 3);
            let cp = sc_core::recipe::FormulaSourceCycleLimits::new(2, 4);
            let dependencies = sources
                .proof()
                .prepare_dependencies(dp)
                .map_err(|e| e.to_string())?;
            let graph = dependencies.admit_acyclic(cp).map_err(|e| e.to_string())?;
            let repeated_dependencies = repeated_sources
                .proof()
                .prepare_dependencies(dp)
                .map_err(|e| e.to_string())?;
            let repeated_graph = repeated_dependencies
                .admit_acyclic(cp)
                .map_err(|e| e.to_string())?;
            // Source proof owns no map borrow:move the namespace and retain both original proofs.
            let op = DraftingProgramOrderLimits::new(1, 3, 9);
            let order = namespace.check_order(&op).map_err(|e| e.to_string())?;
            assert!(std::ptr::eq(order.inputs(), sources.inputs()));
            assert_eq!(order.steps().len(), 1);
            let joined = sources
                .dependency_inputs(&order, &graph)
                .map_err(|e| e.to_string())?;
            assert!(
                std::ptr::eq(joined.sources(), &sources)
                    && std::ptr::eq(joined.order(), &order)
                    && std::ptr::eq(joined.graph(), &graph)
            );
            assert_eq!(joined.graph().dependencies().limits(), dp);
            assert_eq!(joined.graph().limits(), cp);
            let error = sources
                .dependency_inputs(&order, &repeated_graph)
                .expect_err("different native proof over same data");
            assert_eq!(
                error.issue(),
                DraftingProgramDependencyOwnerIssue::SourceGraph
            );
            assert!(std::ptr::eq(error.graph(), &repeated_graph));
            assert!(std::ptr::eq(
                repeated_sources.normalized(),
                sources.normalized()
            ));
            assert!(std::ptr::eq(repeated_sources.inputs(), sources.inputs()));
            // Independent initial binding is rejected even over the same actual name/program owner.
            let other_initial = roles.bind_initial_sources().map_err(|e| e.to_string())?;
            let other_namespace = other_initial
                .bind_initial_namespace()
                .map_err(|e| e.to_string())?;
            let other_order = other_namespace
                .check_order(&op)
                .map_err(|e| e.to_string())?;
            provider.calls.set(0);
            provider.base.trace.set(0);
            let error = sources
                .dependency_inputs(&other_order, &repeated_graph)
                .expect_err("first original association mismatch");
            assert_eq!(
                error.issue(),
                DraftingProgramDependencyOwnerIssue::OrderInputs
            );
            assert!(
                std::ptr::eq(error.sources(), &sources)
                    && std::ptr::eq(error.order(), &other_order)
                    && std::ptr::eq(error.graph(), &repeated_graph)
            );
            assert!(!format!("{joined:?} {error:?}").contains("private"));
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

#[test]
fn dependency_owners_refuse_equal_looking_foreign_root_and_keep_all_actual_provided_proofs(
) -> TestResult {
    let root_a = super::root()?;
    let root_b = super::root()?;
    assert_eq!(root_a.id(), root_b.id());
    assert_eq!(root_a.revision(), root_b.revision());
    let provider_a = Provider::new(&root_a, &root_b, Fault::None)?;
    let provider_b = Provider::new(&root_b, &root_a, Fault::None)?;
    let ca = scope(&root_a)?
        .bind_drafting_context(&provider_a, limits(10000, 10000, 10000)?)
        .map_err(|e| e.to_string())?;
    let cb = scope(&root_b)?
        .bind_drafting_context(&provider_b, limits(10000, 10000, 10000)?)
        .map_err(|e| e.to_string())?;
    let raw_a = DraftingProgramInputs::new(ca, vec![DraftingProgramStep::Scalar { ordinal: 1 }]);
    let raw_b = DraftingProgramInputs::new(cb, vec![DraftingProgramStep::Scalar { ordinal: 1 }]);
    let pp = DraftingProgramLimits::new(limits(10000, 10000, 10000)?, 10, 100, 1000);
    let pa = raw_a
        .admit(&pp)
        .map_err(|e| e.to_string())?
        .normalized()
        .map_err(|e| e.to_string())?;
    let pb = raw_b
        .admit(&pp)
        .map_err(|e| e.to_string())?
        .normalized()
        .map_err(|e| e.to_string())?;
    let na = DraftingProgramNameInputs::new(&pa, vec![], vec![]);
    let nb = DraftingProgramNameInputs::new(&pb, vec![], vec![]);
    let np = DraftingProgramNameLimits::new(limits(10000, 10000, 10000)?, 16, 32);
    let wa = na.admit_name_work(&np).map_err(|e| e.to_string())?;
    let wb = nb.admit_name_work(&np).map_err(|e| e.to_string())?;
    let posa = wa.bind_output_positions().map_err(|e| e.to_string())?;
    let posb = wb.bind_output_positions().map_err(|e| e.to_string())?;
    let ra = posa.bind_role_sources().map_err(|e| e.to_string())?;
    let rb = posb.bind_role_sources().map_err(|e| e.to_string())?;
    let ia = ra.bind_initial_sources().map_err(|e| e.to_string())?;
    let ib = rb.bind_initial_sources().map_err(|e| e.to_string())?;
    let nsa = ia.bind_initial_namespace().map_err(|e| e.to_string())?;
    let nsb = ib.bind_initial_namespace().map_err(|e| e.to_string())?;
    let sp = FormulaSourceCheckLimits::new(0, 0, 0, 0);
    let sa = nsa.check_root_sources(sp).map_err(|e| e.to_string())?;
    let sb = nsb.check_root_sources(sp).map_err(|e| e.to_string())?;
    let dp = sc_core::recipe::FormulaSourceDependencyLimits::new(0, 0, 1);
    let cp = sc_core::recipe::FormulaSourceCycleLimits::new(0, 0);
    let da = sa
        .proof()
        .prepare_dependencies(dp)
        .map_err(|e| e.to_string())?;
    let db = sb
        .proof()
        .prepare_dependencies(dp)
        .map_err(|e| e.to_string())?;
    let ga = da.admit_acyclic(cp).map_err(|e| e.to_string())?;
    let gb = db.admit_acyclic(cp).map_err(|e| e.to_string())?;
    let op = DraftingProgramOrderLimits::new(1, 3, 9);
    let oa = nsa.check_order(&op).map_err(|e| e.to_string())?;
    let ob = nsb.check_order(&op).map_err(|e| e.to_string())?;
    provider_a.calls.set(0);
    provider_a.base.trace.set(0);
    provider_b.calls.set(0);
    provider_b.base.trace.set(0);
    let valid = sa.dependency_inputs(&oa, &ga).map_err(|e| e.to_string())?;
    assert!(
        std::ptr::eq(valid.sources(), &sa)
            && std::ptr::eq(valid.order(), &oa)
            && std::ptr::eq(valid.graph(), &ga)
    );
    let error = sa
        .dependency_inputs(&ob, &gb)
        .expect_err("equal ids/revision/text do not join actual owners");
    assert_eq!(
        error.issue(),
        DraftingProgramDependencyOwnerIssue::OrderInputs
    );
    assert!(
        std::ptr::eq(error.sources(), &sa)
            && std::ptr::eq(error.order(), &ob)
            && std::ptr::eq(error.graph(), &gb)
    );
    let error = sa
        .dependency_inputs(&oa, &gb)
        .expect_err("foreign graph after matching order");
    assert_eq!(
        error.issue(),
        DraftingProgramDependencyOwnerIssue::SourceGraph
    );
    assert!(std::ptr::eq(
        error.graph().dependencies().checked(),
        sb.proof()
    ));
    assert_eq!(provider_a.calls.get(), 0);
    assert_eq!(provider_a.base.trace.get(), 0);
    assert_eq!(provider_b.calls.get(), 0);
    assert_eq!(provider_b.base.trace.get(), 0);
    Ok(())
}
