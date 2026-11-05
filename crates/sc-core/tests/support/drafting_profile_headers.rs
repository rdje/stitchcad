//! Complete actual Profile operation-input headers under original Root owners and a unique budget.
use super::{
    limits, scope, EmptyHost, Frame, Provider as BaseProvider, Snapshot, SourceError, TestResult,
};
use sc_core::{command::DesignRevision, construction::*, design::*, ontology::*, recipe::*};
use std::cell::Cell;
#[path = "drafting_profile_numeric.rs"]
mod numeric;
#[path = "drafting_operation_inputs.rs"]
mod operation_inputs;
#[path = "drafting_profile_parameters.rs"]
mod parameters;
#[path = "drafting_program_rule_work.rs"]
mod rule_work;
use sc_core::value::*;
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
struct Provider<'a> {
    base: BaseProvider<'a>,
    other: &'a DesignInputs<EmptyHost>,
    inputs: sc_core::value::CanonicalInputs,
    selected: SelectedProfilePin,
    wrong_pin: SelectedProfilePin,
    body: DraftingSourceRecord<[u8; 4]>,
    calls: Cell<usize>,
    extra_after: Cell<Option<usize>>,
    rule_calls: Cell<usize>,
    rule_mode: Cell<u8>,
    rules: Vec<RuleRow>,
    extra_entries: Cell<usize>,
    mode: u8,
    header_override: Cell<Option<u8>>,
    parameter_mode: Cell<u8>,
    parameter_calls: Cell<usize>,
    metadata: Vec<DraftingSourceRecord<ParameterMapping>>,
    numeric_metadata: Vec<DraftingSourceRecord<numeric::NumericMapping>>,
    numeric_mode: u8,
    numeric_late: bool,
    numeric_calls: Cell<usize>,
    numeric_body: Option<DraftingSourceRecord<[u8; 4]>>,
    component_import: Option<DraftingSourceRecord<Vec<u8>>>,
    component_calls: Cell<usize>,
    component_mode: Cell<u8>,
}
struct RuleRow {
    declaration: SemanticRuleDeclaration,
    body: DraftingSourceRecord<[u8; 4]>,
}
struct ParameterMapping {
    parameter: ProfileParameterRef,
    target: LogicalInputTarget,
    text: String,
}
impl<'a> Provider<'a> {
    fn new(
        garment: &'a DesignGarment<EmptyHost>,
        other: &'a DesignInputs<EmptyHost>,
        mode: u8,
    ) -> TestResult<Self> {
        let original = super::super::semantic_root::canonical()?;
        let mut logicals = original.logicals().declarations().to_vec();
        logicals.push(LogicalInputDeclaration::new(
            LogicalInputDeclarationDefinition {
                id: id(91),
                source: id(1091),
                kind: LogicalInputKind::NotionSize,
                domain: vec![sc_core::name::MachineToken::new("hardware")?],
                state: LogicalInputState::Unknown {
                    observation: id(1191),
                },
            },
            LogicalInputLimits::new(1, 0, 8, 2),
        )?);
        let inputs = CanonicalInputs::with_logicals(
            original.lengths().to_vec(),
            original.scalars().to_vec(),
            LogicalInputs::new(logicals, LogicalInputInventoryLimits::new(2, 4))?,
        )?;
        let metadata = [
            (702, 90, LogicalInputKind::PocketOpening),
            (703, 91, LogicalInputKind::NotionSize),
        ]
        .into_iter()
        .map(|(parameter, declaration, kind)| {
            DraftingSourceRecord::new(ParameterMapping {
                parameter: ProfileParameterRef::new(id(parameter)),
                target: LogicalInputTarget {
                    declaration: id(declaration),
                    kind,
                },
                text: format!("private mapping {parameter}"),
            })
        })
        .collect();
        Ok(Self {
            base: BaseProvider::new(garment),
            other,
            inputs,
            selected: pin("original")?,
            wrong_pin: pin("other")?,
            body: DraftingSourceRecord::new([1, 2, 3, 4]),
            calls: Cell::new(0),
            extra_after: Cell::new(None),
            rule_calls: Cell::new(0),
            rule_mode: Cell::new(0),
            rules: vec![],
            extra_entries: Cell::new(0),
            mode,
            header_override: Cell::new(None),
            parameter_mode: Cell::new(0),
            parameter_calls: Cell::new(0),
            metadata,
            numeric_metadata: vec![],
            numeric_mode: 0,
            numeric_late: false,
            numeric_calls: Cell::new(0),
            numeric_body: None,
            component_import: None,
            component_calls: Cell::new(0),
            component_mode: Cell::new(0),
        })
    }
}
fn pin(profile: &str) -> TestResult<SelectedProfilePin> {
    Ok(SelectedProfilePin::new(SelectedProfilePinDefinition {
        profile_id: profile.into(),
        schema_version: "schema".into(),
        content_version: "content".into(),
    })?)
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
        self.base.frame()
    }
    fn snapshot(&self) -> &DraftingSourceRecord<Snapshot> {
        self.base.snapshot()
    }
    fn validate_current(&self) -> Result<(), SourceError> {
        self.base.validate_current()
    }
    fn geometry(
        &self,
        request: DraftingGeometryReference,
    ) -> DraftingGeometrySourceResult<'_, Frame, Snapshot, [u8], SourceError> {
        self.base.geometry(request)
    }
    fn charge_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        self.base.charge_resources(sink)?;
        if self
            .extra_after
            .get()
            .is_some_and(|after| self.base.walks.get() >= after)
        {
            sink.charge_entries(
                self.extra_entries.get(),
                DesignResourceLocation {
                    scope: DesignResourceScope::SourceHost,
                    owner: None,
                    field: "rule.late-cache",
                    index: None,
                },
            )?;
        }
        if let Some(source) = &self.component_import {
            sink.charge_entries(
                source.data().len(),
                DesignResourceLocation {
                    scope: DesignResourceScope::SourceHost,
                    owner: None,
                    field: "component.original-import",
                    index: None,
                },
            )?;
        }
        charge_design_input_resources(self.other, sink)?;
        charge_canonical_input_resources(&self.inputs, sink)?;
        sink.charge_entries(
            self.rules.len(),
            DesignResourceLocation {
                scope: DesignResourceScope::SourceHost,
                owner: None,
                field: "rule.records",
                index: None,
            },
        )?;
        for (index, row) in self.rules.iter().enumerate() {
            charge_semantic_rule_resources(
                &row.declaration,
                sink,
                DesignResourceScope::SourceHost,
            )?;
            sink.charge_entries(
                row.body.data().len(),
                DesignResourceLocation {
                    scope: DesignResourceScope::SourceHost,
                    owner: Some(row.declaration.id()),
                    field: "rule.body",
                    index: Some(index),
                },
            )?;
        }
        for selected in [&self.selected, &self.wrong_pin] {
            charge_selected_profile_pin_resources(
                selected,
                sink,
                DesignResourceScope::SourceHost,
                None,
                None,
            )?;
        }
        sink.charge_entries(
            self.metadata.len(),
            DesignResourceLocation {
                scope: DesignResourceScope::SourceHost,
                owner: None,
                field: "profile.parameters",
                index: None,
            },
        )?;
        for (index, record) in self.metadata.iter().enumerate() {
            sink.charge_text(
                &record.data().text,
                DesignResourceLocation {
                    scope: DesignResourceScope::SourceHost,
                    owner: None,
                    field: "profile.parameter.text",
                    index: Some(index),
                },
            )?;
        }
        sink.charge_entries(
            self.numeric_metadata.len(),
            DesignResourceLocation {
                scope: DesignResourceScope::SourceHost,
                owner: None,
                field: "profile.numeric-parameters",
                index: None,
            },
        )?;
        for (index, record) in self.numeric_metadata.iter().enumerate() {
            sink.charge_text(
                &record.data().text,
                DesignResourceLocation {
                    scope: DesignResourceScope::SourceHost,
                    owner: None,
                    field: "profile.numeric-parameter.text",
                    index: Some(index),
                },
            )?;
        }
        if let Some(record) = &self.numeric_body {
            sink.charge_entries(
                record.data().len(),
                DesignResourceLocation {
                    scope: DesignResourceScope::SourceHost,
                    owner: None,
                    field: "profile.numeric-alternate-body",
                    index: None,
                },
            )?;
        }
        sink.charge_entries(
            self.body.data().len(),
            DesignResourceLocation {
                scope: DesignResourceScope::SourceHost,
                owner: None,
                field: "profile.body",
                index: None,
            },
        )
    }
}
impl DraftingGarmentProvider<EmptyHost> for Provider<'_> {
    fn garment_source(&self) -> DraftingGarmentSource<'_, EmptyHost, Snapshot> {
        self.base.garment_source()
    }
}
impl DraftingInitialInputProvider<EmptyHost> for Provider<'_> {
    type Metadata = ();
    fn initial_input<'a>(
        &'a self,
        index: usize,
        input: FormulaCanonicalInput<'a>,
    ) -> DraftingInitialInputSourceResult<'a, (), Snapshot, SourceError> {
        self.base.initial_input(index, input)
    }
}
impl DraftingPlannedOutputProvider<EmptyHost> for Provider<'_> {
    fn planned_output<'v, 'n, 'i>(
        &'v self,
        request: DraftingOutputPosition<'v, 'n, 'i>,
    ) -> DraftingPlannedOutputSourceResult<'v, Frame, Snapshot, SourceError> {
        self.base.planned_output(request)
    }
}
impl DraftingGeometryStepProvider<EmptyHost> for Provider<'_> {
    fn planned_geometry<'v, 'i>(
        &'v self,
        request: DraftingGeometryStepRequest<'v, 'i>,
    ) -> DraftingPlannedOutputSourceResult<'v, Frame, Snapshot, SourceError> {
        self.base.planned_geometry(request)
    }
}
impl DraftingProfileProvider<EmptyHost> for Provider<'_> {
    type Profile = [u8];
    fn profile_source<'s>(
        &'s self,
        _: &SelectedProfilePin,
    ) -> Result<DraftingProfileSource<'s, EmptyHost, Snapshot, [u8]>, SourceError> {
        self.calls.set(self.calls.get() + 1);
        let mode = self.header_override.get().unwrap_or(self.mode);
        if mode == 7 {
            return Err(SourceError::Missing);
        }
        Ok(DraftingProfileSource {
            root: if mode == 5 {
                self.other
            } else {
                self.base.garment.inputs()
            },
            selected: if mode == 5
                || (mode == 3 && self.calls.get() == 2)
                || (self.rule_mode.get() == 4 && self.rule_calls.get() > 0)
            {
                &self.wrong_pin
            } else {
                &self.selected
            },
            inputs: &self.inputs,
            profile: &self.body,
            snapshot: if mode == 6 {
                self.base.alternate.snapshot()
            } else {
                self.base.snapshot()
            },
        })
    }
}
impl DraftingProfileLogicalProvider<EmptyHost> for Provider<'_> {
    type Metadata = ParameterMapping;
    fn profile_logical_source<'s>(
        &'s self,
        mut profile: DraftingProfileSource<'s, EmptyHost, Snapshot, [u8]>,
        parameter: ProfileParameterRef,
    ) -> Result<DraftingProfileLogicalResponse<'s, EmptyHost, Self>, SourceError> {
        self.parameter_calls.set(self.parameter_calls.get() + 1);
        let mode = self.parameter_mode.get();
        let late = self.parameter_calls.get() == 2;
        if mode == 5 {
            return Err(SourceError::Missing);
        }
        let metadata = if mode == 4 && late {
            self.metadata
                .iter()
                .find(|record| record.data().parameter != parameter)
        } else {
            self.metadata
                .iter()
                .find(|record| record.data().parameter == parameter)
        }
        .ok_or(SourceError::Missing)?;
        let mut declaration = self
            .inputs
            .logicals()
            .declaration(metadata.data().target.declaration)
            .ok_or(SourceError::Missing)?;
        if mode == 3 && late {
            declaration = self
                .inputs
                .logicals()
                .declaration(id(
                    if metadata.data().target.kind == LogicalInputKind::PocketOpening {
                        91
                    } else {
                        90
                    },
                ))
                .ok_or(SourceError::Missing)?;
        }
        if mode == 6 && late {
            profile.selected = &self.wrong_pin;
        }
        if mode == 7 && late {
            profile.inputs = self.base.garment.inputs().inputs();
            declaration = profile
                .inputs
                .logicals()
                .declaration(metadata.data().target.declaration)
                .ok_or(SourceError::Missing)?;
        }
        Ok(DraftingProfileLogicalSource {
            profile,
            parameter: &metadata.data().parameter,
            declaration,
            metadata,
        })
    }
}
trait ParameterProvider:
    HeaderProvider
    + DraftingInitialInputProvider<EmptyHost, Metadata = ()>
    + DraftingProfileLogicalProvider<EmptyHost, Metadata = ParameterMapping>
{
}
impl ParameterProvider for Provider<'_> {}
type ParameterSourceView<'a> = dyn ParameterProvider<
        Frame = Frame,
        Snapshot = Snapshot,
        Geometry = [u8],
        Error = SourceError,
        Profile = [u8],
    > + 'a;
trait HeaderProvider: super::SemanticProvider + DraftingProfileProvider<EmptyHost> {}
impl HeaderProvider for Provider<'_> {}
impl super::SemanticProvider for Provider<'_> {}
type SourceView<'a> = dyn HeaderProvider<
        Frame = Frame,
        Snapshot = Snapshot,
        Geometry = [u8],
        Error = SourceError,
        Metadata = (),
        Profile = [u8],
    > + 'a;
#[test]
fn profile_input_headers_keep_original_fields_pins_sources_and_complete_fresh_stage_order(
) -> TestResult {
    for case in 0..9u8 {
        let count = if case == 1 { 0 } else { 2 };
        let garment = super::root_numeric_suppliers::fixture(false, false, count, case == 8)?;
        let other = super::root_numeric_suppliers::fixture(false, false, count, case == 8)?;
        let source = Provider::new(&garment, other.inputs(), case)?;
        let source_view: &SourceView<'_> = &source;
        let content = limits(100000, 100000, 100000)?;
        let context = scope(garment.inputs())?
            .bind_drafting_context(source_view, content)
            .map_err(|e| e.to_string())?;
        let admitted_garment = context
            .admit_garment_source(content)
            .map_err(|e| e.to_string())?;
        let root_scope = context
            .prepare_design_semantic_sources(content)
            .map_err(|e| e.to_string())?;
        let grainline = GrainlinePlacement::new(id(80000), id(90000))?;
        let mut steps = vec![
            DraftingProgramStep::Semantic { ordinal: 1 },
            DraftingProgramStep::Drafting(DraftingOperationInput::PlaceGrainline(&grainline)),
        ];

        if case == 8 {
            steps.push(DraftingProgramStep::Semantic { ordinal: 2 });
        }
        let raw = DraftingProgramInputs::new(context, steps);
        let pp = DraftingProgramLimits::new(content, 1000, 1000, 10000);
        let program = raw
            .admit(&pp)
            .map_err(|e| e.to_string())?
            .normalized()
            .map_err(|e| e.to_string())?;
        let names = DraftingProgramNameInputs::new(&program, vec![], vec![]);
        let np = DraftingProgramNameLimits::new(content, 1000, 10000);
        let work = names.admit_name_work(&np).map_err(|e| e.to_string())?;
        let positions = work.bind_output_positions().map_err(|e| e.to_string())?;
        let roles = positions.bind_role_sources().map_err(|e| e.to_string())?;
        let initials = roles.bind_initial_sources().map_err(|e| e.to_string())?;
        let namespace = initials
            .bind_initial_namespace()
            .map_err(|e| e.to_string())?;
        if case == 0 {
            let refused = namespace
                .check_root_sources(FormulaSourceCheckLimits::new(2, 1000, 0, 10000))
                .expect_err("whole Derived target quota precedes source acceptance");
            assert!(matches!(
                refused.cause().refusal(),
                FormulaSourceCheckRefusal::Targets {
                    allowed: 0,
                    required: 3
                }
            ));
        }
        let sources = namespace
            .check_root_sources(FormulaSourceCheckLimits::new(2, 1000, 3, 10000))
            .map_err(|e| e.cause().to_string())?;
        let dependencies = sources
            .proof()
            .prepare_dependencies(FormulaSourceDependencyLimits::new(2, 0, 3))
            .map_err(|e| e.to_string())?;
        let graph = dependencies
            .admit_acyclic(FormulaSourceCycleLimits::new(2, 4))
            .map_err(|e| e.to_string())?;
        let op = DraftingProgramOrderLimits::new(1000, 10000, 100000);
        let order = namespace.check_order(&op).map_err(|e| e.to_string())?;
        let inputs = sources
            .dependency_inputs(&order, &graph)
            .map_err(|e| e.to_string())?;
        let cp = DraftingProgramCanonicalLimits::new(10000, 100000);
        let canonical = inputs
            .bind_canonical_inputs(&cp)
            .map_err(|e| e.to_string())?;
        let gp = DraftingProgramGeometryLimits::new(content, 10000, 100000, 100000);
        let geometry_work = canonical
            .admit_geometry_work(&gp)
            .map_err(|e| e.to_string())?;
        let geometry = geometry_work.bind_geometry().map_err(|e| e.to_string())?;
        let selected = SelectedProfilePin::new(SelectedProfilePinDefinition {
            profile_id: "original".into(),
            schema_version: "schema".into(),
            content_version: "content".into(),
        })?;

        let mut rows = vec![DraftingSemanticSelectionRow::Rule {
            ordinal: 1,
            origin: DraftingRuleSelection::Design,
        }];
        for index in 0..count {
            rows.push(DraftingSemanticSelectionRow::Profile {
                location: DraftingSemanticSelectionLocation::Semantic {
                    ordinal: 1,
                    field: DraftingSemanticSelectionField::LogicalInput { index: 5 + index },
                },
                selected: &selected,
            });
        }
        if case == 8 {
            rows.push(DraftingSemanticSelectionRow::Rule {
                ordinal: 2,
                origin: DraftingRuleSelection::Design,
            });
            rows.push(DraftingSemanticSelectionRow::Profile {
                location: DraftingSemanticSelectionLocation::Semantic {
                    ordinal: 2,
                    field: DraftingSemanticSelectionField::ButtonholeButton,
                },
                selected: &selected,
            });
        }
        let raw_selections = DraftingSemanticSelectionInputs::new(rows);
        let selections = raw_selections
            .admit(DraftingSemanticSelectionLimits::new(content, 10000))
            .map_err(|e| e.to_string())?;
        let actual = geometry
            .semantic_inputs(&admitted_garment, &selections)
            .map_err(|e| e.to_string())?;
        let mut meter = DesignResourceMeter::new(content);
        actual.charge_resources(&mut meter)?;
        let demand = meter.finish()?;

        let occurrences = 8 + count + if case == 8 { 2 } else { 0 };
        let row_count = 1 + count + if case == 8 { 2 } else { 0 };
        let passes = if case == 8 {
            13
        } else if case == 1 {
            9
        } else if case == 2 {
            11
        } else {
            12
        };
        let per_pass = (demand.collection_entries() + 8) * (demand.collection_entries() + 8);
        let policy = DraftingProgramSemanticWorkLimits::new(
            DraftingProgramSemanticPopulationLimits::new(
                content,
                occurrences,
                2 * occurrences * row_count,
                occurrences + row_count,
            ),
            passes,
            DraftingProgramSemanticChildContentLimits::new(
                passes as u128 * demand.collection_entries() as u128,
                passes as u128 * demand.text_bytes() as u128,
                passes as u128 * u128::from(demand.counter_bits()),
            ),
            passes as u128 * per_pass as u128,
            occurrences + row_count + passes,
        );
        let checked = actual
            .check_direct_selections(&policy)
            .map_err(|e| e.to_string())?;
        let owner = checked
            .root_input_scope(&root_scope)
            .map_err(|e| e.to_string())?;

        let root = owner
            .bind_root_input_members()
            .map_err(|e| e.to_string())?
            .bind_root_logical_suppliers()
            .map_err(|e| e.to_string())?
            .bind_root_numeric_suppliers()
            .map_err(|e| e.to_string())?;
        assert_eq!(
            root.logical()
                .members()
                .owner()
                .selections()
                .reserved_child_passes(),
            9
        );
        source.calls.set(0);
        source.base.walks.set(0);
        if case == 4 {
            source.base.extra_charge.set(100001);
        }
        let result = root.bind_profile_input_headers();
        if case == 0 || case == 1 || case == 8 {
            let proof = result.map_err(|e| e.to_string())?;
            let header_count = count + usize::from(case == 8);
            assert_eq!(proof.records().len(), header_count);
            assert_eq!(
                proof.child_pass_count(),
                if header_count == 0 {
                    0
                } else {
                    header_count + 1
                }
            );
            assert_eq!(proof.demand(), if count == 0 { None } else { Some(demand) });
            assert_eq!(source.calls.get(), header_count);
            assert_eq!(
                source.base.walks.get(),
                if header_count == 0 {
                    0
                } else {
                    header_count + 1
                }
            );
            assert!(core::ptr::eq(
                proof.root().logical().members().owner().inputs(),
                &actual
            ));
            assert_eq!(
                proof
                    .root()
                    .logical()
                    .members()
                    .owner()
                    .selections()
                    .reserved_child_passes(),
                passes
            );
            for (index, record) in proof.records().iter().enumerate() {
                let request = record.request();
                let button = case == 8 && index == 2;
                assert_eq!(
                    (request.ordinal(), request.position()),
                    if button { (2, 3) } else { (1, 1) }
                );
                assert_eq!(request.row_index(), if button { 4 } else { 1 + index });
                assert_eq!(
                    request.parameter(),
                    ProfileParameterRef::new(id(if button { 703 } else { 702 }))
                );
                assert_eq!(
                    request.kind(),
                    if button {
                        sc_core::value::LogicalInputKind::NotionSize
                    } else {
                        sc_core::value::LogicalInputKind::PocketOpening
                    }
                );
                if button {
                    assert!(matches!(
                        request.field(),
                        DraftingSemanticField::ButtonholeButton(NotionSize::Profile(_))
                    ))
                } else {
                    assert!(
                        matches!(request.field(),DraftingSemanticField::LogicalInput {index:original,..} if original==5+index)
                    );
                }
                assert!(core::ptr::eq(
                    request.operation(),
                    garment
                        .inputs()
                        .semantic_recipe()
                        .operations()
                        .get(usize::from(button))
                        .ok_or("actual original operation")?
                ));
                assert!(core::ptr::eq(request.selected(), &selected));
                assert!(core::ptr::eq(record.source().selected(), &selected));
                let original = record.source().source();
                assert!(core::ptr::eq(original.root, garment.inputs()));
                assert!(core::ptr::eq(original.selected, &source.selected));
                assert!(!core::ptr::eq(original.selected, request.selected()));
                assert!(core::ptr::eq(original.inputs, &source.inputs));
                assert!(!core::ptr::eq(original.inputs, garment.inputs().inputs()));
                assert!(original.profile.is_same_record(&source.body));
                assert!(original.snapshot.is_same_record(source.base.snapshot()));
                assert!(original
                    .inputs
                    .logicals()
                    .declarations()
                    .first()
                    .is_some_and(|declaration| declaration.definition().state.tag()
                        == sc_core::value::LogicalInputStateTag::Derived));
                assert!(!format!("{record:?}").contains("original"));
            }
        } else {
            let error = result.expect_err("complete stage or original Profile header refusal");
            assert_eq!(error.header_count(), Some(2));
            assert!(core::ptr::eq(
                error.owner().logical().members().owner().inputs(),
                &actual
            ));
            assert_eq!(error.demand().is_some(), !matches!(case, 2 | 4));
            assert_eq!(
                error
                    .owner()
                    .logical()
                    .members()
                    .owner()
                    .selections()
                    .reserved_child_passes(),
                if case == 2 { 9 } else { 12 }
            );
            match error.refusal() {
                DraftingProgramProfileHeaderRefusal::Work(cause) if case == 2 => {
                    assert!(matches!(
                        cause.refusal(),
                        DraftingProgramSemanticWorkRefusal::ChildPasses {
                            allowed: 11,
                            reserved: 9,
                            requested: 3
                        }
                    ));
                    assert_eq!((source.calls.get(), source.base.walks.get()), (0, 0));
                }
                DraftingProgramProfileHeaderRefusal::Resources(cause) if case == 4 => {
                    assert!(
                        matches!(cause,DesignResourceLimitError::Count {location,..} if location.field=="numeric.extra")
                    );
                    assert_eq!((source.calls.get(), source.base.walks.get()), (0, 1));
                }
                DraftingProgramProfileHeaderRefusal::Source { request, cause } => {
                    assert_eq!(request.row_index(), if case == 3 { 2 } else { 1 });
                    assert!(core::ptr::eq(request.selected(), &selected));
                    assert_eq!(source.calls.get(), if case == 3 { 2 } else { 1 });
                    assert_eq!(source.base.walks.get(), 1 + source.calls.get());
                    match cause.refusal() {
                        DraftingProfileSourceRefusal::Mismatch(
                            DraftingProfileSourceIssue::SelectedPin(
                                SelectedProfilePinField::ProfileId,
                            ),
                        ) if case == 3 => {}
                        DraftingProfileSourceRefusal::Mismatch(
                            DraftingProfileSourceIssue::Root,
                        ) if case == 5 => {}
                        DraftingProfileSourceRefusal::Mismatch(
                            DraftingProfileSourceIssue::Snapshot,
                        ) if case == 6 => {}
                        DraftingProfileSourceRefusal::Source(SourceError::Missing) if case == 7 => {
                        }
                        _ => return Err(format!("wrong original child cause: {cause}").into()),
                    }
                }
                _ => return Err(format!("wrong header stage refusal: {error}").into()),
            }
        }
    }
    Ok(())
}
