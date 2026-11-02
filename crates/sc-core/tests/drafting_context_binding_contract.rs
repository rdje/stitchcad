//! Actual source-owner binding and resource-before-query ordering, without geometry execution.
#[path = "support/drafting_geometry.rs"]
mod geometry;
#[path = "support/drafting_program.rs"]
mod program;
#[path = "support/drafting_program_admission.rs"]
mod program_admission;
#[path = "support/drafting_program_name_inputs.rs"]
mod program_name_inputs;
#[path = "support/drafting_program_name_work.rs"]
mod program_name_work;
#[path = "support/drafting_program_output_positions.rs"]
mod program_output_positions;
#[path = "support/drafting_program_views.rs"]
mod program_views;
use core::{
    cell::Cell,
    fmt,
    num::{NonZeroU64, NonZeroUsize},
};
use sc_core::{
    command::DesignRevision,
    construction::{
        DraftingContextError, DraftingContextSource, DraftingGeometryReference,
        DraftingGeometrySourceResult, DraftingOutputPosition, DraftingPlannedOutputProvider,
        DraftingPlannedOutputSourceResult, DraftingSourceRecord,
    },
    design::*,
    ontology::{CutPlan, EntityId, IdentityLedger, MaterialInventory, PieceInventory},
    recipe::{FormulaRecipeDocument, FormulaSource, FormulaSourceLimits, FormulaSources},
    value::{CanonicalInputs, LengthDeclaration},
};
use std::error::Error;

type TestResult<T = ()> = Result<T, Box<dyn Error>>;
struct EmptyHost;
#[derive(Debug, PartialEq, Eq)]
enum SourceError {
    Missing,
    Rejected,
}
impl fmt::Display for SourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl Error for SourceError {}
impl DesignMetadataHost for EmptyHost {
    type Error = SourceError;
    fn entities(&self) -> Vec<DesignHostEntity> {
        vec![]
    }
    fn validate_current(&self, _: &CanonicalInputs) -> Result<(), Self::Error> {
        Ok(())
    }
    fn measurement_declaration<'a>(
        &'a self,
        _: EntityId,
        _: &'a CanonicalInputs,
    ) -> Result<&'a LengthDeclaration, Self::Error> {
        Err(SourceError::Missing)
    }
}
impl DesignSizeSetHost for EmptyHost {
    fn validate_size_set(
        &self,
        _: DesignSizeSetReference,
        _: &CanonicalInputs,
    ) -> Result<(), Self::Error> {
        Err(SourceError::Missing)
    }
}
impl DesignReferenceHost for EmptyHost {
    fn validate_references(
        &self,
        refs: DesignSourceReferences,
        _: &CanonicalInputs,
    ) -> Result<(), Self::Error> {
        if refs.measurement_table.is_none() && refs.ease_set.is_none() && refs.size_set.is_none() {
            Ok(())
        } else {
            Err(SourceError::Missing)
        }
    }
}
impl DesignResourceSource for EmptyHost {
    fn charge_resources(
        &self,
        _: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        Ok(()) // This fixture host owns no dynamic records or counters.
    }
}
impl DesignFormulaDeclarationSource for EmptyHost {
    type Declarations<'a> =
        std::iter::Empty<Result<sc_core::recipe::FormulaDeclaration<'a>, SourceError>>;
    fn formula_declaration_sources<'a>(
        &'a self,
        _: DesignSourceReferences,
        _: &'a CanonicalInputs,
    ) -> Self::Declarations<'a> {
        std::iter::empty()
    }
}
fn root() -> TestResult<DesignInputs<EmptyHost>> {
    root_with_recipe("")
}
fn root_with_recipe(recipe: &str) -> TestResult<DesignInputs<EmptyHost>> {
    root_with_sources(recipe, vec![])
}
fn root_with_sources(
    recipe: &str,
    formula_sources: Vec<FormulaSource>,
) -> TestResult<DesignInputs<EmptyHost>> {
    root_with_host(
        recipe,
        formula_sources,
        EmptyHost,
        CanonicalInputs::new(vec![], vec![])?,
        vec![],
        DesignSourceReferences {
            measurement_table: None,
            ease_set: None,
            size_set: None,
        },
    )
}
fn root_with_host<H: DesignReferenceHost>(
    recipe: &str,
    formula_sources: Vec<FormulaSource>,
    host: H,
    canonical: CanonicalInputs,
    parameter_definitions: Vec<DesignParameterDefinition>,
    references: DesignSourceReferences,
) -> TestResult<DesignInputs<H>> {
    let parameters = DesignParameters::new(parameter_definitions, &canonical)?;
    let materials = MaterialInventory::new(vec![], &canonical)?;
    let sources = DesignMetadata::new(host, &canonical)?;
    let structure = DesignStructure::new(DesignStructureDefinition {
        id: EntityId::from_bits(1),
        revision: DesignRevision::from_counter(7),
        inputs: canonical,
        pieces: PieceInventory::new(vec![])?,
        cut_plan: CutPlan::new(vec![], &[])?,
        topology: IdentityLedger::new(),
    })?;
    Ok(DesignInputs::new(DesignInputsDefinition {
        structure,
        parameters,
        materials,
        sources,
        recipe: FormulaRecipeDocument::new(recipe.into())?,
        formula_sources: FormulaSources::new(
            formula_sources,
            FormulaSourceLimits::new(1000, 10000, 10000),
        )?,
        references,
    })?)
}
fn limits(entries: usize, text: usize, bits: u64) -> TestResult<DesignResourceLimits> {
    Ok(DesignResourceLimits::new(
        NonZeroUsize::new(entries).ok_or("positive entry allowance")?,
        NonZeroUsize::new(text).ok_or("positive text allowance")?,
        NonZeroU64::new(bits).ok_or("positive bit allowance")?,
    ))
}
fn scope<H: DesignResourceSource>(root: &DesignInputs<H>) -> TestResult<DesignFormulaScope<'_, H>> {
    Ok(root
        .prepare_formula_scope(limits(1000, 1000, 1000)?)
        .map_err(|e| e.to_string())?)
}
// Opaque provider sources deliberately implement neither Clone nor Debug.
struct Frame {
    label: String,
    origin: [i64; 2],
    axes: [[i64; 2]; 2],
}
struct Snapshot {
    label: String,
}
struct Source<'a, H = EmptyHost> {
    root: &'a DesignInputs<H>,
    revision: DesignRevision,
    frame: DraftingSourceRecord<Frame>,
    snapshot: DraftingSourceRecord<Snapshot>,
    cache: Vec<u8>,
    trace: Cell<u64>,
    ignore_refusal: bool,
    reject: bool,
}
impl<'a, H: DesignMetadataHost> Source<'a, H> {
    fn new(root: &'a DesignInputs<H>) -> Self {
        Self {
            root,
            revision: DesignRevision::from_counter(7),
            frame: DraftingSourceRecord::new(Frame {
                label: "private Cartesian frame é".into(),
                origin: [0, 0],
                axes: [[1, 0], [0, 1]],
            }),
            snapshot: DraftingSourceRecord::new(Snapshot {
                label: "private empty snapshot".into(),
            }),
            cache: vec![1, 2, 3],
            trace: Cell::new(0),
            ignore_refusal: false,
            reject: false,
        }
    }
    fn observed(&self, stage: u64) {
        self.trace.set(self.trace.get() * 10 + stage);
    }
    fn location(&self, field: &'static str) -> DesignResourceLocation {
        DesignResourceLocation {
            scope: DesignResourceScope::Inputs,
            owner: Some(self.root.id()),
            field,
            index: None,
        }
    }
}
impl<H: DesignMetadataHost> DraftingContextSource<H> for Source<'_, H> {
    type Frame = Frame;
    type Snapshot = Snapshot;
    type Geometry = [u8];
    type Error = SourceError;
    fn root(&self) -> &DesignInputs<H> {
        self.observed(1);
        self.root
    }
    fn snapshot_revision(&self) -> &DesignRevision {
        self.observed(3);
        &self.revision
    }
    fn frame(&self) -> &DraftingSourceRecord<Frame> {
        self.observed(5);
        &self.frame
    }
    fn snapshot(&self) -> &DraftingSourceRecord<Snapshot> {
        self.observed(6);
        &self.snapshot
    }
    fn charge_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        self.observed(2);
        let first = sink.charge_entries(self.cache.len(), self.location("drafting.cache"));
        if self.ignore_refusal {
            // Deliberately broken return discipline must not clear the sink's first refusal.
            let _ = first;
            let _ = sink.charge_text(
                &self.frame.data().label,
                self.location("drafting.frame.label"),
            );
            return Ok(());
        }
        first?;
        sink.charge_text(
            &self.frame.data().label,
            self.location("drafting.frame.label"),
        )?;
        sink.charge_text(
            &self.snapshot.data().label,
            self.location("drafting.snapshot.label"),
        )?;
        charge_design_revision_resources(
            &self.revision,
            sink,
            self.location("drafting.snapshot.revision"),
        )
    }
    fn validate_current(&self) -> Result<(), Self::Error> {
        self.observed(4);
        if self.reject {
            Err(SourceError::Rejected)
        } else {
            Ok(())
        }
    }
    fn geometry(
        &self,
        _: DraftingGeometryReference,
    ) -> DraftingGeometrySourceResult<'_, Frame, Snapshot, [u8], SourceError> {
        self.observed(7);
        Err(SourceError::Missing) // This actual fixture snapshot has no geometric outputs.
    }
}

#[test]
fn bound_context_holds_actual_non_clone_sources_and_never_queries_geometry() -> TestResult {
    let root = root()?;
    let scope = scope(&root)?;
    let source = Source::new(&root);
    let policy = limits(
        3,
        source.frame.data().label.len() + source.snapshot.data().label.len(),
        3,
    )?;
    let context = scope
        .bind_drafting_context(&source, policy)
        .map_err(|e| e.to_string())?;
    let copied = context;
    assert!(std::ptr::eq(copied.scope().root(), &root));
    assert!(std::ptr::eq(copied.source(), &source));
    assert!(std::ptr::eq(copied.frame(), &source.frame));
    assert!(std::ptr::eq(copied.snapshot(), &source.snapshot));
    assert_eq!(source.trace.get(), 123456);
    assert_eq!(copied.frame().data().origin, [0, 0]);
    assert_eq!(copied.frame().data().axes, [[1, 0], [0, 1]]);
    assert_eq!(copied.limits(), policy);
    assert_eq!(copied.demand().collection_entries(), 3);
    assert_eq!(
        copied.demand().text_bytes(),
        source.frame.data().label.len() + source.snapshot.data().label.len()
    );
    assert_eq!(copied.demand().counter_bits(), 3);
    assert!(!format!("{copied:?}").contains("private"));
    Ok(())
}

#[test]
fn equal_ids_and_revisions_on_a_foreign_actual_owner_refuse_before_accounting() -> TestResult {
    let expected = root()?;
    let other = root()?;
    assert_eq!(expected.id(), other.id());
    assert_eq!(expected.revision(), other.revision());
    let source = Source::new(&other);
    let result = scope(&expected)?.bind_drafting_context(&source, limits(1, 1, 1)?);
    match result {
        Err(DraftingContextError::ForeignOwner {
            expected: held,
            actual,
        }) => {
            assert!(std::ptr::eq(held, &expected));
            assert!(std::ptr::eq(actual, &other));
        }
        other => return Err(format!("expected original owner witnesses, got {other:?}").into()),
    }
    assert_eq!(source.trace.get(), 1);
    Ok(())
}

#[test]
fn complete_content_refuses_before_revision_currentness_or_geometry_and_latches_first_cause(
) -> TestResult {
    let root = root()?;
    for ignore_refusal in [false, true] {
        let mut source = Source::new(&root);
        source.ignore_refusal = ignore_refusal;
        source.revision = DesignRevision::from_counter(8);
        source.reject = true;
        let result = scope(&root)?.bind_drafting_context(&source, limits(1, 1, 1)?);
        let expected = DesignResourceLimitError::Count {
            unit: DesignResourceCount::CollectionEntries,
            maximum: 1,
            consumed: 0,
            requested: 3,
            location: source.location("drafting.cache"),
        };
        assert!(matches!(result,Err(DraftingContextError::Resources(actual)) if actual==expected));
        assert_eq!(source.trace.get(), 12);
    }
    let source = Source::new(&root);
    let result = scope(&root)?.bind_drafting_context(&source, limits(3, 1000, 2)?);
    assert!(
        matches!(result,Err(DraftingContextError::Resources(DesignResourceLimitError::CounterBits {
        maximum:2,consumed:0,requested:3,location
    })) if location==source.location("drafting.snapshot.revision"))
    );
    assert_eq!(source.trace.get(), 12);
    Ok(())
}

#[test]
fn revision_and_original_source_refusals_keep_their_order_and_witnesses() -> TestResult {
    let root = root()?;
    let mut source = Source::new(&root);
    source.revision = DesignRevision::from_counter(8);
    source.reject = true;
    match scope(&root)?.bind_drafting_context(&source, limits(1000, 1000, 1000)?) {
        Err(DraftingContextError::RevisionMismatch { expected, actual }) => {
            assert!(std::ptr::eq(expected, root.revision()));
            assert!(std::ptr::eq(actual, &source.revision));
        }
        other => return Err(format!("expected original revision witnesses, got {other:?}").into()),
    }
    assert_eq!(source.trace.get(), 123);
    source.revision = DesignRevision::from_counter(7);
    source.trace.set(0);
    let result = scope(&root)?.bind_drafting_context(&source, limits(1000, 1000, 1000)?);
    assert!(matches!(
        result,
        Err(DraftingContextError::Source(SourceError::Rejected))
    ));
    if let Err(error) = result {
        assert!(error
            .source()
            .is_some_and(|cause| cause.is::<SourceError>()));
    }
    assert_eq!(source.trace.get(), 1234);
    Ok(())
}

#[test]
fn external_provider_charges_actual_revision_magnitude_without_copying_or_formatting() -> TestResult
{
    let location = DesignResourceLocation {
        scope: DesignResourceScope::Inputs,
        owner: Some(EntityId::from_bits(5)),
        field: "external.snapshot.revision",
        index: Some(2),
    };
    for (revision, expected) in [
        (DesignRevision::initial(), 0),
        (DesignRevision::from_counter(1), 1),
        (DesignRevision::from_counter(u128::MAX).successor(), 129),
    ] {
        let mut meter = DesignResourceMeter::new(limits(1, 1, 129)?);
        charge_design_revision_resources(&revision, &mut meter, location)?;
        assert_eq!(meter.finish()?.counter_bits(), expected);
    }
    let revision = DesignRevision::from_counter(u128::MAX).successor();
    let mut meter = DesignResourceMeter::new(limits(1, 1, 128)?);
    let expected = DesignResourceLimitError::CounterBits {
        maximum: 128,
        consumed: 0,
        requested: 129,
        location,
    };
    assert_eq!(
        charge_design_revision_resources(&revision, &mut meter, location),
        Err(expected)
    );
    assert_eq!(meter.finish(), Err(expected));
    Ok(())
}

impl<H: DesignMetadataHost> DraftingPlannedOutputProvider<H> for Source<'_, H> {
    fn planned_output<'v, 'n, 'i>(
        &'v self,
        request: DraftingOutputPosition<'v, 'n, 'i>,
    ) -> DraftingPlannedOutputSourceResult<'v, Self::Frame, Self::Snapshot, Self::Error> {
        self.observed(8);
        if request.requires_role_binding() {
            Err(SourceError::Missing)
        } else {
            Err(SourceError::Rejected)
        }
    }
}
