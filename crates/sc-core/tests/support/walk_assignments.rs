//! Complete original A/B role attribution over the actual supplied native walk context.
#[path = "walk_roles.rs"]
pub mod roles;
use roles::copies::input::id;
use sc_core::{construction::*, ontology::*};
pub fn with_assignments(run: impl FnOnce(AdmittedWalkNotchAssignments<'_>)) {
    with_assignment_pairs(1, run);
}
pub fn with_assignment_pairs(pairs: usize, run: impl FnOnce(AdmittedWalkNotchAssignments<'_>)) {
    roles::with_roles(pairs, &[2, 9], |roles| {
        let raw = WalkNotchAssignmentManifest::new(
            id(10),
            (0..pairs)
                .flat_map(|pair| {
                    [
                        WalkNotchRoleAssignment::new(pair, SeamSideId::A, LocalTag::new(2)),
                        WalkNotchRoleAssignment::new(pair, SeamSideId::B, LocalTag::new(9)),
                    ]
                })
                .collect(),
        );
        let assignments = raw
            .admit(
                &roles,
                WalkNotchAssignmentLimits::new(pairs, 2 * pairs, 4 * pairs),
            )
            .expect("complete original occurrences");
        run(assignments);
    });
}
