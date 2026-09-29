//! Conformance requirements from `docs/book/src/spec/units-and-tolerances.md` §9, as executable
//! tests.
//!
//! These are **property-style** tests with a recorded seed and no dependencies, for three reasons:
//! the crate must stay dependency-free to serve the `wasm-viewer` profile; a recorded seed makes a
//! failure reproducible rather than intermittent (roadmap §6.3 — constrained-random exploration in
//! testing, deterministic values in production); and the same suite must run on every platform the
//! cross-regression matrix covers (gate G6).
//!
//! Each test names the spec clause it discharges.

//!
//! The workspace denies `clippy::panic` because production code must return a diagnostic rather
//! than abort a session. A test is the opposite case: a test that cannot fail loudly protects
//! nothing. So the lint is allowed in this file only.

#![allow(clippy::panic)]

use sc_units::{
    Angle, Area, Count, Length, Ratio, Tolerance, ToleranceClass, Unit, UnitError, MAX_LENGTH_UM,
    MICRODEGREES_PER_TURN,
};

/// The seed every randomized case below is generated from. Recorded, per roadmap §6.3.
const SEED: u64 = 0x57_49_54_43_48_43_41_44; // "STITCHCAD"

/// A deterministic xorshift64* generator: no dependency, reproducible, and good enough to explore
/// the input space of a conversion. It is *not* used for anything that reaches production output.
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

    /// A signed value inside the declared length domain.
    fn length_um(&mut self) -> i64 {
        let magnitude = (self.next_u64() % (MAX_LENGTH_UM as u64)) as i64;
        if self.next_u64().is_multiple_of(2) {
            magnitude
        } else {
            -magnitude
        }
    }

    /// A small nonzero denominator, so rationals stay readable and products stay bounded.
    fn denominator(&mut self) -> i64 {
        i64::try_from(self.next_u64() % 999 + 1).unwrap_or(1)
    }
}

const CASES: usize = 5_000;

/// §9 conversion round-trip: for every unit whose quantum is a whole number of micrometres, a whole
/// number of that unit survives the trip unchanged.
#[test]
fn conversion_round_trips_for_integral_units() {
    let mut rng = Rng(SEED);
    let units = [
        Unit::Micrometre,
        Unit::Millimetre,
        Unit::Centimetre,
        Unit::Metre,
        Unit::Inch,
        Unit::HpglPlotterUnit,
    ];
    for _ in 0..CASES {
        for unit in units {
            assert!(
                unit.is_integral_in_micrometres(),
                "{unit:?} should be integral"
            );
            // A whole number of this unit, kept small enough that the metre case stays in-domain.
            let value = rng.denominator().min(900);
            let length = Length::from_rational(value, 1, unit)
                .unwrap_or_else(|e| panic!("{unit:?} x {value} should convert: {e}"));
            let (num, den) = length
                .as_rational_in(unit)
                .unwrap_or_else(|e| panic!("{unit:?} should re-express exactly: {e}"));
            assert_eq!(den, 1, "{unit:?} round trip should be a whole number");
            assert_eq!(
                num, value,
                "{unit:?}: {value} did not survive the round trip (got {num})"
            );
        }
    }
}

/// §9 conversion round-trip, second clause: where the display quantum is coarser than 1 µm the round
/// trip is within that quantum — and the test says so rather than pretending it is exact. The PDF
/// point is 25 400/72 µm, so a whole point re-expressed as points must be within one point.
#[test]
fn pdf_point_round_trip_is_within_its_quantum_and_declares_it() {
    let mut rng = Rng(SEED);
    for _ in 0..CASES {
        let points = rng.denominator().min(2_000_000);
        let Ok(length) = Length::from_rational(points, 1, Unit::PdfPoint) else {
            continue; // outside the declared domain: exercised by the domain test below
        };
        let back = length.as_f64_in(Unit::PdfPoint);
        #[allow(clippy::cast_precision_loss)]
        // boundary: comparing against a float the format demands
        let expected = points as f64;
        let deviation = (back - expected).abs();
        // The honest bound is the INTERNAL quantum expressed in the format's unit: one micrometre is
        // 72/25400 of a point, so a whole number of points can come back only within that. Claiming
        // 1e-6 of a point would be claiming precision the representation cannot hold (spec §2.1).
        let one_um_in_points = 72.0 / 25_400.0;
        assert!(
            deviation <= one_um_in_points,
            "a whole point must come back within one internal quantum ({one_um_in_points} pt), \
             got {points} -> {back} (deviation {deviation})"
        );
    }
}

/// §2 rounding: half away from zero is symmetric about zero, so `round(-x) == -round(x)`.
#[test]
fn rounding_is_symmetric_about_zero() {
    let mut rng = Rng(SEED ^ 0x1);
    for _ in 0..CASES {
        let numerator = rng.length_um();
        let denominator = rng.denominator();
        let positive = Length::from_rational(numerator.abs(), denominator, Unit::Micrometre);
        let negative = Length::from_rational(-numerator.abs(), denominator, Unit::Micrometre);
        match (positive, negative) {
            (Ok(p), Ok(n)) => assert_eq!(
                p.as_micrometres(),
                -n.as_micrometres(),
                "rounding is not symmetric for {numerator}/{denominator}"
            ),
            (Err(a), Err(b)) => assert_eq!(a, b, "the same magnitude must fail the same way"),
            (p, n) => panic!("asymmetric outcome: {p:?} vs {n:?}"),
        }
    }
}

/// §2 and §9: mirroring then rounding equals rounding then mirroring, within T1. Mirroring a length
/// is negation, so this is the property a mirrored pattern piece depends on.
#[test]
fn mirror_and_round_commute() {
    let t1 = Tolerance::numerical().unwrap();
    let mut rng = Rng(SEED ^ 0x2);
    for _ in 0..CASES {
        let numerator = rng.length_um();
        let denominator = rng.denominator();
        let rounded_then_mirrored =
            Length::from_rational(numerator, denominator, Unit::Micrometre).map(|l| -l);
        let mirrored_then_rounded =
            Length::from_rational(-numerator, denominator, Unit::Micrometre);
        match (rounded_then_mirrored, mirrored_then_rounded) {
            (Ok(a), Ok(b)) => assert!(
                a.eq_within(b, t1),
                "{a} vs {b} from {numerator}/{denominator}"
            ),
            (Err(a), Err(b)) => assert_eq!(a, b),
            (a, b) => panic!("asymmetric outcome: {a:?} vs {b:?}"),
        }
    }
}

/// §2: a single multiply-then-divide is exact for the ratios that matter, so a quarter inch is
/// exactly 6 350 µm and a half metre is exactly 500 000 µm.
#[test]
fn exact_conversions_have_no_drift() {
    assert_eq!(
        Length::from_rational(1, 4, Unit::Inch)
            .unwrap()
            .as_micrometres(),
        6_350
    );
    assert_eq!(
        Length::from_rational(3, 4, Unit::Inch)
            .unwrap()
            .as_micrometres(),
        19_050
    );
    assert_eq!(
        Length::from_rational(1, 2, Unit::Metre)
            .unwrap()
            .as_micrometres(),
        500_000
    );
    assert_eq!(
        Length::from_rational(1, 1, Unit::HpglPlotterUnit)
            .unwrap()
            .as_micrometres(),
        25
    );
    assert_eq!(
        Length::from_rational(1016, 1, Unit::HpglPlotterUnit)
            .unwrap()
            .as_micrometres(),
        25_400
    );
    assert_eq!(
        Length::from_rational(-5, 8, Unit::Inch)
            .unwrap()
            .as_micrometres(),
        -15_875
    );
}

/// §1.1 and §9: a value beyond the declared domain is a typed diagnostic, never a clamp, wrap or
/// saturation.
#[test]
fn out_of_domain_values_are_diagnostics() {
    let err = Length::from_micrometres(MAX_LENGTH_UM + 1).unwrap_err();
    match err {
        UnitError::DomainExceeded { kind, value, limit } => {
            assert_eq!(kind, "length");
            assert_eq!(value, i128::from(MAX_LENGTH_UM) + 1);
            assert_eq!(limit, i128::from(MAX_LENGTH_UM));
        }
        other => panic!("expected DomainExceeded, got {other:?}"),
    }
    assert_eq!(
        Length::from_micrometres(-MAX_LENGTH_UM - 1).unwrap_err(),
        UnitError::DomainExceeded {
            kind: "length",
            value: -i128::from(MAX_LENGTH_UM) - 1,
            limit: i128::from(MAX_LENGTH_UM),
        }
    );
    // The limit itself is inside the domain (the bound is inclusive).
    assert!(Length::from_micrometres(MAX_LENGTH_UM).is_ok());
}

/// §1.1 and §9: overflow is a diagnostic. A kilometre-long length doubled leaves the domain, and
/// multiplying two large lengths leaves the area domain — neither may wrap.
#[test]
fn overflow_is_a_diagnostic_not_a_wrap() {
    let big = Length::from_micrometres(MAX_LENGTH_UM).unwrap();
    assert!(matches!(
        big.checked_add(big).unwrap_err(),
        UnitError::DomainExceeded { kind: "length", .. }
    ));
    assert!(matches!(
        big.checked_mul_i64(2).unwrap_err(),
        UnitError::DomainExceeded { kind: "length", .. }
    ));
    // 1 km x 1 km = 1e18 µm² is exactly the area limit; one micrometre more is not.
    assert!(big.checked_area(big).is_ok());
    let over = Length::from_micrometres(MAX_LENGTH_UM).unwrap();
    let area_limit = Area::from_square_micrometres(
        i128::from(over.as_micrometres()) * i128::from(over.as_micrometres()) + 1,
    );
    assert!(matches!(
        area_limit.unwrap_err(),
        UnitError::DomainExceeded { kind: "area", .. }
    ));
}

/// §9: division by zero is a diagnostic at every entry point that divides.
#[test]
fn division_by_zero_is_a_diagnostic() {
    assert_eq!(
        Length::from_rational(1, 0, Unit::Millimetre).unwrap_err(),
        UnitError::DivisionByZero {
            operation: "Unit::to_micrometres"
        }
    );
    let one = Length::from_micrometres(1_000).unwrap();
    assert_eq!(
        one.checked_div_i64(0).unwrap_err(),
        UnitError::DivisionByZero {
            operation: "Length::checked_div_i64"
        }
    );
    assert_eq!(
        Angle::from_degrees_rational(45, 0).unwrap_err(),
        UnitError::DivisionByZero {
            operation: "Angle::from_degrees_rational"
        }
    );
    assert_eq!(
        Ratio::from_percent(2, 0).unwrap_err(),
        UnitError::DivisionByZero {
            operation: "Ratio::from_percent"
        }
    );
}

/// §1 and §9: a non-finite float at a boundary is rejected rather than propagated.
#[test]
fn non_finite_floats_are_rejected_at_the_boundary() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(
            Length::from_f64_in(value, Unit::Millimetre).unwrap_err(),
            UnitError::NonFinite {
                operation: "Length::from_f64_in"
            }
        );
    }
    // A finite value at the same boundary is accepted, so the test is not refusing everything.
    assert_eq!(
        Length::from_f64_in(10.0, Unit::Millimetre)
            .unwrap()
            .as_micrometres(),
        10_000
    );
}

/// §9: integer arithmetic inside the domain is exact — no representation error accumulates over a
/// long chain, which is the property a re-evaluated recipe depends on.
#[test]
fn repeated_addition_is_exact() {
    let step = Length::from_micrometres(1).unwrap();
    let mut total = Length::ZERO;
    for _ in 0..1_000_000 {
        total = total.checked_add(step).unwrap();
    }
    assert_eq!(total.as_micrometres(), 1_000_000);
    // And the same chain backwards returns exactly to zero.
    let mut back = total;
    for _ in 0..1_000_000 {
        back = back.checked_sub(step).unwrap();
    }
    assert_eq!(back, Length::ZERO);
}

/// §1.2: angles are normalized to `[0, 360°)`, normalization is idempotent, and whole turns are
/// identity.
#[test]
fn angles_normalize_and_whole_turns_are_identity() {
    let mut rng = Rng(SEED ^ 0x3);
    for _ in 0..CASES {
        let raw = rng.length_um(); // any signed i64 in ±1e9 microdegrees
        let angle = Angle::from_microdegrees(raw);
        let micro = angle.as_microdegrees();
        assert!(
            (0..MICRODEGREES_PER_TURN).contains(&micro),
            "{micro} not normalized"
        );
        assert_eq!(
            Angle::from_microdegrees(micro),
            angle,
            "normalization is not idempotent"
        );
        assert_eq!(
            angle + Angle::from_microdegrees(MICRODEGREES_PER_TURN),
            angle,
            "a whole turn must be identity"
        );
    }
}

/// §1.2: rational degrees are exact, so the common drafting angles carry no approximation.
#[test]
fn common_angles_are_exact() {
    assert_eq!(
        Angle::from_degrees_rational(45, 1)
            .unwrap()
            .as_microdegrees(),
        45_000_000
    );
    assert_eq!(Angle::from_degrees_rational(90, 1).unwrap(), Angle::RIGHT);
    assert_eq!(
        Angle::from_degrees_rational(1, 2)
            .unwrap()
            .as_microdegrees(),
        500_000
    );
    assert_eq!(
        Angle::from_degrees_rational(1, 3)
            .unwrap()
            .as_microdegrees(),
        333_333
    );
    // Half-way rounding, away from zero, in the angular domain too.
    assert_eq!(
        Angle::from_degrees_rational(-1, 3)
            .unwrap()
            .as_microdegrees(),
        359_666_667
    );
}

/// §1.2: `difference` reports the smallest turn, so a grainline at 359.9° is 0.1° from vertical and
/// not 359.9° from it.
#[test]
fn angle_difference_is_the_smallest_turn() {
    let vertical = Angle::ZERO;
    let almost_turn = Angle::from_degrees_rational(3599, 10).unwrap(); // 359.9°
    assert_eq!(almost_turn.difference(vertical).as_microdegrees(), 100_000);
    assert_eq!(
        Angle::RIGHT.difference(vertical).as_microdegrees(),
        90_000_000
    );
    assert_eq!(
        Angle::STRAIGHT.difference(vertical).as_microdegrees(),
        180_000_000
    );
    // And it is symmetric.
    assert_eq!(
        vertical.difference(almost_turn),
        almost_turn.difference(vertical)
    );
}

/// §3: a tolerance is inclusive at its limit, and a deviation one quantum beyond it fails.
#[test]
fn tolerance_is_inclusive_at_its_limit() {
    let t1 = Tolerance::numerical().unwrap();
    let a = Length::from_micrometres(10_000).unwrap();
    let b = Length::from_micrometres(10_001).unwrap();
    let c = Length::from_micrometres(10_002).unwrap();
    assert!(t1.accepts(a, b), "a deviation equal to the limit must pass");
    assert!(!t1.accepts(a, c), "a deviation beyond the limit must fail");
    assert!(t1.accepts_deviation(Length::from_micrometres(-1).unwrap()));
}

/// §3: every tolerance carries the requirement it was derived from, and one without a derivation
/// cannot be constructed. This is the mechanical form of "a value chosen to make a test pass is not a
/// tolerance".
#[test]
fn a_tolerance_without_a_derivation_cannot_exist() {
    let limit = Length::from_micrometres(50).unwrap();
    assert_eq!(
        Tolerance::new(ToleranceClass::FormatQuantization, limit, "").unwrap_err(),
        UnitError::EmptyDerivation
    );
    assert_eq!(
        Tolerance::new(ToleranceClass::FormatQuantization, limit, "   ").unwrap_err(),
        UnitError::EmptyDerivation
    );
    let honest = Tolerance::new(
        ToleranceClass::FormatQuantization,
        limit,
        "the receiver quantizes to 0.05 mm (profile acme-cmt-pt, evidence ev:x)",
    )
    .unwrap();
    assert!(
        honest.derivation().contains("receiver"),
        "derivation must be readable"
    );
}

/// §3 and §9 class separation: using the wrong class produces the wrong verdict, which is what makes
/// the classes load-bearing rather than decorative. A 50 µm deviation is a bug under T1, acceptable
/// under the chordal class.
#[test]
fn the_classes_disagree_so_they_are_load_bearing() {
    let deviation = Length::from_micrometres(50).unwrap();
    let zero = Length::ZERO;
    let t1 = Tolerance::numerical().unwrap();
    let chordal = Tolerance::geometric_chordal().unwrap();
    assert!(
        !t1.accepts(zero, deviation),
        "T1 must reject 50 µm — it means a bug"
    );
    assert!(
        chordal.accepts(zero, deviation),
        "the chordal class allows 50 µm"
    );
    assert_eq!(t1.limit.as_micrometres(), 1);
    assert_eq!(chordal.limit.as_micrometres(), 100);
    assert_ne!(t1.class, chordal.class);
}

/// §1.3: ratios scale a length in one explicit rounded operation — the shape shrinkage and grade
/// multipliers take.
#[test]
fn ratios_scale_lengths_exactly() {
    // A percentage value and a multiplier are different numbers; the API keeps them apart.
    let two_percent = Ratio::from_percent(2, 1).unwrap();
    assert_eq!(two_percent.as_parts_per_million(), 20_000);
    let shrink_factor = Ratio::from_rational(102, 100).unwrap();
    assert_eq!(shrink_factor.as_parts_per_million(), 1_020_000);
    let seam = Length::from_rational(1, 1, Unit::Centimetre).unwrap();
    assert_eq!(shrink_factor.scale(seam).unwrap().as_micrometres(), 10_200);
    assert_eq!(two_percent.scale(seam).unwrap().as_micrometres(), 200);
    // Confusing them is caught by the numbers, which is why the test states both.
    assert_ne!(two_percent, shrink_factor);
    assert_eq!(Ratio::UNITY.scale(seam).unwrap(), seam);
    // Half-way rounding, away from zero, in the ratio domain too.
    let half = Ratio::from_rational(1, 2).unwrap();
    assert_eq!(
        half.scale(Length::from_micrometres(1).unwrap())
            .unwrap()
            .as_micrometres(),
        1
    );
    assert_eq!(
        half.scale(Length::from_micrometres(-1).unwrap())
            .unwrap()
            .as_micrometres(),
        -1
    );
}

/// §1.3: a count is not a length. This test is compile-time evidence as much as runtime: the
/// assertion below would not type-check if `Count` could be added to `Length`.
#[test]
fn counts_are_their_own_dimension() {
    let pieces = Count::new(4);
    let plies = Count::new(2);
    assert_eq!(pieces.get() + plies.get(), 6);
    assert_eq!(pieces.to_string(), "4");
    // There is deliberately no `Length + Count`: uncommenting the next line must fail to compile.
    // let _ = Length::ZERO + pieces;
}

/// §2.1: a format whose quantum is not an integer number of micrometres is declared as such, so a
/// writer knows it must publish its quantization.
#[test]
fn formats_with_a_non_integral_quantum_are_identifiable() {
    assert!(!Unit::PdfPoint.is_integral_in_micrometres());
    assert_eq!(Unit::PdfPoint.ratio_to_micrometres(), (25_400, 72));
    for unit in [
        Unit::Micrometre,
        Unit::Millimetre,
        Unit::Centimetre,
        Unit::Metre,
        Unit::Inch,
        Unit::HpglPlotterUnit,
    ] {
        assert!(
            unit.is_integral_in_micrometres(),
            "{unit:?} should be integral"
        );
        assert_eq!(unit.ratio_to_micrometres().1, 1);
    }
}

/// §2: the exact rational form a serializer reads before quantizing carries no rounding of its own.
#[test]
fn the_exact_rational_form_does_not_round() {
    let length = Length::from_micrometres(1).unwrap();
    let (num, den) = length.as_rational_in(Unit::PdfPoint).unwrap();
    // 1 µm is exactly 72/25400 of a point, returned reduced (gcd 8) — reduction is not rounding.
    assert_eq!(
        (num, den),
        (9, 3_175),
        "1 µm must be exactly 72/25400 = 9/3175 of a point"
    );
    assert_eq!(
        num as i128 * 25_400,
        den as i128 * 72,
        "the reduced form must be equal"
    );
    let inch = Length::from_rational(1, 1, Unit::Inch).unwrap();
    assert_eq!(
        inch.as_rational_in(Unit::Millimetre).unwrap(),
        (127, 5),
        "1 in = 25.4 mm exactly"
    );
}

/// §1.1: the seed is recorded so a failing case can be replayed exactly (roadmap §6.3).
#[test]
fn the_recorded_seed_is_stable() {
    let mut a = Rng(SEED);
    let mut b = Rng(SEED);
    for _ in 0..100 {
        assert_eq!(a.next_u64(), b.next_u64());
    }
    assert_eq!(SEED, 0x57_49_54_43_48_43_41_44);
}
