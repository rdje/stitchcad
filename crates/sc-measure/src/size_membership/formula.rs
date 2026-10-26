//! Formula size values derived from actual immutable authored membership, without axis inference.
use super::{SizeMember, SizeMembership, SizeMembershipError, SizeSetReference};
use core::fmt;
use sc_core::{
    ontology::EntityId,
    recipe::{
        FormulaSizeContext, FormulaSizeContextError, FormulaSizeContextSource, FormulaSizeName,
        FormulaSizeReadError, FormulaSizeValue,
    },
};

/// Expected pin, selected member or derived core-context construction failed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SizeFormulaContextError {
    /// Explicit requested reference differs from the actual membership snapshot.
    ReferenceMismatch {
        /// Caller-supplied expected pinned reference.
        requested: SizeSetReference,
        /// Actual immutable membership reference.
        actual: SizeSetReference,
        /// Actual requested member; no membership is inferred before the pin check.
        member: EntityId,
    },
    /// Actual selected member is absent from the matched membership.
    Membership(SizeMembershipError),
    /// One-based ordinal cannot be represented without wrapping.
    OrdinalOverflow {
        /// Actual matched reference.
        reference: SizeSetReference,
        /// Requested selected member.
        member: EntityId,
        /// Actual zero-based position before the refused addition.
        position: u128,
    },
    /// Actual derived frame failed the core's structural checks; never replaced with a default.
    /// Boxing keeps the uncommon full source/range witness out of the ordinary error footprint.
    Context(Box<FormulaSizeContextError>),
}

impl fmt::Display for SizeFormulaContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReferenceMismatch {
                requested,
                actual,
                member,
            } => write!(
                f,
                "size formula member {member} requests {}@{}, actual {}@{}",
                requested.id, requested.revision, actual.id, actual.revision
            ),
            Self::Membership(error) => fmt::Display::fmt(error, f),
            Self::OrdinalOverflow {
                reference,
                member,
                position,
            } => write!(
                f,
                "size set {} member {member} position {position} cannot form a one-based ordinal",
                reference.id
            ),
            Self::Context(error) => fmt::Display::fmt(error, f),
        }
    }
}
impl std::error::Error for SizeFormulaContextError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Membership(error) => Some(error),
            Self::Context(error) => Some(error.as_ref()),
            Self::ReferenceMismatch { .. } | Self::OrdinalOverflow { .. } => None,
        }
    }
}

/// Canonical membership/member borrows beside exact derived core size values.
/// Matching a supplied expected pin does not prove registry currentness, axes or path readiness.
#[derive(Clone, Copy)]
pub struct SizeFormulaContext<'a> {
    membership: &'a SizeMembership,
    member: &'a SizeMember,
    context: FormulaSizeContext,
}

impl<'a> SizeFormulaContext<'a> {
    /// Actual immutable authored membership that supplied order/count/base and revision.
    #[must_use]
    pub const fn membership(&self) -> &'a SizeMembership {
        self.membership
    }

    /// Actual selected member, retaining its stable identity and exact human label.
    #[must_use]
    pub const fn member(&self) -> &'a SizeMember {
        self.member
    }

    /// Exact core context derived from this snapshot, suitable for a core-only evaluator.
    #[must_use]
    pub const fn context(&self) -> FormulaSizeContext {
        self.context
    }

    /// Reads the requested role without parsing labels or recomputing authored content.
    /// # Errors
    /// Propagates core context absence; construction supplies a present frame, with no fallback.
    pub fn read(&self, name: FormulaSizeName) -> Result<FormulaSizeValue, FormulaSizeReadError> {
        self.context.read(name)
    }
}

impl fmt::Debug for SizeFormulaContext<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Labels remain explicit member access, rather than accidental metadata logs.
        f.debug_struct("SizeFormulaContext")
            .field("context", &self.context)
            .finish_non_exhaustive()
    }
}

impl SizeMembership {
    /// Matches the supplied expected pin, selects by actual stable id, then derives size values.
    ///
    /// Position/count come only from the canonical authored vector. Base is the explicit identity;
    /// label bytes and designation-system intent do not enter computation. Core remains independent
    /// of this crate. The caller still owns registry/currentness and full SizeSet/path validation.
    /// # Errors
    /// Refuses expected-reference mismatch before member lookup, then an absent selected member,
    /// ordinal overflow or an actual core-frame refusal. No current reference/member is guessed.
    pub fn formula_context(
        &self,
        expected: SizeSetReference,
        selected: EntityId,
    ) -> Result<SizeFormulaContext<'_>, SizeFormulaContextError> {
        let actual = self.reference();
        if expected != actual {
            return Err(SizeFormulaContextError::ReferenceMismatch {
                requested: expected,
                actual,
                member: selected,
            });
        }
        let definition = self.definition();
        let (position, member) = definition
            .members
            .iter()
            .enumerate()
            .find(|(_, member)| member.id == selected)
            .ok_or(SizeFormulaContextError::Membership(
                SizeMembershipError::MissingMember {
                    set: actual.id,
                    member: selected,
                },
            ))?;
        let position = position as u128;
        let index = position
            .checked_add(1)
            .ok_or(SizeFormulaContextError::OrdinalOverflow {
                reference: actual,
                member: selected,
                position,
            })?;
        let context = FormulaSizeContext::new(
            FormulaSizeContextSource {
                set: actual.id,
                revision: actual.revision,
                member: member.id,
                base: definition.base,
            },
            index,
            definition.members.len() as u128,
        )
        .map_err(|error| SizeFormulaContextError::Context(Box::new(error)))?;
        Ok(SizeFormulaContext {
            membership: self,
            member,
            context,
        })
    }
}
