//! Exhaustion is a typed refusal; no terminal allocation can wrap or mutate a ledger.
#![allow(clippy::expect_used)]
use sc_core::ontology::{
    DeterministicIdGenerator, EdgeRef, EntityId, IdError, IdGenerator, IdentityLedger, LedgerError,
    OffsetInterval, Param, Rational,
};
use sc_units::Length;
use std::error::Error;

#[test]
fn final_available_ids_are_issued_once_and_exhaustion_is_terminal() {
    for start in [u128::MAX - 1, u128::MAX] {
        let mut ids = DeterministicIdGenerator::from_start(start);
        for value in start..=u128::MAX {
            assert_eq!(ids.next_id(), Ok(EntityId::from_bits(value)));
        }
        for _ in 0..3 {
            assert_eq!(ids.next_id(), Err(IdError::Exhausted));
        }
    }
    let mut seeded_zero = DeterministicIdGenerator::from_start(0);
    assert_eq!(seeded_zero.next_id(), Ok(EntityId::from_bits(0)));
    assert_eq!(seeded_zero.next_id(), Ok(EntityId::from_bits(1)));
    assert_eq!(
        DeterministicIdGenerator::new().next_id(),
        Ok(EntityId::from_bits(1))
    );
}

#[derive(Debug)]
struct RefusingGenerator {
    calls: usize,
    error: IdError,
}
impl IdGenerator for RefusingGenerator {
    fn next_id(&mut self) -> Result<EntityId, IdError> {
        self.calls += 1;
        Err(self.error)
    }
}

#[derive(Clone, Copy, Debug)]
enum Edit {
    Declare,
    Split,
    Merge,
    Reverse,
    Delete,
    Offset,
}
const EDITS: [Edit; 6] = [
    Edit::Declare,
    Edit::Split,
    Edit::Merge,
    Edit::Reverse,
    Edit::Delete,
    Edit::Offset,
];

fn fixture() -> (IdentityLedger, [EdgeRef; 2], Param) {
    let mut ids = DeterministicIdGenerator::new();
    let mut ledger = IdentityLedger::new();
    let edges: [EdgeRef; 2] = ledger
        .declare_edges(&mut ids, 2)
        .expect("two edges")
        .try_into()
        .expect("exact pair");
    let [first, _] = edges;
    let half = Param::new(Rational::new(1, 2).expect("half")).expect("interior");
    ledger
        .register(ids.next_id().expect("owner identity"), first, half)
        .expect("registration");
    (ledger, edges, half)
}

#[test]
fn every_allocating_edit_preserves_complete_state_and_the_original_error() {
    let (before, [first, second], half) = fixture();
    let length = Length::from_micrometres(10).expect("positive length");
    let interval = OffsetInterval::new(Param::START, Param::END).expect("full interval");
    for reason in [
        IdError::Exhausted,
        IdError::TimestampOverflow { value: u64::MAX },
    ] {
        for edit in EDITS {
            let mut ledger = before.clone();
            let mut ids = RefusingGenerator {
                calls: 0,
                error: reason,
            };
            let result = match edit {
                Edit::Declare => ledger.declare_edges(&mut ids, 1).map(|_| ()),
                Edit::Split => ledger.split(&mut ids, first, half).map(|_| ()),
                Edit::Merge => ledger
                    .merge(&mut ids, first, length, second, length)
                    .map(|_| ()),
                Edit::Reverse => ledger.reverse(&mut ids, first).map(|_| ()),
                Edit::Delete => ledger.delete(&mut ids, first).map(|_| ()),
                Edit::Offset => ledger.offset(&mut ids, first, &[interval]).map(|_| ()),
            };
            let error = result.expect_err("allocator refused");
            assert_eq!(error, LedgerError::IdGeneration(reason), "{edit:?}");
            assert_eq!(
                error.source().and_then(|e| e.downcast_ref::<IdError>()),
                Some(&reason)
            );
            assert_eq!(
                ledger, before,
                "{edit:?} must preserve journal/live edges/registrations"
            );
            assert_eq!(ids.calls, 1);
        }
    }
}

#[test]
fn invalid_edits_refuse_before_consuming_an_identity() {
    let (before, [first, second], _) = fixture();
    let length = Length::from_micrometres(10).expect("positive length");
    let missing = EdgeRef::new(EntityId::from_bits(99), sc_core::ontology::LocalTag::FIRST);
    for edit in EDITS {
        let mut ledger = before.clone();
        let mut ids = RefusingGenerator {
            calls: 0,
            error: IdError::Exhausted,
        };
        let result = match edit {
            Edit::Declare => ledger.declare_edges(&mut ids, 0).map(|_| ()),
            Edit::Split => ledger.split(&mut ids, first, Param::START).map(|_| ()),
            Edit::Merge => ledger
                .merge(&mut ids, first, Length::ZERO, second, length)
                .map(|_| ()),
            Edit::Reverse => ledger.reverse(&mut ids, missing).map(|_| ()),
            Edit::Delete => ledger.delete(&mut ids, missing).map(|_| ()),
            Edit::Offset => ledger.offset(&mut ids, first, &[]).map(|_| ()),
        };
        assert!(result.is_err());
        assert_eq!(
            ids.calls, 0,
            "{edit:?}: preconditions run before allocation"
        );
        assert_eq!(ledger, before);
    }
}
