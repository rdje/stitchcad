//! Property tests for the identity layer (`G1-SLICE.3a`), as executable evidence for the acceptance
//! criteria: the parameter is exact under the four operators, an `EntityId` round-trips its canonical form
//! and orders by creation, and a deterministic generator reproduces ids byte-for-byte.
//!
//! These are hand-rolled property tests with a **recorded seed and no dependencies**, per
//! `docs/decisions/decision_property-tests-dependency-free-recorded-seed.md`: `sc-core` is on the
//! `wasm-viewer` critical path, a recorded seed makes a failure reproducible rather than intermittent
//! (roadmap §6.3), and the same suite must run on every platform the cross-regression matrix covers. Each
//! test names the property it discharges.
//!
//! The workspace denies `clippy::panic` because production code returns a diagnostic rather than aborting a
//! session; a test is the opposite case, so the lint is allowed in this file only.

#![allow(clippy::panic)]

use sc_core::ontology::{DeterministicIdGenerator, EntityId, IdGenerator, Param, Rational};

/// The seed every randomized case below is generated from. Recorded, per roadmap §6.3 — the same seed the
/// `sc-units` suite uses, so the whole repository's property tests are reproducible from one constant.
const SEED: u64 = 0x57_49_54_43_48_43_41_44; // "STITCHCAD"

/// A deterministic xorshift64* generator: no dependency, reproducible, good enough to explore the input
/// space. It is *not* used for anything that reaches production output.
struct Rng(u64);

impl Rng {
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// A value in `0..=bound`.
    fn up_to(&mut self, bound: u64) -> u64 {
        self.next_u64() % (bound + 1)
    }

    /// A small positive denominator, so arithmetic stays well inside `i64` and the exactness properties are
    /// exercised rather than the overflow diagnostic (which the unit tests cover separately).
    fn denominator(&mut self) -> i64 {
        i64::try_from(self.up_to(999) + 1).unwrap_or(1)
    }

    /// A rational in `[0, 1]`: a numerator no larger than its denominator.
    #[allow(clippy::expect_used)] // a helper, not a `#[test]` fn, so `.clippy.toml`'s allow-expect-in-tests does not reach it; the expect fails loudly if `denominator()` ever returned 0
    fn unit_rational(&mut self) -> Rational {
        let den = self.denominator();
        let num = i64::try_from(self.up_to(u64::try_from(den).unwrap_or(1))).unwrap_or(0);
        Rational::new(num, den).expect("a small non-zero denominator is always valid")
    }

    /// A signed rational of modest magnitude, so `+ − × ÷` do not overflow `i64`.
    #[allow(clippy::expect_used)] // as above: a helper, not a `#[test]` fn
    fn small_rational(&mut self) -> Rational {
        let den = self.denominator();
        let magnitude = i64::try_from(self.up_to(999)).unwrap_or(0);
        let num = if self.next_u64().is_multiple_of(2) {
            magnitude
        } else {
            -magnitude
        };
        Rational::new(num, den).expect("a small non-zero denominator is always valid")
    }
}

const CASES: usize = 5_000;

/// Addition and multiplication are commutative — the exactness the parameter recomputation relies on, so a
/// value reached by a different route is identical, not merely close.
#[test]
fn addition_and_multiplication_are_commutative() {
    let mut rng = Rng(SEED);
    for _ in 0..CASES {
        let a = rng.small_rational();
        let b = rng.small_rational();
        assert_eq!(
            a.checked_add(b).unwrap(),
            b.checked_add(a).unwrap(),
            "addition commutes for {a} and {b}"
        );
        assert_eq!(
            a.checked_mul(b).unwrap(),
            b.checked_mul(a).unwrap(),
            "multiplication commutes for {a} and {b}"
        );
    }
}

/// Subtraction inverts addition and division inverts multiplication, exactly — so a parameter moved and
/// moved back lands on the identical rational, with no accumulated rounding.
#[test]
fn subtraction_and_division_are_exact_inverses() {
    let mut rng = Rng(SEED);
    for _ in 0..CASES {
        let a = rng.small_rational();
        let b = rng.small_rational();
        assert_eq!(
            a.checked_add(b).unwrap().checked_sub(b).unwrap(),
            a,
            "({a} + {b}) - {b} == {a}"
        );
        if !b.is_zero() {
            assert_eq!(
                a.checked_mul(b).unwrap().checked_div(b).unwrap(),
                a,
                "({a} * {b}) / {b} == {a}"
            );
        }
    }
}

/// The reduced form is canonical: scaling a numerator and denominator by the same factor does not change
/// the value, which is what makes structural equality a correct identity test.
#[test]
fn reduction_is_canonical() {
    let mut rng = Rng(SEED);
    for _ in 0..CASES {
        let n = i64::try_from(rng.up_to(499) + 1).unwrap_or(1);
        let d = i64::try_from(rng.up_to(499) + 1).unwrap_or(1);
        let base = Rational::new(n, d).unwrap();
        let scaled = Rational::new(n * 7, d * 7).unwrap();
        assert_eq!(base, scaled, "{n}/{d} == {} after scaling by 7", 7 * n);
        assert_eq!(
            (base.numerator(), base.denominator()),
            (scaled.numerator(), scaled.denominator()),
            "equal values have an identical reduced form"
        );
    }
}

/// Ordering is consistent with subtraction's sign and total — so the split-point case `t == s` is decided
/// exactly, never within a tolerance.
#[test]
fn ordering_is_consistent_with_the_sign_of_the_difference() {
    let mut rng = Rng(SEED);
    for _ in 0..CASES {
        let a = rng.small_rational();
        let b = rng.small_rational();
        let diff = a.checked_sub(b).unwrap();
        match a.cmp(&b) {
            core::cmp::Ordering::Less => assert!(diff < Rational::ZERO),
            core::cmp::Ordering::Equal => assert!(diff.is_zero()),
            core::cmp::Ordering::Greater => assert!(diff > Rational::ZERO),
        }
    }
}

/// Reverse maps `t` to `1 − t` and is an involution on `[0, 1]`: reversing twice returns the identical
/// parameter, and a parameter in range stays in range (ontology §1.1).
#[test]
fn reverse_is_an_involution_that_preserves_the_unit_interval() {
    let mut rng = Rng(SEED);
    for _ in 0..CASES {
        let t = rng.unit_rational();
        assert!(t.in_unit_interval());
        let reversed = t.one_minus().unwrap();
        assert!(reversed.in_unit_interval(), "1 − {t} is still in [0, 1]");
        assert_eq!(
            reversed.one_minus().unwrap(),
            t,
            "reversing twice returns {t}"
        );
    }
}

/// Split resolves a parameter into the fragment that contains it: for `t < s` the position on the first
/// fragment is `t/s`, and for `t > s` the position on the second is `(t − s)/(1 − s)`. Both stay in
/// `[0, 1]`, and the boundary `t == s` is exactly the split point — the case §1.1 says resolves to both.
#[test]
fn split_keeps_the_resolved_parameter_in_range_and_decides_the_boundary_exactly() {
    let mut rng = Rng(SEED);
    for _ in 0..CASES {
        let t = rng.unit_rational();
        let s = rng.unit_rational();
        match t.cmp(&s) {
            core::cmp::Ordering::Less => {
                // First fragment: t/s, with s != 0 because t >= 0 and t < s.
                let local = t.checked_div(s).unwrap();
                assert!(local.in_unit_interval(), "{t}/{s} must be in [0, 1]");
            }
            core::cmp::Ordering::Greater => {
                // Second fragment: (t − s)/(1 − s), with s != 1 because t <= 1 and t > s.
                let num = t.checked_sub(s).unwrap();
                let den = s.one_minus().unwrap();
                let local = num.checked_div(den).unwrap();
                assert!(
                    local.in_unit_interval(),
                    "({t} − {s})/(1 − {s}) must be in [0, 1]"
                );
            }
            core::cmp::Ordering::Equal => {
                assert_eq!(
                    t, s,
                    "the boundary is decided exactly, not within a tolerance"
                );
            }
        }
    }
}

/// An `EntityId` round-trips its 26-character Crockford form for any 128-bit value, and the canonical text
/// orders the same way as the bytes — ULID's lexicographic sortability, which holds because the Crockford
/// alphabet is ASCII-sorted by value and the form is fixed-length, most-significant character first.
#[test]
fn an_entity_id_round_trips_and_sorts_lexicographically() {
    let mut rng = Rng(SEED);
    for _ in 0..CASES {
        let bits = (u128::from(rng.next_u64()) << 64) | u128::from(rng.next_u64());
        let id = EntityId::from_bits(bits);
        let text = id.to_string();
        assert_eq!(text.len(), 26, "the canonical form is 26 characters");
        assert_eq!(
            text.parse::<EntityId>().unwrap(),
            id,
            "{text} parses back to {id}"
        );
        // A second id: the numeric order of the bits must equal the lexicographic order of the text.
        let other_bits = (u128::from(rng.next_u64()) << 64) | u128::from(rng.next_u64());
        let other = EntityId::from_bits(other_bits);
        assert_eq!(
            id.cmp(&other),
            text.cmp(&other.to_string()),
            "byte order and canonical-text order agree for {id} and {other}"
        );
    }
}

/// A deterministic generator reproduces an identical id sequence — the replay property that makes recipe
/// re-evaluation and CLI re-run byte-identical (ontology §9, `G1-SLICE.10`).
#[test]
fn a_deterministic_generator_reproduces_its_sequence() {
    let mut a = DeterministicIdGenerator::new();
    let mut b = DeterministicIdGenerator::new();
    let mut last: Option<EntityId> = None;
    for _ in 0..CASES {
        let ia = a.next_id();
        let ib = b.next_id();
        assert_eq!(ia, ib, "two fresh generators agree");
        assert_eq!(
            ia.to_string(),
            ib.to_string(),
            "and agree byte-for-byte in canonical form"
        );
        if let Some(prev) = last {
            assert!(
                prev < ia,
                "ids are strictly increasing, so creation order is preserved"
            );
        }
        last = Some(ia);
    }
}

/// A `Param` accepts exactly the rationals in `[0, 1]` and rejects the rest at construction, so a consumer
/// never has to re-check the bound (the invariant is enforced where the value is made).
#[test]
fn a_param_accepts_exactly_the_unit_interval() {
    let mut rng = Rng(SEED);
    for _ in 0..CASES {
        let in_range = rng.unit_rational();
        assert!(Param::new(in_range).is_ok(), "{in_range} is in [0, 1]");
        // A value greater than one is rejected.
        let den = i64::try_from(rng.up_to(499) + 2).unwrap_or(2);
        let over = Rational::new(den, den - 1).unwrap(); // > 1
        assert!(
            Param::new(over).is_err(),
            "{over} is above 1 and must be rejected"
        );
        // A negative value is rejected.
        let neg = Rational::new(-1, den).unwrap();
        assert!(
            Param::new(neg).is_err(),
            "{neg} is below 0 and must be rejected"
        );
    }
}
