//! Complete original component association with honest captured contexts and exact native inputs.
use super::{
    garment_source::{garment, garment_with_piece},
    limits, scope, EmptyHost, Frame, Snapshot, TestResult,
};
use sc_core::{
    command::DesignRevision, construction::*, design::*, ontology::*, recipe::*,
    value::CanonicalInputs,
};
use std::{
    cell::{Cell, RefCell},
    error::Error,
    fmt,
};
#[path = "seam_edit_sources.rs"]
#[allow(clippy::expect_used)]
mod seams;
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn target(piece: EntityId) -> PocketPieceRef {
    PocketPieceRef {
        copy: id(10002),
        piece,
    }
}
fn frame() -> DraftingSourceRecord<Frame> {
    DraftingSourceRecord::new(Frame {
        label: "original component frame é".into(),
        origin: [0, 0],
        axes: [[1, 0], [0, 1]],
    })
}
#[derive(Clone, Copy)]
enum Fault {
    None,
    Piece,
    Frame,
    Snapshot,
}
#[derive(Debug)]
enum Cause {
    Missing(PocketPieceRef),
    Geometry,
}
impl fmt::Display for Cause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("original component source cause")
    }
}
impl Error for Cause {}
struct Recipe<'a> {
    program: &'a DraftingProgramInputs<'a, EmptyHost, Provider<'a>>,
    creator: DraftingComponentCreator<'a>,
    owner: &'a DesignGarment<EmptyHost>,
    alternate_plan: Option<&'a CutPlan>,
}
struct Provider<'a> {
    garment: &'a DesignGarment<EmptyHost>,
    frame: &'a DraftingSourceRecord<Frame>,
    snapshot: DraftingSourceRecord<Snapshot>,
    alternate_frame: DraftingSourceRecord<Frame>,
    alternate_snapshot: DraftingSourceRecord<Snapshot>,
    alternative: Option<&'a DesignGarment<EmptyHost>>,
    imported: DraftingSourceRecord<Vec<u8>>,
    revision: DesignRevision,
    unused: DesignRevision,
    recipe: Option<Recipe<'a>>,
    cache: RefCell<Vec<u8>>,
    fault: Fault,
    ignore: bool,
    missing: bool,
    charges: Cell<usize>,
    queries: Cell<usize>,
    values: Cell<usize>,
}
impl<'a> Provider<'a> {
    fn new(garment: &'a DesignGarment<EmptyHost>, frame: &'a DraftingSourceRecord<Frame>) -> Self {
        Self {
            garment,
            frame,
            snapshot: DraftingSourceRecord::new(Snapshot {
                label: "captured snapshot".into(),
            }),
            alternate_frame: super::component_binding::frame(),
            alternate_snapshot: DraftingSourceRecord::new(Snapshot {
                label: "captured snapshot".into(),
            }),
            alternative: None,
            imported: DraftingSourceRecord::new(vec![7, 8, 9]),
            revision: DesignRevision::from_counter(7),
            unused: DesignRevision::from_counter(u128::MAX).successor(),
            recipe: None,
            cache: RefCell::new(vec![]),
            fault: Fault::None,
            ignore: false,
            missing: false,
            charges: Cell::new(0),
            queries: Cell::new(0),
            values: Cell::new(0),
        }
    }
    fn walk(&self, sink: &mut dyn DesignResourceSink) -> Result<(), DesignResourceLimitError> {
        let Self {
            garment,
            frame,
            snapshot,
            alternate_frame,
            alternate_snapshot,
            alternative,
            imported,
            revision,
            unused,
            recipe,
            cache,
            fault,
            ignore,
            missing,
            charges,
            queries,
            values,
        } = self;
        let _: &&DesignGarment<EmptyHost> = garment;
        let _: (&Fault, &bool, &bool) = (fault, ignore, missing);
        let _: (&Cell<usize>, &Cell<usize>, &Cell<usize>) = (charges, queries, values);
        for record in [*frame, alternate_frame] {
            let Frame {
                label,
                origin,
                axes,
            } = record.data();
            let _: (&[i64; 2], &[[i64; 2]; 2]) = (origin, axes);
            sink.charge_text(label, at("component.frame"))?;
        }
        for record in [snapshot, alternate_snapshot] {
            let Snapshot { label } = record.data();
            sink.charge_text(label, at("component.snapshot"))?;
        }
        sink.charge_entries(imported.data().len(), at("component.imported"))?;
        charge_design_revision_resources(revision, sink, at("component.revision"))?;
        charge_design_revision_resources(unused, sink, at("component.unused"))?;
        sink.charge_entries(cache.borrow().len(), at("component.cache"))?;
        if let Some(other) = alternative {
            charge_design_garment_resources(other, sink)?;
        }
        if let Some(Recipe {
            program,
            creator,
            owner,
            alternate_plan,
        }) = recipe
        {
            charge_design_garment_resources(owner, sink)?;
            let captured = program.context().source();
            if !core::ptr::eq(captured.garment, *owner) {
                charge_design_garment_resources(captured.garment, sink)?;
            }
            captured.charge_resources(sink)?; // A distinct earlier provider,whose recipe is empty.
            program.charge_record_resources(sink)?; // Never reenter the containing provider.
            creator.input().charge_resources(sink)?;
            if let Some(plan) = alternate_plan {
                charge_cut_plan_resources(plan, sink)?;
            }
            let catalog = match creator {
                DraftingComponentCreator::Mirror(d) => d.roles().catalog(),
                DraftingComponentCreator::Join(d) => d.roles().catalog(),
            };
            // These fixtures retain an actually empty admitted catalogue:all five retained
            // collections are empty,including its indexes. No prior workspace quota is charged.
            assert!(
                catalog.catalog().definitions().is_empty()
                    && catalog.catalog().active_tags().is_empty()
                    && catalog.active_roles().len() == 0
            );
            sink.charge_entries(0, at("component.empty_creator_catalog"))?;
        }
        Ok(())
    }
}
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
        self.garment.inputs()
    }
    fn frame(&self) -> &DraftingSourceRecord<Frame> {
        self.frame
    }
    fn snapshot(&self) -> &DraftingSourceRecord<Snapshot> {
        &self.snapshot
    }
    fn snapshot_revision(&self) -> &DesignRevision {
        &self.revision
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
        self.values.set(self.values.get() + 1);
        Ok(())
    }
    fn geometry(
        &self,
        _: DraftingGeometryReference,
    ) -> DraftingGeometrySourceResult<'_, Frame, Snapshot, [u8], Cause> {
        self.values.set(self.values.get() + 1);
        Err(Cause::Geometry)
    }
}
impl DraftingGarmentProvider<EmptyHost> for Provider<'_> {
    fn garment_source(&self) -> DraftingGarmentSource<'_, EmptyHost, Snapshot> {
        DraftingGarmentSource {
            garment: self.garment,
            snapshot: &self.snapshot,
        }
    }
}
impl DraftingComponentProvider<EmptyHost> for Provider<'_> {
    type Imported = Vec<u8>;
    fn component_source<'a>(
        &'a self,
        requested: PocketPieceRef,
    ) -> Result<DraftingComponentResponse<'a, EmptyHost, Self>, Cause> {
        self.queries.set(self.queries.get() + 1);
        if self.missing {
            return Err(Cause::Missing(requested));
        }
        let piece_owner = if matches!(self.fault, Fault::Piece) {
            self.alternative.ok_or(Cause::Missing(requested))?
        } else {
            self.garment
        };
        let piece = piece_owner
            .inputs()
            .structure()
            .pieces()
            .piece(requested.piece)
            .ok_or(Cause::Missing(requested))?;
        let origin = match &self.recipe {
            None => DraftingComponentOrigin::Imported(DraftingImportedComponentSource {
                piece,
                source: &self.imported,
            }),
            Some(Recipe {
                program, creator, ..
            }) => DraftingComponentOrigin::Recipe(DraftingComponentRecipeSource {
                program,
                piece,
                creator: *creator,
            }),
        };
        Ok(DraftingComponentSource {
            origin,
            frame: if matches!(self.fault, Fault::Piece | Fault::Frame) {
                &self.alternate_frame
            } else {
                self.frame
            },
            snapshot: if matches!(self.fault, Fault::None) {
                &self.snapshot
            } else {
                &self.alternate_snapshot
            },
        })
    }
}
fn check<'a>(
    provider: &'a Provider<'a>,
    piece: EntityId,
    expected: Result<Option<usize>, DraftingComponentSourceIssue>,
) -> TestResult {
    let content = limits(10000, 100000, 10000)?;
    let context = scope(provider.garment.inputs())?
        .bind_drafting_context(provider, content)
        .map_err(|e| e.to_string())?;
    let parent = context
        .admit_garment_source(content)
        .map_err(|e| e.to_string())?;
    let work = parent
        .prepare_component_binding(
            target(piece),
            DraftingComponentBindingLimits::new(content, 1000000),
        )
        .map_err(|e| e.to_string())?;
    let charges = provider.charges.get();
    let queries = provider.queries.get();
    let values = provider.values.get();
    match (expected, work.bind_source()) {
        (Ok(position), Ok(bound)) => {
            assert!(core::ptr::eq(bound.work(), &work));
            assert_eq!(bound.creator_position(), position);
            assert_eq!(bound.demand(), work.demand());
            assert_eq!(bound.shape(), work.shape());
            let raw = bound.source();
            assert!(core::ptr::eq(raw.origin.piece(), work.piece()));
            assert!(raw.frame.is_same_record(context.frame()));
            assert!(raw.snapshot.is_same_record(context.snapshot()));
            match raw.origin {
                DraftingComponentOrigin::Imported(imported) => {
                    assert!(imported.source.is_same_record(&provider.imported));
                    assert_eq!(imported.source.data().as_slice(), &[7, 8, 9]);
                }
                DraftingComponentOrigin::Recipe(recipe) => {
                    let actual = provider.recipe.as_ref().ok_or("actual recipe")?;
                    assert!(core::ptr::eq(recipe.program, actual.program));
                    assert!(recipe
                        .program
                        .context()
                        .snapshot()
                        .is_same_record(&actual.program.context().source().snapshot));
                    assert!(
                        !recipe
                            .program
                            .context()
                            .snapshot()
                            .is_same_record(context.snapshot()),
                        "captured original snapshot stays distinct"
                    );
                    assert!(core::ptr::eq(
                        recipe.creator.piece_id(),
                        actual.creator.piece_id()
                    ));
                }
            }
            let mut meter = DesignResourceMeter::new(content);
            bound.charge_resources(&mut meter)?;
            assert_eq!(meter.finish()?, bound.demand());
            assert_eq!(provider.charges.get(), charges + 2);
        }
        (Err(expected), Err(error)) => {
            assert!(core::ptr::eq(error.work(), &work));
            assert_eq!(error.demand(), Some(work.demand()));
            assert_eq!(error.shape(), Some(work.shape()));
            assert!(error.returned().is_some());
            assert!(
                matches!(error.refusal(),DraftingComponentSourceRefusal::Mismatch(actual) if *actual==expected)
            );
            assert_eq!(provider.charges.get(), charges + 1);
        }
        _ => return Err("unexpected complete component association result".into()),
    }
    assert_eq!(provider.queries.get(), queries + 1);
    assert_eq!(provider.values.get(), values);
    Ok(())
}
#[test]
fn original_import_and_exact_header_priority_keep_all_returned_witnesses() -> TestResult {
    let actual = garment(false)?;
    let other = garment(false)?;
    let frame = frame();
    for (fault, expected) in [
        (Fault::None, Ok(None)),
        (Fault::Piece, Err(DraftingComponentSourceIssue::Piece)),
        (Fault::Frame, Err(DraftingComponentSourceIssue::Frame)),
        (Fault::Snapshot, Err(DraftingComponentSourceIssue::Snapshot)),
    ] {
        let mut provider = Provider::new(&actual, &frame);
        provider.fault = fault;
        provider.alternative = Some(&other);
        check(&provider, id(100), expected)?;
    }
    Ok(())
}
#[test]
fn complete_fresh_work_refuses_before_query_and_preserves_original_provider_cause() -> TestResult {
    let actual = garment(false)?;
    let frame = frame();
    for ignore in [false, true] {
        for spare in [0, 1] {
            let mut provider = Provider::new(&actual, &frame);
            provider.ignore = ignore;
            let content = limits(10000, 100000, 10000)?;
            let context = scope(actual.inputs())?
                .bind_drafting_context(&provider, content)
                .map_err(|e| e.to_string())?;
            let parent = context
                .admit_garment_source(content)
                .map_err(|e| e.to_string())?;
            let d = parent.demand();
            let e = d.collection_entries();
            let required = (e + 8) * (e + 8);
            let work = parent
                .prepare_component_binding(
                    target(id(100)),
                    DraftingComponentBindingLimits::new(
                        limits(e + spare, d.text_bytes(), d.counter_bits())?,
                        required,
                    ),
                )
                .map_err(|e| e.to_string())?;
            provider.cache.borrow_mut().push(1); // Deliberate immutable-contract violation falsifies cached credit.
            let error = work.bind_source().err().ok_or("fresh refusal")?;
            assert!(error.demand().is_none() && error.returned().is_none());
            let DraftingComponentSourceRefusal::Work(cause) = error.refusal() else {
                return Err("fresh work cause".into());
            };
            if spare == 0 {
                assert!(matches!(
                    cause.refusal(),
                    DraftingComponentWorkRefusal::Resources(_)
                ));
            } else {
                assert!(
                    matches!(cause.refusal(),DraftingComponentWorkRefusal::MatchingInspections{required:actual,..} if *actual==(e+9)*(e+9))
                );
            }
            assert_eq!(provider.queries.get(), 0);
        }
    }
    let mut provider = Provider::new(&actual, &frame);
    provider.missing = true;
    let content = limits(10000, 100000, 10000)?;
    let context = scope(actual.inputs())?
        .bind_drafting_context(&provider, content)
        .map_err(|e| e.to_string())?;
    let parent = context
        .admit_garment_source(content)
        .map_err(|e| e.to_string())?;
    let work = parent
        .prepare_component_binding(
            target(id(100)),
            DraftingComponentBindingLimits::new(content, 1000000),
        )
        .map_err(|e| e.to_string())?;
    let error = work
        .bind_source()
        .err()
        .ok_or("original provider refusal")?;
    assert!(
        matches!(error.refusal(),DraftingComponentSourceRefusal::Source(Cause::Missing(actual)) if *actual==work.target())
    );
    assert!(Error::source(&error).is_some());
    assert!(error.returned().is_none());
    assert_eq!(provider.queries.get(), 1);
    Ok(())
}
#[test]
fn original_mirror_creator_requires_actual_root_input_and_unique_mixed_position() -> TestResult {
    let actual = garment(false)?;
    let old = garment_with_piece(false, id(101))?;
    let foreign = garment_with_piece(false, id(101))?;
    let frame = frame();
    let other_frame = super::component_binding::frame();
    let source_piece = old
        .inputs()
        .structure()
        .pieces()
        .piece(id(101))
        .ok_or("original Mirror source")?;
    let input = MirrorPieceCopy::new(
        id(10),
        id(101),
        MirrorAxis::ThroughPoints {
            start: PointRef::new(id(90), LocalTag::new(1)),
            end: PointRef::new(id(91), LocalTag::new(1)),
        },
    )?;
    let equal_input = input;
    let sl = MirrorPieceSourceLimits::new(0, 1);
    let source = input
        .bind_piece(source_piece, &sl)
        .map_err(|e| e.to_string())?;
    let cp = MirrorPieceCutPolicy::new(
        1,
        Mirroring::Single,
        MirrorPieceFoldIntent::NoFold,
        CuttingSide::Face,
    );
    let cut = source.bind_cut_policy(&cp).map_err(|e| e.to_string())?;
    let mp = MirrorPieceMetadataPolicy::new(
        MirrorPieceFieldIntent::PreserveSource,
        MirrorPieceFieldIntent::PreserveSource,
        MirrorPieceFieldIntent::PreserveSource,
    );
    let ml = MirrorPieceMetadataLimits::new(1000);
    let metadata = cut.bind_metadata(&mp, &ml).map_err(|e| e.to_string())?;
    let catalog = EdgeOutputRoleCatalog::new(id(10), vec![], vec![]);
    let cl = EdgeOutputRoleCatalogLimits::new(0, 0, 0);
    let admitted = catalog.admit(cl).map_err(|e| e.to_string())?;
    let roles = metadata
        .bind_edge_roles(&admitted, cl)
        .map_err(|e| e.to_string())?;
    for case in 0..6 {
        let captured = Provider::new(if case == 4 { &foreign } else { &old }, &frame);
        let context = scope(captured.garment.inputs())?
            .bind_drafting_context(&captured, limits(10000, 100000, 10000)?)
            .map_err(|e| e.to_string())?;
        let mut steps = vec![
            DraftingProgramStep::Scalar { ordinal: 0 },
            DraftingProgramStep::Drafting(DraftingOperationInput::MirrorPiece(if case == 2 {
                &equal_input
            } else {
                &input
            })),
            DraftingProgramStep::Semantic { ordinal: 999 },
        ];
        if case == 3 {
            steps.push(DraftingProgramStep::Drafting(
                DraftingOperationInput::MirrorPiece(&input),
            ));
        }
        let program = DraftingProgramInputs::new(context, steps);
        let result = id(if case == 1 { 999 } else { 100 });
        let declaration = roles
            .declare_piece_identity(&result)
            .map_err(|e| e.to_string())?;
        let mut provider = Provider::new(&actual, if case == 5 { &other_frame } else { &frame });
        provider.recipe = Some(Recipe {
            program: &program,
            creator: DraftingComponentCreator::Mirror(&declaration),
            owner: &old,
            alternate_plan: None,
        });
        let expected = match case {
            0 => Ok(Some(2)),
            1 => Err(DraftingComponentSourceIssue::CreatorPiece),
            2 => Err(DraftingComponentSourceIssue::MissingCreator),
            3 => Err(DraftingComponentSourceIssue::RepeatedCreator {
                first: 2,
                second: 4,
            }),
            4 => Err(DraftingComponentSourceIssue::RecipePiece { side: None }),
            _ => Err(DraftingComponentSourceIssue::RecipeFrame),
        };
        check(&provider, id(100), expected)?;
    }
    Ok(())
}
fn join_owner() -> TestResult<DesignGarment<EmptyHost>> {
    let seams::Fixture {
        ledger,
        pieces,
        plan,
        graph,
        marks,
    } = seams::fixture(false);
    let canonical = CanonicalInputs::new(vec![], vec![])?;
    let parameters = DesignParameters::new(vec![], &canonical)?;
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
        structure,
        parameters,
        materials,
        sources,
        semantic_recipe: SemanticRecipe::empty(),
        recipe: FormulaRecipeDocument::new("".into())?,
        formula_sources: FormulaSources::new(vec![], FormulaSourceLimits::new(0, 0, 0))?,
        references: DesignSourceReferences {
            measurement_table: None,
            ease_set: None,
            size_set: None,
        },
    })?;
    let sewing = DesignSewing::new(DesignSewingDefinition {
        inputs,
        graph,
        landmarks: marks,
    })
    .map_err(|e| e.to_string())?;
    Ok(DesignGarment::new(DesignGarmentDefinition {
        sewing,
        pockets: PocketInventory::new(vec![])?,
        notions: NotionPlacementInventory::new(vec![])?,
        closures: ClosureInventory::new(vec![])?,
        layers: LayerInventory::new(vec![], vec![], vec![])?,
        hems: HemInventory::new(vec![])?,
        constructions: ConstructionInventory::new(vec![], vec![], vec![], vec![], vec![], vec![])?,
        drafting: String::new(),
    })
    .map_err(|e| e.to_string())?)
}
#[test]
fn original_join_creator_requires_captured_inventory_cutplan_and_exact_native_member() -> TestResult
{
    let actual = garment_with_piece(false, id(5000))?;
    let old = join_owner()?;
    let foreign = join_owner()?;
    let frame = frame();
    let root = old.inputs().structure();
    let definitions = root
        .cut_plan()
        .copies()
        .iter()
        .map(|copy| CutCopyDefinition {
            id: copy.id(),
            piece: copy.piece(),
            orientation: copy.orientation(),
        })
        .collect();
    let other_plan = CutPlan::new(definitions, root.pieces().pieces())?;
    let input = JoinPiecesAtSeam::new(
        id(1000),
        seams::first_span(old.sewing().graph()).definition().id,
        id(100),
        id(101),
    )?;
    let equal_input = input;
    for case in 0..5 {
        let plan = if case == 2 {
            &other_plan
        } else {
            root.cut_plan()
        };
        let source = input
            .bind_source(old.sewing().graph(), plan, root.pieces().pieces())
            .map_err(|e| e.to_string())?;
        let il = JoinCopyInventoryLimits::new(2, 2, 8);
        let inventory = source
            .bind_copy_inventory(root.pieces(), &il)
            .map_err(|e| e.to_string())?;
        let requirement = JoinExactSpliceRequirement::new(JoinSplicePlacement::KeepOriginal);
        let splice = inventory.require_exact_splice(&requirement);
        let policy = JoinPieceMetadataPolicy::new(
            JoinPieceFieldIntent::FromSide(SeamSideId::A),
            JoinPieceFieldIntent::FromSide(SeamSideId::A),
            JoinPieceFieldIntent::FromSide(SeamSideId::A),
            JoinPieceFieldIntent::FromSide(SeamSideId::A),
        );
        let ml = JoinPieceMetadataLimits::new(1000);
        let metadata = splice
            .bind_metadata(&policy, &ml)
            .map_err(|e| e.to_string())?;
        let population = JoinCutPopulation::new(1, Mirroring::Single);
        let cut = metadata
            .bind_cut_population(&population)
            .map_err(|e| e.to_string())?;
        let sources = cut
            .admit_source_pieces(JoinSourcePieceLimits::new(0, 2))
            .map_err(|e| e.to_string())?;
        let banks = sources.bind_cut_banks().map_err(|e| e.to_string())?;
        let catalog = EdgeOutputRoleCatalog::new(id(1000), vec![], vec![]);
        let cl = EdgeOutputRoleCatalogLimits::new(0, 0, 0);
        let admitted = catalog.admit(cl).map_err(|e| e.to_string())?;
        let roles = banks
            .bind_edge_roles(&admitted, cl)
            .map_err(|e| e.to_string())?;
        let result = id(5000);
        let declaration = roles
            .declare_piece_identity(&result, JoinPieceIdentityLimits::new(1))
            .map_err(|e| e.to_string())?;
        let captured = Provider::new(if case == 1 { &foreign } else { &old }, &frame);
        let context = scope(captured.garment.inputs())?
            .bind_drafting_context(&captured, limits(10000, 100000, 10000)?)
            .map_err(|e| e.to_string())?;
        let mut steps = vec![
            DraftingProgramStep::Scalar { ordinal: 0 },
            DraftingProgramStep::Drafting(DraftingOperationInput::JoinPieces(if case == 3 {
                &equal_input
            } else {
                &input
            })),
            DraftingProgramStep::Semantic { ordinal: 999 },
        ];
        if case == 4 {
            steps.push(DraftingProgramStep::Drafting(
                DraftingOperationInput::JoinPieces(&input),
            ));
        }
        let program = DraftingProgramInputs::new(context, steps);
        let mut provider = Provider::new(&actual, &frame);
        provider.recipe = Some(Recipe {
            program: &program,
            creator: DraftingComponentCreator::Join(&declaration),
            owner: &old,
            alternate_plan: if case == 2 { Some(&other_plan) } else { None },
        });
        let expected = match case {
            0 => Ok(Some(2)),
            1 => Err(DraftingComponentSourceIssue::RecipeInventory),
            2 => Err(DraftingComponentSourceIssue::RecipeCutPlan),
            3 => Err(DraftingComponentSourceIssue::MissingCreator),
            _ => Err(DraftingComponentSourceIssue::RepeatedCreator {
                first: 2,
                second: 4,
            }),
        };
        check(&provider, id(5000), expected)?;
    }
    Ok(())
}
