//! Actual complete source/existing objects plus scoped identity-reference exclusions.
#[path = "walk_existing.rs"]
pub mod existing;
use sc_core::construction::*;
pub fn with_known(pairs: usize, run: impl FnOnce(BoundWalkNotchIdentityExclusions<'_>)) {
    let bind = |existing: BoundWalkExistingNotches<'_>| {
        let limits = WalkNotchIdentityExclusionLimits::new(
            2,
            2,
            0,
            3,
            pairs,
            2,
            2 + 4 + 3 + 14 * pairs + 18 + 3,
        );
        let known = existing
            .bind_identity_exclusions(&limits)
            .expect("complete inspected reference scope");
        run(known);
    };
    if pairs == 1 {
        existing::with_existing(bind)
    } else {
        existing::with_existing_pairs(pairs, bind)
    }
}
