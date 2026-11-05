//! Complete original geometry suppliers after fresh content/work and whole creator classification.
use super::{
    AdmittedDraftingProgramGeometryWork, DraftingGeometryStepRequest,
    DraftingProgramGeometryCreatorRefusal, DraftingProgramGeometryRequest,
    DraftingProgramGeometryTarget,
};
use crate::{
    construction::{
        DraftingGeometryBinding, DraftingGeometryError, DraftingGeometryStepProvider,
        DraftingInitialInputProvider, DraftingPlannedOutputSource,
    },
    design::DesignMetadataHost,
};
use core::fmt;
use std::collections::TryReserveError;

/// First original planned response mismatch, checked in this order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingProgramPlannedGeometryIssue {
    /// Authentic family field borrows another actual creating input, even with equal identifiers.
    Input,
    /// Authentic output has a different Point/Edge reference or tag.
    Reference,
    /// Actual frame source record differs from the held context.
    Frame,
    /// Actual snapshot source record differs from the held context.
    Snapshot,
}
/// One supplier for an original occurrence. Planned suppliers remain unissued and carry no values.
/// Individual variants cannot construct the private complete programme geometry owner.
pub enum DraftingProgramGeometrySupplier<
    'g,
    'j,
    H: DesignMetadataHost,
    S: DraftingInitialInputProvider<H> + ?Sized,
> {
    /// Original opaque existing geometry with exact reference/frame/snapshot binding.
    Existing(DraftingGeometryBinding<'g, H, S>),
    /// Strictly earlier exact fixed output from its original native schema, without a lookup.
    Fixed(DraftingGeometryStepRequest<'g, 'j>),
    /// Strictly earlier authentic variable family field, including outputs without global names.
    Planned {
        /// Original creating step, reference and input.
        request: DraftingGeometryStepRequest<'g, 'j>,
        /// Actual matched family field and original context records; no physical geometry payload.
        source: DraftingPlannedOutputSource<'g, S::Frame, S::Snapshot>,
    },
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramGeometrySupplier<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Existing(_) => "DraftingProgramGeometrySupplier::Existing(..)",
            Self::Fixed(_) => "DraftingProgramGeometrySupplier::Fixed(..)",
            Self::Planned { .. } => "DraftingProgramGeometrySupplier::Planned(..)",
        })
    }
}
/// Original first allocation, creator or supplier refusal. No successful prefix is exposed.
pub enum DraftingProgramGeometryBindingRefusal<
    'g,
    'j,
    H: DesignMetadataHost,
    S: DraftingInitialInputProvider<H> + ?Sized,
> {
    /// First original creator failure from a complete-programme scan, before any source lookup.
    Creator {
        /// Zero-based original merged occurrence index.
        index: usize,
        /// Original consumer/reference and creator refusal witnesses.
        request: DraftingProgramGeometryRequest<'g, 'j>,
        /// Original typed creator cause, without replacement by a formula diagnostic.
        cause: DraftingProgramGeometryCreatorRefusal<'g, 'j>,
    },
    /// Original typed existing-source or complete reference/frame/snapshot mismatch.
    Existing {
        /// Zero-based original merged occurrence index.
        index: usize,
        /// Original consumer/reference and existing classification.
        request: DraftingProgramGeometryRequest<'g, 'j>,
        /// Original context source error and its complete witnesses.
        cause: Box<DraftingGeometryError<'g, H, S>>,
    },
    /// Original planned provider refusal, without fallback to live geometry.
    PlannedSource {
        /// Zero-based original merged occurrence index.
        index: usize,
        /// Original consumer/reference and genuine earlier creator.
        request: DraftingProgramGeometryRequest<'g, 'j>,
        /// Original typed source cause.
        cause: Box<S::Error>,
    },
    /// Original authentic family response at its first field mismatch.
    PlannedMismatch {
        /// Zero-based original merged occurrence index.
        index: usize,
        /// Original consumer/reference and genuine earlier creator.
        request: DraftingProgramGeometryRequest<'g, 'j>,
        /// Complete raw response; opaque original records are borrowed, never cloned.
        returned: DraftingPlannedOutputSource<'g, S::Frame, S::Snapshot>,
        /// First actual field that failed binding.
        issue: DraftingProgramPlannedGeometryIssue,
    },
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramGeometryBindingRefusal<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Creator { index, cause, .. } => f
                .debug_struct("Creator")
                .field("index", index)
                .field("cause", cause)
                .finish(),
            Self::Existing { index, .. } => f
                .debug_struct("Existing")
                .field("index", index)
                .finish_non_exhaustive(),
            Self::PlannedSource { index, .. } => f
                .debug_struct("PlannedSource")
                .field("index", index)
                .finish_non_exhaustive(),
            Self::PlannedMismatch { index, issue, .. } => f
                .debug_struct("PlannedMismatch")
                .field("index", index)
                .field("issue", issue)
                .finish_non_exhaustive(),
        }
    }
}

/// First reservation cause or boxed semantic binding refusal. Allocation errors remain inline.
pub enum DraftingProgramGeometryRefusal<
    'g,
    'j,
    H: DesignMetadataHost,
    S: DraftingInitialInputProvider<H> + ?Sized,
> {
    /// Original complete request/supplier vector reservation cause, before callbacks.
    Allocation(TryReserveError),
    /// Original creator or source refusal, retaining its complete original occurrence/witnesses.
    Binding(Box<DraftingProgramGeometryBindingRefusal<'g, 'j, H, S>>),
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramGeometryRefusal<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Allocation(error) => f.debug_tuple("Allocation").field(error).finish(),
            Self::Binding(error) => error.fmt(f),
        }
    }
}
/// Complete original admitted work and first binding refusal; no partial result can escape.
pub struct DraftingProgramGeometryError<
    'g,
    'k,
    'j,
    'i,
    'r,
    'b,
    'n,
    'p,
    'a,
    H: DesignMetadataHost,
    S: DraftingInitialInputProvider<H> + ?Sized,
> {
    work: &'g AdmittedDraftingProgramGeometryWork<'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
    refusal: DraftingProgramGeometryRefusal<'g, 'j, H, S>,
}
impl<
        'g,
        'k,
        'j,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignMetadataHost,
        S: DraftingInitialInputProvider<H> + ?Sized,
    > DraftingProgramGeometryError<'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Exact selected canonical owner, complete fresh demand/shape and geometry policy.
    #[must_use]
    pub const fn work(
        &self,
    ) -> &'g AdmittedDraftingProgramGeometryWork<'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S> {
        self.work
    }
    /// Original allocation or first semantic cause and actual original witnesses.
    #[must_use]
    pub const fn refusal(&self) -> &DraftingProgramGeometryRefusal<'g, 'j, H, S> {
        &self.refusal
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramGeometryError<'_, '_, '_, '_, '_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.refusal.fmt(f)
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> fmt::Display
    for DraftingProgramGeometryError<'_, '_, '_, '_, '_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "complete drafting geometry binding: {:?}", self.refusal)
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> std::error::Error
    for DraftingProgramGeometryError<'_, '_, '_, '_, '_, '_, '_, '_, '_, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.refusal {
            DraftingProgramGeometryRefusal::Allocation(error) => Some(error),
            DraftingProgramGeometryRefusal::Binding(error) => match error.as_ref() {
                DraftingProgramGeometryBindingRefusal::Existing { cause, .. } => {
                    std::error::Error::source(cause.as_ref())
                }
                DraftingProgramGeometryBindingRefusal::PlannedSource { cause, .. } => {
                    Some(cause.as_ref())
                }
                _ => None,
            },
        }
    }
}
/// Complete original geometry associations, borrowing admitted work and every actual supplier.
/// Private fields prevent publication of a prefix or replacement occurrence. This is no semantic,
/// numeric/physical, execution, allocator-capacity or whole-programme acceptance certificate.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn alter<H:DesignMetadataHost,S:DraftingInitialInputProvider<H>>(
///     proof:&mut BoundDraftingProgramGeometry<'_, '_, '_, '_, '_, '_, '_, '_, '_,H,S>) {proof.suppliers=vec![];}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn escape<'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a,H:DesignMetadataHost,S:DraftingInitialInputProvider<H>>(
///     proof:BoundDraftingProgramGeometry<'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>)->BoundDraftingProgramGeometry<'static,'k,'j,'i,'r,'b,'n,'p,'a,H,S> {proof}
/// ```
pub struct BoundDraftingProgramGeometry<
    'g,
    'k,
    'j,
    'i,
    'r,
    'b,
    'n,
    'p,
    'a,
    H: DesignMetadataHost,
    S: DraftingInitialInputProvider<H> + ?Sized,
> {
    work: &'g AdmittedDraftingProgramGeometryWork<'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
    requests: Vec<DraftingProgramGeometryRequest<'g, 'j>>,
    suppliers: Vec<DraftingProgramGeometrySupplier<'g, 'j, H, S>>,
}
impl<
        'g,
        'k,
        'j,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignMetadataHost,
        S: DraftingInitialInputProvider<H> + ?Sized,
    > BoundDraftingProgramGeometry<'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Exact original admitted work and all earlier original owners/policies.
    #[must_use]
    pub const fn work(
        &self,
    ) -> &'g AdmittedDraftingProgramGeometryWork<'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S> {
        self.work
    }
    /// Complete original merged occurrence order; root uses precede mixed uses.
    #[must_use]
    pub fn requests(&self) -> &[DraftingProgramGeometryRequest<'g, 'j>] {
        &self.requests
    }
    /// Exactly one bound supplier per request, in the same complete order, without deduplication.
    #[must_use]
    pub fn suppliers(&self) -> &[DraftingProgramGeometrySupplier<'g, 'j, H, S>] {
        &self.suppliers
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> fmt::Debug
    for BoundDraftingProgramGeometry<'_, '_, '_, '_, '_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundDraftingProgramGeometry")
            .field("occurrences", &self.requests.len())
            .finish_non_exhaustive()
    }
}
impl<
        'g,
        'k,
        'j,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignMetadataHost,
        S: DraftingInitialInputProvider<H> + DraftingGeometryStepProvider<H> + ?Sized,
    > AdmittedDraftingProgramGeometryWork<'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Reserve both complete E-sized vectors under the selected fresh work allowance, then classify
    /// every original creator before any source lookup. Bind existing sources, fixed earlier schemas
    /// and authentic variable fields in original merged order. Variable checks are original input,
    /// exact reference, frame record, snapshot record, in that order. No names or values are made.
    ///
    /// ```
    /// use sc_core::{construction::*,design::DesignMetadataHost};
    /// fn bound_count<H:DesignMetadataHost,S:DraftingInitialInputProvider<H> + DraftingGeometryStepProvider<H> + ?Sized>(
    ///     work:&AdmittedDraftingProgramGeometryWork<'_, '_, '_, '_, '_, '_, '_, '_, '_,H,S>) -> Result<usize,String> {
    ///     Ok(work.bind_geometry().map_err(|error|error.to_string())?.suppliers().len())
    /// }
    /// ```
    /// # Errors
    /// Original inline reservation cause, first creator refusal before lookups, then first source
    /// cause or complete mismatch. All original owners/policies survive; no successful prefix does.
    pub fn bind_geometry(
        &'g self,
    ) -> Result<
        BoundDraftingProgramGeometry<'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
        DraftingProgramGeometryError<'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
    > {
        use DraftingProgramGeometryBindingRefusal as R;
        use DraftingProgramPlannedGeometryIssue as M;
        let refuse = |refusal| DraftingProgramGeometryError {
            work: self,
            refusal,
        };
        let binding = |refusal| refuse(DraftingProgramGeometryRefusal::Binding(Box::new(refusal)));
        let mut requests = Vec::new();
        requests
            .try_reserve_exact(self.shape().occurrence_count())
            .map_err(|cause| refuse(DraftingProgramGeometryRefusal::Allocation(cause)))?;
        let mut suppliers = Vec::new();
        suppliers
            .try_reserve_exact(self.shape().occurrence_count())
            .map_err(|cause| refuse(DraftingProgramGeometryRefusal::Allocation(cause)))?;
        for (index, request) in self.inputs().geometry_requests().enumerate() {
            request.target().map_err(|cause| {
                binding(R::Creator {
                    index,
                    request,
                    cause,
                })
            })?;
            requests.push(request);
        }
        let context = self
            .inputs()
            .inputs()
            .order()
            .inputs()
            .roles()
            .positions()
            .work()
            .inputs()
            .program()
            .admitted()
            .inputs()
            .context();
        for (index, request) in requests.iter().copied().enumerate() {
            let target = match request.target() {
                Ok(target) => target,
                Err(_) => unreachable!("complete immutable creator preflight"),
            };
            let supplier = match target {
                DraftingProgramGeometryTarget::Existing => {
                    DraftingProgramGeometrySupplier::Existing(
                        context
                            .bind_existing_geometry(request.occurrence().reference())
                            .map_err(|cause| {
                                binding(R::Existing {
                                    index,
                                    request,
                                    cause: Box::new(cause),
                                })
                            })?,
                    )
                }
                DraftingProgramGeometryTarget::Earlier(earlier)
                    if !earlier.requires_role_binding() =>
                {
                    DraftingProgramGeometrySupplier::Fixed(earlier)
                }
                DraftingProgramGeometryTarget::Earlier(earlier) => {
                    let returned = context
                        .source()
                        .planned_geometry(earlier)
                        .map_err(|cause| {
                            binding(R::PlannedSource {
                                index,
                                request,
                                cause: Box::new(cause),
                            })
                        })?;
                    let issue = if !returned.output.is_from_input(earlier.input()) {
                        Some(M::Input)
                    } else if returned.output.reference() != earlier.reference() {
                        Some(M::Reference)
                    } else if !context.frame().is_same_record(returned.frame) {
                        Some(M::Frame)
                    } else if !context.snapshot().is_same_record(returned.snapshot) {
                        Some(M::Snapshot)
                    } else {
                        None
                    };
                    if let Some(issue) = issue {
                        return Err(binding(R::PlannedMismatch {
                            index,
                            request,
                            returned,
                            issue,
                        }));
                    }
                    DraftingProgramGeometrySupplier::Planned {
                        request: earlier,
                        source: returned,
                    }
                }
            };
            suppliers.push(supplier);
        }
        Ok(BoundDraftingProgramGeometry {
            work: self,
            requests,
            suppliers,
        })
    }
}

mod retained_resources;
