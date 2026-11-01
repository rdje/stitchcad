//! Actual complete join source/bank parents for role/identity/output contracts.
use sc_core::{construction::*, ontology::*};
pub fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
#[path = "seam_edit_sources.rs"]
mod seams;
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum SpanFixture {
    Declared,
    RetainedWhole,
    RetainedTail,
}
#[derive(Clone, Copy)]
pub struct BankFixture {
    pub source_materials: Option<[EntityId; 2]>,
    pub result_material: Option<EntityId>,
    pub mirroring: Mirroring,
    pub span: SpanFixture,
    pub extra_piece: bool,
}
impl BankFixture {
    pub fn basic() -> Self {
        Self {
            source_materials: None,
            result_material: None,
            mirroring: Mirroring::Single,
            span: SpanFixture::Declared,
            extra_piece: false,
        }
    }
}
pub fn with_banks(run: impl FnOnce(BoundJoinCutBanks<'_>)) {
    with_custom_banks(BankFixture::basic(), run);
}
pub fn with_custom_banks(options: BankFixture, run: impl FnOnce(BoundJoinCutBanks<'_>)) {
    let f = seams::fixture(false);
    let mut pieces = f
        .pieces
        .iter()
        .enumerate()
        .map(|(index, piece)| {
            let mut d = piece.definition().clone();
            if let Some(material) = options
                .source_materials
                .and_then(|ids| ids.get(index).copied())
            {
                d.material = MaterialAssignment::Assigned(material);
            }
            Piece::new(d, &f.ledger).expect("actual original Piece")
        })
        .collect::<Vec<_>>();
    let mut definitions = f
        .plan
        .copies()
        .iter()
        .map(|c| *c.definition())
        .collect::<Vec<_>>();
    if options.extra_piece {
        let mut d = pieces.first().expect("source A").definition().clone();
        d.id = id(102);
        d.material = MaterialAssignment::Assigned(id(602));
        pieces.push(Piece::new(d, &f.ledger).expect("actual unrelated Piece"));
        definitions.push(CutCopyDefinition {
            id: id(202),
            piece: id(102),
            orientation: CopyOrientation::Authored,
        });
    }
    let inventory = PieceInventory::new(pieces).expect("whole original inventory");
    let plan = CutPlan::new(definitions, inventory.pieces()).expect("complete actual plan");
    let original = SewingGraph::new(
        SewingGraphDefinition {
            id: id(300),
            spans: vec![seams::first_span(&f.graph).definition().clone()],
        },
        &plan,
        inventory.pieces(),
        &f.marks,
        &f.ledger,
    )
    .expect("actual source graph");
    let mut definition = seams::first_span(&original).definition().clone();
    match options.span {
        SpanFixture::Declared => {}
        SpanFixture::RetainedWhole => {
            definition.id = id(302);
            definition.ease = seams::first_span(&original).retain_ease_source().into();
        }
        SpanFixture::RetainedTail => {
            let half = Param::new(Rational::new(1, 2).expect("half")).expect("unit interval");
            let [_, tail] = seams::first_span(&original)
                .retain_ease_source()
                .split_at(
                    EdgeAnchor {
                        edge: definition.a.range.edge(),
                        param: half,
                    },
                    EdgeAnchor {
                        edge: definition.b.range.edge(),
                        param: half,
                    },
                )
                .expect("actual retained tail");
            definition.id = id(302);
            definition.a = tail.side(SeamSideId::A);
            definition.b = tail.side(SeamSideId::B);
            definition.stops.clear();
            definition.ease = tail.into();
        }
    }
    let span = definition.id;
    let graph = SewingGraph::new(
        SewingGraphDefinition {
            id: id(300),
            spans: vec![definition],
        },
        &plan,
        inventory.pieces(),
        &f.marks,
        &f.ledger,
    )
    .expect("actual selected graph");
    let edit = JoinPiecesAtSeam::new(id(1000), span, id(100), id(101)).expect("join inputs");
    let source = edit
        .bind_source(&graph, &plan, inventory.pieces())
        .expect("actual source slice");
    let c = plan.copies().len();
    let i = inventory.pieces().len();
    let limits = JoinCopyInventoryLimits::new(c, i, i + 3 * c);
    let copies = source
        .bind_copy_inventory(&inventory, &limits)
        .expect("complete original inventory");
    let request = JoinExactSpliceRequirement::new(JoinSplicePlacement::KeepOriginal);
    let splice = copies.require_exact_splice(&request);
    let material = options
        .result_material
        .map_or(JoinPieceFieldIntent::FromSide(SeamSideId::A), |id| {
            JoinPieceFieldIntent::Authored(MaterialAssignment::Assigned(id))
        });
    let policy = JoinPieceMetadataPolicy::new(
        material,
        JoinPieceFieldIntent::FromSide(SeamSideId::A),
        JoinPieceFieldIntent::FromSide(SeamSideId::A),
        JoinPieceFieldIntent::FromSide(SeamSideId::A),
    );
    let text_limits = JoinPieceMetadataLimits::new(28);
    let metadata = splice
        .bind_metadata(&policy, &text_limits)
        .expect("complete selected fields");
    let population = JoinCutPopulation::new(1, options.mirroring);
    let cut = metadata
        .bind_cut_population(&population)
        .expect("explicit cut population");
    let sources = cut
        .admit_source_pieces(JoinSourcePieceLimits::new(0, 2))
        .expect("whole original H/R");
    let banks = sources
        .bind_cut_banks()
        .expect("actual original A/B cut banks");
    run(banks);
}
