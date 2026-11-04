//! Actual component provenance loans; raw carriers do not establish current/history acceptance.
use super::{DraftingContextSource, DraftingGarmentProvider, DraftingSourceRecord};
use crate::{
    construction::{
        DeclaredJoinPieceIdentity, DeclaredMirrorPieceIdentity, DraftingOperationInput,
        DraftingProgramInputs,
    },
    design::{DesignMetadataHost, DesignReferenceHost},
    ontology::{EntityId, Piece, PocketPieceRef},
};
use core::fmt;

/// Actual native Piece identity declaration and its complete original source/role parents.
/// These are existing Mirror/Join factory results,not numeric geometry roles or invented native
/// positions. A declaration is intent; this carrier certifies no executed creation/history.
/// ```compile_fail,E0308
/// fn fake<'a>(roles:&'a sc_core::construction::BoundMirrorPieceEdgeRoles<'a>)
///     ->sc_core::construction::DraftingComponentCreator<'a> {
///     sc_core::construction::DraftingComponentCreator::Mirror(roles)
/// }
/// ```
/// ```compile_fail
/// fn escape<'a>(creator:sc_core::construction::DraftingComponentCreator<'a>)
///     ->sc_core::construction::DraftingComponentCreator<'static> {creator}
/// ```
#[derive(Clone, Copy)]
pub enum DraftingComponentCreator<'a> {
    /// Original Mirror Piece identity field and original native input/metadata/cut/source parents.
    Mirror(&'a DeclaredMirrorPieceIdentity<'a>),
    /// Original Join Piece identity field and original native input/bank/population/source parents.
    Join(&'a DeclaredJoinPieceIdentity<'a>),
}
impl<'a> DraftingComponentCreator<'a> {
    /// Borrow the actual declared Piece identity field; no identity is issued or made live.
    #[must_use]
    pub const fn piece_id(self) -> &'a EntityId {
        match self {
            Self::Mirror(p) => p.piece_id(),
            Self::Join(p) => p.piece_id(),
        }
    }
    /// Borrow the actual creating native input through its authentic published family parents.
    /// No input clone,ID reconstruction,source lookup or program-position inference occurs.
    #[must_use]
    pub fn input(self) -> DraftingOperationInput<'a> {
        match self {
            Self::Mirror(p) => {
                DraftingOperationInput::MirrorPiece(p.roles().metadata().cut().source().input())
            }
            Self::Join(p) => DraftingOperationInput::JoinPieces(
                p.roles()
                    .banks()
                    .sources()
                    .population()
                    .metadata()
                    .splice()
                    .source()
                    .edit(),
            ),
        }
    }
}
impl fmt::Debug for DraftingComponentCreator<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Mirror(_) => "DraftingComponentCreator::Mirror(..)",
            Self::Join(_) => "DraftingComponentCreator::Join(..)",
        })
    }
}
/// Raw explicit imported primitive and its actual original source record. The payload is supplied
/// by the integration; no importer,format,history,geometry or Root membership is inferred here.
/// Providers must retain the real original primitive source,never an empty replacement or made-up
/// recipe. Its payload and any duplicated records/caches must enter their exhaustive resource walk.
/// ```compile_fail
/// fn escape<'a,T:?Sized>(source:sc_core::construction::DraftingImportedComponentSource<'a,T>)
///     ->sc_core::construction::DraftingImportedComponentSource<'static,T> {source}
/// ```
pub struct DraftingImportedComponentSource<'a, I: ?Sized> {
    /// Actual primitive Piece,not a copied definition or an ID-only placeholder.
    pub piece: &'a Piece,
    /// Actual original imported-source record,not a reconstructed source ID or fake native step.
    pub source: &'a DraftingSourceRecord<I>,
}
impl<I: ?Sized> Copy for DraftingImportedComponentSource<'_, I> {}
impl<I: ?Sized> Clone for DraftingImportedComponentSource<'_, I> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<I: ?Sized> fmt::Debug for DraftingImportedComponentSource<'_, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DraftingImportedComponentSource(..)")
    }
}
/// Raw original recipe/program owner,actual Piece and genuine native Piece identity declaration.
/// Public construction accepts foreign/mismatched data for later typed binding; matching IDs grant
/// no history,currentness,creator membership or accepted position. No ordinal is fabricated.
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn escape<'p,'a,H:DesignMetadataHost,S:DraftingContextSource<H>>(
///     source:DraftingComponentRecipeSource<'p,'a,H,S>)
///     ->DraftingComponentRecipeSource<'static,'a,H,S> {source}
/// ```
pub struct DraftingComponentRecipeSource<
    'p,
    'a,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
> {
    /// Actual original containing mixed recipe owner; its scalar recipe and native sequence stay whole.
    pub program: &'p DraftingProgramInputs<'a, H, S>,
    /// Actual supplied Piece whose original creator association still requires binding.
    pub piece: &'p Piece,
    /// Actual native Piece identity declaration and all original parent loans.
    pub creator: DraftingComponentCreator<'p>,
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> Copy
    for DraftingComponentRecipeSource<'_, '_, H, S>
{
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> Clone
    for DraftingComponentRecipeSource<'_, '_, H, S>
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for DraftingComponentRecipeSource<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingComponentRecipeSource")
            .field("creator", &self.creator)
            .finish_non_exhaustive()
    }
}
/// Explicit raw original provenance,without deriving a recipe from geometry or inventing import history.
pub enum DraftingComponentOrigin<
    'p,
    'a,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
    I: ?Sized,
> {
    /// Explicit original primitive/source loans,without a fake recipe or creating position.
    Imported(DraftingImportedComponentSource<'p, I>),
    /// Actual original recipe and native Piece-identity parent loans,without accepted membership.
    Recipe(DraftingComponentRecipeSource<'p, 'a, H, S>),
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized, I: ?Sized> Copy
    for DraftingComponentOrigin<'_, '_, H, S, I>
{
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized, I: ?Sized> Clone
    for DraftingComponentOrigin<'_, '_, H, S, I>
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized, I: ?Sized> fmt::Debug
    for DraftingComponentOrigin<'_, '_, H, S, I>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Imported(p) => p.fmt(f),
            Self::Recipe(p) => p.fmt(f),
        }
    }
}
impl<'p, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized, I: ?Sized>
    DraftingComponentOrigin<'p, '_, H, S, I>
{
    /// Actual original Piece loan retained by the selected source,without lookup or cloning.
    #[must_use]
    pub const fn piece(self) -> &'p Piece {
        match self {
            Self::Imported(p) => p.piece,
            Self::Recipe(p) => p.piece,
        }
    }
}
/// Raw component origin with original frame/snapshot records. All fields await actual Root/Piece,
/// recipe/native creator membership,frame and snapshot checks under complete content/work limits.
/// This defines data only; no provider query,allocation,source walk or accepted prefix occurs.
/// Additional original recipe/manifest/source payloads belong in the source provider's exhaustive
/// fresh walk; earlier native constructor allowances give no whole-source accounting credit.
pub struct DraftingComponentSource<
    'p,
    'a,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
    I: ?Sized,
> {
    /// Actual original recipe or explicitly imported primitive source.
    pub origin: DraftingComponentOrigin<'p, 'a, H, S, I>,
    /// Actual authored Cartesian frame record,not an inferred transform.
    pub frame: &'p DraftingSourceRecord<S::Frame>,
    /// Actual immutable containing snapshot record,not an ID/version string substitute.
    pub snapshot: &'p DraftingSourceRecord<S::Snapshot>,
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized, I: ?Sized> Copy
    for DraftingComponentSource<'_, '_, H, S, I>
{
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized, I: ?Sized> Clone
    for DraftingComponentSource<'_, '_, H, S, I>
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized, I: ?Sized> fmt::Debug
    for DraftingComponentSource<'_, '_, H, S, I>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingComponentSource")
            .field("origin", &self.origin)
            .finish_non_exhaustive()
    }
}

/// Readonly original component source facet over actual garment/context data. All original import
/// payloads,program records/native fields/creator manifests,retained contexts,unused records,
/// copies/caches and represented counters participate in inherited complete fresh accounting before
/// query. A source-held recipe uses record-only composition without recursively reentering this
/// provider;its actual context bodies are accounted through their real original owners.
///
/// Return the actual already-held source association;matching IDs or declarations do not invent
/// executed creation/history. Query loads,allocates,clones and evaluates nothing. Missing,retired
/// or unsupported sources retain original typed causes,never an empty import or reconstructed
/// recipe. Raw responses still need fresh core bounds,actual Piece/copy,source owner,frame/snapshot
/// and original native membership checks. This is the honest immutable integration contract,
/// not a security boundary against invented metadata or omitted resources.
///
/// Output loans belong to original source records independently of a by-value request:
/// ```
/// use sc_core::{construction::*,design::DesignReferenceHost,ontology::PocketPieceRef};
/// fn original<'a,H:DesignReferenceHost,S:DraftingComponentProvider<H>+?Sized>(provider:&'a S,
///     requested:PocketPieceRef)->Result<DraftingComponentResponse<'a,H,S>,S::Error> {
///     provider.component_source(requested)
/// }
/// ```
/// A request cannot become an invented borrowed import record:
/// ```compile_fail,E0515
/// use sc_core::{construction::*,design::DesignReferenceHost,ontology::{Piece,PocketPieceRef}};
/// fn echo<'a,H:DesignReferenceHost,S:DraftingComponentProvider<H,Imported=PocketPieceRef>+?Sized>(
///     piece:&'a Piece,frame:&'a DraftingSourceRecord<S::Frame>,snapshot:&'a DraftingSourceRecord<S::Snapshot>,
///     requested:PocketPieceRef)->DraftingComponentResponse<'a,H,S> {
///     let invented=DraftingSourceRecord::new(requested);
///     DraftingComponentSource {origin:DraftingComponentOrigin::Imported(DraftingImportedComponentSource {
///         piece,source:&invented}),frame,snapshot}
/// }
/// ```
pub trait DraftingComponentProvider<H: DesignReferenceHost>: DraftingGarmentProvider<H> {
    /// Actual opaque original imported-source payload;no Clone/Debug/size requirement.
    type Imported: ?Sized;
    /// Resolve exactly this explicit copy/expected-Piece request to original held source fields.
    /// Return original program/creator loans for recipe origin,never a reconstructed operation or
    /// numeric creation position. The response itself grants no accepted source or query budget.
    /// # Errors
    /// Original typed missing,reassigned,retired,ambiguous or unsupported component/source cause.
    fn component_source<'a>(
        &'a self,
        requested: PocketPieceRef,
    ) -> Result<DraftingComponentResponse<'a, H, Self>, Self::Error>;
}
/// Raw original response using one provider's actual context/frame/snapshot/imported payload types.
/// Original program context loans may be shortened to this source borrow without cloning data.
/// This alias adds no resource,ownership,currentness,creation/history or execution acceptance.
/// ```
/// use sc_core::{construction::*,design::DesignReferenceHost};
/// fn original_context<'p,'a,H:DesignReferenceHost,S:DraftingComponentProvider<H>+?Sized>(
///     source:DraftingComponentSource<'p,'a,H,S,S::Imported>)->DraftingComponentResponse<'p,H,S>
///     where 'a:'p {source}
/// ```
pub type DraftingComponentResponse<'a, H, S> =
    DraftingComponentSource<'a, 'a, H, S, <S as DraftingComponentProvider<H>>::Imported>;
