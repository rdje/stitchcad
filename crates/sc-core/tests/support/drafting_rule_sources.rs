//! Original rule body/origin loans and honest complete provider accounting; binding stays separate.
use super::{
    limits,
    profile_binding::{Body, Cause as BaseCause, Fault, Provider as Base},
    root, scope, EmptyHost, Frame, Snapshot, TestResult,
};
use sc_core::{
    command::DesignRevision, construction::*, design::*, name::MachineToken, ontology::*,
    recipe::*, value::*,
};
use std::{cell::Cell, error::Error, fmt};
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
pub(super) struct Opaque {
    text: String,
}
#[derive(Clone, Copy)]
enum RowOrigin {
    Design,
    Profile,
}
struct Row {
    declaration: SemanticRuleDeclaration,
    body: DraftingSourceRecord<Opaque>,
    origin: RowOrigin,
}
#[derive(Debug)]
pub(super) enum Cause {
    Base(BaseCause),
    Missing {
        rule: EntityId,
    },
    Unsupported {
        rule: EntityId,
        kind: SemanticOperationKind,
    },
    Origin {
        rule: EntityId,
    },
}
impl fmt::Display for Cause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("original rule source cause")
    }
}
impl Error for Cause {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Base(e) => Some(e),
            Self::Missing { .. } | Self::Unsupported { .. } | Self::Origin { .. } => None,
        }
    }
}
pub(super) struct Provider<'a> {
    inner: Base<'a>,
    rows: Vec<Row>,
    cache: String,
    revision: DesignRevision,
    charges: Cell<usize>,
    queries: Cell<usize>,
    ignore: bool,
}
impl<'a> Provider<'a> {
    pub(super) fn new(
        root: &'a DesignInputs<EmptyHost>,
        other: &'a DesignInputs<EmptyHost>,
    ) -> TestResult<Self> {
        let mut rows = vec![];
        for (n, signature, text, origin) in [
            (
                900,
                SemanticRuleSignature::Logical {
                    result: LogicalInputKind::PocketOpening,
                    inputs: vec![SemanticRuleInput {
                        role: MachineToken::new("option_a")?,
                        kind: SemanticRuleInputKind::Logical(LogicalInputKind::NotionSize),
                    }],
                },
                "original design rule é",
                RowOrigin::Design,
            ),
            (
                901,
                SemanticRuleSignature::Buttonhole {
                    inputs: vec![ButtonholePolicyRequirement {
                        role: MachineToken::new("ratio_0")?,
                        kind: CanonicalNumericInputKind::Scalar(ScalarInputKind::Ratio),
                    }],
                },
                "original policy body",
                RowOrigin::Profile,
            ),
            (
                902,
                SemanticRuleSignature::Logical {
                    result: LogicalInputKind::NotionSize,
                    inputs: vec![SemanticRuleInput {
                        role: MachineToken::new("unused_a")?,
                        kind: SemanticRuleInputKind::Numeric(CanonicalNumericInputKind::Length),
                    }],
                },
                "unused rule body",
                RowOrigin::Design,
            ),
        ] {
            rows.push(Row {
                declaration: SemanticRuleDeclaration::new(
                    SemanticRuleDefinition {
                        id: id(n),
                        signature,
                    },
                    SemanticRuleLimits::new(1, 8, 1),
                )?,
                body: DraftingSourceRecord::new(Opaque { text: text.into() }),
                origin,
            });
        }
        Ok(Self {
            inner: Base::new(root, other, Fault::None)?,
            rows,
            cache: "unused cached rule".into(),
            revision: DesignRevision::from_counter(u128::MAX).successor(),
            charges: Cell::new(0),
            queries: Cell::new(0),
            ignore: false,
        })
    }
    pub(super) fn text(&self, request: &SelectedProfilePin) -> usize {
        self.inner.text(request) - 16 + 100
    }
    pub(super) fn query_count(&self) -> usize {
        self.queries.get()
    }
    pub(super) fn charge_count(&self) -> usize {
        self.charges.get()
    }
    pub(super) fn ignore_refusals(&mut self) {
        self.ignore = true;
    }
    fn walk(&self, sink: &mut dyn DesignResourceSink) -> Result<(), DesignResourceLimitError> {
        let Self {
            inner,
            rows,
            cache,
            revision,
            charges,
            queries,
            ignore,
        } = self;
        let _: (&Cell<usize>, &Cell<usize>, &bool) = (charges, queries, ignore);
        inner.charge_resources(sink)?;
        sink.charge_entries(rows.len(), at("rules", None, None))?;
        for (index, row) in rows.iter().enumerate() {
            let Row {
                declaration,
                body,
                origin,
            } = row;
            fixed(origin);
            charge_semantic_rule_resources(declaration, sink, DesignResourceScope::SourceHost)?;
            let Opaque { text } = body.data();
            sink.charge_text(text, at("rule.body", Some(declaration.id()), Some(index)))?;
        }
        sink.charge_text(cache, at("rule.cache", None, None))?;
        charge_design_revision_resources(revision, sink, at("rule.revision", None, None))
    }
}
fn at(
    field: &'static str,
    owner: Option<EntityId>,
    index: Option<usize>,
) -> DesignResourceLocation {
    DesignResourceLocation {
        scope: DesignResourceScope::SourceHost,
        owner,
        field,
        index,
    }
}
fn fixed<T: Copy>(_: &T) {}
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
        self.charges.set(self.charges.get() + 1);
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
        let row = self
            .rows
            .iter()
            .find(|row| row.declaration.id() == request.rule)
            .ok_or(Cause::Missing { rule: request.rule })?;
        if row.declaration.kind() != request.kind {
            return Err(Cause::Unsupported {
                rule: request.rule,
                kind: request.kind,
            });
        }
        let origin = match (row.origin, request.origin) {
            (RowOrigin::Design, DraftingRuleSelection::Design) => DraftingRuleOrigin::Design {
                root: self.root(),
                snapshot: self.snapshot(),
            },
            (RowOrigin::Profile, DraftingRuleSelection::Profile(selected)) => {
                DraftingRuleOrigin::Profile(self.profile_source(selected)?)
            }
            _ => return Err(Cause::Origin { rule: request.rule }),
        };
        Ok(DraftingRuleSource {
            declaration: &row.declaration,
            body: &row.body,
            origin,
        })
    }
}
#[test]
fn raw_design_and_profile_origins_keep_original_signature_body_pin_and_snapshot_loans() -> TestResult
{
    let root = root()?;
    let other = super::root()?;
    let provider = Provider::new(&root, &other)?;
    for (n, kind) in [
        (900, SemanticOperationKind::LogicalDerivation),
        (901, SemanticOperationKind::Buttonhole),
    ] {
        // Response loans outlive this shorter-lived request pin;the provider owns their records.
        let source = {
            let selected = pin()?;
            provider.rule_source(DraftingRuleRequest {
                rule: id(n),
                kind,
                origin: if n == 900 {
                    DraftingRuleSelection::Design
                } else {
                    DraftingRuleSelection::Profile(&selected)
                },
            })?
        };
        let row = provider
            .rows
            .iter()
            .find(|row| row.declaration.id() == id(n))
            .ok_or("original row")?;
        let copied = source;
        assert!(core::ptr::eq(copied.declaration, &row.declaration));
        assert!(copied.body.is_same_record(&row.body));
        assert!(core::ptr::eq(source.origin.root(), &root));
        assert!(source.origin.snapshot().is_same_record(provider.snapshot()));
        let selected = pin()?;
        let profile = provider.profile_source(&selected)?;
        match source.origin {
            DraftingRuleOrigin::Profile(actual) => {
                assert_eq!(n, 901);
                assert!(core::ptr::eq(actual.selected, profile.selected));
                assert!(!core::ptr::eq(actual.selected, &selected));
                assert!(core::ptr::eq(actual.inputs, profile.inputs));
                assert!(actual.profile.is_same_record(profile.profile));
            }
            DraftingRuleOrigin::Design { .. } => assert_eq!(n, 900),
        }
        assert!(!format!("{source:?}").contains("original"));
        assert!(!format!("{:?}", DraftingRuleSelection::Profile(&selected)).contains("atelier"));
        let body = DraftingSourceRecord::new([1_u8, 2]);
        let body_view: &DraftingSourceRecord<[u8]> = &body;
        let raw = DraftingRuleSource {
            declaration: source.declaration,
            body: body_view,
            origin: source.origin,
        };
        assert_eq!(raw.body.data(), &[1, 2]);
        // Raw construction can expose equal-data foreign metadata;it supplies no accepted binding.
        let foreign = row.declaration.clone();
        let raw = DraftingRuleSource {
            declaration: &foreign,
            body: source.body,
            origin: source.origin,
        };
        assert_eq!(raw.declaration, source.declaration);
        assert!(!core::ptr::eq(raw.declaration, source.declaration));
    }
    assert_eq!(provider.queries.get(), 2);
    Ok(())
}
#[test]
fn whole_context_walk_includes_unused_rules_bodies_cache_and_counter_before_queries() -> TestResult
{
    let root = root()?;
    let other = super::root()?;
    let selected = pin()?;
    let provider = Provider::new(&root, &other)?;
    let text = provider.text(&selected);
    let context = scope(&root)?
        .bind_drafting_context(&provider, limits(1000, 10000, 1000)?)
        .map_err(|e| e.to_string())?;
    provider.charges.set(0);
    let admitted = context
        .prepare_design_semantic_sources(limits(13, text, 138)?)
        .map_err(|e| e.to_string())?;
    assert_eq!(
        (
            admitted.demand().collection_entries(),
            admitted.demand().text_bytes(),
            admitted.demand().counter_bits()
        ),
        (13, text, 138)
    );
    assert_eq!((provider.charges.get(), provider.queries.get()), (1, 0));
    let mut meter = DesignResourceMeter::new(limits(15, text + 3, 139)?);
    meter.charge_entries(2, at("prior", None, None))?;
    meter.charge_text("old", at("prior", None, None))?;
    meter.charge_counter_bits(1, at("prior", None, None))?;
    admitted.charge_resources(&mut meter)?;
    let demand = meter.finish()?;
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (15, text + 3, 139)
    );
    for ignore in [false, true] {
        // Independent phases keep the inherited bounded decimal trace local to one scenario.
        let mut provider = Provider::new(&root, &other)?;
        provider.ignore = ignore;
        let context = scope(&root)?
            .bind_drafting_context(&provider, limits(1000, 10000, 1000)?)
            .map_err(|e| e.to_string())?;
        let error = context
            .prepare_design_semantic_sources(limits(9, text, 138)?)
            .err()
            .ok_or("complete vector refusal")?;
        assert_eq!(
            error.cause(),
            &DesignResourceLimitError::Count {
                unit: DesignResourceCount::CollectionEntries,
                maximum: 9,
                consumed: 7,
                requested: 3,
                location: at("rules", None, None)
            }
        );
        let error = context
            .prepare_design_semantic_sources(limits(13, text - 1, 138)?)
            .err()
            .ok_or("unused cache refusal")?;
        assert_eq!(
            error.cause(),
            &DesignResourceLimitError::Count {
                unit: DesignResourceCount::TextBytes,
                maximum: text - 1,
                consumed: text - 18,
                requested: 18,
                location: at("rule.cache", None, None)
            }
        );
        assert_eq!(provider.queries.get(), 0);
    }
    Ok(())
}
#[test]
fn missing_unsupported_and_profile_causes_remain_original_typed_requests() -> TestResult {
    let root = root()?;
    let other = super::root()?;
    let provider = Provider::new(&root, &other)?;
    let kind = SemanticOperationKind::Buttonhole;
    let missing = provider
        .rule_source(DraftingRuleRequest {
            rule: id(999),
            kind,
            origin: DraftingRuleSelection::Design,
        })
        .err()
        .ok_or("missing rule")?;
    assert!(matches!(missing,Cause::Missing {rule} if rule==id(999)));
    let unsupported = provider
        .rule_source(DraftingRuleRequest {
            rule: id(900),
            kind,
            origin: DraftingRuleSelection::Design,
        })
        .err()
        .ok_or("unsupported family")?;
    assert!(
        matches!(unsupported,Cause::Unsupported {rule,kind:actual} if rule==id(900) && actual==kind)
    );
    let selected = SelectedProfilePin::new(SelectedProfilePinDefinition {
        profile_id: "missing factory".into(),
        schema_version: "2".into(),
        content_version: "7".into(),
    })?;
    let error = provider
        .rule_source(DraftingRuleRequest {
            rule: id(901),
            kind,
            origin: DraftingRuleSelection::Profile(&selected),
        })
        .err()
        .ok_or("original Profile cause")?;
    assert!(matches!(
        error.source().and_then(|e| e.downcast_ref::<BaseCause>()),
        Some(BaseCause::Unavailable)
    ));
    let error = provider
        .rule_source(DraftingRuleRequest {
            rule: id(901),
            kind,
            origin: DraftingRuleSelection::Design,
        })
        .err()
        .ok_or("actual original origin refusal")?;
    assert!(matches!(error,Cause::Origin {rule} if rule==id(901)));
    assert_eq!(provider.queries.get(), 4);
    Ok(())
}
