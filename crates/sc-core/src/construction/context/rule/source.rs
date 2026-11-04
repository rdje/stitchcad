//! Fresh original rule body/origin association, without local signature or result acceptance.
use super::{
    AdmittedDraftingRuleWork, DraftingRuleBindingShape, DraftingRuleProvider, DraftingRuleResponse,
    DraftingRuleSelection,
};
use crate::{
    construction::DraftingProfileSource,
    design::{
        fixed_value, DesignMetadataHost, DesignResourceDemand, DesignResourceLimitError,
        DesignResourceSink, DesignResourceSource,
    },
};
use core::fmt;
mod check;
mod error;
pub use error::{DraftingRuleSourceError, DraftingRuleSourceIssue, DraftingRuleSourceRefusal};
type Profile<'a, H, S> = DraftingProfileSource<
    'a,
    H,
    <S as crate::construction::DraftingContextSource<H>>::Snapshot,
    <S as crate::construction::DraftingProfileProvider<H>>::Profile,
>;
type Admission = (DesignResourceDemand, DraftingRuleBindingShape);

/// Original rule/policy declaration/body under the actual operation's exact source origin,ID and
/// family. It loans its original work parent and preserves fresh admission metadata and original
/// expected/returned source fields. Body/signature conformance is the honest integration contract.
/// This proves no local input/result signature,canonical input/output,value,evidence,G4 truth,
/// physical policy execution or whole-program acceptance.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn alter<H:DesignMetadataHost,S:DraftingRuleProvider<H>>(source:&mut BoundDraftingRuleSource<'_,'_,H,S>) {
///     source.source=todo!();
/// }
/// ```
/// A source proof cannot outlive its original work parent:
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignResourceSource};
/// fn escape<'w,'a,H:DesignResourceSource,S:DraftingRuleProvider<H>>(work:&'w AdmittedDraftingRuleWork<'a,H,S>)
///     ->Result<BoundDraftingRuleSource<'static,'a,H,S>,String> {
///     work.bind_source().map_err(|e|e.to_string())
/// }
/// ```
pub struct BoundDraftingRuleSource<
    'w,
    'a,
    H: DesignMetadataHost,
    S: DraftingRuleProvider<H> + ?Sized,
> {
    work: &'w AdmittedDraftingRuleWork<'a, H, S>,
    profile: Option<Profile<'a, H, S>>,
    source: DraftingRuleResponse<'a, H, S>,
    demand: DesignResourceDemand,
    shape: DraftingRuleBindingShape,
}
impl<'w, 'a, H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized>
    BoundDraftingRuleSource<'w, 'a, H, S>
{
    /// Exact original complete work parent,including actual operation/member/context and intent.
    #[must_use]
    pub const fn work(&self) -> &'w AdmittedDraftingRuleWork<'a, H, S> {
        self.work
    }
    /// Original returned declaration/body/origin with matched Root/snapshot/source owners/ID/family.
    /// Local signature and body conformance are separate;this raw view cannot forge a private proof.
    #[must_use]
    pub const fn source(&self) -> DraftingRuleResponse<'a, H, S> {
        self.source
    }
    /// Actual selected Profile header accepted by the existing Profile binder,when requested.
    /// Original pin/inventory/Profile/snapshot loans are retained without a copied body or domain.
    #[must_use]
    pub const fn profile_source(
        &self,
    ) -> Option<DraftingProfileSource<'a, H, S::Snapshot, S::Profile>> {
        self.profile
    }
    /// Fresh complete content demand from this source bind's repeated work admission.
    #[must_use]
    pub const fn demand(&self) -> DesignResourceDemand {
        self.demand
    }
    /// Fresh prospective matching bound from this source bind,not cached parent credit.
    #[must_use]
    pub const fn shape(&self) -> DraftingRuleBindingShape {
        self.shape
    }
}
impl<H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized> fmt::Debug
    for BoundDraftingRuleSource<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundDraftingRuleSource")
            .field("rule", &self.source.declaration.id())
            .finish_non_exhaustive()
    }
}
impl<H: DesignResourceSource, S: DraftingRuleProvider<H> + ?Sized>
    BoundDraftingRuleSource<'_, '_, H, S>
{
    /// Freshly append every original parent/Root/provider/request field;source/Profile headers are
    /// borrowed views of those accounted records. Stored demand/shape supplies no content credit.
    /// No Profile/rule/currentness/member/value query or local signature scan occurs.
    /// # Errors
    /// Original first cumulative resource cause,including earlier or ignored sink failures.
    pub fn charge_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        let Self {
            work,
            profile,
            source,
            demand,
            shape,
        } = self;
        fixed_value(profile);
        fixed_value(source);
        fixed_value(demand);
        fixed_value(shape);
        work.charge_resources(sink)
    }
}
impl<'a, H: DesignResourceSource, S: DraftingRuleProvider<H> + ?Sized>
    AdmittedDraftingRuleWork<'a, H, S>
{
    /// Repeat complete fresh content/W/member admission,then bind explicit Profile selection when
    /// requested using the existing fresh Profile binder. Resolve one exact rule source and check
    /// original Root,snapshot,origin,Profile owners,rule ID and family in that order. No cached work
    /// credit,body clone,source default,local signature/result or canonical value is accepted.
    /// The Profile binder performs its own fresh content walk before its readonly lookup;both
    /// queries occur after the complete prospective matching preflight under the immutable contract.
    /// # Errors
    /// Original fresh work/Profile/provider cause or first actual association mismatch with all loans.
    pub fn bind_source(
        &self,
    ) -> Result<BoundDraftingRuleSource<'_, 'a, H, S>, DraftingRuleSourceError<'_, 'a, H, S>> {
        let fresh = self
            .context()
            .prepare_rule_binding(
                self.operation().operation().id(),
                self.operation().operation().kind(),
                self.selection(),
                self.limits(),
            )
            .map_err(|cause| {
                DraftingRuleSourceError::new(
                    self,
                    None,
                    None,
                    None,
                    DraftingRuleSourceRefusal::Work(cause),
                )
            })?;
        let admission = Some((fresh.demand(), fresh.shape()));
        let profile = match fresh.selection() {
            DraftingRuleSelection::Design => None,
            DraftingRuleSelection::Profile(selected) => match fresh
                .context()
                .bind_profile_source(selected, fresh.limits().content())
            {
                Ok(bound) => Some(bound.source()),
                Err(cause) => {
                    return Err(DraftingRuleSourceError::new(
                        self,
                        admission,
                        None,
                        None,
                        DraftingRuleSourceRefusal::Profile(cause),
                    ))
                }
            },
        };
        let source = match fresh.context().source().rule_source(fresh.request()) {
            Ok(source) => source,
            Err(cause) => {
                return Err(DraftingRuleSourceError::new(
                    self,
                    admission,
                    profile,
                    None,
                    DraftingRuleSourceRefusal::Source(cause),
                ))
            }
        };
        if let Some(issue) = check::issue(&fresh, profile, source) {
            return Err(DraftingRuleSourceError::new(
                self,
                admission,
                profile,
                Some(source),
                DraftingRuleSourceRefusal::Mismatch(issue),
            ));
        }
        Ok(BoundDraftingRuleSource {
            work: self,
            profile,
            source,
            demand: fresh.demand(),
            shape: fresh.shape(),
        })
    }
}
