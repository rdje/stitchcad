//! Whole populations and content precede original scalar coverage/order; no values are evaluated.
use super::{limits, root_with_recipe, root_with_sources, scope, Source, TestResult};
use sc_core::{
    construction::{
        CenterArcArgument, CenterArcConstruction, DraftingOperationInput as Input,
        DraftingProgramAdmissionRefusal as Refusal, DraftingProgramInputs as Inputs,
        DraftingProgramLimits as Limits, DraftingProgramStep as Step, PointConstruction,
        PointCoordinate, SlashSpreadConstruction, SlashSpreadSide, WalkBalanceNotch,
        WalkBalancePair, WalkSeamEdit,
    },
    design::{
        DesignResourceCount, DesignResourceLimitError, DesignResourceLimits,
        DesignResourceLocation, DesignResourceScope,
    },
    ontology::{
        DirectedRange, Direction, EdgeAnchor, EdgeRange, EdgeRef, EntityId, LocalTag,
        NotchDimensionBindings, NotchProfileBindings, Param, PointRef, ProfileParameterRef,
    },
    recipe::{FormulaExpressionDocument, FormulaSource},
};
fn id(value: u128) -> EntityId {
    EntityId::from_bits(value)
}
fn doc(text: &str) -> TestResult<FormulaExpressionDocument> {
    Ok(FormulaExpressionDocument::new(text.into())?)
}
fn all() -> TestResult<DesignResourceLimits> {
    limits(usize::MAX, usize::MAX, u64::MAX)
}
fn range() -> DirectedRange {
    DirectedRange {
        range: EdgeRange::whole(EdgeRef::new(id(900), LocalTag::FIRST)),
        direction: Direction::Reversed,
    }
}
fn pair() -> WalkBalancePair {
    let parameter = |n| ProfileParameterRef::new(id(n));
    let representation = NotchProfileBindings {
        style: parameter(31),
        sample: NotchDimensionBindings {
            depth: parameter(32),
            width: parameter(33),
        },
        production: NotchDimensionBindings {
            depth: parameter(34),
            width: parameter(35),
        },
        encoding: parameter(36),
    };
    let mark = WalkBalanceNotch::new(
        EdgeAnchor {
            edge: range().range.edge(),
            param: Param::END,
        },
        representation,
    );
    WalkBalancePair::new(mark, mark)
}

#[test]
fn whole_admission_counts_all_three_cohorts_repeats_and_actual_retained_fields() -> TestResult {
    let root = root_with_sources(
        "let width:length=if(1==2,1 mm,2 mm)\nassert closure:eps_geo=width==2 mm",
        vec![
            FormulaSource::new(id(81), doc("if(1==2,1/0,5)")?),
            FormulaSource::new(id(82), doc("missing(7,8)")?),
        ],
    )?;
    let source = Source::new(&root);
    let context = scope(&root)?
        .bind_drafting_context(&source, all()?)
        .map_err(|e| e.to_string())?;
    let point = PointConstruction::new(id(100), doc("private_width")?, doc("1 mm/0")?);
    let arc = CenterArcConstruction::new(
        id(101),
        PointRef::new(id(901), LocalTag::FIRST),
        doc("5 mm")?,
        doc("0 deg")?,
        doc("90 deg")?,
    )?;
    let slash = SlashSpreadConstruction::separation(
        id(102),
        id(800),
        range(),
        SlashSpreadSide::Left,
        range(),
        doc("2 mm")?,
    )?;
    let walk = WalkSeamEdit::new(id(103), id(800), vec![pair(); 3]);
    let inputs = Inputs::new(
        context,
        vec![
            Step::Scalar { ordinal: 1 },
            Step::Drafting(Input::ConstructPoint(&point)),
            Step::Drafting(Input::DrawArc(&arc)),
            Step::Drafting(Input::SlashSpread(&slash)),
            Step::Drafting(Input::ConstructPoint(&point)),
            Step::Drafting(Input::WalkSeam(&walk)),
            Step::Scalar { ordinal: 2 },
        ],
    );
    let native_bytes: usize = [
        point.coordinate(PointCoordinate::X),
        point.coordinate(PointCoordinate::Y),
        arc.argument(CenterArcArgument::Radius),
        arc.argument(CenterArcArgument::StartAngle),
        arc.argument(CenterArcArgument::SweepAngle),
        slash.opening().document(),
        point.coordinate(PointCoordinate::X),
        point.coordinate(PointCoordinate::Y),
    ]
    .into_iter()
    .map(|d| d.source().len() + d.canonical().as_str().len())
    .sum();
    let root_demand = context.scope().demand();
    let context_demand = context.demand();
    let entries = root_demand.collection_entries() + context_demand.collection_entries() + 7 + 3;
    let bytes = root_demand.text_bytes() + context_demand.text_bytes() + native_bytes;
    let bits = root_demand.counter_bits() + context_demand.counter_bits();
    let policy = Limits::new(limits(entries, bytes, bits)?, 13, 31, 84);
    source.trace.set(0);
    let admitted = inputs.admit(&policy).map_err(|e| e.to_string())?;
    assert!(std::ptr::eq(admitted.inputs(), &inputs));
    assert!(std::ptr::eq(admitted.limits(), &policy));
    let shape = admitted.shape();
    assert_eq!(
        (
            shape.step_count(),
            shape.statement_count(),
            shape.source_count(),
            shape.argument_count()
        ),
        (7, 2, 2, 8)
    );
    // Independent hand-count: source bodies11 + scalar operands8 + native arguments12 =31 nodes.
    assert_eq!(
        (
            shape.expression_count(),
            shape.node_count(),
            shape.retained_record_bound()
        ),
        (13, 31, 84)
    );
    assert_eq!(
        (
            admitted.demand().collection_entries(),
            admitted.demand().text_bytes(),
            admitted.demand().counter_bits()
        ),
        (entries, bytes, bits)
    );
    assert_eq!(source.trace.get(), 2); // Fresh charge only; no semantic/currentness/geometry callback.
    assert!(!format!("{admitted:?}").contains("private_width"));
    source.trace.set(0);
    let normalized = admitted.normalized().map_err(|e| e.to_string())?;
    assert!(std::ptr::eq(normalized.admitted().inputs(), &inputs));
    assert!(std::ptr::eq(normalized.admitted().limits(), &policy));
    assert!(std::ptr::eq(normalized.sources().scope().root(), &root));
    assert!(std::ptr::eq(normalized.recipe().scope().root(), &root));
    let [first_source, second_source] = normalized.sources().normalized().sources() else {
        return Err("complete original unused source owners".into());
    };
    assert_eq!(
        (first_source.source().id(), second_source.source().id()),
        (id(81), id(82))
    );
    assert_eq!(normalized.sources().normalized().node_count(), 11);
    assert_eq!(
        &second_source.expression().canonical_form(),
        second_source.source().expression().canonical()
    );
    let [_, assertion] = normalized.recipe().normalized().statements() else {
        return Err("whole original recipe and assertion".into());
    };
    let span = assertion.span();
    assert_eq!(
        inputs.recipe().source().get(span.start()..span.end()),
        Some("assert closure:eps_geo=width==2 mm")
    );
    use sc_core::construction::NormalizedDraftingOperation as N;
    let [None, Some(N::Point(first)), Some(N::CenterArc(actual_arc)), Some(N::SlashSpread {
        input: actual_slash,
        opening,
    }), Some(N::Point(repeated)), Some(N::Other(Input::WalkSeam(actual_walk))), None] =
        normalized.operations()
    else {
        return Err("complete original mixed-position-aligned native arenas".into());
    };
    assert!(std::ptr::eq(first.point(), &point) && std::ptr::eq(repeated.point(), &point));
    assert!(!std::ptr::eq(
        first.coordinate(PointCoordinate::X),
        repeated.coordinate(PointCoordinate::X)
    ));
    assert_eq!(
        &first.coordinate(PointCoordinate::X).canonical_form(),
        point.coordinate(PointCoordinate::X).canonical()
    );
    assert!(std::ptr::eq(actual_arc.arc(), &arc));
    assert!(std::ptr::eq(*actual_slash, &slash));
    assert!(std::ptr::eq(opening.opening(), slash.opening()));
    assert!(std::ptr::eq(*actual_walk, &walk));
    assert_eq!(
        &opening.expression().canonical_form(),
        slash.opening().document().canonical()
    );
    assert_eq!(source.trace.get(), 0);
    assert!(!format!("{normalized:?}").contains("private_width"));
    for (expressions, nodes, records, expected) in [
        (
            12,
            0,
            0,
            Refusal::Expressions {
                allowed: 12,
                required: 13,
            },
        ),
        (
            13,
            30,
            0,
            Refusal::Nodes {
                allowed: 30,
                required: 31,
            },
        ),
        (
            13,
            31,
            83,
            Refusal::RetainedRecords {
                allowed: 83,
                required: 84,
            },
        ),
    ] {
        let insufficient = Limits::new(all()?, expressions, nodes, records);
        let failure = inputs
            .admit(&insufficient)
            .expect_err("complete normalization quota");
        assert_eq!(failure.refusal(), expected);
        assert!(std::ptr::eq(failure.inputs(), &inputs));
        assert!(std::ptr::eq(failure.limits(), &insufficient));
    }
    Ok(())
}

#[test]
fn fresh_root_context_and_mixed_vector_charges_precede_zero_quotas_and_bad_order() -> TestResult {
    let root = root_with_recipe("let width:length=1 mm")?;
    let source = Source::new(&root);
    let context = scope(&root)?
        .bind_drafting_context(&source, all()?)
        .map_err(|e| e.to_string())?;
    let inputs = Inputs::new(context, vec![Step::Scalar { ordinal: 0 }]);
    let root_bytes = context.scope().demand().text_bytes();
    let context_policy = Limits::new(limits(usize::MAX, root_bytes, u64::MAX)?, 0, 0, 0);
    source.trace.set(0);
    let failure = inputs
        .admit(&context_policy)
        .expect_err("cumulative context text");
    let expected = DesignResourceLimitError::Count {
        unit: DesignResourceCount::TextBytes,
        maximum: root_bytes,
        consumed: root_bytes,
        requested: source.frame.data().label.len(),
        location: DesignResourceLocation {
            scope: DesignResourceScope::Inputs,
            owner: Some(root.id()),
            field: "drafting.frame.label",
            index: None,
        },
    };
    assert_eq!(failure.refusal(), Refusal::Resources(expected));
    assert_eq!(
        std::error::Error::source(&failure).map(ToString::to_string),
        Some(expected.to_string())
    );
    assert_eq!(source.trace.get(), 2);
    let root_policy = Limits::new(limits(usize::MAX, 1, u64::MAX)?, 0, 0, 0);
    source.trace.set(0);
    let failure = inputs
        .admit(&root_policy)
        .expect_err("Root source before provider");
    assert!(matches!(
        failure.refusal(),
        Refusal::Resources(DesignResourceLimitError::Count {
            location: DesignResourceLocation {
                field: "recipe.source",
                ..
            },
            ..
        })
    ));
    assert_eq!(source.trace.get(), 0);
    // Deliberately faulty return discipline exercises the fresh whole-program sink fence.
    let mut ignoring = Source::new(&root);
    ignoring.ignore_refusal = true;
    let faulty_context = scope(&root)?
        .bind_drafting_context(&ignoring, all()?)
        .map_err(|e| e.to_string())?;
    let faulty_inputs = Inputs::new(faulty_context, vec![]);
    let root_entries = faulty_context.scope().demand().collection_entries();
    let too_few_entries = Limits::new(limits(root_entries + 2, usize::MAX, u64::MAX)?, 0, 0, 0);
    let failure = faulty_inputs
        .admit(&too_few_entries)
        .expect_err("ignored cache refusal remains first");
    assert!(
        matches!(failure.refusal(), Refusal::Resources(DesignResourceLimitError::Count {
        unit: DesignResourceCount::CollectionEntries, consumed, requested: 3,
        location: DesignResourceLocation { field: "drafting.cache", .. }, ..
    }) if consumed == root_entries)
    );
    let preceding =
        context.scope().demand().collection_entries() + context.demand().collection_entries();
    let vector_policy = Limits::new(limits(preceding, usize::MAX, u64::MAX)?, 0, 0, 0);
    let failure = inputs
        .admit(&vector_policy)
        .expect_err("complete mixed vector before order");
    assert!(
        matches!(failure.refusal(),Refusal::Resources(DesignResourceLimitError::Count {
        unit:DesignResourceCount::CollectionEntries,consumed,requested:1,
        location:DesignResourceLocation{field:"drafting.program.steps",owner:Some(owner),index:None,..},..
    }) if consumed==preceding && owner==root.id())
    );
    Ok(())
}

#[test]
fn original_scalar_coverage_and_relative_order_keep_actual_mixed_positions() -> TestResult {
    let root = root_with_recipe("let width:length=1 mm\nassert closure:eps_geo=width==1 mm")?;
    let source = Source::new(&root);
    let context = scope(&root)?
        .bind_drafting_context(&source, all()?)
        .map_err(|e| e.to_string())?;
    let point = PointConstruction::new(id(100), doc("1 mm/0")?, doc("missing_name")?);
    let policy = Limits::new(all()?, 100, 100, 1000);
    let s = |ordinal| Step::Scalar { ordinal };
    let bad = Inputs::new(
        context,
        vec![s(0), Step::Drafting(Input::ConstructPoint(&point))],
    );
    let no_expressions = Limits::new(all()?, 0, 0, 0);
    assert_eq!(
        bad.admit(&no_expressions)
            .expect_err("complete shape precedes bad order")
            .refusal(),
        Refusal::Expressions {
            allowed: 0,
            required: 5
        }
    );
    for (steps, expected) in [
        (
            vec![],
            Refusal::MissingScalars {
                required: 2,
                provided: 0,
            },
        ),
        (
            vec![s(1)],
            Refusal::MissingScalars {
                required: 2,
                provided: 1,
            },
        ),
        (
            vec![s(2), s(1)],
            Refusal::ScalarOrder {
                position: 1,
                expected: Some(1),
                requested: 2,
            },
        ),
        (
            vec![s(0)],
            Refusal::ScalarOrder {
                position: 1,
                expected: Some(1),
                requested: 0,
            },
        ),
        (
            vec![s(1), Step::Drafting(Input::ConstructPoint(&point)), s(1)],
            Refusal::ScalarOrder {
                position: 3,
                expected: Some(2),
                requested: 1,
            },
        ),
        (
            vec![s(1), s(2), s(2)],
            Refusal::ScalarOrder {
                position: 3,
                expected: None,
                requested: 2,
            },
        ),
        (
            vec![s(1), s(usize::MAX)],
            Refusal::ScalarOrder {
                position: 2,
                expected: Some(2),
                requested: usize::MAX,
            },
        ),
    ] {
        let inputs = Inputs::new(context, steps);
        assert_eq!(
            inputs
                .admit(&policy)
                .expect_err("whole original scalar order")
                .refusal(),
            expected
        );
    }
    let inputs = Inputs::new(
        context,
        vec![s(1), Step::Drafting(Input::ConstructPoint(&point)), s(2)],
    );
    inputs.admit(&policy).map_err(|e| e.to_string())?;
    Ok(())
}

#[test]
fn empty_program_admits_zero_normalization_but_keeps_complete_context_content() -> TestResult {
    let root = root_with_recipe("")?;
    let source = Source::new(&root);
    let context = scope(&root)?
        .bind_drafting_context(&source, all()?)
        .map_err(|e| e.to_string())?;
    let inputs = Inputs::new(context, vec![]);
    let policy = Limits::new(all()?, 0, 0, 0);
    let admitted = inputs.admit(&policy).map_err(|e| e.to_string())?;
    let shape = admitted.shape();
    assert_eq!(
        (
            shape.step_count(),
            shape.expression_count(),
            shape.node_count(),
            shape.retained_record_bound()
        ),
        (0, 0, 0, 0)
    );
    assert!(admitted.demand().collection_entries() >= context.demand().collection_entries());
    source.trace.set(0);
    let normalized = admitted.normalized().map_err(|e| e.to_string())?;
    assert!(normalized.operations().is_empty());
    assert!(normalized.sources().normalized().sources().is_empty());
    assert!(normalized.recipe().normalized().statements().is_empty());
    assert!(std::ptr::eq(
        normalized.admitted().inputs().recipe(),
        root.recipe()
    ));
    assert_eq!(source.trace.get(), 0);
    Ok(())
}

#[test]
fn normalization_keeps_wrong_static_kind_and_unknown_call_without_accepting_point() -> TestResult {
    let root = root_with_recipe("")?;
    let source = Source::new(&root);
    let context = scope(&root)?
        .bind_drafting_context(&source, all()?)
        .map_err(|e| e.to_string())?;
    let point = PointConstruction::new(id(100), doc("1 deg")?, doc("missing(1)")?);
    let inputs = Inputs::new(context, vec![Step::Drafting(Input::ConstructPoint(&point))]);
    let policy = Limits::new(all()?, 2, 3, 9);
    let admitted = inputs.admit(&policy).map_err(|e| e.to_string())?;
    source.trace.set(0);
    let program = admitted.normalized().map_err(|e| e.to_string())?;
    let [Some(sc_core::construction::NormalizedDraftingOperation::Point(actual))] =
        program.operations()
    else {
        return Err("actual complete normalized point arguments".into());
    };
    assert!(std::ptr::eq(actual.point(), &point));
    assert_eq!(actual.coordinate(PointCoordinate::X).node_count(), 1);
    assert_eq!(
        &actual.coordinate(PointCoordinate::X).canonical_form(),
        point.coordinate(PointCoordinate::X).canonical()
    );
    assert_eq!(actual.coordinate(PointCoordinate::Y).node_count(), 2);
    assert_eq!(source.trace.get(), 0);
    Ok(())
}
