//! Exact held semantic owners; equal-looking records never replace original context loans.
use super::{
    garment_source, limits, scope, EmptyHost, Frame, Snapshot, Source, SourceError, TestResult,
};
use sc_core::{
    command::DesignRevision,
    construction::*,
    design::*,
    ontology::{SelectedProfilePin, SelectedProfilePinDefinition},
    recipe::*,
};
use std::cell::Cell;
#[path = "drafting_root_input_members.rs"]
mod root_input_members;
#[path = "drafting_root_logical_suppliers.rs"]
mod root_logical_suppliers;
#[path = "drafting_root_numeric_suppliers.rs"]
mod root_numeric_suppliers;

struct Provider<'a> {
    base: Source<'a>,
    alternate: Source<'a>,
    garment: &'a DesignGarment<EmptyHost>,
    variant: Cell<u8>,
    walks: Cell<usize>,
    ignore_refusal: Cell<bool>,
    extra_charge: Cell<usize>,
}
impl<'a> Provider<'a> {
    fn new(garment: &'a DesignGarment<EmptyHost>) -> Self {
        Self {
            base: Source::new(garment.inputs()),
            alternate: Source::new(garment.inputs()),
            garment,
            variant: Cell::new(0),
            walks: Cell::new(0),
            ignore_refusal: Cell::new(false),
            extra_charge: Cell::new(0),
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
        self.walks.set(self.walks.get() + 1);
        self.base.trace.set(0);
        self.alternate.trace.set(0);
        let first = self.base.charge_resources(sink);
        let charged = if self.ignore_refusal.get() {
            let _ = first;
            let _ = self.alternate.charge_resources(sink);
            Ok(())
        } else {
            first?;
            self.alternate.charge_resources(sink)
        };
        charged?;
        if self.extra_charge.get() > 0 {
            sink.charge_entries(
                self.extra_charge.get(),
                DesignResourceLocation {
                    scope: DesignResourceScope::CommandState,
                    owner: Some(self.garment.inputs().id()),
                    field: "numeric.extra",
                    index: None,
                },
            )?;
        }
        Ok(())
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
    // An actual original native target request stays unbound; counting it is not target lookup.
    let grainline = GrainlinePlacement::new(
        sc_core::ontology::EntityId::from_bits(80000),
        sc_core::ontology::EntityId::from_bits(90000),
    )?;
    let raw = DraftingProgramInputs::new(
        context,
        vec![DraftingProgramStep::Drafting(
            DraftingOperationInput::PlaceGrainline(&grainline),
        )],
    );
    let pp = DraftingProgramLimits::new(content, 0, 0, 1);
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
    let pin_text = ["unused private profile é", "schema 2", "content 7"];
    let [profile_id, schema_version, content_version] = pin_text;
    let selected = SelectedProfilePin::new(SelectedProfilePinDefinition {
        profile_id: profile_id.into(),
        schema_version: schema_version.into(),
        content_version: content_version.into(),
    })?;
    let raw_selections = DraftingSemanticSelectionInputs::new(vec![
        DraftingSemanticSelectionRow::Rule {
            ordinal: usize::MAX,
            origin: DraftingRuleSelection::Design,
        },
        DraftingSemanticSelectionRow::Profile {
            location: DraftingSemanticSelectionLocation::Pocket {
                pocket: 99,
                field: DraftingPocketSelectionField::Opening,
            },
            selected: &selected,
        },
    ]);
    let selections = raw_selections
        .admit(DraftingSemanticSelectionLimits::new(content, 4))
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
    assert_eq!(joined.selections().inputs().rows().len(), 2);
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
    // Independently walk just the actual garment/provider component. One raw/normalized native
    // slot, one order record, two request rows and the graph sentinel add six held entries.
    let mut garment_meter = DesignResourceMeter::new(content);
    actual_garment.charge_resources(&mut garment_meter)?;
    let garment_demand = garment_meter.finish()?;
    let entries = garment_demand.collection_entries() + 6;
    let text =
        garment_demand.text_bytes() + pin_text.iter().map(|value| value.len()).sum::<usize>();
    let bits = garment_demand.counter_bits();
    let policy = limits(entries, text, bits.max(1))?;
    source.walks.set(0);
    let mut whole = DesignResourceMeter::new(policy);
    joined.charge_resources(&mut whole)?;
    let demand = whole.finish()?;
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (entries, text, bits)
    );
    assert_eq!(source.walks.get(), 1);
    assert_eq!(source.base.trace.get(), 2);
    assert_eq!(source.alternate.trace.get(), 2);
    let mut repeated = DesignResourceMeter::new(limits(2 * entries, 2 * text, (2 * bits).max(1))?);
    source.walks.set(0);
    joined.charge_resources(&mut repeated)?;
    joined.charge_resources(&mut repeated)?;
    let twice = repeated.finish()?;
    assert_eq!(
        (
            twice.collection_entries(),
            twice.text_bytes(),
            twice.counter_bits()
        ),
        (2 * entries, 2 * text, 2 * bits)
    );
    assert_eq!(source.walks.get(), 2);
    let mut short = DesignResourceMeter::new(limits(entries, text - 1, bits.max(1))?);
    let expected = DesignResourceLimitError::Count {
        unit: DesignResourceCount::TextBytes,
        maximum: text - 1,
        consumed: text - content_version.len(),
        requested: content_version.len(),
        location: DesignResourceLocation {
            scope: DesignResourceScope::CommandState,
            owner: Some(garment.inputs().id()),
            field: "selected_profile.content_version",
            index: Some(1),
        },
    };
    assert_eq!(joined.charge_resources(&mut short), Err(expected));
    assert_eq!(short.finish(), Err(expected));
    let mut short = DesignResourceMeter::new(limits(entries - 1, text, bits.max(1))?);
    let expected = DesignResourceLimitError::Count {
        unit: DesignResourceCount::CollectionEntries,
        maximum: entries - 1,
        consumed: entries - 1,
        requested: 1,
        location: DesignResourceLocation {
            scope: DesignResourceScope::CommandState,
            owner: Some(garment.inputs().id()),
            field: "checked.source_dependencies.offsets",
            index: None,
        },
    };
    assert_eq!(joined.charge_resources(&mut short), Err(expected));
    assert_eq!(short.finish(), Err(expected));
    // An ignored source-sink failure remains the actual first provider field cause.
    let mut raw_meter = DesignResourceMeter::new(content);
    charge_design_garment_resources(&garment, &mut raw_meter)?;
    let raw_demand = raw_meter.finish()?;
    let frame_text = source.base.frame.data().label.len();
    let maximum = raw_demand.text_bytes() + frame_text - 1;
    let expected = DesignResourceLimitError::Count {
        unit: DesignResourceCount::TextBytes,
        maximum,
        consumed: raw_demand.text_bytes(),
        requested: frame_text,
        location: source.base.location("drafting.frame.label"),
    };
    source.ignore_refusal.set(true);
    let mut short = DesignResourceMeter::new(limits(entries, maximum, bits.max(1))?);
    assert_eq!(joined.charge_resources(&mut short), Err(expected));
    assert_eq!(short.finish(), Err(expected));
    source.ignore_refusal.set(false);
    let mut earlier = DesignResourceMeter::new(policy);
    let first = earlier
        .charge_entries(entries + 1, source.base.location("earlier.content"))
        .expect_err("earlier cumulative field");
    source.walks.set(0);
    assert_eq!(joined.charge_resources(&mut earlier), Err(first));
    assert_eq!(earlier.finish(), Err(first));
    assert_eq!(source.walks.get(), 0);
    assert_eq!(joined.selections().inputs().rows().len(), 2);
    let population_limits = DraftingProgramSemanticPopulationLimits::new(policy, 6, 24, 8);
    source.walks.set(0);
    let population = joined
        .admit_consumer_population(&population_limits)
        .map_err(|e| e.to_string())?;
    let shape = population.shape();
    assert_eq!(
        (
            shape.garment_occurrences(),
            shape.semantic_occurrences(),
            shape.native_occurrences(),
            shape.selection_rows()
        ),
        (5, 0, 1, 2)
    );
    assert_eq!(
        (
            shape.occurrence_count(),
            shape.selection_comparison_bound(),
            shape.association_record_bound()
        ),
        (6, 24, 8)
    );
    assert!(core::ptr::eq(population.inputs(), &joined));
    assert!(core::ptr::eq(population.limits(), &population_limits));
    assert_eq!(population.demand(), demand);
    assert_eq!(source.walks.get(), 1);
    assert_eq!(source.base.trace.get(), 2);
    assert_eq!(source.alternate.trace.get(), 2);
    use DraftingProgramSemanticPopulationRefusal as R;
    for (limits, expected) in [
        (
            DraftingProgramSemanticPopulationLimits::new(policy, 5, 0, 0),
            R::Occurrences {
                allowed: 5,
                required: 6,
            },
        ),
        (
            DraftingProgramSemanticPopulationLimits::new(policy, 6, 23, 0),
            R::SelectionComparisons {
                allowed: 23,
                required: 24,
            },
        ),
        (
            DraftingProgramSemanticPopulationLimits::new(policy, 6, 24, 7),
            R::AssociationRecords {
                allowed: 7,
                required: 8,
            },
        ),
    ] {
        source.walks.set(0);
        let error = joined
            .admit_consumer_population(&limits)
            .expect_err("complete direct quota");
        assert_eq!(error.refusal(), expected);
        assert!(core::ptr::eq(error.inputs(), &joined));
        assert!(core::ptr::eq(error.limits(), &limits));
        assert_eq!(error.demand(), Some(demand));
        assert_eq!(error.shape(), Some(shape));
        assert_eq!(source.walks.get(), 1);
        assert!(!format!("{error:?} {error}").contains("private"));
    }
    let too_short = DraftingProgramSemanticPopulationLimits::new(
        limits(entries, text - 1, bits.max(1))?,
        0,
        0,
        0,
    );
    let error = joined
        .admit_consumer_population(&too_short)
        .expect_err("whole content precedes even zero occurrence quotas");
    assert!(matches!(
        error.refusal(),
        R::Resources(DesignResourceLimitError::Count {
            unit: DesignResourceCount::TextBytes,
            location: DesignResourceLocation {
                field: "selected_profile.content_version",
                index: Some(1),
                ..
            },
            ..
        })
    ));
    assert_eq!(error.demand(), None);
    assert_eq!(error.shape(), None);
    assert!(std::error::Error::source(&error).is_some());
    assert!(!format!("{population:?}").contains("private"));
    assert_eq!(joined.selections().inputs().rows().len(), 2);
    // Two selected passes are capacity only: the invalid raw selections are still untouched.
    // Derive their allowance from the independent garment-plus-record fixture above.
    let child_entries = 2 * entries as u128;
    let child_text = 2 * text as u128;
    let child_bits = 2 * u128::from(bits);
    let per_pass = (entries + 8) * (entries + 8);
    let child_matching = 2 * per_pass as u128;
    let child_content =
        DraftingProgramSemanticChildContentLimits::new(child_entries, child_text, child_bits);
    let work_limits = DraftingProgramSemanticWorkLimits::new(
        population_limits,
        2,
        child_content,
        child_matching,
        10,
    );
    source.walks.set(0);
    let work = joined
        .admit_semantic_work(&work_limits)
        .map_err(|e| e.to_string())?;
    assert_eq!(source.walks.get(), 1);
    assert_eq!(source.base.trace.get(), 2);
    assert_eq!(source.alternate.trace.get(), 2);
    assert!(core::ptr::eq(work.inputs(), &joined));
    assert!(core::ptr::eq(work.limits(), &work_limits));
    assert_eq!(work.demand(), demand);
    assert_eq!(work.population_shape(), shape);
    let work_shape = work.shape();
    assert_eq!(
        (
            work_shape.child_pass_capacity(),
            work_shape.result_record_bound()
        ),
        (2, 10)
    );
    assert_eq!(
        (
            work_shape.child_collection_entry_bound(),
            work_shape.child_text_byte_bound(),
            work_shape.child_counter_bit_bound(),
            work_shape.per_pass_matching_bound(),
            work_shape.child_matching_bound()
        ),
        (
            child_entries,
            child_text,
            child_bits,
            per_pass,
            child_matching
        )
    );
    use DraftingProgramSemanticWorkRefusal as W;
    for (case, entries, text, bits, matching, results) in [
        (0, child_entries - 1, 0, 0, 0, 0),
        (1, child_entries, child_text - 1, 0, 0, 0),
        (2, child_entries, child_text, child_bits - 1, 0, 0),
        (
            3,
            child_entries,
            child_text,
            child_bits,
            child_matching - 1,
            0,
        ),
        (4, child_entries, child_text, child_bits, child_matching, 9),
    ] {
        let limits = DraftingProgramSemanticWorkLimits::new(
            population_limits,
            2,
            DraftingProgramSemanticChildContentLimits::new(entries, text, bits),
            matching,
            results,
        );
        source.walks.set(0);
        let error = joined
            .admit_semantic_work(&limits)
            .expect_err("first complete child quota");
        match (case, error.refusal()) {
            (0, W::ChildCollectionEntries { allowed, required }) => {
                assert_eq!((*allowed, *required), (child_entries - 1, child_entries))
            }
            (1, W::ChildTextBytes { allowed, required }) => {
                assert_eq!((*allowed, *required), (child_text - 1, child_text))
            }
            (2, W::ChildCounterBits { allowed, required }) => {
                assert_eq!((*allowed, *required), (child_bits - 1, child_bits))
            }
            (3, W::ChildMatching { allowed, required }) => {
                assert_eq!((*allowed, *required), (child_matching - 1, child_matching))
            }
            (4, W::ResultRecords { allowed, required }) => {
                assert_eq!((*allowed, *required), (9, 10))
            }
            _ => return Err(format!("wrong child quota: {case}: {error}").into()),
        }
        assert!(core::ptr::eq(error.inputs(), &joined));
        assert!(core::ptr::eq(error.limits(), &limits));
        assert_eq!(error.demand(), Some(demand));
        assert_eq!(error.population_shape(), Some(shape));
        assert_eq!(error.shape(), Some(work_shape));
        assert_eq!(source.walks.get(), 1);
        assert!(!format!("{error:?} {error}").contains("private"));
    }
    let empty_content = DraftingProgramSemanticChildContentLimits::new(0, 0, 0);
    let content_first = DraftingProgramSemanticWorkLimits::new(too_short, 0, empty_content, 0, 0);
    let error = joined
        .admit_semantic_work(&content_first)
        .expect_err("fresh content before child work");
    let W::Population(cause) = error.refusal() else {
        return Err("missing original content cause".into());
    };
    assert!(matches!(
        cause.refusal(),
        R::Resources(DesignResourceLimitError::Count {
            unit: DesignResourceCount::TextBytes,
            location: DesignResourceLocation {
                field: "selected_profile.content_version",
                index: Some(1),
                ..
            },
            ..
        })
    ));
    assert!(core::ptr::eq(cause.inputs(), &joined));
    assert!(core::ptr::eq(cause.limits(), content_first.population()));
    assert_eq!(error.demand(), None);
    assert_eq!(error.population_shape(), None);
    assert_eq!(error.shape(), None);
    assert!(std::error::Error::source(&error).is_some());
    let occurrence_first = DraftingProgramSemanticWorkLimits::new(
        DraftingProgramSemanticPopulationLimits::new(policy, 5, 0, 0),
        usize::MAX,
        empty_content,
        0,
        0,
    );
    let error = joined
        .admit_semantic_work(&occurrence_first)
        .expect_err("population before child arithmetic");
    let W::Population(cause) = error.refusal() else {
        return Err("missing original population cause".into());
    };
    assert_eq!(
        cause.refusal(),
        R::Occurrences {
            allowed: 5,
            required: 6
        }
    );
    assert_eq!(error.demand(), Some(demand));
    assert_eq!(error.population_shape(), Some(shape));
    assert_eq!(error.shape(), None);
    let overflowing =
        DraftingProgramSemanticWorkLimits::new(population_limits, usize::MAX, empty_content, 0, 0);
    let error = joined
        .admit_semantic_work(&overflowing)
        .expect_err("checked result envelope before quotas");
    assert!(matches!(
        error.refusal(),
        W::Arithmetic(DraftingProgramSemanticWorkArithmetic::ResultRecords)
    ));
    assert_eq!(error.demand(), Some(demand));
    assert_eq!(error.population_shape(), Some(shape));
    assert_eq!(error.shape(), None);
    let no_children =
        DraftingProgramSemanticWorkLimits::new(population_limits, 0, empty_content, 0, 8);
    let zero = joined
        .admit_semantic_work(&no_children)
        .map_err(|e| e.to_string())?;
    assert_eq!(zero.shape().child_matching_bound(), 0);
    assert_eq!(zero.shape().result_record_bound(), 8);
    let mut zero = zero.into_budget();
    let error = zero
        .reserve_child_passes(1)
        .expect_err("zero permits no child stage");
    assert!(matches!(
        error.refusal(),
        W::ChildPasses {
            allowed: 0,
            reserved: 0,
            requested: 1
        }
    ));
    assert_eq!((zero.remaining_passes(), zero.reserved_passes()), (0, 0));
    // Reservations consume the unique plan without rewalking or querying any source.
    source.walks.set(0);
    let mut budget = work.into_budget();
    budget.reserve_child_passes(1).map_err(|e| e.to_string())?;
    let error = budget
        .reserve_child_passes(2)
        .expect_err("whole stage refuses atomically");
    assert!(matches!(
        error.refusal(),
        W::ChildPasses {
            allowed: 2,
            reserved: 1,
            requested: 2
        }
    ));
    assert!(core::ptr::eq(error.inputs(), &joined));
    assert!(core::ptr::eq(error.limits(), &work_limits));
    assert_eq!(error.demand(), Some(demand));
    assert_eq!(error.population_shape(), Some(shape));
    assert_eq!(error.shape(), Some(work_shape));
    assert_eq!(
        (budget.remaining_passes(), budget.reserved_passes()),
        (1, 1)
    );
    budget.reserve_child_passes(1).map_err(|e| e.to_string())?;
    budget.reserve_child_passes(0).map_err(|e| e.to_string())?;
    let error = budget
        .reserve_child_passes(1)
        .expect_err("exhausted original capacity");
    assert!(matches!(
        error.refusal(),
        W::ChildPasses {
            allowed: 2,
            reserved: 2,
            requested: 1
        }
    ));
    assert_eq!(
        (budget.remaining_passes(), budget.reserved_passes()),
        (0, 2)
    );
    assert!(core::ptr::eq(budget.work().inputs(), &joined));
    assert_eq!(source.walks.get(), 0);
    assert_eq!(source.base.trace.get(), 2);
    assert_eq!(source.alternate.trace.get(), 2);
    assert_eq!(joined.selections().inputs().rows().len(), 2);
    use DraftingProgramSelectionCoverageRefusal as C;
    source.walks.set(0);
    let error = joined
        .check_direct_selections(&work_limits)
        .expect_err("first raw invalid key");
    assert!(matches!(
        error.refusal(),
        C::UnexpectedSelection {
            row: 0,
            key: DraftingSemanticSelectionKey::Rule {
                ordinal: usize::MAX
            }
        }
    ));
    assert!(core::ptr::eq(error.inputs(), &joined));
    assert!(core::ptr::eq(error.limits(), &work_limits));
    assert_eq!(error.demand(), Some(demand));
    assert_eq!(error.population_shape(), Some(shape));
    assert_eq!(error.work_shape(), Some(work_shape));
    assert_eq!(source.walks.get(), 1);
    let error = joined
        .check_direct_selections(&content_first)
        .expect_err("whole work precedes invalid key");
    assert!(matches!(error.refusal(),C::Work(cause) if matches!(cause.refusal(),W::Population(_))));
    assert_eq!(error.demand(), None);
    assert_eq!(error.population_shape(), None);
    assert_eq!(error.work_shape(), None);
    assert!(std::error::Error::source(&error).is_some());
    let opening = DraftingSemanticSelectionLocation::Pocket {
        pocket: 0,
        field: DraftingPocketSelectionField::Opening,
    };
    let target = DraftingSemanticSelectionLocation::Native {
        position: 1,
        field: DraftingNativeSelectionField::Target(DraftingNativeSemanticTargetRole::Grainline),
    };
    let profile = |location| DraftingSemanticSelectionRow::Profile {
        location,
        selected: &selected,
    };
    let cases = [
        vec![],
        vec![profile(opening)],
        vec![profile(target), profile(opening)],
        vec![profile(DraftingSemanticSelectionLocation::Pocket {
            pocket: 0,
            field: DraftingPocketSelectionField::Served,
        })],
        vec![profile(DraftingSemanticSelectionLocation::Native {
            position: 0,
            field: DraftingNativeSelectionField::Target(
                DraftingNativeSemanticTargetRole::Grainline,
            ),
        })],
        vec![DraftingSemanticSelectionRow::Rule {
            ordinal: 1,
            origin: DraftingRuleSelection::Design,
        }],
        vec![profile(DraftingSemanticSelectionLocation::Native {
            position: 1,
            field: DraftingNativeSelectionField::Target(
                DraftingNativeSemanticTargetRole::OffsetAllowance,
            ),
        })],
        vec![profile(target)],
    ];
    let root_scope = context
        .prepare_design_semantic_sources(content)
        .map_err(|e| e.to_string())?;
    let foreign_scope = alternate_root
        .context()
        .prepare_design_semantic_sources(content)
        .map_err(|e| e.to_string())?;
    let provider_scope = alternate_provider
        .context()
        .prepare_design_semantic_sources(content)
        .map_err(|e| e.to_string())?;
    let frame_scope = alternate_frame
        .context()
        .prepare_design_semantic_sources(content)
        .map_err(|e| e.to_string())?;
    let snapshot_scope = alternate_snapshot
        .context()
        .prepare_design_semantic_sources(content)
        .map_err(|e| e.to_string())?;
    let both_scope = alternate_both
        .context()
        .prepare_design_semantic_sources(content)
        .map_err(|e| e.to_string())?;
    let reset_owner_trace = || {
        for provider in [&source, &other_source, &foreign_root] {
            provider.walks.set(0);
            provider.base.trace.set(0);
            provider.alternate.trace.set(0);
        }
    };
    for (case, rows) in cases.into_iter().enumerate() {
        let row_count = rows.len();
        let pin_count = rows
            .iter()
            .filter(|row| matches!(row, DraftingSemanticSelectionRow::Profile { .. }))
            .count();
        let expected_entries = garment_demand.collection_entries() + 4 + row_count;
        let expected_text = garment_demand.text_bytes()
            + pin_count * pin_text.iter().map(|value| value.len()).sum::<usize>();
        let raw = DraftingSemanticSelectionInputs::new(rows);
        let local = raw
            .admit(DraftingSemanticSelectionLimits::new(content, 4))
            .map_err(|e| e.to_string())?;
        let actual = geometry
            .semantic_inputs(&actual_garment, &local)
            .map_err(|e| e.to_string())?;
        let policy = DraftingProgramSemanticWorkLimits::new(
            DraftingProgramSemanticPopulationLimits::new(
                limits(expected_entries, expected_text, bits.max(1))?,
                6,
                12 * row_count,
                6 + row_count,
            ),
            2,
            DraftingProgramSemanticChildContentLimits::new(
                2 * expected_entries as u128,
                2 * expected_text as u128,
                2 * u128::from(bits),
            ),
            2 * ((expected_entries + 8) * (expected_entries + 8)) as u128,
            8 + row_count,
        );
        source.walks.set(0);
        let result = actual.check_direct_selections(&policy);
        assert_eq!(source.walks.get(), 1);
        assert_eq!(source.base.trace.get(), 2);
        assert_eq!(source.alternate.trace.get(), 2);
        if matches!(case, 1 | 2) {
            let mut proof = result.map_err(|e| e.to_string())?;
            assert!(core::ptr::eq(proof.inputs(), &actual));
            assert!(core::ptr::eq(proof.limits(), &policy));
            assert_eq!(
                (
                    proof.work().demand().collection_entries(),
                    proof.work().demand().text_bytes(),
                    proof.work().demand().counter_bits()
                ),
                (expected_entries, expected_text, bits)
            );
            assert_eq!(proof.associations().len(), 6);
            for (index, association) in proof.associations().iter().enumerate() {
                let expected_row = match (case, index) {
                    (1, 3) => Some(0),
                    (2, 3) => Some(1),
                    (2, 5) => Some(0),
                    _ => None,
                };
                assert_eq!(association.row_index(), expected_row);
            }
            assert_eq!(
                proof
                    .associations()
                    .get(3)
                    .ok_or("actual opening association")?
                    .location(),
                opening
            );
            let native = proof
                .associations()
                .get(5)
                .ok_or("actual target association")?;
            assert_eq!(native.location(), target);
            assert_eq!(
                native.need(),
                DraftingSemanticSelectionNeed::DeferredTargetProfile(target)
            );
            let expected_kinds = if case == 1 {
                vec![DraftingProgramSelectionRowKind::DirectRequirement]
            } else {
                vec![
                    DraftingProgramSelectionRowKind::DeferredTargetContext,
                    DraftingProgramSelectionRowKind::DirectRequirement,
                ]
            };
            assert_eq!(proof.row_kinds(), expected_kinds);
            source.walks.set(0);
            proof.reserve_child_passes(1).map_err(|e| e.to_string())?;
            let error = proof
                .reserve_child_passes(2)
                .expect_err("same unique cursor after coverage");
            assert!(matches!(
                error.refusal(),
                W::ChildPasses {
                    allowed: 2,
                    reserved: 1,
                    requested: 2
                }
            ));
            assert!(core::ptr::eq(error.inputs(), &actual));
            assert!(core::ptr::eq(error.limits(), &policy));
            assert_eq!(
                (
                    proof.remaining_child_passes(),
                    proof.reserved_child_passes()
                ),
                (1, 1)
            );
            assert_eq!(source.walks.get(), 0);
            assert!(!format!("{proof:?}").contains("private"));
            for (candidate, expected) in [
                (&foreign_scope, DraftingProgramSemanticOwnerIssue::Root),
                (&provider_scope, DraftingProgramSemanticOwnerIssue::Provider),
                (&frame_scope, DraftingProgramSemanticOwnerIssue::Frame),
                (&snapshot_scope, DraftingProgramSemanticOwnerIssue::Snapshot),
                (&both_scope, DraftingProgramSemanticOwnerIssue::Frame),
            ] {
                let checked = actual
                    .check_direct_selections(&policy)
                    .map_err(|e| e.to_string())?;
                reset_owner_trace();
                let error = checked
                    .root_input_scope(candidate)
                    .expect_err("actual held Root scope owner");
                assert_eq!(error.issue(), expected);
                assert!(core::ptr::eq(error.inputs(), &actual));
                assert!(core::ptr::eq(error.scope(), candidate));
                assert!(core::ptr::eq(error.selections().limits(), &policy));
                assert_eq!(error.selections().associations().len(), 6);
                assert_eq!(
                    (
                        error.selections().remaining_child_passes(),
                        error.selections().reserved_child_passes()
                    ),
                    (2, 0)
                );
                for provider in [&source, &other_source, &foreign_root] {
                    assert_eq!(provider.walks.get(), 0);
                    assert_eq!(provider.base.trace.get(), 0);
                    assert_eq!(provider.alternate.trace.get(), 0);
                }
                assert!(!format!("{error:?} {error}").contains("private"));
            }
            reset_owner_trace();
            let owners = proof
                .root_input_scope(&root_scope)
                .map_err(|e| e.to_string())?;
            assert!(core::ptr::eq(owners.scope(), &root_scope));
            assert!(core::ptr::eq(owners.inputs(), &actual));
            assert!(core::ptr::eq(owners.selections().limits(), &policy));
            assert_eq!(
                (
                    owners.selections().remaining_child_passes(),
                    owners.selections().reserved_child_passes()
                ),
                (1, 1)
            );
            assert_eq!(owners.selections().associations().len(), 6);
            assert_eq!(owners.selections().row_kinds().len(), row_count);
            for provider in [&source, &other_source, &foreign_root] {
                assert_eq!(provider.walks.get(), 0);
                assert_eq!(provider.base.trace.get(), 0);
                assert_eq!(provider.alternate.trace.get(), 0);
            }
            assert!(!format!("{owners:?}").contains("private"));
            let members = owners
                .bind_root_input_members()
                .map_err(|e| e.to_string())?;
            assert!(members.records().is_empty());
            assert_eq!(members.demand(), None);
            assert!(core::ptr::eq(members.owner().inputs(), &actual));
            assert!(core::ptr::eq(members.owner().scope(), &root_scope));
            assert_eq!(
                (
                    members.owner().selections().remaining_child_passes(),
                    members.owner().selections().reserved_child_passes()
                ),
                (1, 1)
            );
            assert_eq!(source.walks.get(), 0);
            assert_eq!(source.base.trace.get(), 0);
            assert_eq!(source.alternate.trace.get(), 0);
        } else {
            let error = result.expect_err("no accepted coverage prefix");
            match case {
                0 | 7 => assert!(
                    matches!(error.refusal(),C::MissingSelection {location,key:DraftingSemanticSelectionKey::Profile(actual)} if *location==opening && *actual==opening)
                ),
                _ => assert!(matches!(
                    error.refusal(),
                    C::UnexpectedSelection { row: 0, .. }
                )),
            }
            assert!(core::ptr::eq(error.inputs(), &actual));
            assert!(core::ptr::eq(error.limits(), &policy));
            assert!(error.demand().is_some());
            assert!(error.population_shape().is_some());
            assert!(error.work_shape().is_some());
            assert!(!format!("{error:?} {error}").contains("private"));
        }
    }
    Ok(())
}
