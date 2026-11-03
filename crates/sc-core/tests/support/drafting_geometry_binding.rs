//! Complete mixed suppliers and later failures use actual published Offset parents and records.
use super::*;
use sc_core::recipe::{
    FormulaSourceCheckLimits, FormulaSourceCycleLimits, FormulaSourceDependencyLimits,
};
use std::error::Error;

#[test]
fn complete_geometry_binding_keeps_existing_fixed_unnamed_sources_and_original_refusals(
) -> TestResult {
    let f = fixture::fixture();
    let allowance = f.allowance();
    let input = f.input();
    let foreign_input = f.input();
    let bound = input.bind_allowance(&allowance)?;
    let foreign_bound = foreign_input.bind_allowance(&allowance)?;
    let interval = OffsetInterval::new(parameter(1, 4), parameter(3, 4))?;
    let main = OffsetCandidateManifest::new(
        input.operation(),
        *input.source(),
        vec![OffsetCandidateFragment::new(LocalTag::new(9), interval)],
    );
    let other = OffsetCandidateManifest::new(
        input.operation(),
        *input.source(),
        vec![OffsetCandidateFragment::new(LocalTag::new(7), interval)],
    );
    let raw = OffsetRoleCatalog::new(
        input.operation(),
        vec![
            OffsetRoleDefinition::new(LocalTag::new(9), MachineToken::new("main")?),
            OffsetRoleDefinition::new(LocalTag::new(7), MachineToken::new("other")?),
        ],
        vec![LocalTag::new(9), LocalTag::new(7)],
    );
    let catalog = raw
        .admit(OffsetRoleCatalogLimits::new(2, 2, 9))
        .map_err(|e| e.to_string())?;
    let foreign_catalog = raw
        .admit(OffsetRoleCatalogLimits::new(2, 2, 9))
        .map_err(|e| e.to_string())?;
    let admitted = main
        .admit(&bound, OffsetCandidateLimits::new(1))
        .map_err(|e| e.to_string())?;
    let foreign_admitted = main
        .admit(&foreign_bound, OffsetCandidateLimits::new(1))
        .map_err(|e| e.to_string())?;
    let other_admitted = other
        .admit(&bound, OffsetCandidateLimits::new(1))
        .map_err(|e| e.to_string())?;
    let policy = OffsetRolePresenceLimits::new(1, 2, 2);
    let presence = admitted
        .bind_active_roles(&catalog, policy)
        .map_err(|e| e.to_string())?;
    let wrong_input = foreign_admitted
        .bind_active_roles(&catalog, policy)
        .map_err(|e| e.to_string())?;
    let alternate_catalog = admitted
        .bind_active_roles(&foreign_catalog, policy)
        .map_err(|e| e.to_string())?;
    let wrong_reference = other_admitted
        .bind_active_roles(&catalog, policy)
        .map_err(|e| e.to_string())?;
    let fields = [
        presence.outputs().next().ok_or("main")?,
        wrong_input.outputs().next().ok_or("foreign input")?,
        alternate_catalog
            .outputs()
            .next()
            .ok_or("alternate catalogue")?,
        wrong_reference
            .outputs()
            .next()
            .ok_or("different reference")?,
    ];
    let [main_field, ..] = fields;
    let root = root(f)?;
    for mode in 0..11 {
        let fault = match mode {
            1 => Fault::Source,
            2 => Fault::Input,
            3 => Fault::Definition,
            4 => Fault::Frame,
            5 => Fault::Snapshot,
            6 => Fault::Catalogue,
            _ => Fault::None,
        };
        let source = Provider::new(&root, fields, fault);
        let context = scope(&root)?
            .bind_drafting_context(&source, limits(100000, 100000, 100000)?)
            .map_err(|e| e.to_string())?;
        let point = PointConstruction::new(
            id(500),
            FormulaExpressionDocument::new("1 mm/0".into())?,
            FormulaExpressionDocument::new("2 mm".into())?,
        );
        let line = LineConstruction::new(
            id(410),
            if mode == 8 {
                PointRef::new(id(500), LocalTag::new(2))
            } else {
                point.output()
            },
            PointRef::new(id(if mode == 10 { 999 } else { 411 }), LocalTag::FIRST),
        )?;
        let consumer = OffsetConstruction::new(
            id(20000),
            DirectedRange {
                range: EdgeRange::whole(main_field.logical_output()),
                direction: Direction::Original,
            },
            OffsetSide::Right,
            id(20001),
        )?;
        let future = PointConstruction::new(
            id(411),
            FormulaExpressionDocument::new("1 mm".into())?,
            FormulaExpressionDocument::new("2 mm".into())?,
        );
        let mut steps = vec![
            DraftingProgramStep::Drafting(DraftingOperationInput::OffsetEdge(&input)),
            DraftingProgramStep::Drafting(DraftingOperationInput::ConstructPoint(&point)),
            DraftingProgramStep::Drafting(DraftingOperationInput::DrawLine(&line)),
            DraftingProgramStep::Drafting(DraftingOperationInput::OffsetEdge(&consumer)),
            DraftingProgramStep::Drafting(DraftingOperationInput::OffsetEdge(&consumer)),
        ];
        if mode == 7 {
            steps.push(DraftingProgramStep::Drafting(
                DraftingOperationInput::ConstructPoint(&future),
            ));
        }
        if mode == 9 {
            steps.clear();
        }
        let raw = DraftingProgramInputs::new(context, steps);
        let pp = DraftingProgramLimits::new(limits(100000, 100000, 100000)?, 100, 1000, 10000);
        let program = raw
            .admit(&pp)
            .map_err(|e| e.to_string())?
            .normalized()
            .map_err(|e| e.to_string())?;
        let names = DraftingProgramNameInputs::new(&program, vec![], vec![]);
        let np = DraftingProgramNameLimits::new(limits(100000, 100000, 100000)?, 64, 128);
        let work = names.admit_name_work(&np).map_err(|e| e.to_string())?;
        let positions = work.bind_output_positions().map_err(|e| e.to_string())?;
        let roles = positions.bind_role_sources().map_err(|e| e.to_string())?;
        let initial = roles.bind_initial_sources().map_err(|e| e.to_string())?;
        let namespace = initial
            .bind_initial_namespace()
            .map_err(|e| e.to_string())?;
        let sources = namespace
            .check_root_sources(FormulaSourceCheckLimits::new(1, 100, 0, 1000))
            .map_err(|e| e.to_string())?;
        let dependencies = sources
            .proof()
            .prepare_dependencies(FormulaSourceDependencyLimits::new(1, 0, 2))
            .map_err(|e| e.to_string())?;
        let graph = dependencies
            .admit_acyclic(FormulaSourceCycleLimits::new(1, 2))
            .map_err(|e| e.to_string())?;
        let op = DraftingProgramOrderLimits::new(100, 1000, 10000);
        let order = namespace.check_order(&op).map_err(|e| e.to_string())?;
        let inputs = sources
            .dependency_inputs(&order, &graph)
            .map_err(|e| e.to_string())?;
        let cp = DraftingProgramCanonicalLimits::new(0, 0);
        let canonical = inputs
            .bind_canonical_inputs(&cp)
            .map_err(|e| e.to_string())?;
        let n = if mode == 9 { 0 } else { 5 };
        let gp = DraftingProgramGeometryLimits::new(
            limits(100000, 100000, 100000)?,
            n,
            n * 2,
            n * raw.steps().len(),
        );
        let admitted = canonical
            .admit_geometry_work(&gp)
            .map_err(|e| e.to_string())?;
        source.base.trace.set(0);
        source.calls.set(0);
        match admitted.bind_geometry() {
            Ok(proof) => {
                assert!(matches!(mode, 0 | 6 | 9));
                assert!(std::ptr::eq(proof.work(), &admitted));
                assert_eq!(proof.requests().len(), n);
                assert_eq!(proof.suppliers().len(), n);
                if mode != 9 {
                    assert_eq!(
                        proof
                            .requests()
                            .iter()
                            .map(|r| r.occurrence().position())
                            .collect::<Vec<_>>(),
                        [Some(1), Some(3), Some(3), Some(4), Some(5)]
                    );
                    let [DraftingProgramGeometrySupplier::Existing(edge), DraftingProgramGeometrySupplier::Fixed(fixed), DraftingProgramGeometrySupplier::Existing(point_source), DraftingProgramGeometrySupplier::Planned {
                        request,
                        source: planned,
                    }, DraftingProgramGeometrySupplier::Planned {
                        source: repeated, ..
                    }] = proof.suppliers()
                    else {
                        return Err("complete supplier order".into());
                    };
                    assert!(std::ptr::eq(
                        edge.geometry(),
                        source.existing.first().ok_or("actual existing edge")?
                    ));
                    assert!(std::ptr::eq(
                        point_source.geometry(),
                        source.existing.get(1).ok_or("actual existing point")?
                    ));
                    assert_eq!(fixed.step().position(), 2);
                    assert!(!fixed.requires_role_binding());
                    assert!(
                        matches!(fixed.input(),DraftingOperationInput::ConstructPoint(p) if std::ptr::eq(p,&point))
                    );
                    assert_eq!(request.step().position(), 1);
                    assert_eq!(
                        request.reference(),
                        DraftingGeometryReference::Edge(main_field.logical_output())
                    );
                    for response in [planned, repeated] {
                        assert!(response.frame.is_same_record(context.frame()));
                        assert!(response.snapshot.is_same_record(context.snapshot()));
                        assert!(
                            matches!(response.output.input(),DraftingOperationInput::OffsetEdge(p) if std::ptr::eq(p,&input))
                        );
                    }
                    assert_eq!(source.calls.get(), 2);
                    assert_eq!(source.base.trace.get(), 77);
                } else {
                    assert_eq!(source.calls.get(), 0);
                    assert_eq!(source.base.trace.get(), 0);
                }
                assert!(!format!("{proof:?}").contains("private"));
            }
            Err(error) => {
                assert!(std::ptr::eq(error.work(), &admitted));
                let DraftingProgramGeometryRefusal::Binding(refusal) = error.refusal() else {
                    return Err("unexpected allocation failure".into());
                };
                match refusal.as_ref() {
                    DraftingProgramGeometryBindingRefusal::Creator {
                        index,
                        request,
                        cause,
                    } => {
                        assert!(matches!(mode, 7 | 8));
                        assert_eq!(*index, if mode == 7 { 2 } else { 1 });
                        assert_eq!(request.occurrence().position(), Some(3));
                        assert!(matches!(
                            (mode, cause),
                            (
                                7,
                                DraftingProgramGeometryCreatorRefusal::FutureCreator { .. }
                            ) | (8, DraftingProgramGeometryCreatorRefusal::FixedOutput { .. })
                        ));
                        assert_eq!(source.base.trace.get(), 0);
                        assert_eq!(source.calls.get(), 0);
                    }
                    DraftingProgramGeometryBindingRefusal::Existing {
                        index,
                        request,
                        cause,
                    } => {
                        assert_eq!(mode, 10);
                        assert_eq!(*index, 2);
                        assert_eq!(request.occurrence().position(), Some(3));
                        assert!(
                            matches!(cause.as_ref(),DraftingGeometryError::Source {cause,..} if cause.as_ref()==&SourceError::Missing)
                        );
                        assert!(error.source().is_some_and(|e| e.is::<SourceError>()));
                        assert_eq!(source.base.trace.get(), 77);
                        assert_eq!(source.calls.get(), 0);
                    }
                    DraftingProgramGeometryBindingRefusal::PlannedSource {
                        index,
                        request,
                        cause,
                    } => {
                        assert_eq!(mode, 1);
                        assert_eq!(*index, 3);
                        assert_eq!(request.occurrence().position(), Some(4));
                        assert_eq!(cause.as_ref(), &SourceError::Rejected);
                        assert!(error.source().is_some_and(|e| e.is::<SourceError>()));
                        assert_eq!(source.calls.get(), 1);
                        assert_eq!(source.base.trace.get(), 77);
                    }
                    DraftingProgramGeometryBindingRefusal::PlannedMismatch {
                        index,
                        request,
                        returned,
                        issue,
                    } => {
                        assert_eq!(*index, 3);
                        assert_eq!(request.occurrence().position(), Some(4));
                        assert_eq!(
                            *issue,
                            match mode {
                                2 => DraftingProgramPlannedGeometryIssue::Input,
                                3 => DraftingProgramPlannedGeometryIssue::Reference,
                                4 => DraftingProgramPlannedGeometryIssue::Frame,
                                5 => DraftingProgramPlannedGeometryIssue::Snapshot,
                                _ => return Err("unexpected mismatch".into()),
                            }
                        );
                        assert!(returned.snapshot.is_same_record(if mode >= 4 {
                            &source.foreign_snapshot
                        } else {
                            &source.snapshot
                        }));
                        if mode != 5 {
                            assert!(returned.frame.is_same_record(&source.foreign_frame));
                        }
                        assert_eq!(source.calls.get(), 1);
                        assert_eq!(source.base.trace.get(), 77);
                    }
                }
                assert!(!format!("{error:?}").contains("private"));
            }
        }
    }
    Ok(())
}
