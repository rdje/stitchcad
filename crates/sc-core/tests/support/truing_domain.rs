//! Actual partial selected span domains on either adjusted side;no currentness or fitted curve.
#[path = "truing_roles.rs"]
pub mod roles;
use roles::copies::{id, sources};
use sc_core::{construction::*, name::MachineToken, ontology::*};
use sc_units::{Length, Tolerance, ToleranceClass};
pub fn q(n: i64, d: i64) -> Param {
    Param::new(Rational::new(n, d).expect("exact fraction")).expect("unit interval")
}
pub fn with_partial(side: SeamSideId, run: impl FnOnce(BoundTruingEdgeRoles<'_>)) {
    with_domain(side, q(1, 4), q(3, 4), run);
}
pub fn with_domain(
    side: SeamSideId,
    from: Param,
    to: Param,
    run: impl FnOnce(BoundTruingEdgeRoles<'_>),
) {
    with_domain_inventory(side, from, to, |f| f.pieces.clone(), run);
}
pub fn with_domain_inventory(
    side: SeamSideId,
    from: Param,
    to: Param,
    supplied: impl FnOnce(&mut sources::Fixture) -> Vec<Piece>,
    run: impl FnOnce(BoundTruingEdgeRoles<'_>),
) {
    let mut f = sources::fixture(false);
    let pieces = supplied(&mut f);
    let inventory = PieceInventory::new(f.pieces.clone()).expect("whole actual inventory");
    let mut span = sources::first_span(&f.graph).definition().clone();
    let selected = match side {
        SeamSideId::A => &mut span.a,
        SeamSideId::B => &mut span.b,
    };
    selected.range =
        EdgeRange::new(selected.range.edge(), from, to).expect("actual positive partial domain");
    let marks = if side == SeamSideId::A {
        vec![SewingLandmark::Turn(
            TurnPoint::new(
                id(401),
                EdgeAnchor {
                    edge: span.a.range.edge(),
                    param: from,
                },
                inventory.piece(id(100)).expect("actual A"),
                &f.ledger,
            )
            .expect("actual original stop at partial endpoint"),
        )]
    } else {
        f.marks.clone()
    };
    let graph = SewingGraph::new(
        SewingGraphDefinition {
            id: id(300),
            spans: vec![span],
        },
        &f.plan,
        inventory.pieces(),
        &marks,
        &f.ledger,
    )
    .expect("actual graph with partial selected range");
    let fit = Tolerance::new(
        ToleranceClass::GeometricApproximation,
        Length::from_micrometres(7).expect("length"),
        "explicit fixture length-fit source,not fitting evidence",
    )
    .expect("requirement");
    let edit = ConstrainedSeamTruing::new(
        id(1000),
        id(301),
        side,
        if side == SeamSideId::A {
            id(100)
        } else {
            id(101)
        },
        id(9000),
    )
    .expect("inputs");
    let source = edit
        .bind_source(
            &graph,
            &f.plan,
            TruingFitProvider {
                source: id(9000),
                tolerance: &fit,
            },
        )
        .expect("actual selected original source");
    let inventory = PieceInventory::new(pieces).expect("actual separately supplied inventory");
    let copy_limits = TruingCopyInventoryLimits::new(2, 2, 8);
    let copies = source
        .bind_copy_inventory(&inventory, &copy_limits)
        .expect("actual complete copy/Piece context");
    let raw = EdgeOutputRoleCatalog::new(
        id(1000),
        [(9, "refit"), (2, "retained"), (7, "inactive")]
            .into_iter()
            .map(|(tag, key)| {
                EdgeOutputRoleDefinition::new(
                    LocalTag::new(tag),
                    MachineToken::new(key).expect("key"),
                )
            })
            .collect(),
        vec![LocalTag::new(2), LocalTag::new(9)],
    );
    let limits = EdgeOutputRoleCatalogLimits::new(3, 2, 21);
    let catalog = raw.admit(limits).expect("whole catalogue");
    let roles = copies
        .bind_edge_roles(&catalog, limits)
        .expect("actual operation");
    run(roles);
}
