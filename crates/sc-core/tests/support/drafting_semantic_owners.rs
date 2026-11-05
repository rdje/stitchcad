//! Exact held semantic owners; equal-looking records never replace original context loans.
use super::{
    garment_source, limits, scope, EmptyHost, Frame, Snapshot, Source, SourceError, TestResult,
};
use sc_core::{command::DesignRevision, construction::*, design::*, recipe::*};
use std::cell::Cell;

struct Provider<'a> {
    base: Source<'a>,
    alternate: Source<'a>,
    garment: &'a DesignGarment<EmptyHost>,
    variant: Cell<u8>,
}
impl<'a> Provider<'a> {
    fn new(garment: &'a DesignGarment<EmptyHost>) -> Self {
        Self {
            base: Source::new(garment.inputs()),
            alternate: Source::new(garment.inputs()),
            garment,
            variant: Cell::new(0),
        }
    }
}
impl DraftingContextSource<EmptyHost> for Provider<'_> {
    type Frame = Frame;
    type Snapshot = Snapshot;
    type Geometry = [u8];
    type Error = SourceError;
    fn root(&self) -> &DesignInputs<EmptyHost> {
        self.base.root()
    }
    fn snapshot_revision(&self) -> &DesignRevision {
        self.base.snapshot_revision()
    }
    fn frame(&self) -> &DraftingSourceRecord<Frame> {
        if self.variant.get() & 1 == 0 {
            self.base.frame()
        } else {
            self.alternate.frame()
        }
    }
    fn snapshot(&self) -> &DraftingSourceRecord<Snapshot> {
        if self.variant.get() & 2 == 0 {
            self.base.snapshot()
        } else {
            self.alternate.snapshot()
        }
    }
    fn geometry(
        &self,
        r: DraftingGeometryReference,
    ) -> DraftingGeometrySourceResult<'_, Frame, Snapshot, [u8], SourceError> {
        self.base.geometry(r)
    }
    fn validate_current(&self) -> Result<(), SourceError> {
        self.base.validate_current()
    }
    fn charge_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        // Reset the legacy decimal event tracer at each setup accounting boundary. Only the
        // final join's callback-free interval is asserted; long setup chains are not one trace.
        self.base.trace.set(0);
        self.alternate.trace.set(0);
        self.base.charge_resources(sink)?;
        self.alternate.charge_resources(sink)
    }
}
impl DraftingGarmentProvider<EmptyHost> for Provider<'_> {
    fn garment_source(&self) -> DraftingGarmentSource<'_, EmptyHost, Snapshot> {
        self.base.observed(8);
        DraftingGarmentSource {
            garment: self.garment,
            snapshot: self.snapshot(),
        }
    }
}
impl DraftingInitialInputProvider<EmptyHost> for Provider<'_> {
    type Metadata = ();
    fn initial_input<'a>(
        &'a self,
        _: usize,
        _: FormulaCanonicalInput<'a>,
    ) -> DraftingInitialInputSourceResult<'a, (), Snapshot, SourceError> {
        self.base.observed(7);
        Err(SourceError::Missing)
    }
}
impl DraftingPlannedOutputProvider<EmptyHost> for Provider<'_> {
    fn planned_output<'v, 'n, 'i>(
        &'v self,
        _: DraftingOutputPosition<'v, 'n, 'i>,
    ) -> DraftingPlannedOutputSourceResult<'v, Frame, Snapshot, SourceError> {
        self.base.observed(7);
        Err(SourceError::Missing)
    }
}
impl DraftingGeometryStepProvider<EmptyHost> for Provider<'_> {
    fn planned_geometry<'v, 'i>(
        &'v self,
        _: DraftingGeometryStepRequest<'v, 'i>,
    ) -> DraftingPlannedOutputSourceResult<'v, Frame, Snapshot, SourceError> {
        self.base.observed(7);
        Err(SourceError::Missing)
    }
}

// Exercise the public ?Sized source boundary without Clone or Debug on opaque payloads.
trait SemanticProvider:
    DraftingGarmentProvider<EmptyHost>
    + DraftingInitialInputProvider<EmptyHost>
    + DraftingPlannedOutputProvider<EmptyHost>
    + DraftingGeometryStepProvider<EmptyHost>
{
}
impl SemanticProvider for Provider<'_> {}
type SourceView<'a> = dyn SemanticProvider<
        Frame = Frame,
        Snapshot = Snapshot,
        Geometry = [u8],
        Error = SourceError,
        Metadata = (),
    > + 'a;

#[test]
fn exact_semantic_owners_reject_foreign_records_before_callbacks() -> TestResult {
    let garment = garment_source::garment(false).map_err(|e| e.to_string())?;
    let equal_garment = garment_source::garment(false).map_err(|e| e.to_string())?;
    assert_eq!(garment.inputs().id(), equal_garment.inputs().id());
    let source = Provider::new(&garment);
    let other_source = Provider::new(&garment);
    let foreign_root = Provider::new(&equal_garment);
    let source_view: &SourceView<'_> = &source;
    let other_view: &SourceView<'_> = &other_source;
    let foreign_view: &SourceView<'_> = &foreign_root;
    let content = limits(100000, 100000, 100000).map_err(|e| e.to_string())?;
    let input_scope = scope(garment.inputs()).map_err(|e| e.to_string())?;
    let context = input_scope
        .bind_drafting_context(source_view, content)
        .map_err(|e| e.to_string())?;
    let actual_garment = context
        .admit_garment_source(content)
        .map_err(|e| e.to_string())?;
    let alternate_provider = input_scope
        .bind_drafting_context(other_view, content)
        .map_err(|e| e.to_string())?
        .admit_garment_source(content)
        .map_err(|e| e.to_string())?;
    let alternate_root = scope(equal_garment.inputs())
        .map_err(|e| e.to_string())?
        .bind_drafting_context(foreign_view, content)
        .map_err(|e| e.to_string())?
        .admit_garment_source(content)
        .map_err(|e| e.to_string())?;
    // Capture adversarial equal-looking records before the join, then restore the source.
    // The join must inspect the held records, never ask the provider to reselect them.
    source.variant.set(1);
    let alternate_frame = input_scope
        .bind_drafting_context(source_view, content)
        .map_err(|e| e.to_string())?
        .admit_garment_source(content)
        .map_err(|e| e.to_string())?;
    source.variant.set(2);
    let alternate_snapshot = input_scope
        .bind_drafting_context(source_view, content)
        .map_err(|e| e.to_string())?
        .admit_garment_source(content)
        .map_err(|e| e.to_string())?;
    source.variant.set(3);
    let alternate_both = input_scope
        .bind_drafting_context(source_view, content)
        .map_err(|e| e.to_string())?
        .admit_garment_source(content)
        .map_err(|e| e.to_string())?;
    source.variant.set(0);
    let raw = DraftingProgramInputs::new(context, vec![]);
    let pp = DraftingProgramLimits::new(content, 0, 0, 0);
    let program = raw
        .admit(&pp)
        .map_err(|e| e.to_string())?
        .normalized()
        .map_err(|e| e.to_string())?;
    let names = DraftingProgramNameInputs::new(&program, vec![], vec![]);
    let np = DraftingProgramNameLimits::new(content, 64, 128);
    let work = names.admit_name_work(&np).map_err(|e| e.to_string())?;
    let positions = work.bind_output_positions().map_err(|e| e.to_string())?;
    let roles = positions.bind_role_sources().map_err(|e| e.to_string())?;
    let initials = roles.bind_initial_sources().map_err(|e| e.to_string())?;
    let namespace = initials
        .bind_initial_namespace()
        .map_err(|e| e.to_string())?;
    let sources = namespace
        .check_root_sources(FormulaSourceCheckLimits::new(1, 100, 0, 1000))
        .map_err(|e| e.to_string())?;
    let dependencies = sources
        .proof()
        .prepare_dependencies(FormulaSourceDependencyLimits::new(1, 0, 2))
        .map_err(|e| e.to_string())?;
    let graph = dependencies
        .admit_acyclic(FormulaSourceCycleLimits::new(1, 2))
        .map_err(|e| e.to_string())?;
    let op = DraftingProgramOrderLimits::new(100, 1000, 10000);
    let order = namespace.check_order(&op).map_err(|e| e.to_string())?;
    let inputs = sources
        .dependency_inputs(&order, &graph)
        .map_err(|e| e.to_string())?;
    let cp = DraftingProgramCanonicalLimits::new(0, 0);
    let canonical = inputs
        .bind_canonical_inputs(&cp)
        .map_err(|e| e.to_string())?;
    let gp = DraftingProgramGeometryLimits::new(content, 0, 0, 0);
    let geometry_work = canonical
        .admit_geometry_work(&gp)
        .map_err(|e| e.to_string())?;
    let geometry = geometry_work.bind_geometry().map_err(|e| e.to_string())?;
    // Local structural admission intentionally accepts nonexistent coordinates. This owner join
    // must not promote them to source coverage or reject them by running an unbounded scan.
    let raw_selections =
        DraftingSemanticSelectionInputs::new(vec![DraftingSemanticSelectionRow::Rule {
            ordinal: usize::MAX,
            origin: DraftingRuleSelection::Design,
        }]);
    let selections = raw_selections
        .admit(DraftingSemanticSelectionLimits::new(content, 1))
        .map_err(|e| e.to_string())?;
    source.base.trace.set(0);
    source.alternate.trace.set(0);
    let joined = geometry
        .semantic_inputs(&actual_garment, &selections)
        .map_err(|e| e.to_string())?;
    assert!(std::ptr::eq(joined.geometry(), &geometry));
    assert!(std::ptr::eq(joined.garment(), &actual_garment));
    assert!(std::ptr::eq(joined.selections(), &selections));
    assert!(std::ptr::eq(joined.selections().inputs(), &raw_selections));
    assert!(std::ptr::eq(
        joined.geometry().work().inputs().inputs().graph(),
        &graph
    ));
    assert_eq!(joined.selections().inputs().rows().len(), 1);
    for (candidate, expected) in [
        (&alternate_root, DraftingProgramSemanticOwnerIssue::Root),
        (
            &alternate_provider,
            DraftingProgramSemanticOwnerIssue::Provider,
        ),
        (&alternate_frame, DraftingProgramSemanticOwnerIssue::Frame),
        (
            &alternate_snapshot,
            DraftingProgramSemanticOwnerIssue::Snapshot,
        ),
        (&alternate_both, DraftingProgramSemanticOwnerIssue::Frame),
    ] {
        let error = geometry
            .semantic_inputs(candidate, &selections)
            .expect_err("foreign held owner");
        assert_eq!(error.issue(), expected);
        assert!(std::ptr::eq(error.geometry(), &geometry));
        assert!(std::ptr::eq(error.garment(), candidate));
        assert!(std::ptr::eq(error.selections(), &selections));
        assert!(!format!("{error:?} {error}").contains("private"));
    }
    assert_eq!(source.base.trace.get(), 0);
    assert_eq!(source.alternate.trace.get(), 0);
    assert!(!format!("{joined:?}").contains("private"));
    Ok(())
}
