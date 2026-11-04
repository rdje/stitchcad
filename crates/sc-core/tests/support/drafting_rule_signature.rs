//! Exact original named kinds and actual canonical Derived-output relation,without source values.
use super::{
    limits,
    profile_binding::{Body, Cause as BaseCause, Fault as BaseFault, Provider as Base},
    root_with_host_and_semantic, scope, EmptyHost, Frame, Snapshot, TestResult,
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
#[derive(Clone, Copy)]
enum Case {
    Exact,
    DesignInput,
    Result,
    Count,
    Extra,
    Role,
    Numeric,
    Logical,
    MissingOutput,
    OutputKind,
    OutputUnknown,
    OutputKnown,
    OtherDerived,
    Empty,
}
fn root(case: Case) -> TestResult<DesignInputs<EmptyHost>> {
    let mut logicals = vec![];
    if !matches!(case, Case::MissingOutput) {
        let state = match case {
            Case::OutputUnknown => LogicalInputState::Unknown {
                observation: id(290),
            },
            Case::OutputKnown => LogicalInputState::Known {
                value: MachineToken::new("output_a")?,
                evidence: vec![id(390)],
            },
            Case::OtherDerived => LogicalInputState::Derived { operation: id(81) },
            _ => LogicalInputState::Derived { operation: id(80) },
        };
        logicals.push(LogicalInputDeclaration::new(
            LogicalInputDeclarationDefinition {
                id: id(90),
                source: id(190),
                kind: if matches!(case, Case::OutputKind) {
                    LogicalInputKind::NotionSize
                } else {
                    LogicalInputKind::PocketOpening
                },
                domain: vec![MachineToken::new("output_a")?],
                state,
            },
            LogicalInputLimits::new(4, 4, 128, 100),
        )?);
    }
    logicals.push(LogicalInputDeclaration::new(
        LogicalInputDeclarationDefinition {
            id: id(91),
            source: id(191),
            kind: LogicalInputKind::NotionSize,
            domain: vec![MachineToken::new("size_a")?],
            state: LogicalInputState::Unknown {
                observation: id(291),
            },
        },
        LogicalInputLimits::new(1, 0, 6, 2),
    )?);
    let inputs = CanonicalInputs::with_logicals(
        vec![LengthDeclaration::new(LengthDeclarationDefinition {
            id: id(10),
            source: id(110),
            state: LengthState::Unknown {
                observation: id(210),
            },
        })?],
        vec![
            ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
                id: id(20),
                source: id(120),
                kind: ScalarInputKind::Length,
                state: ScalarInputState::Unknown {
                    observation: id(220),
                },
            })?,
            ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
                id: id(21),
                source: id(121),
                kind: ScalarInputKind::Ratio,
                state: ScalarInputState::Unknown {
                    observation: id(221),
                },
            })?,
        ],
        LogicalInputs::new(logicals, LogicalInputInventoryLimits::new(2, 4))?,
    )?;
    let mut logical_inputs = vec![
        LogicalDerivationInput {
            role: MachineToken::new(if matches!(case, Case::Role) {
                "absent_a"
            } else {
                "source_a"
            })?,
            source: if matches!(case, Case::DesignInput) {
                LogicalDerivationInputSource::Logical(LogicalInputTarget {
                    declaration: id(91),
                    kind: LogicalInputKind::NotionSize,
                })
            } else {
                LogicalDerivationInputSource::Profile {
                    parameter: ProfileParameterRef::new(id(600)),
                    kind: if matches!(case, Case::Logical) {
                        LogicalInputKind::PocketOpening
                    } else {
                        LogicalInputKind::NotionSize
                    },
                }
            },
        },
        LogicalDerivationInput {
            role: MachineToken::new("source_b")?,
            source: LogicalDerivationInputSource::Numeric(if matches!(case, Case::Numeric) {
                CanonicalInputTarget::Scalar {
                    declaration: id(20),
                    kind: ScalarInputKind::Length,
                }
            } else {
                CanonicalInputTarget::Length(id(10))
            }),
        },
    ];
    let mut policy_inputs = vec![
        ButtonholePolicyInput {
            role: MachineToken::new("policy_a")?,
            target: CanonicalInputTarget::Scalar {
                declaration: id(21),
                kind: ScalarInputKind::Ratio,
            },
        },
        ButtonholePolicyInput {
            role: MachineToken::new("policy_b")?,
            target: CanonicalInputTarget::Length(id(10)),
        },
    ];
    if matches!(case, Case::Empty) {
        logical_inputs.clear();
        policy_inputs.clear();
    }
    let logical = LogicalDerivationOperation::new(
        LogicalDerivationDefinition {
            id: id(80),
            output: LogicalInputTarget {
                declaration: id(90),
                kind: LogicalInputKind::PocketOpening,
            },
            rule: id(900),
            inputs: logical_inputs,
        },
        LogicalDerivationLimits::new(2, 16, 4),
    )?;
    let button = ButtonholeOperation::new(
        ButtonholeOperationDefinition {
            id: id(70),
            button: NotionSize::Declaration(id(91)),
            policy: id(901),
            inputs: policy_inputs,
        },
        ButtonholeOperationLimits::new(2, 16, 4),
    )?;
    root_with_host_and_semantic(
        "",
        vec![],
        EmptyHost,
        inputs,
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
enum Origin {
    Design,
    Profile,
}
struct Row {
    declaration: SemanticRuleDeclaration,
    body: DraftingSourceRecord<Body>,
    origin: Origin,
}
#[derive(Debug)]
enum Cause {
    Base(BaseCause),
    Missing(EntityId),
    Origin(EntityId),
}
impl fmt::Display for Cause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Base(_) => f.write_str("original signature fixture cause"),
            Self::Missing(id) => write!(f, "missing rule {id}"),
            Self::Origin(id) => write!(f, "wrong origin {id}"),
        }
    }
}
impl Error for Cause {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Base(cause) => Some(cause),
            _ => None,
        }
    }
}
struct Provider<'a> {
    inner: Base<'a>,
    rows: Vec<Row>,
    cache: RefCell<Vec<u8>>,
    ignore: bool,
    queries: Cell<usize>,
    charges: Cell<usize>,
}
impl<'a> Provider<'a> {
    fn new(
        root: &'a DesignInputs<EmptyHost>,
        other: &'a DesignInputs<EmptyHost>,
        case: Case,
        ignore: bool,
    ) -> TestResult<Self> {
        let mut logical = vec![
            SemanticRuleInput {
                role: MachineToken::new("source_b")?,
                kind: SemanticRuleInputKind::Numeric(CanonicalNumericInputKind::Length),
            },
            SemanticRuleInput {
                role: MachineToken::new("source_a")?,
                kind: SemanticRuleInputKind::Logical(LogicalInputKind::NotionSize),
            },
        ];
        let mut policy = vec![
            ButtonholePolicyRequirement {
                role: MachineToken::new("policy_b")?,
                kind: CanonicalNumericInputKind::Length,
            },
            ButtonholePolicyRequirement {
                role: MachineToken::new("policy_a")?,
                kind: CanonicalNumericInputKind::Scalar(ScalarInputKind::Ratio),
            },
        ];
        if matches!(case, Case::Count | Case::Result) {
            logical.pop();
        }
        if matches!(case, Case::Extra) {
            logical.push(SemanticRuleInput {
                role: MachineToken::new("source_c")?,
                kind: SemanticRuleInputKind::Numeric(CanonicalNumericInputKind::Length),
            });
        }
        if matches!(case, Case::Empty) {
            logical.clear();
            policy.clear();
        }
        let mut rows = vec![];
        for (n, signature, origin, text) in [
            (
                900,
                SemanticRuleSignature::Logical {
                    result: if matches!(case, Case::Result) {
                        LogicalInputKind::NotionSize
                    } else {
                        LogicalInputKind::PocketOpening
                    },
                    inputs: logical,
                },
                Origin::Design,
                "original logical rule body",
            ),
            (
                901,
                SemanticRuleSignature::Buttonhole { inputs: policy },
                Origin::Profile,
                "original Buttonhole policy body",
            ),
        ] {
            rows.push(Row {
                declaration: SemanticRuleDeclaration::new(
                    SemanticRuleDefinition {
                        id: id(n),
                        signature,
                    },
                    SemanticRuleLimits::new(3, 24, 9),
                )?,
                body: DraftingSourceRecord::new(Body { text: text.into() }),
                origin,
            });
        }
        Ok(Self {
            inner: Base::new(root, other, BaseFault::None)?,
            rows,
            cache: RefCell::new(vec![]),
            ignore,
            queries: Cell::new(0),
            charges: Cell::new(0),
        })
    }
    fn walk(&self, sink: &mut dyn DesignResourceSink) -> Result<(), DesignResourceLimitError> {
        let Self {
            inner,
            rows,
            cache,
            ignore,
            queries,
            charges,
        } = self;
        fixed(ignore);
        let _: (&Cell<usize>, &Cell<usize>) = (queries, charges);
        inner.charge_resources(sink)?;
        sink.charge_entries(rows.len(), at("signature.rules"))?;
        for (index, row) in rows.iter().enumerate() {
            let Row {
                declaration,
                body,
                origin,
            } = row;
            fixed(origin);
            charge_semantic_rule_resources(declaration, sink, DesignResourceScope::SourceHost)?;
            let Body { text } = body.data();
            sink.charge_text(
                text,
                DesignResourceLocation {
                    scope: DesignResourceScope::SourceHost,
                    owner: Some(declaration.id()),
                    field: "signature.body",
                    index: Some(index),
                },
            )?;
        }
        let cache = cache.borrow();
        sink.charge_entries(cache.len(), at("signature.cache"))?;
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
    type RuleBody = Body;
    fn rule_source<'s>(
        &'s self,
        request: DraftingRuleRequest<'_>,
    ) -> Result<DraftingRuleResponse<'s, EmptyHost, Self>, Cause> {
        self.queries.set(self.queries.get() + 1);
        let row = self
            .rows
            .iter()
            .find(|row| row.declaration.id() == request.rule)
            .ok_or(Cause::Missing(request.rule))?;
        let origin = match (row.origin, request.origin) {
            (Origin::Design, DraftingRuleSelection::Design) => DraftingRuleOrigin::Design {
                root: self.root(),
                snapshot: self.snapshot(),
            },
            (Origin::Profile, DraftingRuleSelection::Profile(selected)) => {
                DraftingRuleOrigin::Profile(self.profile_source(selected)?)
            }
            _ => return Err(Cause::Origin(request.rule)),
        };
        Ok(DraftingRuleSource {
            declaration: &row.declaration,
            body: &row.body,
            origin,
        })
    }
}
#[test]
fn complete_named_matches_preserve_both_original_orders_sources_and_derived_output() -> TestResult {
    for case in [Case::Exact, Case::DesignInput] {
        let root = root(case)?;
        let other = super::root()?;
        for profile in [false, true] {
            let source = Provider::new(&root, &other, case, false)?;
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
            let text = source.inner.text(&selected) + 135 - if profile { 0 } else { 16 };
            let work = context
                .prepare_rule_binding(
                    id(operation),
                    kind,
                    selection,
                    DraftingRuleBindingLimits::new(limits(26, text, 9)?, 1156),
                )
                .map_err(|e| e.to_string())?;
            let parent = work.bind_source().map_err(|e| e.to_string())?;
            let before = source.charges.get();
            let signature = parent.bind_signature().map_err(|e| e.to_string())?;
            assert!(core::ptr::eq(signature.parent(), &parent));
            assert_eq!(source.charges.get(), before + 1);
            assert_eq!(source.queries.get(), 1);
            assert_eq!(
                (
                    signature.demand().collection_entries(),
                    signature.demand().text_bytes(),
                    signature.demand().counter_bits()
                ),
                (26, text, 9)
            );
            assert_eq!(signature.shape().matching_inspection_bound(), 1156);
            assert_eq!(signature.input_count(), 2);
            assert!(signature.input_match(2).is_none());
            for (input_index, requirement_index) in [(0, 1), (1, 0)] {
                let matched = signature
                    .input_match(input_index)
                    .ok_or("complete original named pair")?;
                assert_eq!(
                    (matched.input_index(), matched.requirement_index()),
                    (input_index, requirement_index)
                );
                assert!(core::ptr::eq(
                    matched.operation(),
                    work.operation().operation()
                ));
                assert!(core::ptr::eq(matched.rule(), parent.source().declaration));
                match (
                    matched.pair(),
                    work.operation().operation(),
                    &parent.source().declaration.definition().signature,
                ) {
                    (
                        DraftingRuleInputPair::Logical { input, requirement },
                        SemanticOperation::LogicalDerivation(operation),
                        SemanticRuleSignature::Logical { inputs, .. },
                    ) => {
                        assert!(core::ptr::eq(
                            input,
                            operation
                                .definition()
                                .inputs
                                .get(input_index)
                                .ok_or("original input")?
                        ));
                        assert!(core::ptr::eq(
                            requirement,
                            inputs
                                .get(requirement_index)
                                .ok_or("original requirement")?
                        ));
                        assert_eq!(input.role, requirement.role);
                        assert_ne!(
                            input.role.as_str().as_ptr(),
                            requirement.role.as_str().as_ptr()
                        );
                        if input_index == 0 {
                            match input.source {
                                LogicalDerivationInputSource::Profile { parameter, kind } => {
                                    assert!(!matches!(case, Case::DesignInput));
                                    assert_eq!(
                                        (parameter.id(), kind),
                                        (id(600), LogicalInputKind::NotionSize)
                                    );
                                }
                                LogicalDerivationInputSource::Logical(target) => {
                                    assert!(matches!(case, Case::DesignInput));
                                    assert_eq!(target.declaration, id(91));
                                }
                                _ => return Err("original logical source representation".into()),
                            }
                        }
                    }
                    (
                        DraftingRuleInputPair::Buttonhole { input, requirement },
                        SemanticOperation::Buttonhole(operation),
                        SemanticRuleSignature::Buttonhole { inputs },
                    ) => {
                        assert!(core::ptr::eq(
                            input,
                            operation
                                .definition()
                                .inputs
                                .get(input_index)
                                .ok_or("original policy input")?
                        ));
                        assert!(core::ptr::eq(
                            requirement,
                            inputs
                                .get(requirement_index)
                                .ok_or("original policy requirement")?
                        ));
                        assert_eq!(input.role, requirement.role);
                        assert_eq!(
                            operation.definition().button,
                            NotionSize::Declaration(id(91))
                        );
                    }
                    _ => return Err("original pair families".into()),
                }
                assert!(!format!("{matched:?}").contains("source_a"));
            }
            if profile {
                assert!(signature.output().is_none());
            } else {
                let output = signature.output().ok_or("actual logical output")?;
                assert!(core::ptr::eq(output.inputs(), root.inputs().logicals()));
                assert_eq!(output.index(), 0);
                assert_eq!(output.declaration().id(), id(90));
                assert!(
                    matches!(output.declaration().definition().state,LogicalInputState::Derived {operation} if operation==id(80))
                );
                assert_eq!(
                    output
                        .declaration()
                        .definition()
                        .domain
                        .first()
                        .ok_or("original domain")?
                        .as_str(),
                    "output_a"
                );
            }
            let mut meter = DesignResourceMeter::new(limits(28, text + 3, 10)?);
            meter.charge_entries(2, at("prior"))?;
            meter.charge_text("old", at("prior"))?;
            meter.charge_counter_bits(1, at("prior"))?;
            signature.charge_resources(&mut meter)?;
            let demand = meter.finish()?;
            assert_eq!(
                (
                    demand.collection_entries(),
                    demand.text_bytes(),
                    demand.counter_bits()
                ),
                (28, text + 3, 10)
            );
            assert_eq!(source.queries.get(), 1);
        }
    }
    Ok(())
}
#[test]
fn result_count_missing_role_and_exact_kind_refusals_preserve_first_original_pair_indices(
) -> TestResult {
    for (case, issue) in [
        (
            Case::Result,
            DraftingRuleSignatureIssue::ResultKind {
                expected: LogicalInputKind::PocketOpening,
                actual: LogicalInputKind::NotionSize,
            },
        ),
        (
            Case::Count,
            DraftingRuleSignatureIssue::InputCount {
                inputs: 2,
                requirements: 1,
            },
        ),
        (
            Case::Extra,
            DraftingRuleSignatureIssue::InputCount {
                inputs: 2,
                requirements: 3,
            },
        ),
        (
            Case::Role,
            DraftingRuleSignatureIssue::MissingRole { input: 0 },
        ),
        (
            Case::Numeric,
            DraftingRuleSignatureIssue::InputKind {
                input: 1,
                requirement: 0,
                expected: SemanticRuleInputKind::Numeric(CanonicalNumericInputKind::Length),
                actual: SemanticRuleInputKind::Numeric(CanonicalNumericInputKind::Scalar(
                    ScalarInputKind::Length,
                )),
            },
        ),
        (
            Case::Logical,
            DraftingRuleSignatureIssue::InputKind {
                input: 0,
                requirement: 1,
                expected: SemanticRuleInputKind::Logical(LogicalInputKind::NotionSize),
                actual: SemanticRuleInputKind::Logical(LogicalInputKind::PocketOpening),
            },
        ),
    ] {
        let root = root(case)?;
        let other = super::root()?;
        let source = Provider::new(&root, &other, case, false)?;
        let context = scope(&root)?
            .bind_drafting_context(&source, limits(1000, 10000, 1000)?)
            .map_err(|e| e.to_string())?;
        let work = context
            .prepare_rule_binding(
                id(80),
                SemanticOperationKind::LogicalDerivation,
                DraftingRuleSelection::Design,
                DraftingRuleBindingLimits::new(limits(1000, 10000, 1000)?, usize::MAX),
            )
            .map_err(|e| e.to_string())?;
        let parent = work.bind_source().map_err(|e| e.to_string())?;
        let error = parent
            .bind_signature()
            .err()
            .ok_or("complete signature refusal")?;
        assert!(
            matches!(error.refusal(),DraftingRuleSignatureRefusal::Mismatch(actual) if actual==&issue)
        );
        assert!(core::ptr::eq(error.parent(), &parent));
        assert!(error.output().is_none());
        assert!(error.demand().is_some());
        assert_eq!(source.queries.get(), 1);
    }
    Ok(())
}
#[test]
fn actual_root_output_missing_kind_unknown_known_and_other_derived_causes_remain_original(
) -> TestResult {
    for case in [
        Case::MissingOutput,
        Case::OutputKind,
        Case::OutputUnknown,
        Case::OutputKnown,
        Case::OtherDerived,
    ] {
        let root = root(case)?;
        let other = super::root()?;
        let source = Provider::new(&root, &other, case, false)?;
        let context = scope(&root)?
            .bind_drafting_context(&source, limits(1000, 10000, 1000)?)
            .map_err(|e| e.to_string())?;
        let work = context
            .prepare_rule_binding(
                id(80),
                SemanticOperationKind::LogicalDerivation,
                DraftingRuleSelection::Design,
                DraftingRuleBindingLimits::new(limits(1000, 10000, 1000)?, usize::MAX),
            )
            .map_err(|e| e.to_string())?;
        let parent = work.bind_source().map_err(|e| e.to_string())?;
        let error = parent
            .bind_signature()
            .err()
            .ok_or("actual output refusal")?;
        match (case, error.refusal()) {
            (
                Case::MissingOutput,
                DraftingRuleSignatureRefusal::Output(LogicalInputLookupError::Missing {
                    declaration,
                    expected,
                }),
            ) => {
                assert_eq!(
                    (*declaration, *expected),
                    (id(90), LogicalInputKind::PocketOpening)
                );
                assert!(error.output().is_none());
            }
            (
                Case::OutputKind,
                DraftingRuleSignatureRefusal::Output(LogicalInputLookupError::Kind {
                    expected,
                    actual,
                }),
            ) => {
                assert_eq!(*expected, LogicalInputKind::PocketOpening);
                assert_eq!(actual.kind(), LogicalInputKind::NotionSize);
                assert!(core::ptr::eq(
                    *actual,
                    error.output().ok_or("original wrong-kind output")?
                ));
            }
            (
                Case::OutputUnknown | Case::OutputKnown,
                DraftingRuleSignatureRefusal::OutputNotDerived,
            ) => {
                let output = error.output().ok_or("original non-Derived output")?;
                assert_eq!(
                    output.definition().state.tag(),
                    if matches!(case, Case::OutputUnknown) {
                        LogicalInputStateTag::Unknown
                    } else {
                        LogicalInputStateTag::Known
                    }
                );
            }
            (
                Case::OtherDerived,
                DraftingRuleSignatureRefusal::DerivedOperation { expected, actual },
            ) => assert_eq!((*expected, *actual), (id(80), id(81))),
            _ => return Err("original actual output cause".into()),
        }
        if let Some(actual) = error.output() {
            assert!(core::ptr::eq(
                actual,
                root.inputs()
                    .logicals()
                    .declarations()
                    .first()
                    .ok_or("actual first output record")?
            ));
        }
        assert!(core::ptr::eq(error.parent(), &parent));
        assert_eq!(source.queries.get(), 1);
    }
    Ok(())
}
#[test]
fn explicit_empty_signatures_keep_mandatory_button_and_actual_derived_output_without_values(
) -> TestResult {
    let root = root(Case::Empty)?;
    let other = super::root()?;
    for profile in [false, true] {
        let source = Provider::new(&root, &other, Case::Empty, false)?;
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
        let parent = work.bind_source().map_err(|e| e.to_string())?;
        let signature = parent.bind_signature().map_err(|e| e.to_string())?;
        assert_eq!(signature.input_count(), 0);
        assert!(signature.input_match(0).is_none());
        assert_eq!(signature.output().is_none(), profile);
        assert_eq!(source.queries.get(), 1);
        if let SemanticOperation::Buttonhole(operation) = work.operation().operation() {
            assert_eq!(
                operation.definition().button,
                NotionSize::Declaration(id(91))
            );
        }
    }
    Ok(())
}
#[test]
fn fresh_complete_content_and_work_refusals_precede_bad_role_and_never_requery_sources(
) -> TestResult {
    for (ignore, entries, work_failure) in
        [(false, 26, false), (true, 26, false), (false, 27, true)]
    {
        let root = root(Case::Role)?;
        let other = super::root()?;
        let source = Provider::new(&root, &other, Case::Role, ignore)?;
        let context = scope(&root)?
            .bind_drafting_context(&source, limits(1000, 10000, 1000)?)
            .map_err(|e| e.to_string())?;
        let work = context
            .prepare_rule_binding(
                id(80),
                SemanticOperationKind::LogicalDerivation,
                DraftingRuleSelection::Design,
                DraftingRuleBindingLimits::new(limits(entries, 10000, 1000)?, 1156),
            )
            .map_err(|e| e.to_string())?;
        let parent = work.bind_source().map_err(|e| e.to_string())?;
        assert_eq!(parent.demand().collection_entries(), 26);
        // Deliberate cache-growth negative control falsifies earlier content/work credit.
        source.cache.borrow_mut().push(7);
        let error = parent
            .bind_signature()
            .err()
            .ok_or("fresh signature admission refusal")?;
        let DraftingRuleSignatureRefusal::Work(cause) = error.refusal() else {
            return Err("original fresh work cause".into());
        };
        if work_failure {
            assert!(matches!(
                cause.refusal(),
                DraftingRuleWorkRefusal::MatchingInspections {
                    allowed: 1156,
                    required: 1225
                }
            ));
        } else {
            let expected = DesignResourceLimitError::Count {
                unit: DesignResourceCount::CollectionEntries,
                maximum: 26,
                consumed: 26,
                requested: 1,
                location: at("signature.cache"),
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
        assert!(error.output().is_none());
        assert_eq!(source.queries.get(), 1);
    }
    Ok(())
}
