//! Core authorization semantics with an independently controlled trusted host; no transport authentication claim.
use sc_core::{
    command::{
        authorize_command, AgentApprovalGrantError, ApprovalScopeTarget, ApprovePackage,
        ArtifactPayload, CloseDart, CommandActorClaim as Actor, CommandArtifact,
        CommandAuthority as P, CommandAuthoritySnapshot, CommandAuthoritySource,
        CommandAuthorizationError as Denial, CommandAuthorizationMode as Mode, CommandEvaluation,
        CommandGrant, CommandGrantRefusal as Refusal, CommandGrantRejection,
        CommandGrantStatus as Status, CommandId as C, CommandIdempotencyKey, CommandMutation,
        CommandMutationContext, CommandQuery, CommandRelease, CommandRequest, CommandScope,
        CommandScopeRefusal, CommandSnapshotContext, DesignRevision, EvaluationPayload,
        EvaluationProfile, GenerateTechPack, InstanceSelection, MutationPayload, PrepareRelease,
        QueryPayload, ReleasePayload, ResolveProfile,
    },
    ontology::{EntityId as Id, PackageIdentity, SelectedProfilePin, SelectedProfilePinDefinition},
};
use std::{
    cell::Cell,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

fn at(second: u64) -> SystemTime {
    UNIX_EPOCH + Duration::from_secs(second)
}
fn human() -> Actor {
    Actor::Human(Id::from_bits(2))
}
fn agent() -> Actor {
    Actor::Agent {
        identity: Id::from_bits(3),
        principal: Id::from_bits(2),
    }
}
fn snapshot(actor: Actor) -> CommandSnapshotContext {
    CommandSnapshotContext::new(Id::from_bits(1), actor, DesignRevision::from_counter(7))
}
fn query(actor: Actor) -> CommandRequest {
    CommandRequest::Query(CommandQuery::new(
        snapshot(actor),
        QueryPayload::GetPieceList,
    ))
}
fn mutation(actor: Actor) -> CommandRequest {
    CommandRequest::Mutation(CommandMutation::new(
        CommandMutationContext::new(
            Id::from_bits(1),
            actor,
            DesignRevision::from_counter(7),
            CommandIdempotencyKey::from_entity_id(Id::from_bits(4)),
            None,
        ),
        MutationPayload::CloseDart(CloseDart::new(Id::from_bits(5))),
    ))
}
fn approval(actor: Actor) -> Result<CommandRequest, Box<dyn std::error::Error>> {
    Ok(CommandRequest::Release(CommandRelease::new(
        snapshot(actor),
        ReleasePayload::ApprovePackage(ApprovePackage::new(
            PackageIdentity::new("opaque-v1".to_owned(), vec![0, 7, 0])?,
            ApprovalScopeTarget::from_entity_id(Id::from_bits(8)),
        )),
    )))
}
fn scope(commands: Vec<C>) -> Result<CommandScope, Box<dyn std::error::Error>> {
    Ok(CommandScope::new(
        Id::from_bits(1),
        commands,
        at(10),
        at(20),
    )?)
}
struct Host {
    actor: Option<Actor>,
    now: SystemTime,
    grants: Vec<CommandGrant>,
    reads: Cell<usize>,
}
impl Host {
    fn new(actor: Option<Actor>, grants: Vec<CommandGrant>) -> Self {
        Self {
            actor,
            now: at(15),
            grants,
            reads: Cell::new(0),
        }
    }
}
impl CommandAuthoritySource for Host {
    fn snapshot(&self) -> CommandAuthoritySnapshot<'_> {
        self.reads.set(self.reads.get() + 1);
        CommandAuthoritySnapshot {
            authenticated_actor: self.actor,
            observed_at: self.now,
            grants: &self.grants,
        }
    }
}

#[test]
fn request_claims_cannot_authenticate_or_change_session_role_initiator_or_principal(
) -> Result<(), Box<dyn std::error::Error>> {
    let request = query(human());
    let grant = CommandGrant::new(
        Id::from_bits(9),
        human(),
        vec![P::Inspect],
        scope(vec![C::GetPieceList])?,
        Status::Active,
    )?;
    let absent = Host::new(None, vec![grant]);
    assert_eq!(
        authorize_command(&request, Mode::Execute, &absent),
        Err(Denial::Unauthenticated)
    );
    assert_eq!(absent.reads.get(), 1);
    for (claimed, authenticated) in [
        (human(), agent()),
        (human(), Actor::Human(Id::from_bits(99))),
        (
            agent(),
            Actor::Agent {
                identity: Id::from_bits(99),
                principal: Id::from_bits(2),
            },
        ),
        (
            agent(),
            Actor::Agent {
                identity: Id::from_bits(3),
                principal: Id::from_bits(99),
            },
        ),
        (Actor::Human(Id::from_bits(3)), agent()),
    ] {
        let host = Host::new(Some(authenticated), vec![]);
        assert_eq!(
            authorize_command(&query(claimed), Mode::Execute, &host),
            Err(Denial::ActorMismatch {
                claimed,
                authenticated
            })
        );
        assert_eq!(host.reads.get(), 1);
    }
    Ok(())
}

#[test]
fn agent_may_use_all_ordinary_permissions_but_never_hold_or_issue_approval(
) -> Result<(), Box<dyn std::error::Error>> {
    let permissions = vec![P::Inspect, P::Propose, P::Commit, P::Generate];
    let inventory = C::ALL.to_vec();
    for status in [Status::Active, Status::Revoked] {
        let mut forbidden = permissions.clone();
        forbidden.push(P::Approve);
        assert_eq!(
            CommandGrant::new(
                Id::from_bits(10),
                agent(),
                forbidden,
                scope(inventory.clone())?,
                status
            ),
            Err(AgentApprovalGrantError {
                grant: Id::from_bits(10),
                actor: agent()
            })
        );
    }
    let grant = CommandGrant::new(
        Id::from_bits(10),
        agent(),
        permissions.clone(),
        scope(inventory)?,
        Status::Active,
    )?;
    assert_eq!(grant.permissions(), permissions);
    let artifact = ArtifactPayload::GenerateTechPack(GenerateTechPack::new(
        InstanceSelection::new(vec![Id::from_bits(11)])?,
        EvaluationProfile::Logical,
    ));
    let requests = [
        query(agent()),
        mutation(agent()),
        CommandRequest::Evaluation(CommandEvaluation::new(
            snapshot(agent()),
            EvaluationPayload::ResolveProfile(ResolveProfile::new(SelectedProfilePin::new(
                SelectedProfilePinDefinition {
                    profile_id: "house".to_owned(),
                    schema_version: "2".to_owned(),
                    content_version: "1".to_owned(),
                },
            )?)),
        )),
        CommandRequest::Artifact(CommandArtifact::new(snapshot(agent()), artifact.clone())),
        CommandRequest::Release(CommandRelease::new(
            snapshot(agent()),
            ReleasePayload::PrepareRelease(PrepareRelease::new(vec![artifact])?),
        )),
    ];
    let host = Host::new(Some(agent()), vec![grant]);
    for request in &requests {
        let decision = authorize_command(request, Mode::Execute, &host)?;
        assert!(std::ptr::eq(decision.request(), request));
        assert_eq!(
            decision.required_authority(),
            request.command_id().required_authority()
        );
        assert_eq!(decision.grant(), Id::from_bits(10));
    }
    assert_eq!(
        authorize_command(&approval(agent())?, Mode::Execute, &host),
        Err(Denial::HumanApprovalRequired { actor: agent() })
    );
    // Naming the human principal in untrusted request data cannot change the authenticated role.
    assert_eq!(
        authorize_command(&approval(human())?, Mode::Execute, &host),
        Err(Denial::ActorMismatch {
            claimed: human(),
            authenticated: agent()
        })
    );
    Ok(())
}

#[test]
fn every_grant_requires_its_own_actor_permission_scope_live_status_and_time(
) -> Result<(), Box<dyn std::error::Error>> {
    let request = query(agent());
    let cases = [
        (
            human(),
            vec![P::Inspect],
            scope(vec![C::GetPieceList])?,
            Status::Active,
            at(15),
            Refusal::Actor {
                granted: human(),
                authenticated: agent(),
            },
        ),
        (
            agent(),
            vec![P::Inspect],
            scope(vec![C::GetPieceList])?,
            Status::Revoked,
            at(15),
            Refusal::Revoked,
        ),
        (
            agent(),
            vec![P::Generate, P::Commit, P::Propose],
            scope(vec![C::GetPieceList])?,
            Status::Active,
            at(15),
            Refusal::Permission {
                required: P::Inspect,
            },
        ),
        (
            agent(),
            vec![P::Inspect],
            CommandScope::new(Id::from_bits(99), vec![C::GetPieceList], at(10), at(20))?,
            Status::Active,
            at(15),
            Refusal::Scope(CommandScopeRefusal::Design {
                scoped: Id::from_bits(99),
                requested: Id::from_bits(1),
            }),
        ),
        (
            agent(),
            vec![P::Inspect],
            scope(vec![C::WalkSeamSpan])?,
            Status::Active,
            at(15),
            Refusal::Scope(CommandScopeRefusal::Command {
                requested: C::GetPieceList,
            }),
        ),
        (
            agent(),
            vec![P::Inspect],
            scope(vec![C::GetPieceList])?,
            Status::Active,
            at(9),
            Refusal::Scope(CommandScopeRefusal::NotYetValid {
                at: at(9),
                valid_from: at(10),
            }),
        ),
        (
            agent(),
            vec![P::Inspect],
            scope(vec![C::GetPieceList])?,
            Status::Active,
            at(20),
            Refusal::Scope(CommandScopeRefusal::Expired {
                at: at(20),
                valid_until: at(20),
            }),
        ),
    ];
    for (actor, permissions, scope, status, now, reason) in cases {
        let grant = CommandGrant::new(Id::from_bits(12), actor, permissions, scope, status)?;
        let mut host = Host::new(Some(agent()), vec![grant]);
        host.now = now;
        assert_eq!(
            authorize_command(&request, Mode::Execute, &host),
            Err(Denial::NoMatchingGrant {
                actor: agent(),
                design: Id::from_bits(1),
                command: C::GetPieceList,
                required: P::Inspect,
                rejections: vec![CommandGrantRejection {
                    grant: Id::from_bits(12),
                    reason
                }],
            })
        );
        assert_eq!(host.reads.get(), 1);
    }
    Ok(())
}

#[test]
fn human_approval_requires_explicit_approve_and_returns_original_snapshot_decision(
) -> Result<(), Box<dyn std::error::Error>> {
    let request = approval(human())?;
    let missing = CommandGrant::new(
        Id::from_bits(13),
        human(),
        vec![P::Inspect, P::Propose, P::Commit, P::Generate],
        scope(vec![C::ApprovePackage])?,
        Status::Active,
    )?;
    let host = Host::new(Some(human()), vec![missing.clone()]);
    assert!(matches!(
        authorize_command(&request, Mode::Execute, &host),
        Err(Denial::NoMatchingGrant {
            required: P::Approve,
            ..
        })
    ));
    let permissions = vec![P::Approve, P::Inspect, P::Approve];
    let grant = CommandGrant::new(
        Id::from_bits(14),
        human(),
        permissions.clone(),
        scope(vec![C::ApprovePackage])?,
        Status::Active,
    )?;
    assert_eq!(grant.permissions(), permissions);
    let host = Host::new(Some(human()), vec![missing, grant]);
    let decision = authorize_command(&request, Mode::Execute, &host)?;
    assert!(std::ptr::eq(decision.request(), &request));
    assert_eq!(decision.grant(), Id::from_bits(14));
    assert_eq!(decision.mode(), Mode::Execute);
    assert_eq!(decision.required_authority(), P::Approve);
    assert_eq!(decision.checked_at(), at(15));
    assert_eq!(host.reads.get(), 1);
    Ok(())
}

#[test]
fn mutation_preview_uses_inspect_without_allowing_commit_or_nonmutation_preview(
) -> Result<(), Box<dyn std::error::Error>> {
    let request = mutation(agent());
    let grant = CommandGrant::new(
        Id::from_bits(15),
        agent(),
        vec![P::Inspect, P::Propose, P::Generate],
        scope(vec![C::CloseDart, C::GetPieceList])?,
        Status::Active,
    )?;
    let host = Host::new(Some(agent()), vec![grant]);
    let preview = authorize_command(&request, Mode::Preview, &host)?;
    assert_eq!(preview.mode(), Mode::Preview);
    assert_eq!(preview.required_authority(), P::Inspect);
    assert!(matches!(
        authorize_command(&request, Mode::Execute, &host),
        Err(Denial::NoMatchingGrant {
            required: P::Commit,
            ..
        })
    ));
    assert_eq!(
        authorize_command(&query(agent()), Mode::Preview, &host),
        Err(Denial::PreviewRequiresMutation {
            command: C::GetPieceList
        })
    );
    let human_host = Host::new(Some(human()), vec![]);
    assert_eq!(
        authorize_command(&approval(human())?, Mode::Preview, &human_host),
        Err(Denial::PreviewRequiresMutation {
            command: C::ApprovePackage
        })
    );
    Ok(())
}

#[test]
fn authorization_never_unions_permissions_and_scopes_from_different_grants(
) -> Result<(), Box<dyn std::error::Error>> {
    let request = query(agent());
    let permission_only = CommandGrant::new(
        Id::from_bits(16),
        agent(),
        vec![P::Inspect],
        scope(vec![C::WalkSeamSpan])?,
        Status::Active,
    )?;
    let scope_only = CommandGrant::new(
        Id::from_bits(17),
        agent(),
        vec![],
        scope(vec![C::GetPieceList])?,
        Status::Active,
    )?;
    let host = Host::new(Some(agent()), vec![permission_only, scope_only]);
    assert_eq!(
        authorize_command(&request, Mode::Execute, &host),
        Err(Denial::NoMatchingGrant {
            actor: agent(),
            design: Id::from_bits(1),
            command: C::GetPieceList,
            required: P::Inspect,
            rejections: vec![
                CommandGrantRejection {
                    grant: Id::from_bits(16),
                    reason: Refusal::Scope(CommandScopeRefusal::Command {
                        requested: C::GetPieceList
                    })
                },
                CommandGrantRejection {
                    grant: Id::from_bits(17),
                    reason: Refusal::Permission {
                        required: P::Inspect
                    }
                },
            ],
        })
    );
    let empty = Host::new(Some(agent()), vec![]);
    assert_eq!(
        authorize_command(&request, Mode::Execute, &empty),
        Err(Denial::NoMatchingGrant {
            actor: agent(),
            design: Id::from_bits(1),
            command: C::GetPieceList,
            required: P::Inspect,
            rejections: vec![],
        })
    );
    Ok(())
}

#[test]
fn duplicate_current_grant_identity_cannot_hide_revocation_after_an_earlier_success(
) -> Result<(), Box<dyn std::error::Error>> {
    let request = query(agent());
    let active = CommandGrant::new(
        Id::from_bits(18),
        agent(),
        vec![P::Inspect],
        scope(vec![C::GetPieceList])?,
        Status::Active,
    )?;
    let revoked = CommandGrant::new(
        Id::from_bits(18),
        agent(),
        vec![P::Inspect],
        scope(vec![C::GetPieceList])?,
        Status::Revoked,
    )?;
    for grants in [vec![active.clone(), revoked.clone()], vec![revoked, active]] {
        let host = Host::new(Some(agent()), grants);
        assert_eq!(
            authorize_command(&request, Mode::Execute, &host),
            Err(Denial::DuplicateGrant {
                grant: Id::from_bits(18)
            })
        );
    }
    Ok(())
}

#[test]
fn subsequent_decisions_recheck_revocation_and_time_instead_of_reusing_prior_success(
) -> Result<(), Box<dyn std::error::Error>> {
    let request = query(agent());
    let active = CommandGrant::new(
        Id::from_bits(19),
        agent(),
        vec![P::Inspect],
        scope(vec![C::GetPieceList])?,
        Status::Active,
    )?;
    let mut host = Host::new(Some(agent()), vec![active.clone()]);
    let prior = authorize_command(&request, Mode::Execute, &host)?;
    host.grants = vec![CommandGrant::new(
        Id::from_bits(19),
        agent(),
        vec![P::Inspect],
        scope(vec![C::GetPieceList])?,
        Status::Revoked,
    )?];
    assert_eq!(
        authorize_command(&request, Mode::Execute, &host),
        Err(Denial::NoMatchingGrant {
            actor: agent(),
            design: Id::from_bits(1),
            command: C::GetPieceList,
            required: P::Inspect,
            rejections: vec![CommandGrantRejection {
                grant: Id::from_bits(19),
                reason: Refusal::Revoked
            }],
        })
    );
    host.grants = vec![active];
    host.now = at(20);
    assert_eq!(
        authorize_command(&request, Mode::Execute, &host),
        Err(Denial::NoMatchingGrant {
            actor: agent(),
            design: Id::from_bits(1),
            command: C::GetPieceList,
            required: P::Inspect,
            rejections: vec![CommandGrantRejection {
                grant: Id::from_bits(19),
                reason: Refusal::Scope(CommandScopeRefusal::Expired {
                    at: at(20),
                    valid_until: at(20)
                })
            }],
        })
    );
    assert_eq!(host.reads.get(), 3);
    assert_eq!(
        prior.checked_at(),
        at(15),
        "earlier decision describes only its original observation"
    );
    assert!(std::ptr::eq(prior.request(), &request));
    host.now = at(19);
    assert_eq!(
        authorize_command(&request, Mode::Execute, &host)?.checked_at(),
        at(19)
    );
    assert_eq!(host.reads.get(), 4);
    Ok(())
}
