//! Complete original Pocket sources borrow their stable actual current-member work parents.
use super::DraftingProgramPocketWork;
use crate::{
    construction::{
        BoundDraftingComponentSource, CheckedDraftingProgramDirectSelections,
        DraftingComponentProvider, DraftingComponentSourceError, DraftingProgramSemanticWorkError,
    },
    design::{
        DesignReferenceHost, DesignResourceDemand, DesignResourceLimitError, DesignResourceMeter,
        DesignResourceSink, DesignResourceSource,
    },
};
use core::fmt;
use std::collections::TryReserveError;
/// One complete original field/work parent and its associated actual source/header/creator loans.
pub struct DraftingProgramPocketSource<
    'x,
    's,
    'a,
    H: DesignReferenceHost,
    S: DraftingComponentProvider<H> + ?Sized,
> {
    work_index: usize,
    work: &'x DraftingProgramPocketWork<'s, 'a, H, S>,
    source: BoundDraftingComponentSource<'x, 's, 'a, H, S>,
}
impl<'x, 's, 'a, H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized>
    DraftingProgramPocketSource<'x, 's, 'a, H, S>
{
    /// Original zero-based requiring field in the complete stable work slice.
    #[must_use]
    pub const fn work_index(&self) -> usize {
        self.work_index
    }
    /// Exact original requiring field/current-member work parent.
    #[must_use]
    pub const fn work(&self) -> &'x DraftingProgramPocketWork<'s, 'a, H, S> {
        self.work
    }
    /// Original complete source association,including import/recipe/frame/snapshot/creator loans.
    #[must_use]
    pub const fn source(&self) -> &BoundDraftingComponentSource<'x, 's, 'a, H, S> {
        &self.source
    }
}
impl<H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramPocketSource<'_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramPocketSource")
            .field("work_index", &self.work_index)
            .finish_non_exhaustive()
    }
}
/// Complete original Pocket source class,borrowing disjoint original input/cursor and work fields.
/// No source body is copied and no self-referential result owner is created.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignReferenceHost};
/// fn alter<P,H:DesignReferenceHost,S:DraftingComponentProvider<H>>(p:&mut BoundDraftingProgramPocketSources<'_,'_,'_,P,H,S>) {p.records.clear();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignReferenceHost};
/// fn duplicate<P,H:DesignReferenceHost,S:DraftingComponentProvider<H>>(p:BoundDraftingProgramPocketSources<'_,'_,'_,P,H,S>) {let _=p.clone();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignReferenceHost};
/// fn escape<'x,'s,'a,P,H:DesignReferenceHost,S:DraftingComponentProvider<H>>(p:BoundDraftingProgramPocketSources<'x,'s,'a,P,H,S>)->BoundDraftingProgramPocketSources<'static,'s,'a,P,H,S> {p}
/// ```
pub struct BoundDraftingProgramPocketSources<
    'x,
    's,
    'a,
    P,
    H: DesignReferenceHost,
    S: DraftingComponentProvider<H> + ?Sized,
> {
    parent: &'x mut P,
    work: &'x [DraftingProgramPocketWork<'s, 'a, H, S>],
    demand: Option<DesignResourceDemand>,
    records: Vec<DraftingProgramPocketSource<'x, 's, 'a, H, S>>,
}
impl<'x, 's, 'a, P, H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized>
    BoundDraftingProgramPocketSources<'x, 's, 'a, P, H, S>
{
    /// Complete original input proof and same unique cursor with all prior/new reservations.
    #[must_use]
    pub const fn parent(&self) -> &P {
        self.parent
    }
    /// Entire actual stable current-member parent slice,including repeated targets.
    #[must_use]
    pub const fn work(&self) -> &'x [DraftingProgramPocketWork<'s, 'a, H, S>] {
        self.work
    }
    /// Fresh complete aggregate demand for N>0;None for the actually empty work class.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.demand
    }
    /// All original requiring fields and original complete source associations in work order.
    #[must_use]
    pub fn records(&self) -> &[DraftingProgramPocketSource<'x, 's, 'a, H, S>] {
        &self.records
    }
    /// Zero for N0;otherwise N+1 complete aggregate/source pass envelopes.
    #[must_use]
    pub fn child_pass_count(&self) -> usize {
        if self.records.is_empty() {
            0
        } else {
            self.records.len() + 1
        }
    }
}
impl<P, H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized> fmt::Debug
    for BoundDraftingProgramPocketSources<'_, '_, '_, P, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundDraftingProgramPocketSources")
            .field("records", &self.records.len())
            .finish_non_exhaustive()
    }
}
/// First complete original Pocket-source-stage refusal,with no successful prefix.
pub enum DraftingProgramPocketSourceRefusal<
    'x,
    's,
    'w,
    'a,
    I: ?Sized,
    H: DesignReferenceHost,
    S: DraftingComponentProvider<H> + ?Sized,
> {
    /// Complete N+1 pass count cannot fit usize.
    PopulationOverflow,
    /// Original unique cursor refuses the whole stage atomically.
    Work(DraftingProgramSemanticWorkError<'w, I>),
    /// First fresh complete aggregate content cause,before result reservation or source query.
    Resources(DesignResourceLimitError),
    /// Complete original result population could not be reserved.
    Reservation {
        /// Complete requiring field population.
        records: usize,
        /// Original allocator cause.
        cause: TryReserveError,
    },
    /// First actual original work-bound source refusal,including unchanged returned witnesses.
    Source {
        /// Original zero-based requiring field in the retained complete work slice.
        work_index: usize,
        /// Original complete current work,returned source and typed cause.
        cause: DraftingComponentSourceError<'x, 's, 'a, H, S>,
    },
}
impl<I: ?Sized, H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramPocketSourceRefusal<'_, '_, '_, '_, I, H, S>
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
            Self::Source { work_index, cause } => f
                .debug_struct("Source")
                .field("work_index", work_index)
                .field("cause", cause)
                .finish(),
        }
    }
}
/// Entire original borrowed input/work owners and first refusal;private accepted prefixes drop.
pub struct DraftingProgramPocketSourceError<
    'x,
    's,
    'w,
    'a,
    P,
    I: ?Sized,
    H: DesignReferenceHost,
    S: DraftingComponentProvider<H> + ?Sized,
> {
    details: Box<Failure<'x, 's, 'w, 'a, P, I, H, S>>,
}
struct Failure<
    'x,
    's,
    'w,
    'a,
    P,
    I: ?Sized,
    H: DesignReferenceHost,
    S: DraftingComponentProvider<H> + ?Sized,
> {
    parent: &'x mut P,
    work: &'x [DraftingProgramPocketWork<'s, 'a, H, S>],
    demand: Option<DesignResourceDemand>,
    refusal: DraftingProgramPocketSourceRefusal<'x, 's, 'w, 'a, I, H, S>,
}
impl<
        'x,
        's,
        'w,
        'a,
        P,
        I: ?Sized,
        H: DesignReferenceHost,
        S: DraftingComponentProvider<H> + ?Sized,
    > DraftingProgramPocketSourceError<'x, 's, 'w, 'a, P, I, H, S>
{
    /// Complete original input proof and unchanged or newly consumed work cursor.
    #[must_use]
    pub const fn parent(&self) -> &P {
        self.details.parent
    }
    /// All actual original stable current-member work parents,including the refused field.
    #[must_use]
    pub const fn work(&self) -> &'x [DraftingProgramPocketWork<'s, 'a, H, S>] {
        self.details.work
    }
    /// Complete original requiring-field count,without target deduplication.
    #[must_use]
    pub fn source_count(&self) -> usize {
        self.details.work.len()
    }
    /// Completed fresh aggregate demand only after its entire content walk passed.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.details.demand
    }
    /// First original work/resource/allocator/source cause and actual child witnesses.
    #[must_use]
    pub const fn refusal(&self) -> &DraftingProgramPocketSourceRefusal<'x, 's, 'w, 'a, I, H, S> {
        &self.details.refusal
    }
}
impl<P, I: ?Sized, H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramPocketSourceError<'_, '_, '_, '_, P, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramPocketSourceError")
            .field("refusal", &self.details.refusal)
            .finish_non_exhaustive()
    }
}
impl<P, I: ?Sized, H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized> fmt::Display
    for DraftingProgramPocketSourceError<'_, '_, '_, '_, P, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "drafting Pocket sources: {:?}", self.details.refusal)
    }
}
impl<P, I: ?Sized, H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized>
    std::error::Error for DraftingProgramPocketSourceError<'_, '_, '_, '_, P, I, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.details.refusal {
            DraftingProgramPocketSourceRefusal::Resources(c) => Some(c),
            DraftingProgramPocketSourceRefusal::Reservation { cause, .. } => Some(cause),
            DraftingProgramPocketSourceRefusal::Work(c) => std::error::Error::source(c),
            DraftingProgramPocketSourceRefusal::Source { cause, .. } => {
                std::error::Error::source(cause)
            }
            DraftingProgramPocketSourceRefusal::PopulationOverflow => None,
        }
    }
}
/// Complete original Pocket sources or entire original borrowed input/work owners and first cause.
pub type DraftingProgramPocketSourcesResult<'x, 's, 'w, 'a, P, I, H, S> = Result<
    BoundDraftingProgramPocketSources<'x, 's, 'a, P, H, S>,
    DraftingProgramPocketSourceError<'x, 's, 'w, 'a, P, I, H, S>,
>;

pub(in crate::construction::program::dependencies) struct Outcome<
    'x,
    's,
    'w,
    'a,
    I: ?Sized,
    H: DesignReferenceHost,
    S: DraftingComponentProvider<H> + ?Sized,
> {
    demand: Option<DesignResourceDemand>,
    result: Result<
        Vec<DraftingProgramPocketSource<'x, 's, 'a, H, S>>,
        DraftingProgramPocketSourceRefusal<'x, 's, 'w, 'a, I, H, S>,
    >,
}
impl<
        'x,
        's,
        'w,
        'a,
        I: ?Sized,
        H: DesignReferenceHost,
        S: DraftingComponentProvider<H> + ?Sized,
    > Outcome<'x, 's, 'w, 'a, I, H, S>
{
    pub(in crate::construction::program::dependencies) fn finish<P>(
        self,
        parent: &'x mut P,
        work: &'x [DraftingProgramPocketWork<'s, 'a, H, S>],
    ) -> DraftingProgramPocketSourcesResult<'x, 's, 'w, 'a, P, I, H, S> {
        match self.result {
            Ok(records) => Ok(BoundDraftingProgramPocketSources {
                parent,
                work,
                demand: self.demand,
                records,
            }),
            Err(refusal) => Err(DraftingProgramPocketSourceError {
                details: Box::new(Failure {
                    parent,
                    work,
                    demand: self.demand,
                    refusal,
                }),
            }),
        }
    }
}
pub(in crate::construction::program::dependencies) fn prepare<
    'x,
    's,
    'w,
    'a,
    I: ?Sized,
    H: DesignReferenceHost + DesignResourceSource,
    S: DraftingComponentProvider<H> + ?Sized,
>(
    cursor: &mut CheckedDraftingProgramDirectSelections<'w, I>,
    work: &'x [DraftingProgramPocketWork<'s, 'a, H, S>],
    charge: impl FnOnce(&mut dyn DesignResourceSink) -> Result<(), DesignResourceLimitError>,
) -> Outcome<'x, 's, 'w, 'a, I, H, S> {
    use DraftingProgramPocketSourceRefusal as R;
    let mut demand = None;
    let result = (|| {
        let n = work.len();
        let passes = if n == 0 {
            0
        } else {
            n.checked_add(1).ok_or(R::PopulationOverflow)?
        };
        cursor.reserve_child_passes(passes).map_err(R::Work)?;
        if n == 0 {
            return Ok(Vec::new());
        }
        let mut meter = DesignResourceMeter::new(cursor.limits().population().content());
        let charged = charge(&mut meter);
        let complete = meter.finish().map_err(R::Resources)?;
        charged.map_err(R::Resources)?;
        demand = Some(complete);
        let mut records = Vec::new();
        records
            .try_reserve_exact(n)
            .map_err(|cause| R::Reservation { records: n, cause })?;
        for (work_index, work) in work.iter().enumerate() {
            let source = work
                .work()
                .bind_source()
                .map_err(|cause| R::Source { work_index, cause })?;
            records.push(DraftingProgramPocketSource {
                work_index,
                work,
                source,
            });
        }
        Ok(records)
    })();
    Outcome { demand, result }
}
