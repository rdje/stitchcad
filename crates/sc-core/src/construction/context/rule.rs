//! Raw original rule declarations/bodies and explicit Design/Profile origins, without binding.
use super::{
    DraftingContextSource, DraftingProfileProvider, DraftingProfileSource, DraftingSourceRecord,
};
use crate::{
    design::{DesignInputs, DesignMetadataHost},
    ontology::{EntityId, SelectedProfilePin},
    recipe::{SemanticOperationKind, SemanticRuleDeclaration},
};
use core::fmt;

/// Exact requested rule origin; a Profile pin is an intent,not a replacement source record.
#[derive(Clone, Copy)]
pub enum DraftingRuleSelection<'a> {
    /// Original rule in the actual containing Design.
    Design,
    /// Original rule in this explicit profile/schema/content selection; no latest/default lookup.
    Profile(&'a SelectedProfilePin),
}
impl fmt::Debug for DraftingRuleSelection<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Design => "Design",
            Self::Profile(_) => "Profile(..)",
        })
    }
}
/// Exact original rule/policy identity,family and explicit containing-source selection.
#[derive(Clone, Copy, Debug)]
pub struct DraftingRuleRequest<'a> {
    /// Requested actual rule/policy ID,not an operation or canonical input ID.
    pub rule: EntityId,
    /// Exact expected declaration family; equality of numeric IDs does not establish it.
    pub kind: SemanticOperationKind,
    /// Explicit Design/Profile selection,without inferred factory identity or content revision.
    pub origin: DraftingRuleSelection<'a>,
}
/// Raw original containing source. Every variant exposes actual Root and snapshot loans.
/// Profile origins also preserve actual selected pin,canonical inventory and Profile body loans.
/// Public construction can represent foreign/mismatched sources for later typed refusal.
pub enum DraftingRuleOrigin<'a, H: DesignMetadataHost, S: ?Sized, P: ?Sized> {
    /// Actual Design owner and containing immutable snapshot; no copied owner or ID certificate.
    Design {
        /// Original containing Design.
        root: &'a DesignInputs<H>,
        /// Original containing snapshot record.
        snapshot: &'a DraftingSourceRecord<S>,
    },
    /// Actual selected Profile source,without duplicating its declarations,pin or payload.
    Profile(DraftingProfileSource<'a, H, S, P>),
}
impl<H: DesignMetadataHost, S: ?Sized, P: ?Sized> Copy for DraftingRuleOrigin<'_, H, S, P> {}
impl<H: DesignMetadataHost, S: ?Sized, P: ?Sized> Clone for DraftingRuleOrigin<'_, H, S, P> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<H: DesignMetadataHost, S: ?Sized, P: ?Sized> fmt::Debug for DraftingRuleOrigin<'_, H, S, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Design { .. } => "DraftingRuleOrigin::Design(..)",
            Self::Profile(_) => "DraftingRuleOrigin::Profile(..)",
        })
    }
}
impl<'a, H: DesignMetadataHost, S: ?Sized, P: ?Sized> DraftingRuleOrigin<'a, H, S, P> {
    /// Actual original containing Root,without checking it against a bound context.
    #[must_use]
    pub const fn root(&self) -> &'a DesignInputs<H> {
        match self {
            Self::Design { root, .. } => root,
            Self::Profile(source) => source.root,
        }
    }
    /// Actual original snapshot record,not a supplied revision/currentness flag.
    #[must_use]
    pub const fn snapshot(&self) -> &'a DraftingSourceRecord<S> {
        match self {
            Self::Design { snapshot, .. } => snapshot,
            Self::Profile(source) => source.snapshot,
        }
    }
}
/// Raw original declaration/body association under its actual containing source. This carrier
/// copies only loans;no Clone/Debug/size bound is imposed on host,snapshot,Profile or body payload.
/// The declaration must be the actual original metadata exposed by this body's source,not a
/// reconstructed signature or copied operation. The honest immutable provider contract owns that
/// association;this raw view grants no source truth,currentness,resource admission or execution.
/// Every held declaration,body,unused record,copy/cache and counter belongs in the provider walk.
/// ```compile_fail
/// use sc_core::{construction::DraftingRuleSource,design::DesignMetadataHost};
/// fn escape<'a,H:DesignMetadataHost,S:?Sized,P:?Sized,B:?Sized>(source:DraftingRuleSource<'a,H,S,P,B>)
///     ->DraftingRuleSource<'static,H,S,P,B> {source}
/// ```
/// Original declaration loans prevent replacement while the source remains in use:
/// ```compile_fail,E0506
/// use sc_core::{construction::*,design::DesignMetadataHost,recipe::SemanticRuleDeclaration};
/// fn replace<'a,H:DesignMetadataHost,S:?Sized,P:?Sized,B:?Sized>(declaration:&'a mut SemanticRuleDeclaration,
///     replacement:SemanticRuleDeclaration,origin:DraftingRuleOrigin<'a,H,S,P>,body:&'a DraftingSourceRecord<B>) {
///     let source=DraftingRuleSource {declaration:&*declaration,origin,body};
///     *declaration=replacement;
///     let _=source.declaration.id();
/// }
/// ```
pub struct DraftingRuleSource<'a, H: DesignMetadataHost, S: ?Sized, P: ?Sized, B: ?Sized> {
    /// Actual original immutable declaration/signature metadata belonging to this source.
    pub declaration: &'a SemanticRuleDeclaration,
    /// Actual original opaque body record,not an empty placeholder or invented evaluator.
    pub body: &'a DraftingSourceRecord<B>,
    /// Explicit actual containing Design or selected Profile source.
    pub origin: DraftingRuleOrigin<'a, H, S, P>,
}
impl<H: DesignMetadataHost, S: ?Sized, P: ?Sized, B: ?Sized> Copy
    for DraftingRuleSource<'_, H, S, P, B>
{
}
impl<H: DesignMetadataHost, S: ?Sized, P: ?Sized, B: ?Sized> Clone
    for DraftingRuleSource<'_, H, S, P, B>
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<H: DesignMetadataHost, S: ?Sized, P: ?Sized, B: ?Sized> fmt::Debug
    for DraftingRuleSource<'_, H, S, P, B>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingRuleSource")
            .field("rule", &self.declaration.id())
            .field("kind", &self.declaration.kind())
            .field("origin", &self.origin)
            .finish_non_exhaustive()
    }
}
/// Readonly original rule/policy facet over the actual immutable context and Profile source ports.
/// Design-only integrations may refuse unsupported Profile selections through their original
/// typed cause;they must never manufacture a Profile/body to satisfy this interface.
///
/// Inherited complete accounting covers all actual declarations (using the signature producer),
/// bodies,origins,pins,canonical inventories,unused records,copies/caches and represented counters
/// before lookup. Precharge collections before traversal;opaque bodies still require their actual
/// exhaustive integration-owned producer. Query loads/clones/allocates nothing and evaluates no
/// body or value. It supplies no signature/body conformance,G4 truth or physical certificate.
/// Returned fields belong to original source records independently of the request pin's lifetime:
/// ```
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn original<'s,H:DesignMetadataHost,S:DraftingRuleProvider<H>+?Sized>(provider:&'s S,
///     request:DraftingRuleRequest<'_>)->Result<DraftingRuleResponse<'s,H,S>,S::Error> {
///     provider.rule_source(request)
/// }
/// ```
/// A request pin cannot replace an original source pin:
/// ```compile_fail,E0621
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn echo<'s,H:DesignMetadataHost,S:DraftingRuleProvider<H>+?Sized>(provider:&'s S,
///     request:DraftingRuleRequest<'_>)->Result<DraftingRuleResponse<'s,H,S>,S::Error> {
///     let mut source=provider.rule_source(request)?;
///     if let DraftingRuleSelection::Profile(selected)=request.origin {
///         if let DraftingRuleOrigin::Profile(ref mut profile)=source.origin {profile.selected=selected;}
///     }
///     Ok(source)
/// }
/// ```
pub trait DraftingRuleProvider<H: DesignMetadataHost>: DraftingProfileProvider<H> {
    /// Actual immutable original body payload,without Clone/Debug/size requirements.
    type RuleBody: ?Sized;
    /// Resolve only this explicit rule/family/origin to actual already-held original source fields.
    /// Raw responses remain inspectable before core checks their association;no default rule/body
    /// or accepted prefix is returned for missing,ambiguous,retired or unsupported sources.
    /// # Errors
    /// Original typed lookup/current-source refusal,retaining its exact request and cause chain.
    fn rule_source<'s>(
        &'s self,
        request: DraftingRuleRequest<'_>,
    ) -> Result<DraftingRuleResponse<'s, H, Self>, Self::Error>;
}
/// Original raw response using one provider's actual snapshot,Profile and rule-body payload types.
/// This transparent alias changes no ownership,lifetime,error or acceptance semantics.
pub type DraftingRuleResponse<'a, H, S> = DraftingRuleSource<
    'a,
    H,
    <S as DraftingContextSource<H>>::Snapshot,
    <S as DraftingProfileProvider<H>>::Profile,
    <S as DraftingRuleProvider<H>>::RuleBody,
>;
mod work;
pub use work::{
    AdmittedDraftingRuleWork, DraftingRuleBindingLimits, DraftingRuleBindingShape,
    DraftingRuleWorkError, DraftingRuleWorkRefusal,
};
