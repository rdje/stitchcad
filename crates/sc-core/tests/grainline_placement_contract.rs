//! Actual semantic placement sources and association boundaries, without generated grain geometry.
#![allow(clippy::expect_used)]
use sc_core::{construction::*, ontology::*};
use sc_units::Angle;

fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
struct Fixture {
    ledger: IdentityLedger,
    piece: Piece,
    definition: GrainlineDefinition,
}
fn fixture() -> Fixture {
    let mut ledger = IdentityLedger::new();
    let mut ids = DeterministicIdGenerator::new();
    let [axis, arrow, stripe, plaid] = std::array::from_fn(|_| DirectedRange {
        range: EdgeRange::whole(
            *ledger
                .declare_edges(&mut ids, 1)
                .expect("source allocation")
                .first()
                .expect("one source"),
        ),
        direction: Direction::Original,
    });
    let edge = |source: DirectedRange| DirectedEdge {
        edge: source.range.edge(),
        direction: source.direction,
    };
    let piece = Piece::new(
        PieceDefinition {
            id: id(200),
            boundary: vec![edge(axis)],
            holes: vec![],
            construction_lines: vec![edge(arrow), edge(stripe), edge(plaid)],
            quantity: 1,
            mirroring: Mirroring::Single,
            cut_on_fold: false,
            fold_edges: vec![],
            cutting_side: CuttingSide::Face,
            material: MaterialAssignment::Unresolved("test selection".into()),
            layer_index: 0,
            label: LabelText {
                name: "test".into(),
                size: "M".into(),
                fabric: "woven".into(),
                colorway: "plain".into(),
            },
        },
        &ledger,
    )
    .expect("structural Piece");
    Fixture {
        ledger,
        piece,
        definition: GrainlineDefinition {
            id: id(100),
            arrow,
            alignment: GrainAlignment::ParallelTo(axis),
            stripe: Some(stripe),
            plaid: Some(DirectedRange {
                direction: Direction::Reversed,
                ..plaid
            }),
        },
    }
}
impl Fixture {
    fn grainline(&self) -> Grainline {
        Grainline::new(self.definition.clone(), &self.piece, &self.ledger)
            .expect("semantic grain intent")
    }
    fn placement(&self) -> GrainlinePlacement {
        GrainlinePlacement::new(id(10), self.definition.id).expect("distinct target")
    }
}

#[test]
fn every_alignment_origin_and_independent_print_role_remain_attached_to_actual_owners() {
    let mut f = fixture();
    let reference = f.definition.alignment.reference();
    for alignment in [
        GrainAlignment::ParallelTo(reference),
        GrainAlignment::AtAngle {
            reference,
            angle: GrainAngle::Explicit(Angle::STRAIGHT),
        },
        GrainAlignment::AtAngle {
            reference,
            angle: GrainAngle::Formula(id(300)),
        },
        GrainAlignment::AtAngle {
            reference,
            angle: GrainAngle::Profile(ProfileParameterRef::new(id(301))),
        },
    ] {
        f.definition.alignment = alignment;
        let grainline = f.grainline();
        let placement = f.placement();
        let bound = placement
            .bind_grainline(&grainline)
            .expect("actual source association");
        assert!(std::ptr::eq(bound.placement(), &placement));
        assert!(std::ptr::eq(bound.grainline(), &grainline));
        assert!(std::ptr::eq(
            bound.alignment(),
            &grainline.definition().alignment
        ));
        assert_eq!(*bound.alignment(), alignment);
        assert_eq!(bound.placement().operation(), id(10));
        assert_eq!(bound.placement().grainline_id(), grainline.id());
        assert_eq!(bound.grainline().piece(), f.piece.id());
        assert_eq!(
            bound.references().collect::<Vec<_>>(),
            vec![
                (GrainReferenceRole::Arrow, f.definition.arrow),
                (GrainReferenceRole::Alignment, reference),
                (
                    GrainReferenceRole::Stripe,
                    f.definition.stripe.expect("declared stripe")
                ),
                (
                    GrainReferenceRole::Plaid,
                    f.definition.plaid.expect("declared plaid")
                ),
            ]
        );
    }
}

#[test]
fn target_identity_precedes_every_actual_self_source_and_repeated_self_sources_refuse_arrow_first()
{
    let mut f = fixture();
    let grainline = f.grainline();
    assert_eq!(
        GrainlinePlacement::new(id(100), id(100)),
        Err(GrainlinePlacementError::SelfTarget(id(100)))
    );
    for (role, reference) in grainline.references() {
        let operation = reference.range.edge().creator();
        let placement =
            GrainlinePlacement::new(operation, grainline.id()).expect("distinct target");
        assert_eq!(
            placement
                .bind_grainline(&grainline)
                .expect_err("each original source role"),
            GrainlinePlacementError::SelfSource {
                operation,
                role,
                reference
            }
        );
    }
    let operation = f.definition.arrow.range.edge().creator();
    let wrong = GrainlinePlacement::new(operation, id(101)).expect("different required target");
    assert_eq!(
        wrong
            .bind_grainline(&grainline)
            .expect_err("target mismatch is first"),
        GrainlinePlacementError::TargetIdentity {
            expected: id(101),
            actual: id(100)
        }
    );
    let reference = f.definition.arrow;
    f.definition.alignment = GrainAlignment::ParallelTo(reference);
    f.definition.stripe = Some(reference);
    f.definition.plaid = Some(reference);
    let repeated = f.grainline();
    let placement = GrainlinePlacement::new(operation, repeated.id()).expect("distinct target");
    assert_eq!(
        placement
            .bind_grainline(&repeated)
            .expect_err("first original role"),
        GrainlinePlacementError::SelfSource {
            operation,
            role: GrainReferenceRole::Arrow,
            reference
        }
    );
}

#[test]
fn absent_print_roles_stay_absent_and_equal_sources_still_have_two_distinct_semantic_uses() {
    let mut f = fixture();
    f.definition.stripe = None;
    f.definition.plaid = None;
    f.definition.alignment = GrainAlignment::ParallelTo(f.definition.arrow);
    let grainline = f.grainline();
    let placement = f.placement();
    let bound = placement
        .bind_grainline(&grainline)
        .expect("no optional defaults");
    assert_eq!(
        bound.references().collect::<Vec<_>>(),
        vec![
            (GrainReferenceRole::Arrow, f.definition.arrow),
            (GrainReferenceRole::Alignment, f.definition.arrow),
        ]
    );
    assert_eq!(bound.grainline().definition().stripe, None);
    assert_eq!(bound.grainline().definition().plaid, None);
}

#[test]
fn later_plaid_deletion_remains_a_currentness_error_without_rewriting_bound_semantic_intent() {
    let mut f = fixture();
    let grainline = f.grainline();
    let held = grainline.definition().clone();
    let source = held.plaid.expect("actual independent plaid");
    f.ledger
        .delete(
            &mut DeterministicIdGenerator::from_start(1000),
            source.range.edge(),
        )
        .expect("delete plaid source");
    assert!(matches!(
        grainline.validate_current(&f.piece, &f.ledger),
        Err(GrainlineError::UnresolvedReference {
            role: GrainReferenceRole::Plaid,
            ..
        })
    ));
    let placement = f.placement();
    let bound = placement
        .bind_grainline(&grainline)
        .expect("association excludes no missing ledger source");
    assert_eq!(bound.grainline().definition(), &held);
    assert_eq!(
        bound.grainline().geometric_validation(),
        GeometricValidation::DeferredToG2
    );
}
