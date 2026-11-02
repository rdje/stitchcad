//! Raw mixed ownership preserves sources and invalid data before whole admission.
use super::{limits, root_with_recipe, scope, Source, TestResult};
use sc_core::{
    construction::{
        DraftingOperationInput, DraftingProgramInputs, DraftingProgramStep as Step,
        PointConstruction, PointCoordinate,
    },
    ontology::EntityId,
    recipe::FormulaExpressionDocument,
};

#[test]
fn mixed_owner_keeps_actual_root_recipe_context_native_sources_and_distinct_ordinals() -> TestResult
{
    let source_text = "let private_width:length=1 mm/0\nassert closure:eps_geo=private_width==2 mm";
    let root = root_with_recipe(source_text)?;
    let source = Source::new(&root);
    let context = scope(&root)?
        .bind_drafting_context(&source, limits(1000, 1000, 1000)?)
        .map_err(|e| e.to_string())?;
    let point = PointConstruction::new(
        EntityId::from_bits(10),
        FormulaExpressionDocument::new("private_width".into())?,
        FormulaExpressionDocument::new("1 mm/0".into())?,
    );
    let steps = vec![
        Step::Scalar { ordinal: 1 },
        Step::Drafting(DraftingOperationInput::ConstructPoint(&point)),
        Step::Scalar { ordinal: 2 },
    ];
    let original_buffer = steps.as_ptr();
    let trace = source.trace.get();
    let inputs = DraftingProgramInputs::new(context, steps);
    assert_eq!(inputs.steps().as_ptr(), original_buffer);
    assert_eq!(inputs.steps().len(), 3);
    assert!(std::ptr::eq(inputs.context().scope().root(), &root));
    assert!(std::ptr::eq(inputs.context().source(), &source));
    assert!(inputs.context().frame().is_same_record(context.frame()));
    assert!(inputs
        .context()
        .snapshot()
        .is_same_record(context.snapshot()));
    assert!(std::ptr::eq(inputs.recipe(), root.recipe()));
    assert_eq!(inputs.recipe().source(), source_text);
    assert_eq!(inputs.recipe().shape().statement_count(), 2);
    // Original scalar 2 occurs at mixed position 3; neither its ordinal nor span is replaced.
    let [Step::Scalar { ordinal: 1 }, Step::Drafting(DraftingOperationInput::ConstructPoint(original)), Step::Scalar { ordinal: 2 }] =
        inputs.steps()
    else {
        return Err("original mixed sequence and separate scalar ordinals".into());
    };
    assert!(std::ptr::eq(*original, &point));
    assert_eq!(
        original.coordinate(PointCoordinate::X).source(),
        "private_width"
    );
    assert_eq!(source.trace.get(), trace);
    assert!(!format!("{inputs:?}").contains("private_width"));
    Ok(())
}

#[test]
fn empty_and_invalid_sequences_survive_raw_construction_without_child_work() -> TestResult {
    let root = root_with_recipe("let width:length=1 mm")?;
    let source = Source::new(&root);
    let context = scope(&root)?
        .bind_drafting_context(&source, limits(1000, 1000, 1000)?)
        .map_err(|e| e.to_string())?;
    let trace = source.trace.get();
    let empty = DraftingProgramInputs::new(context, vec![]);
    assert!(empty.steps().is_empty());
    assert!(std::ptr::eq(empty.recipe(), root.recipe()));
    let invalid = DraftingProgramInputs::new(
        context,
        vec![
            Step::Scalar { ordinal: 0 },
            Step::Scalar { ordinal: 2 },
            Step::Scalar { ordinal: 2 },
            Step::Scalar {
                ordinal: usize::MAX,
            },
        ],
    );
    let ordinals: Vec<_> = invalid
        .steps()
        .iter()
        .map(|step| match step {
            Step::Scalar { ordinal } => *ordinal,
            Step::Drafting(_) => unreachable!("explicit scalar test requests"),
        })
        .collect();
    assert_eq!(ordinals, [0, 2, 2, usize::MAX]);
    assert_eq!(source.trace.get(), trace);
    Ok(())
}
