//! Explicit result field selection and canonical print data,without new Piece/copy identities.
use super::BoundMirrorPieceCutPolicy;
use crate::ontology::{LabelField, LabelText, MaterialAssignment, PrintedLabel};
use core::fmt;

/// Deliberate treatment of one mirror-result metadata field;there is no implicit inheritance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MirrorPieceFieldIntent<T> {
    /// Borrow the actual original source field unchanged.
    PreserveSource,
    /// Borrow this explicitly supplied replacement without normalizing its spelling or value.
    Replace(T),
}
impl<T> MirrorPieceFieldIntent<T> {
    fn resolve<'a>(&'a self, source: &'a T) -> &'a T {
        match self {
            Self::PreserveSource => source,
            Self::Replace(value) => value,
        }
    }
}
/// Explicit independent field choices;cut fields remain authoritative in the actual bound cut policy.
/// ```compile_fail
/// let policy:sc_core::construction::MirrorPieceMetadataPolicy=Default::default();
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MirrorPieceMetadataPolicy {
    material: MirrorPieceFieldIntent<MaterialAssignment>,
    layer_index: MirrorPieceFieldIntent<i32>,
    label: MirrorPieceFieldIntent<LabelText>,
}
impl MirrorPieceMetadataPolicy {
    /// Require each preservation/replacement choice explicitly,without copying source metadata.
    #[must_use]
    pub const fn new(
        material: MirrorPieceFieldIntent<MaterialAssignment>,
        layer_index: MirrorPieceFieldIntent<i32>,
        label: MirrorPieceFieldIntent<LabelText>,
    ) -> Self {
        Self {
            material,
            layer_index,
            label,
        }
    }
    /// Actual original material-field choice.
    #[must_use]
    pub const fn material(&self) -> &MirrorPieceFieldIntent<MaterialAssignment> {
        &self.material
    }
    /// Actual original assembly-layer choice.
    #[must_use]
    pub const fn layer_index(&self) -> &MirrorPieceFieldIntent<i32> {
        &self.layer_index
    }
    /// Actual original label-text choice;quantity/hand/fold are never entered here.
    #[must_use]
    pub const fn label(&self) -> &MirrorPieceFieldIntent<LabelText> {
        &self.label
    }
}
/// Explicit allowance for all resolved result label/material-reason text,before blank scans.
/// This does not bound unselected source metadata or already allocated source/policy storage.
/// ```compile_fail
/// let limits:sc_core::construction::MirrorPieceMetadataLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MirrorPieceMetadataLimits {
    text_bytes: usize,
}
impl MirrorPieceMetadataLimits {
    /// Allow the complete UTF-8 byte sum of four label fields and any unresolved material reason.
    #[must_use]
    pub const fn new(text_bytes: usize) -> Self {
        Self { text_bytes }
    }
    /// Complete resolved-result text allowance.
    #[must_use]
    pub const fn text_bytes(self) -> usize {
        self.text_bytes
    }
}
/// First complete resolved-text admission or canonical field-completeness cause.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirrorPieceMetadataRefusal {
    /// Complete selected text byte sum exceeds usize.
    TextOverflow,
    /// Complete selected label/material-reason bytes exceed the explicit allowance.
    TextBytes {
        /// Actual complete resolved byte count.
        required: usize,
        /// Explicit caller allowance.
        allowed: usize,
    },
    /// First blank label field in canonical name/size/fabric/colorway order.
    IncompleteLabel(LabelField),
    /// Selected unresolved material has no nonblank explanation.
    UnexplainedMaterial,
}
/// The actual cut/source context,field policy and explicit limit survive every refusal.
pub struct MirrorPieceMetadataError<'a> {
    cut: &'a BoundMirrorPieceCutPolicy<'a>,
    policy: &'a MirrorPieceMetadataPolicy,
    limits: &'a MirrorPieceMetadataLimits,
    refusal: MirrorPieceMetadataRefusal,
}
impl<'a> MirrorPieceMetadataError<'a> {
    /// Actual original cut/source association.
    #[must_use]
    pub const fn cut(&self) -> &'a BoundMirrorPieceCutPolicy<'a> {
        self.cut
    }
    /// Actual original preservation/replacement choices and values.
    #[must_use]
    pub const fn policy(&self) -> &'a MirrorPieceMetadataPolicy {
        self.policy
    }
    /// Actual caller-owned resolved-text allowance.
    #[must_use]
    pub const fn limits(&self) -> &'a MirrorPieceMetadataLimits {
        self.limits
    }
    /// First whole-text or canonical completeness refusal.
    #[must_use]
    pub const fn refusal(&self) -> MirrorPieceMetadataRefusal {
        self.refusal
    }
}
impl fmt::Debug for MirrorPieceMetadataError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MirrorPieceMetadataError")
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for MirrorPieceMetadataError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mirror Piece metadata: {:?}", self.refusal)
    }
}
impl std::error::Error for MirrorPieceMetadataError<'_> {}

impl<'s> BoundMirrorPieceCutPolicy<'s> {
    /// Bind explicit result field choices and canonical print data without allocation or mutation.
    /// Complete resolved byte lengths precede scans;O(1+B) work and constant auxiliary space.
    /// Selected strings/reasons remain exact. This is local metadata admission,not actual material
    /// existence,state/size truth,output identity,copy/companion or geometric/semantic evidence.
    /// # Errors
    /// Checked complete text sum,whole byte quota,first canonical blank label,then material reason.
    pub fn bind_metadata<'a>(
        &'a self,
        policy: &'a MirrorPieceMetadataPolicy,
        limits: &'a MirrorPieceMetadataLimits,
    ) -> Result<BoundMirrorPieceMetadata<'a>, MirrorPieceMetadataError<'a>>
    where
        's: 'a,
    {
        let refuse = |refusal| MirrorPieceMetadataError {
            cut: self,
            policy,
            limits,
            refusal,
        };
        let source = self.source().piece().definition();
        let material = policy.material.resolve(&source.material);
        let layer_index = policy.layer_index.resolve(&source.layer_index);
        let label = policy.label.resolve(&source.label);
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
        .ok_or_else(|| refuse(MirrorPieceMetadataRefusal::TextOverflow))?;
        if text_bytes > limits.text_bytes {
            return Err(refuse(MirrorPieceMetadataRefusal::TextBytes {
                required: text_bytes,
                allowed: limits.text_bytes,
            }));
        }
        if let Some(field) = label.first_incomplete_field() {
            return Err(refuse(MirrorPieceMetadataRefusal::IncompleteLabel(field)));
        }
        if !material.has_required_reason() {
            return Err(refuse(MirrorPieceMetadataRefusal::UnexplainedMaterial));
        }
        Ok(BoundMirrorPieceMetadata {
            cut: self,
            policy,
            limits,
            material,
            layer_index,
            label,
            text_bytes,
        })
    }
}
/// Authentic selected result fields and a print view derived from the actual canonical cut request.
/// ```compile_fail
/// fn forge<'a>(mut metadata:sc_core::construction::BoundMirrorPieceMetadata<'a>,
///     other:&'a sc_core::ontology::LabelText) {
///     metadata.label=other;
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct BoundMirrorPieceMetadata<'a> {
    cut: &'a BoundMirrorPieceCutPolicy<'a>,
    policy: &'a MirrorPieceMetadataPolicy,
    limits: &'a MirrorPieceMetadataLimits,
    material: &'a MaterialAssignment,
    layer_index: &'a i32,
    label: &'a LabelText,
    text_bytes: usize,
}
impl<'a> BoundMirrorPieceMetadata<'a> {
    /// Actual original canonical cut/source association.
    #[must_use]
    pub const fn cut(&self) -> &'a BoundMirrorPieceCutPolicy<'a> {
        self.cut
    }
    /// Actual original complete field choices.
    #[must_use]
    pub const fn policy(&self) -> &'a MirrorPieceMetadataPolicy {
        self.policy
    }
    /// Actual original whole resolved-text allowance.
    #[must_use]
    pub const fn limits(&self) -> &'a MirrorPieceMetadataLimits {
        self.limits
    }
    /// Actual selected canonical material field,including exact unresolved reason text.
    #[must_use]
    pub const fn material(&self) -> &'a MaterialAssignment {
        self.material
    }
    /// Actual selected canonical assembly layer,without physical placement evidence.
    #[must_use]
    pub const fn layer_index(&self) -> &'a i32 {
        self.layer_index
    }
    /// Actual selected authored text;cut indicators are derived separately from the cut request.
    #[must_use]
    pub const fn label(&self) -> &'a LabelText {
        self.label
    }
    /// Complete admitted result-text byte count,not an overall model-storage guarantee.
    #[must_use]
    pub const fn text_bytes(&self) -> usize {
        self.text_bytes
    }
    /// Print data derives quantity,hand and fold from the same actual canonical cut policy.
    /// This is not a print-layout,material/size readiness,physical or release certificate.
    #[must_use]
    pub fn printed_label(&self) -> PrintedLabel<'a> {
        PrintedLabel {
            text: self.label,
            quantity: *self.cut.quantity(),
            mirroring: *self.cut.mirroring(),
            cut_on_fold: self.cut.cut_on_fold(),
        }
    }
}
