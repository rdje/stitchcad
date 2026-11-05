//! Actual populated garment ownership and fresh source accounting precede semantic queries.
use super::{limits, scope, EmptyHost, Frame, Snapshot, Source, SourceError, TestResult};
use sc_core::{
    command::DesignRevision, construction::*, design::*, ontology::*, recipe::*,
    value::CanonicalInputs,
};
use std::error::Error;
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
pub(super) fn garment(damaged: bool) -> TestResult<DesignGarment<EmptyHost>> {
    garment_with_piece(damaged, id(100))
}
pub(super) fn garment_with_piece(
    damaged: bool,
    identity: EntityId,
) -> TestResult<DesignGarment<EmptyHost>> {
    build(
        damaged,
        identity,
        vec![],
        PocketOpening::Profile(ProfileParameterRef::new(id(700))),
    )
}
pub(super) fn garment_with_closures(
    kinds: Vec<ClosureKind>,
) -> TestResult<DesignGarment<EmptyHost>> {
    build(
        false,
        id(100),
        kinds,
        PocketOpening::Profile(ProfileParameterRef::new(id(700))),
    )
}
pub(super) fn garment_with_opening(opening: PocketOpening) -> TestResult<DesignGarment<EmptyHost>> {
    build(false, id(100), vec![], opening)
}
fn build(
    damaged: bool,
    identity: EntityId,
    kinds: Vec<ClosureKind>,
    opening: PocketOpening,
) -> TestResult<DesignGarment<EmptyHost>> {
    build_with_semantics(
        damaged,
        identity,
        kinds,
        opening,
        CanonicalInputs::new(vec![], vec![])?,
        SemanticRecipe::empty(),
        vec![],
    )
}
pub(super) fn garment_with_semantics(
    canonical: CanonicalInputs,
    semantic_recipe: SemanticRecipe,
) -> TestResult<DesignGarment<EmptyHost>> {
    build_with_semantics(
        false,
        id(100),
        vec![],
        PocketOpening::Profile(ProfileParameterRef::new(id(700))),
        canonical,
        semantic_recipe,
        vec![],
    )
}
pub(super) fn garment_with_declarations(
    canonical: CanonicalInputs,
    parameters: Vec<DesignParameterDefinition>,
    kinds: Vec<ClosureKind>,
    opening: PocketOpening,
) -> TestResult<DesignGarment<EmptyHost>> {
    garment_with_semantic_declarations(
        canonical,
        parameters,
        kinds,
        opening,
        SemanticRecipe::empty(),
    )
}
pub(super) fn garment_with_semantic_declarations(
    canonical: CanonicalInputs,
    parameters: Vec<DesignParameterDefinition>,
    kinds: Vec<ClosureKind>,
    opening: PocketOpening,
    semantic_recipe: SemanticRecipe,
) -> TestResult<DesignGarment<EmptyHost>> {
    build_with_semantics(
        false,
        id(100),
        kinds,
        opening,
        canonical,
        semantic_recipe,
        parameters,
    )
}

fn build_with_semantics(
    damaged: bool,
    identity: EntityId,
    kinds: Vec<ClosureKind>,
    opening: PocketOpening,
    canonical: CanonicalInputs,
    semantic_recipe: SemanticRecipe,
    parameter_definitions: Vec<DesignParameterDefinition>,
) -> TestResult<DesignGarment<EmptyHost>> {
    build_with_semantic_data(
        damaged,
        identity,
        kinds,
        opening,
        SemanticFixture {
            canonical,
            semantic_recipe,
            parameter_definitions,
            formula_sources: FormulaSources::new(vec![], FormulaSourceLimits::new(0, 0, 0))?,
        },
    )
}
struct SemanticFixture {
    canonical: CanonicalInputs,
    semantic_recipe: SemanticRecipe,
    parameter_definitions: Vec<DesignParameterDefinition>,
    formula_sources: FormulaSources,
}
pub(super) fn garment_with_formula_declarations(
    canonical: CanonicalInputs,
    parameters: Vec<DesignParameterDefinition>,
    kinds: Vec<ClosureKind>,
    opening: PocketOpening,
    formula_sources: FormulaSources,
) -> TestResult<DesignGarment<EmptyHost>> {
    build_with_semantic_data(
        false,
        id(100),
        kinds,
        opening,
        SemanticFixture {
            canonical,
            semantic_recipe: SemanticRecipe::empty(),
            parameter_definitions: parameters,
            formula_sources,
        },
    )
}
pub(super) fn garment_with_declarations_and_pockets(
    canonical: CanonicalInputs,
    parameters: Vec<DesignParameterDefinition>,
    kinds: Vec<ClosureKind>,
    opening: PocketOpening,
    pocket_mode: u8,
) -> TestResult<DesignGarment<EmptyHost>> {
    build_with_pocket_mode(
        pocket_mode == 1,
        id(100),
        kinds,
        opening,
        SemanticFixture {
            canonical,
            semantic_recipe: SemanticRecipe::empty(),
            parameter_definitions: parameters,
            formula_sources: FormulaSources::new(vec![], FormulaSourceLimits::new(0, 0, 0))?,
        },
        pocket_mode,
    )
}
fn build_with_semantic_data(
    damaged: bool,
    identity: EntityId,
    kinds: Vec<ClosureKind>,
    opening: PocketOpening,
    data: SemanticFixture,
) -> TestResult<DesignGarment<EmptyHost>> {
    build_with_pocket_mode(damaged, identity, kinds, opening, data, 0)
}
fn build_with_pocket_mode(
    damaged: bool,
    identity: EntityId,
    kinds: Vec<ClosureKind>,
    opening: PocketOpening,
    data: SemanticFixture,
    pocket_mode: u8,
) -> TestResult<DesignGarment<EmptyHost>> {
    build_with_placement_mode(damaged, identity, kinds, opening, data, pocket_mode, 0)
}
fn build_with_placement_mode(
    damaged: bool,
    identity: EntityId,
    kinds: Vec<ClosureKind>,
    opening: PocketOpening,
    data: SemanticFixture,
    pocket_mode: u8,
    placement_mode: u8,
) -> TestResult<DesignGarment<EmptyHost>> {
    let SemanticFixture {
        canonical,
        semantic_recipe,
        parameter_definitions,
        formula_sources,
    } = data;
    let mut ledger = IdentityLedger::new();
    let mut ids = DeterministicIdGenerator::new();
    let edges = ledger.declare_edges(&mut ids, 1)?;
    let edge = *edges.first().ok_or("actual declared edge")?;
    let piece = Piece::new(
        PieceDefinition {
            id: identity,
            boundary: vec![DirectedEdge {
                edge,
                direction: Direction::Original,
            }],
            holes: vec![],
            construction_lines: vec![],
            quantity: 2,
            mirroring: Mirroring::Single,
            cut_on_fold: false,
            fold_edges: vec![],
            cutting_side: CuttingSide::Face,
            material: MaterialAssignment::Unresolved("actual material pending".into()),
            layer_index: 0,
            label: LabelText {
                name: "private garment Piece".into(),
                size: "M".into(),
                fabric: "woven".into(),
                colorway: "plain".into(),
            },
        },
        &ledger,
    )?;
    let pieces = vec![piece];
    let definitions = vec![
        CutCopyDefinition {
            id: id(10001),
            piece: identity,
            orientation: CopyOrientation::Authored,
        },
        CutCopyDefinition {
            id: id(10002),
            piece: identity,
            orientation: CopyOrientation::Authored,
        },
    ];
    let plan = CutPlan::new(definitions.clone(), &pieces)?;
    let pocket = Pocket::new(
        PocketDefinition {
            id: id(10003),
            served: PocketPieceRef {
                copy: id(10001),
                piece: identity,
            },
            position: EdgeAnchor {
                edge,
                param: Param::START,
            },
            orientation: DirectedRange {
                range: EdgeRange::whole(edge),
                direction: Direction::Original,
            },
            opening,
            components: vec![PocketPieceRef {
                copy: id(10002),
                piece: identity,
            }],
        },
        &plan,
        &pieces,
        &ledger,
    )?;
    let original_placement_plan = if placement_mode == 4 {
        Some(plan.clone())
    } else {
        None
    };
    let plan = if damaged {
        CutPlan::new(
            definitions
                .into_iter()
                .enumerate()
                .map(|(index, mut definition)| {
                    if index == 1 {
                        definition.id = id(10005);
                    }
                    definition
                })
                .collect(),
            &pieces,
        )?
    } else {
        plan
    };
    let parameters = DesignParameters::new(parameter_definitions, &canonical)?;
    let materials = MaterialInventory::new(vec![], &canonical)?;
    let sources = DesignMetadata::new(EmptyHost, &canonical)?;
    let structure = DesignStructure::new(DesignStructureDefinition {
        id: id(10000),
        revision: DesignRevision::from_counter(7),
        inputs: canonical,
        pieces: PieceInventory::new(pieces)?,
        cut_plan: plan,
        topology: ledger,
    })?;
    let inputs = DesignInputs::new(DesignInputsDefinition {
        semantic_recipe,
        structure,
        parameters,
        materials,
        sources,
        recipe: FormulaRecipeDocument::new("".into())?,
        formula_sources,
        references: DesignSourceReferences {
            measurement_table: None,
            ease_set: None,
            size_set: None,
        },
    })?;
    let structure = inputs.structure();
    let notion_plan = original_placement_plan
        .as_ref()
        .unwrap_or(structure.cut_plan());
    let mut placements = vec![];
    let mut closures = vec![];
    for (index, kind) in kinds.into_iter().enumerate() {
        let offset = u128::try_from(index)? * 10;
        let pair = [id(20000 + offset), id(20001 + offset)];
        for (copy, placement) in [
            id(10001),
            id(if damaged && placement_mode != 4 {
                10005
            } else {
                10002
            }),
        ]
        .into_iter()
        .zip(pair)
        {
            placements.push(NotionPlacement::new(
                NotionPlacementDefinition {
                    id: placement,
                    copy,
                    anchor: EdgeAnchor {
                        edge,
                        param: Param::START,
                    },
                    direction: DirectedRange {
                        range: EdgeRange::whole(edge),
                        direction: Direction::Original,
                    },
                },
                notion_plan,
                structure.pieces().pieces().first().ok_or("actual Piece")?,
                structure.topology(),
            )?);
        }
        let mut instances = vec![ClosureInstance {
            id: id(20003 + offset),
            first: pair[0],
            second: pair[1],
        }];
        if placement_mode == 3 {
            let extra = [id(21000 + offset), id(21001 + offset)];
            for (original, identity) in pair.into_iter().zip(extra) {
                let mut definition = *placements
                    .iter()
                    .find(|p| p.id() == original)
                    .ok_or("original notion")?
                    .definition();
                definition.id = identity;
                placements.push(NotionPlacement::new(
                    definition,
                    notion_plan,
                    structure.pieces().pieces().first().ok_or("actual Piece")?,
                    structure.topology(),
                )?);
            }
            instances.push(ClosureInstance {
                id: id(22000 + offset),
                first: extra[0],
                second: extra[1],
            });
        }
        closures.push(Closure::new(
            ClosureDefinition {
                id: id(20002 + offset),
                kind,
                instances,
            },
            &placements,
            notion_plan,
            structure.pieces().pieces(),
            structure.topology(),
        )?);
    }
    if placement_mode == 1 {
        placements.retain(|p| p.id() != id(20011));
    }
    if placement_mode == 2 {
        placements.reverse();
    }
    let graph = SewingGraph::new(
        SewingGraphDefinition {
            id: id(10004),
            spans: vec![],
        },
        structure.cut_plan(),
        structure.pieces().pieces(),
        &[],
        structure.topology(),
    )?;
    let sewing = DesignSewing::new(DesignSewingDefinition {
        inputs,
        graph,
        landmarks: vec![],
    })
    .map_err(|e| e.to_string())?;
    let pockets = PocketInventory::new(match pocket_mode {
        2 => vec![],
        3 => {
            let mut definition = pocket.definition().clone();
            definition.id = id(11003);
            vec![
                pocket,
                Pocket::new(
                    definition,
                    sewing.inputs().structure().cut_plan(),
                    sewing.inputs().structure().pieces().pieces(),
                    sewing.inputs().structure().topology(),
                )?,
            ]
        }
        _ => vec![pocket],
    })?;
    Ok(DesignGarment::new(DesignGarmentDefinition {
        sewing,
        pockets,
        notions: NotionPlacementInventory::new(placements)?,
        closures: ClosureInventory::new(closures)?,
        layers: LayerInventory::new(vec![], vec![], vec![])?,
        hems: HemInventory::new(vec![])?,
        constructions: ConstructionInventory::new(vec![], vec![], vec![], vec![], vec![], vec![])?,
    })
    .map_err(|e| e.to_string())?)
}
pub(super) struct Provider<'a> {
    pub(super) base: Source<'a>,
    garment: &'a DesignGarment<EmptyHost>,
    foreign_snapshot: DraftingSourceRecord<Snapshot>,
    wrong_snapshot: bool,
    ignore_refusal: bool,
    retained: Vec<u8>,
}
impl<'a> Provider<'a> {
    pub(super) fn new(
        root: &'a DesignInputs<EmptyHost>,
        garment: &'a DesignGarment<EmptyHost>,
        wrong_snapshot: bool,
        ignore_refusal: bool,
    ) -> Self {
        Self {
            base: Source::new(root),
            garment,
            foreign_snapshot: DraftingSourceRecord::new(Snapshot {
                label: "foreign private semantic snapshot".into(),
            }),
            wrong_snapshot,
            ignore_refusal,
            retained: vec![1, 2, 3, 4, 5],
        }
    }
    fn location(&self) -> DesignResourceLocation {
        self.base.location("garment.retained")
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
        self.base.frame()
    }
    fn snapshot(&self) -> &DraftingSourceRecord<Snapshot> {
        self.base.snapshot()
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
        self.base.charge_resources(sink)?;
        sink.charge_text(
            &self.foreign_snapshot.data().label,
            self.base.location("garment.foreign_snapshot"),
        )?;
        let result = sink.charge_entries(self.retained.len(), self.location());
        if self.ignore_refusal {
            let _ = result;
            let _ = sink.charge_entries(1, self.base.location("garment.later"));
            Ok(())
        } else {
            result
        }
    }
}
impl DraftingGarmentProvider<EmptyHost> for Provider<'_> {
    fn garment_source(&self) -> DraftingGarmentSource<'_, EmptyHost, Snapshot> {
        self.base.observed(8);
        DraftingGarmentSource {
            garment: self.garment,
            snapshot: if self.wrong_snapshot {
                &self.foreign_snapshot
            } else {
                &self.base.snapshot
            },
        }
    }
}
#[test]
fn actual_populated_garment_source_retains_original_root_snapshot_and_fresh_demand() -> TestResult {
    for damaged in [false, true] {
        let garment = garment(damaged)?;
        assert_eq!(garment.validate_current().is_err(), damaged);
        let provider = Provider::new(garment.inputs(), &garment, false, false);
        let content = limits(100000, 100000, 100000)?;
        let context = scope(garment.inputs())?
            .bind_drafting_context(&provider, content)
            .map_err(|e| e.to_string())?;
        let mut meter = DesignResourceMeter::new(content);
        charge_design_garment_resources(&garment, &mut meter)?;
        provider.charge_resources(&mut meter)?;
        let demand = meter.finish()?;
        provider.base.trace.set(0);
        let proof = context
            .admit_garment_source(content)
            .map_err(|e| e.to_string())?;
        assert_eq!(proof.limits(), content);
        assert_eq!(proof.demand(), demand);
        assert!(std::ptr::eq(
            proof.context().scope().root(),
            garment.inputs()
        ));
        let returned = proof.source();
        let copied = returned;
        assert!(std::ptr::eq(copied.garment, &garment));
        assert!(copied.snapshot.is_same_record(context.snapshot()));
        assert_eq!(copied.garment.pockets().pockets().len(), 1);
        assert_eq!(
            provider.base.trace.get(),
            82,
            "raw getter then fresh resource walk only"
        );
        assert!(!format!("{proof:?} {returned:?}").contains("private"));
    }
    Ok(())
}
#[test]
fn foreign_actual_garment_root_refuses_before_content_even_with_equal_ids_and_revision(
) -> TestResult {
    let actual = garment(false)?;
    let other = garment(false)?;
    assert_eq!(actual.inputs().id(), other.inputs().id());
    assert_eq!(actual.inputs().revision(), other.inputs().revision());
    let provider = Provider::new(actual.inputs(), &other, true, false);
    let context = scope(actual.inputs())?
        .bind_drafting_context(&provider, limits(100000, 100000, 100000)?)
        .map_err(|e| e.to_string())?;
    provider.base.trace.set(0);
    let selected = limits(1, 1, 1)?;
    let error = context
        .admit_garment_source(selected)
        .err()
        .ok_or("actual foreign owner")?;
    assert_eq!(error.issue(), DraftingGarmentSourceIssue::Root);
    assert_eq!(error.limits(), selected);
    assert!(std::ptr::eq(
        error.context().scope().root(),
        actual.inputs()
    ));
    assert!(std::ptr::eq(error.returned().garment, &other));
    assert!(error
        .returned()
        .snapshot
        .is_same_record(&provider.foreign_snapshot));
    assert_eq!(provider.base.trace.get(), 8);
    Ok(())
}
#[test]
fn complete_original_content_refuses_before_snapshot_and_preserves_ignored_sink_cause() -> TestResult
{
    let garment = garment(false)?;
    for ignore in [false, true] {
        let provider = Provider::new(garment.inputs(), &garment, true, ignore);
        let content = limits(100000, 100000, 100000)?;
        let context = scope(garment.inputs())?
            .bind_drafting_context(&provider, content)
            .map_err(|e| e.to_string())?;
        let mut meter = DesignResourceMeter::new(content);
        charge_design_garment_resources(&garment, &mut meter)?;
        provider.charge_resources(&mut meter)?;
        let demand = meter.finish()?;
        // The ignoring arm deliberately charges one additional retained entry after the first group.
        let short = limits(
            demand
                .collection_entries()
                .checked_sub(if ignore { 2 } else { 1 })
                .ok_or("nonempty retained content")?,
            100000,
            100000,
        )?;
        let mut meter = DesignResourceMeter::new(short);
        charge_design_garment_resources(&garment, &mut meter)?;
        let _ = provider.charge_resources(&mut meter);
        let original = meter
            .finish()
            .err()
            .ok_or("first retained group exceeds allowance")?;
        assert!(
            matches!(original,DesignResourceLimitError::Count {location,..} if location==provider.location())
        );
        provider.base.trace.set(0);
        let error = context
            .admit_garment_source(short)
            .err()
            .ok_or("complete content before snapshot")?;
        assert_eq!(
            error.issue(),
            DraftingGarmentSourceIssue::Resources(original)
        );
        assert!(error
            .source()
            .is_some_and(|e| e.is::<DesignResourceLimitError>()));
        assert_eq!(provider.base.trace.get(), 82);
        provider.base.trace.set(0);
        let error = context
            .admit_garment_source(content)
            .err()
            .ok_or("original snapshot record differs")?;
        assert_eq!(error.issue(), DraftingGarmentSourceIssue::Snapshot);
        assert!(error
            .returned()
            .snapshot
            .is_same_record(&provider.foreign_snapshot));
        assert_eq!(provider.base.trace.get(), 82);
        assert!(!format!("{error:?}").contains("private"));
    }
    Ok(())
}

pub(super) fn garment_with_placement_members(
    kinds: Vec<ClosureKind>,
    mode: u8,
) -> TestResult<DesignGarment<EmptyHost>> {
    build_with_placement_mode(
        false,
        id(100),
        kinds,
        PocketOpening::Profile(ProfileParameterRef::new(id(700))),
        SemanticFixture {
            canonical: CanonicalInputs::new(vec![], vec![])?,
            semantic_recipe: SemanticRecipe::empty(),
            parameter_definitions: vec![],
            formula_sources: FormulaSources::new(vec![], FormulaSourceLimits::new(0, 0, 0))?,
        },
        0,
        mode,
    )
}

pub(super) fn garment_with_placement_work(
    canonical: CanonicalInputs,
    semantic_recipe: SemanticRecipe,
    kinds: Vec<ClosureKind>,
    mode: u8,
) -> TestResult<DesignGarment<EmptyHost>> {
    build_with_placement_mode(
        mode == 4,
        id(100),
        kinds,
        PocketOpening::Declaration(id(90)),
        SemanticFixture {
            canonical,
            semantic_recipe,
            parameter_definitions: vec![],
            formula_sources: FormulaSources::new(vec![], FormulaSourceLimits::new(0, 0, 0))?,
        },
        2,
        mode,
    )
}
