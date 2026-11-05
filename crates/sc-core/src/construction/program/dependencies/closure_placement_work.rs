//! Complete original Closure placement members and current copy/Piece work under one cursor.
use super::{CheckedDraftingProgramDirectSelections, DraftingProgramSemanticWorkError};
use crate::{
    construction::{
        AdmittedDraftingComponentWork, AdmittedDraftingGarmentSource,
        BoundDraftingClosurePlacementMember, DraftingClosurePlacementLimits,
        DraftingClosurePlacementMemberError, DraftingClosurePlacementRequest,
        DraftingComponentBindingLimits, DraftingComponentProvider, DraftingComponentWorkError,
        DraftingGarmentSemanticField, DraftingGarmentSemanticUse,
    },
    design::{
        DesignReferenceHost, DesignResourceDemand, DesignResourceLimitError, DesignResourceMeter,
        DesignResourceSink, DesignResourceSource,
    },
    ontology::{ClosurePlacementRole, PocketPieceRef},
};
use core::fmt;
use std::collections::TryReserveError;

/// One actual placement-role field and its complete freshly bounded current copy/Piece join.
/// The original garment field has no invented semantic ordinal or mixed programme position.
pub struct DraftingProgramClosurePlacementWork<
    's,
    'a,
    H: DesignReferenceHost,
    S: DraftingComponentProvider<H> + ?Sized,
> {
    occurrence: DraftingGarmentSemanticUse<'a, H>,
    member: BoundDraftingClosurePlacementMember<'s, 'a, H, S>,
    work: AdmittedDraftingComponentWork<'s, 'a, H, S>,
}
impl<'s, 'a, H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized>
    DraftingProgramClosurePlacementWork<'s, 'a, H, S>
{
    /// Original garment, Closure placement inventory index and exact placement-role field loan.
    #[must_use]
    pub const fn occurrence(&self) -> DraftingGarmentSemanticUse<'a, H> {
        self.occurrence
    }
    /// Exact original Closure/instance/role-to-canonical-notion relation under fresh bounds.
    #[must_use]
    pub const fn member(&self) -> &BoundDraftingClosurePlacementMember<'s, 'a, H, S> {
        &self.member
    }
    /// Exact original admitted garment parent, request, current copy/Piece and fresh bounds.
    #[must_use]
    pub const fn work(&self) -> &AdmittedDraftingComponentWork<'s, 'a, H, S> {
        &self.work
    }
}
impl<H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramClosurePlacementWork<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramClosurePlacementWork")
            .field("occurrence", &self.occurrence)
            .finish_non_exhaustive()
    }
}

/// Complete Closure placement current-member class, owning the exact prior input proof and unique cursor.
/// P is the original input-proof type, inferred from the consuming factory. Its original loans
/// survive unchanged; no input/source body is cloned and no self-referential owner is created.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignReferenceHost};
/// fn alter<P,H:DesignReferenceHost,S:DraftingComponentProvider<H>>(p:&mut BoundDraftingProgramClosurePlacementWork<'_,'_,P,H,S>) {p.records.clear();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignReferenceHost};
/// fn duplicate<P,H:DesignReferenceHost,S:DraftingComponentProvider<H>>(p:BoundDraftingProgramClosurePlacementWork<'_,'_,P,H,S>) {let _=p.clone();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignReferenceHost};
/// fn escape<'s,'a,P,H:DesignReferenceHost,S:DraftingComponentProvider<H>>(p:BoundDraftingProgramClosurePlacementWork<'s,'a,P,H,S>)->BoundDraftingProgramClosurePlacementWork<'static,'a,P,H,S> {p}
/// ```
pub struct BoundDraftingProgramClosurePlacementWork<
    's,
    'a,
    P,
    H: DesignReferenceHost,
    S: DraftingComponentProvider<H> + ?Sized,
> {
    parent: P,
    demand: Option<DesignResourceDemand>,
    records: Vec<DraftingProgramClosurePlacementWork<'s, 'a, H, S>>,
}
impl<'s, 'a, P, H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized>
    BoundDraftingProgramClosurePlacementWork<'s, 'a, P, H, S>
{
    /// Entire original numeric/logical garment input proof and same unique work cursor.
    #[must_use]
    pub const fn parent(&self) -> &P {
        &self.parent
    }
    /// Fresh complete aggregate demand for N>0; None for the actually empty Closure placement class.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.demand
    }
    /// Every original placement-role occurrence once, in actual Closure placement/local field order.
    #[must_use]
    pub fn records(&self) -> &[DraftingProgramClosurePlacementWork<'s, 'a, H, S>] {
        &self.records
    }
    /// Zero for N0; otherwise 2N+1 complete aggregate/current-member pass envelopes.
    #[must_use]
    pub fn child_pass_count(&self) -> usize {
        if self.records.is_empty() {
            0
        } else {
            2 * self.records.len() + 1
        }
    }
    /// Release this class's records and recover its exact prior proof, without refunding work.
    #[must_use]
    pub fn into_parent(self) -> P {
        self.parent
    }
}
impl<P, H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized> fmt::Debug
    for BoundDraftingProgramClosurePlacementWork<'_, '_, P, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundDraftingProgramClosurePlacementWork")
            .field("records", &self.records.len())
            .finish_non_exhaustive()
    }
}

/// First complete Closure placement placement-work-stage refusal, retaining original typed witnesses.
pub enum DraftingProgramClosurePlacementWorkRefusal<
    's,
    'w,
    'a,
    I: ?Sized,
    H: DesignReferenceHost,
    S: DraftingComponentProvider<H> + ?Sized,
> {
    /// Complete population or 2N+1 pass count cannot fit usize.
    PopulationOverflow,
    /// Whole request refused atomically, without changing prior reservations.
    Work(DraftingProgramSemanticWorkError<'w, I>),
    /// First fresh complete aggregate content cause, before allocation/current-member work.
    Resources(DesignResourceLimitError),
    /// Complete N-result reservation failed before any child factory.
    Reservation {
        /// Complete original placement-role population.
        records: usize,
        /// Original allocator cause.
        cause: TryReserveError,
    },
    /// First original requiring Closure/instance/role-to-notion member cause.
    Member {
        /// Original garment Closure/instance occurrence.
        occurrence: DraftingGarmentSemanticUse<'a, H>,
        /// Actual request from that original instance and role.
        request: DraftingClosurePlacementRequest,
        /// Original whole content/work/member refusal with actual partial requiring loans.
        cause: DraftingClosurePlacementMemberError<'s, 'a, H, S>,
    },
    /// First original placement-role current-member refusal, without an accepted prefix.
    Component {
        /// Actual original requiring Closure/instance field loan.
        occurrence: DraftingGarmentSemanticUse<'a, H>,
        /// Exact freshly bound original requiring role and canonical notion member.
        member: Box<BoundDraftingClosurePlacementMember<'s, 'a, H, S>>,
        /// Original child parent/request/content/matching/current copy/Piece cause.
        cause: DraftingComponentWorkError<'s, 'a, H, S>,
    },
}
impl<I: ?Sized, H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramClosurePlacementWorkRefusal<'_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PopulationOverflow => f.write_str("PopulationOverflow"),
            Self::Work(c) => f.debug_tuple("Work").field(c).finish(),
            Self::Resources(c) => f.debug_tuple("Resources").field(c).finish(),
            Self::Reservation { records, cause } => f
                .debug_struct("Reservation")
                .field("records", records)
                .field("cause", cause)
                .finish(),
            Self::Member {
                occurrence,
                request,
                cause,
            } => f
                .debug_struct("Member")
                .field("occurrence", occurrence)
                .field("request", request)
                .field("cause", cause)
                .finish(),
            Self::Component {
                occurrence,
                member,
                cause,
            } => f
                .debug_struct("Component")
                .field("occurrence", occurrence)
                .field("member", member)
                .field("cause", cause)
                .finish(),
        }
    }
}
/// Entire exact prior input proof/cursor and first refusal. Successful private prefixes drop.
pub struct DraftingProgramClosurePlacementWorkError<
    's,
    'w,
    'a,
    P,
    I: ?Sized,
    H: DesignReferenceHost,
    S: DraftingComponentProvider<H> + ?Sized,
> {
    details: Box<Failure<'s, 'w, 'a, P, I, H, S>>,
}
struct Failure<
    's,
    'w,
    'a,
    P,
    I: ?Sized,
    H: DesignReferenceHost,
    S: DraftingComponentProvider<H> + ?Sized,
> {
    parent: P,
    count: Option<usize>,
    demand: Option<DesignResourceDemand>,
    refusal: DraftingProgramClosurePlacementWorkRefusal<'s, 'w, 'a, I, H, S>,
}
impl<
        's,
        'w,
        'a,
        P,
        I: ?Sized,
        H: DesignReferenceHost,
        S: DraftingComponentProvider<H> + ?Sized,
    > DraftingProgramClosurePlacementWorkError<'s, 'w, 'a, P, I, H, S>
{
    /// Entire original input proof and unchanged or newly consumed cursor reservations.
    #[must_use]
    pub const fn parent(&self) -> &P {
        &self.details.parent
    }
    /// Complete original placement-role population, when count arithmetic succeeded.
    #[must_use]
    pub const fn component_count(&self) -> Option<usize> {
        self.details.count
    }
    /// Complete fresh aggregate demand only after its entire walk passed.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.details.demand
    }
    /// First original arithmetic/work/resource/allocator/current-member refusal.
    #[must_use]
    pub const fn refusal(
        &self,
    ) -> &DraftingProgramClosurePlacementWorkRefusal<'s, 'w, 'a, I, H, S> {
        &self.details.refusal
    }
    /// Recover the exact original prior proof without refunding reserved work.
    #[must_use]
    pub fn into_parent(self) -> P {
        self.details.parent
    }
}
impl<P, I: ?Sized, H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramClosurePlacementWorkError<'_, '_, '_, P, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramClosurePlacementWorkError")
            .field("refusal", &self.details.refusal)
            .finish_non_exhaustive()
    }
}
impl<P, I: ?Sized, H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized> fmt::Display
    for DraftingProgramClosurePlacementWorkError<'_, '_, '_, P, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "drafting Closure placement current members: {:?}",
            self.details.refusal
        )
    }
}
impl<P, I: ?Sized, H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized>
    std::error::Error for DraftingProgramClosurePlacementWorkError<'_, '_, '_, P, I, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.details.refusal {
            DraftingProgramClosurePlacementWorkRefusal::Resources(c) => Some(c),
            DraftingProgramClosurePlacementWorkRefusal::Reservation { cause, .. } => Some(cause),
            DraftingProgramClosurePlacementWorkRefusal::Work(c) => std::error::Error::source(c),
            DraftingProgramClosurePlacementWorkRefusal::Member { cause, .. } => {
                std::error::Error::source(cause)
            }
            DraftingProgramClosurePlacementWorkRefusal::Component { cause, .. } => {
                std::error::Error::source(cause)
            }
            DraftingProgramClosurePlacementWorkRefusal::PopulationOverflow => None,
        }
    }
}
/// Complete original current-member class or same entire prior proof and first original cause.
pub type DraftingProgramClosurePlacementWorkResult<'s, 'w, 'a, P, I, H, S> = Result<
    BoundDraftingProgramClosurePlacementWork<'s, 'a, P, H, S>,
    DraftingProgramClosurePlacementWorkError<'s, 'w, 'a, P, I, H, S>,
>;

fn requests<H: DesignReferenceHost>(
    garment: &crate::design::DesignGarment<H>,
) -> impl Iterator<
    Item = (
        DraftingGarmentSemanticUse<'_, H>,
        DraftingClosurePlacementRequest,
    ),
> {
    garment
        .pocket_closure_dependencies()
        .filter_map(
            |occurrence| match (occurrence.owner(), occurrence.field()) {
                (
                    crate::construction::DraftingGarmentSemanticOwner::Closure { closure, .. },
                    DraftingGarmentSemanticField::ClosureInstance { instance, .. },
                ) => Some((occurrence, closure, instance)),
                _ => None,
            },
        )
        .flat_map(|(occurrence, closure, instance)| {
            [ClosurePlacementRole::First, ClosurePlacementRole::Second]
                .into_iter()
                .map(move |role| {
                    (
                        occurrence,
                        DraftingClosurePlacementRequest {
                            closure: closure.id(),
                            instance: instance.id,
                            role,
                        },
                    )
                })
        })
}
pub(super) struct Outcome<
    's,
    'w,
    'a,
    I: ?Sized,
    H: DesignReferenceHost,
    S: DraftingComponentProvider<H> + ?Sized,
> {
    count: Option<usize>,
    demand: Option<DesignResourceDemand>,
    result: Result<
        Vec<DraftingProgramClosurePlacementWork<'s, 'a, H, S>>,
        DraftingProgramClosurePlacementWorkRefusal<'s, 'w, 'a, I, H, S>,
    >,
}
impl<'s, 'w, 'a, I: ?Sized, H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized>
    Outcome<'s, 'w, 'a, I, H, S>
{
    pub(super) fn finish<P>(
        self,
        parent: P,
    ) -> DraftingProgramClosurePlacementWorkResult<'s, 'w, 'a, P, I, H, S> {
        match self.result {
            Ok(records) => Ok(BoundDraftingProgramClosurePlacementWork {
                parent,
                demand: self.demand,
                records,
            }),
            Err(refusal) => Err(DraftingProgramClosurePlacementWorkError {
                details: Box::new(Failure {
                    parent,
                    count: self.count,
                    demand: self.demand,
                    refusal,
                }),
            }),
        }
    }
}
pub(super) fn prepare<
    's,
    'w,
    'a,
    I: ?Sized,
    H: DesignReferenceHost + DesignResourceSource,
    S: DraftingComponentProvider<H> + ?Sized,
>(
    cursor: &mut CheckedDraftingProgramDirectSelections<'w, I>,
    garment: &'s AdmittedDraftingGarmentSource<'a, H, S>,
    charge: impl FnOnce(&mut dyn DesignResourceSink) -> Result<(), DesignResourceLimitError>,
) -> Outcome<'s, 'w, 'a, I, H, S> {
    use DraftingProgramClosurePlacementWorkRefusal as R;
    let mut count = None;
    let mut demand = None;
    let result = (|| {
        let source = garment.source().garment;
        let n = requests(source)
            .try_fold(0usize, |n, _| n.checked_add(1))
            .ok_or(R::PopulationOverflow)?;
        count = Some(n);
        let passes = if n == 0 {
            0
        } else {
            n.checked_mul(2)
                .and_then(|twice| twice.checked_add(1))
                .ok_or(R::PopulationOverflow)?
        };
        cursor.reserve_child_passes(passes).map_err(R::Work)?;
        if n == 0 {
            return Ok(Vec::new());
        }
        let content = cursor.limits().population().content();
        let mut meter = DesignResourceMeter::new(content);
        let charged = charge(&mut meter);
        let complete = meter.finish().map_err(R::Resources)?;
        charged.map_err(R::Resources)?;
        demand = Some(complete);
        let mut records = Vec::new();
        records
            .try_reserve_exact(n)
            .map_err(|cause| R::Reservation { records: n, cause })?;
        let limits = DraftingComponentBindingLimits::new(
            content,
            cursor.work().shape().per_pass_matching_bound(),
        );
        let member_limits = DraftingClosurePlacementLimits::new(
            content,
            cursor.work().shape().per_pass_matching_bound(),
        );
        for (occurrence, request) in requests(source) {
            let member = garment
                .bind_closure_placement_member(request, member_limits)
                .map_err(|cause| R::Member {
                    occurrence,
                    request,
                    cause,
                })?;
            let target = PocketPieceRef {
                copy: member.notion().copy(),
                piece: member.notion().piece(),
            };
            let work = match garment.prepare_component_binding(target, limits) {
                Ok(work) => work,
                Err(cause) => {
                    return Err(R::Component {
                        occurrence,
                        member: Box::new(member),
                        cause,
                    })
                }
            };
            records.push(DraftingProgramClosurePlacementWork {
                occurrence,
                member,
                work,
            });
        }
        Ok(records)
    })();
    Outcome {
        count,
        demand,
        result,
    }
}
