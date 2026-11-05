//! Original selected Profile and logical/numeric metadata loans;no G4 resolution.
use super::{DraftingContextSource, DraftingSourceRecord};
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

/// Readonly original selected-Profile facet under the immutable containing context contract.
/// Every actual pin,canonical inventory,body,metadata record,copy/cache and represented counter
/// belongs in inherited exhaustive resource accounting before lookup,including unused sources.
/// Returned fields expose the actual association held by the original Profile record;this relies
/// on the honest immutable integration contract,not a security boundary against invented sources.
/// Query allocates/clones nothing,loads no new body and reads no physical/authored parameter value.
///
/// Response loans belong to the provider independently of the request pin's lifetime;the argument
/// is a selection claim,not a substitute original pin. Core compares actual Root,exact labels and
/// snapshot before private binding. G4 registry/schema/evidence/physical validation stays separate.
/// ```
/// use sc_core::{construction::{DraftingProfileProvider,DraftingProfileSource},
///     design::DesignMetadataHost,ontology::SelectedProfilePin};
/// fn original<'s,H:DesignMetadataHost,S:DraftingProfileProvider<H>>(
///     provider:&'s S,requested:&SelectedProfilePin)
///     ->Result<DraftingProfileSource<'s,H,S::Snapshot,S::Profile>,S::Error> {
///     provider.profile_source(requested)
/// }
/// ```
/// A shorter-lived request cannot replace the returned original source pin:
/// ```compile_fail,E0621
/// use sc_core::{construction::{DraftingProfileProvider,DraftingProfileSource},
///     design::DesignMetadataHost,ontology::SelectedProfilePin};
/// fn echo<'s,H:DesignMetadataHost,S:DraftingProfileProvider<H>>(
///     provider:&'s S,requested:&SelectedProfilePin)
///     ->Result<DraftingProfileSource<'s,H,S::Snapshot,S::Profile>,S::Error> {
///     let mut source=provider.profile_source(requested)?;
///     source.selected=requested;
///     Ok(source)
/// }
/// ```
pub trait DraftingProfileProvider<H: DesignMetadataHost>: DraftingContextSource<H> {
    /// Actual original immutable Profile payload type;no Clone/Debug/size requirement.
    type Profile: ?Sized;
    /// Resolve this exact explicit profile/schema/content selection to an already-held source.
    /// No default/latest/single-candidate fallback or manufactured empty success is permitted.
    /// # Errors
    /// Original typed missing,retired,unsupported selection or actual current-source cause.
    fn profile_source<'s>(
        &'s self,
        selected: &SelectedProfilePin,
    ) -> Result<DraftingProfileSource<'s, H, Self::Snapshot, Self::Profile>, Self::Error>;
}
/// Readonly actual logical-parameter metadata facet under an original selected Profile source.
/// The header identifies the already-held Profile,not an accepted parameter. Return original
/// metadata/parameter/declaration fields without echoing an ID as a fabricated record. Every held
/// metadata collection,text,copy/cache and counter belongs in inherited complete accounting before
/// lookup. No value read,cloning,allocation,body loading or G4 schema/evidence resolution occurs.
/// A by-value request cannot serve as the returned original metadata field:
/// ```compile_fail,E0515
/// use sc_core::{construction::{DraftingProfileSource,DraftingProfileLogicalSource,DraftingSourceRecord},
///     design::DesignMetadataHost,ontology::ProfileParameterRef,value::LogicalInputDeclaration};
/// fn echo<'a,H:DesignMetadataHost,S:?Sized,P:?Sized,M:?Sized>(
///     profile:DraftingProfileSource<'a,H,S,P>,parameter:ProfileParameterRef,
///     declaration:&'a LogicalInputDeclaration,metadata:&'a DraftingSourceRecord<M>)
///     ->DraftingProfileLogicalSource<'a,H,S,P,M> {
///     DraftingProfileLogicalSource {profile,parameter:&parameter,declaration,metadata}
/// }
/// ```
pub trait DraftingProfileLogicalProvider<H: DesignMetadataHost>:
    DraftingProfileProvider<H>
{
    /// Actual immutable metadata record payload;no Clone/Debug/size requirement.
    type Metadata: ?Sized;
    /// Resolve one explicit parameter to actual original metadata and canonical declaration loans.
    /// The parameter is passed by value;the returned field reference belongs to original metadata.
    /// # Errors
    /// Original typed missing,retired,unsupported parameter or actual source cause.
    fn profile_logical_source<'s>(
        &'s self,
        profile: DraftingProfileSource<'s, H, Self::Snapshot, Self::Profile>,
        parameter: ProfileParameterRef,
    ) -> Result<DraftingProfileLogicalResponse<'s, H, Self>, Self::Error>;
}
/// Original raw logical response using one provider's actual snapshot/Profile/metadata types.
/// This alias changes no ownership,lifetime,source acceptance or error semantics.
pub type DraftingProfileLogicalResponse<'a, H, S> = DraftingProfileLogicalSource<
    'a,
    H,
    <S as DraftingContextSource<H>>::Snapshot,
    <S as DraftingProfileProvider<H>>::Profile,
    <S as DraftingProfileLogicalProvider<H>>::Metadata,
>;
mod binding;
mod numeric_port;
mod parameter;
pub use binding::{
    BoundDraftingProfileSource, DraftingProfileSourceError, DraftingProfileSourceIssue,
    DraftingProfileSourceRefusal,
};
pub use numeric_port::{
    DraftingProfileNumericProvider, DraftingProfileNumericResponse, DraftingProfileNumericSource,
};
pub use parameter::{
    BoundDraftingProfileLogicalSource, DraftingProfileLogicalError, DraftingProfileLogicalIssue,
    DraftingProfileLogicalRefusal,
};
