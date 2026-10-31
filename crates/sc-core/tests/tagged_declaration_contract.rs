//! Canonical explicit-tag declarations preserve original sources;no geometry or issuance authority.
#![allow(clippy::expect_used)]
use sc_core::ontology::*;
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn half() -> Param {
    Param::new(Rational::new(1, 2).expect("half")).expect("unit interval")
}
fn fixture() -> (IdentityLedger, EdgeRef) {
    let mut ledger = IdentityLedger::new();
    let source = *ledger
        .declare_edges(&mut DeterministicIdGenerator::new(), 1)
        .expect("source declaration")
        .first()
        .expect("one source");
    ledger
        .register(id(1000), source, half())
        .expect("original source registration");
    (ledger, source)
}
#[test]
fn explicit_canonical_tags_preserve_source_identity_registrations_and_actual_journal_order() {
    let (mut ledger, source) = fixture();
    let original = ledger.resolve(source, half());
    let registrations = ledger.registrations().collect::<Vec<_>>();
    let tags = [LocalTag::new(9), LocalTag::new(2), LocalTag::new(u32::MAX)];
    let edges = ledger
        .declare_tagged_edges(id(500), &tags)
        .expect("already named canonical operation");
    assert_eq!(
        edges,
        tags.iter()
            .map(|tag| EdgeRef::new(id(500), *tag))
            .collect::<Vec<_>>()
    );
    assert!(ledger.is_live(source));
    assert_eq!(ledger.resolve(source, half()), original);
    assert_eq!(ledger.registrations().collect::<Vec<_>>(), registrations);
    assert!(edges.iter().all(|edge| ledger.is_live(*edge)));
    let record = ledger.journal().last().expect("actual declaration");
    assert_eq!(record.operation(), id(500));
    assert!(matches!(record.edit(),TopologyEdit::Declared { edges:actual } if actual==&edges));
    assert_eq!(ledger.open_repairs().len(), 0);
}
#[test]
fn empty_complete_capacity_duplicate_tags_and_recorded_operation_refuse_before_state_changes() {
    let (mut ledger, source) = fixture();
    let oversized = vec![LocalTag::FIRST; MAX_EDGES_PER_OPERATION + 1];
    for (operation, tags, cause) in [
        (source.creator(), &[][..], LedgerError::NoEdges),
        (
            source.creator(),
            oversized.as_slice(),
            LedgerError::TooManyEdges {
                count: MAX_EDGES_PER_OPERATION + 1,
            },
        ),
        (
            source.creator(),
            &[LocalTag::new(9), LocalTag::new(2), LocalTag::new(9)][..],
            LedgerError::DuplicateLocalTag {
                tag: LocalTag::new(9),
            },
        ),
        (
            source.creator(),
            &[LocalTag::new(9)][..],
            LedgerError::OperationAlreadyRecorded {
                operation: source.creator(),
            },
        ),
    ] {
        let before = ledger.clone();
        assert_eq!(ledger.declare_tagged_edges(operation, tags), Err(cause));
        assert_eq!(ledger, before);
    }
}
#[test]
fn fresh_ledger_replay_is_identical_and_all_recorded_operations_remain_reserved_after_retirement() {
    let (mut a, source) = fixture();
    let (mut b, _) = fixture();
    let tags = [LocalTag::new(13), LocalTag::new(4)];
    let outputs = a
        .declare_tagged_edges(id(500), &tags)
        .expect("canonical outputs");
    assert_eq!(
        b.declare_tagged_edges(id(500), &tags)
            .expect("fresh replay"),
        outputs
    );
    assert_eq!(a, b);
    let mut ids = DeterministicIdGenerator::from_start(10000);
    for edge in outputs {
        a.delete(&mut ids, edge).expect("retire output");
    }
    let before = a.clone();
    assert_eq!(
        a.declare_tagged_edges(id(500), &[LocalTag::new(99)]),
        Err(LedgerError::OperationAlreadyRecorded { operation: id(500) })
    );
    assert_eq!(a, before);
    let reverse = a.reverse(&mut ids, source).expect("noncreating edit");
    let before = a.clone();
    assert_eq!(
        a.declare_tagged_edges(reverse, &[LocalTag::new(99)]),
        Err(LedgerError::OperationAlreadyRecorded { operation: reverse })
    );
    assert_eq!(a, before);
    assert!(a.is_live(source));
}
