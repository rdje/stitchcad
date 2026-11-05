//! Complete original mixed formula-name canonical bindings, before geometry or execution.
use super::{DraftingProgramDependencyInputs, DraftingProgramNameDependency};
use crate::{
    construction::DraftingInitialInputProvider,
    design::DesignMetadataHost,
    recipe::{
        FormulaCanonicalInput, FormulaCanonicalInputAdapter, FormulaCanonicalInputError,
        FormulaCheckedSource, FormulaDeclarationSource, FormulaDerivedInput,
    },
};
use core::fmt;
use std::collections::TryReserveError;

/// Explicit additional occurrence and logical binding-record allowances, without defaults.
/// All original formula-name uses count,including tolerance headers and noncanonical metadata.
/// Direct native/semantic ID references have separate typed producers;no AST span is invented.
/// Records cover D occurrence slots plus C canonical associations, including their borrowed
/// Derived relation. Existing content, names, order and source/graph policies stay separate.
/// These are not allocator bytes, capacity, scratch, CPU or execution allowances.
/// ```compile_fail
/// let limits:sc_core::construction::DraftingProgramCanonicalLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DraftingProgramCanonicalLimits {
    occurrences: usize,
    retained_records: usize,
}
impl DraftingProgramCanonicalLimits {
    /// Select both complete allowances; zero permits an empty mixed dependency stream.
    #[must_use]
    pub const fn new(occurrences: usize, retained_records: usize) -> Self {
        Self {
            occurrences,
            retained_records,
        }
    }
    /// Maximum complete mixed dependency occurrences, without deduplication.
    #[must_use]
    pub const fn occurrences(self) -> usize {
        self.occurrences
    }
    /// Maximum complete additional D+C logical records.
    #[must_use]
    pub const fn retained_records(self) -> usize {
        self.retained_records
    }
}
/// Factory-derived complete original occurrence and canonical-association populations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DraftingProgramCanonicalShape {
    occurrences: usize,
    canonical: usize,
    retained_records: usize,
}
impl DraftingProgramCanonicalShape {
    /// Every original mixed use, including noncanonical declarations and repeated names.
    #[must_use]
    pub const fn occurrence_count(self) -> usize {
        self.occurrences
    }
    /// Every actual Input or LengthInput occurrence, without collapsing shared targets.
    #[must_use]
    pub const fn canonical_count(self) -> usize {
        self.canonical
    }
    /// Complete additional D+C logical records, not measured allocation bytes.
    #[must_use]
    pub const fn retained_record_bound(self) -> usize {
        self.retained_records
    }
}
/// Actual native Derived target and its exact selected native source proof.
/// Only the complete factory creates this relation; no caller-supplied source id is accepted.
#[derive(Clone, Copy)]
pub struct DraftingProgramDerivedBinding<'j> {
    input: FormulaDerivedInput<'j>,
    source: &'j FormulaCheckedSource<'j>,
}
impl<'j> DraftingProgramDerivedBinding<'j> {
    /// Original native state/target/source join, with the actual canonical declaration loan.
    #[must_use]
    pub const fn input(self) -> FormulaDerivedInput<'j> {
        self.input
    }
    /// Actual source proof in the selected complete source/acyclic owners.
    #[must_use]
    pub const fn source(self) -> &'j FormulaCheckedSource<'j> {
        self.source
    }
}
impl fmt::Debug for DraftingProgramDerivedBinding<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramDerivedBinding")
            .finish_non_exhaustive()
    }
}
/// Actual canonical inventory join and optional native Derived source relation, without values.
#[derive(Clone, Copy)]
pub struct DraftingProgramCanonicalBinding<'j> {
    input: FormulaCanonicalInput<'j>,
    derived: Option<DraftingProgramDerivedBinding<'j>>,
}
impl<'j> DraftingProgramCanonicalBinding<'j> {
    /// Existing native inventory/family/kind/Length-owner certificate; state is unchanged.
    #[must_use]
    pub const fn input(self) -> FormulaCanonicalInput<'j> {
        self.input
    }
    /// Actual Derived relation, or None for other canonical states, including Unknown.
    #[must_use]
    pub const fn derived(self) -> Option<DraftingProgramDerivedBinding<'j>> {
        self.derived
    }
}
impl fmt::Debug for DraftingProgramCanonicalBinding<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramCanonicalBinding")
            .field("derived", &self.derived.is_some())
            .finish_non_exhaustive()
    }
}
/// One original occurrence and its canonical association when the actual declaration has one.
/// None retains Recipe/Reserved/Point/Edge metadata; it grants no geometry source acceptance.
#[derive(Clone, Copy, Debug)]
pub struct DraftingProgramCanonicalOccurrence<'j> {
    occurrence: DraftingProgramNameDependency<'j, 'j>,
    canonical: Option<DraftingProgramCanonicalBinding<'j>>,
}
impl<'j> DraftingProgramCanonicalOccurrence<'j> {
    /// Actual original consumer/role/span/ordinal/declaration, including assertion headers.
    #[must_use]
    pub const fn occurrence(self) -> DraftingProgramNameDependency<'j, 'j> {
        self.occurrence
    }
    /// Actual Root join for Input/LengthInput, or None for noncanonical metadata.
    #[must_use]
    pub const fn canonical(self) -> Option<DraftingProgramCanonicalBinding<'j>> {
        self.canonical
    }
}
/// Original native canonical cause and actual consumer occurrence, kept together on refusal.
pub struct DraftingProgramCanonicalJoinFailure<'j> {
    index: usize,
    occurrence: DraftingProgramNameDependency<'j, 'j>,
    cause: FormulaCanonicalInputError<'j>,
}
impl<'j> DraftingProgramCanonicalJoinFailure<'j> {
    /// Zero-based original mixed dependency occurrence, not a statement ordinal.
    #[must_use]
    pub const fn index(&self) -> usize {
        self.index
    }
    /// Actual original consumer/role/span/declaration.
    #[must_use]
    pub const fn occurrence(&self) -> DraftingProgramNameDependency<'j, 'j> {
        self.occurrence
    }
    /// Original typed native family/inventory/kind/Length-owner refusal.
    #[must_use]
    pub const fn cause(&self) -> &FormulaCanonicalInputError<'j> {
        &self.cause
    }
}
impl fmt::Debug for DraftingProgramCanonicalJoinFailure<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramCanonicalJoinFailure")
            .field("index", &self.index)
            .field("occurrence", &self.occurrence)
            .finish_non_exhaustive()
    }
}
/// First complete population, quota, reservation or original native canonical refusal.
pub enum DraftingProgramCanonicalRefusal<'j> {
    /// Complete actual arithmetic cannot be represented; checked before any quota.
    PopulationOverflow,
    /// Complete actual mixed dependency occurrences exceed the selected allowance.
    Occurrences {
        /// Selected maximum.
        allowed: usize,
        /// Complete requirement.
        required: usize,
    },
    /// Complete actual D+C logical records exceed the selected allowance.
    RetainedRecords {
        /// Selected maximum.
        allowed: usize,
        /// Complete requirement.
        required: usize,
    },
    /// Original full result-vector reservation cause, retained inline.
    Allocation(TryReserveError),
    /// Original native inventory join cause and its exact original consumer occurrence.
    Canonical(Box<DraftingProgramCanonicalJoinFailure<'j>>),
}
impl fmt::Debug for DraftingProgramCanonicalRefusal<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PopulationOverflow => f.write_str("PopulationOverflow"),
            Self::Occurrences { allowed, required } => f
                .debug_struct("Occurrences")
                .field("allowed", allowed)
                .field("required", required)
                .finish(),
            Self::RetainedRecords { allowed, required } => f
                .debug_struct("RetainedRecords")
                .field("allowed", allowed)
                .field("required", required)
                .finish(),
            Self::Allocation(_) => f.write_str("Allocation"),
            Self::Canonical(cause) => f.debug_tuple("Canonical").field(cause).finish(),
        }
    }
}
fn is_canonical(source: FormulaDeclarationSource<'_>) -> bool {
    match source {
        FormulaDeclarationSource::Input { .. } | FormulaDeclarationSource::LengthInput { .. } => {
            true
        }
        FormulaDeclarationSource::Recipe { .. }
        | FormulaDeclarationSource::Reserved(_)
        | FormulaDeclarationSource::Point(_)
        | FormulaDeclarationSource::Edge(_) => false,
    }
}

/// Complete original selected dependency owners and policy on refusal; no accepted prefix.
pub struct DraftingProgramCanonicalError<
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
    inputs: &'k DraftingProgramDependencyInputs<'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
    limits: &'k DraftingProgramCanonicalLimits,
    refusal: DraftingProgramCanonicalRefusal<'j>,
}
impl<
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
    > DraftingProgramCanonicalError<'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Actual selected source/order/graph owners and all original input policies.
    #[must_use]
    pub const fn inputs(
        &self,
    ) -> &'k DraftingProgramDependencyInputs<'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S> {
        self.inputs
    }
    /// Actual selected complete canonical binding policy.
    #[must_use]
    pub const fn limits(&self) -> &'k DraftingProgramCanonicalLimits {
        self.limits
    }
    /// Original first complete-work, reservation or canonical cause.
    #[must_use]
    pub const fn refusal(&self) -> &DraftingProgramCanonicalRefusal<'j> {
        &self.refusal
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramCanonicalError<'_, '_, '_, '_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramCanonicalError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> fmt::Display
    for DraftingProgramCanonicalError<'_, '_, '_, '_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "complete drafting canonical binding: {:?}", self.refusal)
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> std::error::Error
    for DraftingProgramCanonicalError<'_, '_, '_, '_, '_, '_, '_, '_, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.refusal {
            DraftingProgramCanonicalRefusal::Allocation(error) => Some(error),
            DraftingProgramCanonicalRefusal::PopulationOverflow
            | DraftingProgramCanonicalRefusal::Occurrences { .. }
            | DraftingProgramCanonicalRefusal::RetainedRecords { .. }
            | DraftingProgramCanonicalRefusal::Canonical(_) => None,
        }
    }
}

/// Complete original mixed canonical associations under exact source/order/acyclic owners.
/// No public constructor or mutable slots permit an accepted prefix or replacement declaration.
/// This grants no geometry/semantic acceptance, currentness, values or programme execution.
/// ```compile_fail,E0616
/// use sc_core::{construction::{BoundDraftingProgramCanonicalInputs,DraftingInitialInputProvider},design::DesignMetadataHost};
/// fn alter<H:DesignMetadataHost,S:DraftingInitialInputProvider<H>>(
///     proof:&mut BoundDraftingProgramCanonicalInputs<'_, '_, '_, '_, '_, '_, '_, '_,H,S>) {proof.occurrences=vec![];}
/// ```
/// ```compile_fail
/// use sc_core::{construction::{BoundDraftingProgramCanonicalInputs,DraftingInitialInputProvider},design::DesignMetadataHost};
/// fn escape<'k,'j,'i,'r,'b,'n,'p,'a,H:DesignMetadataHost,S:DraftingInitialInputProvider<H>>(
///     proof:BoundDraftingProgramCanonicalInputs<'k,'j,'i,'r,'b,'n,'p,'a,H,S>)
///     ->BoundDraftingProgramCanonicalInputs<'static,'j,'i,'r,'b,'n,'p,'a,H,S> {proof}
/// ```
pub struct BoundDraftingProgramCanonicalInputs<
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
    inputs: &'k DraftingProgramDependencyInputs<'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
    limits: &'k DraftingProgramCanonicalLimits,
    shape: DraftingProgramCanonicalShape,
    occurrences: Vec<DraftingProgramCanonicalOccurrence<'j>>,
}
impl<
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
    > BoundDraftingProgramCanonicalInputs<'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Exact selected complete dependency inputs, not equal-looking rebuilt proofs.
    #[must_use]
    pub const fn inputs(
        &self,
    ) -> &'k DraftingProgramDependencyInputs<'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S> {
        self.inputs
    }
    /// Original complete additional canonical binding policy.
    #[must_use]
    pub const fn limits(&self) -> &'k DraftingProgramCanonicalLimits {
        self.limits
    }
    /// Actual complete original occurrence and canonical populations.
    #[must_use]
    pub const fn shape(&self) -> DraftingProgramCanonicalShape {
        self.shape
    }
    /// Complete original stream order, with one slot per original use including noncanonical ones.
    #[must_use]
    pub fn occurrences(&self) -> &[DraftingProgramCanonicalOccurrence<'j>] {
        &self.occurrences
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> fmt::Debug
    for BoundDraftingProgramCanonicalInputs<'_, '_, '_, '_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundDraftingProgramCanonicalInputs")
            .field("limits", &self.limits)
            .field("shape", &self.shape)
            .finish_non_exhaustive()
    }
}
impl<
        'j,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignMetadataHost,
        S: DraftingInitialInputProvider<H> + ?Sized,
    > DraftingProgramDependencyInputs<'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Bind every original formula-name canonical occurrence after complete arithmetic and quotas, then full
    /// result reservation. Native inventory joins retain actual family/kind/Length owners.
    /// Derived matching reuses the native input/source iterator and exact selected source proof;
    /// repeated occurrences remain separate, and Unknown is retained without a numeric read.
    ///
    /// ```
    /// use sc_core::{construction::*,design::DesignMetadataHost};
    /// fn canonical_counts<H:DesignMetadataHost,S:DraftingInitialInputProvider<H>>(
    ///     inputs:&DraftingProgramDependencyInputs<'_, '_, '_, '_, '_, '_, '_,H,S>,
    ///     limits:&DraftingProgramCanonicalLimits) -> Result<(usize,usize),String> {
    ///     let bound=inputs.bind_canonical_inputs(limits).map_err(|e|e.to_string())?;
    ///     assert!(std::ptr::eq(bound.inputs(),inputs));
    ///     assert!(std::ptr::eq(bound.limits(),limits));
    ///     Ok((bound.occurrences().len(),bound.shape().canonical_count()))
    /// }
    /// ```
    /// # Errors
    /// Complete checked arithmetic, occurrences then records quotas, original allocation cause,
    /// then first native canonical cause with its actual original occurrence. No prefix escapes.
    pub fn bind_canonical_inputs<'k>(
        &'k self,
        limits: &'k DraftingProgramCanonicalLimits,
    ) -> Result<
        BoundDraftingProgramCanonicalInputs<'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
        DraftingProgramCanonicalError<'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
    > {
        let refuse = |refusal| DraftingProgramCanonicalError {
            inputs: self,
            limits,
            refusal,
        };
        let overflow = || refuse(DraftingProgramCanonicalRefusal::PopulationOverflow);
        let mut shape = DraftingProgramCanonicalShape {
            occurrences: 0,
            canonical: 0,
            retained_records: 0,
        };
        for occurrence in self.order().dependencies() {
            shape.occurrences = shape.occurrences.checked_add(1).ok_or_else(overflow)?;
            if is_canonical(occurrence.declaration().source()) {
                shape.canonical = shape.canonical.checked_add(1).ok_or_else(overflow)?;
            }
        }
        shape.retained_records = shape
            .occurrences
            .checked_add(shape.canonical)
            .ok_or_else(overflow)?;
        if shape.occurrences > limits.occurrences {
            return Err(refuse(DraftingProgramCanonicalRefusal::Occurrences {
                allowed: limits.occurrences,
                required: shape.occurrences,
            }));
        }
        if shape.retained_records > limits.retained_records {
            return Err(refuse(DraftingProgramCanonicalRefusal::RetainedRecords {
                allowed: limits.retained_records,
                required: shape.retained_records,
            }));
        }
        let mut occurrences = Vec::new();
        occurrences
            .try_reserve_exact(shape.occurrences)
            .map_err(|error| refuse(DraftingProgramCanonicalRefusal::Allocation(error)))?;
        let sources = self.sources().proof();
        let source_inputs = sources.normalized().inputs();
        for (index, occurrence) in self.order().dependencies().enumerate() {
            let canonical = if is_canonical(occurrence.declaration().source()) {
                let input = FormulaCanonicalInput::resolve(
                    occurrence.declaration(),
                    source_inputs.inputs(),
                )
                .map_err(|cause| {
                    refuse(DraftingProgramCanonicalRefusal::Canonical(Box::new(
                        DraftingProgramCanonicalJoinFailure {
                            index,
                            occurrence,
                            cause,
                        },
                    )))
                })?;
                let derived = source_inputs
                    .derived_inputs()
                    .find(|derived| match input.adapter() {
                        FormulaCanonicalInputAdapter::Length(actual) => derived.length().is_some_and(
                            |record| core::ptr::eq(record, actual.canonical_declaration()),
                        ),
                        FormulaCanonicalInputAdapter::Scalar(actual) => derived.scalar().is_some_and(
                            |record| core::ptr::eq(record, actual.canonical_declaration()),
                        ),
                    })
                    .map(|derived| {
                        let Some(source) = sources.source(derived.source().id()).filter(|proof| {
                            core::ptr::eq(proof.normalized().source(), derived.source())
                        }) else {
                            unreachable!(
                                "complete immutable source proof retains every actual Derived source"
                            );
                        };
                        DraftingProgramDerivedBinding {
                            input: derived,
                            source,
                        }
                    });
                Some(DraftingProgramCanonicalBinding { input, derived })
            } else {
                None
            };
            occurrences.push(DraftingProgramCanonicalOccurrence {
                occurrence,
                canonical,
            });
        }
        Ok(BoundDraftingProgramCanonicalInputs {
            inputs: self,
            limits,
            shape,
            occurrences,
        })
    }
}

mod retained_resources;
