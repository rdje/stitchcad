//! Query targets/context retain exact snapshots, actor/principal roles and inspect-only metadata.
use sc_core::{
    command::{
        CommandActorClaim as Actor, CommandAuthority, CommandClass, CommandId, CommandQuery,
        CommandQueryContext as Context, CommandReversibility, CommandUndoGranularity,
        DesignRevision, QueryPayload,
    },
    ontology::EntityId as Id,
};

#[test]
fn snapshot_context_keeps_exact_large_revisions_and_distinct_actor_principal_roles() {
    let initial = DesignRevision::initial();
    let beyond_machine_width = DesignRevision::from_counter(u128::MAX)
        .successor()
        .successor();
    for revision in [initial, beyond_machine_width] {
        for actor in [
            Actor::Human(Id::from_bits(2)),
            Actor::Agent {
                identity: Id::from_bits(3),
                principal: Id::from_bits(2),
            },
        ] {
            let context = Context::new(Id::from_bits(1), actor, revision.clone());
            assert_eq!(context.design(), Id::from_bits(1));
            assert_eq!(context.actor(), actor);
            assert_eq!(context.actor().principal(), Id::from_bits(2));
            assert_eq!(context.expected_revision(), &revision);
            let request = CommandQuery::new(context.clone(), QueryPayload::GetPieceList);
            assert_eq!(request.context(), &context);
            assert_eq!(request.context().expected_revision(), &revision);
            assert_ne!(request.context().expected_revision(), &revision.successor());
        }
    }
}

#[test]
fn actual_query_payload_determines_identity_and_retains_canonical_span_target() {
    let context = Context::new(
        Id::from_bits(1),
        Actor::Human(Id::from_bits(2)),
        DesignRevision::initial(),
    );
    let span = Id::from_bits(u128::MAX);
    for (payload, identity) in [
        (QueryPayload::GetPieceList, CommandId::GetPieceList),
        (QueryPayload::WalkSeamSpan { span }, CommandId::WalkSeamSpan),
    ] {
        let request = CommandQuery::new(context.clone(), payload.clone());
        assert_eq!(request.payload(), &payload);
        assert_eq!(request.context(), &context);
        assert_eq!(request.command_id(), identity);
        assert_eq!(identity.class(), CommandClass::Query);
        assert_eq!(identity.required_authority(), CommandAuthority::Inspect);
        assert_eq!(
            identity.reversibility(),
            CommandReversibility::NotApplicable
        );
        assert_eq!(
            identity.undo_granularity(),
            CommandUndoGranularity::NotApplicable
        );
    }
}
