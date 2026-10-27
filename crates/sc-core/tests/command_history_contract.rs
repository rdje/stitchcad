//! Current operator and original authoring data remain distinct before runtime history validation.
#![allow(clippy::expect_used)]
use sc_core::{
    command::{
        CloseDart, CommandActorClaim as Actor, CommandGroupId,
        CommandHistoryDirection as Direction, CommandHistoryRequest as Request,
        CommandHistoryRequestError as Error, CommandIdempotencyKey, CommandMutation,
        CommandMutationContext as Context, CommandMutationGroup, DesignRevision, MutationPayload,
    },
    ontology::EntityId,
};

fn id(value: u128) -> EntityId {
    EntityId::from_bits(value)
}
fn group_id() -> CommandGroupId {
    CommandGroupId::from_entity_id(id(3))
}
fn author() -> Actor {
    Actor::Agent {
        identity: id(4),
        principal: id(5),
    }
}
fn revision() -> DesignRevision {
    DesignRevision::from_counter(u128::MAX).successor()
}
fn context(
    actor: Actor,
    revision: DesignRevision,
    group: Option<CommandGroupId>,
    design: EntityId,
) -> Context {
    Context::new(
        design,
        actor,
        revision,
        CommandIdempotencyKey::from_entity_id(id(6)),
        group,
    )
}
fn original() -> CommandMutationGroup {
    let context = context(author(), revision(), Some(group_id()), id(1));
    CommandMutationGroup::new(
        group_id(),
        vec![
            CommandMutation::new(
                context.clone(),
                MutationPayload::CloseDart(CloseDart::new(id(9))),
            ),
            CommandMutation::new(context, MutationPayload::CloseDart(CloseDart::new(id(7)))),
        ],
    )
    .expect("exact original group, including repeated key as submitted")
}

#[test]
fn history_keeps_current_operator_revision_and_key_separate_from_original_author_and_members() {
    let original = original();
    let operator = context(
        Actor::Human(id(5)),
        revision().successor(),
        Some(group_id()),
        id(1),
    );
    for direction in [Direction::Undo, Direction::Redo] {
        let request = Request::new(operator.clone(), original.clone(), direction)
            .expect("same Design/target with a distinct current operator");
        assert_eq!(request.context(), &operator);
        assert_eq!(request.original(), &original);
        assert_eq!(request.original_actor(), author());
        assert_ne!(request.original_actor(), request.context().actor());
        assert_eq!(request.direction(), direction);
        assert_eq!(request.original().mutations().len(), 2);
        assert_eq!(
            request.original().context().expected_revision(),
            &revision()
        );
        assert_eq!(
            request.context().expected_revision(),
            &revision().successor()
        );
        assert_eq!(
            request
                .original()
                .mutations()
                .first()
                .expect("first original target")
                .payload(),
            &MutationPayload::CloseDart(CloseDart::new(id(9)))
        );
    }
}

#[test]
fn history_requires_explicit_original_group_membership_before_design_consistency() {
    for requested in [None, Some(CommandGroupId::from_entity_id(id(99)))] {
        let operator = context(
            Actor::Human(id(5)),
            revision().successor(),
            requested,
            id(99),
        );
        assert_eq!(
            Request::new(operator, original(), Direction::Undo),
            Err(Error::GroupMismatch {
                expected: group_id(),
                requested
            })
        );
    }
    let operator = context(Actor::Human(id(5)), revision(), Some(group_id()), id(99));
    assert_eq!(
        Request::new(operator, original(), Direction::Redo),
        Err(Error::DesignMismatch {
            expected: id(1),
            requested: id(99)
        })
    );
}

#[test]
fn history_schema_preserves_untrusted_operator_claims_and_does_not_bind_currentness_or_replay() {
    let original = original();
    let context = context(
        Actor::Agent {
            identity: id(8),
            principal: id(5),
        },
        DesignRevision::initial(),
        Some(group_id()),
        id(1),
    );
    let request = Request::new(context.clone(), original.clone(), Direction::Undo)
        .expect("schema preserves a claim even when it predates original revision");
    assert_eq!(request.context(), &context);
    assert_eq!(request.original(), &original);
    let redo = Request::new(context, original, Direction::Redo)
        .expect("a different direction is a different intent even under the same submitted key");
    assert_ne!(request, redo);
    assert_eq!(
        request.context().idempotency_key(),
        redo.context().idempotency_key()
    );
}
