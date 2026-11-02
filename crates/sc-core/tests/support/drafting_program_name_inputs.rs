//! Raw naming retains real schema/catalogue sources without admitting their requested association.
use super::{limits, root, scope, Source, TestResult};
use sc_core::{
    construction::*,
    name::MachineToken,
    ontology::{EdgeRef, EntityId, LocalTag, PointRef},
    recipe::{
        FormulaBindingKind, FormulaDeclaration, FormulaDeclarationSource,
        FormulaExpressionDocument as Doc, FormulaInitialDeclaration, FormulaScalarInputOrigin,
    },
};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn point(n: u128) -> PointRef {
    PointRef::new(id(n), LocalTag::FIRST)
}

#[test]
fn raw_names_keep_all_six_actual_sources_global_names_and_original_vectors() -> TestResult {
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
    Ok(())
}
