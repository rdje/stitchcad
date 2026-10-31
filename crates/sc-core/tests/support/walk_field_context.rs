//! Actual two-copy contexts,including two physical copies of one pattern Piece.
#[path = "walk_known.rs"]
pub mod known;
use known::existing::assignments::roles::copies::input::{fixture, id, profiles, t};
use sc_core::{construction::*, name::MachineToken, ontology::*};
#[derive(Clone, Copy)]
pub struct Request {
    pub roles: [u32; 2],
    pub anchors: [Param; 2],
    pub profiles: [NotchProfileBindings; 2],
}
impl Request {
    pub fn same() -> Self {
        Self {
            roles: [2, 2],
            anchors: [t(1, 3); 2],
            profiles: [profiles(1000); 2],
        }
    }
}
pub fn with_requests(
    shared: bool,
    requests: &[Request],
    prior: Option<WalkNotchField>,
    run: impl FnOnce(BoundWalkNotchIdentityExclusions<'_>),
) {
    let f = fixture();
    let mut a = f.pieces.first().expect("A Piece").definition().clone();
    if shared {
        a.quantity = 2;
    }
    let mut pieces = vec![Piece::new(a, &f.ledger).expect("actual A")];
    if !shared {
        pieces.push(f.pieces.get(1).expect("actual B").clone());
    }
    let inventory = PieceInventory::new(pieces).expect("actual complete inventory");
    let plan = CutPlan::new(
        vec![
            CutCopyDefinition {
                id: id(200),
                piece: id(100),
                orientation: CopyOrientation::Authored,
            },
            CutCopyDefinition {
                id: id(201),
                piece: if shared { id(100) } else { id(101) },
                orientation: CopyOrientation::Authored,
            },
        ],
        inventory.pieces(),
    )
    .expect("two actual physical copies");
    let mut span = f.span().definition().clone();
    if shared {
        span.b.range = EdgeRange::new(span.a.range.edge(), t(1, 5), t(4, 5))
            .expect("B range on shared pattern");
    }
    let graph = SewingGraph::new(
        SewingGraphDefinition {
            id: id(300),
            spans: vec![span],
        },
        &plan,
        inventory.pieces(),
        &[],
        &f.ledger,
    )
    .expect("actual same/different pattern graph");
    let span = graph.spans().first().expect("actual span");
    let edit = WalkSeamEdit::new(
        id(10),
        id(301),
        requests
            .iter()
            .map(|r| {
                WalkBalancePair::new(
                    WalkBalanceNotch::new(
                        EdgeAnchor {
                            edge: span.definition().a.range.edge(),
                            param: r.anchors[0],
                        },
                        r.profiles[0],
                    ),
                    WalkBalanceNotch::new(
                        EdgeAnchor {
                            edge: span.definition().b.range.edge(),
                            param: r.anchors[1],
                        },
                        r.profiles[1],
                    ),
                )
            })
            .collect(),
    );
    let admitted = edit
        .admit(WalkEditLimits::new(requests.len()))
        .expect("complete requests");
    let source = admitted
        .bind_source(&graph)
        .expect("actual source intervals");
    let copy_limits =
        WalkCopyInventoryLimits::new(2, inventory.pieces().len(), inventory.pieces().len() + 6);
    let copies = source
        .bind_copy_inventory(&plan, &inventory, &copy_limits)
        .expect("actual copy/pattern context");
    let raw_roles = NotchOutputRoleCatalog::new(
        id(10),
        [(2, "end"), (9, "balance")]
            .into_iter()
            .map(|(tag, key)| {
                NotchOutputRoleDefinition::new(
                    LocalTag::new(tag),
                    MachineToken::new(key).expect("key"),
                )
            })
            .collect(),
        vec![LocalTag::new(2), LocalTag::new(9)],
    );
    let role_limits = NotchOutputRoleCatalogLimits::new(2, 2, 10);
    let catalog = raw_roles.admit(role_limits).expect("typed roles");
    let roles = copies
        .bind_notch_roles(&catalog, role_limits)
        .expect("actual operation");
    let raw_assignments = WalkNotchAssignmentManifest::new(
        id(10),
        requests
            .iter()
            .enumerate()
            .flat_map(|(pair, r)| {
                [
                    WalkNotchRoleAssignment::new(pair, SeamSideId::A, LocalTag::new(r.roles[0])),
                    WalkNotchRoleAssignment::new(pair, SeamSideId::B, LocalTag::new(r.roles[1])),
                ]
            })
            .collect(),
    );
    let assignments = raw_assignments
        .admit(
            &roles,
            WalkNotchAssignmentLimits::new(requests.len(), 2 * requests.len(), 4 * requests.len()),
        )
        .expect("all original occurrences");
    let old: [Notch; 2] = std::array::from_fn(|index| {
        let item = assignments
            .assignment(
                0,
                if index == 0 {
                    SeamSideId::A
                } else {
                    SeamSideId::B
                },
            )
            .expect("first pair");
        let mut anchor = *item.requested().anchor();
        let mut representation = *item.requested().representation();
        if index == 0 {
            match prior {
                Some(WalkNotchField::Anchor) => anchor.param = t(1, 4),
                Some(WalkNotchField::Profiles) => {
                    representation.production.width = profiles(3000).production.width
                }
                _ => {}
            }
        }
        Notch::new(
            NotchDefinition {
                id: id(700 + index as u128),
                anchor,
                representation,
            },
            item.piece(),
            &f.ledger,
        )
        .expect("actual prior Notch")
    });
    let existing = assignments
        .bind_existing_notches(&old, WalkExistingNotchLimits::new(2))
        .expect("provided prior objects");
    let (i, r) = (inventory.pieces().len(), if shared { 2 } else { 3 });
    let limits = WalkNotchIdentityExclusionLimits::new(
        2,
        i,
        0,
        r,
        requests.len(),
        2,
        2 + 2 * i + r + 14 * requests.len() + 18 + 3,
    );
    let known = existing
        .bind_identity_exclusions(&limits)
        .expect("whole inspected references");
    run(known);
}
