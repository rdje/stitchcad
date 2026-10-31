//! Preserve actual physical-copy identities;population association is not current Design proof.
use super::RequiredMirrorPieceCorrespondence;
use crate::ontology::{CopyOrientation, CutCopy, CutPlan, EntityId, Mirroring, PieceDefinition};
use core::fmt;
mod identities;
pub use identities::{
    BoundMirrorPieceCopyIdentities, MirrorPieceCopyIdentityError, MirrorPieceCopyIdentityLimits,
    MirrorPieceCopyIdentityRefusal,
};

fn original<'a>(context: &RequiredMirrorPieceCorrespondence<'a>) -> &'a PieceDefinition {
    context
        .contours()
        .coverage()
        .edges()
        .identity()
        .roles()
        .metadata()
        .cut()
        .source()
        .piece()
        .definition()
}

/// Explicit complete actual CutPlan population allowance,before any source-copy scan.
/// ```compile_fail
/// let policy:sc_core::construction::MirrorPieceSourceCopyLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MirrorPieceSourceCopyLimits {
    copies: usize,
}
impl MirrorPieceSourceCopyLimits {
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
/// Whole quota,then original source-copy orientation,quantity and paired-population refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirrorPieceSourceCopyRefusal {
    /// Whole plan exceeds explicit allowance,including unrelated copies.
    Copies {
        /// Complete required population.
        required: usize,
        /// Actual explicit allowance.
        allowed: usize,
    },
    /// Reflected source copy contradicts actual Single or separately handed PairMember context.
    UnexpectedReflection {
        /// First actual offending copy in original plan order.
        copy: EntityId,
    },
    /// Target population differs from the actual original authoritative total quantity.
    Quantity {
        /// Original canonical total quantity.
        expected: u32,
        /// Complete observed source population.
        actual: usize,
    },
    /// MirroredPairs source population lacks equal authored/reflected counts.
    UnbalancedPairs {
        /// Complete authored source population.
        authored: usize,
        /// Complete reflected source population.
        reflected: usize,
    },
}
/// Actual whole context context,CutPlan and caller-owned policy survive every refusal.
pub struct MirrorPieceSourceCopyError<'a> {
    context: &'a RequiredMirrorPieceCorrespondence<'a>,
    plan: &'a CutPlan,
    limits: &'a MirrorPieceSourceCopyLimits,
    refusal: MirrorPieceSourceCopyRefusal,
}
impl<'a> MirrorPieceSourceCopyError<'a> {
    /// Actual complete original correspondence/contour/point/source context.
    #[must_use]
    pub const fn context(&self) -> &'a RequiredMirrorPieceCorrespondence<'a> {
        self.context
    }
    /// Actual complete immutable physical-copy owner.
    #[must_use]
    pub const fn plan(&self) -> &'a CutPlan {
        self.plan
    }
    /// Original borrowed explicit whole-copy policy.
    #[must_use]
    pub const fn limits(&self) -> &'a MirrorPieceSourceCopyLimits {
        self.limits
    }
    /// First complete quota or source-population cause.
    #[must_use]
    pub const fn refusal(&self) -> MirrorPieceSourceCopyRefusal {
        self.refusal
    }
}
impl fmt::Debug for MirrorPieceSourceCopyError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MirrorPieceSourceCopyError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for MirrorPieceSourceCopyError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mirror source physical copies: {:?}", self.refusal)
    }
}
impl std::error::Error for MirrorPieceSourceCopyError<'_> {}
impl<'s> RequiredMirrorPieceCorrespondence<'s> {
    /// Preserve every actual original source physical copy after a whole C quota and population association.
    /// Source quantity/mirroring comes from the actual original Piece,never the new result cut policy.
    /// Admission uses O(C) work and constant auxiliary space.
    /// No allocation,identity generation,copy selection,reflection,sewing edit or model mutation.
    /// Current Design/namespace/companion truth and actual all-copy effects remain required evidence.
    /// # Errors
    /// Whole copy quota,first unexpected source reflection,complete source quantity,then pair balance.
    pub fn preserve_source_copies<'a>(
        &'a self,
        plan: &'a CutPlan,
        limits: &'a MirrorPieceSourceCopyLimits,
    ) -> Result<BoundMirrorPieceSourceCopies<'a>, MirrorPieceSourceCopyError<'a>>
    where
        's: 'a,
    {
        let refuse = |refusal| MirrorPieceSourceCopyError {
            context: self,
            plan,
            limits,
            refusal,
        };
        let c = plan.copies().len();
        if c > limits.copies {
            return Err(refuse(MirrorPieceSourceCopyRefusal::Copies {
                required: c,
                allowed: limits.copies,
            }));
        }
        let source = original(self);
        let mut authored = 0usize;
        let mut reflected = 0usize;
        for copy in plan.copies_for_piece(source.id) {
            match copy.orientation() {
                CopyOrientation::Authored => authored += 1,
                CopyOrientation::Reflected => {
                    if source.mirroring != Mirroring::MirroredPairs {
                        return Err(refuse(MirrorPieceSourceCopyRefusal::UnexpectedReflection {
                            copy: copy.id(),
                        }));
                    }
                    reflected += 1;
                }
            }
        }
        // Each disjoint population is bounded by the actual C-entry slice,so their sum fits usize.
        let copy_count = authored + reflected;
        if u32::try_from(copy_count).ok() != Some(source.quantity) {
            return Err(refuse(MirrorPieceSourceCopyRefusal::Quantity {
                expected: source.quantity,
                actual: copy_count,
            }));
        }
        if source.mirroring == Mirroring::MirroredPairs && authored != reflected {
            return Err(refuse(MirrorPieceSourceCopyRefusal::UnbalancedPairs {
                authored,
                reflected,
            }));
        }
        Ok(BoundMirrorPieceSourceCopies {
            context: self,
            plan,
            limits,
            copy_count,
            authored,
            reflected,
        })
    }
}
/// Authentic actual source-copy inventory association;no copy geometry or currentness certificate.
/// ```compile_fail
/// fn escape<'a>(context:&'a sc_core::construction::RequiredMirrorPieceCorrespondence<'a>)
///     ->sc_core::construction::BoundMirrorPieceSourceCopies<'a> {
///     let plan=sc_core::ontology::CutPlan::new(vec![],&[]).unwrap();
///     let limits=sc_core::construction::MirrorPieceSourceCopyLimits::new(0);
///     context.preserve_source_copies(&plan,&limits).unwrap()
/// }
/// ```
/// ```compile_fail
/// fn forge<'a>(mut copies:sc_core::construction::BoundMirrorPieceSourceCopies<'a>,other:&'a sc_core::ontology::CutPlan) {
///     copies.plan=other;
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct BoundMirrorPieceSourceCopies<'a> {
    context: &'a RequiredMirrorPieceCorrespondence<'a>,
    plan: &'a CutPlan,
    limits: &'a MirrorPieceSourceCopyLimits,
    copy_count: usize,
    authored: usize,
    reflected: usize,
}
impl<'a> BoundMirrorPieceSourceCopies<'a> {
    /// Actual complete original correspondence/contour/point/source context.
    #[must_use]
    pub const fn context(&self) -> &'a RequiredMirrorPieceCorrespondence<'a> {
        self.context
    }
    /// Actual complete original CutPlan,including unrelated copies.
    #[must_use]
    pub const fn plan(&self) -> &'a CutPlan {
        self.plan
    }
    /// Actual original whole-copy policy.
    #[must_use]
    pub const fn limits(&self) -> &'a MirrorPieceSourceCopyLimits {
        self.limits
    }
    /// Complete admitted source population,an immutable proof summary rather than another quantity.
    #[must_use]
    pub const fn copy_count(&self) -> usize {
        self.copy_count
    }
    /// Complete actual authored source-copy population,not a result-copy request.
    #[must_use]
    pub const fn authored_count(&self) -> usize {
        self.authored
    }
    /// Complete actual reflected source-copy population,not edge traversal or an applied transform.
    #[must_use]
    pub const fn reflected_count(&self) -> usize {
        self.reflected
    }
    /// All actual source-copy borrows in authored plan order;reverse traversal keeps identities.
    /// Copy orientation is independent of source/output edge traversal. No ids are made from order.
    pub fn copies(&self) -> impl DoubleEndedIterator<Item = &'a CutCopy> {
        let piece = original(self.context).id;
        self.plan
            .copies()
            .iter()
            .filter(move |copy| copy.piece() == piece)
    }
}
