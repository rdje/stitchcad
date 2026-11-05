//! Explicit immutable drafting sources; raw returned records are not context acceptance.
use crate::{
    command::DesignRevision,
    design::{DesignInputs, DesignMetadataHost, DesignResourceLimitError, DesignResourceSink},
    ontology::{EdgeRef, PointRef},
};
use core::fmt;
mod binding;
pub use binding::{DraftingContext, DraftingContextError};
mod geometry;
pub use geometry::{
    DraftingGeometryBinding, DraftingGeometryBindingIssue, DraftingGeometryError,
    DraftingGeometryMismatch,
};
mod record;
pub use record::DraftingSourceRecord;
mod initial;
mod planned;
pub use initial::{
    DraftingInitialInputProvider, DraftingInitialInputSource, DraftingInitialInputSourceResult,
};
pub use planned::{
    DraftingGeometryStepProvider, DraftingPlannedOutput, DraftingPlannedOutputProvider,
    DraftingPlannedOutputSource, DraftingPlannedOutputSourceResult,
};

/// Exact requested geometric output kind and persistent reference, without a live lookup.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DraftingGeometryReference {
    /// An actual point output of its original creating operation.
    Point(PointRef),
    /// An actual edge output of its original creating operation.
    Edge(EdgeRef),
}

/// Raw source-port response borrowing the actual records, not an accepted geometry binding.
///
/// Every field is explicit. A provider can return this response before core checks its reference,
/// frame and snapshot against the requested context. Construction issues no identity and proves
/// no currentness, owner, revision, frame compatibility, geometric value or numeric evidence.
/// Copying this view copies only its reference and borrows, including for unsized source records.
///
/// Provider records need neither `Copy` nor `Debug`; their private payload stays out of this view's
/// debug output. This example constructs only raw fixture data, not a geometry snapshot:
/// ```
/// use sc_core::{construction::{DraftingGeometryReference, DraftingGeometrySource, DraftingSourceRecord},
///     ontology::{EntityId, LocalTag, PointRef}};
/// struct Snapshot;
/// let frame = DraftingSourceRecord::new("private authored frame definition");
/// let geometry: &[u8] = &[41, 42, 43];
/// let snapshot = DraftingSourceRecord::new(Snapshot);
/// let source = DraftingGeometrySource {
///     reference: DraftingGeometryReference::Point(
///         PointRef::new(EntityId::from_bits(10), LocalTag::new(2))),
///     frame: &frame, snapshot: &snapshot, geometry,
/// };
/// let copied = source;
/// assert!(std::ptr::eq(copied.geometry, geometry));
/// assert!(std::ptr::eq(copied.snapshot, &snapshot));
/// assert!(!format!("{copied:?}").contains("private authored frame"));
/// ```
///
/// A source cannot outlive its actual geometric record:
/// ```compile_fail
/// use sc_core::construction::DraftingGeometrySource;
/// fn escape<'a>(source: DraftingGeometrySource<'a, (), (), String>)
///     -> DraftingGeometrySource<'static, (), (), String> { source }
/// ```
/// A held source also prevents replacing its actual frame before the view's last use:
/// ```compile_fail,E0506
/// use sc_core::construction::{DraftingGeometryReference, DraftingGeometrySource, DraftingSourceRecord};
/// fn replace(frame: &mut DraftingSourceRecord<String>, reference: DraftingGeometryReference) {
///     let source = DraftingGeometrySource {
///         reference, frame: &*frame, snapshot: &DraftingSourceRecord::new(()), geometry: &(),
///     };
///     *frame = DraftingSourceRecord::new(String::new());
///     let _ = source.frame.data().len();
/// }
/// ```
pub struct DraftingGeometrySource<'a, F: ?Sized, S: ?Sized, G: ?Sized> {
    /// Actual record's typed persistent output reference; not a replacement for the request.
    pub reference: DraftingGeometryReference,
    /// Actual authored Cartesian frame source of this record, not an edge parameter frame.
    pub frame: &'a DraftingSourceRecord<F>,
    /// Actual immutable snapshot containing this original record.
    pub snapshot: &'a DraftingSourceRecord<S>,
    /// Actual geometry source record; its data is neither cloned nor evaluated by this carrier.
    pub geometry: &'a G,
}
impl<F: ?Sized, S: ?Sized, G: ?Sized> Copy for DraftingGeometrySource<'_, F, S, G> {}
impl<F: ?Sized, S: ?Sized, G: ?Sized> Clone for DraftingGeometrySource<'_, F, S, G> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<F: ?Sized, S: ?Sized, G: ?Sized> fmt::Debug for DraftingGeometrySource<'_, F, S, G> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingGeometrySource")
            .field("reference", &self.reference)
            .finish_non_exhaustive()
    }
}

/// A raw borrowed geometry-source response or the provider's original typed refusal.
/// This alias adds no source-binding validation or context acceptance.
pub type DraftingGeometrySourceResult<'a, F, S, G, E> =
    Result<DraftingGeometrySource<'a, F, S, G>, E>;

/// Readonly port into actual containing Design, Cartesian-frame and geometry-snapshot sources.
///
/// Implementations retain their exact immutable root, snapshot revision and selected authored
/// frame. Enumeration/accounting must be exhaustive and stable, and shared access must preserve
/// these original sources. Numeric ids or equal-looking copied models do not establish binding.
/// Like existing Design source ports, this contract relies on an honest implementation; it is
/// not a security boundary against an implementation that invents records or hides content.
///
/// Frame/snapshot/geometry types belong to the provider. This interface supplies no default
/// coordinate system, transform, synthetic registry, geometry algorithm or accepted context.
/// Earlier declared program outputs stay outside this existing-geometry lookup until execution
/// creates them. Canonical semantic-target resolution remains a separate dependency obligation.
pub trait DraftingContextSource<H: DesignMetadataHost> {
    /// Actual authored Cartesian definition, held in its original owned source record.
    type Frame: ?Sized;
    /// Actual complete immutable snapshot payload, held in its original owned source record.
    type Snapshot: ?Sized;
    /// Original geometry record type, retaining creator/kind/frame provenance and actual data.
    type Geometry: ?Sized;
    /// Original typed current-source or lookup refusal, without fabricated formula tokens.
    type Error: std::error::Error + 'static;

    /// Exact containing Design owner; the core must compare the actual owner before queries.
    fn root(&self) -> &DesignInputs<H>;

    /// Actual snapshot's represented revision, borrowed without formatting or cloning counters.
    fn snapshot_revision(&self) -> &DesignRevision;

    /// Actual explicitly selected Cartesian frame source; choose no fallback or ambient frame.
    fn frame(&self) -> &DraftingSourceRecord<Self::Frame>;

    /// Actual complete immutable snapshot, not a caller-supplied liveness/certification flag.
    fn snapshot(&self) -> &DraftingSourceRecord<Self::Snapshot>;

    /// Charge all held context content before validation or geometry queries.
    ///
    /// Precharge owned collections before traversal, then every retained text field, duplicated
    /// snapshot/cache and represented unbounded counter in stable source/authored order. Charge
    /// the frame and snapshot definitions too; core-owned Design content composes separately.
    /// Use `crate::design::charge_design_revision_resources` for each retained DesignRevision,
    /// including the snapshot's revision, without decimal formatting or supplied counter totals.
    /// Use original locations under the containing input scope. Do not clone, normalize, query
    /// geometry, validate currentness or supply a precomputed total instead of walking content.
    /// The restricted sink retains its first refusal even if an implementation ignores it.
    /// # Errors
    /// Returns the original first logical-content refusal, without changing the source snapshot.
    fn charge_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError>;

    /// Validate the actual complete snapshot's root/revision/frame and typed current provenance.
    ///
    /// This is a structural/current-source query after complete content admission. It reads no
    /// coordinates, arc lengths or formula values, and certifies no geometric/physical feasibility.
    /// The core's own source-binding checks remain required; a successful callback is not itself
    /// a public accepted-context constructor.
    /// # Errors
    /// Returns the original missing, retired, wrong-kind, frame or unsupported source refusal.
    fn validate_current(&self) -> Result<(), Self::Error>;

    /// Resolve an existing typed output to its actual held reference, frame, snapshot and record.
    ///
    /// Return original source handles, not copied substitutes or a nearest surviving output.
    /// No planned-output materialization, numeric geometry query, implicit transform, reference
    /// repair or identity issuance occurs. The raw result still needs core binding checks.
    /// # Errors
    /// Returns the original typed request/current-source refusal, without an empty success.
    fn geometry(
        &self,
        reference: DraftingGeometryReference,
    ) -> DraftingGeometrySourceResult<'_, Self::Frame, Self::Snapshot, Self::Geometry, Self::Error>;
}

mod component;
mod design_semantic;
mod garment;
mod profile;
mod rule;
pub use component::{
    AdmittedDraftingComponentWork, BoundDraftingComponentSource, DraftingComponentBindingLimits,
    DraftingComponentBindingShape, DraftingComponentCreator, DraftingComponentOrigin,
    DraftingComponentProvider, DraftingComponentRecipeSource, DraftingComponentResponse,
    DraftingComponentSource, DraftingComponentSourceError, DraftingComponentSourceIssue,
    DraftingComponentSourceRefusal, DraftingComponentWorkError, DraftingComponentWorkRefusal,
    DraftingImportedComponentSource,
};
pub use design_semantic::{
    DraftingDesignLogicalSource, DraftingDesignNumericSource, DraftingDesignOperationSource,
    DraftingDesignSemanticLookupCause, DraftingDesignSemanticLookupError,
    DraftingDesignSemanticRequest, DraftingDesignSemanticScope, DraftingDesignSemanticScopeError,
};
pub use garment::{
    AdmittedDraftingGarmentSource, BoundDraftingClosureButtonhole, DraftingClosureButtonholeError,
    DraftingClosureButtonholeLimits, DraftingClosureButtonholeRefusal,
    DraftingClosureButtonholeShape, DraftingClosureSizeRole, DraftingGarmentProvider,
    DraftingGarmentSemanticField, DraftingGarmentSemanticOwner, DraftingGarmentSemanticUse,
    DraftingGarmentSource, DraftingGarmentSourceError, DraftingGarmentSourceIssue,
};
pub use profile::{
    BoundDraftingProfileLogicalSource, BoundDraftingProfileNumericSource,
    BoundDraftingProfileSource, DraftingProfileLogicalError, DraftingProfileLogicalIssue,
    DraftingProfileLogicalProvider, DraftingProfileLogicalRefusal, DraftingProfileLogicalResponse,
    DraftingProfileLogicalSource, DraftingProfileNumericError, DraftingProfileNumericIssue,
    DraftingProfileNumericProvider, DraftingProfileNumericRefusal, DraftingProfileNumericResponse,
    DraftingProfileNumericSource, DraftingProfileProvider, DraftingProfileSource,
    DraftingProfileSourceError, DraftingProfileSourceIssue, DraftingProfileSourceRefusal,
};
pub use rule::{
    AdmittedDraftingRuleWork, BoundDraftingRuleSignature, BoundDraftingRuleSource,
    DraftingRuleBindingLimits, DraftingRuleBindingShape, DraftingRuleInputMatch,
    DraftingRuleInputPair, DraftingRuleOrigin, DraftingRuleProvider, DraftingRuleRequest,
    DraftingRuleResponse, DraftingRuleSelection, DraftingRuleSignatureError,
    DraftingRuleSignatureIssue, DraftingRuleSignatureRefusal, DraftingRuleSource,
    DraftingRuleSourceError, DraftingRuleSourceIssue, DraftingRuleSourceRefusal,
    DraftingRuleWorkError, DraftingRuleWorkRefusal,
};
