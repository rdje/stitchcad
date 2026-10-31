//! Canonical Piece identity declaration,local typed-owner exclusions and honest nonreuse boundaries.
#![allow(clippy::expect_used)]
#[path = "support/mirror_roles.rs"]
mod common;
use common::metadata::source as sources;
use common::with_roles;
use sc_core::{construction::*, ontology::*};

fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}

#[test]
fn canonical_identity_borrows_actual_field_and_parents_without_minting_or_reinterpreting_zero() {
    with_roles(|roles| {
        assert_eq!(
            *roles.metadata().cut().source().input().axis(),
            sources::axis()
        );
        for piece_id in [id(200), id(0)] {
            let declared = roles
                .declare_piece_identity(&piece_id)
                .expect("local declaration only");
            assert!(std::ptr::eq(declared.roles(), &roles));
            assert!(std::ptr::eq(declared.piece_id(), &piece_id));
            assert_eq!(*declared.piece_id(), piece_id);
            assert_eq!(
                declared.roles().metadata().cut().source().piece().id(),
                id(100)
            );
        }
    });
}

#[test]
fn intrinsic_and_original_source_operations_refuse_with_actual_context_and_first_field() {
    use MirrorPieceIdentityRefusal as R;
    with_roles(|roles| {
        let piece = roles.metadata().cut().source().piece();
        for (value, refusal) in [
            (10, R::Operation),
            (100, R::SourcePiece),
            (
                40,
                R::SourceOperation {
                    location: MirrorPieceEdgeLocation::Boundary(0),
                    edge: piece.definition().boundary.first().expect("first").edge,
                },
            ),
            (
                41,
                R::SourceOperation {
                    location: MirrorPieceEdgeLocation::Boundary(1),
                    edge: piece.definition().boundary.get(1).expect("second").edge,
                },
            ),
            (
                42,
                R::SourceOperation {
                    location: MirrorPieceEdgeLocation::Hole { hole: 0, edge: 0 },
                    edge: piece
                        .definition()
                        .holes
                        .first()
                        .and_then(|h| h.first())
                        .expect("hole0")
                        .edge,
                },
            ),
            (
                43,
                R::SourceOperation {
                    location: MirrorPieceEdgeLocation::Hole { hole: 1, edge: 0 },
                    edge: piece
                        .definition()
                        .holes
                        .get(1)
                        .and_then(|h| h.first())
                        .expect("hole1")
                        .edge,
                },
            ),
            (90, R::AxisOperation { index: 0 }),
            (91, R::AxisOperation { index: 1 }),
        ] {
            let piece_id = id(value);
            let error = roles
                .declare_piece_identity(&piece_id)
                .expect_err("known original owner");
            assert_eq!(error.refusal(), refusal);
            assert!(std::ptr::eq(error.roles(), &roles));
            assert!(std::ptr::eq(error.piece_id(), &piece_id));
            assert!(format!("{error}").contains("mirror result Piece"));
        }
    });
}

fn with_pair_roles(companion: u128, run: impl FnOnce(BoundMirrorPieceEdgeRoles<'_>)) {
    let (ledger, piece) = sources::fixture();
    let mut definition = piece.definition().clone();
    definition.quantity = 1;
    definition.mirroring = Mirroring::PairMember {
        handedness: Handedness::Right,
        companion: id(200),
    };
    definition.material = MaterialAssignment::Assigned(id(700));
    let piece = Piece::new(definition, &ledger).expect("actual source with planned companion");
    let input = sources::input(
        10,
        100,
        MirrorAxis::AlongRange(DirectedRange {
            range: EdgeRange::whole(EdgeRef::new(id(99), LocalTag::new(17))),
            direction: Direction::Reversed,
        }),
    );
    let source_limits = MirrorPieceSourceLimits::new(2, 8);
    let source = input
        .bind_piece(&piece, &source_limits)
        .expect("actual source fields");
    let policy = MirrorPieceCutPolicy::new(
        1,
        Mirroring::PairMember {
            handedness: Handedness::Left,
            companion: id(companion),
        },
        MirrorPieceFoldIntent::NoFold,
        CuttingSide::Face,
    );
    let cut = source
        .bind_cut_policy(&policy)
        .expect("explicit requested cut metadata");
    let fields = MirrorPieceMetadataPolicy::new(
        MirrorPieceFieldIntent::Replace(MaterialAssignment::Assigned(id(800))),
        MirrorPieceFieldIntent::PreserveSource,
        MirrorPieceFieldIntent::PreserveSource,
    );
    let field_limits = MirrorPieceMetadataLimits::new(27);
    let metadata = cut
        .bind_metadata(&fields, &field_limits)
        .expect("selected label bytes");
    let raw = EdgeOutputRoleCatalog::new(id(10), vec![], vec![]);
    let limits = EdgeOutputRoleCatalogLimits::new(0, 0, 0);
    let catalog = raw
        .admit(limits)
        .expect("metadata state,not executable outputs");
    let roles = metadata
        .bind_edge_roles(&catalog, limits)
        .expect("actual owner");
    run(roles);
}

#[test]
fn source_companion_can_name_planned_result_but_requested_self_companion_refuses_first() {
    with_pair_roles(100, |roles| {
        let result = id(200);
        let declared = roles
            .declare_piece_identity(&result)
            .expect("a companion reference is a use,not an owner");
        assert!(std::ptr::eq(declared.piece_id(), &result));
        assert_eq!(
            declared
                .roles()
                .metadata()
                .cut()
                .source()
                .piece()
                .definition()
                .mirroring,
            Mirroring::PairMember {
                handedness: Handedness::Right,
                companion: result
            }
        );
    });
    for value in [200, 99, 700, 800] {
        with_pair_roles(value, |roles| {
            let result = id(value);
            assert_eq!(
                roles
                    .declare_piece_identity(&result)
                    .expect_err("self companion before other aliases")
                    .refusal(),
                MirrorPieceIdentityRefusal::SelfCompanion
            );
        });
    }
}

#[test]
fn directed_range_axis_and_both_explicit_material_sources_keep_their_actual_causes() {
    with_pair_roles(100, |roles| {
        for (value, refusal) in [
            (99, MirrorPieceIdentityRefusal::AxisOperation { index: 0 }),
            (700, MirrorPieceIdentityRefusal::SourceMaterial),
            (800, MirrorPieceIdentityRefusal::ResultMaterial),
        ] {
            let result = id(value);
            assert_eq!(
                roles
                    .declare_piece_identity(&result)
                    .expect_err("actual typed owner")
                    .refusal(),
                refusal
            );
        }
    });
}
