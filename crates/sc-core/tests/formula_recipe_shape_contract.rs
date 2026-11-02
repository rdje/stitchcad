//! Factory population metadata is complete, immutable and distinct from evaluation or admission.
use core::num::{NonZeroU64, NonZeroUsize};
use sc_core::{
    design::{charge_recipe_document_resources, DesignResourceLimits, DesignResourceMeter},
    recipe::{FormulaRecipe, FormulaRecipeDocument as Document},
};
use std::error::Error;

#[test]
fn counts_all_statements_assertion_operands_calls_and_untaken_division_without_evaluation(
) -> Result<(), Box<dyn Error>> {
    let source = "let confidential:count=if(1==2,1/0,5)\nassert closure:eps_num=(confidential+6)==missing(7,8)";
    let document = Document::new(source.into())?;
    let shape = document.shape();
    assert_eq!(
        (
            shape.statement_count(),
            shape.expression_count(),
            shape.node_count()
        ),
        (2, 3, 14)
    );
    let normalized = document.normalized()?;
    assert_eq!(normalized.statements().len(), shape.statement_count());
    assert_eq!(&normalized.canonical_form(), document.canonical());
    assert_eq!(document.source(), source);
    assert!(!format!("{document:?}").contains("confidential"));
    assert_eq!(document.clone().shape(), shape);
    Ok(())
}

#[test]
fn empty_and_alias_documents_keep_original_text_identity_and_zero_or_equal_shapes(
) -> Result<(), Box<dyn Error>> {
    for source in ["", " \n\t"] {
        let doc = Document::new(source.into())?;
        assert_eq!(
            (
                doc.shape().statement_count(),
                doc.shape().expression_count(),
                doc.shape().node_count()
            ),
            (0, 0, 0)
        );
        assert_eq!(doc.source(), source);
        assert_eq!(doc.canonical().as_str(), "(recipe)");
    }
    let first = Document::new("let width:length=2.5 cm".into())?;
    let alias = Document::new(" let width : length = ((25 mm)) \n".into())?;
    assert_ne!(first, alias);
    assert_eq!(first.canonical(), alias.canonical());
    assert_eq!(first.shape(), alias.shape());
    assert_eq!(
        (
            first.shape().statement_count(),
            first.shape().expression_count(),
            first.shape().node_count()
        ),
        (1, 1, 1)
    );
    Ok(())
}

#[test]
fn fixed_factory_metadata_changes_no_logical_recipe_buffer_charges() -> Result<(), Box<dyn Error>> {
    let doc = Document::new("let private_width:length=25 mm".into())?;
    let bytes = doc.source().len() + doc.canonical().as_str().len();
    let mut meter = DesignResourceMeter::new(DesignResourceLimits::new(
        NonZeroUsize::MIN,
        NonZeroUsize::new(bytes).ok_or("positive text allowance")?,
        NonZeroU64::MIN,
    ));
    let before = doc.clone();
    charge_recipe_document_resources(&doc, &mut meter)?;
    let demand = meter.finish()?;
    assert_eq!(demand.text_bytes(), bytes);
    assert_eq!((demand.collection_entries(), demand.counter_bits()), (0, 0));
    assert_eq!(doc, before);
    Ok(())
}

#[test]
fn existing_maximum_statement_bound_counts_both_operands_without_a_new_aggregate_cap(
) -> Result<(), Box<dyn Error>> {
    let source = "assert repeated:eps_num=1==1\n".repeat(FormulaRecipe::MAX_STATEMENTS);
    let document = Document::new(source)?;
    assert_eq!(document.shape().statement_count(), 4096);
    assert_eq!(document.shape().expression_count(), 8192);
    assert_eq!(document.shape().node_count(), 8192);
    Ok(())
}
