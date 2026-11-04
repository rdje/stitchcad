//! Fresh original parameter metadata,exact selected-source header and lending canonical causes.
use super::{
    limits,
    profile_binding::{Body, Cause, Fault as BaseFault, Provider as Base},
    root, scope,
    semantic_root::canonical,
    EmptyHost, Frame, Snapshot, TestResult,
};
use sc_core::{
    command::DesignRevision, construction::*, design::*, name::MachineToken, ontology::*, value::*,
};
use std::cell::Cell;
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
enum Fault {
    None,
    Root,
    Pin,
    Inventory,
    Profile,
    Snapshot,
    Parameter,
    Detached,
    Missing,
}
struct Row {
    parameter: ProfileParameterRef,
    target: EntityId,
    text: String,
}
struct Provider<'a> {
    inner: Base<'a>,
    other: &'a DesignInputs<EmptyHost>,
    rows: Vec<DraftingSourceRecord<Row>>,
    other_pin: SelectedProfilePin,
    other_inputs: CanonicalInputs,
    other_profile: DraftingSourceRecord<Body>,
    other_snapshot: DraftingSourceRecord<Snapshot>,
    missing: LogicalInputDeclaration,
    fault: Fault,
    ignore: bool,
    charges: Cell<usize>,
    profiles: Cell<usize>,
    queries: Cell<usize>,
}
impl<'a> Provider<'a> {
    fn new(
        root: &'a DesignInputs<EmptyHost>,
        other: &'a DesignInputs<EmptyHost>,
        fault: Fault,
    ) -> TestResult<Self> {
        Ok(Self {
            inner: Base::new(root, other, BaseFault::None)?,
            other,
            rows: vec![
                DraftingSourceRecord::new(Row {
                    parameter: ProfileParameterRef::new(id(600)),
                    target: id(90),
                    text: "private original parameter".into(),
                }),
                DraftingSourceRecord::new(Row {
                    parameter: ProfileParameterRef::new(id(601)),
                    target: id(90),
                    text: "private other parameter".into(),
                }),
            ],
            other_pin: pin()?,
            other_inputs: canonical()?,
            other_profile: DraftingSourceRecord::new(Body {
                text: "private alternate Profile body".into(),
            }),
            other_snapshot: DraftingSourceRecord::new(Snapshot {
                label: "private alternate parameter snapshot".into(),
            }),
            missing: LogicalInputDeclaration::new(
                LogicalInputDeclarationDefinition {
                    id: id(999),
                    source: id(4000),
                    kind: LogicalInputKind::PocketOpening,
                    domain: vec![MachineToken::new("option_a")?],
                    state: LogicalInputState::Unknown {
                        observation: id(4001),
                    },
                },
                LogicalInputLimits::new(1, 0, 8, 2),
            )?,
            fault,
            ignore: false,
            charges: Cell::new(0),
            profiles: Cell::new(0),
            queries: Cell::new(0),
        })
    }
    fn at(&self, field: &'static str, index: Option<usize>) -> DesignResourceLocation {
        DesignResourceLocation {
            scope: DesignResourceScope::SourceHost,
            owner: None,
            field,
            index,
        }
    }
    fn extra(&self, sink: &mut dyn DesignResourceSink) -> Result<(), DesignResourceLimitError> {
        sink.charge_entries(self.rows.len(), self.at("profile.parameters", None))?;
        for (index, row) in self.rows.iter().enumerate() {
            sink.charge_text(
                &row.data().text,
                self.at("profile.parameters.text", Some(index)),
            )?;
        }
        charge_selected_profile_pin_resources(
            &self.other_pin,
            sink,
            DesignResourceScope::SourceHost,
            None,
            None,
        )?;
        charge_canonical_input_resources(&self.other_inputs, sink)?;
        charge_logical_declaration_resources(&self.missing, sink)?;
        sink.charge_text(
            &self.other_profile.data().text,
            self.at("profile.parameter.other_profile", None),
        )?;
        sink.charge_text(
            &self.other_snapshot.data().label,
            self.at("profile.parameter.other_snapshot", None),
        )
    }
    fn text(&self, requested: &SelectedProfilePin) -> usize {
        self.inner.text(requested)
            + self.rows.iter().map(|r| r.data().text.len()).sum::<usize>()
            + 16
            + 8
            + 8
            + self.other_profile.data().text.len()
            + self.other_snapshot.data().label.len()
    }
}
impl DraftingContextSource<EmptyHost> for Provider<'_> {
    type Frame = Frame;
    type Snapshot = Snapshot;
    type Geometry = [u8];
    type Error = Cause;
    fn root(&self) -> &DesignInputs<EmptyHost> {
        self.inner.root()
    }
    fn snapshot_revision(&self) -> &DesignRevision {
        self.inner.snapshot_revision()
    }
    fn frame(&self) -> &DraftingSourceRecord<Frame> {
        self.inner.frame()
    }
    fn snapshot(&self) -> &DraftingSourceRecord<Snapshot> {
        self.inner.snapshot()
    }
    fn validate_current(&self) -> Result<(), Cause> {
        self.inner.validate_current()
    }
    fn geometry(
        &self,
        r: DraftingGeometryReference,
    ) -> DraftingGeometrySourceResult<'_, Frame, Snapshot, [u8], Cause> {
        self.inner.geometry(r)
    }
    fn charge_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        self.charges.set(self.charges.get() + 1);
        let a = self.inner.charge_resources(sink);
        let b = self.extra(sink);
        if self.ignore {
            let _ = (a, b);
            Ok(())
        } else {
            a?;
            b
        }
    }
}
impl DraftingProfileProvider<EmptyHost> for Provider<'_> {
    type Profile = Body;
    fn profile_source<'s>(
        &'s self,
        selected: &SelectedProfilePin,
    ) -> Result<DraftingProfileSource<'s, EmptyHost, Snapshot, Body>, Cause> {
        self.profiles.set(self.profiles.get() + 1);
        self.inner.profile_source(selected)
    }
}
impl DraftingProfileLogicalProvider<EmptyHost> for Provider<'_> {
    type Metadata = Row;
    fn profile_logical_source<'s>(
        &'s self,
        mut profile: DraftingProfileSource<'s, EmptyHost, Snapshot, Body>,
        parameter: ProfileParameterRef,
    ) -> Result<DraftingProfileLogicalSource<'s, EmptyHost, Snapshot, Body, Row>, Cause> {
        self.queries.set(self.queries.get() + 1);
        let index = if self.fault == Fault::Parameter { 1 } else { 0 };
        let row = self
            .rows
            .get(index)
            .filter(|r| self.fault == Fault::Parameter || r.data().parameter == parameter)
            .ok_or(Cause::MissingParameter {
                parameter: parameter.id(),
            })?;
        let original = profile
            .inputs
            .logicals()
            .declaration(row.data().target)
            .ok_or(Cause::Unavailable)?;
        let declaration = match self.fault {
            Fault::Detached => self
                .other_inputs
                .logicals()
                .declaration(id(90))
                .ok_or(Cause::Unavailable)?,
            Fault::Missing => &self.missing,
            _ => original,
        };
        match self.fault {
            Fault::Root => {
                profile.root = self.other;
                profile.selected = &self.other_pin;
                profile.snapshot = &self.other_snapshot;
            }
            Fault::Pin => {
                profile.selected = &self.other_pin;
                profile.inputs = &self.other_inputs;
                profile.profile = &self.other_profile;
            }
            Fault::Inventory => {
                profile.inputs = &self.other_inputs;
                profile.profile = &self.other_profile;
            }
            Fault::Profile => {
                profile.profile = &self.other_profile;
                profile.snapshot = &self.other_snapshot;
            }
            Fault::Snapshot => profile.snapshot = &self.other_snapshot,
            _ => {}
        }
        Ok(DraftingProfileLogicalSource {
            profile,
            parameter: &row.data().parameter,
            declaration,
            metadata: row,
        })
    }
}
#[test]
fn original_parameter_metadata_binds_only_actual_profile_canonical_member_without_value_or_repeat_profile_lookup(
) -> TestResult {
    let root = root()?;
    let other = super::root()?;
    let source = Provider::new(&root, &other, Fault::None)?;
    let context = scope(&root)?
        .bind_drafting_context(&source, limits(10000, 10000, 1000)?)
        .map_err(|e| e.to_string())?;
    let selected = pin()?;
    let text = source.text(&selected);
    let policy = limits(14, text, 9)?;
    let parent = context
        .bind_profile_source(&selected, policy)
        .map_err(|e| e.to_string())?;
    source.charges.set(0);
    source.profiles.set(0);
    source.queries.set(0);
    let parameter = ProfileParameterRef::new(id(600));
    let bound = parent
        .bind_logical_parameter(parameter, LogicalInputKind::PocketOpening, policy)
        .map_err(|e| e.to_string())?;
    assert!(std::ptr::eq(bound.parent(), &parent));
    assert_eq!(bound.parameter(), parameter);
    assert_eq!(bound.kind(), LogicalInputKind::PocketOpening);
    assert_eq!(bound.limits(), policy);
    assert_eq!(
        (
            source.charges.get(),
            source.profiles.get(),
            source.queries.get()
        ),
        (1, 0, 1)
    );
    let original = parent
        .source()
        .inputs
        .logicals()
        .declaration(id(90))
        .ok_or("original canonical member")?;
    assert!(std::ptr::eq(
        bound.binding().inputs(),
        parent.source().inputs.logicals()
    ));
    assert!(std::ptr::eq(bound.declaration(), original));
    assert_eq!(bound.binding().index(), 0);
    let row = source.rows.first().ok_or("actual original metadata")?;
    assert!(bound.source().metadata.is_same_record(row));
    assert!(std::ptr::eq(
        bound.source().parameter,
        &row.data().parameter
    ));
    assert_eq!(bound.parameter().id(), id(600));
    assert_eq!(bound.declaration().id(), id(90));
    assert!(
        matches!(bound.declaration().definition().state,LogicalInputState::Derived {operation} if operation==id(80))
    );
    let d = bound.demand();
    assert_eq!(
        (d.collection_entries(), d.text_bytes(), d.counter_bits()),
        (14, text, 9)
    );
    source.charges.set(0);
    source.queries.set(0);
    let mut meter = DesignResourceMeter::new(limits(15, text + 2, 10)?);
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
    let d = meter.finish()?;
    assert_eq!(
        (d.collection_entries(), d.text_bytes(), d.counter_bits()),
        (15, text + 2, 10)
    );
    assert_eq!(
        (
            source.charges.get(),
            source.profiles.get(),
            source.queries.get()
        ),
        (1, 0, 0)
    );
    assert!(!format!("{bound:?}").contains("private"));
    Ok(())
}
#[test]
fn profile_parameter_refusals_keep_original_header_request_metadata_and_lending_member_causes(
) -> TestResult {
    for (fault, expected) in [
        (Fault::Root, DraftingProfileLogicalIssue::Root),
        (Fault::Pin, DraftingProfileLogicalIssue::SelectedPin),
        (Fault::Inventory, DraftingProfileLogicalIssue::Inventory),
        (Fault::Profile, DraftingProfileLogicalIssue::Profile),
        (Fault::Snapshot, DraftingProfileLogicalIssue::Snapshot),
        (Fault::Parameter, DraftingProfileLogicalIssue::Parameter),
        (Fault::Detached, DraftingProfileLogicalIssue::Declaration),
    ] {
        let root = root()?;
        let other = super::root()?;
        let source = Provider::new(&root, &other, fault)?;
        let context = scope(&root)?
            .bind_drafting_context(&source, limits(10000, 10000, 1000)?)
            .map_err(|e| e.to_string())?;
        let selected = pin()?;
        let policy = limits(10000, 10000, 1000)?;
        let parent = context
            .bind_profile_source(&selected, policy)
            .map_err(|e| e.to_string())?;
        let parameter = ProfileParameterRef::new(id(600));
        source.queries.set(0);
        source.profiles.set(0);
        let error = parent
            .bind_logical_parameter(parameter, LogicalInputKind::PocketOpening, policy)
            .err()
            .ok_or("original mismatch")?;
        assert!(
            matches!(error.refusal(),DraftingProfileLogicalRefusal::Mismatch(actual) if *actual==expected)
        );
        assert!(std::ptr::eq(error.parent(), &parent));
        assert_eq!(error.parameter(), parameter);
        assert_eq!(error.kind(), LogicalInputKind::PocketOpening);
        assert_eq!(error.limits(), policy);
        let returned = error.returned().ok_or("original raw source")?;
        assert!(source
            .rows
            .iter()
            .any(|r| r.is_same_record(returned.metadata)));
        assert_eq!((source.profiles.get(), source.queries.get()), (0, 1));
    }
    for fault in [Fault::None, Fault::Missing] {
        let root = root()?;
        let other = super::root()?;
        let source = Provider::new(&root, &other, fault)?;
        let context = scope(&root)?
            .bind_drafting_context(&source, limits(10000, 10000, 1000)?)
            .map_err(|e| e.to_string())?;
        let selected = pin()?;
        let parent = context
            .bind_profile_source(&selected, limits(10000, 10000, 1000)?)
            .map_err(|e| e.to_string())?;
        let kind = if fault == Fault::None {
            LogicalInputKind::NotionSize
        } else {
            LogicalInputKind::PocketOpening
        };
        let error = parent
            .bind_logical_parameter(
                ProfileParameterRef::new(id(600)),
                kind,
                limits(10000, 10000, 1000)?,
            )
            .err()
            .ok_or("actual canonical cause")?;
        match error.refusal() {
            DraftingProfileLogicalRefusal::Canonical(LogicalInputLookupError::Kind {
                expected,
                actual,
            }) if fault == Fault::None => {
                assert_eq!(*expected, kind);
                assert!(std::ptr::eq(
                    *actual,
                    parent
                        .source()
                        .inputs
                        .logicals()
                        .declaration(id(90))
                        .ok_or("actual member cause")?
                ));
            }
            DraftingProfileLogicalRefusal::Canonical(LogicalInputLookupError::Missing {
                declaration,
                expected,
            }) if fault == Fault::Missing => {
                assert_eq!((*declaration, *expected), (id(999), kind));
            }
            other => return Err(format!("original canonical refusal:{other:?}").into()),
        }
        assert!(error.returned().is_some());
    }
    Ok(())
}
#[test]
fn profile_parameter_complete_content_and_original_missing_provider_cause_never_expose_a_prefix(
) -> TestResult {
    let root = root()?;
    let other = super::root()?;
    for ignore in [false, true] {
        let mut source = Provider::new(&root, &other, Fault::None)?;
        source.ignore = ignore;
        let context = scope(&root)?
            .bind_drafting_context(&source, limits(10000, 10000, 1000)?)
            .map_err(|e| e.to_string())?;
        let selected = pin()?;
        let parent = context
            .bind_profile_source(&selected, limits(10000, 10000, 1000)?)
            .map_err(|e| e.to_string())?;
        source.queries.set(0);
        source.profiles.set(0);
        let error = parent
            .bind_logical_parameter(
                ProfileParameterRef::new(id(404)),
                LogicalInputKind::PocketOpening,
                limits(8, 10000, 1000)?,
            )
            .err()
            .ok_or("complete metadata vector before missing query")?;
        assert!(matches!(
            error.refusal(),
            DraftingProfileLogicalRefusal::Resources(DesignResourceLimitError::Count {
                unit: DesignResourceCount::CollectionEntries,
                maximum: 8,
                consumed: 7,
                requested: 2,
                location: DesignResourceLocation {
                    field: "profile.parameters",
                    ..
                }
            })
        ));
        assert!(error.returned().is_none());
        assert_eq!((source.profiles.get(), source.queries.get()), (0, 0));
    }
    let source = Provider::new(&root, &other, Fault::None)?;
    let context = scope(&root)?
        .bind_drafting_context(&source, limits(10000, 10000, 1000)?)
        .map_err(|e| e.to_string())?;
    let selected = pin()?;
    let parent = context
        .bind_profile_source(&selected, limits(10000, 10000, 1000)?)
        .map_err(|e| e.to_string())?;
    let error = parent
        .bind_logical_parameter(
            ProfileParameterRef::new(id(404)),
            LogicalInputKind::PocketOpening,
            limits(10000, 10000, 1000)?,
        )
        .err()
        .ok_or("original missing parameter")?;
    assert!(
        matches!(error.refusal(),DraftingProfileLogicalRefusal::Source(Cause::MissingParameter {parameter}) if *parameter==id(404))
    );
    assert!(std::error::Error::source(&error).is_some_and(|e| e.is::<Cause>()));
    assert!(error.returned().is_none());
    Ok(())
}
