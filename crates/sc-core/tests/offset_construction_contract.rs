//! Offset input association through real allowance records, without fabricated fragment outputs.
#![allow(clippy::expect_used)]
use sc_core::{
    construction::{OffsetBindingError, OffsetConstruction, OffsetConstructionError, OffsetSide},
    ontology::*,
};
use sc_units::Length;

fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn parameter(n: i64, d: i64) -> Param {
    Param::new(Rational::new(n, d).expect("exact rational")).expect("unit interval")
}
#[path = "support/offset_sources.rs"]
mod sources;
use sources::fixture;
#[test]
fn actual_width_origins_and_corners_are_borrowed_from_the_bound_descriptor_without_defaults() {
    let mut f = fixture();
    for width in [
        AllowanceWidth::Explicit {
            parameter: id(70),
            value: Length::from_micrometres(0).expect("zero"),
        },
        AllowanceWidth::Formula(id(70)),
        AllowanceWidth::Profile(ProfileParameterRef::new(id(70))),
    ] {
        for corner in [
            CornerTreatment::Miter,
            CornerTreatment::Slant,
            CornerTreatment::Envelope,
            CornerTreatment::Trim,
            CornerTreatment::Step,
        ] {
            f.definition.width = width;
            f.definition.corner = corner;
            let allowance = f.allowance();
            let input = f.input();
            let bound = input
                .bind_allowance(&allowance)
                .expect("exact original identity and edge");
            assert!(std::ptr::eq(bound.construction(), &input));
            assert!(std::ptr::eq(bound.allowance(), &allowance));
            assert!(std::ptr::eq(bound.width(), &allowance.definition().width));
            assert!(std::ptr::eq(bound.corner(), &allowance.definition().corner));
            assert!(std::ptr::eq(
                bound.inclusion(),
                &allowance.definition().inclusion
            ));
            assert_eq!(*bound.width(), width);
            assert_eq!(*bound.corner(), corner);
            assert_eq!(bound.allowance().piece(), f.piece.id());
        }
    }
}

#[test]
fn wrong_descriptor_precedes_wrong_source_and_equal_id_cannot_retarget_the_original_edge() {
    let mut f = fixture();
    let input = f.input();
    f.definition.id = id(81);
    f.definition.edge = f
        .piece
        .definition()
        .construction_lines
        .first()
        .expect("construction source")
        .edge;
    let wrong = f.allowance();
    assert_eq!(
        input.bind_allowance(&wrong).expect_err("both mismatches"),
        OffsetBindingError::AllowanceIdentity {
            expected: id(80),
            actual: id(81)
        }
    );
    f.definition.id = id(80);
    let wrong_source = f.allowance();
    assert_eq!(
        input
            .bind_allowance(&wrong_source)
            .expect_err("same id is insufficient"),
        OffsetBindingError::SourceEdge {
            source: *input.source(),
            allowance_edge: f.definition.edge
        }
    );
    assert_ne!(input.source().range.edge(), f.definition.edge);
}

#[test]
fn definite_local_self_supply_keeps_original_range_and_precedes_allowance_identity_refusal() {
    let edge = EdgeRef::new(id(10), LocalTag::new(37));
    let source = DirectedRange {
        range: EdgeRange::new(edge, parameter(1, 3), parameter(2, 3)).expect("original range"),
        direction: Direction::Reversed,
    };
    assert_eq!(
        OffsetConstruction::new(id(10), source, OffsetSide::Right, id(10)),
        Err(OffsetConstructionError::SelfSource {
            operation: id(10),
            source
        })
    );
    let prior = DirectedRange {
        range: EdgeRange::whole(EdgeRef::new(id(99), LocalTag::new(37))),
        direction: Direction::Original,
    };
    assert_eq!(
        OffsetConstruction::new(id(10), prior, OffsetSide::Left, id(10)),
        Err(OffsetConstructionError::SelfAllowance(id(10)))
    );
}

#[test]
fn traversal_and_partial_source_survive_unchanged_and_association_does_not_claim_currentness() {
    let mut f = fixture();
    let allowance = f.allowance();
    let original = f.definition.clone();
    let range = f.input().source().range;
    for direction in [Direction::Original, Direction::Reversed] {
        for side in [OffsetSide::Left, OffsetSide::Right] {
            let input = OffsetConstruction::new(
                id(10),
                DirectedRange { range, direction },
                side,
                allowance.id(),
            )
            .expect("input");
            let bound = input.bind_allowance(&allowance).expect("association");
            assert_eq!(bound.construction().operation(), id(10));
            assert_eq!(
                *bound.construction().source(),
                DirectedRange { range, direction }
            );
            assert_eq!(bound.construction().direction(), direction);
            assert_eq!(bound.construction().side(), side);
            assert_eq!(bound.construction().allowance_id(), allowance.id());
        }
    }
    f.ledger
        .delete(
            &mut DeterministicIdGenerator::from_start(1000),
            original.edge,
        )
        .expect("delete source");
    assert!(matches!(
        allowance.validate_current(&f.piece, &f.ledger),
        Err(SeamAllowanceError::UnresolvedEdge(_))
    ));
    let input = f.input();
    let bound = input
        .bind_allowance(&allowance)
        .expect("association remains purely structural");
    assert_eq!(bound.allowance().definition(), &original);
    assert_eq!(*bound.width(), AllowanceWidth::Formula(id(70)));
}
