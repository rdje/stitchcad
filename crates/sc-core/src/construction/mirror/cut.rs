//! Explicit new-Piece cut declarations;no copied quantity,handedness or cutting-side default.
use super::BoundMirrorPieceCopy;
use crate::ontology::{CuttingSide, EdgeRef, Mirroring};
use core::fmt;

/// Deliberate result fold treatment,without manufacturing a reflected edge identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirrorPieceFoldIntent {
    /// Explicitly request no fold declaration on the new Piece.
    NoFold,
    /// Require a reflected output attributed to the actual source Piece's original fold edge.
    /// The new edge/correspondence and its geometric suitability remain later obligations.
    ReflectedSource,
}
/// Caller-authored canonical result cut fields;there are no source-derived defaults.
/// Raw construction retains even invalid quantities for contextual admission against the source.
/// ```compile_fail
/// let policy:sc_core::construction::MirrorPieceCutPolicy=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MirrorPieceCutPolicy {
    quantity: u32,
    mirroring: Mirroring,
    fold: MirrorPieceFoldIntent,
    cutting_side: CuttingSide,
}
impl MirrorPieceCutPolicy {
    /// Declare all four result cut fields explicitly,without allocating any Piece or physical copy.
    /// ```
    /// use sc_core::{construction::*,ontology::*};
    /// let policy=MirrorPieceCutPolicy::new(1,Mirroring::Single,
    ///     MirrorPieceFoldIntent::NoFold,CuttingSide::WrongSide);
    /// assert_eq!(*policy.quantity(),1);
    /// assert_eq!(*policy.mirroring(),Mirroring::Single);
    /// ```
    #[must_use]
    pub const fn new(
        quantity: u32,
        mirroring: Mirroring,
        fold: MirrorPieceFoldIntent,
        cutting_side: CuttingSide,
    ) -> Self {
        Self {
            quantity,
            mirroring,
            fold,
            cutting_side,
        }
    }
    /// Original canonical requested total quantity,not a copied source amount.
    #[must_use]
    pub const fn quantity(&self) -> &u32 {
        &self.quantity
    }
    /// Actual requested cut mode,including explicit handedness/companion when separately paired.
    #[must_use]
    pub const fn mirroring(&self) -> &Mirroring {
        &self.mirroring
    }
    /// Actual original result fold policy.
    #[must_use]
    pub const fn fold(&self) -> &MirrorPieceFoldIntent {
        &self.fold
    }
    /// Explicit result cutting side,not inferred by toggling source fabric face.
    #[must_use]
    pub const fn cutting_side(&self) -> &CuttingSide {
        &self.cutting_side
    }
}
/// First local canonical quantity or original-fold association failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirrorPieceCutPolicyRefusal {
    /// The result would request no physical cuts.
    ZeroQuantity,
    /// MirroredPairs requires an even total quantity,including both hands.
    UnpairedQuantity {
        /// Actual odd requested total.
        quantity: u32,
    },
    /// ReflectedSource requires an actual original declared source fold edge.
    MissingSourceFold,
}
/// Original bound source and caller-owned cut policy survive each admission refusal.
pub struct MirrorPieceCutPolicyError<'a> {
    source: &'a BoundMirrorPieceCopy<'a>,
    policy: &'a MirrorPieceCutPolicy,
    refusal: MirrorPieceCutPolicyRefusal,
}
impl<'a> MirrorPieceCutPolicyError<'a> {
    /// Actual original source/input/population association.
    #[must_use]
    pub const fn source(&self) -> &'a BoundMirrorPieceCopy<'a> {
        self.source
    }
    /// Actual unchanged caller policy,including all other requested result fields.
    #[must_use]
    pub const fn policy(&self) -> &'a MirrorPieceCutPolicy {
        self.policy
    }
    /// First local quantity or source-fold cause.
    #[must_use]
    pub const fn refusal(&self) -> MirrorPieceCutPolicyRefusal {
        self.refusal
    }
}
impl fmt::Debug for MirrorPieceCutPolicyError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MirrorPieceCutPolicyError")
            .field("policy", &self.policy)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for MirrorPieceCutPolicyError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mirror Piece cut policy: {:?}", self.refusal)
    }
}
impl std::error::Error for MirrorPieceCutPolicyError<'_> {}

impl<'s> BoundMirrorPieceCopy<'s> {
    /// Associate explicit result cut intent with this actual source,without allocation or mutation.
    /// PairMember companion identity/reciprocity,actual result identity,full CutPlan,fold output,
    /// handedness/face correspondence,semantic effects and geometry remain later requirements.
    /// # Errors
    /// Zero quantity,odd MirroredPairs total,then missing source fold,in that order.
    pub fn bind_cut_policy<'a>(
        &'a self,
        policy: &'a MirrorPieceCutPolicy,
    ) -> Result<BoundMirrorPieceCutPolicy<'a>, MirrorPieceCutPolicyError<'a>>
    where
        's: 'a,
    {
        let refuse = |refusal| MirrorPieceCutPolicyError {
            source: self,
            policy,
            refusal,
        };
        if policy.quantity == 0 {
            return Err(refuse(MirrorPieceCutPolicyRefusal::ZeroQuantity));
        }
        if policy.mirroring == Mirroring::MirroredPairs && !policy.quantity.is_multiple_of(2) {
            return Err(refuse(MirrorPieceCutPolicyRefusal::UnpairedQuantity {
                quantity: policy.quantity,
            }));
        }
        let fold_source = match policy.fold {
            MirrorPieceFoldIntent::NoFold => None,
            MirrorPieceFoldIntent::ReflectedSource => Some(
                self.piece()
                    .definition()
                    .fold_edges
                    .first()
                    .ok_or_else(|| refuse(MirrorPieceCutPolicyRefusal::MissingSourceFold))?,
            ),
        };
        Ok(BoundMirrorPieceCutPolicy {
            source: self,
            policy,
            fold_source,
        })
    }
}
/// Authentic source/canonical-cut request association,not a newly created Piece or CutPlan.
/// ```compile_fail
/// fn forge<'a>(mut cut:sc_core::construction::BoundMirrorPieceCutPolicy<'a>,
///     other:&'a sc_core::construction::MirrorPieceCutPolicy) {
///     cut.policy=other;
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct BoundMirrorPieceCutPolicy<'a> {
    source: &'a BoundMirrorPieceCopy<'a>,
    policy: &'a MirrorPieceCutPolicy,
    fold_source: Option<&'a EdgeRef>,
}
impl<'a> BoundMirrorPieceCutPolicy<'a> {
    /// Actual source and authored mirror input,without currentness or geometric proof.
    #[must_use]
    pub const fn source(&self) -> &'a BoundMirrorPieceCopy<'a> {
        self.source
    }
    /// Actual original canonical result policy.
    #[must_use]
    pub const fn policy(&self) -> &'a MirrorPieceCutPolicy {
        self.policy
    }
    /// Actual requested total quantity;future print/copy views must consume this same authority.
    #[must_use]
    pub const fn quantity(&self) -> &'a u32 {
        self.policy.quantity()
    }
    /// Actual explicit mirroring/handedness/companion request,without an inventory certificate.
    #[must_use]
    pub const fn mirroring(&self) -> &'a Mirroring {
        self.policy.mirroring()
    }
    /// Fold state derived solely from the explicit policy,not independently authored again.
    #[must_use]
    pub const fn cut_on_fold(&self) -> bool {
        matches!(self.policy.fold, MirrorPieceFoldIntent::ReflectedSource)
    }
    /// Exact original fold field that the later output must reflect,when explicitly requested.
    #[must_use]
    pub const fn fold_source(&self) -> Option<&'a EdgeRef> {
        self.fold_source
    }
    /// Actual explicit result cutting side,without a physical face/handedness proof.
    #[must_use]
    pub const fn cutting_side(&self) -> &'a CuttingSide {
        self.policy.cutting_side()
    }
}
