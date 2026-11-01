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
    RetainedMiddle,
}
#[derive(Clone, Copy)]
pub struct BankFixture {
    pub source_materials: Option<[EntityId; 2]>,
    pub result_material: Option<EntityId>,
    pub mirroring: Mirroring,
    pub result_quantity: u32,
    pub span: SpanFixture,
    pub extra_piece: bool,
    pub additional_source_pair: bool,
    pub additional_authored_copies: bool,
    pub source_companion: bool,
    pub extra_occurrences: bool,
    pub bank_in_holes: bool,
    pub fold_on_other_boundary: bool,
    pub extra_boundary_edges: bool,
    pub shared_edge_fields: bool,
    pub seam_direction: SeamDirection,
}
impl BankFixture {
    pub fn basic() -> Self {
        Self {
            source_materials: None,
            result_material: None,
            mirroring: Mirroring::Single,
            result_quantity: 1,
            span: SpanFixture::Declared,
            extra_piece: false,
            additional_source_pair: false,
            additional_authored_copies: false,
            source_companion: false,
            extra_occurrences: false,
            bank_in_holes: false,
            fold_on_other_boundary: false,
            extra_boundary_edges: false,
            shared_edge_fields: false,
            seam_direction: SeamDirection::Opposite,
        }
    }
}
pub fn with_banks(run: impl FnOnce(BoundJoinCutBanks<'_>)) {
    with_custom_banks(BankFixture::basic(), run);
}
pub fn with_custom_banks(options: BankFixture, run: impl FnOnce(BoundJoinCutBanks<'_>)) {
    let mut f = seams::fixture(false);
    let holes = if options.extra_occurrences {
        f.ledger
            .declare_edges(&mut DeterministicIdGenerator::from_start(2), 2)
            .expect("actual additional hole edges")
    } else {
        vec![]
    };
    let extra = if options.extra_boundary_edges {
        f.ledger
            .declare_edges(&mut DeterministicIdGenerator::from_start(3), 4)
            .expect("actual additional boundary edges")
    } else {
        vec![]
    };
    let mut pieces = f
        .pieces
        .iter()
        .enumerate()
        .map(|(index, piece)| {
            let mut d = piece.definition().clone();
            if options.source_companion && index == 0 {
                assert!(
                    options.extra_piece
                        && !options.additional_source_pair
                        && !options.additional_authored_copies
                );
                d.mirroring = Mirroring::PairMember {
                    handedness: Handedness::Left,
                    companion: id(102),
                };
            }
            if options.additional_source_pair || options.additional_authored_copies {
                assert!(!(options.additional_source_pair && options.additional_authored_copies));
                d.quantity = 2;
                d.mirroring = if options.additional_authored_copies {
                    Mirroring::Single
                } else {
                    Mirroring::MirroredPairs
                };
            }
            if options.extra_boundary_edges {
                d.boundary.extend([
                    DirectedEdge {
                        edge: *extra.get(index * 2).expect("first extra field"),
                        direction: Direction::Reversed,
                    },
                    DirectedEdge {
                        edge: *extra.get(index * 2 + 1).expect("second extra field"),
                        direction: Direction::Original,
                    },
                ]);
            }
            if options.extra_occurrences {
                d.holes = vec![vec![DirectedEdge {
                    edge: *holes.get(index).expect("actual hole"),
                    direction: Direction::Reversed,
                }]];
                let edge = d.boundary.first().expect("original bank").edge;
                d.construction_lines = vec![
                    DirectedEdge {
                        edge,
                        direction: Direction::Original,
                    },
                    DirectedEdge {
                        edge,
                        direction: Direction::Reversed,
                    },
                ];
                d.cut_on_fold = true;
                d.fold_edges = vec![edge];
                if options.fold_on_other_boundary {
                    assert!(!options.bank_in_holes);
                    let outside = *d
                        .holes
                        .pop()
                        .expect("actual extra loop")
                        .first()
                        .expect("actual extra field");
                    d.boundary.push(outside);
                    d.fold_edges = vec![outside.edge];
                }
                if options.bank_in_holes {
                    let outside = *d.holes.first().expect("hole").first().expect("hole field");
                    let bank = *d.boundary.first().expect("bank field");
                    d.boundary = vec![outside];
                    d.holes = vec![vec![bank]];
                    d.fold_edges = vec![outside.edge];
                }
            }
            if let Some(material) = options
                .source_materials
                .and_then(|ids| ids.get(index).copied())
            {
                d.material = MaterialAssignment::Assigned(material);
            }
            Piece::new(d, &f.ledger).expect("actual original Piece")
        })
        .collect::<Vec<_>>();
    if options.shared_edge_fields {
        let fields = pieces.first().expect("actual A").definition().clone();
        let mut b = pieces.get(1).expect("actual B").definition().clone();
        b.boundary = fields.boundary;
        b.holes = fields.holes;
        b.construction_lines = fields.construction_lines;
        b.cut_on_fold = fields.cut_on_fold;
        b.fold_edges = fields.fold_edges;
        *pieces.get_mut(1).expect("actual B slot") =
            Piece::new(b, &f.ledger).expect("distinct Piece with byte-equal source fields");
    }
    let mut definitions = f
        .plan
        .copies()
        .iter()
        .map(|c| *c.definition())
        .collect::<Vec<_>>();
    if options.additional_source_pair || options.additional_authored_copies {
        let orientation = if options.additional_authored_copies {
            CopyOrientation::Authored
        } else {
            CopyOrientation::Reflected
        };
        definitions.extend([
            CutCopyDefinition {
                id: id(203),
                piece: id(100),
                orientation,
            },
            CutCopyDefinition {
                id: id(204),
                piece: id(101),
                orientation,
            },
        ]);
    }
    if options.extra_piece {
        let mut d = pieces.first().expect("source A").definition().clone();
        d.id = id(102);
        d.quantity = 1;
        d.mirroring = if options.source_companion {
            Mirroring::PairMember {
                handedness: Handedness::Right,
                companion: id(100),
            }
        } else {
            Mirroring::Single
        };
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
    let mut source_span = seams::first_span(&f.graph).definition().clone();
    source_span.direction = options.seam_direction;
    if options.shared_edge_fields {
        source_span.b.range = EdgeRange::whole(source_span.a.range.edge());
    }
    let original = SewingGraph::new(
        SewingGraphDefinition {
            id: id(300),
            spans: vec![source_span],
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
        SpanFixture::RetainedTail | SpanFixture::RetainedMiddle => {
            let middle = matches!(options.span, SpanFixture::RetainedMiddle);
            let cut = Param::new(Rational::new(1, if middle { 4 } else { 2 }).expect("source cut"))
                .expect("unit interval");
            let opposite = Param::new(cut.as_rational().one_minus().expect("opposite cut"))
                .expect("unit interval");
            let [_, tail] = seams::first_span(&original)
                .retain_ease_source()
                .split_at(
                    EdgeAnchor {
                        edge: definition.a.range.edge(),
                        param: cut,
                    },
                    EdgeAnchor {
                        edge: definition.b.range.edge(),
                        param: if options.seam_direction == SeamDirection::Same {
                            cut
                        } else {
                            opposite
                        },
                    },
                )
                .expect("actual retained tail");
            let tail = if middle {
                let [selected, _] = tail
                    .split_at(
                        EdgeAnchor {
                            edge: definition.a.range.edge(),
                            param: opposite,
                        },
                        EdgeAnchor {
                            edge: definition.b.range.edge(),
                            param: if options.seam_direction == SeamDirection::Same {
                                opposite
                            } else {
                                cut
                            },
                        },
                    )
                    .expect("actual retained middle");
                selected
            } else {
                tail
            };
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
    let population = JoinCutPopulation::new(options.result_quantity, options.mirroring);
    let cut = metadata
        .bind_cut_population(&population)
        .expect("explicit cut population");
    let original = [SeamSideId::A, SeamSideId::B]
        .map(|side| cut.metadata().splice().source().side(side).piece());
    let h = original.iter().map(|p| p.definition().holes.len()).sum();
    let r = original
        .iter()
        .map(|p| {
            let d = p.definition();
            d.boundary.len()
                + d.holes.iter().map(Vec::len).sum::<usize>()
                + d.construction_lines.len()
                + d.fold_edges.len()
        })
        .sum();
    let sources = cut
        .admit_source_pieces(JoinSourcePieceLimits::new(h, r))
        .expect("whole original H/R");
    let banks = sources
        .bind_cut_banks()
        .expect("actual original A/B cut banks");
    run(banks);
}
