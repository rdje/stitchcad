//! Original numeric metadata associations,without canonical membership or parameter acceptance.
use super::{DraftingContextSource, DraftingProfileProvider, DraftingProfileSource};
use crate::{
    construction::DraftingSourceRecord, design::DesignMetadataHost, ontology::ProfileParameterRef,
    value::CanonicalInputTarget,
};
use core::fmt;

/// Raw original Profile numeric parameter-to-target metadata,independent of the logical facet.
/// `parameter` and `target` loan actual fields of `metadata`;an echoed request or invented target
/// is not an authentic association. The target retains its original dedicated Length or general
/// Scalar family and scalar annotation. The actual canonical member still needs resolution under
/// this exact Profile's inventory;metadata target IDs never infer a same-ID Design member.
///
/// This carrier accepts inspectable mismatches and missing targets. It proves no parent/member/
/// expected-kind binding,content or work allowance,value,Derived supplier or G4 evidence/schema
/// truth. Original uncertainty/state/provenance stays in the source inventory. Opaque metadata,
/// unused records,copies and caches all belong in the provider's exhaustive fresh resource walk.
/// Payloads may be unsized and need neither Clone nor Debug. Copying the view copies only loans:
/// ```
/// use sc_core::{construction::*,design::DesignMetadataHost,ontology::ProfileParameterRef,
///     value::CanonicalInputTarget};
/// fn raw<'a,H:DesignMetadataHost>(profile:DraftingProfileSource<'a,H,[u8],[u8]>,
///     parameter:&'a ProfileParameterRef,target:&'a CanonicalInputTarget,
///     metadata:&'a DraftingSourceRecord<[u8]>) -> DraftingProfileNumericSource<'a,H,[u8],[u8],[u8]> {
///     let source=DraftingProfileNumericSource {profile,parameter,target,metadata};
///     let copied=source;
///     assert!(std::ptr::eq(source.target,copied.target));
///     assert!(source.metadata.is_same_record(copied.metadata));
///     copied
/// }
/// ```
/// The carrier cannot outlive its original metadata fields or source:
/// ```compile_fail
/// use sc_core::{construction::DraftingProfileNumericSource,design::DesignMetadataHost};
/// fn escape<'a,H:DesignMetadataHost>(source:DraftingProfileNumericSource<'a,H,(),(),()>)
///     ->DraftingProfileNumericSource<'static,H,(),(),()> {source}
/// ```
pub struct DraftingProfileNumericSource<'a, H: DesignMetadataHost, S: ?Sized, P: ?Sized, M: ?Sized>
{
    /// Original selected Profile header,including actual Root,pin,inventory,body and snapshot.
    pub profile: DraftingProfileSource<'a, H, S, P>,
    /// Actual metadata field naming this numeric parameter,distinct from its canonical target.
    pub parameter: &'a ProfileParameterRef,
    /// Actual metadata field naming the exact canonical representation and scalar kind.
    pub target: &'a CanonicalInputTarget,
    /// Original complete immutable metadata record exposing both association fields.
    pub metadata: &'a DraftingSourceRecord<M>,
}
impl<H: DesignMetadataHost, S: ?Sized, P: ?Sized, M: ?Sized> Copy
    for DraftingProfileNumericSource<'_, H, S, P, M>
{
}
impl<H: DesignMetadataHost, S: ?Sized, P: ?Sized, M: ?Sized> Clone
    for DraftingProfileNumericSource<'_, H, S, P, M>
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<H: DesignMetadataHost, S: ?Sized, P: ?Sized, M: ?Sized> fmt::Debug
    for DraftingProfileNumericSource<'_, H, S, P, M>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProfileNumericSource")
            .field("target", &self.target)
            .finish_non_exhaustive()
    }
}

/// Readonly original numeric parameter metadata facet under an already-held selected Profile.
/// Responses loan actual metadata fields;the request does not manufacture an association. Every
/// held source,metadata collection,text,copy/cache and represented counter belongs in inherited
/// exhaustive accounting before lookup,including unused parameters. No allocation,clone,body
/// loading,currentness query,value read or G4 registry/schema/evidence resolution occurs here.
/// This relies on the honest immutable integration contract,like the containing context port.
/// A by-value request cannot substitute for a returned original metadata field:
/// ```compile_fail,E0515
/// use sc_core::{construction::*,design::DesignMetadataHost,ontology::ProfileParameterRef,
///     value::CanonicalInputTarget};
/// fn echo<'a,H:DesignMetadataHost>(profile:DraftingProfileSource<'a,H,(),()>,
///     parameter:ProfileParameterRef,target:&'a CanonicalInputTarget,
///     metadata:&'a DraftingSourceRecord<()>) -> DraftingProfileNumericSource<'a,H,(),(),()> {
///     DraftingProfileNumericSource {profile,parameter:&parameter,target,metadata}
/// }
/// ```
pub trait DraftingProfileNumericProvider<H: DesignMetadataHost>:
    DraftingProfileProvider<H>
{
    /// Actual immutable numeric metadata payload;no Clone/Debug/size requirement.
    type NumericMetadata: ?Sized;
    /// Resolve one explicit parameter to its original metadata fields under this actual source.
    /// The returned parameter/target belong to held metadata,independently of the request value.
    /// # Errors
    /// Original typed missing,retired,unsupported parameter or actual current-source cause.
    fn profile_numeric_source<'s>(
        &'s self,
        profile: DraftingProfileSource<'s, H, Self::Snapshot, Self::Profile>,
        parameter: ProfileParameterRef,
    ) -> Result<DraftingProfileNumericResponse<'s, H, Self>, Self::Error>;
}
/// Raw numeric response using the provider's original snapshot,Profile and numeric metadata types.
/// This alias adds no binding or lifetime extension and exposes no selected numeric value.
pub type DraftingProfileNumericResponse<'a, H, S> = DraftingProfileNumericSource<
    'a,
    H,
    <S as DraftingContextSource<H>>::Snapshot,
    <S as DraftingProfileProvider<H>>::Profile,
    <S as DraftingProfileNumericProvider<H>>::NumericMetadata,
>;
