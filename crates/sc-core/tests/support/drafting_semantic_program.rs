//! Original semantic ordinals/Root loans through mixed syntax and scalar-name metadata only.
use super::{
    limits, root_with_host_and_semantic, scope,
    semantic_root::{canonical, recipe},
    EmptyHost, Source, TestResult,
};
use sc_core::{construction::*, design::*, name::MachineToken, ontology::*, recipe::*};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn root() -> TestResult<DesignInputs<EmptyHost>> {
    root_with_host_and_semantic(
        "let private_width:length=1 mm",
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
fn policy() -> TestResult<DraftingProgramLimits> {
    Ok(DraftingProgramLimits::new(
        limits(10000, 10000, 10000)?,
        100,
        100,
        1000,
    ))
}
#[test]
fn semantic_ordinals_require_complete_independent_original_coverage_after_whole_bounds(
) -> TestResult {
    use DraftingProgramAdmissionRefusal as R;
    use DraftingProgramStep::{Scalar as C, Semantic as S};
    let root = root()?;
    let source = Source::new(&root);
    let context = scope(&root)?
        .bind_drafting_context(&source, limits(10000, 10000, 10000)?)
        .map_err(|e| e.to_string())?;
    let selected = policy()?;
    for (steps, expected) in [
        (
            vec![C { ordinal: 1 }, S { ordinal: 0 }],
            R::SemanticOrder {
                position: 2,
                expected: Some(1),
                requested: 0,
            },
        ),
        (
            vec![S { ordinal: 2 }, C { ordinal: 0 }],
            R::SemanticOrder {
                position: 1,
                expected: Some(1),
                requested: 2,
            },
        ),
        (
            vec![C { ordinal: 0 }, S { ordinal: 0 }],
            R::ScalarOrder {
                position: 1,
                expected: Some(1),
                requested: 0,
            },
        ),
        (
            vec![C { ordinal: 1 }, S { ordinal: 1 }, S { ordinal: 1 }],
            R::SemanticOrder {
                position: 3,
                expected: Some(2),
                requested: 1,
            },
        ),
        (
            vec![
                C { ordinal: 1 },
                S { ordinal: 1 },
                S { ordinal: 2 },
                S { ordinal: 3 },
            ],
            R::SemanticOrder {
                position: 4,
                expected: None,
                requested: 3,
            },
        ),
        (
            vec![C { ordinal: 1 }, S { ordinal: 1 }],
            R::MissingSemantics {
                required: 2,
                provided: 1,
            },
        ),
        (
            vec![],
            R::MissingScalars {
                required: 1,
                provided: 0,
            },
        ),
    ] {
        let raw = DraftingProgramInputs::new(context, steps);
        source.trace.set(0);
        let error = raw
            .admit(&selected)
            .err()
            .ok_or("whole original coverage refusal")?;
        assert_eq!(error.refusal(), expected);
        assert!(std::ptr::eq(error.inputs(), &raw) && std::ptr::eq(error.limits(), &selected));
        assert_eq!(source.trace.get(), 2); // fresh held content, no source/geometry queries
    }
    let raw = DraftingProgramInputs::new(context, vec![S { ordinal: 0 }]);
    // Four canonical entries,two semantic members,then one button input precede this field.
    let short_content = DraftingProgramLimits::new(limits(7, 10000, 10000)?, 0, 0, 0);
    source.trace.set(0);
    let error = raw
        .admit(&short_content)
        .err()
        .ok_or("unused actual semantic body charged first")?;
    assert!(
        matches!(error.refusal(),R::Resources(DesignResourceLimitError::Count {
        location:DesignResourceLocation {field:"semantic.logical.inputs",owner:Some(owner),..},..
    }) if owner==id(80)),
        "{:?}",
        error.refusal()
    );
    assert_eq!(source.trace.get(), 0);
    let short_shape = DraftingProgramLimits::new(limits(10000, 10000, 10000)?, 0, 0, 0);
    assert_eq!(
        raw.admit(&short_shape)
            .err()
            .ok_or("whole shape before bad ordinal")?
            .refusal(),
        R::Expressions {
            allowed: 0,
            required: 1
        }
    );
    let empty = super::root()?;
    let empty_source = Source::new(&empty);
    let context = scope(&empty)?
        .bind_drafting_context(&empty_source, limits(10000, 10000, 10000)?)
        .map_err(|e| e.to_string())?;
    let raw = DraftingProgramInputs::new(context, vec![S { ordinal: 1 }]);
    assert_eq!(
        raw.admit(&selected)
            .err()
            .ok_or("no implicit semantic cohort")?
            .refusal(),
        R::SemanticOrder {
            position: 1,
            expected: None,
            requested: 1
        }
    );
    Ok(())
}
#[test]
fn semantic_views_keep_actual_root_members_and_scalar_name_order_without_new_arenas() -> TestResult
{
    let root = root()?;
    let source = Source::new(&root);
    let context = scope(&root)?
        .bind_drafting_context(&source, limits(10000, 10000, 10000)?)
        .map_err(|e| e.to_string())?;
    let point = PointConstruction::new(
        id(100),
        FormulaExpressionDocument::new("private_width".into())?,
        FormulaExpressionDocument::new("0 mm".into())?,
    );
    let raw = DraftingProgramInputs::new(
        context,
        vec![
            DraftingProgramStep::Semantic { ordinal: 1 },
            DraftingProgramStep::Scalar { ordinal: 1 },
            DraftingProgramStep::Drafting(DraftingOperationInput::ConstructPoint(&point)),
            DraftingProgramStep::Semantic { ordinal: 2 },
        ],
    );
    assert!(std::ptr::eq(raw.semantic_recipe(), root.semantic_recipe()));
    let selected = policy()?;
    let admitted = raw.admit(&selected).map_err(|e| e.to_string())?;
    let shape = admitted.shape();
    assert_eq!(
        (
            shape.step_count(),
            shape.statement_count(),
            shape.semantic_operation_count()
        ),
        (4, 1, 2)
    );
    assert_eq!(
        (
            shape.expression_count(),
            shape.node_count(),
            shape.argument_count(),
            shape.retained_record_bound()
        ),
        (3, 3, 2, 14)
    );
    let short = DraftingProgramLimits::new(selected.content(), 3, 3, 13);
    assert_eq!(
        raw.admit(&short)
            .err()
            .ok_or("all mixed slots bounded")?
            .refusal(),
        DraftingProgramAdmissionRefusal::RetainedRecords {
            allowed: 13,
            required: 14
        }
    );
    source.trace.set(0);
    let program = admitted.normalized().map_err(|e| e.to_string())?;
    assert_eq!(source.trace.get(), 0);
    assert_eq!(
        program
            .operations()
            .iter()
            .map(Option::is_some)
            .collect::<Vec<_>>(),
        [false, false, true, false]
    );
    for (index, ordinal) in [(0, 1), (3, 2)] {
        let view = program.steps().nth(index).ok_or("actual semantic step")?;
        let original = root
            .semantic_recipe()
            .operations()
            .get(ordinal - 1)
            .ok_or("Root semantic member")?;
        assert_eq!(view.position(), index + 1);
        assert!(
            matches!(view.kind(),DraftingProgramStepKind::Semantic {ordinal:n,operation}
            if n==ordinal && std::ptr::eq(operation, original))
        );
        assert!(!format!("{view:?}").contains("policy_a"));
    }
    assert!(matches!(
        program
            .steps()
            .next_back()
            .ok_or("last original step")?
            .kind(),
        DraftingProgramStepKind::Semantic { ordinal: 2, .. }
    ));
    let mut meter = DesignResourceMeter::new(selected.content());
    source.trace.set(0);
    program.charge_resources(&mut meter)?;
    let retained = meter.finish()?;
    let mut meter = DesignResourceMeter::new(selected.content());
    source.trace.set(0);
    raw.charge_resources(&mut meter)?;
    // One scalar record,one scalar node,four slots,two native nodes;semantic bodies aren't cloned.
    assert_eq!(
        retained.collection_entries() - meter.finish()?.collection_entries(),
        8
    );
    let names = DraftingProgramNameInputs::new(&program, vec![], vec![]);
    let np = DraftingProgramNameLimits::new(selected.content(), 64, 128);
    source.trace.set(0);
    let work = names.admit_name_work(&np).map_err(|e| e.to_string())?;
    let positions = work.bind_output_positions().map_err(|e| e.to_string())?;
    let roles = positions.bind_role_sources().map_err(|e| e.to_string())?;
    let initial = roles.bind_initial_sources().map_err(|e| e.to_string())?;
    let namespace = initial
        .bind_initial_namespace()
        .map_err(|e| e.to_string())?;
    let op = DraftingProgramOrderLimits::new(3, 3, 14);
    source.trace.set(0);
    let order = namespace.check_order(&op).map_err(|e| e.to_string())?;
    assert_eq!(source.trace.get(), 0);
    for (index, ordinal) in [(0, 1), (3, 2)] {
        let step = order
            .steps()
            .get(index)
            .ok_or("original semantic order slot")?;
        let original = root
            .semantic_recipe()
            .operations()
            .get(ordinal - 1)
            .ok_or("Root semantic member")?;
        assert!(
            matches!(step.kind(),DraftingProgramOrderProofKind::Semantic {ordinal:n,operation}
            if *n==ordinal && std::ptr::eq(*operation,original))
        );
    }
    assert_eq!(
        order
            .dependencies()
            .map(|u| (u.position(), u.declaration().name()))
            .collect::<Vec<_>>(),
        [(3, "private_width")]
    );
    Ok(())
}
#[test]
fn semantic_steps_refuse_geometry_names_initials_and_creators_before_provider_lookup() -> TestResult
{
    for mode in 0..4 {
        let root = root()?;
        let source = Source::new(&root);
        let context = scope(&root)?
            .bind_drafting_context(&source, limits(10000, 10000, 10000)?)
            .map_err(|e| e.to_string())?;
        let consumer = LineConstruction::new(
            id(100),
            PointRef::new(id(70), LocalTag::FIRST),
            PointRef::new(id(900), LocalTag::FIRST),
        )?;
        let duplicate = PointConstruction::new(
            id(70),
            FormulaExpressionDocument::new("1 mm".into())?,
            FormulaExpressionDocument::new("0 mm".into())?,
        );
        let mut steps = vec![
            DraftingProgramStep::Scalar { ordinal: 1 },
            DraftingProgramStep::Semantic { ordinal: 1 },
            DraftingProgramStep::Drafting(DraftingOperationInput::DrawLine(&consumer)),
            DraftingProgramStep::Semantic { ordinal: 2 },
        ];
        if mode == 1 {
            steps.push(DraftingProgramStep::Drafting(
                DraftingOperationInput::ConstructPoint(&duplicate),
            ));
        }
        let raw = DraftingProgramInputs::new(context, steps);
        let pp = policy()?;
        let program = raw
            .admit(&pp)
            .map_err(|e| e.to_string())?
            .normalized()
            .map_err(|e| e.to_string())?;
        let outputs = if mode == 3 {
            vec![DraftingOutputName::new(
                2,
                MachineToken::new("private_edge")?,
                DraftingGeometryNameSource::Line(&consumer),
            )]
        } else {
            vec![]
        };
        let initial_name = MachineToken::new("private_point")?;
        let initials = if mode == 2 {
            vec![
                FormulaDeclaration::point(&initial_name, PointRef::new(id(70), LocalTag::FIRST))
                    .try_into()
                    .map_err(|_| "geometry initial")?,
            ]
        } else {
            vec![]
        };
        let names = DraftingProgramNameInputs::new(&program, initials, outputs);
        let np = DraftingProgramNameLimits::new(pp.content(), 64, 128);
        source.trace.set(0);
        let work = names.admit_name_work(&np).map_err(|e| e.to_string())?;
        if mode == 3 {
            source.trace.set(0);
            let error = work
                .bind_output_positions()
                .err()
                .ok_or("semantic has no named geometry")?;
            assert_eq!(
                error.refusal(),
                DraftingOutputPositionRefusal::SemanticStep { ordinal: 1 }
            );
            assert_eq!(source.trace.get(), 0);
            continue;
        }
        let positions = work.bind_output_positions().map_err(|e| e.to_string())?;
        let roles = positions.bind_role_sources().map_err(|e| e.to_string())?;
        source.trace.set(0);
        if mode == 2 {
            let error = roles
                .bind_initial_sources()
                .err()
                .ok_or("no semantic ID as existing geometry")?;
            assert!(
                matches!(error.refusal(),DraftingInitialRefusal::PlannedCreator {index:0,creator,position:2} if *creator==id(70))
            );
            assert_eq!(source.trace.get(), 0);
            continue;
        }
        let initial = roles.bind_initial_sources().map_err(|e| e.to_string())?;
        let namespace = initial
            .bind_initial_namespace()
            .map_err(|e| e.to_string())?;
        let sources = namespace
            .check_root_sources(FormulaSourceCheckLimits::new(0, 0, 0, 0))
            .map_err(|e| e.to_string())?;
        let dependencies = sources
            .proof()
            .prepare_dependencies(FormulaSourceDependencyLimits::new(0, 0, 1))
            .map_err(|e| e.to_string())?;
        let graph = dependencies
            .admit_acyclic(FormulaSourceCycleLimits::new(0, 0))
            .map_err(|e| e.to_string())?;
        let op = DraftingProgramOrderLimits::new(100, 100, 1000);
        let order = namespace.check_order(&op).map_err(|e| e.to_string())?;
        let inputs = sources
            .dependency_inputs(&order, &graph)
            .map_err(|e| e.to_string())?;
        let cp = DraftingProgramCanonicalLimits::new(100, 100);
        let bound = inputs
            .bind_canonical_inputs(&cp)
            .map_err(|e| e.to_string())?;
        source.trace.set(0);
        let request = bound
            .geometry_requests()
            .next()
            .ok_or("original native field")?;
        assert_eq!(request.occurrence().position(), Some(3));
        match request.target() {
            Err(DraftingProgramGeometryCreatorRefusal::SemanticCreator { step }) if mode == 0 => {
                assert_eq!(step.position(), 2);
                let original = root
                    .semantic_recipe()
                    .operations()
                    .first()
                    .ok_or("first Root semantic member")?;
                assert!(
                    matches!(step.kind(),DraftingProgramStepKind::Semantic {ordinal:1,operation}
                    if std::ptr::eq(operation,original))
                );
            }
            Err(DraftingProgramGeometryCreatorRefusal::Ambiguous { first, second })
                if mode == 1 =>
            {
                assert_eq!((first.position(), second.position()), (2, 5));
            }
            other => return Err(format!("original semantic creator refusal:{other:?}").into()),
        }
        assert_eq!(source.trace.get(), 0);
    }
    Ok(())
}
