//! Complete actual native walk fields;provider requests are still separate metadata.
#[path = "walk_known.rs"]
pub mod known;
use known::existing::assignments::roles::copies::input::id;
use sc_core::{construction::*, ontology::LocalTag};
pub fn with_fields(pairs: usize, run: impl FnOnce(AdmittedWalkNotchFields<'_>)) {
    known::with_known(pairs, |known| {
        let raw_ids = WalkNotchIdentityManifest::new(
            id(10),
            vec![
                WalkNotchIdentityIntent::Retain {
                    role: LocalTag::new(2),
                    notch: id(700),
                },
                WalkNotchIdentityIntent::Propose {
                    role: LocalTag::new(9),
                    id: id(500),
                },
            ],
        );
        let ids = raw_ids
            .admit(
                &known,
                WalkNotchIdentityLimits::new(2, 2 * pairs, 6 + 2 * pairs),
            )
            .expect("whole identity proposals");
        let raw = WalkNotchFieldManifest::new(
            id(10),
            vec![
                WalkNotchFieldIntent::new(
                    LocalTag::new(9),
                    WalkNotchSharingPolicy::CoalesceIdentical,
                    WalkNotchFieldPolicy::NewRequested,
                ),
                WalkNotchFieldIntent::new(
                    LocalTag::new(2),
                    WalkNotchSharingPolicy::CoalesceIdentical,
                    WalkNotchFieldPolicy::RetainUnchanged,
                ),
            ],
        );
        let fields = raw
            .admit(&ids, WalkNotchFieldLimits::new(2, 2 * pairs, 2, 6))
            .expect("complete explicit field agreement");
        run(fields);
    });
}
