//! Original record-only resource composition without Root/source callbacks or order/value checks.
use super::{limits, root_with_recipe, scope, Source, TestResult};
use sc_core::{construction::*, design::*, ontology::EntityId, recipe::FormulaExpressionDocument};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn native() -> TestResult<(PointConstruction, usize)> {
    let x = FormulaExpressionDocument::new("private_width".into())?;
    let y = FormulaExpressionDocument::new("1 mm/0".into())?;
    let text = x.source().len()
        + x.canonical().as_str().len()
        + y.source().len()
        + y.canonical().as_str().len();
    Ok((PointConstruction::new(id(100), x, y), text))
}
fn at(owner: u128, field: &'static str) -> DesignResourceLocation {
    DesignResourceLocation {
        scope: DesignResourceScope::CommandState,
        owner: Some(id(owner)),
        field,
        index: None,
    }
}
#[test]
fn original_mixed_records_and_repeated_native_fields_compose_without_root_or_provider_credit(
) -> TestResult {
    let root = root_with_recipe("let width:length=1 mm")?;
    let source = Source::new(&root);
    let context = scope(&root)?
        .bind_drafting_context(&source, limits(1000, 10000, 1000)?)
        .map_err(|e| e.to_string())?;
    let (point, text) = native()?;
    let program = DraftingProgramInputs::new(
        context,
        vec![
            DraftingProgramStep::Scalar { ordinal: 0 },
            DraftingProgramStep::Drafting(DraftingOperationInput::ConstructPoint(&point)),
            DraftingProgramStep::Semantic { ordinal: 999 },
            DraftingProgramStep::Drafting(DraftingOperationInput::ConstructPoint(&point)),
        ],
    );
    source.trace.set(0);
    // Root/snapshot revisions need three bits apiece;one bit cannot admit either whole body.
    let mut meter = DesignResourceMeter::new(limits(4, 2 * text, 1)?);
    program.charge_record_resources(&mut meter)?;
    let demand = meter.finish()?;
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (4, 2 * text, 0)
    );
    assert_eq!(source.trace.get(), 0);
    assert_eq!(program.steps().len(), 4);
    let Some(DraftingProgramStep::Drafting(DraftingOperationInput::ConstructPoint(original))) =
        program.steps().get(1)
    else {
        return Err("original native loan".into());
    };
    assert!(core::ptr::eq(*original, &point));
    let mut meter = DesignResourceMeter::new(limits(6, 2 * text + 3, 1)?);
    meter.charge_entries(2, at(1, "prior"))?;
    meter.charge_text("old", at(1, "prior"))?;
    meter.charge_counter_bits(1, at(1, "prior"))?;
    program.charge_record_resources(&mut meter)?;
    let demand = meter.finish()?;
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (6, 2 * text + 3, 1)
    );
    assert_eq!(source.trace.get(), 0);
    Ok(())
}
#[test]
fn complete_step_vector_and_original_native_text_refuse_before_callbacks_or_later_fields(
) -> TestResult {
    let root = root_with_recipe("let width:length=1 mm")?;
    let source = Source::new(&root);
    let context = scope(&root)?
        .bind_drafting_context(&source, limits(1000, 10000, 1000)?)
        .map_err(|e| e.to_string())?;
    let (point, text) = native()?;
    let program = DraftingProgramInputs::new(
        context,
        vec![
            DraftingProgramStep::Scalar { ordinal: 0 },
            DraftingProgramStep::Drafting(DraftingOperationInput::ConstructPoint(&point)),
        ],
    );
    source.trace.set(0);
    for (entries, text, expected) in [
        (
            1,
            text,
            DesignResourceLimitError::Count {
                unit: DesignResourceCount::CollectionEntries,
                maximum: 1,
                consumed: 0,
                requested: 2,
                location: at(1, "drafting.program.steps"),
            },
        ),
        (
            2,
            12,
            DesignResourceLimitError::Count {
                unit: DesignResourceCount::TextBytes,
                maximum: 12,
                consumed: 0,
                requested: 13,
                location: at(100, "drafting.point.x.source"),
            },
        ),
    ] {
        let mut meter = DesignResourceMeter::new(limits(entries, text, 1)?);
        assert_eq!(program.charge_record_resources(&mut meter), Err(expected));
        assert_eq!(meter.finish(), Err(expected));
        assert_eq!(source.trace.get(), 0);
    }
    Ok(())
}
#[test]
fn empty_and_nonempty_record_walks_preserve_the_original_prior_sink_failure() -> TestResult {
    let root = root_with_recipe("let width:length=1 mm")?;
    let source = Source::new(&root);
    let context = scope(&root)?
        .bind_drafting_context(&source, limits(1000, 10000, 1000)?)
        .map_err(|e| e.to_string())?;
    let empty = DraftingProgramInputs::new(context, vec![]);
    let ordinal = DraftingProgramInputs::new(
        context,
        vec![DraftingProgramStep::Semantic { ordinal: 999 }],
    );
    source.trace.set(0);
    let mut meter = DesignResourceMeter::new(limits(1, 1, 1)?);
    empty.charge_record_resources(&mut meter)?;
    let demand = meter.finish()?;
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (0, 0, 0)
    );
    for program in [&empty, &ordinal] {
        let mut meter = DesignResourceMeter::new(limits(1, 1, 1)?);
        let expected = meter
            .charge_entries(2, at(1, "original.first"))
            .err()
            .ok_or("original first refusal")?;
        assert_eq!(program.charge_record_resources(&mut meter), Err(expected));
        assert_eq!(meter.finish(), Err(expected));
    }
    assert_eq!(source.trace.get(), 0);
    Ok(())
}
