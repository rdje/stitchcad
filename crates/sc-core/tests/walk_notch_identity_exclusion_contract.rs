//! Scoped identity-reference exclusions under complete resource accounting,never global freshness.
#![allow(clippy::expect_used)]
#[path = "support/walk_existing.rs"]
mod common;
use common::{
    assignments::{
        roles::copies::input::{fixture, id, profiles},
        with_assignments,
    },
    with_existing,
};
use sc_core::{construction::*, name::MachineToken, ontology::*};
fn full(k: usize) -> WalkNotchIdentityExclusionLimits {
    WalkNotchIdentityExclusionLimits::new(2, 2, 0, 3, 1, k, 2 + 4 + 3 + 14 + 9 * k + 3)
}
#[test]
fn exact_fixture_known_fields_include_all_piece_creators_profiles_existing_objects_and_actual_fixed_owners(
) {
    with_existing(|existing| {
        let limits = full(2);
        let bound = existing
            .bind_identity_exclusions(&limits)
            .expect("whole inspected scope");
        assert!(std::ptr::eq(bound.existing(), &existing));
        assert!(std::ptr::eq(bound.limits(), &limits));
        assert_eq!(
            (
                limits.copies(),
                limits.pieces(),
                limits.holes(),
                limits.edges(),
                limits.pairs(),
                limits.notches(),
                limits.workspace_entries()
            ),
            (2, 2, 0, 3, 1, 2, 44)
        );
        assert_eq!(
            (bound.holes(), bound.edges(), bound.workspace_bound()),
            (0, 3, 44)
        );
        let expected = [
            1, 2, 3, 10, 100, 101, 200, 201, 300, 301, 700, 701, 1000, 1001, 1002, 1003, 1004,
            1005, 2000, 2001, 2002, 2003, 2004, 2005,
        ];
        assert_eq!(bound.indexed_count(), expected.len());
        for bits in expected {
            assert!(
                bound.excludes_new_notch(id(bits)),
                "explicit known fixture id {bits}"
            );
        }
        assert!(!bound.excludes_new_notch(id(999)));
        assert!(std::ptr::eq(
            bound
                .existing()
                .notch(id(700))
                .expect("actual original object"),
            existing.notch(id(700)).expect("same object")
        ));
        let extra = existing
            .assignments()
            .roles()
            .copies()
            .piece(SeamSideId::A)
            .definition()
            .construction_lines
            .first()
            .expect("unrequested construction field")
            .edge
            .creator();
        assert_eq!(extra, id(3));
        assert!(bound.excludes_new_notch(extra));
    });
}
fn with_holes_fold_materials(run: impl FnOnce(BoundWalkExistingNotches<'_>)) {
    let f = fixture();
    let mut pieces = Vec::new();
    for original in &f.pieces {
        let mut definition = original.definition().clone();
        definition.material = MaterialAssignment::Assigned(if original.id() == id(100) {
            id(777)
        } else {
            id(888)
        });
        if original.id() == id(100) {
            definition.holes = vec![vec![DirectedEdge {
                edge: f.extra,
                direction: Direction::Reversed,
            }]];
            definition.cut_on_fold = true;
            definition.fold_edges =
                vec![definition.boundary.first().expect("actual outer edge").edge];
        }
        pieces.push(Piece::new(definition, &f.ledger).expect("actual complete Piece fields"));
    }
    let inventory = PieceInventory::new(pieces).expect("actual inventory");
    let edit = WalkSeamEdit::new(
        id(10),
        f.span().id(),
        vec![f.pair(
            common::assignments::roles::copies::input::t(1, 3),
            common::assignments::roles::copies::input::t(2, 5),
        )],
    );
    let input = edit
        .admit(WalkEditLimits::new(1))
        .expect("original requests");
    let source = input.bind_source(&f.graph).expect("actual span");
    let copy_limits = WalkCopyInventoryLimits::new(2, 2, 8);
    let copies = source
        .bind_copy_inventory(&f.plan, &inventory, &copy_limits)
        .expect("actual copy/Piece association");
    let raw_roles = NotchOutputRoleCatalog::new(
        id(10),
        vec![
            NotchOutputRoleDefinition::new(
                LocalTag::new(2),
                MachineToken::new("end").expect("key"),
            ),
            NotchOutputRoleDefinition::new(
                LocalTag::new(9),
                MachineToken::new("balance").expect("key"),
            ),
        ],
        vec![LocalTag::new(2), LocalTag::new(9)],
    );
    let role_limits = NotchOutputRoleCatalogLimits::new(2, 2, 10);
    let catalog = raw_roles.admit(role_limits).expect("typed catalogue");
    let roles = copies
        .bind_notch_roles(&catalog, role_limits)
        .expect("same walk owner");
    let raw = WalkNotchAssignmentManifest::new(
        id(10),
        vec![
            WalkNotchRoleAssignment::new(0, SeamSideId::A, LocalTag::new(2)),
            WalkNotchRoleAssignment::new(0, SeamSideId::B, LocalTag::new(9)),
        ],
    );
    let assignments = raw
        .admit(&roles, WalkNotchAssignmentLimits::new(1, 2, 4))
        .expect("complete attribution");
    let notches = common::supplied(&assignments);
    let existing = assignments
        .bind_existing_notches(&notches, WalkExistingNotchLimits::new(2))
        .expect("provided actual objects");
    run(existing);
}
#[test]
fn complete_c_i_n_k_then_h_r_workspace_refusals_preserve_actual_context_and_materials_fold_occurrences_count(
) {
    use WalkNotchIdentityExclusionRefusal as R;
    with_holes_fold_materials(|existing| {
        for (limits, expected) in [
            (
                WalkNotchIdentityExclusionLimits::new(1, 0, 0, 0, 0, 0, 0),
                R::Copies {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                WalkNotchIdentityExclusionLimits::new(2, 1, 0, 0, 0, 0, 0),
                R::Pieces {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                WalkNotchIdentityExclusionLimits::new(2, 2, 0, 0, 0, 0, 0),
                R::Pairs {
                    required: 1,
                    allowed: 0,
                },
            ),
            (
                WalkNotchIdentityExclusionLimits::new(2, 2, 0, 0, 1, 1, 0),
                R::Notches {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                WalkNotchIdentityExclusionLimits::new(2, 2, 0, 0, 1, 2, 0),
                R::Holes {
                    required: 1,
                    allowed: 0,
                },
            ),
            (
                WalkNotchIdentityExclusionLimits::new(2, 2, 1, 4, 1, 2, 0),
                R::Edges {
                    required: 5,
                    allowed: 4,
                },
            ),
            (
                WalkNotchIdentityExclusionLimits::new(2, 2, 1, 5, 1, 2, 45),
                R::Workspace {
                    required: 46,
                    allowed: 45,
                },
            ),
        ] {
            let error = existing
                .bind_identity_exclusions(&limits)
                .expect_err("whole-resource refusal");
            assert!(std::ptr::eq(error.existing(), &existing));
            assert!(std::ptr::eq(error.limits(), &limits));
            assert_eq!(error.refusal(), expected);
            assert!(error
                .to_string()
                .starts_with("walk known identity exclusions:"));
            assert!(format!("{error:?}").contains("WalkNotchIdentityExclusionError"));
        }
        let limits = WalkNotchIdentityExclusionLimits::new(2, 2, 1, 5, 1, 2, 46);
        let bound = existing
            .bind_identity_exclusions(&limits)
            .expect("all complete resources");
        assert_eq!(
            (
                bound.holes(),
                bound.edges(),
                bound.workspace_bound(),
                bound.indexed_count()
            ),
            (1, 5, 46, 26)
        );
        assert!(bound.excludes_new_notch(id(777)));
        assert!(bound.excludes_new_notch(id(888)));
    });
}
#[test]
fn all_unrelated_provided_notch_owner_anchor_and_profile_fields_remain_excluded_without_retargeting(
) {
    with_assignments(|assignments| {
        let mut f = fixture();
        let edge = *f
            .ledger
            .declare_tagged_edges(id(4), &[LocalTag::FIRST])
            .expect("explicit fixture source registration")
            .first()
            .expect("one edge");
        let mut definition = assignments
            .roles()
            .copies()
            .piece(SeamSideId::A)
            .definition()
            .clone();
        definition.id = id(600);
        definition.boundary = vec![DirectedEdge {
            edge,
            direction: Direction::Original,
        }];
        let piece = Piece::new(definition, &f.ledger).expect("independent actual Piece");
        let notch = Notch::new(
            NotchDefinition {
                id: id(702),
                anchor: EdgeAnchor {
                    edge,
                    param: common::assignments::roles::copies::input::t(1, 2),
                },
                representation: profiles(3000),
            },
            &piece,
            &f.ledger,
        )
        .expect("unrelated actual canonical Notch");
        let mut notches = common::supplied(&assignments);
        notches.push(notch);
        let snapshot = notches.clone();
        let existing = assignments
            .bind_existing_notches(&notches, WalkExistingNotchLimits::new(3))
            .expect("whole provided collection");
        let limits = full(3);
        let bound = existing
            .bind_identity_exclusions(&limits)
            .expect("all provided typed references");
        assert_eq!((bound.workspace_bound(), bound.indexed_count()), (53, 33));
        for bits in [4, 600, 702, 3000, 3001, 3002, 3003, 3004, 3005] {
            assert!(bound.excludes_new_notch(id(bits)));
        }
        assert_eq!(
            bound
                .existing()
                .notch(id(702))
                .expect("same held object")
                .piece(),
            id(600)
        );
        assert_eq!(notches, snapshot);
    });
}
#[test]
fn empty_supplied_notch_population_drops_no_original_source_exclusions_or_requests() {
    with_assignments(|assignments| {
        let notches = [];
        let existing = assignments
            .bind_existing_notches(&notches, WalkExistingNotchLimits::new(0))
            .expect("explicit provided empty");
        let limits = full(0);
        let bound = existing
            .bind_identity_exclusions(&limits)
            .expect("all other source fields");
        assert_eq!((bound.workspace_bound(), bound.indexed_count()), (26, 22));
        assert!(bound.excludes_new_notch(id(1000)));
        assert!(bound.excludes_new_notch(id(3)));
        assert!(!bound.excludes_new_notch(id(700)));
        assert_eq!(bound.existing().assignments().request_count(), 2);
    });
}
