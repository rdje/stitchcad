//! Random-access iteration still borrows actual scalar and native arena owners.
use super::{limits, root_with_recipe, scope, Source, TestResult};
use sc_core::{
    construction::{
        DraftingContextSource, DraftingOperationInput, DraftingProgramInputs,
        DraftingProgramLimits, DraftingProgramStep, DraftingProgramStepKind,
        NormalizedDraftingOperation, PointConstruction, PointCoordinate,
    },
    design::*,
    ontology::EntityId,
    recipe::FormulaExpressionDocument,
};

#[test]
fn forward_nth_reverse_and_interleaved_views_keep_original_scalar_and_native_owners() -> TestResult
{
    let text = "let private_width:length=2.5 cm\n  assert closure:eps_geo=private_width==25 mm";
    let root = root_with_recipe(text)?;
    let source = Source::new(&root);
    let context = scope(&root)?
        .bind_drafting_context(&source, limits(1000, 1000, 1000)?)
        .map_err(|e| e.to_string())?;
    let point = PointConstruction::new(
        EntityId::from_bits(100),
        FormulaExpressionDocument::new("private_width".into())?,
        FormulaExpressionDocument::new("0 mm".into())?,
    );
    let inputs = DraftingProgramInputs::new(
        context,
        vec![
            DraftingProgramStep::Scalar { ordinal: 1 },
            DraftingProgramStep::Drafting(DraftingOperationInput::ConstructPoint(&point)),
            DraftingProgramStep::Scalar { ordinal: 2 },
        ],
    );
    let policy = DraftingProgramLimits::new(limits(1000, 1000, 1000)?, 5, 5, 20);
    let program = inputs
        .admit(&policy)
        .map_err(|e| e.to_string())?
        .normalized()
        .map_err(|e| e.to_string())?;
    source.trace.set(0);
    let [first_statement, second_statement] = program.recipe().normalized().statements() else {
        return Err("complete actual scalar owners".into());
    };
    let first = program.steps().next().ok_or("first mixed step")?;
    assert_eq!(first.position(), 1);
    assert!(
        matches!(first.kind(),DraftingProgramStepKind::Scalar{ordinal:1,statement}
        if std::ptr::eq(statement,first_statement))
    );
    assert!(!format!("{first:?}").contains("private_width"));
    // nth skips original requests; it must not leave a hidden scalar cursor at statement1.
    let third = program.steps().nth(2).ok_or("third mixed position")?;
    assert_eq!(third.position(), 3);
    let DraftingProgramStepKind::Scalar { ordinal, statement } = third.kind() else {
        return Err("original second scalar at mixed position3".into());
    };
    assert_eq!(ordinal, 2);
    assert!(std::ptr::eq(statement, second_statement));
    let span = statement.span();
    assert_eq!(
        span.start(),
        text.find("assert")
            .ok_or("original assertion source offset")?
    );
    assert_eq!(
        text.get(span.start()..span.end()),
        Some("assert closure:eps_geo=private_width==25 mm")
    );
    let second = program.steps().nth(1).ok_or("native mixed position2")?;
    let DraftingProgramStepKind::Drafting(actual) = second.kind() else {
        return Err("native arena owner at mixed position2".into());
    };
    let expected = program
        .operations()
        .get(1)
        .and_then(Option::as_ref)
        .ok_or("actual native slot")?;
    assert!(std::ptr::eq(actual, expected));
    let NormalizedDraftingOperation::Point(original) = actual else {
        return Err("actual original point arena".into());
    };
    assert!(std::ptr::eq(original.point(), &point));
    let reverse: Vec<_> = program.steps().rev().map(|step| step.position()).collect();
    assert_eq!(reverse, [3, 2, 1]);
    let skipped: Vec<_> = program
        .steps()
        .skip(2)
        .map(|step| step.position())
        .collect();
    assert_eq!(skipped, [3]);
    let mut interleaved = program.steps();
    assert_eq!(interleaved.len(), 3);
    assert_eq!(interleaved.next_back().ok_or("last step")?.position(), 3);
    assert_eq!(interleaved.len(), 2);
    assert_eq!(
        interleaved
            .nth(1)
            .ok_or("remaining native step")?
            .position(),
        2
    );
    assert_eq!(interleaved.len(), 0);
    assert!(interleaved.next().is_none() && interleaved.next_back().is_none());
    assert_eq!(source.trace.get(), 0);
    // Independent record population:3 raw slots +2 normalized statement slots +3 scalar
    // nodes +3 aligned native slots +2 Point nodes. Borrowed Root/provider data is separate.
    let record_text = [PointCoordinate::X, PointCoordinate::Y]
        .into_iter()
        .map(|coordinate| {
            let doc = point.coordinate(coordinate);
            doc.source().len() + doc.canonical().as_str().len()
        })
        .sum::<usize>();
    let mut records = DesignResourceMeter::new(limits(13, record_text, 1)?);
    program.charge_record_resources(&mut records)?;
    let record_demand = records.finish()?;
    assert_eq!(
        (
            record_demand.collection_entries(),
            record_demand.text_bytes(),
            record_demand.counter_bits()
        ),
        (13, record_text, 0)
    );
    assert_eq!(source.trace.get(), 0);
    let mut short = DesignResourceMeter::new(limits(12, record_text, 1)?);
    let expected = DesignResourceLimitError::Count {
        unit: DesignResourceCount::CollectionEntries,
        maximum: 12,
        consumed: 12,
        requested: 1,
        location: DesignResourceLocation {
            scope: DesignResourceScope::CommandState,
            owner: Some(EntityId::from_bits(100)),
            field: "drafting.normalized.point.y.nodes",
            index: Some(1),
        },
    };
    assert_eq!(program.charge_record_resources(&mut short), Err(expected));
    assert_eq!(short.finish(), Err(expected));
    assert_eq!(source.trace.get(), 0);
    // Preserve the original full producer's one Root/provider walk and complete demand.
    let mut whole = DesignResourceMeter::new(limits(10000, 10000, 10000)?);
    program.charge_resources(&mut whole)?;
    let whole = whole.finish()?;
    source.trace.set(0);
    let mut containing = DesignResourceMeter::new(limits(10000, 10000, 10000)?);
    charge_design_input_resources(&root, &mut containing)?;
    source.charge_resources(&mut containing)?;
    let containing = containing.finish()?;
    assert_eq!(
        (
            whole.collection_entries(),
            whole.text_bytes(),
            whole.counter_bits()
        ),
        (
            containing.collection_entries() + 13,
            containing.text_bytes() + record_text,
            containing.counter_bits()
        )
    );
    Ok(())
}
