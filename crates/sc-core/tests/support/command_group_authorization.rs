//! Whole-group authority uses actual core grants and the controlled source shared with single checks.
#![allow(clippy::expect_used)]
use super::*;
use sc_core::{
    command::{
        authorize_mutation_group, AssignMaterial, CommandGroupAuthorizationError as GroupDenial,
        CommandGroupGrantRejection, CommandGroupId, CommandMutationGroup, PieceSelection,
    },
    ontology::MaterialAssignment,
};

fn group(actor: Actor) -> Result<CommandMutationGroup, Box<dyn std::error::Error>> {
    let group = CommandGroupId::from_entity_id(Id::from_bits(30));
    let context = |key| {
        CommandMutationContext::new(
            Id::from_bits(1),
            actor,
            DesignRevision::from_counter(7),
            CommandIdempotencyKey::from_entity_id(Id::from_bits(key)),
            Some(group),
        )
    };
    Ok(CommandMutationGroup::new(
        group,
        vec![
            CommandMutation::new(
                context(4),
                MutationPayload::CloseDart(CloseDart::new(Id::from_bits(5))),
            ),
            CommandMutation::new(
                context(6),
                MutationPayload::AssignMaterial(AssignMaterial::new(
                    PieceSelection::new(vec![Id::from_bits(50)])?,
                    MaterialAssignment::Unresolved("observe material".into()),
                )?),
            ),
        ],
    )?)
}
fn grant(
    identity: u128,
    permissions: Vec<P>,
    commands: Vec<C>,
    status: Status,
) -> Result<CommandGrant, Box<dyn std::error::Error>> {
    Ok(CommandGrant::new(
        Id::from_bits(identity),
        agent(),
        permissions,
        scope(commands)?,
        status,
    )?)
}
fn all_commands() -> Vec<C> {
    vec![C::CloseDart, C::AssignMaterial]
}

#[test]
fn one_later_complete_grant_covers_original_group_at_one_trusted_snapshot(
) -> Result<(), Box<dyn std::error::Error>> {
    let group = group(agent())?;
    let mut host = Host::new(
        Some(agent()),
        vec![
            grant(10, vec![P::Inspect], vec![C::CloseDart], Status::Active)?,
            grant(11, vec![P::Inspect], all_commands(), Status::Active)?,
        ],
    );
    let checked = authorize_mutation_group(&group, Mode::Preview, &host)?;
    assert!(core::ptr::eq(checked.group(), &group));
    assert_eq!(checked.grant(), Id::from_bits(11));
    assert_eq!(checked.mode(), Mode::Preview);
    assert_eq!(checked.required_authority(), P::Inspect);
    assert_eq!(checked.checked_at(), at(15));
    assert_eq!(
        host.reads.get(),
        1,
        "all grants/members use one source observation"
    );
    assert!(matches!(
        authorize_mutation_group(&group, Mode::Execute, &host),
        Err(GroupDenial::NoMatchingGrant {
            required: P::Commit,
            ..
        })
    ));
    host.grants = vec![grant(12, vec![P::Commit], all_commands(), Status::Active)?];
    assert_eq!(
        authorize_mutation_group(&group, Mode::Execute, &host)?.required_authority(),
        P::Commit
    );
    assert!(
        matches!(
            authorize_mutation_group(&group, Mode::Preview, &host),
            Err(GroupDenial::NoMatchingGrant {
                required: P::Inspect,
                ..
            })
        ),
        "commit implies no inspect permission"
    );
    assert_eq!(
        checked.grant(),
        Id::from_bits(11),
        "old decision remains its descriptive snapshot"
    );
    Ok(())
}

#[test]
fn separately_authorized_members_cannot_union_partial_grants_for_the_group(
) -> Result<(), Box<dyn std::error::Error>> {
    let group = group(agent())?;
    let host = Host::new(
        Some(agent()),
        vec![
            grant(10, vec![P::Inspect], vec![C::CloseDart], Status::Active)?,
            grant(
                11,
                vec![P::Inspect],
                vec![C::AssignMaterial],
                Status::Active,
            )?,
        ],
    );
    for member in group.mutations() {
        let request = CommandRequest::Mutation(member.clone());
        assert!(
            authorize_command(&request, Mode::Preview, &host).is_ok(),
            "each member alone has a real matching grant"
        );
    }
    let denied = authorize_mutation_group(&group, Mode::Preview, &host)
        .expect_err("one complete grant required");
    let GroupDenial::NoMatchingGrant {
        group: actual,
        actor,
        design,
        required,
        rejections,
    } = denied
    else {
        return Err("expected complete group-scope refusal".into());
    };
    assert_eq!(
        (actual, actor, design, required),
        (group.id(), agent(), Id::from_bits(1), P::Inspect)
    );
    assert_eq!(rejections.len(), 2);
    assert!(
        matches!(rejections.first(), Some(CommandGroupGrantRejection { grant, index: 1,
        command: C::AssignMaterial, reason: Refusal::Scope(_) }) if *grant == Id::from_bits(10))
    );
    assert!(
        matches!(rejections.get(1), Some(CommandGroupGrantRejection { grant, index: 0,
        command: C::CloseDart, reason: Refusal::Scope(_) }) if *grant == Id::from_bits(11))
    );
    assert_eq!(host.reads.get(), 3);
    Ok(())
}

#[test]
fn group_identity_failures_preserve_original_core_cause_before_grant_selection(
) -> Result<(), Box<dyn std::error::Error>> {
    let group = group(agent())?;
    for (authenticated, expected) in [
        (None, Denial::Unauthenticated),
        (
            Some(human()),
            Denial::ActorMismatch {
                claimed: agent(),
                authenticated: human(),
            },
        ),
    ] {
        let repeated = grant(10, vec![P::Inspect], all_commands(), Status::Active)?;
        let host = Host::new(authenticated, vec![repeated.clone(), repeated]);
        let error = authorize_mutation_group(&group, Mode::Preview, &host)
            .expect_err("identity before duplicate grants");
        assert_eq!(
            error,
            GroupDenial::Source {
                group: group.id(),
                issue: Box::new(expected.clone())
            }
        );
        assert_eq!(
            std::error::Error::source(&error)
                .expect("original core source")
                .downcast_ref::<Denial>(),
            Some(&expected)
        );
        assert_eq!(host.reads.get(), 1);
    }
    Ok(())
}

#[test]
fn duplicate_inventory_cannot_hide_revocation_behind_an_earlier_complete_grant(
) -> Result<(), Box<dyn std::error::Error>> {
    let group = group(agent())?;
    let host = Host::new(
        Some(agent()),
        vec![
            grant(10, vec![P::Inspect], all_commands(), Status::Active)?,
            grant(10, vec![P::Inspect], all_commands(), Status::Revoked)?,
        ],
    );
    assert_eq!(
        authorize_mutation_group(&group, Mode::Preview, &host),
        Err(GroupDenial::Source {
            group: group.id(),
            issue: Box::new(Denial::DuplicateGrant {
                grant: Id::from_bits(10)
            })
        })
    );
    Ok(())
}

#[test]
fn repeated_group_authorization_refreshes_revocation_and_exclusive_expiry(
) -> Result<(), Box<dyn std::error::Error>> {
    let group = group(agent())?;
    let mut host = Host::new(
        Some(agent()),
        vec![grant(10, vec![P::Inspect], all_commands(), Status::Active)?],
    );
    let earlier = authorize_mutation_group(&group, Mode::Preview, &host)?;
    host.grants = vec![grant(
        10,
        vec![P::Inspect],
        all_commands(),
        Status::Revoked,
    )?];
    let error =
        authorize_mutation_group(&group, Mode::Preview, &host).expect_err("fresh revocation");
    assert!(
        matches!(error, GroupDenial::NoMatchingGrant { ref rejections, .. }
        if matches!(rejections.first(),Some(CommandGroupGrantRejection { index: 0, reason: Refusal::Revoked, .. })))
    );
    host.grants = vec![grant(10, vec![P::Inspect], all_commands(), Status::Active)?];
    host.now = at(20);
    let error =
        authorize_mutation_group(&group, Mode::Preview, &host).expect_err("exclusive expiry");
    assert!(
        matches!(error, GroupDenial::NoMatchingGrant { ref rejections, .. }
        if matches!(rejections.first(),Some(CommandGroupGrantRejection { index: 0, reason: Refusal::Scope(_), .. })))
    );
    assert_eq!(earlier.checked_at(), at(15));
    assert_eq!(host.reads.get(), 3);
    Ok(())
}
