//! Group consistency preserves exact submitted metadata without fabricating authority or execution.
#![allow(clippy::expect_used)]
use sc_core::{
    command::{
        CloseDart, CommandActorClaim as Actor, CommandGroupId as GroupId, CommandId,
        CommandIdempotencyKey as Key, CommandMutation, CommandMutationContext as Context,
        CommandMutationGroup as Group, CommandMutationGroupError as Error, DesignRevision,
        MutationPayload,
    },
    ontology::EntityId,
};
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn actor() -> Actor {
    Actor::Agent {
        identity: id(2),
        principal: id(u128::MAX),
    }
}
fn group_id() -> GroupId {
    GroupId::from_entity_id(id(3))
}
fn revision() -> DesignRevision {
    DesignRevision::from_counter(u128::MAX).successor()
}
fn context() -> Context {
    Context::new(
        id(1),
        actor(),
        revision(),
        Key::from_entity_id(id(4)),
        Some(group_id()),
    )
}
fn mutation(context: Context, dart: u128) -> CommandMutation {
    CommandMutation::new(
        context,
        MutationPayload::CloseDart(CloseDart::new(id(dart))),
    )
}

#[test]
fn group_keeps_original_order_payloads_keys_actor_and_exact_revision_without_replay() {
    let mutations = vec![mutation(context(), 9), mutation(context(), 7)];
    let saved = mutations.clone();
    let group = Group::new(group_id(), mutations).expect("consistent immutable claims");
    assert_eq!(group.id(), group_id());
    assert_eq!(group.context().design(), id(1));
    assert_eq!(group.context().actor(), actor());
    assert_eq!(group.context().expected_revision(), &revision());
    assert_eq!(group.mutations(), saved);
    assert_eq!(
        group.mutations().len(),
        2,
        "equal retry keys do not silently deduplicate submitted data"
    );
    let single =
        Group::new(group_id(), vec![mutation(context(), 5)]).expect("explicit group of one");
    assert_eq!(single.mutations().len(), 1);
}

#[test]
fn empty_and_omitted_or_wrong_membership_refuse_original_group_and_index() {
    assert_eq!(
        Group::new(group_id(), vec![]),
        Err(Error::NoMutations { group: group_id() })
    );
    for requested in [None, Some(GroupId::from_entity_id(id(99)))] {
        let inconsistent = Context::new(
            id(1),
            actor(),
            revision(),
            Key::from_entity_id(id(4)),
            requested,
        );
        assert_eq!(
            Group::new(
                group_id(),
                vec![mutation(context(), 9), mutation(inconsistent, 7)]
            ),
            Err(Error::GroupMismatch {
                index: 1,
                command: CommandId::CloseDart,
                expected: group_id(),
                requested
            })
        );
    }
}

#[test]
fn mismatched_design_and_complete_actor_claim_keep_original_expected_and_requested_values() {
    let wrong_design = Context::new(
        id(99),
        actor(),
        revision(),
        Key::from_entity_id(id(4)),
        Some(group_id()),
    );
    assert_eq!(
        Group::new(
            group_id(),
            vec![mutation(context(), 9), mutation(wrong_design, 7)]
        ),
        Err(Error::DesignMismatch {
            index: 1,
            command: CommandId::CloseDart,
            expected: id(1),
            requested: id(99)
        })
    );
    for requested in [
        Actor::Human(id(u128::MAX)),
        Actor::Agent {
            identity: id(2),
            principal: id(99),
        },
    ] {
        let changed = Context::new(
            id(1),
            requested,
            revision(),
            Key::from_entity_id(id(4)),
            Some(group_id()),
        );
        assert_eq!(
            Group::new(
                group_id(),
                vec![mutation(context(), 9), mutation(changed, 7)]
            ),
            Err(Error::ActorMismatch {
                index: 1,
                command: CommandId::CloseDart,
                expected: actor(),
                requested
            })
        );
    }
}

#[test]
fn inconsistent_beyond_u128_revision_and_first_bad_command_order_are_exact() {
    let requested = revision().successor();
    let changed = Context::new(
        id(1),
        actor(),
        requested.clone(),
        Key::from_entity_id(id(4)),
        Some(group_id()),
    );
    let later = Context::new(
        id(99),
        actor(),
        revision(),
        Key::from_entity_id(id(4)),
        Some(group_id()),
    );
    assert_eq!(
        Group::new(
            group_id(),
            vec![
                mutation(context(), 9),
                mutation(changed, 7),
                mutation(later, 8)
            ]
        ),
        Err(Error::RevisionMismatch {
            index: 1,
            command: CommandId::CloseDart,
            expected: revision(),
            requested
        })
    );
}
