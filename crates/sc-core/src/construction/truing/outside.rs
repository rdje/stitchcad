//! Explicit native outside-fragment roles in the actual whole held edge frame;not fitted geometry.
use super::{RequiredTruingEvidence, TruingEdgeRoleBindingRefusal, TruingOutcomeIntent};
use crate::{
    construction::{ActiveEdgeOutputRole, EdgeOutputRoleCatalogLimits},
    ontology::{EdgeRange, EdgeRef, LocalTag, Param, RangeError},
};
use core::fmt;

/// Exact outside portion in the original containing edge's held parameter frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TruingOutsidePart {
    /// Original0 through selected lower parameter.
    Prefix,
    /// Selected upper parameter through original1.
    Suffix,
}
/// Explicit absence or persistent role;absence is not inferred and no role is invented.
/// ```compile_fail
/// let intent:sc_core::construction::TruingOutsideIntent=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TruingOutsideIntent {
    /// No outside output fragment for this field;admission checks the actual outcome/domain.
    Absent,
    /// Explicit persistent tag from the actual bound edge catalogue,not an array position.
    Role(LocalTag),
}
/// Complete selected-candidate and actual catalogue D/A/key-byte allowances;no defaults.
/// ```compile_fail
/// let limits:sc_core::construction::TruingOutsideRoleLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TruingOutsideRoleLimits {
    candidates: usize,
    catalog: EdgeOutputRoleCatalogLimits,
}
impl TruingOutsideRoleLimits {
    /// Choose whole selected N and whole D/A/B before either fixed outside field is inspected.
    #[must_use]
    pub const fn new(candidates: usize, catalog: EdgeOutputRoleCatalogLimits) -> Self {
        Self {
            candidates,
            catalog,
        }
    }
    /// Whole selected candidate allowance.
    #[must_use]
    pub const fn candidates(self) -> usize {
        self.candidates
    }
    /// Whole actual role-catalogue allowance,including inactive definitions and all keys.
    #[must_use]
    pub const fn catalog(self) -> EdgeOutputRoleCatalogLimits {
        self.catalog
    }
}
/// First whole quota,explicit field/domain or actual role-association refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TruingOutsideRoleRefusal {
    /// Complete selected candidate population exceeds policy.
    Candidates {
        /// Actual whole population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Original full-catalogue D/A/B/owner admission cause.
    Catalog(TruingEdgeRoleBindingRefusal),
    /// Exact source preservation cannot introduce an outside candidate role.
    PreserveSource {
        /// First offending fixed field.
        part: TruingOutsidePart,
    },
    /// Refit omitted a positive actual outside portion.
    Missing {
        /// First omitted fixed field.
        part: TruingOutsidePart,
    },
    /// A supplied role has no positive outside range in this actual source frame.
    SourceRange {
        /// Original fixed field.
        part: TruingOutsidePart,
        /// Exact canonical range-construction cause,including original bounds.
        cause: RangeError,
    },
    /// Supplied tag names no actual catalogue definition.
    Undefined {
        /// Original fixed field.
        part: TruingOutsidePart,
        /// Actual supplied tag.
        tag: LocalTag,
    },
    /// Supplied role is canonically inactive.
    Inactive {
        /// Original fixed field.
        part: TruingOutsidePart,
        /// Actual supplied tag.
        tag: LocalTag,
    },
    /// Outside tag is already assigned to a selected-domain candidate.
    SelectedRole {
        /// Original fixed field.
        part: TruingOutsidePart,
        /// Actual reused tag.
        tag: LocalTag,
    },
    /// Prefix and suffix cannot declare the same logical output edge.
    Duplicate {
        /// Actual repeated persistent tag.
        tag: LocalTag,
    },
}
/// Actual complete original fixed proposal/evidence/source parents,policy and first cause.
pub struct TruingOutsideRoleError<'a> {
    proposal: &'a TruingOutsideRoles,
    evidence: &'a RequiredTruingEvidence<'a>,
    limits: TruingOutsideRoleLimits,
    refusal: TruingOutsideRoleRefusal,
}
impl<'a> TruingOutsideRoleError<'a> {
    /// Actual whole original fixed proposal,including any unaccepted field.
    #[must_use]
    pub const fn proposal(&self) -> &'a TruingOutsideRoles {
        self.proposal
    }
    /// Actual complete evidence/lock/outcome/candidate/catalogue/copy/Piece/source context.
    #[must_use]
    pub const fn evidence(&self) -> &'a RequiredTruingEvidence<'a> {
        self.evidence
    }
    /// Original explicit whole N/D/A/B policy.
    #[must_use]
    pub const fn limits(&self) -> TruingOutsideRoleLimits {
        self.limits
    }
    /// First quota or fixed-field cause.
    #[must_use]
    pub const fn refusal(&self) -> TruingOutsideRoleRefusal {
        self.refusal
    }
}
impl fmt::Debug for TruingOutsideRoleError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TruingOutsideRoleError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for TruingOutsideRoleError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "truing outside roles: {:?}", self.refusal)
    }
}
impl std::error::Error for TruingOutsideRoleError<'_> {}
/// Two explicit outside fields,not a copied source range or a complete replacement Piece.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TruingOutsideRoles {
    prefix: TruingOutsideIntent,
    suffix: TruingOutsideIntent,
}
impl TruingOutsideRoles {
    /// Retain both explicit fields without default tags or replacement source frames.
    #[must_use]
    pub const fn new(prefix: TruingOutsideIntent, suffix: TruingOutsideIntent) -> Self {
        Self { prefix, suffix }
    }
    /// Actual original field for a fixed source portion.
    #[must_use]
    pub const fn intent(&self, part: TruingOutsidePart) -> &TruingOutsideIntent {
        match part {
            TruingOutsidePart::Prefix => &self.prefix,
            TruingOutsidePart::Suffix => &self.suffix,
        }
    }
    /// Admit whole N,then actual catalogue D/A/B/owner,before prefix and suffix fields.
    /// Refit requires a role for each positive outside portion and absence for an empty portion.
    /// PreserveSource requires both absent,including partial selected domains. Roles must be
    /// defined,active and distinct from selected output tags and each other. Exact ranges derive
    /// only from original0/from and to/1;no caller frame,epsilon,geometry or registry is introduced.
    /// Two fixed associations take constant space with no allocation;work O(log(N+D+2)). Earlier
    /// owned catalogue/index/storage and later whole-Piece/geometry/history proof remain separate.
    /// # Errors
    /// N,catalogue;then each original prefix/suffix's preservation,missing/empty range,undefined,
    /// inactive,selected-tag reuse or duplicate outside tag cause. No accepted prefix escapes.
    pub fn admit<'a>(
        &'a self,
        evidence: &'a RequiredTruingEvidence<'a>,
        limits: TruingOutsideRoleLimits,
    ) -> Result<AdmittedTruingOutsideRoles<'a>, TruingOutsideRoleError<'a>> {
        use TruingOutsideRoleRefusal as R;
        let refuse = |refusal| TruingOutsideRoleError {
            proposal: self,
            evidence,
            limits,
            refusal,
        };
        let outcome = evidence.locks().outcome();
        let domain = outcome.source_domain();
        let selected = domain.candidates();
        let n = selected.edge_count();
        if n > limits.candidates {
            return Err(refuse(R::Candidates {
                required: n,
                allowed: limits.candidates,
            }));
        }
        let roles = selected.roles();
        roles
            .copies()
            .bind_edge_roles(roles.catalog(), limits.catalog)
            .map_err(|cause| refuse(R::Catalog(cause.refusal())))?;
        let mut outside = [None, None];
        let mut previous = None;
        for (slot, (part, intent)) in outside.iter_mut().zip([
            (TruingOutsidePart::Prefix, &self.prefix),
            (TruingOutsidePart::Suffix, &self.suffix),
        ]) {
            if outcome.intent() == TruingOutcomeIntent::PreserveSource {
                if matches!(intent, TruingOutsideIntent::Role(_)) {
                    return Err(refuse(R::PreserveSource { part }));
                }
                continue;
            }
            let original = domain.domain();
            let (from, to) = match part {
                TruingOutsidePart::Prefix => (Param::START, original.from()),
                TruingOutsidePart::Suffix => (original.to(), Param::END),
            };
            let tag = match intent {
                TruingOutsideIntent::Absent => {
                    if from < to {
                        return Err(refuse(R::Missing { part }));
                    }
                    continue;
                }
                TruingOutsideIntent::Role(tag) => *tag,
            };
            let source = EdgeRange::new(original.edge(), from, to)
                .map_err(|cause| refuse(R::SourceRange { part, cause }))?;
            let catalog = roles.catalog();
            if catalog.definition(tag).is_none() {
                return Err(refuse(R::Undefined { part, tag }));
            }
            let role = catalog
                .active_role(tag)
                .ok_or_else(|| refuse(R::Inactive { part, tag }))?;
            if selected.edge(tag).is_some() {
                return Err(refuse(R::SelectedRole { part, tag }));
            }
            if previous == Some(tag) {
                return Err(refuse(R::Duplicate { tag }));
            }
            previous = Some(tag);
            *slot = Some(OutsideAssociation {
                part,
                intent,
                source,
                role,
            });
        }
        Ok(AdmittedTruingOutsideRoles {
            proposal: self,
            evidence,
            limits,
            outside,
        })
    }
}
#[derive(Debug)]
struct OutsideAssociation<'a> {
    part: TruingOutsidePart,
    intent: &'a TruingOutsideIntent,
    source: EdgeRange,
    role: ActiveEdgeOutputRole<'a>,
}
/// Authentic complete fixed outside-role attribution,not geometry,whole-Piece placement or issuance.
/// ```compile_fail
/// fn forge<'a>(mut outside:sc_core::construction::AdmittedTruingOutsideRoles<'a>,
///     other:&'a sc_core::construction::RequiredTruingEvidence<'a>) {outside.evidence=other;}
/// ```
/// ```compile_fail
/// fn escape<'a>(evidence:&'a sc_core::construction::RequiredTruingEvidence<'a>)
///     ->sc_core::construction::AdmittedTruingOutsideRoles<'a> {
///     use sc_core::construction::*;let raw=TruingOutsideRoles::new(TruingOutsideIntent::Absent,TruingOutsideIntent::Absent);
///     raw.admit(evidence,TruingOutsideRoleLimits::new(0,EdgeOutputRoleCatalogLimits::new(0,0,0))).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct AdmittedTruingOutsideRoles<'a> {
    proposal: &'a TruingOutsideRoles,
    evidence: &'a RequiredTruingEvidence<'a>,
    limits: TruingOutsideRoleLimits,
    outside: [Option<OutsideAssociation<'a>>; 2],
}
impl<'a> AdmittedTruingOutsideRoles<'a> {
    /// Actual whole original two-field proposal.
    #[must_use]
    pub const fn proposal(&self) -> &'a TruingOutsideRoles {
        self.proposal
    }
    /// Actual complete evidence/lock/outcome/source parents.
    #[must_use]
    pub const fn evidence(&self) -> &'a RequiredTruingEvidence<'a> {
        self.evidence
    }
    /// Original explicit whole N/D/A/B policy.
    #[must_use]
    pub const fn limits(&self) -> TruingOutsideRoleLimits {
        self.limits
    }
    /// All present outside roles in prefix/suffix order,including reverse traversal;at most two.
    pub fn roles(&self) -> impl DoubleEndedIterator<Item = TruingOutsideOutput<'_>> {
        self.outside.iter().filter_map(|slot| {
            slot.as_ref().map(|association| TruingOutsideOutput {
                context: self,
                association,
            })
        })
    }
}
/// Authentic fixed outside portion and original role field;its logical edge remains unissued.
/// ```compile_fail
/// fn forge<'a>(mut output:sc_core::construction::TruingOutsideOutput<'a>,
///     other:&'a sc_core::construction::AdmittedTruingOutsideRoles<'a>) {output.context=other;}
/// ```
#[derive(Clone, Copy, Debug)]
pub struct TruingOutsideOutput<'a> {
    context: &'a AdmittedTruingOutsideRoles<'a>,
    association: &'a OutsideAssociation<'a>,
}
impl<'a> TruingOutsideOutput<'a> {
    /// Actual whole original outside attribution and source owner.
    #[must_use]
    pub const fn context(self) -> &'a AdmittedTruingOutsideRoles<'a> {
        self.context
    }
    /// Prefix/suffix source role,independent of output traversal direction.
    #[must_use]
    pub const fn part(self) -> TruingOutsidePart {
        self.association.part
    }
    /// Actual original explicit proposal field.
    #[must_use]
    pub const fn intent(self) -> &'a TruingOutsideIntent {
        self.association.intent
    }
    /// Exact outside range derived from the actual original held edge and selected bounds.
    #[must_use]
    pub const fn source(self) -> &'a EdgeRange {
        &self.association.source
    }
    /// Actual original active role from its actual admitted catalogue.
    #[must_use]
    pub const fn role(self) -> ActiveEdgeOutputRole<'a> {
        self.association.role
    }
    /// Declared original operation/tag tuple,not a minted topology identity or publication permit.
    #[must_use]
    pub fn logical_output(self) -> EdgeRef {
        EdgeRef::new(
            self.association.role.catalog().catalog().operation(),
            *self.association.role.tag(),
        )
    }
}
