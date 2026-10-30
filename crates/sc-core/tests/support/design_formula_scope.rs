//! Actual owner normalization and admission ordering,without numerical or static proof claims.
use super::{
    id,
    input_composition::{definition, Markers, MissingFixtureRecord},
};
use core::{
    cell::Cell,
    num::{NonZeroU64, NonZeroUsize},
};
use sc_core::{
    design::{
        charge_design_input_resources, DesignFormulaScopeRefusal, DesignHostEntity, DesignInputs,
        DesignInputsDefinition, DesignInputsError, DesignMetadata, DesignMetadataError,
        DesignMetadataHost, DesignReferenceHost, DesignResourceDemand, DesignResourceLimitError,
        DesignResourceLimits, DesignResourceLocation, DesignResourceMeter, DesignResourceScope,
        DesignResourceSink, DesignResourceSource, DesignSizeSetHost, DesignSizeSetReference,
        DesignSourceReferences,
    },
    ontology::EntityId,
    recipe::{
        FormulaRecipeDocument, FormulaSourceNormalizationLimits, FormulaSourceNormalizationRefusal,
    },
    value::{CanonicalInputs, LengthDeclaration, LengthValueError},
};
use std::rc::Rc;
// Neither Clone nor Debug:borrowing the actual owner must not require either host capability.
pub(super) struct ObservedHost {
    actual: Markers,
    semantic: Rc<Cell<usize>>,
    reject: Rc<Cell<bool>>,
}
impl DesignMetadataHost for ObservedHost {
    type Error = MissingFixtureRecord;
    fn entities(&self) -> Vec<DesignHostEntity> {
        self.semantic.set(self.semantic.get() + 1);
        self.actual.entities()
    }
    fn validate_current(&self, canonical: &CanonicalInputs) -> Result<(), Self::Error> {
        self.semantic.set(self.semantic.get() + 1);
        if self.reject.get() {
            Err(MissingFixtureRecord)
        } else {
            self.actual.validate_current(canonical)
        }
    }
    fn measurement_declaration<'a>(
        &'a self,
        measurement: EntityId,
        canonical: &'a CanonicalInputs,
    ) -> Result<&'a LengthDeclaration, Self::Error> {
        self.semantic.set(self.semantic.get() + 1);
        self.actual.measurement_declaration(measurement, canonical)
    }
}
impl DesignSizeSetHost for ObservedHost {
    fn validate_size_set(
        &self,
        reference: DesignSizeSetReference,
        canonical: &CanonicalInputs,
    ) -> Result<(), Self::Error> {
        self.actual.validate_size_set(reference, canonical)
    }
}
impl DesignReferenceHost for ObservedHost {
    fn validate_references(
        &self,
        references: DesignSourceReferences,
        canonical: &CanonicalInputs,
    ) -> Result<(), Self::Error> {
        self.actual.validate_references(references, canonical)
    }
}
impl DesignResourceSource for ObservedHost {
    fn charge_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        self.actual.charge_resources(sink)
    }
}
fn generous() -> DesignResourceLimits {
    DesignResourceLimits::new(
        NonZeroUsize::new(10000).expect("entries"),
        NonZeroUsize::new(10000).expect("text"),
        NonZeroU64::new(10000).expect("bits"),
    )
}
fn measured<H: DesignResourceSource>(root: &DesignInputs<H>) -> DesignResourceDemand {
    let mut meter = DesignResourceMeter::new(generous());
    charge_design_input_resources(root, &mut meter).expect("complete actual held content");
    meter.finish().expect("all actual content admitted")
}
fn exact(demand: DesignResourceDemand) -> DesignResourceLimits {
    DesignResourceLimits::new(
        NonZeroUsize::new(demand.collection_entries()).expect("entries"),
        NonZeroUsize::new(demand.text_bytes()).expect("text"),
        NonZeroU64::new(demand.counter_bits()).expect("bits"),
    )
}
pub(super) fn observed() -> (DesignInputs<ObservedHost>, Rc<Cell<usize>>, Rc<Cell<bool>>) {
    let supplied = definition();
    let semantic = Rc::new(Cell::new(0));
    let reject = Rc::new(Cell::new(false));
    let sources = DesignMetadata::new(
        ObservedHost {
            actual: Markers(supplied.sources.host().entities()),
            semantic: Rc::clone(&semantic),
            reject: Rc::clone(&reject),
        },
        supplied.structure.inputs(),
    )
    .expect("actual source metadata");
    let root = DesignInputs::new(DesignInputsDefinition {
        structure: supplied.structure,
        parameters: supplied.parameters,
        recipe: supplied.recipe,
        formula_sources: supplied.formula_sources,
        materials: supplied.materials,
        sources,
        references: supplied.references,
    })
    .expect("actual root");
    semantic.set(0);
    (root, semantic, reject)
}
#[test]
fn exact_complete_content_and_normalized_documents_pin_actual_nonclone_owner_without_values() {
    let (root, semantic, _) = observed();
    let demand = measured(&root);
    let limits = exact(demand);
    assert_eq!(
        semantic.get(),
        0,
        "resource walking is not semantic validation"
    );
    let scope = root
        .prepare_formula_scope(limits)
        .expect("exact full held content");
    let copied = scope;
    assert!(semantic.get() > 0);
    assert!(std::ptr::eq(copied.root(), &root));
    assert_eq!(scope.demand(), demand);
    assert_eq!(scope.limits(), limits);
    assert!(std::ptr::eq(scope.source_inputs().inputs(), root.inputs()));
    assert!(std::ptr::eq(
        scope.source_inputs().sources(),
        root.formula_sources()
    ));
    let normalized = scope
        .normalize_sources(FormulaSourceNormalizationLimits::new(1, 3, 7))
        .expect("complete actual source normalization");
    assert!(std::ptr::eq(normalized.scope().root(), &root));
    assert!(std::ptr::eq(
        normalized.normalized().inputs().inputs(),
        root.inputs()
    ));
    let source = normalized
        .normalized()
        .sources()
        .first()
        .expect("actual normalized source");
    assert!(std::ptr::eq(
        source.source(),
        root.formula_sources()
            .source(id(9003))
            .expect("actual source")
    ));
    assert_eq!(normalized.normalized().node_count(), 3);
    assert_eq!(normalized.normalized().retained_records(), 7);
    let recipe = copied
        .normalize_recipe()
        .expect("syntax accepts a still-unbound name");
    assert!(std::ptr::eq(recipe.scope().root(), &root));
    let statement = recipe
        .normalized()
        .statements()
        .first()
        .expect("actual recipe let");
    let span = statement.name_span();
    let name = root
        .recipe()
        .source()
        .get(span.start()..span.end())
        .expect("actual authored name");
    assert!(std::ptr::eq(statement.name(), name));
    assert_eq!(name, "authored");
    assert_eq!(
        root.inputs()
            .length(id(40))
            .expect("actual Derived")
            .authored_value(),
        Err(LengthValueError::RequiresEvaluation {
            declaration: id(40),
            formula: id(9003)
        })
    );
}
#[test]
fn original_first_content_refusal_precedes_every_current_semantic_callback() {
    let (root, semantic, reject) = observed();
    reject.set(true);
    let limits = DesignResourceLimits::new(
        NonZeroUsize::new(10000).expect("entries"),
        NonZeroUsize::new(10000).expect("text"),
        NonZeroU64::new(1).expect("bits"),
    );
    let error = root
        .prepare_formula_scope(limits)
        .expect_err("no current callback before held content");
    assert_eq!(semantic.get(), 0);
    assert!(std::ptr::eq(error.root(), &root));
    assert_eq!(error.limits(), limits);
    assert!(
        matches!(error.refusal(),DesignFormulaScopeRefusal::Resources(
        DesignResourceLimitError::CounterBits{maximum:1,consumed:0,requested:129,
            location:DesignResourceLocation{scope:DesignResourceScope::Structure,owner:Some(owner),
                field:"design.revision",index:None}}) if *owner==id(1))
    );
    assert!(std::error::Error::source(&error).is_some());
}
#[test]
fn later_current_source_refusal_preserves_original_typed_cause_without_scope() {
    let (root, semantic, reject) = observed();
    let limits = exact(measured(&root));
    reject.set(true);
    let error = root
        .prepare_formula_scope(limits)
        .expect_err("source changed after construction");
    assert!(semantic.get() > 0);
    assert!(std::ptr::eq(error.root(), &root));
    assert_eq!(error.limits(), limits);
    assert!(matches!(error.refusal(),
        DesignFormulaScopeRefusal::Inputs(DesignInputsError::Sources(cause))
            if matches!(cause.as_ref(),DesignMetadataError::Host(MissingFixtureRecord))));
}
#[test]
fn whole_source_normalization_refusal_and_empty_recipe_keep_actual_containing_root() {
    let mut supplied = definition();
    supplied.recipe = FormulaRecipeDocument::new(String::new()).expect("empty recipe");
    let root = DesignInputs::new(supplied).expect("actual root still owns required source9003");
    let scope = root
        .prepare_formula_scope(exact(measured(&root)))
        .expect("complete actual content");
    let limits = FormulaSourceNormalizationLimits::new(0, 3, 7);
    let error = scope
        .normalize_sources(limits)
        .expect_err("whole source population before prefix");
    assert!(std::ptr::eq(error.inputs().inputs(), root.inputs()));
    assert!(std::ptr::eq(
        error.inputs().sources(),
        root.formula_sources()
    ));
    assert_eq!(error.limits(), limits);
    assert!(matches!(
        error.refusal(),
        FormulaSourceNormalizationRefusal::Sources {
            allowed: 0,
            required: 1
        }
    ));
    let empty = scope.normalize_recipe().expect("actual empty syntax");
    assert!(empty.normalized().statements().is_empty());
    assert!(std::ptr::eq(empty.scope().root(), &root));
}
