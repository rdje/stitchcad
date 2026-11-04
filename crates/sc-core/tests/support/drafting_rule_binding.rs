//! Fresh original rule source association with actual mismatched witnesses and no signature proof.
use super::{
    limits,
    profile_binding::Body,
    root_with_host_and_semantic,
    rule_sources::{Cause as BaseCause, Opaque, Provider as Base},
    scope,
    semantic_root::canonical,
    EmptyHost, Frame, Snapshot, TestResult,
};
use sc_core::{
    command::DesignRevision, construction::*, design::*, name::MachineToken, ontology::*,
    recipe::*, value::*,
};
use std::{
    cell::{Cell, RefCell},
    error::Error,
    fmt,
};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
pub(super) fn root() -> TestResult<DesignInputs<EmptyHost>> {
    let button = ButtonholeOperation::new(
        ButtonholeOperationDefinition {
            id: id(70),
            button: NotionSize::Declaration(id(1100)),
            policy: id(901),
            inputs: vec![ButtonholePolicyInput {
                role: MachineToken::new("policy_a")?,
                target: CanonicalInputTarget::Length(id(10)),
            }],
        },
        ButtonholeOperationLimits::new(1, 8, 1),
    )?;
    let logical = LogicalDerivationOperation::new(
        LogicalDerivationDefinition {
            id: id(80),
            output: LogicalInputTarget {
                declaration: id(90),
                kind: LogicalInputKind::PocketOpening,
            },
            rule: id(900),
            inputs: vec![LogicalDerivationInput {
                role: MachineToken::new("source_a")?,
                source: LogicalDerivationInputSource::Numeric(CanonicalInputTarget::Scalar {
                    declaration: id(20),
                    kind: ScalarInputKind::Boolean,
                }),
            }],
        },
        LogicalDerivationLimits::new(1, 8, 1),
    )?;
    root_with_host_and_semantic(
        "",
        vec![],
        EmptyHost,
        canonical()?,
        vec![],
        DesignSourceReferences {
            measurement_table: None,
            ease_set: None,
            size_set: None,
        },
        SemanticRecipe::new(
            vec![
                SemanticOperation::Buttonhole(button),
                SemanticOperation::LogicalDerivation(logical),
            ],
            SemanticRecipeLimits::new(2, 4),
        )?,
    )
}
fn pin() -> TestResult<SelectedProfilePin> {
    Ok(SelectedProfilePin::new(SelectedProfilePinDefinition {
        profile_id: "atelier é".into(),
        schema_version: " 2.0 ".into(),
        content_version: "7".into(),
    })?)
}
#[derive(Clone, Copy)]
enum Fault {
    None,
    Root,
    Snapshot,
    Origin,
    Pin,
    Inventory,
    Profile,
    Rule,
    Kind,
    Unavailable,
}
#[derive(Debug)]
enum Cause {
    Base(BaseCause),
    Unavailable { rule: EntityId },
}
impl fmt::Display for Cause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("original rule binding fixture cause")
    }
}
impl Error for Cause {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Base(cause) => Some(cause),
            Self::Unavailable { .. } => None,
        }
    }
}
struct Provider<'a> {
    inner: Base<'a>,
    other: &'a DesignInputs<EmptyHost>,
    pin: SelectedProfilePin,
    inputs: CanonicalInputs,
    profile: DraftingSourceRecord<Body>,
    snapshot: DraftingSourceRecord<Snapshot>,
    wrong_id: SemanticRuleDeclaration,
    wrong_kind: SemanticRuleDeclaration,
    cache: RefCell<Vec<u8>>,
    fault: Fault,
    ignore: bool,
    queries: Cell<usize>,
}
impl<'a> Provider<'a> {
    fn new(
        root: &'a DesignInputs<EmptyHost>,
        other: &'a DesignInputs<EmptyHost>,
        fault: Fault,
        ignore: bool,
    ) -> TestResult<Self> {
        let declaration = |n| {
            SemanticRuleDeclaration::new(
                SemanticRuleDefinition {
                    id: id(n),
                    signature: SemanticRuleSignature::Buttonhole { inputs: vec![] },
                },
                SemanticRuleLimits::new(0, 0, 0),
            )
        };
        Ok(Self {
            inner: Base::new(root, other)?,
            other,
            pin: pin()?,
            inputs: canonical()?,
            profile: DraftingSourceRecord::new(Body {
                text: "private original Profile body".into(),
            }),
            snapshot: DraftingSourceRecord::new(Snapshot {
                label: "private empty snapshot".into(),
            }),
            wrong_id: declaration(999)?,
            wrong_kind: declaration(900)?,
            cache: RefCell::new(vec![]),
            fault,
            ignore,
            queries: Cell::new(0),
        })
    }
    fn walk(&self, sink: &mut dyn DesignResourceSink) -> Result<(), DesignResourceLimitError> {
        let Self {
            inner,
            other,
            pin,
            inputs,
            profile,
            snapshot,
            wrong_id,
            wrong_kind,
            cache,
            fault,
            ignore,
            queries,
        } = self;
        fixed(other);
        fixed(fault);
        fixed(ignore);
        let _: &Cell<usize> = queries;
        inner.charge_resources(sink)?; // Includes the original other Root;the loan above is no copy.
        charge_selected_profile_pin_resources(
            pin,
            sink,
            DesignResourceScope::SourceHost,
            None,
            None,
        )?;
        charge_canonical_input_resources(inputs, sink)?;
        let Body { text } = profile.data();
        sink.charge_text(text, at("rule.other_profile"))?;
        let Snapshot { label } = snapshot.data();
        sink.charge_text(label, at("rule.other_snapshot"))?;
        charge_semantic_rule_resources(wrong_id, sink, DesignResourceScope::SourceHost)?;
        charge_semantic_rule_resources(wrong_kind, sink, DesignResourceScope::SourceHost)?;
        let cache = cache.borrow();
        sink.charge_entries(cache.len(), at("rule.source.cache"))?;
        let _: &[u8] = &cache;
        Ok(())
    }
}
fn fixed<T: Copy>(_: &T) {}
fn at(field: &'static str) -> DesignResourceLocation {
    DesignResourceLocation {
        scope: DesignResourceScope::SourceHost,
        owner: None,
        field,
        index: None,
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
    fn charge_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        let result = self.walk(sink);
        if self.ignore {
            Ok(())
        } else {
            result
        }
    }
    fn validate_current(&self) -> Result<(), Cause> {
        self.inner.validate_current().map_err(Cause::Base)
    }
    fn geometry(
        &self,
        r: DraftingGeometryReference,
    ) -> DraftingGeometrySourceResult<'_, Frame, Snapshot, [u8], Cause> {
        self.inner.geometry(r).map_err(Cause::Base)
    }
}
impl DraftingProfileProvider<EmptyHost> for Provider<'_> {
    type Profile = Body;
    fn profile_source<'s>(
        &'s self,
        selected: &SelectedProfilePin,
    ) -> Result<DraftingProfileSource<'s, EmptyHost, Snapshot, Body>, Cause> {
        self.inner.profile_source(selected).map_err(Cause::Base)
    }
}
impl DraftingRuleProvider<EmptyHost> for Provider<'_> {
    type RuleBody = Opaque;
    fn rule_source<'s>(
        &'s self,
        request: DraftingRuleRequest<'_>,
    ) -> Result<DraftingRuleResponse<'s, EmptyHost, Self>, Cause> {
        self.queries.set(self.queries.get() + 1);
        if matches!(self.fault, Fault::Unavailable) {
            return Err(Cause::Unavailable { rule: request.rule });
        }
        let mut source = self.inner.rule_source(request).map_err(Cause::Base)?;
        match self.fault {
            Fault::Root => {
                source.origin = DraftingRuleOrigin::Design {
                    root: self.other,
                    snapshot: &self.snapshot,
                };
                source.declaration = &self.wrong_id;
            }
            Fault::Snapshot => {
                source.origin = DraftingRuleOrigin::Design {
                    root: self.root(),
                    snapshot: &self.snapshot,
                };
                source.declaration = &self.wrong_id;
            }
            Fault::Origin => {
                source.origin = DraftingRuleOrigin::Design {
                    root: self.root(),
                    snapshot: self.snapshot(),
                };
                source.declaration = &self.wrong_id;
            }
            Fault::Pin | Fault::Inventory | Fault::Profile => {
                let DraftingRuleOrigin::Profile(ref mut profile) = source.origin else {
                    return Err(Cause::Unavailable { rule: request.rule });
                };
                if matches!(self.fault, Fault::Pin) {
                    profile.selected = &self.pin;
                }
                if matches!(self.fault, Fault::Pin | Fault::Inventory) {
                    profile.inputs = &self.inputs;
                }
                profile.profile = &self.profile;
                source.declaration = &self.wrong_id;
            }
            Fault::Rule => source.declaration = &self.wrong_id,
            Fault::Kind => source.declaration = &self.wrong_kind,
            Fault::None | Fault::Unavailable => {}
        }
        Ok(source)
    }
}
#[test]
fn fresh_design_and_profile_rule_associations_preserve_original_work_body_and_owners_only(
) -> TestResult {
    let root = root()?;
    let other = super::root()?;
    for (operation, kind, profile, rule) in [
        (80, SemanticOperationKind::LogicalDerivation, false, 900),
        (70, SemanticOperationKind::Buttonhole, true, 901),
    ] {
        let source = Base::new(&root, &other)?;
        let context = scope(&root)?
            .bind_drafting_context(&source, limits(1000, 10000, 1000)?)
            .map_err(|e| e.to_string())?;
        let selected = pin()?;
        let selection = if profile {
            DraftingRuleSelection::Profile(&selected)
        } else {
            DraftingRuleSelection::Design
        };
        let text = source.text(&selected) + 24 + if profile { 16 } else { 0 };
        let work = context
            .prepare_rule_binding(
                id(operation),
                kind,
                selection,
                DraftingRuleBindingLimits::new(limits(21, text, 138)?, 841),
            )
            .map_err(|e| e.to_string())?;
        let before = source.charge_count();
        let bound = work.bind_source().map_err(|e| e.to_string())?;
        assert!(core::ptr::eq(bound.work(), &work));
        assert_eq!(
            (
                bound.source().declaration.id(),
                bound.source().declaration.kind()
            ),
            (id(rule), kind)
        );
        assert_eq!(source.charge_count(), before + if profile { 2 } else { 1 });
        assert_eq!(source.query_count(), 1);
        assert_eq!(bound.shape(), work.shape());
        assert_eq!(
            (
                bound.demand().collection_entries(),
                bound.demand().text_bytes(),
                bound.demand().counter_bits()
            ),
            (21, text, 138)
        );
        assert!(core::ptr::eq(bound.source().origin.root(), &root));
        assert!(bound
            .source()
            .origin
            .snapshot()
            .is_same_record(context.snapshot()));
        match (bound.profile_source(), bound.source().origin) {
            (Some(expected), DraftingRuleOrigin::Profile(actual)) => {
                assert!(profile);
                assert!(core::ptr::eq(expected.selected, actual.selected));
                assert!(!core::ptr::eq(actual.selected, &selected));
                assert!(core::ptr::eq(expected.inputs, actual.inputs));
                assert!(expected.profile.is_same_record(actual.profile));
            }
            (None, DraftingRuleOrigin::Design { .. }) => assert!(!profile),
            _ => return Err("original source origin".into()),
        }
        // These authored local roles intentionally differ from the source signature. Source binding
        // alone must not certify complete signature/argument/output or executable acceptance.
        let mut meter = DesignResourceMeter::new(limits(23, text + 3, 139)?);
        meter.charge_entries(2, at("prior"))?;
        meter.charge_text("old", at("prior"))?;
        meter.charge_counter_bits(1, at("prior"))?;
        bound.charge_resources(&mut meter)?;
        let demand = meter.finish()?;
        assert_eq!(
            (
                demand.collection_entries(),
                demand.text_bytes(),
                demand.counter_bits()
            ),
            (23, text + 3, 139)
        );
        assert_eq!(source.query_count(), 1);
        assert!(!format!("{bound:?}").contains("atelier"));
    }
    Ok(())
}
#[test]
fn original_foreign_header_and_rule_mismatches_keep_priority_and_complete_witnesses() -> TestResult
{
    let root = root()?;
    let other = self::root()?;
    for (fault, issue, profile) in [
        (Fault::Root, DraftingRuleSourceIssue::Root, false),
        (Fault::Snapshot, DraftingRuleSourceIssue::Snapshot, false),
        (Fault::Origin, DraftingRuleSourceIssue::Origin, true),
        (Fault::Pin, DraftingRuleSourceIssue::SelectedPin, true),
        (Fault::Inventory, DraftingRuleSourceIssue::Inventory, true),
        (Fault::Profile, DraftingRuleSourceIssue::Profile, true),
        (Fault::Rule, DraftingRuleSourceIssue::Rule, false),
        (Fault::Kind, DraftingRuleSourceIssue::Kind, false),
    ] {
        let source = Provider::new(&root, &other, fault, false)?;
        let context = scope(&root)?
            .bind_drafting_context(&source, limits(1000, 10000, 1000)?)
            .map_err(|e| e.to_string())?;
        let selected = pin()?;
        let (operation, kind, selection) = if profile {
            (
                70,
                SemanticOperationKind::Buttonhole,
                DraftingRuleSelection::Profile(&selected),
            )
        } else {
            (
                80,
                SemanticOperationKind::LogicalDerivation,
                DraftingRuleSelection::Design,
            )
        };
        let work = context
            .prepare_rule_binding(
                id(operation),
                kind,
                selection,
                DraftingRuleBindingLimits::new(limits(1000, 10000, 1000)?, usize::MAX),
            )
            .map_err(|e| e.to_string())?;
        let error = work.bind_source().err().ok_or("original source mismatch")?;
        assert!(
            matches!(error.refusal(),DraftingRuleSourceRefusal::Mismatch(actual) if actual==&issue)
        );
        assert!(core::ptr::eq(error.work(), &work));
        assert_eq!(source.queries.get(), 1);
        assert_eq!(
            error.demand().ok_or("fresh content")?.collection_entries(),
            33
        );
        let returned = error.returned().ok_or("original mismatched source")?;
        assert_eq!(
            returned.declaration.id(),
            if matches!(fault, Fault::Kind) {
                id(900)
            } else {
                id(999)
            }
        );
        if profile {
            assert!(error.profile_source().is_some());
        } else {
            assert!(error.profile_source().is_none());
        }
        if matches!(fault, Fault::Root) {
            assert_eq!(root.id(), other.id());
            assert!(core::ptr::eq(returned.origin.root(), &other));
            assert!(!core::ptr::eq(returned.origin.root(), &root));
        }
        if matches!(fault, Fault::Snapshot) {
            assert!(returned.origin.snapshot().is_same_record(&source.snapshot));
            assert_eq!(
                returned.origin.snapshot().data().label,
                context.snapshot().data().label
            );
        }
        if let (Some(expected), DraftingRuleOrigin::Profile(actual)) =
            (error.profile_source(), returned.origin)
        {
            if matches!(fault, Fault::Pin) {
                assert_eq!(expected.selected, actual.selected);
                assert!(!core::ptr::eq(expected.selected, actual.selected));
            }
            if matches!(fault, Fault::Inventory) {
                assert_eq!(expected.inputs, actual.inputs);
                assert!(!core::ptr::eq(expected.inputs, actual.inputs));
            }
            if matches!(fault, Fault::Profile) {
                assert_eq!(expected.profile.data().text, actual.profile.data().text);
                assert!(!expected.profile.is_same_record(actual.profile));
            }
        }
    }
    Ok(())
}
#[test]
fn fresh_cache_work_profile_and_provider_failures_never_expose_an_accepted_source_prefix(
) -> TestResult {
    let root = root()?;
    let other = self::root()?;
    for (ignore, entries, expected_work) in
        [(false, 33, false), (true, 33, false), (false, 34, true)]
    {
        let source = Provider::new(&root, &other, Fault::None, ignore)?;
        let context = scope(&root)?
            .bind_drafting_context(&source, limits(1000, 10000, 1000)?)
            .map_err(|e| e.to_string())?;
        let work = context
            .prepare_rule_binding(
                id(80),
                SemanticOperationKind::LogicalDerivation,
                DraftingRuleSelection::Design,
                DraftingRuleBindingLimits::new(limits(entries, 10000, 1000)?, 1681),
            )
            .map_err(|e| e.to_string())?;
        assert_eq!(work.shape().held_collection_entries(), 33);
        // Deliberate negative control:grow actual separately held cache after work,while semantic
        // records stay unchanged. This falsifies reuse of the earlier demand as fresh credit.
        source.cache.borrow_mut().push(7);
        let error = work
            .bind_source()
            .err()
            .ok_or("fresh complete admission refusal")?;
        let DraftingRuleSourceRefusal::Work(cause) = error.refusal() else {
            return Err("original work cause".into());
        };
        if expected_work {
            assert!(matches!(
                cause.refusal(),
                DraftingRuleWorkRefusal::MatchingInspections {
                    allowed: 1681,
                    required: 1764
                }
            ));
        } else {
            let expected = DesignResourceLimitError::Count {
                unit: DesignResourceCount::CollectionEntries,
                maximum: 33,
                consumed: 33,
                requested: 1,
                location: at("rule.source.cache"),
            };
            assert!(
                matches!(cause.refusal(),DraftingRuleWorkRefusal::Resources(actual) if actual==&expected)
            );
            assert!(error
                .source()
                .and_then(|e| e.downcast_ref::<DesignResourceLimitError>())
                .is_some());
        }
        assert!(error.demand().is_none());
        assert!(error.returned().is_none());
        assert_eq!(source.queries.get(), 0);
    }
    for profile in [false, true] {
        let source = Provider::new(&root, &other, Fault::Unavailable, false)?;
        let context = scope(&root)?
            .bind_drafting_context(&source, limits(1000, 10000, 1000)?)
            .map_err(|e| e.to_string())?;
        let selected = SelectedProfilePin::new(SelectedProfilePinDefinition {
            profile_id: "missing factory".into(),
            schema_version: "2".into(),
            content_version: "7".into(),
        })?;
        let selection = if profile {
            DraftingRuleSelection::Profile(&selected)
        } else {
            DraftingRuleSelection::Design
        };
        let work = context
            .prepare_rule_binding(
                id(80),
                SemanticOperationKind::LogicalDerivation,
                selection,
                DraftingRuleBindingLimits::new(limits(1000, 10000, 1000)?, usize::MAX),
            )
            .map_err(|e| e.to_string())?;
        let error = work
            .bind_source()
            .err()
            .ok_or("original typed source refusal")?;
        if profile {
            let DraftingRuleSourceRefusal::Profile(cause) = error.refusal() else {
                return Err("original Profile cause".into());
            };
            assert!(core::ptr::eq(cause.selected(), &selected));
            assert!(matches!(
                error.source().and_then(|e| e.downcast_ref::<Cause>()),
                Some(Cause::Base(BaseCause::Base(
                    super::profile_binding::Cause::Unavailable
                )))
            ));
            assert_eq!(source.queries.get(), 0);
        } else {
            assert!(
                matches!(error.refusal(),DraftingRuleSourceRefusal::Source(Cause::Unavailable {rule}) if *rule==id(900))
            );
            assert!(
                matches!(error.source().and_then(|e|e.downcast_ref::<Cause>()),Some(Cause::Unavailable {rule}) if *rule==id(900))
            );
            assert_eq!(source.queries.get(), 1);
        }
        assert!(error.demand().is_some());
        assert!(error.returned().is_none());
        assert!(core::ptr::eq(error.work(), &work));
    }
    Ok(())
}
