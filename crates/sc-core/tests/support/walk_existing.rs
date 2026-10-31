//! Actual whole supplied existing objects over complete original native attribution.
#[path = "walk_assignments.rs"]
pub mod assignments;
use assignments::roles::copies::input::{fixture, id};
use sc_core::{construction::*, ontology::*};
pub fn supplied(source: &AdmittedWalkNotchAssignments<'_>) -> Vec<Notch> {
    let f = fixture();
    [SeamSideId::B, SeamSideId::A]
        .into_iter()
        .map(|side| {
            let item = source.assignment(0, side).expect("actual occurrence");
            Notch::new(
                NotchDefinition {
                    id: if side == SeamSideId::A {
                        id(700)
                    } else {
                        id(701)
                    },
                    anchor: *item.requested().anchor(),
                    representation: *item.requested().representation(),
                },
                item.piece(),
                &f.ledger,
            )
            .expect("actual born Notch")
        })
        .collect()
}
pub fn with_existing(run: impl FnOnce(BoundWalkExistingNotches<'_>)) {
    assignments::with_assignments(|assignments| {
        let notches = supplied(&assignments);
        let existing = assignments
            .bind_existing_notches(&notches, WalkExistingNotchLimits::new(2))
            .expect("complete provided objects");
        run(existing);
    });
}
