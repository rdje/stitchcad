//! Exact splice intent keeps actual partial/eased/reflected sources;no geometry is certified.
#![allow(clippy::expect_used)]
use sc_core::{construction::*, ontology::*};
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn q(n: i64, d: i64) -> Param {
    Param::new(Rational::new(n, d).expect("fraction")).expect("unit interval")
}
#[path = "support/seam_edit_sources.rs"]
mod common;
use common::{first_span, fixture};
fn with_source(
    direction: SeamDirection,
    partial: bool,
    run: impl FnOnce(BoundJoinCopyInventory<'_>),
) {
    let f = fixture(false);
    let a = f.pieces.first().expect("A").clone();
    let mut b = f.pieces.get(1).expect("B").definition().clone();
    b.quantity = 2;
    b.mirroring = Mirroring::MirroredPairs;
    let inventory = PieceInventory::new(vec![Piece::new(b, &f.ledger).expect("paired B"), a])
        .expect("actual inventory");
    let plan = CutPlan::new(
        vec![
            CutCopyDefinition {
                id: id(201),
                piece: id(101),
                orientation: CopyOrientation::Reflected,
            },
            *f.plan.copy(id(200)).expect("original A").definition(),
            CutCopyDefinition {
                id: id(203),
                piece: id(101),
                orientation: CopyOrientation::Authored,
            },
        ],
        inventory.pieces(),
    )
    .expect("complete actual plan");
    let mut definition = first_span(&f.graph).definition().clone();
    definition.direction = direction;
    let original = SewingGraph::new(
        SewingGraphDefinition {
            id: id(300),
            spans: vec![definition],
        },
        &plan,
        inventory.pieces(),
        &f.marks,
        &f.ledger,
    )
    .expect("actual full source");
    let mut child = first_span(&original).definition().clone();
    if partial {
        let [portion, _] = first_span(&original)
            .retain_ease_source()
            .split_at(
                EdgeAnchor {
                    edge: child.a.range.edge(),
                    param: q(1, 3),
                },
                EdgeAnchor {
                    edge: child.b.range.edge(),
                    param: q(2, 5),
                },
            )
            .expect("actual retained portion");
        child.id = id(302);
        child.a = portion.side(SeamSideId::A);
        child.b = portion.side(SeamSideId::B);
        child.ease = portion.into();
    }
    let selected = child.id;
    let graph = SewingGraph::new(
        SewingGraphDefinition {
            id: id(300),
            spans: vec![child],
        },
        &plan,
        inventory.pieces(),
        &f.marks,
        &f.ledger,
    )
    .expect("actual selected source");
    let edit = JoinPiecesAtSeam::new(id(1000), selected, id(100), id(101))
        .expect("explicit distinct sources");
    let source = edit
        .bind_source(&graph, &plan, inventory.pieces())
        .expect("actual source slice");
    let limits = JoinCopyInventoryLimits::new(3, 2, 11);
    let bound = source
        .bind_copy_inventory(&inventory, &limits)
        .expect("actual complete inventory");
    run(bound);
}

#[test]
fn explicit_placement_keeps_both_endpoint_pairs_actual_ease_and_copy_frames() {
    let placements = [
        JoinSplicePlacement::KeepOriginal,
        JoinSplicePlacement::RigidAlign {
            fixed: SeamSideId::A,
            orientation: JoinMovingOrientation::Preserve,
        },
        JoinSplicePlacement::RigidAlign {
            fixed: SeamSideId::A,
            orientation: JoinMovingOrientation::Reflect,
        },
        JoinSplicePlacement::RigidAlign {
            fixed: SeamSideId::B,
            orientation: JoinMovingOrientation::Preserve,
        },
        JoinSplicePlacement::RigidAlign {
            fixed: SeamSideId::B,
            orientation: JoinMovingOrientation::Reflect,
        },
    ];
    for direction in [SeamDirection::Same, SeamDirection::Opposite] {
        for partial in [false, true] {
            with_source(direction, partial, |inventory| {
                let original = inventory.source().span().definition();
                let before = original.clone();
                for placement in placements {
                    let request = JoinExactSpliceRequirement::new(placement);
                    let splice = inventory.require_exact_splice(&request);
                    assert!(std::ptr::eq(splice.inventory(), &inventory));
                    assert!(std::ptr::eq(splice.requirement(), &request));
                    assert!(std::ptr::eq(
                        splice.requirement().placement(),
                        request.placement()
                    ));
                    assert_eq!(*request.placement(), placement);
                    assert!(std::ptr::eq(splice.source(), inventory.source()));
                    assert!(std::ptr::eq(splice.ease(), &original.ease));
                    assert_eq!(splice.ease().portion().is_some(), partial);
                    assert_eq!(
                        splice.ease().declaration().amount,
                        EaseAmount::Profile(ProfileParameterRef::new(id(8000)))
                    );
                    assert_eq!(
                        splice.source().side(SeamSideId::B).copy().orientation(),
                        CopyOrientation::Reflected
                    );
                    assert_eq!(splice.endpoint_pairs().len(), 2);
                    assert_eq!(
                        splice
                            .endpoint_pairs()
                            .rev()
                            .map(JoinSpliceEndpointPair::end)
                            .collect::<Vec<_>>(),
                        vec![JoinSpliceEnd::ATo, JoinSpliceEnd::AFrom]
                    );
                    let b_params = match direction {
                        SeamDirection::Same => [original.b.range.from(), original.b.range.to()],
                        SeamDirection::Opposite => [original.b.range.to(), original.b.range.from()],
                    };
                    for (pair, (end, a_param, b_param)) in splice.endpoint_pairs().zip([
                        (
                            JoinSpliceEnd::AFrom,
                            original.a.range.from(),
                            b_params.first().copied().expect("B first"),
                        ),
                        (
                            JoinSpliceEnd::ATo,
                            original.a.range.to(),
                            b_params.get(1).copied().expect("B second"),
                        ),
                    ]) {
                        assert!(std::ptr::eq(pair.context(), &splice));
                        assert_eq!(pair.end(), end);
                        assert!(std::ptr::eq(pair.source_side(SeamSideId::A), &original.a));
                        assert!(std::ptr::eq(pair.source_side(SeamSideId::B), &original.b));
                        assert_eq!(
                            pair.anchor(SeamSideId::A),
                            EdgeAnchor {
                                edge: original.a.range.edge(),
                                param: a_param
                            }
                        );
                        assert_eq!(
                            pair.anchor(SeamSideId::B),
                            EdgeAnchor {
                                edge: original.b.range.edge(),
                                param: b_param
                            }
                        );
                    }
                    assert!(format!("{splice:?}").contains("RequiredJoinExactSplice"));
                }
                assert_eq!(inventory.source().span().definition(), &before);
            });
        }
    }
}
