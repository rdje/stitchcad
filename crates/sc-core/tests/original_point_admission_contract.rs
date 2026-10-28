//! Actual journal/source admission only;inverse coordinates are not produced by this primitive.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
use core::num::{NonZeroU64, NonZeroUsize};
use sc_core::ontology::{
    DeterministicIdGenerator, Direction, EdgeRef, EntityId, IdentityLedger, LocalTag,
    OriginalPointAdmission, OriginalPointAdmissionError, OriginalPointLimits, Param, Rational,
    UniquePointLimits, UniquePointOutcome,
};
fn t(n: i64, d: i64) -> Param {
    Param::new(Rational::new(n, d).unwrap()).unwrap()
}
fn limits(entries: usize, nested: usize, bits: u64) -> OriginalPointLimits {
    OriginalPointLimits::new(
        UniquePointLimits::new(
            NonZeroUsize::new(entries).unwrap(),
            NonZeroUsize::new(nested).unwrap(),
        ),
        NonZeroUsize::new(1).unwrap(),
        NonZeroU64::new(bits).unwrap(),
    )
}
fn fixture(count: usize) -> (IdentityLedger, DeterministicIdGenerator, Vec<EdgeRef>) {
    let mut ledger = IdentityLedger::new();
    let mut ids = DeterministicIdGenerator::new();
    let edges = ledger.declare_edges(&mut ids, count).unwrap();
    (ledger, ids, edges)
}

#[test]
fn successful_admission_retains_actual_creation_current_point_and_explicit_limits() {
    fn copy_only<T: Copy>() {}
    copy_only::<OriginalPointLimits>();
    copy_only::<OriginalPointAdmission<'_>>();
    let (ledger, _, edges) = fixture(2);
    let source = *edges.first().unwrap();
    let selected = limits(1, 2, 508);
    let token = ledger
        .admit_original_point(source, t(1, 4), source, selected)
        .unwrap();
    assert_eq!(
        (token.reference(), token.param(), token.source_edge()),
        (source, t(1, 4), source)
    );
    assert_eq!(
        token.source_creation(),
        ledger.journal().first().unwrap().operation()
    );
    assert_eq!(
        (
            token.current().edge(),
            token.current().param(),
            token.current().direction()
        ),
        (source, t(1, 4), Direction::Original)
    );
    assert_eq!(
        (token.component_bit_bound(), token.integer_bit_bound()),
        (190, 508)
    );
    assert_eq!(token.limits(), selected);
    assert_eq!(
        selected.current(),
        UniquePointLimits::new(NonZeroUsize::new(1).unwrap(), NonZeroUsize::new(2).unwrap())
    );
    assert_eq!(selected.candidates().get(), 1);
    assert_eq!(selected.integer_bits().get(), 508);
    let debug = format!("{token:?}");
    assert!(!debug.contains("Declared") && !debug.contains("registrations"));
    // Both are real created edges. Admission cannot claim that an inverse preimage exists.
    let unrelated = *edges.get(1).unwrap();
    assert!(ledger
        .admit_original_point(source, t(1, 4), unrelated, selected)
        .is_ok());
}

#[test]
fn reversed_same_identity_keeps_creation_provenance_and_exact_integer_refusal() {
    let (mut ledger, mut ids, edges) = fixture(1);
    let source = *edges.first().unwrap();
    let creation = ledger.journal().first().unwrap().operation();
    ledger.reverse(&mut ids, source).unwrap();
    let before = ledger.clone();
    let error = ledger
        .admit_original_point(source, t(1, 4), source, limits(2, 1, 761))
        .unwrap_err();
    assert_eq!(
        error,
        OriginalPointAdmissionError::IntegerBits {
            source,
            reference: source,
            param: t(1, 4),
            required: 762,
            limit: 761,
        }
    );
    let token = ledger
        .admit_original_point(source, t(1, 4), source, limits(2, 1, 762))
        .unwrap();
    assert_eq!(token.source_creation(), creation);
    assert_eq!(token.current().param(), t(3, 4));
    assert_eq!(token.current().direction(), Direction::Reversed);
    assert_eq!(
        (token.component_bit_bound(), token.integer_bit_bound()),
        (317, 762)
    );
    assert_eq!(ledger, before);
}

#[test]
fn actual_current_admission_and_choice_refusals_precede_unknown_target_and_integer_budget() {
    let (mut ledger, mut ids, edges) = fixture(1);
    let source = *edges.first().unwrap();
    let unknown = EdgeRef::new(EntityId::from_bits(u128::MAX), LocalTag::FIRST);
    let error = ledger
        .admit_original_point(source, t(1, 4), unknown, limits(1, 1, 508))
        .unwrap_err();
    assert_eq!(
        error,
        OriginalPointAdmissionError::UnknownSource {
            source: unknown,
            reference: source,
            param: t(1, 4)
        }
    );
    let fragments = ledger.split(&mut ids, source, t(1, 2)).unwrap();
    let quota = ledger
        .admit_original_point(source, t(1, 2), unknown, limits(1, 1, 1))
        .unwrap_err();
    let OriginalPointAdmissionError::Current {
        source: goal,
        resolution,
    } = quota
    else {
        panic!("actual current quota")
    };
    assert_eq!(goal, unknown);
    assert_eq!(
        resolution.outcome(),
        &UniquePointOutcome::JournalEntries {
            required: 2,
            limit: 1
        }
    );
    let choice = ledger
        .admit_original_point(source, t(1, 2), unknown, limits(2, 1, 1))
        .unwrap_err();
    let OriginalPointAdmissionError::Current { resolution, .. } = choice else {
        panic!("actual first choice")
    };
    assert_eq!(
        (resolution.reference(), resolution.param()),
        (source, t(1, 2))
    );
    assert!(
        matches!(resolution.outcome(), UniquePointOutcome::SplitChoice { first, second, .. }
        if *first == fragments.first() && *second == fragments.second())
    );
    let missing = ledger
        .admit_original_point(unknown, t(1, 4), source, limits(2, 1, 762))
        .unwrap_err();
    let OriginalPointAdmissionError::Current { resolution, .. } = missing else {
        panic!("actual missing held source")
    };
    assert_eq!(resolution.outcome(), &UniquePointOutcome::UnknownSource);
}

#[test]
fn genuine_current_numeric_failure_keeps_original_holder_and_standard_error_source() {
    let (mut ledger, mut ids, edges) = fixture(1);
    let source = *edges.first().unwrap();
    ledger.split(&mut ids, source, t(2, 3)).unwrap();
    let held = t(1, i64::MAX);
    let error = ledger
        .admit_original_point(source, held, source, limits(2, 1, 762))
        .unwrap_err();
    let issue = std::error::Error::source(&error)
        .unwrap()
        .downcast_ref::<sc_units::UnitError>();
    assert_eq!(
        issue,
        Some(&sc_units::UnitError::Overflow {
            operation: "Rational::checked_div"
        })
    );
    let OriginalPointAdmissionError::Current { resolution, .. } = error else {
        panic!("actual numeric repair")
    };
    assert_eq!((resolution.reference(), resolution.param()), (source, held));
}

#[test]
fn equal_forward_witnesses_do_not_cancel_the_original_reference_choice() {
    let (mut ledger, mut ids, edges) = fixture(1);
    let source = *edges.first().unwrap();
    let half = t(1, 2);
    let fragments = ledger.split(&mut ids, source, half).unwrap();
    let one = sc_units::Length::from_micrometres(1).unwrap();
    let merged = ledger
        .merge(&mut ids, fragments.first(), one, fragments.second(), one)
        .unwrap();
    let held = ledger.resolve(source, half);
    let first = held
        .clone()
        .choose(sc_core::ontology::SplitSide::First)
        .resolved()
        .unwrap();
    let second = held
        .choose(sc_core::ontology::SplitSide::Second)
        .resolved()
        .unwrap();
    assert_eq!(first, second);
    assert_eq!((first.edge(), first.param()), (merged, half));
    let selected = limits(3, 1, 1016);
    let token = ledger
        .admit_original_point(merged, half, source, selected)
        .unwrap();
    assert_eq!(
        (token.reference(), token.source_edge(), token.current()),
        (merged, source, first)
    );
    let error = ledger
        .admit_original_point(source, half, source, selected)
        .unwrap_err();
    let OriginalPointAdmissionError::Current { resolution, .. } = error else {
        panic!("original held choice")
    };
    assert!(matches!(
        resolution.outcome(),
        UniquePointOutcome::SplitChoice { .. }
    ));
}
