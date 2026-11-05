//! Complete original Closure/Buttonhole relations under the same finite semantic cursor.
use super::{CheckedDraftingProgramDirectSelections, DraftingProgramSemanticWorkError};
use crate::{
    construction::{
        AdmittedDraftingGarmentSource, BoundDraftingClosureButtonhole,
        DraftingClosureButtonholeError, DraftingClosureButtonholeLimits, DraftingGarmentProvider,
        DraftingGarmentSemanticField, DraftingGarmentSemanticUse,
    },
    design::{
        DesignReferenceHost, DesignResourceDemand, DesignResourceLimitError, DesignResourceMeter,
        DesignResourceSink, DesignResourceSource,
    },
    ontology::EntityId,
};
use core::fmt;
use std::collections::TryReserveError;

/// One actual Closure Buttonhole field and its complete freshly bounded typed original operation/button origin join.
/// The original garment field has no invented semantic ordinal or mixed programme position.
pub struct DraftingProgramClosureButtonhole<
    's,
    'a,
    H: DesignReferenceHost,
    S: DraftingGarmentProvider<H> + ?Sized,
> {
    occurrence: DraftingGarmentSemanticUse<'a, H>,
    relation: BoundDraftingClosureButtonhole<'s, 'a, H, S>,
}
impl<'s, 'a, H: DesignReferenceHost, S: DraftingGarmentProvider<H> + ?Sized>
    DraftingProgramClosureButtonhole<'s, 'a, H, S>
{
    /// Original garment, Closure Buttonhole inventory index and exact Closure Buttonhole field loan.
    #[must_use]
    pub const fn occurrence(&self) -> DraftingGarmentSemanticUse<'a, H> {
        self.occurrence
    }
    /// Exact original admitted garment parent, request, typed original operation/button origin and fresh bounds.
    #[must_use]
    pub const fn relation(&self) -> &BoundDraftingClosureButtonhole<'s, 'a, H, S> {
        &self.relation
    }
}
impl<H: DesignReferenceHost, S: DraftingGarmentProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramClosureButtonhole<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramClosureButtonhole")
            .field("occurrence", &self.occurrence)
            .finish_non_exhaustive()
    }
}

/// Complete Closure Buttonhole relation class, owning the exact prior input proof and unique cursor.
/// P is the original input-proof type, inferred from the consuming factory. Its original loans
/// survive unchanged; no input/source body is cloned and no self-referential owner is created.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignReferenceHost};
/// fn alter<P,H:DesignReferenceHost,S:DraftingGarmentProvider<H>>(p:&mut BoundDraftingProgramClosureButtonholes<'_,'_,P,H,S>) {p.records.clear();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignReferenceHost};
/// fn duplicate<P,H:DesignReferenceHost,S:DraftingGarmentProvider<H>>(p:BoundDraftingProgramClosureButtonholes<'_,'_,P,H,S>) {let _=p.clone();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignReferenceHost};
/// fn escape<'s,'a,P,H:DesignReferenceHost,S:DraftingGarmentProvider<H>>(p:BoundDraftingProgramClosureButtonholes<'s,'a,P,H,S>)->BoundDraftingProgramClosureButtonholes<'static,'a,P,H,S> {p}
/// ```
pub struct BoundDraftingProgramClosureButtonholes<
    's,
    'a,
    P,
    H: DesignReferenceHost,
    S: DraftingGarmentProvider<H> + ?Sized,
> {
    parent: P,
    demand: Option<DesignResourceDemand>,
    records: Vec<DraftingProgramClosureButtonhole<'s, 'a, H, S>>,
}
impl<'s, 'a, P, H: DesignReferenceHost, S: DraftingGarmentProvider<H> + ?Sized>
    BoundDraftingProgramClosureButtonholes<'s, 'a, P, H, S>
{
    pub(in crate::construction::program::dependencies) fn parent_mut(&mut self) -> &mut P {
        &mut self.parent
    }
    /// Entire original numeric/logical garment input proof and same unique work cursor.
    #[must_use]
    pub const fn parent(&self) -> &P {
        &self.parent
    }
    /// Fresh complete aggregate demand for N>0; None for the actually empty Closure Buttonhole class.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.demand
    }
    /// Every original Closure Buttonhole occurrence once, in actual Closure Buttonhole/local field order.
    #[must_use]
    pub fn records(&self) -> &[DraftingProgramClosureButtonhole<'s, 'a, H, S>] {
        &self.records
    }
    /// Zero for N0; otherwise N+1 complete aggregate/relation pass envelopes.
    #[must_use]
    pub fn child_pass_count(&self) -> usize {
        if self.records.is_empty() {
            0
        } else {
            self.records.len() + 1
        }
    }
    /// Release this class's records and recover its exact prior proof, without refunding work.
    #[must_use]
    pub fn into_parent(self) -> P {
        self.parent
    }
}
impl<P, H: DesignReferenceHost, S: DraftingGarmentProvider<H> + ?Sized> fmt::Debug
    for BoundDraftingProgramClosureButtonholes<'_, '_, P, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundDraftingProgramClosureButtonholes")
            .field("records", &self.records.len())
            .finish_non_exhaustive()
    }
}

/// First complete Closure Buttonhole relation-stage refusal, retaining original typed witnesses.
pub enum DraftingProgramClosureButtonholeRefusal<
    's,
    'w,
    'a,
    I: ?Sized,
    H: DesignReferenceHost,
    S: DraftingGarmentProvider<H> + ?Sized,
> {
    /// Complete population or N+1 pass count cannot fit usize.
    PopulationOverflow,
    /// Whole request refused atomically, without changing prior reservations.
    Work(DraftingProgramSemanticWorkError<'w, I>),
    /// First fresh complete aggregate content cause, before allocation/relation work.
    Resources(DesignResourceLimitError),
    /// Complete N-result reservation failed before any child factory.
    Reservation {
        /// Complete original Closure Buttonhole population.
        records: usize,
        /// Original allocator cause.
        cause: TryReserveError,
    },
    /// First original Closure Buttonhole relation refusal, without an accepted prefix.
    Relation {
        /// Actual original requiring Closure Buttonhole/local field loan.
        occurrence: DraftingGarmentSemanticUse<'a, H>,
        /// Original child parent/content/matching/member/button-origin cause.
        cause: DraftingClosureButtonholeError<'s, 'a, H, S>,
    },
}
impl<I: ?Sized, H: DesignReferenceHost, S: DraftingGarmentProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramClosureButtonholeRefusal<'_, '_, '_, I, H, S>
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
            Self::Relation { occurrence, cause } => f
                .debug_struct("Relation")
                .field("occurrence", occurrence)
                .field("cause", cause)
                .finish(),
        }
    }
}
/// Entire exact prior input proof/cursor and first refusal. Successful private prefixes drop.
pub struct DraftingProgramClosureButtonholeError<
    's,
    'w,
    'a,
    P,
    I: ?Sized,
    H: DesignReferenceHost,
    S: DraftingGarmentProvider<H> + ?Sized,
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
    S: DraftingGarmentProvider<H> + ?Sized,
> {
    parent: P,
    count: Option<usize>,
    demand: Option<DesignResourceDemand>,
    refusal: DraftingProgramClosureButtonholeRefusal<'s, 'w, 'a, I, H, S>,
}
impl<'s, 'w, 'a, P, I: ?Sized, H: DesignReferenceHost, S: DraftingGarmentProvider<H> + ?Sized>
    DraftingProgramClosureButtonholeError<'s, 'w, 'a, P, I, H, S>
{
    /// Entire original input proof and unchanged or newly consumed cursor reservations.
    #[must_use]
    pub const fn parent(&self) -> &P {
        &self.details.parent
    }
    /// Complete original Closure Buttonhole population, when count arithmetic succeeded.
    #[must_use]
    pub const fn relation_count(&self) -> Option<usize> {
        self.details.count
    }
    /// Complete fresh aggregate demand only after its entire walk passed.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.details.demand
    }
    /// First original arithmetic/work/resource/allocator/relation refusal.
    #[must_use]
    pub const fn refusal(&self) -> &DraftingProgramClosureButtonholeRefusal<'s, 'w, 'a, I, H, S> {
        &self.details.refusal
    }
    /// Recover the exact original prior proof without refunding reserved work.
    #[must_use]
    pub fn into_parent(self) -> P {
        self.details.parent
    }
}
impl<P, I: ?Sized, H: DesignReferenceHost, S: DraftingGarmentProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramClosureButtonholeError<'_, '_, '_, P, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramClosureButtonholeError")
            .field("refusal", &self.details.refusal)
            .finish_non_exhaustive()
    }
}
impl<P, I: ?Sized, H: DesignReferenceHost, S: DraftingGarmentProvider<H> + ?Sized> fmt::Display
    for DraftingProgramClosureButtonholeError<'_, '_, '_, P, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "drafting Closure Buttonhole relations: {:?}",
            self.details.refusal
        )
    }
}
impl<P, I: ?Sized, H: DesignReferenceHost, S: DraftingGarmentProvider<H> + ?Sized> std::error::Error
    for DraftingProgramClosureButtonholeError<'_, '_, '_, P, I, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.details.refusal {
            DraftingProgramClosureButtonholeRefusal::Resources(c) => Some(c),
            DraftingProgramClosureButtonholeRefusal::Reservation { cause, .. } => Some(cause),
            DraftingProgramClosureButtonholeRefusal::Work(c) => std::error::Error::source(c),
            DraftingProgramClosureButtonholeRefusal::Relation { cause, .. } => {
                std::error::Error::source(cause)
            }
            DraftingProgramClosureButtonholeRefusal::PopulationOverflow => None,
        }
    }
}
/// Complete original relation class or same entire prior proof and first original cause.
pub type DraftingProgramClosureButtonholeResult<'s, 'w, 'a, P, I, H, S> = Result<
    BoundDraftingProgramClosureButtonholes<'s, 'a, P, H, S>,
    DraftingProgramClosureButtonholeError<'s, 'w, 'a, P, I, H, S>,
>;

fn request<H: DesignReferenceHost>(
    occurrence: DraftingGarmentSemanticUse<'_, H>,
) -> Option<EntityId> {
    match (occurrence.owner(), occurrence.field()) {
        (
            crate::construction::DraftingGarmentSemanticOwner::Closure { closure, .. },
            DraftingGarmentSemanticField::ClosureButtonhole(_),
        ) => Some(closure.id()),
        _ => None,
    }
}
pub(super) struct Outcome<
    's,
    'w,
    'a,
    I: ?Sized,
    H: DesignReferenceHost,
    S: DraftingGarmentProvider<H> + ?Sized,
> {
    count: Option<usize>,
    demand: Option<DesignResourceDemand>,
    result: Result<
        Vec<DraftingProgramClosureButtonhole<'s, 'a, H, S>>,
        DraftingProgramClosureButtonholeRefusal<'s, 'w, 'a, I, H, S>,
    >,
}
impl<'s, 'w, 'a, I: ?Sized, H: DesignReferenceHost, S: DraftingGarmentProvider<H> + ?Sized>
    Outcome<'s, 'w, 'a, I, H, S>
{
    pub(super) fn finish<P>(
        self,
        parent: P,
    ) -> DraftingProgramClosureButtonholeResult<'s, 'w, 'a, P, I, H, S> {
        match self.result {
            Ok(records) => Ok(BoundDraftingProgramClosureButtonholes {
                parent,
                demand: self.demand,
                records,
            }),
            Err(refusal) => Err(DraftingProgramClosureButtonholeError {
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
    S: DraftingGarmentProvider<H> + ?Sized,
>(
    cursor: &mut CheckedDraftingProgramDirectSelections<'w, I>,
    garment: &'s AdmittedDraftingGarmentSource<'a, H, S>,
    charge: impl FnOnce(&mut dyn DesignResourceSink) -> Result<(), DesignResourceLimitError>,
) -> Outcome<'s, 'w, 'a, I, H, S> {
    use DraftingProgramClosureButtonholeRefusal as R;
    let mut count = None;
    let mut demand = None;
    let result = (|| {
        let source = garment.source().garment;
        let n = source
            .pocket_closure_dependencies()
            .filter_map(request)
            .try_fold(0usize, |n, _| n.checked_add(1))
            .ok_or(R::PopulationOverflow)?;
        count = Some(n);
        let passes = if n == 0 {
            0
        } else {
            n.checked_add(1).ok_or(R::PopulationOverflow)?
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
        let limits = DraftingClosureButtonholeLimits::new(
            content,
            cursor.work().shape().per_pass_matching_bound(),
        );
        for occurrence in source.pocket_closure_dependencies() {
            let Some(closure) = request(occurrence) else {
                continue;
            };
            let relation = garment
                .bind_closure_buttonhole(closure, limits)
                .map_err(|cause| R::Relation { occurrence, cause })?;
            records.push(DraftingProgramClosureButtonhole {
                occurrence,
                relation,
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
