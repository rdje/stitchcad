//! Attributed fullness requirements from original inputs;no second amount or conserved intake.
use super::{
    BoundSlashSpreadCopies, SlashSpreadConstruction, SlashSpreadFullnessIntent,
    SlashSpreadOutputIntent,
};
use crate::{construction::SlashSpreadOpening, ontology::EntityId};

/// Explicit required effect sources,not supplied values,accepted evidence or closure authority.
/// ```compile_fail
/// let request:sc_core::construction::SlashSpreadEffectRequirement=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlashSpreadEffectRequirement {
    derivation: EntityId,
    attribution: EntityId,
    canonical_intakes: EntityId,
}
impl SlashSpreadEffectRequirement {
    /// Request actual derived effect,attribution to the same original operation inputs and whole
    /// canonical-intake/closure reconciliation. No independent Length/Area/intake or default exists.
    #[must_use]
    pub const fn new(
        derivation: EntityId,
        attribution: EntityId,
        canonical_intakes: EntityId,
    ) -> Self {
        Self {
            derivation,
            attribution,
            canonical_intakes,
        }
    }
    /// Original source requesting the actual effect derivation and supported observable/domain.
    #[must_use]
    pub const fn derivation(&self) -> &EntityId {
        &self.derivation
    }
    /// Original source requesting attribution to this actual operation's original inputs.
    #[must_use]
    pub const fn attribution(&self) -> &EntityId {
        &self.attribution
    }
    /// Original source requesting canonical Dart/Tuck/Pleat/Gather intake/closure reconciliation.
    #[must_use]
    pub const fn canonical_intakes(&self) -> &EntityId {
        &self.canonical_intakes
    }
}
impl<'s> BoundSlashSpreadCopies<'s> {
    /// Require attributed fullness derivation for all actual admitted target copies and their
    /// semantic/sewing consumers. The original all-copy evidence request remains authoritative.
    /// Fixed borrowed association allocates/evaluates nothing and accepts no provider evidence.
    #[must_use]
    pub fn require_attributed_effect<'a>(
        &'a self,
        requirement: &'a SlashSpreadEffectRequirement,
    ) -> RequiredSlashSpreadEffect<'a>
    where
        's: 'a,
    {
        RequiredSlashSpreadEffect {
            copies: self,
            requirement,
        }
    }
}
/// Authentic original effect/input and complete copy context;not an achieved effect certificate.
/// ```compile_fail
/// fn forge<'a>(mut effect:sc_core::construction::RequiredSlashSpreadEffect<'a>,other:&'a sc_core::construction::SlashSpreadEffectRequirement) {
///     effect.requirement=other;
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct RequiredSlashSpreadEffect<'a> {
    copies: &'a BoundSlashSpreadCopies<'a>,
    requirement: &'a SlashSpreadEffectRequirement,
}
impl<'a> RequiredSlashSpreadEffect<'a> {
    /// Actual complete admitted copy/metadata/boundary/source context.
    #[must_use]
    pub const fn copies(&self) -> &'a BoundSlashSpreadCopies<'a> {
        self.copies
    }
    /// Actual original derivation/attribution/canonical-intake source requests.
    #[must_use]
    pub const fn requirement(&self) -> &'a SlashSpreadEffectRequirement {
        self.requirement
    }
    /// Actual original output intent,without a duplicate fullness or boundary policy.
    #[must_use]
    pub fn output_intent(&self) -> &'a SlashSpreadOutputIntent {
        self.copies
            .metadata()
            .boundary()
            .consumers()
            .registration_plan()
            .sources()
            .contours()
            .edges()
            .bound_roles()
            .intent()
    }
    /// Actual original Added/Removed/Unchanged request,not an inferred sign or measured result.
    #[must_use]
    pub fn requested_fullness(&self) -> &'a SlashSpreadFullnessIntent {
        self.output_intent().fullness()
    }
    /// Actual original Piece/cut/bank/pivot-or-axis/opening owner,without reparsing or copying.
    #[must_use]
    pub fn input(&self) -> &'a SlashSpreadConstruction {
        self.output_intent().input()
    }
    /// Actual original opening component,not a second solved amount or inferred effect.
    #[must_use]
    pub fn opening(&self) -> &'a SlashSpreadOpening {
        self.input().opening()
    }
    /// Actual original all-copy-effect request from canonical metadata preservation.
    #[must_use]
    pub fn all_copy_effects_source(&self) -> &'a EntityId {
        self.copies.metadata().requirement().all_copy_effects()
    }
}
