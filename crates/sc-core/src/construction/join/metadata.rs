//! Explicit two-source result metadata selection,without copy/fold/mark reconciliation.
use super::RequiredJoinExactSplice;
use crate::ontology::{CuttingSide, LabelField, LabelText, MaterialAssignment, SeamSideId};
use core::fmt;

/// Deliberate origin of one joined metadata field;no implicit winner between A and B.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JoinPieceFieldIntent<T> {
    /// Borrow the actual original field from this canonical A/B role.
    FromSide(SeamSideId),
    /// Borrow this explicitly authored value without normalization or copied provenance.
    Authored(T),
}
impl<T> JoinPieceFieldIntent<T> {
    fn resolve<'a>(&'a self, a: &'a T, b: &'a T) -> &'a T {
        match self {
            Self::FromSide(SeamSideId::A) => a,
            Self::FromSide(SeamSideId::B) => b,
            Self::Authored(value) => value,
        }
    }
}
/// Four required independent field choices;quantity/mirroring/fold indicators remain separate.
/// ```compile_fail
/// let policy:sc_core::construction::JoinPieceMetadataPolicy=Default::default();
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JoinPieceMetadataPolicy {
    material: JoinPieceFieldIntent<MaterialAssignment>,
    layer_index: JoinPieceFieldIntent<i32>,
    label: JoinPieceFieldIntent<LabelText>,
    cutting_side: JoinPieceFieldIntent<CuttingSide>,
}
impl JoinPieceMetadataPolicy {
    /// Require all original source/replacement choices without copying any source metadata.
    #[must_use]
    pub const fn new(
        material: JoinPieceFieldIntent<MaterialAssignment>,
        layer_index: JoinPieceFieldIntent<i32>,
        label: JoinPieceFieldIntent<LabelText>,
        cutting_side: JoinPieceFieldIntent<CuttingSide>,
    ) -> Self {
        Self {
            material,
            layer_index,
            label,
            cutting_side,
        }
    }
    /// Actual original material assignment choice.
    #[must_use]
    pub const fn material(&self) -> &JoinPieceFieldIntent<MaterialAssignment> {
        &self.material
    }
    /// Actual original assembly-layer choice.
    #[must_use]
    pub const fn layer_index(&self) -> &JoinPieceFieldIntent<i32> {
        &self.layer_index
    }
    /// Actual original four-field authored label-text choice.
    #[must_use]
    pub const fn label(&self) -> &JoinPieceFieldIntent<LabelText> {
        &self.label
    }
    /// Actual original cutting-face choice;no reflection/placement inference.
    #[must_use]
    pub const fn cutting_side(&self) -> &JoinPieceFieldIntent<CuttingSide> {
        &self.cutting_side
    }
}
/// Complete selected result label/material-reason UTF-8 allowance before canonical blank scans.
/// ```compile_fail
/// let limits:sc_core::construction::JoinPieceMetadataLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinPieceMetadataLimits {
    text_bytes: usize,
}
impl JoinPieceMetadataLimits {
    /// Bound all four selected label fields plus the selected unresolved material reason.
    #[must_use]
    pub const fn new(text_bytes: usize) -> Self {
        Self { text_bytes }
    }
    /// Complete selected result UTF-8 byte allowance,not prior/unselected input storage.
    #[must_use]
    pub const fn text_bytes(self) -> usize {
        self.text_bytes
    }
}
/// First whole selected-text or canonical completeness refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinPieceMetadataRefusal {
    /// Checked selected text sum cannot be represented.
    TextOverflow,
    /// Complete selected text exceeds the explicit allowance.
    TextBytes {
        /// Full selected UTF-8 count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// First canonical blank label field,in name/size/fabric/colorway order.
    IncompleteLabel(LabelField),
    /// Selected unresolved material lacks a nonblank reason.
    UnexplainedMaterial,
}
/// Whole original splice,field policy,limits and actual first cause survive refusal.
pub struct JoinPieceMetadataError<'a> {
    splice: &'a RequiredJoinExactSplice<'a>,
    policy: &'a JoinPieceMetadataPolicy,
    limits: &'a JoinPieceMetadataLimits,
    refusal: JoinPieceMetadataRefusal,
}
impl<'a> JoinPieceMetadataError<'a> {
    /// Actual original complete join/splice/source context.
    #[must_use]
    pub const fn splice(&self) -> &'a RequiredJoinExactSplice<'a> {
        self.splice
    }
    /// Actual original field choices and authored replacements.
    #[must_use]
    pub const fn policy(&self) -> &'a JoinPieceMetadataPolicy {
        self.policy
    }
    /// Actual original whole selected-text allowance.
    #[must_use]
    pub const fn limits(&self) -> &'a JoinPieceMetadataLimits {
        self.limits
    }
    /// First complete population or canonical field-completeness cause.
    #[must_use]
    pub const fn refusal(&self) -> JoinPieceMetadataRefusal {
        self.refusal
    }
}
impl fmt::Debug for JoinPieceMetadataError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinPieceMetadataError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for JoinPieceMetadataError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "join piece metadata: {:?}", self.refusal)
    }
}
impl std::error::Error for JoinPieceMetadataError<'_> {}

impl<'s> RequiredJoinExactSplice<'s> {
    /// Bind selected actual A/B or authored field values under complete selected-text admission.
    /// Cached String lengths determine full B before canonical label/reason blank scans. Earlier
    /// storage and unselected source/policy text remain separate;they are neither scanned nor copied.
    /// O(B+1) work,constant space,no allocation/provider/geometry call. Material existence/kind,
    /// size/profile/face compatibility,cut/fold/mark policy,currentness and authority stay required.
    /// # Errors
    /// Checked whole B,then B quota,then canonical label order and unresolved-material reason.
    pub fn bind_metadata<'a>(
        &'a self,
        policy: &'a JoinPieceMetadataPolicy,
        limits: &'a JoinPieceMetadataLimits,
    ) -> Result<BoundJoinPieceMetadata<'a>, JoinPieceMetadataError<'a>>
    where
        's: 'a,
    {
        let refuse = |refusal| JoinPieceMetadataError {
            splice: self,
            policy,
            limits,
            refusal,
        };
        let a = self.source().side(SeamSideId::A).piece().definition();
        let b = self.source().side(SeamSideId::B).piece().definition();
        let material = policy.material.resolve(&a.material, &b.material);
        let layer_index = policy.layer_index.resolve(&a.layer_index, &b.layer_index);
        let label = policy.label.resolve(&a.label, &b.label);
        let cutting_side = policy
            .cutting_side
            .resolve(&a.cutting_side, &b.cutting_side);
        let reason_bytes = match material {
            MaterialAssignment::Assigned(_) => 0,
            MaterialAssignment::Unresolved(reason) => reason.len(),
        };
        let text_bytes = [
            label.name.len(),
            label.size.len(),
            label.fabric.len(),
            label.colorway.len(),
            reason_bytes,
        ]
        .into_iter()
        .try_fold(0usize, usize::checked_add)
        .ok_or_else(|| refuse(JoinPieceMetadataRefusal::TextOverflow))?;
        if text_bytes > limits.text_bytes {
            return Err(refuse(JoinPieceMetadataRefusal::TextBytes {
                required: text_bytes,
                allowed: limits.text_bytes,
            }));
        }
        if let Some(field) = label.first_incomplete_field() {
            return Err(refuse(JoinPieceMetadataRefusal::IncompleteLabel(field)));
        }
        if !material.has_required_reason() {
            return Err(refuse(JoinPieceMetadataRefusal::UnexplainedMaterial));
        }
        Ok(BoundJoinPieceMetadata {
            splice: self,
            policy,
            limits,
            material,
            layer_index,
            label,
            cutting_side,
            text_bytes,
        })
    }
}
/// Authentic complete selected result fields;not a born Piece,print view or compatibility proof.
/// ```compile_fail
/// fn forge<'a>(mut fields:sc_core::construction::BoundJoinPieceMetadata<'a>,
///     other:&'a sc_core::ontology::LabelText) {fields.label=other;}
/// ```
/// ```compile_fail
/// fn escape<'a>(splice:&'a sc_core::construction::RequiredJoinExactSplice<'a>,
///     policy:&'a sc_core::construction::JoinPieceMetadataPolicy)->sc_core::construction::BoundJoinPieceMetadata<'a> {
///     let limits=sc_core::construction::JoinPieceMetadataLimits::new(20);
///     splice.bind_metadata(policy,&limits).unwrap()
/// }
/// ```
pub struct BoundJoinPieceMetadata<'a> {
    splice: &'a RequiredJoinExactSplice<'a>,
    policy: &'a JoinPieceMetadataPolicy,
    limits: &'a JoinPieceMetadataLimits,
    material: &'a MaterialAssignment,
    layer_index: &'a i32,
    label: &'a LabelText,
    cutting_side: &'a CuttingSide,
    text_bytes: usize,
}
impl<'a> BoundJoinPieceMetadata<'a> {
    /// Actual original whole splice and source/copy inventory context.
    #[must_use]
    pub const fn splice(&self) -> &'a RequiredJoinExactSplice<'a> {
        self.splice
    }
    /// Actual original field choices and authored values.
    #[must_use]
    pub const fn policy(&self) -> &'a JoinPieceMetadataPolicy {
        self.policy
    }
    /// Actual original complete selected-text allowance.
    #[must_use]
    pub const fn limits(&self) -> &'a JoinPieceMetadataLimits {
        self.limits
    }
    /// Actual selected original material field,including unchanged unresolved reason.
    #[must_use]
    pub const fn material(&self) -> &'a MaterialAssignment {
        self.material
    }
    /// Actual selected original assembly-layer field.
    #[must_use]
    pub const fn layer_index(&self) -> &'a i32 {
        self.layer_index
    }
    /// Actual selected four-field label text;cut indicators remain separately authoritative.
    #[must_use]
    pub const fn label(&self) -> &'a LabelText {
        self.label
    }
    /// Actual selected cutting-face field,without placement/parity compatibility evidence.
    #[must_use]
    pub const fn cutting_side(&self) -> &'a CuttingSide {
        self.cutting_side
    }
    /// Complete admitted selected result UTF-8 count,not all source storage.
    #[must_use]
    pub const fn text_bytes(&self) -> usize {
        self.text_bytes
    }
}
impl fmt::Debug for BoundJoinPieceMetadata<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundJoinPieceMetadata")
            .field("text_bytes", &self.text_bytes)
            .finish_non_exhaustive()
    }
}
