//! Declared mutation metadata retains exact identities and counter values without granting trust.
use sc_core::{
    command::{
        CommandActorClaim as Actor, CommandGroupId as Group, CommandIdempotencyKey as Key,
        CommandMutationContext as Context, DesignRevision as Revision,
    },
    ontology::EntityId as Id,
};

#[test]
fn agent_and_principal_remain_distinct_in_complete_mutation_context() {
    let human = Id::from_bits(u128::MAX);
    let agent = Id::from_bits(7);
    let design = Id::from_bits(11);
    let key = Key::from_entity_id(Id::from_bits(19));
    let group = Group::from_entity_id(Id::from_bits(23));
    let actor = Actor::Agent {
        identity: agent,
        principal: human,
    };
    let revision = Revision::from_counter(u128::MAX).successor();
    let context = Context::new(design, actor, revision.clone(), key, Some(group));
    assert_eq!(context.design(), design);
    assert_eq!(context.actor().initiator(), agent);
    assert_eq!(context.actor().principal(), human);
    assert_eq!(context.expected_revision(), &revision);
    assert_eq!(context.idempotency_key().entity_id(), Id::from_bits(19));
    assert_eq!(
        context.group().map(Group::entity_id),
        Some(Id::from_bits(23))
    );
    let standalone = Context::new(design, Actor::Human(human), revision, key, None);
    assert_eq!(
        standalone.actor().initiator(),
        standalone.actor().principal()
    );
    assert_eq!(standalone.group(), None);
    assert_eq!(
        context.group(),
        Some(group),
        "other contexts remain immutable"
    );
}

#[test]
fn revisions_are_exact_beyond_u128_without_advancing_the_original_value() {
    let initial = Revision::initial();
    assert_eq!(initial, Revision::default());
    assert_eq!(initial.to_string(), "0");
    let first = initial.successor();
    assert_eq!(first.to_string(), "1");
    assert_eq!(initial.to_string(), "0");
    let max = Revision::from_counter(u128::MAX);
    let next = max.successor();
    assert_eq!(max.to_string(), u128::MAX.to_string());
    assert_eq!(next.to_string(), "340282366920938463463374607431768211456");
    assert_eq!(
        next.successor().to_string(),
        "340282366920938463463374607431768211457"
    );
    assert!(initial < first && first < max && max < next);
}
