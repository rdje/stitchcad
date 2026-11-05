//! Complete original Pocket current-copy/Piece joins under the existing unique semantic cursor.
use super::{CheckedDraftingProgramDirectSelections, DraftingProgramSemanticWorkError};
use crate::{
    construction::{
        AdmittedDraftingComponentWork, AdmittedDraftingGarmentSource,
        DraftingComponentBindingLimits, DraftingComponentProvider, DraftingComponentWorkError,
        DraftingGarmentSemanticField, DraftingGarmentSemanticUse,
    },
    design::{
        DesignReferenceHost, DesignResourceDemand, DesignResourceLimitError, DesignResourceMeter,
        DesignResourceSink, DesignResourceSource,
    },
    ontology::PocketPieceRef,
};
use core::fmt;
use std::collections::TryReserveError;

/// One actual served/component field and its complete freshly bounded current copy/Piece join.
/// The original garment field has no invented semantic ordinal or mixed programme position.
pub struct DraftingProgramPocketWork<
    's,
    'a,
    H: DesignReferenceHost,
    S: DraftingComponentProvider<H> + ?Sized,
> {
    occurrence: DraftingGarmentSemanticUse<'a, H>,
    work: AdmittedDraftingComponentWork<'s, 'a, H, S>,
}
impl<'s, 'a, H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized>
    DraftingProgramPocketWork<'s, 'a, H, S>
{
    /// Original garment, Pocket inventory index and exact served/component field loan.
    #[must_use]
    pub const fn occurrence(&self) -> DraftingGarmentSemanticUse<'a, H> {
        self.occurrence
    }
    /// Exact original admitted garment parent, request, current copy/Piece and fresh bounds.
    #[must_use]
    pub const fn work(&self) -> &AdmittedDraftingComponentWork<'s, 'a, H, S> {
        &self.work
    }
}
impl<H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramPocketWork<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramPocketWork")
            .field("occurrence", &self.occurrence)
            .finish_non_exhaustive()
    }
}

/// Complete Pocket current-member class, owning the exact prior input proof and unique cursor.
/// P is the original input-proof type, inferred from the consuming factory. Its original loans
/// survive unchanged; no input/source body is cloned and no self-referential owner is created.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignReferenceHost};
/// fn alter<P,H:DesignReferenceHost,S:DraftingComponentProvider<H>>(p:&mut BoundDraftingProgramPocketWork<'_,'_,P,H,S>) {p.records.clear();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignReferenceHost};
/// fn duplicate<P,H:DesignReferenceHost,S:DraftingComponentProvider<H>>(p:BoundDraftingProgramPocketWork<'_,'_,P,H,S>) {let _=p.clone();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignReferenceHost};
/// fn escape<'s,'a,P,H:DesignReferenceHost,S:DraftingComponentProvider<H>>(p:BoundDraftingProgramPocketWork<'s,'a,P,H,S>)->BoundDraftingProgramPocketWork<'static,'a,P,H,S> {p}
/// ```
pub struct BoundDraftingProgramPocketWork<
    's,
    'a,
    P,
    H: DesignReferenceHost,
    S: DraftingComponentProvider<H> + ?Sized,
> {
    parent: P,
    demand: Option<DesignResourceDemand>,
    records: Vec<DraftingProgramPocketWork<'s, 'a, H, S>>,
}
impl<'s, 'a, P, H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized>
    BoundDraftingProgramPocketWork<'s, 'a, P, H, S>
{
    pub(super) fn parts(&mut self) -> (&mut P, &[DraftingProgramPocketWork<'s, 'a, H, S>]) {
        (&mut self.parent, &self.records)
    }
    /// Entire original numeric/logical garment input proof and same unique work cursor.
    #[must_use]
    pub const fn parent(&self) -> &P {
        &self.parent
    }
    /// Fresh complete aggregate demand for N>0; None for the actually empty Pocket class.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.demand
    }
    /// Every original served/component occurrence once, in actual Pocket/local field order.
    #[must_use]
    pub fn records(&self) -> &[DraftingProgramPocketWork<'s, 'a, H, S>] {
        &self.records
    }
    /// Zero for N0; otherwise N+1 complete aggregate/current-member pass envelopes.
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
impl<P, H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized> fmt::Debug
    for BoundDraftingProgramPocketWork<'_, '_, P, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundDraftingProgramPocketWork")
            .field("records", &self.records.len())
            .finish_non_exhaustive()
    }
}

/// First complete Pocket current-member-stage refusal, retaining original typed witnesses.
pub enum DraftingProgramPocketWorkRefusal<
    's,
    'w,
    'a,
    I: ?Sized,
    H: DesignReferenceHost,
    S: DraftingComponentProvider<H> + ?Sized,
> {
    /// Complete population or N+1 pass count cannot fit usize.
    PopulationOverflow,
    /// Whole request refused atomically, without changing prior reservations.
    Work(DraftingProgramSemanticWorkError<'w, I>),
    /// First fresh complete aggregate content cause, before allocation/current-member work.
    Resources(DesignResourceLimitError),
    /// Complete N-result reservation failed before any child factory.
    Reservation {
        /// Complete original served/component population.
        records: usize,
        /// Original allocator cause.
        cause: TryReserveError,
    },
    /// First original served/component current-member refusal, without an accepted prefix.
    Component {
        /// Actual original requiring Pocket/local field loan.
        occurrence: DraftingGarmentSemanticUse<'a, H>,
        /// Original child parent/request/content/matching/current copy/Piece cause.
        cause: DraftingComponentWorkError<'s, 'a, H, S>,
    },
}
impl<I: ?Sized, H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramPocketWorkRefusal<'_, '_, '_, I, H, S>
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
            Self::Component { occurrence, cause } => f
                .debug_struct("Component")
                .field("occurrence", occurrence)
                .field("cause", cause)
                .finish(),
        }
    }
}
/// Entire exact prior input proof/cursor and first refusal. Successful private prefixes drop.
pub struct DraftingProgramPocketWorkError<
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
    refusal: DraftingProgramPocketWorkRefusal<'s, 'w, 'a, I, H, S>,
}
impl<
        's,
        'w,
        'a,
        P,
        I: ?Sized,
        H: DesignReferenceHost,
        S: DraftingComponentProvider<H> + ?Sized,
    > DraftingProgramPocketWorkError<'s, 'w, 'a, P, I, H, S>
{
    /// Entire original input proof and unchanged or newly consumed cursor reservations.
    #[must_use]
    pub const fn parent(&self) -> &P {
        &self.details.parent
    }
    /// Complete original served/component population, when count arithmetic succeeded.
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
    pub const fn refusal(&self) -> &DraftingProgramPocketWorkRefusal<'s, 'w, 'a, I, H, S> {
        &self.details.refusal
    }
    /// Recover the exact original prior proof without refunding reserved work.
    #[must_use]
    pub fn into_parent(self) -> P {
        self.details.parent
    }
}
impl<P, I: ?Sized, H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramPocketWorkError<'_, '_, '_, P, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramPocketWorkError")
            .field("refusal", &self.details.refusal)
            .finish_non_exhaustive()
    }
}
impl<P, I: ?Sized, H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized> fmt::Display
    for DraftingProgramPocketWorkError<'_, '_, '_, P, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "drafting Pocket current members: {:?}",
            self.details.refusal
        )
    }
}
impl<P, I: ?Sized, H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized>
    std::error::Error for DraftingProgramPocketWorkError<'_, '_, '_, P, I, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.details.refusal {
            DraftingProgramPocketWorkRefusal::Resources(c) => Some(c),
            DraftingProgramPocketWorkRefusal::Reservation { cause, .. } => Some(cause),
            DraftingProgramPocketWorkRefusal::Work(c) => std::error::Error::source(c),
            DraftingProgramPocketWorkRefusal::Component { cause, .. } => {
                std::error::Error::source(cause)
            }
            DraftingProgramPocketWorkRefusal::PopulationOverflow => None,
        }
    }
}
/// Complete original current-member class or same entire prior proof and first original cause.
pub type DraftingProgramPocketWorkResult<'s, 'w, 'a, P, I, H, S> = Result<
    BoundDraftingProgramPocketWork<'s, 'a, P, H, S>,
    DraftingProgramPocketWorkError<'s, 'w, 'a, P, I, H, S>,
>;

fn request(field: DraftingGarmentSemanticField<'_>) -> Option<PocketPieceRef> {
    use DraftingGarmentSemanticField as F;
    match field {
        F::PocketServed(target)
        | F::PocketComponent {
            index: _,
            reference: target,
        } => Some(*target),
        F::PocketPosition(_)
        | F::PocketOrientation(_)
        | F::PocketOpening(_)
        | F::ClosureLength(_)
        | F::ClosureSize { role: _, source: _ }
        | F::ClosureButtonhole(_)
        | F::ClosureInstance {
            index: _,
            instance: _,
        }
        | F::ClosureUnsupportedFly(_) => None,
    }
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
        Vec<DraftingProgramPocketWork<'s, 'a, H, S>>,
        DraftingProgramPocketWorkRefusal<'s, 'w, 'a, I, H, S>,
    >,
}
impl<'s, 'w, 'a, I: ?Sized, H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized>
    Outcome<'s, 'w, 'a, I, H, S>
{
    pub(super) fn finish<P>(
        self,
        parent: P,
    ) -> DraftingProgramPocketWorkResult<'s, 'w, 'a, P, I, H, S> {
        match self.result {
            Ok(records) => Ok(BoundDraftingProgramPocketWork {
                parent,
                demand: self.demand,
                records,
            }),
            Err(refusal) => Err(DraftingProgramPocketWorkError {
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
    use DraftingProgramPocketWorkRefusal as R;
    let mut count = None;
    let mut demand = None;
    let result = (|| {
        let source = garment.source().garment;
        let n = source
            .pocket_closure_dependencies()
            .filter_map(|o| request(o.field()))
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
        let limits = DraftingComponentBindingLimits::new(
            content,
            cursor.work().shape().per_pass_matching_bound(),
        );
        for occurrence in source.pocket_closure_dependencies() {
            let Some(target) = request(occurrence.field()) else {
                continue;
            };
            let work = garment
                .prepare_component_binding(target, limits)
                .map_err(|cause| R::Component { occurrence, cause })?;
            records.push(DraftingProgramPocketWork { occurrence, work });
        }
        Ok(records)
    })();
    Outcome {
        count,
        demand,
        result,
    }
}

pub(super) mod sources;
pub use sources::{
    BoundDraftingProgramPocketSources, DraftingProgramPocketSource,
    DraftingProgramPocketSourceError, DraftingProgramPocketSourceRefusal,
    DraftingProgramPocketSourcesResult,
};
