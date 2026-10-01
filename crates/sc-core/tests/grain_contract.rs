//! Directed grain contracts, with independent expected traversal/order and no geometry claims.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
use sc_core::ontology::{
    CuttingSide, DeterministicIdGenerator, DirectedEdge, DirectedRange, Direction, EdgeRange,
    EdgeRef, EntityId, GeometricValidation, GrainAlignment, GrainAngle, GrainReferenceRole,
    Grainline, GrainlineDefinition, GrainlineError, IdGenerator, IdentityLedger, LabelText,
    LocalTag, MaterialAssignment, Mirroring, Param, Piece, PieceDefinition,
    ProfileBindingValidation, ProfileParameterRef, RangeIssue, RangePortion, Rational,
};
use sc_units::{Angle, Length};
fn t(n: i64, d: i64) -> Param {
    Param::new(Rational::new(n, d).unwrap()).unwrap()
}
struct Fixture {
    piece: Piece,
    ledger: IdentityLedger,
    ids: DeterministicIdGenerator,
    definition: GrainlineDefinition,
}
fn fixture() -> Fixture {
    let mut ledger = IdentityLedger::new();
    let mut ids = DeterministicIdGenerator::new();
    let edges = ledger.declare_edges(&mut ids, 4).unwrap();
    let directed = |index| DirectedEdge {
        edge: *edges.get(index).unwrap(),
        direction: Direction::Original,
    };
    let piece = Piece::new(
        PieceDefinition {
            id: ids.next_id(),
            boundary: vec![directed(0)],
            holes: vec![],
            construction_lines: vec![directed(1), directed(2), directed(3)],
            quantity: 1,
            mirroring: Mirroring::Single,
            cut_on_fold: false,
            fold_edges: vec![],
            cutting_side: CuttingSide::Face,
            material: MaterialAssignment::Unresolved("selection pending".into()),
            layer_index: 0,
            label: LabelText {
                name: "test piece".into(),
                size: "M".into(),
                fabric: "woven".into(),
                colorway: "plain".into(),
            },
        },
        &ledger,
    )
    .unwrap();
    let range = |index| DirectedRange {
        range: EdgeRange::whole(*edges.get(index).unwrap()),
        direction: Direction::Original,
    };
    Fixture {
        piece,
        ledger,
        ids,
        definition: GrainlineDefinition {
            id: EntityId::from_bits(100),
            arrow: range(1),
            alignment: GrainAlignment::ParallelTo(range(0)),
            stripe: Some(range(2)),
            plaid: Some(DirectedRange {
                direction: Direction::Reversed,
                ..range(3)
            }),
        },
    }
}
impl Fixture {
    fn grain(&self) -> Result<Grainline, GrainlineError> {
        Grainline::new(self.definition.clone(), &self.piece, &self.ledger)
    }
    fn set(&mut self, role: GrainReferenceRole, range: DirectedRange) {
        match role {
            GrainReferenceRole::Arrow => self.definition.arrow = range,
            GrainReferenceRole::Alignment => {
                self.definition.alignment = GrainAlignment::ParallelTo(range)
            }
            GrainReferenceRole::Stripe => self.definition.stripe = Some(range),
            GrainReferenceRole::Plaid => self.definition.plaid = Some(range),
        }
    }
}
#[test]
fn semantic_arrow_alignment_and_both_print_references_are_retained_independently() {
    let f = fixture();
    let grain = f.grain().unwrap();
    assert_eq!(grain.id(), f.definition.id);
    assert_eq!(grain.piece(), f.piece.id());
    assert_eq!(grain.definition(), &f.definition);
    assert_eq!(
        grain.references().map(|(role, _)| role).collect::<Vec<_>>(),
        vec![
            GrainReferenceRole::Arrow,
            GrainReferenceRole::Alignment,
            GrainReferenceRole::Stripe,
            GrainReferenceRole::Plaid
        ]
    );
    assert_ne!(grain.definition().stripe, grain.definition().plaid);
    assert_eq!(
        grain.geometric_validation(),
        GeometricValidation::DeferredToG2
    );
    assert_eq!(grain.profile_binding_validation(), None);
}
#[test]
fn explicit_bias_antiparallel_and_symbolic_angles_never_claim_alignment_or_defaults() {
    let mut f = fixture();
    let reference = f.definition.alignment.reference();
    for angle in [
        GrainAngle::Explicit(Angle::from_degrees_rational(45, 1).unwrap()),
        GrainAngle::Explicit(Angle::STRAIGHT),
        GrainAngle::Formula(EntityId::from_bits(200)),
        GrainAngle::Profile(ProfileParameterRef::new(EntityId::from_bits(201))),
    ] {
        f.definition.alignment = GrainAlignment::AtAngle { reference, angle };
        let grain = f.grain().unwrap();
        assert_eq!(grain.definition().alignment, f.definition.alignment);
        assert_eq!(
            grain.geometric_validation(),
            GeometricValidation::DeferredToG2
        );
        assert_eq!(
            grain.profile_binding_validation(),
            if matches!(angle, GrainAngle::Profile(_)) {
                Some(ProfileBindingValidation::DeferredToG4)
            } else {
                None
            }
        );
    }
}
#[test]
fn opposite_arrows_are_distinct_and_reversal_composes_with_authored_traversal() {
    for authored in [Direction::Original, Direction::Reversed] {
        let mut f = fixture();
        f.definition.arrow.direction = authored;
        let grain = f.grain().unwrap();
        let before = grain.definition().arrow.resolve(&f.ledger);
        let expected_start = if authored == Direction::Original {
            Param::START
        } else {
            Param::END
        };
        assert_eq!(before.start().resolved().unwrap().param(), expected_start);
        assert_eq!(before.portions().next().unwrap().direction, authored);
        f.ledger
            .reverse(&mut f.ids, f.definition.arrow.range.edge())
            .unwrap();
        let after = grain.definition().arrow.resolve(&f.ledger);
        assert_eq!(
            after.start().resolved().unwrap().param(),
            if authored == Direction::Original {
                Param::END
            } else {
                Param::START
            }
        );
        assert_eq!(
            after.portions().next().unwrap().direction,
            authored.reversed()
        );
        assert_eq!(after.end().resolved().unwrap().param(), expected_start);
        assert_eq!(after.held(), f.definition.arrow);
        assert_eq!(grain.definition(), &f.definition);
    }
    let mut f = fixture();
    let first = f.grain().unwrap();
    f.definition.arrow.direction = Direction::Reversed;
    assert_ne!(first, f.grain().unwrap());
}
#[test]
fn reversed_traversal_orders_split_fragments_and_interior_repairs_from_its_own_start() {
    let mut f = fixture();
    f.definition.arrow.direction = Direction::Reversed;
    let grain = f.grain().unwrap();
    let edge = f.definition.arrow.range.edge();
    let first = f.ledger.split(&mut f.ids, edge, t(1, 3)).unwrap();
    let second = f.ledger.split(&mut f.ids, first.second(), t(1, 2)).unwrap();
    f.ledger.delete(&mut f.ids, second.first()).unwrap();
    let resolved = grain.definition().arrow.resolve(&f.ledger);
    assert!(!resolved.evidence().has_full_coverage());
    let parts = resolved.portions().collect::<Vec<_>>();
    assert_eq!(parts.len(), 3);
    assert!(
        matches!(parts.first().unwrap().portion,RangePortion::Resolved(part) if part.range().edge()==second.second())
    );
    assert!(
        matches!(parts.get(1).unwrap().portion,RangePortion::Unresolved(task) if task.held()==f.definition.arrow.range)
    );
    assert!(
        matches!(parts.last().unwrap().portion,RangePortion::Resolved(part) if part.range().edge()==first.first())
    );
    assert!(parts
        .iter()
        .all(|part| part.direction == Direction::Reversed));
    assert_eq!(resolved.start().resolved().unwrap().edge(), second.second());
    assert_eq!(resolved.end().resolved().unwrap().edge(), first.first());
    assert_eq!(grain.definition(), &f.definition);
}
#[test]
fn every_reference_role_refuses_unknown_and_live_foreign_intervals() {
    for role in [
        GrainReferenceRole::Arrow,
        GrainReferenceRole::Alignment,
        GrainReferenceRole::Stripe,
        GrainReferenceRole::Plaid,
    ] {
        let mut f = fixture();
        let missing = EdgeRef::new(EntityId::from_bits(u128::MAX), LocalTag::FIRST);
        f.set(
            role,
            DirectedRange {
                range: EdgeRange::whole(missing),
                direction: Direction::Original,
            },
        );
        assert!(
            matches!(f.grain(),Err(GrainlineError::UnresolvedReference {role:actual,evidence}) if actual==role && evidence.repairs().next().unwrap().issue()==RangeIssue::UnknownSource)
        );
        let foreign = *f
            .ledger
            .declare_edges(&mut f.ids, 1)
            .unwrap()
            .first()
            .unwrap();
        f.set(
            role,
            DirectedRange {
                range: EdgeRange::whole(foreign),
                direction: Direction::Original,
            },
        );
        assert_eq!(
            f.grain(),
            Err(GrainlineError::OutsidePiece {
                role,
                piece: f.piece.id()
            })
        );
    }
}
#[test]
fn positive_coverage_does_not_silently_choose_an_ambiguous_born_endpoint() {
    let mut f = fixture();
    let edge = f.definition.arrow.range.edge();
    f.ledger.split(&mut f.ids, edge, t(1, 2)).unwrap();
    f.definition.arrow.range = EdgeRange::new(edge, t(1, 2), Param::END).unwrap();
    assert!(
        matches!(f.grain(),Err(GrainlineError::UnresolvedReference {role:GrainReferenceRole::Arrow,evidence}) if evidence.has_full_coverage() && evidence.start().resolved().is_none())
    );
}
#[test]
fn merged_foreign_remainder_is_rejected_in_current_frame_after_reversal() {
    let mut f = fixture();
    let source = f.definition.arrow.range.edge();
    let foreign = *f
        .ledger
        .declare_edges(&mut f.ids, 1)
        .unwrap()
        .first()
        .unwrap();
    let merged = f
        .ledger
        .merge(
            &mut f.ids,
            source,
            Length::from_micrometres(3).unwrap(),
            foreign,
            Length::from_micrometres(7).unwrap(),
        )
        .unwrap();
    f.definition.arrow.range = EdgeRange::new(merged, Param::START, t(1, 5)).unwrap();
    assert!(f.grain().is_ok());
    f.definition.arrow.range = EdgeRange::new(merged, t(1, 5), t(4, 5)).unwrap();
    assert_eq!(
        f.grain(),
        Err(GrainlineError::OutsidePiece {
            role: GrainReferenceRole::Arrow,
            piece: f.piece.id()
        })
    );
    f.ledger.reverse(&mut f.ids, merged).unwrap();
    assert!(matches!(
        f.grain(),
        Err(GrainlineError::OutsidePiece { .. })
    ));
}
#[test]
fn optional_print_references_are_explicit_and_definition_copies_cannot_mutate_grain() {
    let mut f = fixture();
    f.definition.stripe = None;
    f.definition.plaid = None;
    let grain = f.grain().unwrap();
    assert_eq!(grain.references().count(), 2);
    let mut edited = grain.definition().clone();
    edited.arrow.direction = Direction::Reversed;
    edited.stripe = Some(edited.arrow); // Coincident references are explicit intent, not proved alignment.
    let replacement = Grainline::new(edited, &f.piece, &f.ledger).unwrap();
    assert_eq!(replacement.references().count(), 3);
    assert_ne!(replacement, grain);
    assert_eq!(grain.definition(), &f.definition);
}
#[test]
fn a_reversed_reference_retains_raw_evidence_and_every_role_reports_repairs() {
    let mut f = fixture();
    let grain = f.grain().unwrap();
    let edge = f.definition.plaid.unwrap().range.edge();
    f.ledger.delete(&mut f.ids, edge).unwrap();
    let (role, resolved) = grain
        .reference_resolutions(&f.ledger)
        .find(|(role, _)| *role == GrainReferenceRole::Plaid)
        .unwrap();
    assert_eq!(role, GrainReferenceRole::Plaid);
    assert_eq!(
        resolved.evidence(),
        &f.ledger.resolve_range(resolved.held().range)
    );
    assert_eq!(
        resolved.portions().next().unwrap().direction,
        Direction::Reversed
    );
    assert_eq!(resolved.evidence().repairs().count(), 1);
    assert!(resolved.start().resolved().is_none());
}
