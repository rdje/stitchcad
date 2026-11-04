//! Actual provider-owned Profile records,complete fresh content and original first causes.
use super::{
    limits, root, scope, semantic_root::canonical, EmptyHost, Frame, Snapshot, Source, SourceError,
    TestResult,
};
use sc_core::{
    command::DesignRevision, construction::*, design::*, ontology::*, value::CanonicalInputs,
};
use std::fmt;
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn pin() -> TestResult<SelectedProfilePin> {
    Ok(SelectedProfilePin::new(SelectedProfilePinDefinition {
        profile_id: "atelier é".into(),
        schema_version: " 2.0 ".into(),
        content_version: "7".into(),
    })?)
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Fault {
    None,
    Root,
    Profile,
    Schema,
    Content,
    Snapshot,
    Unavailable,
}
#[derive(Debug)]
pub(super) enum Cause {
    Base(SourceError),
    Unavailable,
    MissingParameter { parameter: EntityId },
}
impl fmt::Display for Cause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("original Profile fixture source cause")
    }
}
impl std::error::Error for Cause {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Base(e) => Some(e),
            Self::Unavailable | Self::MissingParameter { .. } => None,
        }
    }
}
pub(super) struct Body {
    pub(super) text: String,
}
pub(super) struct Provider<'a> {
    base: Source<'a>,
    other: &'a DesignInputs<EmptyHost>,
    selected: SelectedProfilePin,
    inputs: CanonicalInputs,
    body: DraftingSourceRecord<Body>,
    other_snapshot: DraftingSourceRecord<Snapshot>,
    fault: Fault,
    ignore: bool,
}
impl<'a> Provider<'a> {
    pub(super) fn new(
        root: &'a DesignInputs<EmptyHost>,
        other: &'a DesignInputs<EmptyHost>,
        fault: Fault,
    ) -> TestResult<Self> {
        let mut definition = pin()?.definition().clone();
        if matches!(fault, Fault::Root | Fault::Profile) {
            definition.profile_id = "other factory".into();
        }
        if matches!(fault, Fault::Root | Fault::Profile | Fault::Schema) {
            definition.schema_version = "other schema".into();
        }
        if matches!(
            fault,
            Fault::Root | Fault::Profile | Fault::Schema | Fault::Content
        ) {
            definition.content_version = "other content".into();
        }
        Ok(Self {
            base: Source::new(root),
            other,
            selected: SelectedProfilePin::new(definition)?,
            inputs: canonical()?,
            body: DraftingSourceRecord::new(Body {
                text: "private original Profile body".into(),
            }),
            other_snapshot: DraftingSourceRecord::new(Snapshot {
                label: "private alternate Profile snapshot".into(),
            }),
            fault,
            ignore: false,
        })
    }
    fn location(&self, field: &'static str) -> DesignResourceLocation {
        DesignResourceLocation {
            scope: DesignResourceScope::SourceHost,
            owner: None,
            field,
            index: None,
        }
    }
    pub(super) fn text(&self, requested: &SelectedProfilePin) -> usize {
        let pin_text = |p: &SelectedProfilePin| {
            let d = p.definition();
            d.profile_id.len() + d.schema_version.len() + d.content_version.len()
        };
        8 + self.base.frame.data().label.len()
            + self.base.snapshot.data().label.len()
            + 8
            + 8
            + pin_text(&self.selected)
            + self.body.data().text.len()
            + self.other_snapshot.data().label.len()
            + pin_text(requested)
    }
}
impl DraftingContextSource<EmptyHost> for Provider<'_> {
    type Frame = Frame;
    type Snapshot = Snapshot;
    type Geometry = [u8];
    type Error = Cause;
    fn root(&self) -> &DesignInputs<EmptyHost> {
        self.base.root()
    }
    fn snapshot_revision(&self) -> &DesignRevision {
        self.base.snapshot_revision()
    }
    fn frame(&self) -> &DraftingSourceRecord<Frame> {
        self.base.frame()
    }
    fn snapshot(&self) -> &DraftingSourceRecord<Snapshot> {
        self.base.snapshot()
    }
    fn charge_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        // All owned/retained sources count,including the unused alternate Root/snapshot.
        let results = [
            self.base.charge_resources(sink),
            charge_design_input_resources(self.other, sink),
            charge_canonical_input_resources(&self.inputs, sink),
            charge_selected_profile_pin_resources(
                &self.selected,
                sink,
                DesignResourceScope::SourceHost,
                None,
                None,
            ),
            sink.charge_text(&self.body.data().text, self.location("profile.body")),
            sink.charge_text(
                &self.other_snapshot.data().label,
                self.location("profile.other_snapshot.label"),
            ),
        ];
        for result in results {
            if !self.ignore {
                result?;
            }
        }
        Ok(())
    }
    fn validate_current(&self) -> Result<(), Cause> {
        self.base.validate_current().map_err(Cause::Base)
    }
    fn geometry(
        &self,
        r: DraftingGeometryReference,
    ) -> DraftingGeometrySourceResult<'_, Frame, Snapshot, [u8], Cause> {
        self.base.geometry(r).map_err(Cause::Base)
    }
}
impl DraftingProfileProvider<EmptyHost> for Provider<'_> {
    type Profile = Body;
    fn profile_source<'s>(
        &'s self,
        selected: &SelectedProfilePin,
    ) -> Result<DraftingProfileSource<'s, EmptyHost, Snapshot, Body>, Cause> {
        self.base.observed(7);
        if self.fault == Fault::Unavailable
            || (self.fault == Fault::None && selected != &self.selected)
        {
            return Err(Cause::Unavailable);
        }
        Ok(DraftingProfileSource {
            root: if self.fault == Fault::Root {
                self.other
            } else {
                self.base.root
            },
            selected: &self.selected,
            inputs: &self.inputs,
            profile: &self.body,
            snapshot: if matches!(
                self.fault,
                Fault::Root | Fault::Profile | Fault::Schema | Fault::Content | Fault::Snapshot
            ) {
                &self.other_snapshot
            } else {
                &self.base.snapshot
            },
        })
    }
}
#[test]
fn selected_profile_binding_keeps_original_source_and_request_loans_after_complete_fresh_accounting(
) -> TestResult {
    let root = root()?;
    let other = super::root()?;
    let source = Provider::new(&root, &other, Fault::None)?;
    let context = scope(&root)?
        .bind_drafting_context(&source, limits(10000, 10000, 1000)?)
        .map_err(|e| e.to_string())?;
    // Request pin is a distinct equal-data object created after the containing context.
    let requested = pin()?;
    let text = source.text(&requested);
    let policy = limits(7, text, 9)?;
    source.base.trace.set(0);
    let bound = context
        .bind_profile_source(&requested, policy)
        .map_err(|e| e.to_string())?;
    assert_eq!(source.base.trace.get(), 27); // complete provider walk,then one lookup only
    assert!(std::ptr::eq(bound.selected(), &requested));
    assert!(std::ptr::eq(bound.source().selected, &source.selected));
    assert!(!std::ptr::eq(bound.selected(), bound.source().selected));
    assert_eq!(bound.selected(), bound.source().selected);
    assert!(std::ptr::eq(bound.context().scope().root(), &root));
    assert!(std::ptr::eq(bound.source().inputs, &source.inputs));
    assert!(bound.source().profile.is_same_record(&source.body));
    assert!(bound.source().snapshot.is_same_record(context.snapshot()));
    assert_eq!(bound.limits(), policy);
    let demand = bound.demand();
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (7, text, 9)
    );
    source.base.trace.set(0);
    let mut meter = DesignResourceMeter::new(limits(8, text + 2, 10)?);
    let at = DesignResourceLocation {
        scope: DesignResourceScope::CommandState,
        owner: None,
        field: "prior",
        index: None,
    };
    meter.charge_entries(1, at)?;
    meter.charge_text("é", at)?;
    meter.charge_counter_bits(1, at)?;
    bound.charge_resources(&mut meter)?;
    let sum = meter.finish()?;
    assert_eq!(
        (
            sum.collection_entries(),
            sum.text_bytes(),
            sum.counter_bits()
        ),
        (8, text + 2, 10)
    );
    assert_eq!(source.base.trace.get(), 2); // no repeat lookup,currentness or value query
    assert!(!format!("{bound:?}").contains("atelier"));
    assert_eq!(
        requested.validation(),
        ProfileBindingValidation::DeferredToG4
    );
    Ok(())
}
#[test]
fn profile_binding_preserves_typed_provider_and_first_root_pin_field_snapshot_causes() -> TestResult
{
    for (fault, issue) in [
        (Fault::Root, DraftingProfileSourceIssue::Root),
        (
            Fault::Profile,
            DraftingProfileSourceIssue::SelectedPin(SelectedProfilePinField::ProfileId),
        ),
        (
            Fault::Schema,
            DraftingProfileSourceIssue::SelectedPin(SelectedProfilePinField::SchemaVersion),
        ),
        (
            Fault::Content,
            DraftingProfileSourceIssue::SelectedPin(SelectedProfilePinField::ContentVersion),
        ),
        (Fault::Snapshot, DraftingProfileSourceIssue::Snapshot),
    ] {
        let root = root()?;
        let other = super::root()?;
        let source = Provider::new(&root, &other, fault)?;
        let context = scope(&root)?
            .bind_drafting_context(&source, limits(10000, 10000, 1000)?)
            .map_err(|e| e.to_string())?;
        let requested = pin()?;
        let policy = limits(10000, 10000, 1000)?;
        source.base.trace.set(0);
        let error = context
            .bind_profile_source(&requested, policy)
            .err()
            .ok_or("actual source mismatch")?;
        assert!(
            matches!(error.refusal(),DraftingProfileSourceRefusal::Mismatch(actual) if *actual==issue)
        );
        assert!(std::ptr::eq(error.selected(), &requested));
        assert_eq!(error.limits(), policy);
        assert!(std::ptr::eq(error.context().scope().root(), &root));
        let returned = error.returned().ok_or("original returned records")?;
        assert!(returned.profile.is_same_record(&source.body));
        assert!(std::ptr::eq(returned.selected, &source.selected));
        assert_eq!(source.base.trace.get(), 27);
        assert!(!format!("{error:?}").contains("atelier"));
    }
    let root = root()?;
    let other = super::root()?;
    let source = Provider::new(&root, &other, Fault::Unavailable)?;
    let context = scope(&root)?
        .bind_drafting_context(&source, limits(10000, 10000, 1000)?)
        .map_err(|e| e.to_string())?;
    let requested = pin()?;
    source.base.trace.set(0);
    let error = context
        .bind_profile_source(&requested, limits(10000, 10000, 1000)?)
        .err()
        .ok_or("original provider refusal")?;
    assert!(matches!(
        error.refusal(),
        DraftingProfileSourceRefusal::Source(Cause::Unavailable)
    ));
    assert!(error.returned().is_none());
    assert_eq!(source.base.trace.get(), 27);
    assert!(std::error::Error::source(&error).is_some_and(|e| e.is::<Cause>()));
    Ok(())
}
#[test]
fn profile_request_and_all_provider_fields_refuse_before_lookup_even_when_sink_failure_is_ignored(
) -> TestResult {
    let root = root()?;
    let other = super::root()?;
    for ignore in [false, true] {
        let mut source = Provider::new(&root, &other, Fault::None)?;
        source.ignore = ignore;
        let context = scope(&root)?
            .bind_drafting_context(&source, limits(10000, 10000, 1000)?)
            .map_err(|e| e.to_string())?;
        let requested = pin()?;
        source.base.trace.set(0);
        let error = context
            .bind_profile_source(&requested, limits(2, 10000, 1000)?)
            .err()
            .ok_or("first provider cache refusal")?;
        let expected = DesignResourceLimitError::Count {
            unit: DesignResourceCount::CollectionEntries,
            maximum: 2,
            consumed: 0,
            requested: 3,
            location: source.base.location("drafting.cache"),
        };
        assert!(
            matches!(error.refusal(),DraftingProfileSourceRefusal::Resources(actual) if *actual==expected)
        );
        assert!(error.returned().is_none());
        assert_eq!(source.base.trace.get(), 2);
    }
    let source = Provider::new(&root, &other, Fault::None)?;
    let context = scope(&root)?
        .bind_drafting_context(&source, limits(10000, 10000, 1000)?)
        .map_err(|e| e.to_string())?;
    let requested = pin()?;
    let text = source.text(&requested);
    source.base.trace.set(0);
    let error = context
        .bind_profile_source(&requested, limits(7, text - 1, 9)?)
        .err()
        .ok_or("complete requested pin text")?;
    let expected = DesignResourceLimitError::Count {
        unit: DesignResourceCount::TextBytes,
        maximum: text - 1,
        consumed: text - 1,
        requested: 1,
        location: DesignResourceLocation {
            scope: DesignResourceScope::CommandState,
            owner: Some(id(1)),
            field: "selected_profile.content_version",
            index: None,
        },
    };
    assert!(
        matches!(error.refusal(),DraftingProfileSourceRefusal::Resources(actual) if *actual==expected)
    );
    assert!(error.returned().is_none());
    assert_eq!(source.base.trace.get(), 2);
    Ok(())
}
