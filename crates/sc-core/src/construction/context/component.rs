//! Actual component provenance loans; raw carriers do not establish current/history acceptance.
use super::{DraftingContextSource, DraftingSourceRecord};
use crate::{
    construction::{
        DeclaredJoinPieceIdentity, DeclaredMirrorPieceIdentity, DraftingOperationInput,
        DraftingProgramInputs,
    },
    design::DesignMetadataHost,
    ontology::{EntityId, Piece},
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
