//! Complete original A/B role attribution over the actual supplied native walk context.
#[path = "walk_roles.rs"]
pub mod roles;
use roles::copies::input::id;
use sc_core::{construction::*, ontology::*};
pub fn with_assignments(run: impl FnOnce(AdmittedWalkNotchAssignments<'_>)) {
    roles::with_roles(1, &[2, 9], |roles| {
        let raw = WalkNotchAssignmentManifest::new(
            id(10),
            vec![
                WalkNotchRoleAssignment::new(0, SeamSideId::A, LocalTag::new(2)),
                WalkNotchRoleAssignment::new(0, SeamSideId::B, LocalTag::new(9)),
            ],
        );
        let assignments = raw
            .admit(&roles, WalkNotchAssignmentLimits::new(1, 2, 4))
            .expect("complete original occurrences");
        run(assignments);
    });
}
