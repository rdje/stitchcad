//! Complete requests over actual registered source consumers,without resolution or mutation.
use super::{BoundSlashSpreadSourcePlan, SlashSpreadGlobalSourceIntent, SlashSpreadSourceRequest};
use crate::ontology::{EdgeAnchor, EdgeRef, EntityId, IdentityLedger, Param, Registration};
use core::fmt;
use std::collections::BTreeMap;

/// Explicit requested treatment of an actual registered reference,not an achieved repair.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlashSpreadRegistrationIntent {
    /// Require the original global source to remain;local held-object membership still needs proof.
    KeepOriginalGlobalSource,
    /// Require proved correspondence or a visible unresolved repair;never choose from metadata alone.
    RequireCorrespondenceOrVisibleRepair,
    /// Request legitimate holder withdrawal and coordinated semantic-state/authority review.
    /// This never calls retire or hides an existing repair.
    RequestWithdrawalReview,
}
/// Original exact owner/anchor and explicit treatment request;no split choice is invented here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlashSpreadRegistrationDisposition {
    owner: EntityId,
    anchor: EdgeAnchor,
    intent: SlashSpreadRegistrationIntent,
}
impl SlashSpreadRegistrationDisposition {
    /// Own all actual authored fields without claiming the registration exists.
    #[must_use]
    pub const fn new(
        owner: EntityId,
        anchor: EdgeAnchor,
        intent: SlashSpreadRegistrationIntent,
    ) -> Self {
        Self {
            owner,
            anchor,
            intent,
        }
    }
    /// Original declared holder identity,without authority proof.
    #[must_use]
    pub const fn owner(&self) -> EntityId {
        self.owner
    }
    /// Original exact source-frame anchor,not a mapped target.
    #[must_use]
    pub const fn anchor(&self) -> &EdgeAnchor {
        &self.anchor
    }
    /// Actual explicit request,not performed repair or withdrawal.
    #[must_use]
    pub const fn intent(&self) -> &SlashSpreadRegistrationIntent {
        &self.intent
    }
}
/// Every actual registered source consumer needs one request;unregistered held objects stay separate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SlashSpreadRegistrationPlan {
    requests: Vec<SlashSpreadRegistrationDisposition>,
}
impl SlashSpreadRegistrationPlan {
    /// Retain the complete authored order,including malformed requests.
    #[must_use]
    pub const fn new(requests: Vec<SlashSpreadRegistrationDisposition>) -> Self {
        Self { requests }
    }
    /// All original request fields.
    #[must_use]
    pub fn requests(&self) -> &[SlashSpreadRegistrationDisposition] {
        &self.requests
    }
}
/// Complete S source identities,L whole-ledger registrations and K requests;no default.
/// ```compile_fail
/// let policy:sc_core::construction::SlashSpreadRegistrationPlanLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlashSpreadRegistrationPlanLimits {
    sources: usize,
    registrations: usize,
    requests: usize,
}
impl SlashSpreadRegistrationPlanLimits {
    /// Select complete source,whole-ledger registration and authored-request allowances.
    #[must_use]
    pub const fn new(sources: usize, registrations: usize, requests: usize) -> Self {
        Self {
            sources,
            registrations,
            requests,
        }
    }
    /// Complete source-identity allowance.
    #[must_use]
    pub const fn sources(self) -> usize {
        self.sources
    }
    /// Complete ledger registration allowance,including registrations on unrelated edges.
    #[must_use]
    pub const fn registrations(self) -> usize {
        self.registrations
    }
    /// Complete authored request allowance.
    #[must_use]
    pub const fn requests(self) -> usize {
        self.requests
    }
}
/// Whole-resource refusals,then original-order request errors,then missing ledger registration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlashSpreadRegistrationPlanRefusal {
    /// Conservative S+L+2K workspace cannot be counted.
    CountOverflow,
    /// Complete source-identity population exceeds policy.
    Sources {
        /// Complete required population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete whole-ledger population exceeds policy.
    Registrations {
        /// Complete required population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete authored request population exceeds policy.
    Requests {
        /// Complete required population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Request source belongs to no actual bound source request.
    Source {
        /// Original request position.
        request: usize,
    },
    /// The exact owner/edge/Param key has no registration in this actual ledger.
    Registration {
        /// Original request position.
        request: usize,
    },
    /// Two original requests name the same exact owner/edge/Param key.
    Duplicate {
        /// First original request.
        first: usize,
        /// Repeated original request.
        second: usize,
    },
    /// Keeping the original global source contradicts its retirement-review request.
    SourceRetirement {
        /// Original registration request position.
        request: usize,
    },
    /// An actual registered source consumer lacks an explicit request.
    Missing {
        /// Exact first uncovered registration in actual ledger order,including split choice.
        registration: Registration,
    },
}
/// Actual raw plan,source plan,immutable ledger and caller policy survive refusal unchanged.
pub struct SlashSpreadRegistrationPlanError<'a> {
    plan: &'a SlashSpreadRegistrationPlan,
    sources: &'a BoundSlashSpreadSourcePlan<'a>,
    ledger: &'a IdentityLedger,
    limits: &'a SlashSpreadRegistrationPlanLimits,
    refusal: SlashSpreadRegistrationPlanRefusal,
}
impl<'a> SlashSpreadRegistrationPlanError<'a> {
    /// Actual complete original registration request owner.
    #[must_use]
    pub const fn plan(&self) -> &'a SlashSpreadRegistrationPlan {
        self.plan
    }
    /// Actual complete bound source-identity plan.
    #[must_use]
    pub const fn sources(&self) -> &'a BoundSlashSpreadSourcePlan<'a> {
        self.sources
    }
    /// Actual immutable registration owner;no detached replacement is substituted.
    #[must_use]
    pub const fn ledger(&self) -> &'a IdentityLedger {
        self.ledger
    }
    /// Original borrowed whole-resource policy.
    #[must_use]
    pub const fn limits(&self) -> &'a SlashSpreadRegistrationPlanLimits {
        self.limits
    }
    /// First whole-resource or original-field cause.
    #[must_use]
    pub const fn refusal(&self) -> SlashSpreadRegistrationPlanRefusal {
        self.refusal
    }
}
impl fmt::Debug for SlashSpreadRegistrationPlanError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SlashSpreadRegistrationPlanError")
            .field("plan", &self.plan)
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for SlashSpreadRegistrationPlanError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "slash/spread registration plan: {:?}", self.refusal)
    }
}
impl std::error::Error for SlashSpreadRegistrationPlanError<'_> {}
type RegistrationKey = (EntityId, EdgeRef, Param);
fn key(registration: Registration) -> RegistrationKey {
    (
        registration.owner(),
        registration.edge(),
        registration.param(),
    )
}

impl SlashSpreadRegistrationPlan {
    /// Bind every actual ledger registration on the declared sources to one explicit request.
    /// Complete S/L/K quotas and S+L+2K workspace precede indexing or field checks. The immutable
    /// ledger borrow keeps original split choices inspectable;no resolution or mutation occurs.
    /// This scope does not certify current Design/semantic ownership or completeness of registration.
    /// # Errors
    /// Count overflow,S/L/K quotas,then original-order unknown source/registration,duplicate,
    /// conflicting keep-source request,then first missing source registration in ledger order.
    pub fn bind<'a>(
        &'a self,
        sources: &'a BoundSlashSpreadSourcePlan<'a>,
        ledger: &'a IdentityLedger,
        limits: &'a SlashSpreadRegistrationPlanLimits,
    ) -> Result<BoundSlashSpreadRegistrationPlan<'a>, SlashSpreadRegistrationPlanError<'a>> {
        let refuse = |refusal| SlashSpreadRegistrationPlanError {
            plan: self,
            sources,
            ledger,
            limits,
            refusal,
        };
        let s = sources.sources().len();
        let l = ledger.registration_count();
        let k = self.requests.len();
        let workspace_entries = k
            .checked_mul(2)
            .and_then(|x| x.checked_add(s))
            .and_then(|x| x.checked_add(l))
            .ok_or_else(|| refuse(SlashSpreadRegistrationPlanRefusal::CountOverflow))?;
        if s > limits.sources {
            return Err(refuse(SlashSpreadRegistrationPlanRefusal::Sources {
                required: s,
                allowed: limits.sources,
            }));
        }
        if l > limits.registrations {
            return Err(refuse(SlashSpreadRegistrationPlanRefusal::Registrations {
                required: l,
                allowed: limits.registrations,
            }));
        }
        if k > limits.requests {
            return Err(refuse(SlashSpreadRegistrationPlanRefusal::Requests {
                required: k,
                allowed: limits.requests,
            }));
        }
        let source_index: BTreeMap<_, _> = sources
            .sources()
            .map(|source| (source.request().edge(), source))
            .collect();
        let registrations: BTreeMap<_, _> = ledger
            .registrations()
            .filter(|registration| source_index.contains_key(&registration.edge()))
            .map(|registration| (key(registration), registration))
            .collect();
        let mut requests = BTreeMap::new();
        let mut bound = Vec::with_capacity(k);
        for (request, original) in self.requests.iter().enumerate() {
            let source = source_index
                .get(&original.anchor.edge)
                .ok_or_else(|| refuse(SlashSpreadRegistrationPlanRefusal::Source { request }))?;
            let key = (original.owner, original.anchor.edge, original.anchor.param);
            let registration = registrations.get(&key).ok_or_else(|| {
                refuse(SlashSpreadRegistrationPlanRefusal::Registration { request })
            })?;
            if let Some(first) = requests.insert(key, request) {
                return Err(refuse(SlashSpreadRegistrationPlanRefusal::Duplicate {
                    first,
                    second: request,
                }));
            }
            if original.intent == SlashSpreadRegistrationIntent::KeepOriginalGlobalSource
                && source.request().intent() != &SlashSpreadGlobalSourceIntent::Preserve
            {
                return Err(refuse(
                    SlashSpreadRegistrationPlanRefusal::SourceRetirement { request },
                ));
            }
            bound.push((*source, *registration));
        }
        for (key, registration) in registrations {
            if !requests.contains_key(&key) {
                return Err(refuse(SlashSpreadRegistrationPlanRefusal::Missing {
                    registration,
                }));
            }
        }
        Ok(BoundSlashSpreadRegistrationPlan {
            plan: self,
            sources,
            ledger,
            limits,
            bound,
            workspace_entries,
        })
    }
}
/// Complete actual registered-source coverage;not a census of all held semantic/structural objects.
#[derive(Debug)]
pub struct BoundSlashSpreadRegistrationPlan<'a> {
    plan: &'a SlashSpreadRegistrationPlan,
    sources: &'a BoundSlashSpreadSourcePlan<'a>,
    ledger: &'a IdentityLedger,
    limits: &'a SlashSpreadRegistrationPlanLimits,
    bound: Vec<(SlashSpreadSourceRequest<'a>, Registration)>,
    workspace_entries: usize,
}
impl<'a> BoundSlashSpreadRegistrationPlan<'a> {
    /// Actual complete raw request plan.
    #[must_use]
    pub const fn plan(&self) -> &'a SlashSpreadRegistrationPlan {
        self.plan
    }
    /// Actual complete source-identity plan.
    #[must_use]
    pub const fn sources(&self) -> &'a BoundSlashSpreadSourcePlan<'a> {
        self.sources
    }
    /// Actual immutable ledger from which registrations and choices were inspected.
    #[must_use]
    pub const fn ledger(&self) -> &'a IdentityLedger {
        self.ledger
    }
    /// Actual borrowed whole-resource policy.
    #[must_use]
    pub const fn limits(&self) -> &'a SlashSpreadRegistrationPlanLimits {
        self.limits
    }
    /// Conservative S+L+2K entries for temporary indexes plus retained K association pairs.
    #[must_use]
    pub const fn workspace_entries(&self) -> usize {
        self.workspace_entries
    }
    /// Every actual source-registration association in original request order.
    pub fn requests(
        &self,
    ) -> impl DoubleEndedIterator<Item = SlashSpreadRegisteredSourceRequest<'_>> + ExactSizeIterator
    {
        self.plan.requests.iter().zip(&self.bound).enumerate().map(
            |(index, (request, (source, registration)))| SlashSpreadRegisteredSourceRequest {
                plan: self,
                index,
                request,
                source: *source,
                registration: *registration,
            },
        )
    }
}
/// Private authentic original-request/source/registration association with unchanged split choice.
/// ```compile_fail
/// fn forge<'a>(mut request:sc_core::construction::SlashSpreadRegisteredSourceRequest<'a>,other:&'a sc_core::construction::BoundSlashSpreadRegistrationPlan<'a>) {
///     request.plan=other;
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct SlashSpreadRegisteredSourceRequest<'a> {
    plan: &'a BoundSlashSpreadRegistrationPlan<'a>,
    index: usize,
    request: &'a SlashSpreadRegistrationDisposition,
    source: SlashSpreadSourceRequest<'a>,
    registration: Registration,
}
impl<'a> SlashSpreadRegisteredSourceRequest<'a> {
    /// Actual complete bound registration plan.
    #[must_use]
    pub const fn plan(self) -> &'a BoundSlashSpreadRegistrationPlan<'a> {
        self.plan
    }
    /// Original request position,not identity.
    #[must_use]
    pub const fn index(self) -> usize {
        self.index
    }
    /// Actual original authored request.
    #[must_use]
    pub const fn request(self) -> &'a SlashSpreadRegistrationDisposition {
        self.request
    }
    /// Authentic original source request,with every source/retained/provenance occurrence available.
    #[must_use]
    pub const fn source(self) -> SlashSpreadSourceRequest<'a> {
        self.source
    }
    /// Actual registration value from the still-borrowed ledger,including its unchanged split choice.
    #[must_use]
    pub const fn registration(self) -> Registration {
        self.registration
    }
}
