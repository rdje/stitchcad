//! Finite selected aggregate child-work capacity after fresh complete original populations.
use super::{
    AdmittedDraftingProgramSemanticPopulation, DraftingProgramSemanticInputs,
    DraftingProgramSemanticPopulationError, DraftingProgramSemanticPopulationLimits,
    DraftingProgramSemanticPopulationShape,
};
use crate::{
    construction::{DraftingGarmentProvider, DraftingInitialInputProvider},
    design::{DesignReferenceHost, DesignResourceDemand, DesignResourceSource},
};
use core::fmt;

/// Explicit cumulative child-pass content allowances. These u128 totals count repeated logical
/// walks, independently of the platform-sized population/capacity of one actual source snapshot.
/// No default or runtime/allocator-byte estimate is provided; zero is an explicit empty budget.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DraftingProgramSemanticChildContentLimits {
    entries: u128,
    text: u128,
    bits: u128,
}
impl DraftingProgramSemanticChildContentLimits {
    /// Select cumulative collection-entry,text-byte and counter-bit allowances explicitly.
    #[must_use]
    pub const fn new(collection_entries: u128, text_bytes: u128, counter_bits: u128) -> Self {
        Self {
            entries: collection_entries,
            text: text_bytes,
            bits: counter_bits,
        }
    }
    /// Maximum cumulative child-pass collection entries.
    #[must_use]
    pub const fn collection_entries(self) -> u128 {
        self.entries
    }
    /// Maximum cumulative child-pass logical text bytes, not string-comparison CPU.
    #[must_use]
    pub const fn text_bytes(self) -> u128 {
        self.text
    }
    /// Maximum cumulative child-pass represented counter bits.
    #[must_use]
    pub const fn counter_bits(self) -> u128 {
        self.bits
    }
}
/// Explicit fresh population policy and finite selected child-pass/matching/result capacity.
/// Pass capacity is authored policy,not a claim about how many child stages a source will need.
/// ```compile_fail
/// let limits:sc_core::construction::DraftingProgramSemanticWorkLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DraftingProgramSemanticWorkLimits {
    population: DraftingProgramSemanticPopulationLimits,
    passes: usize,
    content: DraftingProgramSemanticChildContentLimits,
    inspections: u128,
    results: usize,
}
impl DraftingProgramSemanticWorkLimits {
    /// Select fresh population policy,P child passes,their cumulative content/matching allowance,
    /// and total A+P direct/child-result record allowance. Nothing implicitly defaults P.
    #[must_use]
    pub const fn new(
        population: DraftingProgramSemanticPopulationLimits,
        child_passes: usize,
        child_content: DraftingProgramSemanticChildContentLimits,
        child_matching_inspections: u128,
        result_records: usize,
    ) -> Self {
        Self {
            population,
            passes: child_passes,
            content: child_content,
            inspections: child_matching_inspections,
            results: result_records,
        }
    }
    /// Exact nested original whole-content/direct-population policy.
    #[must_use]
    pub const fn population(&self) -> &DraftingProgramSemanticPopulationLimits {
        &self.population
    }
    /// Selected maximum child passes,not measured or automatically sufficient source work.
    #[must_use]
    pub const fn child_passes(self) -> usize {
        self.passes
    }
    /// Selected cumulative content policy for every permitted child pass.
    #[must_use]
    pub const fn child_content(self) -> DraftingProgramSemanticChildContentLimits {
        self.content
    }
    /// Selected cumulative logical child matching inspections, excluding string CPU.
    #[must_use]
    pub const fn child_matching_inspections(self) -> u128 {
        self.inspections
    }
    /// Selected total direct-association and child-result logical slot allowance.
    #[must_use]
    pub const fn result_records(self) -> usize {
        self.results
    }
}
/// Complete checked upper envelopes for the selected finite P child passes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DraftingProgramSemanticWorkShape {
    passes: usize,
    entries: u128,
    text: u128,
    bits: u128,
    per_pass_inspections: usize,
    inspections: u128,
    results: usize,
}
impl DraftingProgramSemanticWorkShape {
    /// Selected P child-pass capacity; zero permits no child pass.
    #[must_use]
    pub const fn child_pass_capacity(self) -> usize {
        self.passes
    }
    /// Checked E*P cumulative collection-entry upper bound.
    #[must_use]
    pub const fn child_collection_entry_bound(self) -> u128 {
        self.entries
    }
    /// Checked T*P cumulative logical text-byte upper bound.
    #[must_use]
    pub const fn child_text_byte_bound(self) -> u128 {
        self.text
    }
    /// Checked B*P cumulative represented-counter-bit upper bound.
    #[must_use]
    pub const fn child_counter_bit_bound(self) -> u128 {
        self.bits
    }
    /// Existing child (E+8)^2 matching envelope;zero when no pass is permitted.
    #[must_use]
    pub const fn per_pass_matching_bound(self) -> usize {
        self.per_pass_inspections
    }
    /// Checked P*(E+8)^2 cumulative logical matching bound.
    #[must_use]
    pub const fn child_matching_bound(self) -> u128 {
        self.inspections
    }
    /// Checked A+P direct-association and child-result logical slots,not allocator capacity.
    #[must_use]
    pub const fn result_record_bound(self) -> usize {
        self.results
    }
}
/// First failed checked envelope component,without saturation or formula diagnostics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingProgramSemanticWorkArithmetic {
    /// A platform-sized source/pass population does not fit the aggregate u128 domain.
    AggregateWidth,
    /// Child collection-entry multiplication overflow.
    ChildCollectionEntries,
    /// Child logical text-byte multiplication overflow.
    ChildTextBytes,
    /// Child counter-bit multiplication overflow.
    ChildCounterBits,
    /// Existing per-child E+8 or square does not fit usize.
    PerPassMatching,
    /// Aggregate child matching multiplication overflow.
    ChildMatching,
    /// Direct association slots plus possible child results do not fit usize.
    ResultRecords,
}
/// First original population or aggregate work refusal. No source acceptance is exposed.
pub enum DraftingProgramSemanticWorkRefusal<'w, I: ?Sized> {
    /// Original typed whole-content/direct-population refusal and every original witness.
    Population(DraftingProgramSemanticPopulationError<'w, I>),
    /// First checked arithmetic component.
    Arithmetic(DraftingProgramSemanticWorkArithmetic),
    /// Complete permitted child collection-entry envelope exceeds policy.
    ChildCollectionEntries {
        /// Selected maximum.
        allowed: u128,
        /// Complete selected-pass upper bound.
        required: u128,
    },
    /// Complete permitted child text-byte envelope exceeds policy.
    ChildTextBytes {
        /// Selected maximum.
        allowed: u128,
        /// Complete selected-pass upper bound.
        required: u128,
    },
    /// Complete permitted child counter-bit envelope exceeds policy.
    ChildCounterBits {
        /// Selected maximum.
        allowed: u128,
        /// Complete selected-pass upper bound.
        required: u128,
    },
    /// Complete permitted child matching envelope exceeds policy.
    ChildMatching {
        /// Selected maximum.
        allowed: u128,
        /// Complete selected-pass upper bound.
        required: u128,
    },
    /// Complete A+P record envelope exceeds policy.
    ResultRecords {
        /// Selected maximum.
        allowed: usize,
        /// Complete direct/child-result bound.
        required: usize,
    },
    /// A stage requests more passes than remain;the cursor is unchanged on refusal.
    ChildPasses {
        /// Original selected capacity.
        allowed: usize,
        /// Passes already reserved before earlier stages.
        reserved: usize,
        /// Complete requested reservation before this stage may run.
        requested: usize,
    },
}
impl<I: ?Sized> fmt::Debug for DraftingProgramSemanticWorkRefusal<'_, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Population(cause) => f.debug_tuple("Population").field(cause).finish(),
            Self::Arithmetic(cause) => f.debug_tuple("Arithmetic").field(cause).finish(),
            Self::ChildCollectionEntries { allowed, required } => f
                .debug_struct("ChildCollectionEntries")
                .field("allowed", allowed)
                .field("required", required)
                .finish(),
            Self::ChildTextBytes { allowed, required } => f
                .debug_struct("ChildTextBytes")
                .field("allowed", allowed)
                .field("required", required)
                .finish(),
            Self::ChildCounterBits { allowed, required } => f
                .debug_struct("ChildCounterBits")
                .field("allowed", allowed)
                .field("required", required)
                .finish(),
            Self::ChildMatching { allowed, required } => f
                .debug_struct("ChildMatching")
                .field("allowed", allowed)
                .field("required", required)
                .finish(),
            Self::ResultRecords { allowed, required } => f
                .debug_struct("ResultRecords")
                .field("allowed", allowed)
                .field("required", required)
                .finish(),
            Self::ChildPasses {
                allowed,
                reserved,
                requested,
            } => f
                .debug_struct("ChildPasses")
                .field("allowed", allowed)
                .field("reserved", reserved)
                .field("requested", requested)
                .finish(),
        }
    }
}
/// Exact original joined owner/policy,completed-stage metadata and original first typed cause.
/// A boxed refused diagnostic creates no accepted source/prefix or result vector.
pub struct DraftingProgramSemanticWorkError<'w, I: ?Sized> {
    details: Box<Failure<'w, I>>,
}
struct Failure<'w, I: ?Sized> {
    inputs: &'w I,
    limits: &'w DraftingProgramSemanticWorkLimits,
    demand: Option<DesignResourceDemand>,
    population: Option<DraftingProgramSemanticPopulationShape>,
    shape: Option<DraftingProgramSemanticWorkShape>,
    refusal: DraftingProgramSemanticWorkRefusal<'w, I>,
}
impl<'w, I: ?Sized> DraftingProgramSemanticWorkError<'w, I> {
    fn new(
        inputs: &'w I,
        limits: &'w DraftingProgramSemanticWorkLimits,
        demand: Option<DesignResourceDemand>,
        population: Option<DraftingProgramSemanticPopulationShape>,
        shape: Option<DraftingProgramSemanticWorkShape>,
        refusal: DraftingProgramSemanticWorkRefusal<'w, I>,
    ) -> Self {
        Self {
            details: Box::new(Failure {
                inputs,
                limits,
                demand,
                population,
                shape,
                refusal,
            }),
        }
    }
    /// Original exact joined owner and all its actual source/proof/request loans.
    #[must_use]
    pub const fn inputs(&self) -> &'w I {
        self.details.inputs
    }
    /// Exact original complete selected policy,including nested population allowance.
    #[must_use]
    pub const fn limits(&self) -> &'w DraftingProgramSemanticWorkLimits {
        self.details.limits
    }
    /// Whole original fresh content only after that entire stage passed.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.details.demand
    }
    /// Complete original population shape only after its arithmetic passed.
    #[must_use]
    pub const fn population_shape(&self) -> Option<DraftingProgramSemanticPopulationShape> {
        self.details.population
    }
    /// Complete child-work shape only after its arithmetic passed;not accepted quota credit.
    #[must_use]
    pub const fn shape(&self) -> Option<DraftingProgramSemanticWorkShape> {
        self.details.shape
    }
    /// Original first typed cause. Borrowed population causes stay inspectable here.
    #[must_use]
    pub const fn refusal(&self) -> &DraftingProgramSemanticWorkRefusal<'w, I> {
        &self.details.refusal
    }
}
impl<I: ?Sized> fmt::Debug for DraftingProgramSemanticWorkError<'_, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramSemanticWorkError")
            .field("refusal", &self.details.refusal)
            .finish_non_exhaustive()
    }
}
impl<I: ?Sized> fmt::Display for DraftingProgramSemanticWorkError<'_, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "drafting semantic work: {:?}", self.details.refusal)
    }
}
impl<I: ?Sized> std::error::Error for DraftingProgramSemanticWorkError<'_, I> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.details.refusal {
            DraftingProgramSemanticWorkRefusal::Population(cause) => {
                std::error::Error::source(cause)
            }
            _ => None,
        }
    }
}
/// Fresh complete populations and admitted finite selected child-work capacity. This budget
/// approves no actual source,coverage,nested geometry,value,physical result or complete cohort.
/// A source may require more stages than P;the consuming cursor must refuse before those stages.
/// ```compile_fail,E0616
/// fn alter<I:?Sized>(proof:&mut sc_core::construction::AdmittedDraftingProgramSemanticWork<'_,I>) {proof.shape=todo!();}
/// ```
/// ```compile_fail
/// use sc_core::construction::AdmittedDraftingProgramSemanticWork;
/// fn escape<'w,I:?Sized>(proof:AdmittedDraftingProgramSemanticWork<'w,I>)
///     ->AdmittedDraftingProgramSemanticWork<'static,I> {proof}
/// ```
pub struct AdmittedDraftingProgramSemanticWork<'w, I: ?Sized> {
    population: AdmittedDraftingProgramSemanticPopulation<'w, I>,
    limits: &'w DraftingProgramSemanticWorkLimits,
    shape: DraftingProgramSemanticWorkShape,
}
impl<'w, I: ?Sized> AdmittedDraftingProgramSemanticWork<'w, I> {
    /// Exact original joined input owner.
    #[must_use]
    pub const fn inputs(&self) -> &'w I {
        self.population.inputs()
    }
    /// Exact original complete selected policy.
    #[must_use]
    pub const fn limits(&self) -> &'w DraftingProgramSemanticWorkLimits {
        self.limits
    }
    /// Fresh complete original demand;never a caller-seeded or earlier proof total.
    #[must_use]
    pub const fn demand(&self) -> DesignResourceDemand {
        self.population.demand()
    }
    /// Fresh original direct consumer/request shape.
    #[must_use]
    pub const fn population_shape(&self) -> DraftingProgramSemanticPopulationShape {
        self.population.shape()
    }
    /// Complete checked finite child-work and record envelopes.
    #[must_use]
    pub const fn shape(&self) -> DraftingProgramSemanticWorkShape {
        self.shape
    }
    /// Consume this unique fresh plan into one nonresettable reservation cursor. Reusing an old
    /// plan cannot create another cursor;another attempt must freshly admit the original owner.
    #[must_use]
    pub fn into_budget(self) -> DraftingProgramSemanticWorkBudget<'w, I> {
        let remaining = self.shape.child_pass_capacity();
        DraftingProgramSemanticWorkBudget {
            work: self,
            remaining,
        }
    }
}
impl<I: ?Sized> fmt::Debug for AdmittedDraftingProgramSemanticWork<'_, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AdmittedDraftingProgramSemanticWork")
            .field("shape", &self.shape)
            .finish_non_exhaustive()
    }
}
/// Unique consuming child-pass reservation cursor tied to its original fresh scope and policy.
/// It executes no factory or query. Callers reserve every stage's complete passes before running
/// that stage;one pass permits at most one whole-demand content walk and one matching envelope,
/// with at most one additional child-result slot. Multi-walk stages reserve multiple passes.
/// No Clone,reset,mutable-policy access or successful source-prefix API is provided.
/// ```compile_fail
/// fn duplicate<I:?Sized>(budget:sc_core::construction::DraftingProgramSemanticWorkBudget<'_,I>) {let _=budget.clone();}
/// ```
pub struct DraftingProgramSemanticWorkBudget<'w, I: ?Sized> {
    work: AdmittedDraftingProgramSemanticWork<'w, I>,
    remaining: usize,
}
impl<'w, I: ?Sized> DraftingProgramSemanticWorkBudget<'w, I> {
    /// Original unique admitted plan,with actual owner,fresh data and selected policy.
    #[must_use]
    pub const fn work(&self) -> &AdmittedDraftingProgramSemanticWork<'w, I> {
        &self.work
    }
    /// Remaining child passes;these are policy reservations,not measured callback/CPU counts.
    #[must_use]
    pub const fn remaining_passes(&self) -> usize {
        self.remaining
    }
    /// Passes reserved before earlier child stages;zero-reservation calls leave this unchanged.
    #[must_use]
    pub const fn reserved_passes(&self) -> usize {
        self.work.shape.child_pass_capacity() - self.remaining
    }
    /// Atomically reserve a complete stage's passes before its content walks,matching,queries or
    /// child-result reservation. The stage's actual scope must remain within the original full
    /// demand and existing (E+8)^2 child contract. Reservation executes no source work or callback.
    /// # Errors
    /// Original complete requested pass count and original capacity/previous reservations. A
    /// refusal leaves the cursor unchanged and preserves its exact original owner/policy/shape.
    pub fn reserve_child_passes(
        &mut self,
        requested: usize,
    ) -> Result<(), DraftingProgramSemanticWorkError<'w, I>> {
        if requested > self.remaining {
            return Err(DraftingProgramSemanticWorkError::new(
                self.work.inputs(),
                self.work.limits(),
                Some(self.work.demand()),
                Some(self.work.population_shape()),
                Some(self.work.shape()),
                DraftingProgramSemanticWorkRefusal::ChildPasses {
                    allowed: self.work.shape.child_pass_capacity(),
                    reserved: self.reserved_passes(),
                    requested,
                },
            ));
        }
        self.remaining -= requested;
        Ok(())
    }
}
impl<I: ?Sized> fmt::Debug for DraftingProgramSemanticWorkBudget<'_, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramSemanticWorkBudget")
            .field("remaining", &self.remaining)
            .finish_non_exhaustive()
    }
}
fn envelope(
    entries: usize,
    text: usize,
    bits: u64,
    associations: usize,
    passes: usize,
) -> Result<DraftingProgramSemanticWorkShape, DraftingProgramSemanticWorkArithmetic> {
    use DraftingProgramSemanticWorkArithmetic as A;
    let p = u128::try_from(passes).map_err(|_| A::AggregateWidth)?;
    let entries128 = u128::try_from(entries).map_err(|_| A::AggregateWidth)?;
    let text128 = u128::try_from(text).map_err(|_| A::AggregateWidth)?;
    let entries_total = entries128.checked_mul(p).ok_or(A::ChildCollectionEntries)?;
    let text_total = text128.checked_mul(p).ok_or(A::ChildTextBytes)?;
    let bits_total = u128::from(bits).checked_mul(p).ok_or(A::ChildCounterBits)?;
    let per_pass_inspections = if passes == 0 {
        0
    } else {
        let padded = entries.checked_add(8).ok_or(A::PerPassMatching)?;
        padded.checked_mul(padded).ok_or(A::PerPassMatching)?
    };
    let inspections = u128::try_from(per_pass_inspections)
        .map_err(|_| A::AggregateWidth)?
        .checked_mul(p)
        .ok_or(A::ChildMatching)?;
    let results = associations.checked_add(passes).ok_or(A::ResultRecords)?;
    Ok(DraftingProgramSemanticWorkShape {
        passes,
        entries: entries_total,
        text: text_total,
        bits: bits_total,
        per_pass_inspections,
        inspections,
        results,
    })
}
impl<
        's,
        'g,
        'k,
        'j,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignReferenceHost + DesignResourceSource,
        S: DraftingInitialInputProvider<H> + DraftingGarmentProvider<H> + ?Sized,
    > DraftingProgramSemanticInputs<'s, 'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Freshly admit complete original content/populations,then derive checked bounds for every
    /// selected P child passes before child quotas or work. Each permitted pass covers at most
    /// one original whole-demand walk and the existing child (E+8)^2 matching envelope;stages that
    /// repeat admission reserve multiple passes before running. Aggregate counts use u128 while
    /// one pass's matching and A+P physical record slots remain platform-sized.
    ///
    /// Check child entries,text,bits,matching,then total result quota. P is explicit selected
    /// capacity,not predicted source need;zero permits no child stage. Source binders must use
    /// the unique consuming cursor and retain no accepted prefix on exhaustion. No source/member/
    /// value query,coverage scan,clone or successful result buffer occurs during this admission.
    /// Only the required original resource walk runs;old proof demands supply no credit.
    /// # Errors
    /// Original population/content cause,first complete arithmetic issue,then first child quota,
    /// retaining actual owner,selected full policy and fully completed stage data only.
    pub fn admit_semantic_work<'w>(
        &'w self,
        limits: &'w DraftingProgramSemanticWorkLimits,
    ) -> Result<
        AdmittedDraftingProgramSemanticWork<'w, Self>,
        DraftingProgramSemanticWorkError<'w, Self>,
    > {
        use DraftingProgramSemanticWorkRefusal as R;
        let population = match self.admit_consumer_population(limits.population()) {
            Ok(population) => population,
            Err(cause) => {
                return Err(DraftingProgramSemanticWorkError::new(
                    self,
                    limits,
                    cause.demand(),
                    cause.shape(),
                    None,
                    R::Population(cause),
                ))
            }
        };
        let demand = population.demand();
        let direct = population.shape();
        let shape = envelope(
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits(),
            direct.association_record_bound(),
            limits.child_passes(),
        )
        .map_err(|cause| {
            DraftingProgramSemanticWorkError::new(
                self,
                limits,
                Some(demand),
                Some(direct),
                None,
                R::Arithmetic(cause),
            )
        })?;
        let refuse = |refusal| {
            DraftingProgramSemanticWorkError::new(
                self,
                limits,
                Some(demand),
                Some(direct),
                Some(shape),
                refusal,
            )
        };
        if shape.entries > limits.content.entries {
            return Err(refuse(R::ChildCollectionEntries {
                allowed: limits.content.entries,
                required: shape.entries,
            }));
        }
        if shape.text > limits.content.text {
            return Err(refuse(R::ChildTextBytes {
                allowed: limits.content.text,
                required: shape.text,
            }));
        }
        if shape.bits > limits.content.bits {
            return Err(refuse(R::ChildCounterBits {
                allowed: limits.content.bits,
                required: shape.bits,
            }));
        }
        if shape.inspections > limits.inspections {
            return Err(refuse(R::ChildMatching {
                allowed: limits.inspections,
                required: shape.inspections,
            }));
        }
        if shape.results > limits.results {
            return Err(refuse(R::ResultRecords {
                allowed: limits.results,
                required: shape.results,
            }));
        }
        Ok(AdmittedDraftingProgramSemanticWork {
            population,
            limits,
            shape,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::{envelope, DraftingProgramSemanticWorkArithmetic as A};
    #[test]
    fn semantic_child_envelope_keeps_aggregate_width_zero_capacity_and_checked_boundaries() {
        let actual = envelope(5, 7, 3, 8, 2).map(|s| {
            (
                s.entries,
                s.text,
                s.bits,
                s.per_pass_inspections,
                s.inspections,
                s.results,
            )
        });
        assert_eq!(actual, Ok((10, 14, 6, 169, 338, 10)));
        let empty = envelope(usize::MAX, usize::MAX, u64::MAX, 8, 0).map(|s| {
            (
                s.entries,
                s.text,
                s.bits,
                s.per_pass_inspections,
                s.inspections,
                s.results,
            )
        });
        assert_eq!(empty, Ok((0, 0, 0, 0, 0, 8)));
        assert_eq!(
            envelope(1, 2, u64::MAX, 0, 2).map(|s| s.bits),
            Ok(36_893_488_147_419_103_230)
        );
        assert_eq!(envelope(usize::MAX, 0, 0, 0, 1), Err(A::PerPassMatching));
        assert_eq!(
            envelope(usize::MAX - 8, 0, 0, 0, 1),
            Err(A::PerPassMatching)
        );
        assert_eq!(envelope(0, 0, 0, usize::MAX, 1), Err(A::ResultRecords));
    }
}
