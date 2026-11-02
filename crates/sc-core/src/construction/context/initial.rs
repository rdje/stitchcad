//! Actual initial metadata-record loans and existing canonical joins;no values or source grant.
use super::{DraftingContextSource, DraftingSourceRecord};
use crate::{design::DesignMetadataHost, recipe::FormulaCanonicalInput};
use core::fmt;

/// Raw source response retaining an actual canonical join and original metadata/snapshot records.
/// The existing join preserves actual inventory membership,source family and canonical kind/Length
/// owner without reading Unknown/Derived values. It does not prove that the metadata record exposes
/// this declaration or belongs to the containing Root/snapshot. Core's later binding checks and
/// the immutable honest/exhaustive provider obligation remain required.
///
/// This view borrows original records;no canonical state,metadata payload or snapshot is cloned.
/// Payloads need neither Clone nor Debug. Scalar metadata has no invented Cartesian frame.
/// ```
/// use sc_core::{construction::{DraftingInitialInputSource, DraftingSourceRecord},
///     recipe::FormulaCanonicalInput};
/// fn raw_initial_source<'a,M:?Sized,S:?Sized>(input:FormulaCanonicalInput<'a>,
///     metadata:&'a DraftingSourceRecord<M>,snapshot:&'a DraftingSourceRecord<S>)
///     ->DraftingInitialInputSource<'a,M,S> {
///     let source=DraftingInitialInputSource {input,metadata,snapshot};
///     let copied=source;
///     assert!(copied.metadata.is_same_record(metadata));
///     copied
/// }
/// ```
/// ```compile_fail
/// use sc_core::construction::DraftingInitialInputSource;
/// fn escape<'a>(source:DraftingInitialInputSource<'a,(),()>)
///     ->DraftingInitialInputSource<'static,(),()> {source}
/// ```
pub struct DraftingInitialInputSource<'a, M: ?Sized, S: ?Sized> {
    /// Actual existing canonical source-to-inventory join derived from the original metadata.
    pub input: FormulaCanonicalInput<'a>,
    /// Actual metadata record exposing this name,origin,metadata id and canonical target.
    pub metadata: &'a DraftingSourceRecord<M>,
    /// Original immutable snapshot containing this metadata source under the actual provider.
    pub snapshot: &'a DraftingSourceRecord<S>,
}
impl<M: ?Sized, S: ?Sized> Copy for DraftingInitialInputSource<'_, M, S> {}
impl<M: ?Sized, S: ?Sized> Clone for DraftingInitialInputSource<'_, M, S> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<M: ?Sized, S: ?Sized> fmt::Debug for DraftingInitialInputSource<'_, M, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let declaration = self.input.metadata();
        f.debug_struct("DraftingInitialInputSource")
            .field("kind", &declaration.kind())
            .field("origin", &declaration.origin())
            .finish_non_exhaustive()
    }
}
/// Original typed provider refusal or raw source loans;no accepted metadata/context association.
pub type DraftingInitialInputSourceResult<'a, M, S, E> =
    Result<DraftingInitialInputSource<'a, M, S>, E>;

/// Readonly actual initial-input metadata facet of the immutable containing drafting source.
/// Core supplies the actual Root canonical join after complete content/work bounds. Providers
/// expose the actual original metadata record and its canonical declaration under their actual
/// snapshot;matching reported ids or echoing the caller's claim alone does not establish a source.
///
/// Every held metadata collection,token,retained cache and snapshot belongs in inherited exhaustive
/// resource accounting before queries. Lookup allocates/clones nothing and reads no authored or
/// computed value. Actual unsupported/missing/retired/wrong-origin sources retain original typed
/// causes. Source enumeration and shared access preserve original immutable records. This is an
/// honest integration contract,not a security boundary against dishonest metadata fabrication.
/// Existing point/edge inputs use the separate existing-geometry port;planned creators cannot seed
/// initial geometry. Neither this facet nor the existing canonical join grants a namespace/program.
pub trait DraftingInitialInputProvider<H: DesignMetadataHost>: DraftingContextSource<H> {
    /// Actual provider-owned metadata record type;no Clone/Debug/size requirement.
    type Metadata: ?Sized;
    /// Resolve one original extra-initial occurrence to its actual metadata and canonical source.
    /// The explicit index identifies that original occurrence;it is no caller resource total.
    /// # Errors
    /// Original actual missing/retired/wrong-origin/current-source cause,without a fabricated record.
    fn initial_input<'a>(
        &'a self,
        index: usize,
        input: FormulaCanonicalInput<'a>,
    ) -> DraftingInitialInputSourceResult<'a, Self::Metadata, Self::Snapshot, Self::Error>;
}
