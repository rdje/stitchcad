//! Actual public ledger and semantic-source boundaries;no slash geometry or repair authority.
#![allow(clippy::expect_used)]
use sc_core::ontology::*;
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn parameter(n: i64, d: i64) -> Param {
    Param::new(Rational::new(n, d).expect("rational")).expect("unit interval")
}
fn profile() -> NotchProfileBindings {
    let dimensions = |depth, width| NotchDimensionBindings {
        depth: ProfileParameterRef::new(id(depth)),
        width: ProfileParameterRef::new(id(width)),
    };
    NotchProfileBindings {
        style: ProfileParameterRef::new(id(9000)),
        sample: dimensions(9010, 9011),
        production: dimensions(9020, 9021),
        encoding: ProfileParameterRef::new(id(9030)),
    }
}
fn piece(piece_id: u128, edge: EdgeRef, ledger: &IdentityLedger) -> Piece {
    Piece::new(
        PieceDefinition {
            id: id(piece_id),
            boundary: vec![DirectedEdge {
                edge,
                direction: Direction::Original,
            }],
            holes: vec![],
            construction_lines: vec![],
            quantity: 1,
            mirroring: Mirroring::Single,
            cut_on_fold: false,
            fold_edges: vec![],
            cutting_side: CuttingSide::Face,
            material: MaterialAssignment::Unresolved("explicit pending selection".into()),
            layer_index: 0,
            label: LabelText {
                name: "source".into(),
                size: "M".into(),
                fabric: "woven".into(),
                colorway: "plain".into(),
            },
        },
        ledger,
    )
    .expect("actual structural Piece")
}
#[test]
fn global_delete_affects_both_owners_but_explicit_registration_repairs_are_owner_specific() {
    let mut ledger = IdentityLedger::new();
    let mut ids = DeterministicIdGenerator::new();
    let source = *ledger
        .declare_edges(&mut ids, 1)
        .expect("source declaration")
        .first()
        .expect("source");
    let at = parameter(1, 2);
    let a = id(1000);
    let b = id(2000);
    ledger.register(a, source, at).expect("owner A");
    ledger.register(b, source, at).expect("owner B");
    let original = ledger.registrations().collect::<Vec<_>>();
    assert_eq!(original.len(), 2);
    ledger.delete(&mut ids, source).expect("global edge edit");
    assert!(!ledger.is_live(source));
    assert_eq!(ledger.registrations().collect::<Vec<_>>(), original);
    assert_eq!(
        ledger
            .open_repairs()
            .iter()
            .map(|repair| repair.owner())
            .collect::<Vec<_>>(),
        vec![a, b]
    );
    assert!(matches!(
        ledger
            .resolve_registered(a, source, at)
            .expect("registration A"),
        Resolution::Unresolved(_)
    ));
    assert!(matches!(
        ledger
            .resolve_registered(b, source, at)
            .expect("registration B"),
        Resolution::Unresolved(_)
    ));
    let target = *ledger
        .declare_tagged_edges(id(10), &[LocalTag::new(9)])
        .expect("separate explicit target")
        .first()
        .expect("target");
    let selected = parameter(1, 4);
    let journal = ledger.journal().len();
    ledger
        .repoint(a, source, at, target, selected)
        .expect("explicit owner A repair");
    assert_eq!(ledger.journal().len(), journal);
    assert_eq!(
        ledger
            .open_repairs()
            .iter()
            .map(|repair| repair.owner())
            .collect::<Vec<_>>(),
        vec![b]
    );
    let point = ledger
        .resolve_registered(a, target, selected)
        .expect("new A registration")
        .resolved()
        .expect("new A resolves");
    assert_eq!((point.edge(), point.param()), (target, selected));
    assert_eq!(
        ledger
            .resolve_registered(a, source, at)
            .expect_err("old A registration removed"),
        LedgerError::UnknownRegistration {
            owner: a,
            edge: source,
            param: at
        }
    );
    ledger
        .retire(b, source, at)
        .expect("explicit withdrawal of B registration");
    assert_eq!(ledger.journal().len(), journal);
    assert_eq!(ledger.registrations().count(), 1);
    assert!(ledger.open_repairs().is_empty());
    assert!(!ledger.is_live(source));
    assert!(matches!(
        ledger.resolve(source, at),
        Resolution::Unresolved(_)
    ));
    assert_eq!(
        ledger
            .registrations()
            .next()
            .expect("only A remains")
            .owner(),
        a
    );
}
#[test]
fn source_liveness_and_ledger_resolution_do_not_establish_replacement_piece_membership() {
    let mut ledger = IdentityLedger::new();
    let mut ids = DeterministicIdGenerator::new();
    let source = *ledger
        .declare_edges(&mut ids, 1)
        .expect("source")
        .first()
        .expect("one source");
    let original = piece(100, source, &ledger);
    let other = piece(200, source, &ledger);
    let inventory = PieceInventory::new(vec![original.clone(), other.clone()])
        .expect("actual shared-source structural inventory");
    assert_eq!(inventory.pieces().len(), 2);
    let anchor = EdgeAnchor {
        edge: source,
        param: parameter(1, 2),
    };
    let a = Notch::new(
        NotchDefinition {
            id: id(1000),
            anchor,
            representation: profile(),
        },
        &original,
        &ledger,
    )
    .expect("born on first Piece");
    let b = Notch::new(
        NotchDefinition {
            id: id(2000),
            anchor,
            representation: profile(),
        },
        &other,
        &ledger,
    )
    .expect("born on other Piece");
    ledger
        .register(a.id(), source, anchor.param)
        .expect("registered A");
    ledger
        .register(b.id(), source, anchor.param)
        .expect("registered B");
    let before_a = a.clone();
    let before_b = b.clone();
    let before_registrations = ledger.registrations().collect::<Vec<_>>();
    let target = *ledger
        .declare_tagged_edges(id(10), &[LocalTag::new(9)])
        .expect("new logical target")
        .first()
        .expect("target");
    let mut definition = original.definition().clone();
    definition.boundary = vec![DirectedEdge {
        edge: target,
        direction: Direction::Original,
    }];
    let replacement =
        Piece::new(definition, &ledger).expect("detached structural replacement only");
    assert!(ledger.is_live(source));
    assert!(a.resolve(&ledger).resolved().is_some());
    assert!(b.resolve(&ledger).resolved().is_some());
    assert!(ledger.open_repairs().is_empty());
    assert_eq!(ledger.release_readiness(), ReleaseReadiness::Releasable);
    // Public birth validation is used solely as a membership diagnostic,not as a held-object repair.
    assert_eq!(
        Notch::new(a.definition().clone(), &replacement, &ledger)
            .expect_err("old anchor is outside proposed target"),
        NotchError::AnchorOutsidePiece {
            piece: original.id(),
            anchor
        }
    );
    assert!(Notch::new(b.definition().clone(), &other, &ledger).is_ok());
    assert_eq!(
        ledger.registrations().collect::<Vec<_>>(),
        before_registrations
    );
    assert_eq!(a, before_a);
    assert_eq!(b, before_b);
    assert_eq!(
        original
            .definition()
            .boundary
            .first()
            .expect("original boundary")
            .edge,
        source
    );
    assert_eq!(
        replacement
            .definition()
            .boundary
            .first()
            .expect("replacement boundary")
            .edge,
        target
    );
    // The selected parameter is explicit test caller data,not a computed slash correspondence.
    let selected = parameter(1, 4);
    ledger
        .repoint(a.id(), source, anchor.param, target, selected)
        .expect("explicit A registration repair");
    let registered = ledger
        .resolve_registered(a.id(), target, selected)
        .expect("new A registration")
        .resolved()
        .expect("registered target resolves");
    assert_eq!((registered.edge(), registered.param()), (target, selected));
    assert_eq!(a, before_a);
    assert_eq!(
        a.resolve(&ledger)
            .resolved()
            .expect("held anchor still resolves globally")
            .edge(),
        source
    );
    assert_eq!(
        Notch::new(a.definition().clone(), &replacement, &ledger)
            .expect_err("registration edit did not change held semantic anchor"),
        NotchError::AnchorOutsidePiece {
            piece: original.id(),
            anchor
        }
    );
    assert_eq!(
        ledger
            .resolve_registered(b.id(), source, anchor.param)
            .expect("B registration unchanged")
            .resolved()
            .expect("B still resolves")
            .edge(),
        source
    );
    assert_eq!(b, before_b);
}
