//! Raw naming retains real schema/catalogue sources without admitting their requested association.
use super::{limits, root, scope, Source, TestResult};
use sc_core::{
    construction::*,
    design::{
        DesignResourceCount, DesignResourceLimitError, DesignResourceLocation, DesignResourceMeter,
        DesignResourceScope, DesignResourceSink,
    },
    name::MachineToken,
    ontology::{EdgeRef, EntityId, LocalTag, PointRef},
    recipe::{
        FormulaBindingKind, FormulaDeclaration, FormulaDeclarationSource,
        FormulaExpressionDocument as Doc, FormulaInitialDeclaration, FormulaInputOrigin,
        FormulaScalarInputOrigin,
    },
    value::{LengthDeclaration, LengthDeclarationDefinition, LengthState},
};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn point(n: u128) -> PointRef {
    PointRef::new(id(n), LocalTag::FIRST)
}

#[test]
fn raw_names_keep_all_six_actual_sources_global_names_and_original_vectors() -> TestResult {
    // No statements, but actual authored Root text remains before every later component.
    let root = super::root_with_recipe(" \n")?;
    let source = Source::new(&root);
    let context = scope(&root)?
        .bind_drafting_context(&source, limits(1000, 1000, 1000)?)
        .map_err(|e| e.to_string())?;
    let input = DraftingProgramInputs::new(context, vec![]);
    let policy = DraftingProgramLimits::new(limits(1000, 1000, 1000)?, 0, 0, 0);
    let program = input
        .admit(&policy)
        .map_err(|e| e.to_string())?
        .normalized()
        .map_err(|e| e.to_string())?;
    let p = PointConstruction::new(
        id(100),
        Doc::new("private_customer_length".into())?,
        Doc::new("0 mm".into())?,
    );
    let line = LineConstruction::new(id(101), point(80), point(81))?;
    let arc = CenterArcConstruction::new(
        id(102),
        point(80),
        Doc::new("5 mm".into())?,
        Doc::new("0 deg".into())?,
        Doc::new("90 deg".into())?,
    )?;
    let bezier = BezierConstruction::new(
        id(103),
        BezierControlPoints::Quadratic {
            start: point(80),
            control: point(81),
            end: point(82),
        },
    )?;
    let point_tag = LocalTag::new(7);
    let edge_tag = LocalTag::new(8);
    let point_catalog = PointOutputRoleCatalog::new(
        id(104),
        vec![PointOutputRoleDefinition::new(
            point_tag,
            MachineToken::new("local_point")?,
        )],
        vec![point_tag],
    );
    let point_catalog = point_catalog
        .admit(PointOutputRoleCatalogLimits::new(1, 1, 11))
        .map_err(|e| e.to_string())?;
    let edge_catalog = EdgeOutputRoleCatalog::new(
        id(105),
        vec![EdgeOutputRoleDefinition::new(
            edge_tag,
            MachineToken::new("local_edge")?,
        )],
        vec![edge_tag],
    );
    let edge_catalog = edge_catalog
        .admit(EdgeOutputRoleCatalogLimits::new(1, 1, 10))
        .map_err(|e| e.to_string())?;
    let point_role = point_catalog
        .active_role(point_tag)
        .ok_or("actual point membership")?;
    let edge_role = edge_catalog
        .active_role(edge_tag)
        .ok_or("actual edge membership")?;
    let scalar_name = MachineToken::new("extra_angle")?;
    let initial = vec![
        FormulaInitialDeclaration::try_from(FormulaDeclaration::input(
            &scalar_name,
            FormulaScalarInputOrigin::Profile,
            id(201),
            id(202),
            FormulaBindingKind::Angle,
        ))
        .map_err(|_| "original initial metadata")?,
    ];
    use DraftingGeometryNameSource as G;
    let sources = [
        G::Point(&p),
        G::Line(&line),
        G::CenterArc(&arc),
        G::Bezier(&bezier),
        G::PointRole(point_role),
        G::EdgeRole(edge_role),
    ];
    let positions = [0, usize::MAX, 1, 1, 7, 7];
    let global_names = [
        "global_point",
        "global_line",
        "global_arc",
        "global_bezier",
        "renamed_point",
        "renamed_edge",
    ];
    let mut outputs = Vec::new();
    for ((source, position), name) in sources.into_iter().zip(positions).zip(global_names) {
        outputs.push(DraftingOutputName::new(
            position,
            MachineToken::new(name)?,
            source,
        ));
    }
    let initial_buffer = initial.as_ptr();
    let output_buffer = outputs.as_ptr();
    source.trace.set(0);
    let names = DraftingProgramNameInputs::new(&program, initial, outputs);
    assert!(std::ptr::eq(names.program(), &program));
    assert_eq!(names.initial().as_ptr(), initial_buffer);
    assert_eq!(names.outputs().as_ptr(), output_buffer);
    let [original] = names.initial() else {
        return Err("original complete initial vector".into());
    };
    assert_eq!(original.declaration().name(), scalar_name.as_str());
    assert!(
        matches!(original.declaration().source(),FormulaDeclarationSource::Input {input,declaration,..} if input==id(201) && declaration==id(202))
    );
    let expected = [
        DraftingGeometryReference::Point(p.output()),
        DraftingGeometryReference::Edge(line.output()),
        DraftingGeometryReference::Edge(arc.output()),
        DraftingGeometryReference::Edge(bezier.output()),
        DraftingGeometryReference::Point(PointRef::new(id(104), point_tag)),
        DraftingGeometryReference::Edge(EdgeRef::new(id(105), edge_tag)),
    ];
    assert_eq!(names.outputs().len(), 6);
    for (((output, reference), position), name) in names
        .outputs()
        .iter()
        .zip(expected)
        .zip(positions)
        .zip(global_names)
    {
        assert_eq!(output.position(), position);
        assert_eq!(output.name().as_str(), name);
        assert_eq!(output.source().reference(), reference);
        assert_eq!(output.declaration().name(), name);
        match (output.declaration().source(), reference) {
            (
                FormulaDeclarationSource::Point(actual),
                DraftingGeometryReference::Point(expected),
            ) => assert_eq!(actual, expected),
            (FormulaDeclarationSource::Edge(actual), DraftingGeometryReference::Edge(expected)) => {
                assert_eq!(actual, expected)
            }
            _ => return Err("original source output kind".into()),
        }
    }
    let role_name = names.outputs().get(4).ok_or("original point-role name")?;
    let G::PointRole(actual) = role_name.source() else {
        return Err("actual original point role".into());
    };
    assert!(std::ptr::eq(actual.catalog(), &point_catalog));
    assert!(std::ptr::eq(actual.definition(), point_role.definition()));
    assert_eq!(actual.definition().key().as_str(), "local_point");
    assert_eq!(role_name.name().as_str(), "renamed_point");
    assert!(!format!("{names:?} {role_name:?}").contains("renamed_point"));
    assert!(!format!("{names:?}").contains("private_customer_length"));
    assert_eq!(source.trace.get(), 0);
    let mut meter = DesignResourceMeter::new(limits(usize::MAX, usize::MAX, 1)?);
    names.charge_extra_resources(&mut meter)?;
    let demand = meter.finish()?;
    let fixed_documents = [
        p.coordinate(PointCoordinate::X),
        p.coordinate(PointCoordinate::Y),
        arc.argument(CenterArcArgument::Radius),
        arc.argument(CenterArcArgument::StartAngle),
        arc.argument(CenterArcArgument::SweepAngle),
    ];
    let document_bytes: usize = fixed_documents
        .into_iter()
        .map(|d| d.source().len() + d.canonical().as_str().len())
        .sum();
    let name_bytes: usize = names
        .outputs()
        .iter()
        .map(|n| n.name().as_str().len())
        .sum();
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (
            17,
            scalar_name.as_str().len() + name_bytes + document_bytes + 11 + 10,
            0
        )
    );
    let mut limited = DesignResourceMeter::new(limits(16, usize::MAX, 1)?);
    let error = names
        .charge_extra_resources(&mut limited)
        .expect_err("last actual retained role vector exceeds quota");
    assert!(
        matches!(error,DesignResourceLimitError::Count{unit:DesignResourceCount::CollectionEntries,
        maximum:16,consumed:16,requested:1,location:DesignResourceLocation{owner:Some(owner),
        field:"drafting.names.edge_roles.active_roles",index:Some(5),..}} if owner==id(105))
    );
    assert_eq!(limited.finish(), Err(error));
    // Empty containing arenas still retain actual Root/context fields before the17 name entries.
    let raw_entries =
        context.scope().demand().collection_entries() + context.demand().collection_entries();
    let full_entries = raw_entries + 17;
    let full_bytes = context.scope().demand().text_bytes()
        + context.demand().text_bytes()
        + scalar_name.as_str().len()
        + name_bytes
        + document_bytes
        + 11
        + 10;
    let full_bits = context.scope().demand().counter_bits() + context.demand().counter_bits();
    source.trace.set(0);
    let mut meter = DesignResourceMeter::new(limits(full_entries, full_bytes, full_bits)?);
    names.charge_resources(&mut meter)?;
    let full = meter.finish()?;
    assert_eq!(
        (
            full.collection_entries(),
            full.text_bytes(),
            full.counter_bits()
        ),
        (full_entries, full_bytes, full_bits)
    );
    assert_eq!(source.trace.get(), 2);
    let mut limited = DesignResourceMeter::new(limits(full_entries - 1, full_bytes, full_bits)?);
    let error = names
        .charge_resources(&mut limited)
        .expect_err("one cumulative allowance");
    assert!(matches!(error,DesignResourceLimitError::Count {
        unit:DesignResourceCount::CollectionEntries,consumed,requested:1,
        location:DesignResourceLocation {owner:Some(owner),
            field:"drafting.names.edge_roles.active_roles",index:Some(5),..},..
    } if consumed == full_entries - 1 && owner == id(105)));
    assert_eq!(limited.finish(), Err(error));
    source.trace.set(0);
    let mut limited = DesignResourceMeter::new(limits(usize::MAX, 1, u64::MAX)?);
    assert!(matches!(
        names.charge_resources(&mut limited),
        Err(DesignResourceLimitError::Count {
            location: DesignResourceLocation {
                field: "recipe.source",
                ..
            },
            ..
        })
    ));
    assert_eq!(source.trace.get(), 0);
    Ok(())
}

#[test]
fn empty_name_inputs_keep_the_actual_program_without_callbacks_or_implicit_names() -> TestResult {
    let root = root()?;
    let source = Source::new(&root);
    let context = scope(&root)?
        .bind_drafting_context(&source, limits(1000, 1000, 1000)?)
        .map_err(|e| e.to_string())?;
    let input = DraftingProgramInputs::new(context, vec![]);
    let policy = DraftingProgramLimits::new(limits(1000, 1000, 1000)?, 0, 0, 0);
    let program = input
        .admit(&policy)
        .map_err(|e| e.to_string())?
        .normalized()
        .map_err(|e| e.to_string())?;
    source.trace.set(0);
    let names = DraftingProgramNameInputs::new(&program, vec![], vec![]);
    assert!(names.initial().is_empty() && names.outputs().is_empty());
    assert!(std::ptr::eq(names.program(), &program));
    assert_eq!(source.trace.get(), 0);
    let mut meter = DesignResourceMeter::new(limits(1, 1, 1)?);
    let first = meter
        .charge_text(
            "too much",
            DesignResourceLocation {
                scope: DesignResourceScope::CommandState,
                owner: None,
                field: "earlier.source",
                index: None,
            },
        )
        .expect_err("earlier text refusal");
    assert_eq!(names.charge_extra_resources(&mut meter), Err(first));
    assert_eq!(names.charge_resources(&mut meter), Err(first));
    assert_eq!(meter.finish(), Err(first));
    assert_eq!(source.trace.get(), 0);
    Ok(())
}

#[test]
fn extra_resources_count_canonical_evidence_inactive_definitions_indexes_and_repeated_catalogues(
) -> TestResult {
    let root = root()?;
    let source = Source::new(&root);
    let context = scope(&root)?
        .bind_drafting_context(&source, limits(1000, 1000, 1000)?)
        .map_err(|e| e.to_string())?;
    let input = DraftingProgramInputs::new(context, vec![]);
    let policy = DraftingProgramLimits::new(limits(1000, 1000, 1000)?, 0, 0, 0);
    let program = input
        .admit(&policy)
        .map_err(|e| e.to_string())?
        .normalized()
        .map_err(|e| e.to_string())?;
    let name = MachineToken::new("extra_length")?;
    let record = LengthDeclaration::new(LengthDeclarationDefinition {
        id: id(500),
        source: id(501),
        state: LengthState::Known {
            value: sc_units::Length::from_micrometres(25)?,
            evidence: vec![id(502), id(503)],
        },
    })?;
    let initial = FormulaInitialDeclaration::try_from(FormulaDeclaration::length_input(
        &name,
        FormulaInputOrigin::Measurement,
        id(504),
        &record,
    ))
    .map_err(|_| "actual canonical length metadata")?;
    let catalog = EdgeOutputRoleCatalog::new(
        id(600),
        vec![
            EdgeOutputRoleDefinition::new(LocalTag::new(7), MachineToken::new("active_edge")?),
            EdgeOutputRoleDefinition::new(LocalTag::new(9), MachineToken::new("inactive_edge")?),
        ],
        vec![LocalTag::new(7)],
    );
    let admitted = catalog
        .admit(EdgeOutputRoleCatalogLimits::new(2, 1, 24))
        .map_err(|e| e.to_string())?;
    let role = admitted
        .active_role(LocalTag::new(7))
        .ok_or("original active role")?;
    let names = DraftingProgramNameInputs::new(
        &program,
        vec![initial],
        vec![
            DraftingOutputName::new(
                1,
                MachineToken::new("one")?,
                DraftingGeometryNameSource::EdgeRole(role),
            ),
            DraftingOutputName::new(
                2,
                MachineToken::new("two")?,
                DraftingGeometryNameSource::EdgeRole(role),
            ),
        ],
    );
    let mut meter = DesignResourceMeter::new(limits(19, 66, 1)?);
    source.trace.set(0);
    names.charge_extra_resources(&mut meter)?;
    let demand = meter.finish()?;
    // Initial vector1+evidence2+outputs2+each catalogue's raw2+1/index2+1/association1 twice.
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (19, 66, 0)
    );
    assert_eq!(source.trace.get(), 0);
    let mut evidence_limit = DesignResourceMeter::new(limits(2, usize::MAX, 1)?);
    let failure = names
        .charge_extra_resources(&mut evidence_limit)
        .expect_err("actual original evidence vector");
    assert!(
        matches!(failure,DesignResourceLimitError::Count{unit:DesignResourceCount::CollectionEntries,
        consumed:1,requested:2,location:DesignResourceLocation{owner:Some(owner),field:"drafting.names.initial.length.evidence",index:Some(0),..},..} if owner==id(500))
    );
    assert_eq!(evidence_limit.finish(), Err(failure));
    Ok(())
}
