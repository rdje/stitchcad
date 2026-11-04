//! Raw original selected Profile and logical-parameter source loans;no binding or G4 resolution.
use super::DraftingSourceRecord;
use crate::{
    design::{DesignInputs, DesignMetadataHost},
    ontology::{ProfileParameterRef, SelectedProfilePin},
    value::{CanonicalInputs, LogicalInputDeclaration},
};
use core::fmt;

/// Original raw selected Profile source under its actual containing Design/snapshot.
/// All fields loan original immutable source data. The canonical inventory belongs to this
/// Profile source;it is not implicitly the Design's inventory. The pin retains distinct actual
/// profile/schema/content labels. No default/latest profile or replacement body is supplied.
///
/// Public construction admits inspectable foreign/mismatched records for later typed refusal.
/// It proves no selected-pin association,canonical ownership,currentness,Profile schema/evidence,
/// resource allowance,semantic dependency,value or physical result. The provider's complete fresh
/// walk must include every actual pin,canonical declaration,body,metadata copy and cache before
/// lookup. An opaque payload still needs its actual exhaustive integration-owned producer.
/// ```compile_fail
/// use sc_core::{construction::DraftingProfileSource,design::DesignMetadataHost};
/// fn escape<'a,H:DesignMetadataHost>(source:DraftingProfileSource<'a,H,(),()>)
///     ->DraftingProfileSource<'static,H,(),()> {source}
/// ```
pub struct DraftingProfileSource<'a, H: DesignMetadataHost, S: ?Sized, P: ?Sized> {
    /// Actual containing Design owner,without copying or attaching a factory pin to the model.
    pub root: &'a DesignInputs<H>,
    /// Actual selected profile/schema/content pin in this original source.
    pub selected: &'a SelectedProfilePin,
    /// Actual canonical declarations owned by this Profile source,including unused entries.
    pub inputs: &'a CanonicalInputs,
    /// Original complete immutable Profile payload record,not a fabricated empty body.
    pub profile: &'a DraftingSourceRecord<P>,
    /// Original containing context snapshot record,not an id or caller liveness flag.
    pub snapshot: &'a DraftingSourceRecord<S>,
}
impl<H: DesignMetadataHost, S: ?Sized, P: ?Sized> Copy for DraftingProfileSource<'_, H, S, P> {}
impl<H: DesignMetadataHost, S: ?Sized, P: ?Sized> Clone for DraftingProfileSource<'_, H, S, P> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<H: DesignMetadataHost, S: ?Sized, P: ?Sized> fmt::Debug
    for DraftingProfileSource<'_, H, S, P>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProfileSource")
            .finish_non_exhaustive()
    }
}

/// Original raw logical parameter metadata and actual canonical declaration under a Profile source.
/// The parameter ref and declaration must be loans exposed by that original metadata record;
/// echoing a request or returning an equal-data substitute does not establish an authentic source.
/// Core later checks original Profile/Root/pin/snapshot and canonical membership/kind separately.
///
/// This carrier reads no selected value and clones no domain/state/provenance or body. It may
/// represent a missing association or wrong member for inspection;it is no accepted source proof.
/// The metadata payload,including duplicated records/caches,belongs in the complete provider walk.
/// Profile parameter IDs and canonical declaration IDs keep their distinct original roles.
/// ```
/// use sc_core::{construction::{DraftingProfileSource,DraftingProfileLogicalSource,DraftingSourceRecord},
///     design::DesignMetadataHost,ontology::ProfileParameterRef,value::LogicalInputDeclaration};
/// fn raw_parameter<'a,H:DesignMetadataHost,S:?Sized,P:?Sized,M:?Sized>(
///     profile:DraftingProfileSource<'a,H,S,P>,parameter:&'a ProfileParameterRef,
///     declaration:&'a LogicalInputDeclaration,metadata:&'a DraftingSourceRecord<M>)
///     ->DraftingProfileLogicalSource<'a,H,S,P,M> {
///     DraftingProfileLogicalSource {profile,parameter,declaration,metadata}
/// }
/// ```
pub struct DraftingProfileLogicalSource<'a, H: DesignMetadataHost, S: ?Sized, P: ?Sized, M: ?Sized>
{
    /// Original Profile source,including actual selected pin,inventory,Root and snapshot loans.
    pub profile: DraftingProfileSource<'a, H, S, P>,
    /// Actual metadata field naming this logical Profile parameter;not an invented row ordinal.
    pub parameter: &'a ProfileParameterRef,
    /// Actual source's declared canonical logical record;membership/kind still need binding.
    pub declaration: &'a LogicalInputDeclaration,
    /// Original metadata record exposing this parameter-to-declaration association.
    pub metadata: &'a DraftingSourceRecord<M>,
}
impl<H: DesignMetadataHost, S: ?Sized, P: ?Sized, M: ?Sized> Copy
    for DraftingProfileLogicalSource<'_, H, S, P, M>
{
}
impl<H: DesignMetadataHost, S: ?Sized, P: ?Sized, M: ?Sized> Clone
    for DraftingProfileLogicalSource<'_, H, S, P, M>
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<H: DesignMetadataHost, S: ?Sized, P: ?Sized, M: ?Sized> fmt::Debug
    for DraftingProfileLogicalSource<'_, H, S, P, M>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProfileLogicalSource")
            .field("kind", &self.declaration.kind())
            .finish_non_exhaustive()
    }
}
