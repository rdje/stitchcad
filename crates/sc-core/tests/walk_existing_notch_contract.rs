//! Whole supplied-object membership only;existing Notches keep canonical ids,owners and held fields.
#![allow(clippy::expect_used)]
#[path = "support/walk_assignments.rs"]
mod common;
use common::{
    roles::copies::input::{fixture, id},
    with_assignments,
};
use sc_core::{construction::*, ontology::*};
fn supplied(assignments: &AdmittedWalkNotchAssignments<'_>) -> Vec<Notch> {
    let f = fixture();
    [SeamSideId::B, SeamSideId::A]
        .into_iter()
        .map(|side| {
            let item = assignments
                .assignment(0, side)
                .expect("original occurrence");
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
            .expect("actual canonical born Notch")
        })
        .collect()
}
#[test]
fn actual_objects_keep_original_order_semantic_identity_fields_and_complete_context() {
    with_assignments(|assignments| {
        let notches = supplied(&assignments);
        let snapshot = notches.clone();
        let limits = WalkExistingNotchLimits::new(2);
        let bound = assignments
            .bind_existing_notches(&notches, limits)
            .expect("whole actual supplied collection");
        assert!(std::ptr::eq(bound.assignments(), &assignments));
        assert!(std::ptr::eq(bound.notches(), notches.as_slice()));
        assert_eq!(bound.limits(), limits);
        assert_eq!(limits.notches(), 2);
        assert_eq!(bound.indexed_count(), 2);
        assert_eq!(
            bound.notches().iter().map(Notch::id).collect::<Vec<_>>(),
            vec![id(701), id(700)]
        );
        for notch in &notches {
            let actual = bound.notch(notch.id()).expect("actual indexed object");
            assert!(std::ptr::eq(actual, notch));
            assert_eq!(actual.piece(), notch.piece());
            assert_eq!(actual.definition(), notch.definition());
            assert_eq!(
                actual.profile_binding_validation(),
                ProfileBindingValidation::DeferredToG4
            );
        }
        assert_eq!(
            bound.notch(id(700)).expect("by id,not ordinal").piece(),
            id(100)
        );
        assert_eq!(bound.notch(id(701)).expect("B").piece(), id(101));
        assert!(bound.notch(id(999)).is_none());
        assert_eq!(notches, snapshot);
        assert_eq!(bound.assignments().request_count(), 2);
    });
}
#[test]
fn complete_quota_precedes_duplicate_and_errors_keep_whole_actual_objects_and_policy() {
    use WalkExistingNotchRefusal as R;
    with_assignments(|assignments| {
        let mut notches = supplied(&assignments);
        notches.push(notches.first().expect("original first").clone());
        let snapshot = notches.clone();
        for (limits, expected) in [
            (
                WalkExistingNotchLimits::new(2),
                R::Notches {
                    required: 3,
                    allowed: 2,
                },
            ),
            (
                WalkExistingNotchLimits::new(3),
                R::DuplicateId {
                    first: 0,
                    second: 2,
                    id: id(701),
                },
            ),
        ] {
            let error = assignments
                .bind_existing_notches(&notches, limits)
                .expect_err("no prefix or duplicate acceptance");
            assert!(std::ptr::eq(error.assignments(), &assignments));
            assert!(std::ptr::eq(error.notches(), notches.as_slice()));
            assert_eq!(error.limits(), limits);
            assert_eq!(error.refusal(), expected);
            assert!(error.to_string().starts_with("walk existing Notches:"));
            assert!(format!("{error:?}").contains("WalkExistingNotchError"));
            assert_eq!(notches, snapshot);
        }
        let f = fixture();
        let item = assignments.assignment(0, SeamSideId::A).expect("A");
        let conflicting = Notch::new(
            NotchDefinition {
                id: id(701),
                anchor: *item.requested().anchor(),
                representation: *item.requested().representation(),
            },
            item.piece(),
            &f.ledger,
        )
        .expect("same id,different canonical owner/fields");
        let records = vec![notches.first().expect("B").clone(), conflicting];
        assert_eq!(
            assignments
                .bind_existing_notches(&records, WalkExistingNotchLimits::new(2))
                .expect_err("id uniqueness independent of object equality")
                .refusal(),
            R::DuplicateId {
                first: 0,
                second: 1,
                id: id(701)
            }
        );
    });
}
#[test]
fn empty_supplied_collection_preserves_all_original_requests_without_global_absence_or_freshness_claim(
) {
    with_assignments(|assignments| {
        let notches = [];
        let bound = assignments
            .bind_existing_notches(&notches, WalkExistingNotchLimits::new(0))
            .expect("explicit empty supplied collection");
        assert_eq!(bound.indexed_count(), 0);
        assert!(bound.notches().is_empty());
        assert!(bound.notch(id(700)).is_none());
        assert_eq!(bound.assignments().request_count(), 2);
        assert_eq!(bound.assignments().assignments().len(), 2);
    });
}
#[test]
fn unrelated_supplied_objects_keep_their_actual_born_owner_without_retargeting_to_a_walk_pattern() {
    with_assignments(|assignments| {
        let f = fixture();
        let mut definition = assignments
            .roles()
            .copies()
            .piece(SeamSideId::A)
            .definition()
            .clone();
        definition.id = id(600);
        let other = Piece::new(definition, &f.ledger).expect("independent actual born pattern");
        let item = assignments
            .assignment(0, SeamSideId::A)
            .expect("actual A fields");
        let unrelated = Notch::new(
            NotchDefinition {
                id: id(702),
                anchor: *item.requested().anchor(),
                representation: *item.requested().representation(),
            },
            &other,
            &f.ledger,
        )
        .expect("canonical unrelated born Notch");
        let mut notches = supplied(&assignments);
        notches.push(unrelated);
        let snapshot = notches.clone();
        let bound = assignments
            .bind_existing_notches(&notches, WalkExistingNotchLimits::new(3))
            .expect("whole provided membership only");
        let actual = bound.notch(id(702)).expect("provided unrelated object");
        assert_eq!(actual.piece(), id(600));
        assert!(bound
            .assignments()
            .roles()
            .copies()
            .inventory()
            .piece(id(600))
            .is_none());
        assert!(std::ptr::eq(
            actual,
            notches.last().expect("actual last object")
        ));
        assert_eq!(notches, snapshot);
    });
}
