//! Whole initial-source loans,actual owner mismatches and planned creators before live lookup.
use super::{
    limits, root_with_host, scope, EmptyHost, Frame, Snapshot, Source, SourceError, TestResult,
};
use core::cell::Cell;
use sc_core::{
    command::DesignRevision,
    construction::*,
    design::*,
    name::MachineToken,
    ontology::{EdgeRef, EntityId, LocalTag, PointRef},
    recipe::{
        FormulaBindingKind as K, FormulaCanonicalInput, FormulaCanonicalInputRefusal,
        FormulaDeclaration as D, FormulaDeclarationSource as DS, FormulaExpressionDocument,
        FormulaInitialDeclaration, FormulaInputOrigin, FormulaScalarInputOrigin,
    },
    value::*,
};
use std::error::Error;
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn root() -> TestResult<DesignInputs<EmptyHost>> {
    root_with_host(
        "let untouched:length=1 mm/0",
        vec![],
        EmptyHost,
        CanonicalInputs::new(
            vec![LengthDeclaration::new(LengthDeclarationDefinition {
                id: id(101),
                source: id(201),
                state: LengthState::Unknown {
                    observation: id(301),
                },
            })?],
            vec![ScalarInputDeclaration::new(
                ScalarInputDeclarationDefinition {
                    id: id(102),
                    source: id(202),
                    kind: ScalarInputKind::Count,
                    state: ScalarInputState::Unknown {
                        observation: id(302),
                    },
                },
            )?],
        )?,
        vec![],
        DesignSourceReferences {
            measurement_table: None,
            ease_set: None,
            size_set: None,
        },
    )
}
struct Metadata {
    name: MachineToken,
    id: EntityId,
    target: EntityId,
    length: bool,
}
impl Metadata {
    fn declaration<'a>(&'a self, root: &'a DesignInputs<EmptyHost>) -> TestResult<D<'a>> {
        Ok(if self.length {
            D::length_input(
                &self.name,
                FormulaInputOrigin::Profile,
                self.id,
                root.inputs().length(self.target).ok_or("actual length")?,
            )
        } else {
            D::input(
                &self.name,
                FormulaScalarInputOrigin::Material,
                self.id,
                self.target,
                K::Count,
            )
        })
    }
}
#[derive(Clone, Copy)]
enum Fault {
    None,
    Inventory,
    Name,
    Declaration,
    Snapshot,
    Provider,
    GeometrySnapshot,
}
struct Provider<'a> {
    base: Source<'a>,
    other: &'a DesignInputs<EmptyHost>,
    metadata: [DraftingSourceRecord<Metadata>; 2],
    other_name: MachineToken,
    other_snapshot: DraftingSourceRecord<Snapshot>,
    geometry: [DraftingGeometryReference; 4],
    fault: Fault,
    calls: Cell<u64>,
}
impl<'a> Provider<'a> {
    fn new(
        root: &'a DesignInputs<EmptyHost>,
        other: &'a DesignInputs<EmptyHost>,
        fault: Fault,
    ) -> TestResult<Self> {
        Ok(Self {
            base: Source::new(root),
            other,
            metadata: [
                DraftingSourceRecord::new(Metadata {
                    name: MachineToken::new("private_length")?,
                    id: id(50),
                    target: id(101),
                    length: true,
                }),
                DraftingSourceRecord::new(Metadata {
                    name: MachineToken::new("private_count")?,
                    id: id(51),
                    target: id(102),
                    length: false,
                }),
            ],
            other_name: MachineToken::new("private_count")?,
            other_snapshot: DraftingSourceRecord::new(Snapshot {
                label: "private empty snapshot".into(),
            }),
            geometry: [
                DraftingGeometryReference::Point(PointRef::new(id(80), LocalTag::FIRST)),
                DraftingGeometryReference::Edge(EdgeRef::new(id(81), LocalTag::FIRST)),
                DraftingGeometryReference::Point(PointRef::new(id(500), LocalTag::FIRST)),
                DraftingGeometryReference::Edge(EdgeRef::new(id(501), LocalTag::FIRST)),
            ],
            fault,
            calls: Cell::new(0),
        })
    }
    fn initial(&self, index: usize) -> TestResult<FormulaInitialDeclaration<'_>> {
        self.metadata
            .get(index)
            .ok_or("original metadata")?
            .data()
            .declaration(self.base.root)?
            .try_into()
            .map_err(|_| "initial declaration".into())
    }
}
impl DraftingContextSource<EmptyHost> for Provider<'_> {
    type Frame = Frame;
    type Snapshot = Snapshot;
    type Geometry = DraftingGeometryReference;
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
    fn charge_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        self.base.charge_resources(sink)?;
        charge_design_input_resources(self.other, sink)?;
        for (index, record) in self.metadata.iter().enumerate() {
            sink.charge_text(
                record.data().name.as_str(),
                DesignResourceLocation {
                    scope: DesignResourceScope::Inputs,
                    owner: Some(record.data().id),
                    field: "initial.metadata.name",
                    index: Some(index),
                },
            )?;
        }
        sink.charge_text(
            self.other_name.as_str(),
            self.base.location("initial.other_name"),
        )?;
        sink.charge_text(
            &self.other_snapshot.data().label,
            self.base.location("initial.other_snapshot"),
        )
    }
    fn geometry(
        &self,
        requested: DraftingGeometryReference,
    ) -> DraftingGeometrySourceResult<'_, Frame, Snapshot, Self::Geometry, SourceError> {
        self.calls.set(self.calls.get() * 10 + 2);
        let geometry = self
            .geometry
            .iter()
            .find(|r| **r == requested)
            .ok_or(SourceError::Missing)?;
        Ok(DraftingGeometrySource {
            reference: *geometry,
            frame: &self.base.frame,
            snapshot: if matches!(self.fault, Fault::GeometrySnapshot) {
                &self.other_snapshot
            } else {
                &self.base.snapshot
            },
            geometry,
        })
    }
}
impl DraftingPlannedOutputProvider<EmptyHost> for Provider<'_> {
    fn planned_output<'v, 'n, 'i>(
        &'v self,
        _: DraftingOutputPosition<'v, 'n, 'i>,
    ) -> DraftingPlannedOutputSourceResult<'v, Frame, Snapshot, SourceError> {
        self.calls.set(self.calls.get() * 10 + 3);
        Err(SourceError::Missing)
    }
}
impl DraftingInitialInputProvider<EmptyHost> for Provider<'_> {
    type Metadata = Metadata;
    fn initial_input<'a>(
        &'a self,
        index: usize,
        input: FormulaCanonicalInput<'a>,
    ) -> DraftingInitialInputSourceResult<'a, Metadata, Snapshot, SourceError> {
        self.calls.set(self.calls.get() * 10 + 1);
        if index == 1 && matches!(self.fault, Fault::Provider) {
            return Err(SourceError::Rejected);
        }
        let metadata_id = match input.metadata().source() {
            DS::Input { input, .. } | DS::LengthInput { input, .. } => input,
            _ => return Err(SourceError::Rejected),
        };
        let metadata = self
            .metadata
            .iter()
            .find(|r| r.data().id == metadata_id)
            .ok_or(SourceError::Missing)?;
        let root = if index == 1 && matches!(self.fault, Fault::Inventory) {
            self.other
        } else {
            self.base.root
        };
        let mut declaration = metadata
            .data()
            .declaration(root)
            .map_err(|_| SourceError::Missing)?;
        if index == 1 && matches!(self.fault, Fault::Name) {
            declaration = D::input(
                &self.other_name,
                FormulaScalarInputOrigin::Material,
                metadata_id,
                id(102),
                K::Count,
            );
        }
        if index == 1 && matches!(self.fault, Fault::Declaration) {
            declaration = D::input(
                &metadata.data().name,
                FormulaScalarInputOrigin::Profile,
                metadata_id,
                id(102),
                K::Count,
            );
        }
        Ok(DraftingInitialInputSource {
            input: FormulaCanonicalInput::resolve(declaration, root.inputs())
                .map_err(|_| SourceError::Rejected)?,
            metadata,
            snapshot: if index == 1 && matches!(self.fault, Fault::Snapshot) {
                &self.other_snapshot
            } else {
                &self.base.snapshot
            },
        })
    }
}
fn point(n: u128) -> PointRef {
    PointRef::new(id(n), LocalTag::FIRST)
}
fn initial_geometry(
    name: &MachineToken,
    reference: DraftingGeometryReference,
) -> TestResult<FormulaInitialDeclaration<'_>> {
    match reference {
        DraftingGeometryReference::Point(p) => D::point(name, p),
        DraftingGeometryReference::Edge(e) => D::edge(name, e),
    }
    .try_into()
    .map_err(|_| "initial geometry".into())
}
#[test]
fn initial_binding_keeps_complete_original_order_unknowns_and_opaque_source_loans() -> TestResult {
    let root = root()?;
    let other = root_with_host(
        "",
        vec![],
        EmptyHost,
        CanonicalInputs::new(vec![], vec![])?,
        vec![],
        DesignSourceReferences {
            measurement_table: None,
            ease_set: None,
            size_set: None,
        },
    )?;
    let provider = Provider::new(&root, &other, Fault::None)?;
    let context = scope(&root)?
        .bind_drafting_context(&provider, limits(10000, 10000, 10000)?)
        .map_err(|e| e.to_string())?;
    let created_point = PointConstruction::new(
        id(500),
        FormulaExpressionDocument::new("1 mm/0".into())?,
        FormulaExpressionDocument::new("1 mm".into())?,
    );
    let raw = DraftingProgramInputs::new(
        context,
        vec![
            DraftingProgramStep::Scalar { ordinal: 1 },
            DraftingProgramStep::Drafting(DraftingOperationInput::ConstructPoint(&created_point)),
        ],
    );
    let program_policy = DraftingProgramLimits::new(limits(10000, 10000, 10000)?, 100, 1000, 10000);
    let program = raw
        .admit(&program_policy)
        .map_err(|e| e.to_string())?
        .normalized()
        .map_err(|e| e.to_string())?;
    let names = [
        MachineToken::new("private_point")?,
        MachineToken::new("private_edge")?,
    ];
    let initials = vec![
        provider.initial(0)?,
        provider.initial(1)?,
        provider.initial(0)?,
        initial_geometry(&names[0], DraftingGeometryReference::Point(point(80)))?,
        initial_geometry(
            &names[1],
            DraftingGeometryReference::Edge(EdgeRef::new(id(81), LocalTag::FIRST)),
        )?,
    ];
    let names = DraftingProgramNameInputs::new(&program, initials, vec![]);
    let policy = DraftingProgramNameLimits::new(limits(10000, 10000, 10000)?, 64, 128);
    let work = names.admit_name_work(&policy).map_err(|e| e.to_string())?;
    let positions = work.bind_output_positions().map_err(|e| e.to_string())?;
    let roles = positions.bind_role_sources().map_err(|e| e.to_string())?;
    provider.calls.set(0);
    let bound = roles.bind_initial_sources().map_err(|e| e.to_string())?;
    assert!(core::ptr::eq(bound.roles(), &roles));
    assert_eq!(bound.sources().len(), 5);
    for (index, binding) in bound.sources().iter().enumerate() {
        match binding {
            DraftingInitialBinding::Input(source) => {
                let original = provider
                    .metadata
                    .get(if index == 1 { 1 } else { 0 })
                    .ok_or("original source")?;
                assert!(source.metadata.is_same_record(original));
                assert!(core::ptr::eq(source.input.inputs(), root.inputs()));
                assert!(source.snapshot.is_same_record(&provider.base.snapshot));
                match source.input.adapter() {
                    sc_core::recipe::FormulaCanonicalInputAdapter::Length(input) => {
                        assert!(matches!(input.state(), LengthState::Unknown { .. }))
                    }
                    sc_core::recipe::FormulaCanonicalInputAdapter::Scalar(input) => {
                        assert!(matches!(input.state(), ScalarInputState::Unknown { .. }))
                    }
                }
            }
            DraftingInitialBinding::Geometry(binding) => {
                assert_eq!(
                    index,
                    if matches!(binding.reference(), DraftingGeometryReference::Point(_)) {
                        3
                    } else {
                        4
                    }
                );
                assert!(core::ptr::eq(
                    binding.geometry(),
                    provider.geometry.get(index - 3).ok_or("actual geometry")?
                ));
            }
        }
    }
    assert_eq!(provider.calls.get(), 11122);
    assert!(!format!("{bound:?} {:?}", bound.sources()).contains("private"));
    let empty = DraftingProgramNameInputs::new(&program, vec![], vec![]);
    let work = empty.admit_name_work(&policy).map_err(|e| e.to_string())?;
    let positions = work.bind_output_positions().map_err(|e| e.to_string())?;
    let roles = positions.bind_role_sources().map_err(|e| e.to_string())?;
    provider.calls.set(0);
    assert!(roles
        .bind_initial_sources()
        .map_err(|e| e.to_string())?
        .sources()
        .is_empty());
    assert_eq!(provider.calls.get(), 0);
    Ok(())
}
#[test]
fn initial_binding_refuses_second_actual_source_mismatch_or_original_provider_cause() -> TestResult
{
    let root = crate::initial_binding::root()?;
    let other = crate::initial_binding::root()?;
    for (fault, issue) in [
        (
            Fault::Inventory,
            Some(DraftingInitialBindingIssue::Inventory),
        ),
        (Fault::Name, Some(DraftingInitialBindingIssue::Name)),
        (
            Fault::Declaration,
            Some(DraftingInitialBindingIssue::Declaration),
        ),
        (Fault::Snapshot, Some(DraftingInitialBindingIssue::Snapshot)),
        (Fault::Provider, None),
    ] {
        let provider = Provider::new(&root, &other, fault)?;
        let context = scope(&root)?
            .bind_drafting_context(&provider, limits(10000, 10000, 10000)?)
            .map_err(|e| e.to_string())?;
        let raw =
            DraftingProgramInputs::new(context, vec![DraftingProgramStep::Scalar { ordinal: 1 }]);
        let pp = DraftingProgramLimits::new(limits(10000, 10000, 10000)?, 100, 1000, 10000);
        let program = raw
            .admit(&pp)
            .map_err(|e| e.to_string())?
            .normalized()
            .map_err(|e| e.to_string())?;
        let names = DraftingProgramNameInputs::new(
            &program,
            vec![
                provider.initial(0)?,
                provider.initial(1)?,
                provider.initial(0)?,
            ],
            vec![],
        );
        let policy = DraftingProgramNameLimits::new(limits(10000, 10000, 10000)?, 64, 128);
        let work = names.admit_name_work(&policy).map_err(|e| e.to_string())?;
        let positions = work.bind_output_positions().map_err(|e| e.to_string())?;
        let roles = positions.bind_role_sources().map_err(|e| e.to_string())?;
        provider.calls.set(0);
        let error = roles
            .bind_initial_sources()
            .expect_err("second original failure;no prefix");
        assert!(core::ptr::eq(error.roles(), &roles));
        assert_eq!(provider.calls.get(), 11);
        assert!(!format!("{error:?}").contains("private"));
        match error.refusal() {
            DraftingInitialRefusal::Mismatch {
                index,
                source,
                issue: actual,
            } => {
                assert_eq!(*index, 1);
                assert_eq!(Some(*actual), issue);
                assert!(source
                    .metadata
                    .is_same_record(provider.metadata.get(1).ok_or("original metadata")?));
                if matches!(fault, Fault::Inventory) {
                    assert!(core::ptr::eq(source.input.inputs(), other.inputs()));
                }
                if matches!(fault, Fault::Snapshot) {
                    assert!(source.snapshot.is_same_record(&provider.other_snapshot));
                }
            }
            DraftingInitialRefusal::Source { index, cause } => {
                assert_eq!(*index, 1);
                assert_eq!(**cause, SourceError::Rejected);
                assert!(error
                    .source()
                    .is_some_and(|cause| cause.is::<SourceError>()));
            }
            other => return Err(format!("original response/cause expected:{other:?}").into()),
        }
    }
    Ok(())
}
#[test]
fn initial_binding_keeps_canonical_and_geometry_causes_and_rejects_all_program_creators_before_lookup(
) -> TestResult {
    let root = crate::initial_binding::root()?;
    let other = crate::initial_binding::root()?;
    for fault in [Fault::None, Fault::GeometrySnapshot] {
        let provider = Provider::new(&root, &other, fault)?;
        let context = scope(&root)?
            .bind_drafting_context(&provider, limits(10000, 10000, 10000)?)
            .map_err(|e| e.to_string())?;
        let p = PointConstruction::new(
            id(500),
            FormulaExpressionDocument::new("1 mm".into())?,
            FormulaExpressionDocument::new("1 mm".into())?,
        );
        let line = LineConstruction::new(id(501), point(80), point(82))?;
        let raw = DraftingProgramInputs::new(
            context,
            vec![
                DraftingProgramStep::Scalar { ordinal: 1 },
                DraftingProgramStep::Drafting(DraftingOperationInput::ConstructPoint(&p)),
                DraftingProgramStep::Drafting(DraftingOperationInput::DrawLine(&line)),
            ],
        );
        let pp = DraftingProgramLimits::new(limits(10000, 10000, 10000)?, 100, 1000, 10000);
        let program = raw
            .admit(&pp)
            .map_err(|e| e.to_string())?
            .normalized()
            .map_err(|e| e.to_string())?;
        let name = MachineToken::new("private_refusal")?;
        let cases = [
            D::input(
                &name,
                FormulaScalarInputOrigin::Profile,
                id(99),
                id(999),
                K::Count,
            ),
            D::length_input(
                &name,
                FormulaInputOrigin::Profile,
                id(99),
                other
                    .inputs()
                    .length(id(101))
                    .ok_or("foreign same-id length")?,
            ),
            D::point(&name, point(500)),
            D::edge(&name, EdgeRef::new(id(501), LocalTag::FIRST)),
            D::point(&name, point(999)),
            D::point(&name, point(80)),
        ];
        for (case, declaration) in cases.into_iter().enumerate() {
            if case == 5 && matches!(fault, Fault::None) {
                continue;
            }
            let names = DraftingProgramNameInputs::new(
                &program,
                vec![
                    provider.initial(1)?,
                    declaration.try_into().map_err(|_| "initial candidate")?,
                ],
                vec![],
            );
            let policy = DraftingProgramNameLimits::new(limits(10000, 10000, 10000)?, 64, 128);
            let work = names.admit_name_work(&policy).map_err(|e| e.to_string())?;
            let positions = work.bind_output_positions().map_err(|e| e.to_string())?;
            let roles = positions.bind_role_sources().map_err(|e| e.to_string())?;
            provider.calls.set(0);
            let error = roles
                .bind_initial_sources()
                .expect_err("original initial refusal");
            assert_eq!(provider.calls.get(), if case < 4 { 1 } else { 12 });
            assert!(core::ptr::eq(error.roles(), &roles));
            match error.refusal() {
                DraftingInitialRefusal::Canonical { index, cause } => {
                    assert_eq!(*index, 1);
                    assert!(core::ptr::eq(cause.inputs(), root.inputs()));
                    assert!(matches!(
                        (case, cause.refusal()),
                        (0, FormulaCanonicalInputRefusal::MissingScalar { .. })
                            | (1, FormulaCanonicalInputRefusal::LengthOwner { .. })
                    ));
                }
                DraftingInitialRefusal::PlannedCreator {
                    index,
                    creator,
                    position,
                } => {
                    assert_eq!(*index, 1);
                    assert_eq!(
                        (*creator, *position),
                        if case == 2 {
                            (id(500), 2)
                        } else {
                            (id(501), 3)
                        }
                    );
                }
                DraftingInitialRefusal::Geometry { index, cause } => {
                    assert_eq!(*index, 1);
                    match cause.as_ref() {
                        DraftingGeometryError::Source { cause, .. } => {
                            assert_eq!(**cause, SourceError::Missing)
                        }
                        DraftingGeometryError::Mismatch(mismatch) => {
                            assert_eq!(mismatch.issue(), DraftingGeometryBindingIssue::Snapshot);
                            assert!(mismatch
                                .returned()
                                .snapshot
                                .is_same_record(&provider.other_snapshot));
                        }
                    }
                }
                other => return Err(format!("original cause expected:{other:?}").into()),
            }
        }
    }
    Ok(())
}

#[test]
fn initial_namespace_keeps_bound_extras_and_refuses_repeated_or_reserved_names_without_a_prefix(
) -> TestResult {
    use sc_core::recipe::{FormulaKind, FormulaNamespaceError};
    let root = crate::initial_binding::root()?;
    let other = crate::initial_binding::root()?;
    let provider = Provider::new(&root, &other, Fault::None)?;
    let context = scope(&root)?
        .bind_drafting_context(&provider, limits(10000, 10000, 10000)?)
        .map_err(|e| e.to_string())?;
    let p = PointConstruction::new(
        id(500),
        FormulaExpressionDocument::new("1 mm".into())?,
        FormulaExpressionDocument::new("1 mm".into())?,
    );
    let raw = DraftingProgramInputs::new(
        context,
        vec![
            DraftingProgramStep::Scalar { ordinal: 1 },
            DraftingProgramStep::Drafting(DraftingOperationInput::ConstructPoint(&p)),
        ],
    );
    let pp = DraftingProgramLimits::new(limits(10000, 10000, 10000)?, 100, 1000, 10000);
    let program = raw
        .admit(&pp)
        .map_err(|e| e.to_string())?
        .normalized()
        .map_err(|e| e.to_string())?;
    let reserved = MachineToken::new("eps_geo")?;
    let geometry_name = MachineToken::new("existing_point")?;
    for mode in 0..3 {
        let mut extras = vec![provider.initial(0)?, provider.initial(1)?];
        extras.push(match mode {
            0 => initial_geometry(&geometry_name, DraftingGeometryReference::Point(point(80)))?,
            1 => provider.initial(0)?,
            _ => initial_geometry(&reserved, DraftingGeometryReference::Point(point(80)))?,
        });
        let names = DraftingProgramNameInputs::new(
            &program,
            extras,
            vec![DraftingOutputName::new(
                2,
                MachineToken::new("future_output")?,
                DraftingGeometryNameSource::Point(&p),
            )],
        );
        let policy = DraftingProgramNameLimits::new(limits(10000, 10000, 10000)?, 64, 128);
        let work = names.admit_name_work(&policy).map_err(|e| e.to_string())?;
        let positions = work.bind_output_positions().map_err(|e| e.to_string())?;
        let roles = positions.bind_role_sources().map_err(|e| e.to_string())?;
        let initial = roles.bind_initial_sources().map_err(|e| e.to_string())?;
        provider.calls.set(0);
        match initial.bind_initial_namespace() {
            Ok(namespace) => {
                assert_eq!(mode, 0);
                assert!(std::ptr::eq(namespace.inputs(), &initial));
                for (index, record) in provider.metadata.iter().enumerate() {
                    let actual = namespace
                        .visible_names()
                        .resolve(&record.data().name)
                        .map_err(|error| error.to_string())?;
                    assert!(std::ptr::eq(actual.name(), record.data().name.as_str()));
                    assert_eq!(
                        actual.kind(),
                        if index == 0 {
                            FormulaKind::Length
                        } else {
                            FormulaKind::Count
                        }
                    );
                }
                assert_eq!(
                    namespace
                        .visible_names()
                        .resolve(&geometry_name)
                        .map_err(|error| error.to_string())?
                        .kind(),
                    FormulaKind::Point
                );
                assert!(namespace
                    .visible_names()
                    .resolve(&MachineToken::new("untouched")?)
                    .is_err());
                assert!(namespace
                    .visible_names()
                    .resolve(&MachineToken::new("future_output")?)
                    .is_err());
            }
            Err(error) => {
                assert!(std::ptr::eq(error.inputs(), &initial));
                assert!(!format!("{error:?}").contains("private"));
                match error.cause() {
                    DesignFormulaError::Namespace(FormulaNamespaceError::AmbiguousName {
                        sources,
                    }) => {
                        assert_eq!(mode, 1);
                        let [first, second] = **sources;
                        assert!(std::ptr::eq(first.name(), second.name()));
                    }
                    DesignFormulaError::Namespace(FormulaNamespaceError::ReservedBinding {
                        attempted,
                        ..
                    }) => {
                        assert_eq!(mode, 2);
                        assert!(std::ptr::eq(attempted.name(), reserved.as_str()));
                    }
                    other => {
                        return Err(format!("original extra collision expected:{other:?}").into())
                    }
                }
            }
        }
        assert_eq!(provider.calls.get(), 0);
    }
    Ok(())
}
