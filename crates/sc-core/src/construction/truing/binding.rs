//! Exact actual span/copy/provider association,without current-root or geometric certification.
use super::{ConstrainedSeamTruing, TruingFitProvider};
use crate::ontology::{CutCopy, CutPlan, SeamSide, SeamSideId, SeamSpan, SewingGraph};
use core::fmt;
use sc_units::ToleranceClass;

/// First actual source or quantitative role refusal;all original providers remain inspectable.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TruingBindingRefusal {
    /// Requested span is absent from the provided graph.
    MissingSpan,
    /// Selected side's physical copy is absent from the provided plan.
    MissingCopy,
    /// Actual selected copy belongs to another pattern Piece.
    Piece,
    /// Supplied tolerance source differs from the original requested source.
    ToleranceSource,
    /// Actual tolerance is not GeometricApproximation for this length-fit role.
    ToleranceClass,
    /// Actual quantitative limit is negative;zero remains an explicit possible requirement.
    NegativeTolerance,
}
/// Actual original request,provided graph/plan/quantitative provider and first binding refusal.
pub struct TruingBindingError<'a> {
    edit: &'a ConstrainedSeamTruing,
    graph: &'a SewingGraph,
    plan: &'a CutPlan,
    provider: TruingFitProvider<'a>,
    refusal: TruingBindingRefusal,
}
impl fmt::Debug for TruingBindingError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TruingBindingError")
            .field("operation", &self.edit.operation())
            .field("span", &self.edit.span_id())
            .field("provider", &self.provider)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl<'a> TruingBindingError<'a> {
    /// Actual original request,not an equal-looking reconstructed input.
    #[must_use]
    pub const fn edit(&self) -> &'a ConstrainedSeamTruing {
        self.edit
    }
    /// Actual supplied graph,including its unchanged spans/ease/stops.
    #[must_use]
    pub const fn graph(&self) -> &'a SewingGraph {
        self.graph
    }
    /// Actual supplied physical-copy plan.
    #[must_use]
    pub const fn plan(&self) -> &'a CutPlan {
        self.plan
    }
    /// Original provider identity and actual quantitative tolerance reference.
    #[must_use]
    pub const fn fit_provider(&self) -> TruingFitProvider<'a> {
        self.provider
    }
    /// First actual binding or quantitative cause,without a fallback result.
    #[must_use]
    pub const fn refusal(&self) -> TruingBindingRefusal {
        self.refusal
    }
}
impl fmt::Display for TruingBindingError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "truing {} source binding: {:?}",
            self.edit.operation(),
            self.refusal
        )
    }
}
impl std::error::Error for TruingBindingError<'_> {}

/// Actual immutable source association and nonnegative T2 fit role;no geometry result exists here.
/// ```compile_fail
/// fn forge<'a>(edit:&'a sc_core::construction::ConstrainedSeamTruing,
///     graph:&'a sc_core::ontology::SewingGraph,plan:&'a sc_core::ontology::CutPlan,
///     span:&'a sc_core::ontology::SeamSpan,copy:&'a sc_core::ontology::CutCopy,
///     provider:sc_core::construction::TruingFitProvider<'a>)->sc_core::construction::BoundConstrainedTruing<'a> {
///     sc_core::construction::BoundConstrainedTruing { edit,graph,plan,span,copy,provider }
/// }
/// ```
/// ```compile_fail
/// fn escape(edit:sc_core::construction::ConstrainedSeamTruing,graph:sc_core::ontology::SewingGraph,
///     plan:sc_core::ontology::CutPlan,tolerance:sc_units::Tolerance)
///     ->sc_core::construction::BoundConstrainedTruing<'static> {
///     let provider=sc_core::construction::TruingFitProvider { source:edit.fit_tolerance_source(),tolerance:&tolerance };
///     edit.bind_source(&graph,&plan,provider).unwrap()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct BoundConstrainedTruing<'a> {
    edit: &'a ConstrainedSeamTruing,
    graph: &'a SewingGraph,
    plan: &'a CutPlan,
    span: &'a SeamSpan,
    copy: &'a CutCopy,
    provider: TruingFitProvider<'a>,
}
impl<'a> BoundConstrainedTruing<'a> {
    /// Actual original request with its explicit constrained policy.
    #[must_use]
    pub const fn edit(self) -> &'a ConstrainedSeamTruing {
        self.edit
    }
    /// Actual graph;membership in a current Design is not certified.
    #[must_use]
    pub const fn graph(self) -> &'a SewingGraph {
        self.graph
    }
    /// Actual provided plan;its current ownership is not certified.
    #[must_use]
    pub const fn plan(self) -> &'a CutPlan {
        self.plan
    }
    /// Actual span retaining original direction,ease allocation and stops.
    #[must_use]
    pub const fn span(self) -> &'a SeamSpan {
        self.span
    }
    /// Actual selected side field,not a copied or remapped range.
    #[must_use]
    pub fn source_side(self) -> &'a SeamSide {
        match self.edit.adjusted_side() {
            SeamSideId::A => &self.span.definition().a,
            SeamSideId::B => &self.span.definition().b,
        }
    }
    /// Actual other side;shared Piece/copy constraints may prevent it remaining geometrically fixed.
    #[must_use]
    pub fn counterpart_side(self) -> &'a SeamSide {
        match self.edit.adjusted_side() {
            SeamSideId::A => &self.span.definition().b,
            SeamSideId::B => &self.span.definition().a,
        }
    }
    /// Actual selected physical copy whose Piece association was checked.
    #[must_use]
    pub const fn selected_copy(self) -> &'a CutCopy {
        self.copy
    }
    /// Every supplied-plan copy of this Piece,in original plan order,including the counterpart
    /// when shared. This is a copy inventory,not a complete observer/dependency validation.
    pub fn affected_copies(self) -> impl Iterator<Item = &'a CutCopy> + 'a {
        self.plan.copies_for_piece(self.edit.piece_id())
    }
    /// Original identity and actual borrowed quantitative fit budget,including its derivation.
    #[must_use]
    pub const fn fit_provider(self) -> TruingFitProvider<'a> {
        self.provider
    }
}
impl ConstrainedSeamTruing {
    /// Bind actual graph span,selected copy/target Piece and explicit quantitative fit provider.
    /// Checks exact identities/class/domain only;no alias,registry evidence,current-root,curve
    /// length,fit feasibility,stop preservation,all-observer or editing authority is certified.
    /// T1 accounting remains separate;the provider's pointwise T2 provenance alone cannot prove
    /// a length-fit budget. G3 must establish that evidence and refuse infeasible constrained fits.
    /// # Errors
    /// Missing span,missing selected copy,Piece mismatch,provider source,class,then negative limit.
    pub fn bind_source<'a>(
        &'a self,
        graph: &'a SewingGraph,
        plan: &'a CutPlan,
        provider: TruingFitProvider<'a>,
    ) -> Result<BoundConstrainedTruing<'a>, TruingBindingError<'a>> {
        let refuse = |refusal| TruingBindingError {
            edit: self,
            graph,
            plan,
            provider,
            refusal,
        };
        let span = graph
            .spans()
            .iter()
            .find(|span| span.id() == self.span_id())
            .ok_or_else(|| refuse(TruingBindingRefusal::MissingSpan))?;
        let side = match self.adjusted_side() {
            SeamSideId::A => &span.definition().a,
            SeamSideId::B => &span.definition().b,
        };
        let copy = plan
            .copy(side.copy)
            .ok_or_else(|| refuse(TruingBindingRefusal::MissingCopy))?;
        if copy.piece() != self.piece_id() {
            return Err(refuse(TruingBindingRefusal::Piece));
        }
        if provider.source != self.fit_tolerance_source() {
            return Err(refuse(TruingBindingRefusal::ToleranceSource));
        }
        if provider.tolerance.class != ToleranceClass::GeometricApproximation {
            return Err(refuse(TruingBindingRefusal::ToleranceClass));
        }
        if provider.tolerance.limit.as_micrometres() < 0 {
            return Err(refuse(TruingBindingRefusal::NegativeTolerance));
        }
        Ok(BoundConstrainedTruing {
            edit: self,
            graph,
            plan,
            span,
            copy,
            provider,
        })
    }
}
