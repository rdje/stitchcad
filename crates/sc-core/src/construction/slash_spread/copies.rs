//! Preserve actual physical-copy identities;population association is not current Design proof.
use super::BoundSlashSpreadPieceMetadata;
use crate::ontology::{CopyOrientation, CutCopy, CutPlan, EntityId, Mirroring};
use core::fmt;

/// Explicit complete actual CutPlan population allowance,before any target-copy scan.
/// ```compile_fail
/// let policy:sc_core::construction::SlashSpreadCopyLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlashSpreadCopyLimits {
    copies: usize,
}
impl SlashSpreadCopyLimits {
    /// Select the whole plan allowance,including copies of unrelated Pieces.
    #[must_use]
    pub const fn new(copies: usize) -> Self {
        Self { copies }
    }
    /// Complete actual copy allowance.
    #[must_use]
    pub const fn copies(self) -> usize {
        self.copies
    }
}
/// Whole quota,then original target-copy orientation,quantity and paired-population refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlashSpreadCopyRefusal {
    /// Whole plan exceeds explicit allowance,including unrelated copies.
    Copies {
        /// Complete required population.
        required: usize,
        /// Actual explicit allowance.
        allowed: usize,
    },
    /// Reflected target copy contradicts actual Single or separately handed PairMember metadata.
    UnexpectedReflection {
        /// First actual offending copy in original plan order.
        copy: EntityId,
    },
    /// Target population differs from the actual inherited authoritative total quantity.
    Quantity {
        /// Original canonical total quantity.
        expected: u32,
        /// Complete observed target population.
        actual: usize,
    },
    /// MirroredPairs target population lacks equal authored/reflected counts.
    UnbalancedPairs {
        /// Complete authored target population.
        authored: usize,
        /// Complete reflected target population.
        reflected: usize,
    },
}
/// Actual whole metadata context,CutPlan and caller-owned policy survive every refusal.
pub struct SlashSpreadCopyError<'a> {
    metadata: &'a BoundSlashSpreadPieceMetadata<'a>,
    plan: &'a CutPlan,
    limits: &'a SlashSpreadCopyLimits,
    refusal: SlashSpreadCopyRefusal,
}
impl<'a> SlashSpreadCopyError<'a> {
    /// Actual complete inherited metadata/boundary/source context.
    #[must_use]
    pub const fn metadata(&self) -> &'a BoundSlashSpreadPieceMetadata<'a> {
        self.metadata
    }
    /// Actual complete immutable physical-copy owner.
    #[must_use]
    pub const fn plan(&self) -> &'a CutPlan {
        self.plan
    }
    /// Original borrowed explicit whole-copy policy.
    #[must_use]
    pub const fn limits(&self) -> &'a SlashSpreadCopyLimits {
        self.limits
    }
    /// First complete quota or target-population cause.
    #[must_use]
    pub const fn refusal(&self) -> SlashSpreadCopyRefusal {
        self.refusal
    }
}
impl fmt::Debug for SlashSpreadCopyError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SlashSpreadCopyError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for SlashSpreadCopyError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "slash/spread physical copies: {:?}", self.refusal)
    }
}
impl std::error::Error for SlashSpreadCopyError<'_> {}
impl<'s> BoundSlashSpreadPieceMetadata<'s> {
    /// Preserve every actual target physical copy after a whole C quota and population association.
    /// No allocation,identity generation,copy selection,reflection,sewing edit or model mutation.
    /// Current Design/namespace/companion truth and actual all-copy effects remain required evidence.
    /// # Errors
    /// Whole copy quota,first unexpected target reflection,complete target quantity,then pair balance.
    pub fn preserve_physical_copies<'a>(
        &'a self,
        plan: &'a CutPlan,
        limits: &'a SlashSpreadCopyLimits,
    ) -> Result<BoundSlashSpreadCopies<'a>, SlashSpreadCopyError<'a>>
    where
        's: 'a,
    {
        let refuse = |refusal| SlashSpreadCopyError {
            metadata: self,
            plan,
            limits,
            refusal,
        };
        let c = plan.copies().len();
        if c > limits.copies {
            return Err(refuse(SlashSpreadCopyRefusal::Copies {
                required: c,
                allowed: limits.copies,
            }));
        }
        let mut authored = 0usize;
        let mut reflected = 0usize;
        for copy in plan.copies_for_piece(*self.piece_id()) {
            match copy.orientation() {
                CopyOrientation::Authored => authored += 1,
                CopyOrientation::Reflected => {
                    if self.mirroring() != &Mirroring::MirroredPairs {
                        return Err(refuse(SlashSpreadCopyRefusal::UnexpectedReflection {
                            copy: copy.id(),
                        }));
                    }
                    reflected += 1;
                }
            }
        }
        // Each disjoint population is bounded by the actual C-entry slice,so their sum fits usize.
        let copy_count = authored + reflected;
        if u32::try_from(copy_count).ok() != Some(*self.quantity()) {
            return Err(refuse(SlashSpreadCopyRefusal::Quantity {
                expected: *self.quantity(),
                actual: copy_count,
            }));
        }
        if self.mirroring() == &Mirroring::MirroredPairs && authored != reflected {
            return Err(refuse(SlashSpreadCopyRefusal::UnbalancedPairs {
                authored,
                reflected,
            }));
        }
        Ok(BoundSlashSpreadCopies {
            metadata: self,
            plan,
            limits,
            copy_count,
        })
    }
}
/// Authentic actual target-copy inventory association;no copy geometry or currentness certificate.
/// ```compile_fail
/// fn forge<'a>(mut copies:sc_core::construction::BoundSlashSpreadCopies<'a>,other:&'a sc_core::ontology::CutPlan) {
///     copies.plan=other;
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct BoundSlashSpreadCopies<'a> {
    metadata: &'a BoundSlashSpreadPieceMetadata<'a>,
    plan: &'a CutPlan,
    limits: &'a SlashSpreadCopyLimits,
    copy_count: usize,
}
impl<'a> BoundSlashSpreadCopies<'a> {
    /// Actual complete inherited metadata/boundary/source context.
    #[must_use]
    pub const fn metadata(&self) -> &'a BoundSlashSpreadPieceMetadata<'a> {
        self.metadata
    }
    /// Actual complete original CutPlan,including unrelated copies.
    #[must_use]
    pub const fn plan(&self) -> &'a CutPlan {
        self.plan
    }
    /// Actual original whole-copy policy.
    #[must_use]
    pub const fn limits(&self) -> &'a SlashSpreadCopyLimits {
        self.limits
    }
    /// Complete admitted target population,an immutable proof summary rather than another quantity.
    #[must_use]
    pub const fn copy_count(&self) -> usize {
        self.copy_count
    }
    /// All actual target-copy borrows in authored plan order;reverse traversal keeps identities.
    /// Copy orientation is independent of source/output edge traversal. No ids are made from order.
    pub fn copies(&self) -> impl DoubleEndedIterator<Item = &'a CutCopy> {
        let piece = *self.metadata.piece_id();
        self.plan
            .copies()
            .iter()
            .filter(move |copy| copy.piece() == piece)
    }
}
