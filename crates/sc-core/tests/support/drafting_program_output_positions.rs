//! Actual fixed owners and genuine mixed positions;role catalogues remain candidates,not manifests.
use super::{limits, root_with_recipe, scope, Source, TestResult};
use sc_core::{
    construction::*,
    design::{
        DesignResourceCount, DesignResourceLimitError, DesignResourceLocation, DesignResourceMeter,
    },
    name::MachineToken,
    ontology::{DirectedRange, Direction, EdgeRange, EdgeRef, EntityId, LocalTag, PointRef},
    recipe::FormulaExpressionDocument as Doc,
};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}

#[test]
fn published_offset_aliases_use_existing_edge_name_accounting_and_position_candidates() -> TestResult
{
    let root = super::root()?;
    let source = Source::new(&root);
    let context = scope(&root)?
        .bind_drafting_context(&source, limits(1000, 10000, 1000)?)
        .map_err(|e| e.to_string())?;
    let offset = OffsetConstruction::new(
        id(400),
        DirectedRange {
            range: EdgeRange::whole(EdgeRef::new(id(401), LocalTag::FIRST)),
            direction: Direction::Original,
        },
        OffsetSide::Left,
        id(402),
    )?;
    let inputs = DraftingProgramInputs::new(
        context,
        vec![DraftingProgramStep::Drafting(
            DraftingOperationInput::OffsetEdge(&offset),
        )],
    );
    let raw_policy = DraftingProgramLimits::new(limits(1000, 10000, 1000)?, 0, 0, 1);
    let program = inputs
        .admit(&raw_policy)
        .map_err(|e| e.to_string())?
        .normalized()
        .map_err(|e| e.to_string())?;
    let tag = LocalTag::new(7);
    let raw = OffsetRoleCatalog::new(
        id(400),
        vec![OffsetRoleDefinition::new(
            tag,
            MachineToken::new("local_offset")?,
        )],
        vec![tag],
    );
    let catalog = raw
        .admit(OffsetRoleCatalogLimits::new(1, 1, 12))
        .map_err(|e| e.to_string())?;
    // Direct public type assignment/enum construction:these published names are intentional aliases.
    let role: ActiveOffsetRole<'_> = catalog.active_role(tag).ok_or("actual Offset API role")?;
    let original_catalog: &AdmittedEdgeOutputRoleCatalog<'_> = role.catalog();
    let named_source = DraftingGeometryNameSource::EdgeRole(role);
    assert_eq!(
        named_source.reference(),
        DraftingGeometryReference::Edge(EdgeRef::new(id(400), tag))
    );
    let names = DraftingProgramNameInputs::new(
        &program,
        vec![],
        vec![DraftingOutputName::new(
            1,
            MachineToken::new("offset_fragment")?,
            named_source,
        )],
    );
    source.trace.set(0);
    let mut extra = DesignResourceMeter::new(limits(6, 27, 1)?);
    names.charge_extra_resources(&mut extra)?;
    let demand = extra.finish()?;
    // Output vector1 + raw definition1/active1 + retained by-tag1/active1/association1 =6.
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (6, 27, 0)
    );
    assert_eq!(source.trace.get(), 0);
    let mut short = DesignResourceMeter::new(limits(5, 27, 1)?);
    let error = names
        .charge_extra_resources(&mut short)
        .expect_err("shared actual retained catalogue vector");
    assert!(
        matches!(error,DesignResourceLimitError::Count { unit:DesignResourceCount::CollectionEntries,consumed:5,requested:1,
        location:DesignResourceLocation { owner:Some(owner),field:"drafting.names.edge_roles.active_roles",index:Some(0),..},.. } if owner==id(400))
    );
    assert_eq!(short.finish(), Err(error));
    let policy = DraftingProgramNameLimits::new(limits(1000, 10000, 1000)?, 9, 10);
    let work = names.admit_name_work(&policy).map_err(|e| e.to_string())?;
    let raw_entries =
        context.scope().demand().collection_entries() + context.demand().collection_entries();
    assert_eq!(work.demand().collection_entries(), raw_entries + 1 + 1 + 6); // Mixed input and actual normalized slot.
    assert_eq!(
        work.demand().text_bytes(),
        context.scope().demand().text_bytes() + context.demand().text_bytes() + 27
    );
    source.trace.set(0);
    let positions = work.bind_output_positions().map_err(|e| e.to_string())?;
    let view = positions.outputs().next().ok_or("Offset position")?;
    assert_eq!(view.step().position(), 1);
    assert!(view.requires_role_binding());
    let DraftingGeometryNameSource::EdgeRole(actual) = view.output().source() else {
        return Err("shared actual role source".into());
    };
    assert!(std::ptr::eq(actual.catalog(), original_catalog));
    let DraftingProgramStepKind::Drafting(actual) = view.step().kind() else {
        return Err("genuine native Offset step".into());
    };
    assert!(
        matches!(actual.input(),DraftingOperationInput::OffsetEdge(p) if std::ptr::eq(p,&offset))
    );
    assert_eq!(source.trace.get(), 0);
    Ok(())
}
fn point(n: u128) -> PointRef {
    PointRef::new(id(n), LocalTag::FIRST)
}
fn doc(s: &str) -> TestResult<Doc> {
    Ok(Doc::new(s.into())?)
}
#[test]
fn complete_positions_keep_actual_fixed_owners_and_unresolved_role_candidates() -> TestResult {
    let root = root_with_recipe("let scalar:length=1 mm")?;
    let source = Source::new(&root);
    let context = scope(&root)?
        .bind_drafting_context(&source, limits(1000, 10000, 1000)?)
        .map_err(|e| e.to_string())?;
    let p = PointConstruction::new(id(100), doc("private_width")?, doc("1 mm/0")?);
    let line = LineConstruction::new(id(101), point(80), point(81))?;
    let arc = CenterArcConstruction::new(
        id(102),
        point(80),
        doc("5 mm")?,
        doc("0 deg")?,
        doc("90 deg")?,
    )?;
    let bezier = BezierConstruction::new(
        id(103),
        BezierControlPoints::Quadratic {
            start: point(80),
            control: point(81),
            end: point(82),
        },
    )?;
    let input = DraftingProgramInputs::new(
        context,
        vec![
            DraftingProgramStep::Scalar { ordinal: 1 },
            DraftingProgramStep::Drafting(DraftingOperationInput::ConstructPoint(&p)),
            DraftingProgramStep::Drafting(DraftingOperationInput::DrawLine(&line)),
            DraftingProgramStep::Drafting(DraftingOperationInput::DrawArc(&arc)),
            DraftingProgramStep::Drafting(DraftingOperationInput::DrawBezier(&bezier)),
        ],
    );
    let raw_policy = DraftingProgramLimits::new(limits(1000, 10000, 1000)?, 100, 1000, 10000);
    let program = input
        .admit(&raw_policy)
        .map_err(|e| e.to_string())?
        .normalized()
        .map_err(|e| e.to_string())?;
    let point_raw = PointOutputRoleCatalog::new(
        id(101),
        vec![PointOutputRoleDefinition::new(
            LocalTag::new(7),
            MachineToken::new("local_point")?,
        )],
        vec![LocalTag::new(7)],
    );
    let point_catalog = point_raw
        .admit(PointOutputRoleCatalogLimits::new(1, 1, 11))
        .map_err(|e| e.to_string())?;
    let edge_raw = EdgeOutputRoleCatalog::new(
        id(101),
        vec![EdgeOutputRoleDefinition::new(
            LocalTag::new(8),
            MachineToken::new("local_edge")?,
        )],
        vec![LocalTag::new(8)],
    );
    let edge_catalog = edge_raw
        .admit(EdgeOutputRoleCatalogLimits::new(1, 1, 10))
        .map_err(|e| e.to_string())?;
    use DraftingGeometryNameSource as G;
    let point_role = point_catalog
        .active_role(LocalTag::new(7))
        .ok_or("point catalogue candidate")?;
    let edge_role = edge_catalog
        .active_role(LocalTag::new(8))
        .ok_or("edge catalogue candidate")?;
    let sources = [
        G::Point(&p),
        G::Line(&line),
        G::CenterArc(&arc),
        G::Bezier(&bezier),
        G::PointRole(point_role),
        G::EdgeRole(edge_role),
    ];
    let positions = [2, 3, 4, 5, 3, 3];
    let outputs = sources
        .into_iter()
        .zip(positions)
        .enumerate()
        .map(|(index, (source, position))| {
            Ok(DraftingOutputName::new(
                position,
                MachineToken::new(format!("global_{index}"))?,
                source,
            ))
        })
        .collect::<TestResult<Vec<_>>>()?;
    let names = DraftingProgramNameInputs::new(&program, vec![], outputs);
    let policy = DraftingProgramNameLimits::new(limits(10000, 10000, 10000)?, 64, 128);
    let work = names.admit_name_work(&policy).map_err(|e| e.to_string())?;
    source.trace.set(0);
    let bound = work.bind_output_positions().map_err(|e| e.to_string())?;
    assert!(std::ptr::eq(bound.work().inputs(), &names));
    for ((index, view), position) in bound.outputs().enumerate().zip(positions) {
        assert_eq!(view.output_index(), index);
        assert!(std::ptr::eq(
            view.output(),
            names
                .outputs()
                .get(index)
                .ok_or("original name occurrence")?
        ));
        assert_eq!(view.step().position(), position);
        assert_eq!(view.requires_role_binding(), index >= 4);
        assert!(matches!(
            view.step().kind(),
            DraftingProgramStepKind::Drafting(_)
        ));
    }
    let mut iter = bound.outputs();
    assert_eq!(iter.len(), 6);
    assert_eq!(iter.next_back().ok_or("last")?.output_index(), 5);
    assert_eq!(iter.nth(1).ok_or("second")?.step().position(), 3);
    assert_eq!(
        iter.next_back()
            .ok_or("point-role candidate")?
            .output_index(),
        4
    );
    assert_eq!(iter.len(), 2);
    assert_eq!(iter.next().ok_or("arc")?.step().position(), 4);
    assert_eq!(iter.next_back().ok_or("bezier")?.step().position(), 5);
    assert!(iter.next().is_none() && iter.next_back().is_none());
    assert_eq!(source.trace.get(), 0);
    assert!(!format!("{bound:?} {:?}", bound.outputs().next()).contains("private_width"));
    // A genuine catalogue at a Line step is still only a creator candidate,not a family role proof.
    assert!(bound
        .outputs()
        .nth(4)
        .ok_or("unresolved point role")?
        .requires_role_binding());
    let foreign = LineConstruction::new(id(101), point(80), point(81))?;
    use DraftingOutputPositionRefusal as R;
    for (position, candidate, expected) in [
        (0, G::Point(&p), R::MissingStep),
        (usize::MAX, G::Point(&p), R::MissingStep),
        (6, G::Point(&p), R::MissingStep),
        (1, G::Point(&p), R::ScalarStep { ordinal: 1 }),
        (
            3,
            G::Point(&p),
            R::Family {
                expected: DraftingOperationFamily::ConstructPoint,
                actual: DraftingOperationFamily::DrawLine,
            },
        ),
        (3, G::Line(&foreign), R::FixedOwner),
        (
            2,
            G::EdgeRole(edge_role),
            R::RoleCreator {
                requested: id(101),
                actual: id(100),
            },
        ),
    ] {
        let names = DraftingProgramNameInputs::new(
            &program,
            vec![],
            vec![
                DraftingOutputName::new(2, MachineToken::new("earlier_valid")?, G::Point(&p)),
                DraftingOutputName::new(position, MachineToken::new("actual_bad_name")?, candidate),
                DraftingOutputName::new(
                    usize::MAX,
                    MachineToken::new("later_bad_name")?,
                    G::Line(&line),
                ),
            ],
        );
        let work = names.admit_name_work(&policy).map_err(|e| e.to_string())?;
        source.trace.set(0);
        let error = work
            .bind_output_positions()
            .expect_err("first original output refusal,no prefix");
        assert_eq!(error.output_index(), 1);
        assert_eq!(error.output().position(), position);
        assert_eq!(error.output().name().as_str(), "actual_bad_name");
        assert_eq!(error.refusal(), expected);
        assert!(std::ptr::eq(error.work().inputs(), &names));
        assert_eq!(source.trace.get(), 0);
        assert!(!format!("{error:?}").contains("actual_bad_name"));
    }
    let names = DraftingProgramNameInputs::new(&program, vec![], vec![]);
    let work = names.admit_name_work(&policy).map_err(|e| e.to_string())?;
    source.trace.set(0);
    assert_eq!(
        work.bind_output_positions()
            .map_err(|e| e.to_string())?
            .outputs()
            .len(),
        0
    );
    assert_eq!(source.trace.get(), 0);
    Ok(())
}
