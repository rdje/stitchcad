//! Complete current garment/source work before actual copy/Piece membership,without source lookup.
use super::DraftingComponentProvider;
use crate::{
    construction::AdmittedDraftingGarmentSource,
    design::{
        fixed_value, DesignReferenceHost, DesignResourceDemand, DesignResourceLimitError,
        DesignResourceMeter, DesignResourceSink, DesignResourceSource,
    },
    ontology::{CutCopy, Piece, PocketPieceRef},
};
use core::fmt;
mod error;
mod limits;
pub use error::{DraftingComponentWorkError, DraftingComponentWorkRefusal};
pub use limits::{DraftingComponentBindingLimits, DraftingComponentBindingShape};

/// Actual current CutPlan copy and expected Piece after fresh whole garment/provider content and
/// prospective matching bounds. No original source,creator,frame/history,geometry or execution is
/// accepted. The copy's original orientation remains metadata;no transform or identity is produced.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignReferenceHost};
/// fn alter<H:DesignReferenceHost,S:DraftingComponentProvider<H>>(work:&mut AdmittedDraftingComponentWork<'_,'_,H,S>) {
///     work.copy=todo!();
/// }
/// ```
/// The work scope cannot outlive its actual admitted garment parent:
/// ```compile_fail
/// use sc_core::{construction::*,design::{DesignReferenceHost,DesignResourceSource},ontology::PocketPieceRef};
/// fn escape<'g,'a,H:DesignReferenceHost+DesignResourceSource,S:DraftingComponentProvider<H>>(
///     parent:&'g AdmittedDraftingGarmentSource<'a,H,S>,target:PocketPieceRef,limits:DraftingComponentBindingLimits)
///     ->Result<AdmittedDraftingComponentWork<'static,'a,H,S>,String> {
///     parent.prepare_component_binding(target,limits).map_err(|e|e.to_string())
/// }
/// ```
pub struct AdmittedDraftingComponentWork<
    'g,
    'a,
    H: DesignReferenceHost,
    S: DraftingComponentProvider<H> + ?Sized,
> {
    parent: &'g AdmittedDraftingGarmentSource<'a, H, S>,
    target: PocketPieceRef,
    copy: &'a CutCopy,
    piece: &'a Piece,
    limits: DraftingComponentBindingLimits,
    demand: DesignResourceDemand,
    shape: DraftingComponentBindingShape,
}
impl<'g, 'a, H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized>
    AdmittedDraftingComponentWork<'g, 'a, H, S>
{
    /// Exact original admitted current garment/context/source parent.
    #[must_use]
    pub const fn parent(&self) -> &'g AdmittedDraftingGarmentSource<'a, H, S> {
        self.parent
    }
    /// Exact original explicit copy/expected-Piece request.
    #[must_use]
    pub const fn target(&self) -> PocketPieceRef {
        self.target
    }
    /// Actual original current CutPlan member,including its unchanged orientation.
    #[must_use]
    pub const fn copy(&self) -> &'a CutCopy {
        self.copy
    }
    /// Actual original current Piece inventory member named by that copy and request.
    #[must_use]
    pub const fn piece(&self) -> &'a Piece {
        self.piece
    }
    /// Exact selected fresh complete content/work allowance.
    #[must_use]
    pub const fn limits(&self) -> DraftingComponentBindingLimits {
        self.limits
    }
    /// Complete freshly walked garment/provider demand,without parent admission credit.
    #[must_use]
    pub const fn demand(&self) -> DesignResourceDemand {
        self.demand
    }
    /// Complete held collection population and conservative original matching bound.
    #[must_use]
    pub const fn shape(&self) -> DraftingComponentBindingShape {
        self.shape
    }
}
impl<H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized> fmt::Debug
    for AdmittedDraftingComponentWork<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AdmittedDraftingComponentWork")
            .field("target", &self.target)
            .field("shape", &self.shape)
            .finish_non_exhaustive()
    }
}
impl<H: DesignReferenceHost + DesignResourceSource, S: DraftingComponentProvider<H> + ?Sized>
    AdmittedDraftingComponentWork<'_, '_, H, S>
{
    /// Freshly rewalk all original parent garment/provider fields;target/copy/Piece are fixed or
    /// borrowed views and stored demand/shape grants no credit. No membership/source/value query.
    /// # Errors
    /// Original first cumulative content cause,including earlier or ignored provider failure.
    pub fn charge_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        let Self {
            parent,
            target,
            copy,
            piece,
            limits,
            demand,
            shape,
        } = self;
        fixed_value(target);
        fixed_value(copy);
        fixed_value(piece);
        fixed_value(limits);
        fixed_value(demand);
        fixed_value(shape);
        parent.charge_resources(sink)
    }
}
impl<
        'a,
        H: DesignReferenceHost + DesignResourceSource,
        S: DraftingComponentProvider<H> + ?Sized,
    > AdmittedDraftingGarmentSource<'a, H, S>
{
    /// Freshly charge all actual garment/provider fields,derive checked (E+8)² and refuse work
    /// shortage before current copy/expected-Piece scans. E includes all held original program,
    /// import,unused record/copy/cache entries under the honest provider contract. The conservative
    /// bound covers linear current/member/creator and fixed header inspections,not string bytes,
    /// provider-internal CPU,allocator capacity,geometry or execution. No component/currentness/
    /// value query or result vector occurs;only a refused diagnostic is boxed.
    /// # Errors
    /// First complete content,arithmetic/work,missing copy,reassigned copy or missing Piece cause.
    pub fn prepare_component_binding(
        &self,
        target: PocketPieceRef,
        limits: DraftingComponentBindingLimits,
    ) -> Result<AdmittedDraftingComponentWork<'_, 'a, H, S>, DraftingComponentWorkError<'_, 'a, H, S>>
    {
        let refuse = |refusal| DraftingComponentWorkError::new(self, target, limits, refusal);
        let mut meter = DesignResourceMeter::new(limits.content());
        let charged = self.charge_resources(&mut meter);
        let demand = meter
            .finish()
            .map_err(|cause| refuse(DraftingComponentWorkRefusal::Resources(cause)))?;
        charged.map_err(|cause| refuse(DraftingComponentWorkRefusal::Resources(cause)))?;
        let shape = DraftingComponentBindingShape::admit(demand.collection_entries(), limits)
            .map_err(refuse)?;
        let root = self.source().garment.inputs().structure();
        let copy = root
            .cut_plan()
            .copy(target.copy)
            .ok_or_else(|| refuse(DraftingComponentWorkRefusal::MissingCopy))?;
        if copy.piece() != target.piece {
            return Err(refuse(DraftingComponentWorkRefusal::CopyPiece {
                actual: copy,
            }));
        }
        let piece = root
            .pieces()
            .piece(target.piece)
            .ok_or_else(|| refuse(DraftingComponentWorkRefusal::MissingPiece))?;
        Ok(AdmittedDraftingComponentWork {
            parent: self,
            target,
            copy,
            piece,
            limits,
            demand,
            shape,
        })
    }
}
