//! Fresh complete held content and checked prospective name populations;no namespace prefix.
use super::DraftingProgramNameInputs;
use crate::{
    construction::DraftingContextSource,
    design::{
        DesignFormulaDeclarationSource, DesignMetadataHost, DesignResourceDemand,
        DesignResourceLimitError, DesignResourceLimits, DesignResourceMeter, DesignResourceSource,
    },
    recipe::FormulaReservedName,
};
use core::fmt;

/// Explicit complete held-content and prospective mixed namespace allowances,without defaults.
/// Namespace entries conservatively include all eight reserved names,actual Root parameters and
/// selected host rows,extra initial names,named outputs and every scalar statement (assertions too).
/// Retained records add one association slot per named output and one binding slot per extra initial.
/// These logical populations grant
/// no allocator bytes/capacity,comparison CPU,kind checking,dependency or runtime budget.
/// ```compile_fail
/// let limits:sc_core::construction::DraftingProgramNameLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DraftingProgramNameLimits {
    content: DesignResourceLimits,
    namespace_entries: usize,
    retained_records: usize,
}
impl DraftingProgramNameLimits {
    /// Select every allowance;zero name limits intentionally refuse even reserved metadata.
    #[must_use]
    pub const fn new(
        content: DesignResourceLimits,
        namespace_entries: usize,
        retained_records: usize,
    ) -> Self {
        Self {
            content,
            namespace_entries,
            retained_records,
        }
    }
    /// Cumulative actual normalized-program and additional-name held-content policy.
    #[must_use]
    pub const fn content(self) -> DesignResourceLimits {
        self.content
    }
    /// Complete conservative namespace population allowance.
    #[must_use]
    pub const fn namespace_entries(self) -> usize {
        self.namespace_entries
    }
    /// Complete namespace plus output-association and extra-initial binding-slot allowance.
    #[must_use]
    pub const fn retained_records(self) -> usize {
        self.retained_records
    }
}

/// Factory-derived complete prospective populations;not a supplied hint or namespace proof.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DraftingProgramNameShape {
    parameters: usize,
    host: usize,
    initial: usize,
    outputs: usize,
    statements: usize,
    namespace_entries: usize,
    retained_records: usize,
}
impl DraftingProgramNameShape {
    /// Actual Root parameter rows.
    #[must_use]
    pub const fn parameter_count(self) -> usize {
        self.parameters
    }
    /// Actual selected measurement/Ease source rows,including fallible binding items.
    #[must_use]
    pub const fn host_declaration_count(self) -> usize {
        self.host
    }
    /// Original additional initial declaration occurrences.
    #[must_use]
    pub const fn initial_count(self) -> usize {
        self.initial
    }
    /// Original named-output occurrences,including repeated or invalid requested positions.
    #[must_use]
    pub const fn output_count(self) -> usize {
        self.outputs
    }
    /// Actual whole scalar statement count;assertions conservatively reserve a slot too.
    #[must_use]
    pub const fn statement_count(self) -> usize {
        self.statements
    }
    /// Conservative complete namespace entries,including all language reserved names.
    #[must_use]
    pub const fn namespace_entry_bound(self) -> usize {
        self.namespace_entries
    }
    /// Namespace entry bound plus one slot per output association and per extra-initial binding.
    #[must_use]
    pub const fn retained_record_bound(self) -> usize {
        self.retained_records
    }
}

/// Original content or complete prospective population refusal;no formula diagnostic is invented.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingProgramNameWorkRefusal {
    /// First actual raw/normalized/name field refusal,with original owner/role/index.
    Resources(DesignResourceLimitError),
    /// Complete checked population addition cannot be represented.
    PopulationOverflow,
    /// Complete namespace bound exceeds its explicit allowance.
    NamespaceEntries {
        /// Selected maximum.
        allowed: usize,
        /// Complete conservative population.
        required: usize,
    },
    /// Complete namespace and association records exceed their explicit allowance.
    RetainedRecords {
        /// Selected maximum.
        allowed: usize,
        /// Complete conservative population.
        required: usize,
    },
}
/// Refusal retaining the actual original name/program owner and exact selected policy.
/// Boxing compacts this diagnostic only;no namespace or source inventory is allocated.
pub struct DraftingProgramNameWorkError<
    'b,
    'n,
    'p,
    'a,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
> {
    inputs: &'b DraftingProgramNameInputs<'n, 'p, 'a, H, S>,
    limits: &'b DraftingProgramNameLimits,
    refusal: Box<DraftingProgramNameWorkRefusal>,
}
impl<'b, 'n, 'p, 'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    DraftingProgramNameWorkError<'b, 'n, 'p, 'a, H, S>
{
    /// Actual original input owner,including all invalid raw source/position requests.
    #[must_use]
    pub const fn inputs(&self) -> &'b DraftingProgramNameInputs<'n, 'p, 'a, H, S> {
        self.inputs
    }
    /// Exact selected policy.
    #[must_use]
    pub const fn limits(&self) -> &'b DraftingProgramNameLimits {
        self.limits
    }
    /// Original first complete content or prospective population refusal.
    #[must_use]
    pub fn refusal(&self) -> DraftingProgramNameWorkRefusal {
        *self.refusal
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for DraftingProgramNameWorkError<'_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramNameWorkError")
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Display
    for DraftingProgramNameWorkError<'_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "complete drafting name work: {:?}", self.refusal)
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> std::error::Error
    for DraftingProgramNameWorkError<'_, '_, '_, '_, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.refusal.as_ref() {
            DraftingProgramNameWorkRefusal::Resources(error) => Some(error),
            DraftingProgramNameWorkRefusal::PopulationOverflow
            | DraftingProgramNameWorkRefusal::NamespaceEntries { .. }
            | DraftingProgramNameWorkRefusal::RetainedRecords { .. } => None,
        }
    }
}

/// Private bounded actual name/program owner;no association,namespace,kind or source-truth grant.
/// The actual host is immutable and its selected-row iterator was counted without consuming items.
/// ```compile_fail,E0616
/// use sc_core::{construction::{AdmittedDraftingProgramNameWork,DraftingContextSource},design::DesignMetadataHost};
/// fn change<H:DesignMetadataHost,S:DraftingContextSource<H>>(work:&mut AdmittedDraftingProgramNameWork<'_, '_, '_, '_,H,S>) {
///     work.shape=work.shape();
/// }
/// ```
/// ```compile_fail
/// use sc_core::{construction::{AdmittedDraftingProgramNameWork,DraftingContextSource},design::DesignMetadataHost};
/// fn escape<'b,'n,'p,'a,H:DesignMetadataHost,S:DraftingContextSource<H>>(work:AdmittedDraftingProgramNameWork<'b,'n,'p,'a,H,S>)
///     ->AdmittedDraftingProgramNameWork<'static,'n,'p,'a,H,S> {work}
/// ```
pub struct AdmittedDraftingProgramNameWork<
    'b,
    'n,
    'p,
    'a,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
> {
    inputs: &'b DraftingProgramNameInputs<'n, 'p, 'a, H, S>,
    limits: &'b DraftingProgramNameLimits,
    demand: DesignResourceDemand,
    shape: DraftingProgramNameShape,
}
impl<'b, 'n, 'p, 'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    AdmittedDraftingProgramNameWork<'b, 'n, 'p, 'a, H, S>
{
    /// Actual complete original owner,never a replacement program or caller namespace.
    #[must_use]
    pub const fn inputs(&self) -> &'b DraftingProgramNameInputs<'n, 'p, 'a, H, S> {
        self.inputs
    }
    /// Exact selected held-content/work policy.
    #[must_use]
    pub const fn limits(&self) -> &'b DraftingProgramNameLimits {
        self.limits
    }
    /// Fresh complete actual held-content demand.
    #[must_use]
    pub const fn demand(&self) -> DesignResourceDemand {
        self.demand
    }
    /// Complete prospective logical name populations.
    #[must_use]
    pub const fn shape(&self) -> DraftingProgramNameShape {
        self.shape
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for AdmittedDraftingProgramNameWork<'_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AdmittedDraftingProgramNameWork")
            .field("shape", &self.shape)
            .finish_non_exhaustive()
    }
}
fn sum(values: impl IntoIterator<Item = usize>) -> Result<usize, DraftingProgramNameWorkRefusal> {
    values.into_iter().try_fold(0usize, |total, value| {
        total
            .checked_add(value)
            .ok_or(DraftingProgramNameWorkRefusal::PopulationOverflow)
    })
}
impl<
        'n,
        'p,
        'a,
        H: DesignResourceSource + DesignFormulaDeclarationSource,
        S: DraftingContextSource<H> + ?Sized,
    > DraftingProgramNameInputs<'n, 'p, 'a, H, S>
{
    /// Admit fresh complete held content,then complete prospective name populations before any
    /// namespace factory or source/position/kind checking. Count the actual borrowed selected-host
    /// iterator without advancing it;no declarations,namespace or association index are allocated.
    /// No accepted prefix or caller-provided shape/demand can supply this private bounded owner.
    ///
    /// ```
    /// use sc_core::{construction::{DraftingContextSource, DraftingProgramNameInputs,
    ///     DraftingProgramNameLimits, DraftingProgramNameShape},design::{DesignFormulaDeclarationSource,
    ///     DesignResourceDemand, DesignResourceSource}};
    /// fn bounded_names<H:DesignResourceSource+DesignFormulaDeclarationSource,
    ///     S:DraftingContextSource<H> + ?Sized>(names:&DraftingProgramNameInputs<'_, '_, '_,H,S>,
    ///     policy:DraftingProgramNameLimits)->Result<(DesignResourceDemand,DraftingProgramNameShape),String> {
    ///     let admitted=names.admit_name_work(&policy).map_err(|error|error.to_string())?;
    ///     Ok((admitted.demand(),admitted.shape()))
    /// }
    /// ```
    /// # Errors
    /// Content precedes checked shape arithmetic,namespace quota,then retained-record quota.
    /// Exact original input/policy and first typed resource cause remain available on failure.
    pub fn admit_name_work<'b>(
        &'b self,
        limits: &'b DraftingProgramNameLimits,
    ) -> Result<
        AdmittedDraftingProgramNameWork<'b, 'n, 'p, 'a, H, S>,
        DraftingProgramNameWorkError<'b, 'n, 'p, 'a, H, S>,
    > {
        let refuse = |refusal| DraftingProgramNameWorkError {
            inputs: self,
            limits,
            refusal: Box::new(refusal),
        };
        let mut meter = DesignResourceMeter::new(limits.content);
        self.charge_resources(&mut meter)
            .map_err(|error| refuse(DraftingProgramNameWorkRefusal::Resources(error)))?;
        let demand = meter
            .finish()
            .map_err(|error| refuse(DraftingProgramNameWorkRefusal::Resources(error)))?;
        let program = self.program();
        let root = program.admitted().inputs().context().scope().root();
        let parameters = root.parameters().parameters().len();
        let host = root
            .sources()
            .host()
            .formula_declaration_sources(root.references(), root.inputs())
            .len();
        let initial = self.initial().len();
        let outputs = self.outputs().len();
        let statements = program.recipe().normalized().statements().len();
        let namespace_entries = sum([
            FormulaReservedName::ALL.len(),
            parameters,
            host,
            initial,
            outputs,
            statements,
        ])
        .map_err(refuse)?;
        let retained_records = sum([namespace_entries, outputs, initial]).map_err(refuse)?;
        if namespace_entries > limits.namespace_entries {
            return Err(refuse(DraftingProgramNameWorkRefusal::NamespaceEntries {
                allowed: limits.namespace_entries,
                required: namespace_entries,
            }));
        }
        if retained_records > limits.retained_records {
            return Err(refuse(DraftingProgramNameWorkRefusal::RetainedRecords {
                allowed: limits.retained_records,
                required: retained_records,
            }));
        }
        Ok(AdmittedDraftingProgramNameWork {
            inputs: self,
            limits,
            demand,
            shape: DraftingProgramNameShape {
                parameters,
                host,
                initial,
                outputs,
                statements,
                namespace_entries,
                retained_records,
            },
        })
    }
}
#[cfg(test)]
mod tests {
    use super::{sum, DraftingProgramNameWorkRefusal};
    #[test]
    fn complete_population_arithmetic_refuses_unrepresentable_sum() {
        assert_eq!(sum([usize::MAX, 0]), Ok(usize::MAX));
        assert_eq!(
            sum([usize::MAX, 1]),
            Err(DraftingProgramNameWorkRefusal::PopulationOverflow)
        );
    }
}

mod retained_resources;
