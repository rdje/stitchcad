//! Canonical construction intents preserve references without inventing geometry or profile values.
#![allow(clippy::expect_used)]
use sc_core::{
    command::{
        AddNotch, CloseDart, CommandActorClaim as Actor, CommandId, CommandIdempotencyKey as Key,
        CommandMutation, CommandMutationContext as Context, DesignRevision, MutationPayload,
        SplitSeamSpan,
    },
    ontology::{
        DeterministicIdGenerator, EdgeAnchor, EdgeRef, EntityId as Id, IdGenerator, LocalTag,
        NotchDimensionBindings, NotchProfileBindings, Param, ProfileParameterRef as Profile,
        Rational, SeamSideId,
    },
};

fn context() -> Context {
    Context::new(
        Id::from_bits(1),
        Actor::Human(Id::from_bits(2)),
        DesignRevision::initial(),
        Key::from_entity_id(Id::from_bits(3)),
        None,
    )
}

#[test]
fn paired_span_split_preserves_both_original_frames_and_its_canonical_target() {
    let side_a = EdgeAnchor {
        edge: EdgeRef::new(Id::from_bits(71), LocalTag::FIRST),
        param: Param::new(Rational::new(2, 3).expect("fraction")).expect("interior"),
    };
    let side_b = EdgeAnchor {
        edge: EdgeRef::new(Id::from_bits(72), LocalTag::FIRST),
        param: Param::new(Rational::new(1, 5).expect("fraction")).expect("interior"),
    };
    let target = Id::from_bits(u128::MAX);
    let intent = SplitSeamSpan::new(target, side_a, side_b);
    assert_eq!(intent.span(), target);
    assert_eq!(intent.cut(SeamSideId::A), side_a);
    assert_eq!(intent.cut(SeamSideId::B), side_b);
    let metadata = context();
    let request = CommandMutation::new(metadata.clone(), MutationPayload::SplitSeamSpan(intent));
    assert_eq!(request.context(), &metadata);
    assert_eq!(request.command_id(), CommandId::SplitSeamSpan);
    assert_eq!(request.payload(), &MutationPayload::SplitSeamSpan(intent));
    let swapped = SplitSeamSpan::new(target, side_b, side_a);
    assert_eq!(swapped.cut(SeamSideId::A), side_b);
    assert_eq!(swapped.cut(SeamSideId::B), side_a);
    assert_ne!(swapped, intent);
}

#[test]
fn split_intent_neither_guesses_copy_correspondence_nor_certifies_interior_cuts() {
    let edge = EdgeRef::new(Id::from_bits(71), LocalTag::FIRST);
    // Distinct physical copies can share a source edge; the actual canonical span supplies copies.
    for param in [Param::START, Param::END] {
        let held = EdgeAnchor { edge, param };
        let intent = SplitSeamSpan::new(Id::from_bits(73), held, held);
        assert_eq!(intent.cut(SeamSideId::A), held);
        assert_eq!(intent.cut(SeamSideId::B), held);
    }
    // Preserving authored input is not proof of the actual span's strict interior or ownership.
}

#[test]
fn dart_intent_targets_canonical_content_without_a_copied_intake_or_operation() {
    let dart = Id::from_bits(u128::MAX);
    let intent = CloseDart::new(dart);
    assert_eq!(intent.dart(), dart);
    let metadata = context();
    let request = CommandMutation::new(metadata.clone(), MutationPayload::CloseDart(intent));
    assert_eq!(request.context(), &metadata);
    assert_eq!(request.command_id(), CommandId::CloseDart);
}

#[test]
fn notch_creation_preserves_all_bindings_and_waits_for_an_explicit_new_identity() {
    let anchor = EdgeAnchor {
        edge: EdgeRef::new(Id::from_bits(17), LocalTag::FIRST),
        param: Param::new(Rational::new(2, 3).expect("fraction")).expect("interior"),
    };
    let bindings = NotchProfileBindings {
        style: Profile::new(Id::from_bits(21)),
        sample: NotchDimensionBindings {
            depth: Profile::new(Id::from_bits(22)),
            width: Profile::new(Id::from_bits(23)),
        },
        production: NotchDimensionBindings {
            depth: Profile::new(Id::from_bits(24)),
            width: Profile::new(Id::from_bits(25)),
        },
        encoding: Profile::new(Id::from_bits(26)),
    };
    let intent = AddNotch::new(Id::from_bits(18), anchor, bindings);
    assert_eq!(intent.piece(), Id::from_bits(18));
    assert_eq!(intent.anchor(), anchor);
    assert_eq!(intent.representation(), bindings);
    let mut ids = DeterministicIdGenerator::from_start(100);
    let first_id = ids.next_id().expect("bus allocation");
    let second_id = ids.next_id().expect("next bus allocation");
    let first = intent.definition_with_id(first_id);
    let second = intent.definition_with_id(second_id);
    assert_eq!(first.id, first_id);
    assert_eq!(second.id, second_id);
    assert_ne!(first.id, second.id);
    assert_eq!((first.anchor, second.anchor), (anchor, anchor));
    assert_eq!(
        (first.representation, second.representation),
        (bindings, bindings)
    );
    let request = CommandMutation::new(context(), MutationPayload::AddNotch(intent));
    assert_eq!(request.command_id(), CommandId::AddNotch);
}
