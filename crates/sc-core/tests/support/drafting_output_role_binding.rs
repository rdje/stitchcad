//! Genuine parent/catalogue/context identity and no accepted prefix after a later planned refusal.
use super::{limits, scope, EmptyHost, Frame, Source, SourceError, TestResult};
use core::cell::Cell;
use sc_core::{
    command::DesignRevision,
    construction::*,
    design::*,
    name::MachineToken,
    ontology::*,
    recipe::{
        FormulaExpressionDocument, FormulaRecipeDocument, FormulaSource, FormulaSourceLimits,
        FormulaSources,
    },
    value::CanonicalInputs,
};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
#[allow(clippy::expect_used)]
fn parameter(n: i64, d: i64) -> Param {
    Param::new(Rational::new(n, d).expect("fixture rational")).expect("fixture parameter")
}
#[path = "offset_sources.rs"]
#[allow(clippy::expect_used)]
mod fixture;
#[derive(Clone, Copy)]
enum Fault {
    None,
    Source,
    RoleKind,
    Input,
    Catalogue,
    Definition,
    Frame,
    Snapshot,
}
struct Snapshot<'a> {
    label: String,
    fields: [ActiveOffsetCandidate<'a>; 4],
}
struct Provider<'a> {
    base: Source<'a>,
    snapshot: DraftingSourceRecord<Snapshot<'a>>,
    foreign_snapshot: DraftingSourceRecord<Snapshot<'a>>,
    foreign_frame: DraftingSourceRecord<Frame>,
    fault: Fault,
    calls: Cell<usize>,
}
impl<'a> Provider<'a> {
    fn new(
        root: &'a DesignInputs<EmptyHost>,
        fields: [ActiveOffsetCandidate<'a>; 4],
        fault: Fault,
    ) -> Self {
        Self {
            base: Source::new(root),
            snapshot: DraftingSourceRecord::new(Snapshot {
                label: "planned immutable source".into(),
                fields,
            }),
            foreign_snapshot: DraftingSourceRecord::new(Snapshot {
                label: "planned immutable source".into(),
                fields,
            }),
            foreign_frame: DraftingSourceRecord::new(Frame {
                label: "private Cartesian frame é".into(),
                origin: [0, 0],
                axes: [[1, 0], [0, 1]],
            }),
            fault,
            calls: Cell::new(0),
        }
    }
}
fn fixed<T: Copy>(_: &T) {}
impl<'a> DraftingContextSource<EmptyHost> for Provider<'a> {
    type Frame = Frame;
    type Snapshot = Snapshot<'a>;
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
    fn snapshot(&self) -> &DraftingSourceRecord<Self::Snapshot> {
        self.base.observed(6);
        &self.snapshot
    }
    fn validate_current(&self) -> Result<(), Self::Error> {
        self.base.validate_current()
    }
    fn geometry(
        &self,
        _: DraftingGeometryReference,
    ) -> DraftingGeometrySourceResult<'_, Self::Frame, Self::Snapshot, Self::Geometry, Self::Error>
    {
        self.base.observed(7);
        Err(SourceError::Missing)
    }
    fn charge_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        self.base.charge_resources(sink)?; // Its original snapshot is also a retained cache.
        let at = |field, index| DesignResourceLocation {
            scope: DesignResourceScope::CommandState,
            owner: Some(self.base.root.id()),
            field,
            index: Some(index),
        };
        sink.charge_text(
            &self.foreign_frame.data().label,
            at("planned.foreign_frame", 0),
        )?;
        for (snapshot_index, snapshot) in [&self.snapshot, &self.foreign_snapshot]
            .into_iter()
            .enumerate()
        {
            sink.charge_text(
                &snapshot.data().label,
                at("planned.snapshot.label", snapshot_index),
            )?;
            for (index, field) in snapshot.data().fields.iter().copied().enumerate() {
                let original_index = snapshot_index * 4 + index;
                let parent = field.presence().admitted();
                sink.charge_entries(
                    parent.manifest().fragments().len(),
                    at("planned.offset.fragments", original_index),
                )?;
                sink.charge_entries(
                    field.presence().outputs().len(),
                    at("planned.offset.associations", original_index),
                )?;
                // Original input and allowance fields are fixed;no concealed dynamic definition.
                let SeamAllowanceDefinition {
                    id,
                    edge,
                    width,
                    corner,
                    inclusion,
                } = parent.bound_input().allowance().definition();
                fixed(id);
                fixed(edge);
                fixed(width);
                fixed(corner);
                fixed(inclusion);
                DraftingPlannedOutput::Offset(field)
                    .name_source()
                    .charge_resources(sink, original_index)?;
            }
        }
        Ok(())
    }
}
impl DraftingPlannedOutputProvider<EmptyHost> for Provider<'_> {
    fn planned_output<'v, 'n, 'i>(
        &'v self,
        request: DraftingOutputPosition<'v, 'n, 'i>,
    ) -> DraftingPlannedOutputSourceResult<'v, Self::Frame, Self::Snapshot, Self::Error> {
        self.base.observed(8);
        self.calls.set(self.calls.get() + 1);
        if !request.requires_role_binding() {
            return Err(SourceError::Rejected);
        }
        let fault = if request.output_index() == 2 {
            self.fault
        } else {
            Fault::None
        };
        if matches!(fault, Fault::Source) {
            return Err(SourceError::Rejected);
        }
        let index = match fault {
            Fault::Input => 1,
            Fault::Catalogue => 2,
            Fault::Definition => 3,
            _ => 0,
        };
        let field = self
            .snapshot
            .data()
            .fields
            .get(index)
            .copied()
            .ok_or(SourceError::Missing)?;
        let frame = if matches!(
            fault,
            Fault::Input | Fault::Catalogue | Fault::Definition | Fault::Frame
        ) {
            &self.foreign_frame
        } else {
            &self.base.frame
        };
        let snapshot = if matches!(fault, Fault::Frame | Fault::Snapshot) {
            &self.foreign_snapshot
        } else {
            &self.snapshot
        };
        Ok(DraftingPlannedOutputSource {
            output: DraftingPlannedOutput::Offset(field),
            frame,
            snapshot,
        })
    }
}
impl DraftingInitialInputProvider<EmptyHost> for Provider<'_> {
    type Metadata = ();
    fn initial_input<'a>(
        &'a self,
        _: usize,
        _: sc_core::recipe::FormulaCanonicalInput<'a>,
    ) -> DraftingInitialInputSourceResult<'a, Self::Metadata, Self::Snapshot, Self::Error> {
        Err(SourceError::Missing)
    }
}
impl DraftingGeometryStepProvider<EmptyHost> for Provider<'_> {
    fn planned_geometry<'v, 'i>(
        &'v self,
        request: DraftingGeometryStepRequest<'v, 'i>,
    ) -> DraftingPlannedOutputSourceResult<'v, Self::Frame, Self::Snapshot, Self::Error> {
        self.calls.set(self.calls.get() + 1);
        let field = self
            .snapshot
            .data()
            .fields
            .first()
            .copied()
            .ok_or(SourceError::Missing)?;
        let output = DraftingPlannedOutput::Offset(field);
        if !request.requires_role_binding()
            || request.reference() != output.reference()
            || !matches!((request.input(),output.input()),(DraftingOperationInput::OffsetEdge(a),DraftingOperationInput::OffsetEdge(b)) if std::ptr::eq(a,b))
        {
            return Err(SourceError::Missing);
        }
        if matches!(self.fault, Fault::Source) {
            return Err(SourceError::Rejected);
        }
        Ok(DraftingPlannedOutputSource {
            output,
            frame: &self.base.frame,
            snapshot: &self.snapshot,
        })
    }
}
fn root(fixture: fixture::Fixture) -> TestResult<DesignInputs<EmptyHost>> {
    let canonical = CanonicalInputs::new(vec![], vec![])?;
    let parameters = DesignParameters::new(vec![], &canonical)?;
    let materials = MaterialInventory::new(vec![], &canonical)?;
    let sources = DesignMetadata::new(EmptyHost, &canonical)?;
    let pieces = vec![fixture.piece];
    let cut_plan = CutPlan::new(
        vec![CutCopyDefinition {
            id: id(10001),
            piece: id(100),
            orientation: CopyOrientation::Authored,
        }],
        &pieces,
    )?;
    let structure = DesignStructure::new(DesignStructureDefinition {
        id: id(10000),
        revision: DesignRevision::from_counter(7),
        inputs: canonical,
        pieces: PieceInventory::new(pieces)?,
        cut_plan,
        topology: fixture.ledger,
    })?;
    Ok(DesignInputs::new(DesignInputsDefinition {
        structure,
        parameters,
        materials,
        sources,
        recipe: FormulaRecipeDocument::new("".into())?,
        formula_sources: FormulaSources::new(
            vec![FormulaSource::new(
                id(70),
                FormulaExpressionDocument::new("2 mm".into())?,
            )],
            FormulaSourceLimits::new(1, 100, 100),
        )?,
        references: DesignSourceReferences {
            measurement_table: None,
            ease_set: None,
            size_set: None,
        },
    })?)
}
#[test]
fn complete_role_sources_keep_actual_owners_records_and_first_later_cause() -> TestResult {
    let f = fixture::fixture();
    let allowance = f.allowance();
    let input = f.input();
    let foreign_input = f.input();
    let bound = input.bind_allowance(&allowance)?;
    let foreign_bound = foreign_input.bind_allowance(&allowance)?;
    let interval = OffsetInterval::new(parameter(1, 4), parameter(3, 4))?;
    let manifest = |tag| {
        OffsetCandidateManifest::new(
            input.operation(),
            *input.source(),
            vec![OffsetCandidateFragment::new(LocalTag::new(tag), interval)],
        )
    };
    let raw = OffsetRoleCatalog::new(
        input.operation(),
        vec![
            OffsetRoleDefinition::new(LocalTag::new(9), MachineToken::new("main")?),
            OffsetRoleDefinition::new(LocalTag::new(7), MachineToken::new("other")?),
        ],
        vec![LocalTag::new(9), LocalTag::new(7)],
    );
    let catalog = raw
        .admit(OffsetRoleCatalogLimits::new(2, 2, 9))
        .map_err(|e| e.to_string())?;
    let foreign_catalog = raw
        .admit(OffsetRoleCatalogLimits::new(2, 2, 9))
        .map_err(|e| e.to_string())?;
    let main = manifest(9);
    let other = manifest(7);
    let admitted = main
        .admit(&bound, OffsetCandidateLimits::new(1))
        .map_err(|e| e.to_string())?;
    let foreign_admitted = main
        .admit(&foreign_bound, OffsetCandidateLimits::new(1))
        .map_err(|e| e.to_string())?;
    let other_admitted = other
        .admit(&bound, OffsetCandidateLimits::new(1))
        .map_err(|e| e.to_string())?;
    let limit = OffsetRolePresenceLimits::new(1, 2, 2);
    let presence = admitted
        .bind_active_roles(&catalog, limit)
        .map_err(|e| e.to_string())?;
    let wrong_input = foreign_admitted
        .bind_active_roles(&catalog, limit)
        .map_err(|e| e.to_string())?;
    let wrong_catalog = admitted
        .bind_active_roles(&foreign_catalog, limit)
        .map_err(|e| e.to_string())?;
    let wrong_definition = other_admitted
        .bind_active_roles(&catalog, limit)
        .map_err(|e| e.to_string())?;
    let fields = [
        presence.outputs().next().ok_or("main field")?,
        wrong_input.outputs().next().ok_or("foreign input")?,
        wrong_catalog.outputs().next().ok_or("foreign catalogue")?,
        wrong_definition
            .outputs()
            .next()
            .ok_or("other definition")?,
    ];
    let root = root(f)?;
    let point_raw = PointOutputRoleCatalog::new(
        input.operation(),
        vec![PointOutputRoleDefinition::new(
            LocalTag::new(9),
            MachineToken::new("point")?,
        )],
        vec![LocalTag::new(9)],
    );
    let point_catalog = point_raw
        .admit(PointOutputRoleCatalogLimits::new(1, 1, 5))
        .map_err(|e| e.to_string())?;
    let point_role = point_catalog
        .active_role(LocalTag::new(9))
        .ok_or("distinct Point role domain")?;
    let line = LineConstruction::new(
        id(410),
        PointRef::new(id(411), LocalTag::FIRST),
        PointRef::new(id(412), LocalTag::FIRST),
    )?;
    for (fault, expected) in [
        (Fault::None, None),
        (Fault::Source, None),
        (
            Fault::RoleKind,
            Some(DraftingOutputRoleBindingIssue::RoleKind),
        ),
        (Fault::Input, Some(DraftingOutputRoleBindingIssue::Input)),
        (
            Fault::Catalogue,
            Some(DraftingOutputRoleBindingIssue::Catalogue),
        ),
        (
            Fault::Definition,
            Some(DraftingOutputRoleBindingIssue::Definition),
        ),
        (Fault::Frame, Some(DraftingOutputRoleBindingIssue::Frame)),
        (
            Fault::Snapshot,
            Some(DraftingOutputRoleBindingIssue::Snapshot),
        ),
    ] {
        let source = Provider::new(&root, fields, fault);
        let context = scope(&root)?
            .bind_drafting_context(&source, limits(100000, 100000, 100000)?)
            .map_err(|e| e.to_string())?;
        let inputs = DraftingProgramInputs::new(
            context,
            vec![
                DraftingProgramStep::Drafting(DraftingOperationInput::OffsetEdge(&input)),
                DraftingProgramStep::Drafting(DraftingOperationInput::DrawLine(&line)),
            ],
        );
        let raw_policy =
            DraftingProgramLimits::new(limits(100000, 100000, 100000)?, 100, 1000, 10000);
        let program = inputs
            .admit(&raw_policy)
            .map_err(|e| e.to_string())?
            .normalized()
            .map_err(|e| e.to_string())?;
        let role = catalog
            .active_role(LocalTag::new(9))
            .ok_or("original named role")?;
        let names = DraftingProgramNameInputs::new(
            &program,
            vec![],
            vec![
                DraftingOutputName::new(
                    1,
                    MachineToken::new("first")?,
                    DraftingGeometryNameSource::EdgeRole(role),
                ),
                DraftingOutputName::new(
                    2,
                    MachineToken::new("fixed")?,
                    DraftingGeometryNameSource::Line(&line),
                ),
                DraftingOutputName::new(
                    1,
                    MachineToken::new("second")?,
                    if matches!(fault, Fault::RoleKind) {
                        DraftingGeometryNameSource::PointRole(point_role)
                    } else {
                        DraftingGeometryNameSource::EdgeRole(role)
                    },
                ),
            ],
        );
        let policy = DraftingProgramNameLimits::new(limits(100000, 100000, 100000)?, 64, 128);
        let work = names.admit_name_work(&policy).map_err(|e| e.to_string())?;
        let positions = work.bind_output_positions().map_err(|e| e.to_string())?;
        source.base.trace.set(0);
        match positions.bind_role_sources() {
            Ok(bound) => {
                assert!(matches!(fault, Fault::None));
                assert!(std::ptr::eq(bound.positions(), &positions));
                let [Some(first), None, Some(second)] = bound.sources() else {
                    return Err("whole original source order".into());
                };
                assert!(
                    first.frame.is_same_record(context.frame())
                        && second.snapshot.is_same_record(context.snapshot())
                );
                assert!(
                    matches!(first.output.input(),DraftingOperationInput::OffsetEdge(p) if std::ptr::eq(p,&input))
                );
                assert!(!format!("{bound:?}").contains("first"));
            }
            Err(error) => {
                assert!(std::ptr::eq(error.positions(), &positions));
                match error.refusal() {
                    DraftingOutputRoleRefusal::Source { index, cause } => {
                        assert!(matches!(fault, Fault::Source));
                        assert_eq!(*index, 2);
                        assert_eq!(cause.as_ref(), &SourceError::Rejected);
                        assert!(std::error::Error::source(&error).is_some());
                    }
                    DraftingOutputRoleRefusal::Mismatch(mismatch) => {
                        assert_eq!(mismatch.output_index(), 2);
                        assert_eq!(Some(mismatch.issue()), expected);
                        let original = mismatch.returned();
                        assert!(std::ptr::eq(
                            original.snapshot,
                            if matches!(fault, Fault::Frame | Fault::Snapshot) {
                                &source.foreign_snapshot
                            } else {
                                &source.snapshot
                            }
                        ));
                    }
                    DraftingOutputRoleRefusal::Allocation(_) => {
                        return Err("unexpected fixture allocation failure".into())
                    }
                }
                assert!(!format!("{error:?}").contains("second"));
            }
        }
        assert_eq!(source.calls.get(), 2);
        assert_eq!(source.base.trace.get(), 88);
        let preceding =
            context.scope().demand().collection_entries() + context.demand().collection_entries();
        let short =
            DraftingProgramNameLimits::new(limits(preceding - 1, usize::MAX, u64::MAX)?, 0, 0);
        let error = names
            .admit_name_work(&short)
            .expect_err("complete provider model fields before zero work quotas");
        assert!(matches!(
            error.refusal(),
            DraftingProgramNameWorkRefusal::Resources(DesignResourceLimitError::Count {
                location: DesignResourceLocation {
                    field: "drafting.names.edge_roles.active_roles",
                    index: Some(7),
                    ..
                },
                ..
            })
        ));
        assert_eq!(source.calls.get(), 2);
        if matches!(fault, Fault::None | Fault::Source) {
            // Exercise the raw source facet, not the future geometry admission/binding controller.
            let consumer = OffsetConstruction::new(
                id(20000),
                DirectedRange {
                    range: EdgeRange::whole(fields.first().ok_or("actual field")?.logical_output()),
                    direction: Direction::Original,
                },
                OffsetSide::Right,
                id(20001),
            )?;
            let raw = DraftingProgramInputs::new(
                context,
                vec![
                    DraftingProgramStep::Drafting(DraftingOperationInput::OffsetEdge(&input)),
                    DraftingProgramStep::Drafting(DraftingOperationInput::OffsetEdge(&consumer)),
                ],
            );
            let program = raw
                .admit(&raw_policy)
                .map_err(|e| e.to_string())?
                .normalized()
                .map_err(|e| e.to_string())?;
            let names = DraftingProgramNameInputs::new(&program, vec![], vec![]);
            let work = names.admit_name_work(&policy).map_err(|e| e.to_string())?;
            let positions = work.bind_output_positions().map_err(|e| e.to_string())?;
            let roles = positions.bind_role_sources().map_err(|e| e.to_string())?;
            let initial = roles.bind_initial_sources().map_err(|e| e.to_string())?;
            let namespace = initial
                .bind_initial_namespace()
                .map_err(|e| e.to_string())?;
            let sources = namespace
                .check_root_sources(sc_core::recipe::FormulaSourceCheckLimits::new(
                    1, 100, 0, 1000,
                ))
                .map_err(|e| e.to_string())?;
            let dependencies = sources
                .proof()
                .prepare_dependencies(sc_core::recipe::FormulaSourceDependencyLimits::new(1, 0, 2))
                .map_err(|e| e.to_string())?;
            let graph = dependencies
                .admit_acyclic(sc_core::recipe::FormulaSourceCycleLimits::new(1, 2))
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
            source.calls.set(0);
            source.base.trace.set(0);
            let request = canonical
                .geometry_requests()
                .find(|r| r.occurrence().position() == Some(2))
                .ok_or("unnamed consumer")?;
            let DraftingProgramGeometryTarget::Earlier(request) =
                request.target().map_err(|e| format!("{e:?}"))?
            else {
                return Err("actual earlier source".into());
            };
            assert_eq!(request.step().position(), 1);
            assert!(request.requires_role_binding());
            assert_eq!(source.calls.get(), 0);
            match source.planned_geometry(request) {
                Ok(response) => {
                    assert!(matches!(fault, Fault::None));
                    assert_eq!(response.output.reference(), request.reference());
                    assert!(
                        matches!(response.output.input(),DraftingOperationInput::OffsetEdge(actual) if std::ptr::eq(actual,&input))
                    );
                    assert!(response.frame.is_same_record(context.frame()));
                    assert!(response.snapshot.is_same_record(context.snapshot()));
                    let copied = response;
                    assert!(copied.snapshot.is_same_record(response.snapshot));
                }
                Err(error) => {
                    assert!(matches!(fault, Fault::Source));
                    assert_eq!(error, SourceError::Rejected);
                }
            }
            assert_eq!(source.calls.get(), 1);
        }
    }
    Ok(())
}
