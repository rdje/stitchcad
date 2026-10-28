//! Preflight for wide exact original-coordinate projection;the inverse query is a later slice.

use super::{
    unique_point, EdgeRef, EntityId, IdentityLedger, JournalEntry, OrphaningEdit, Param,
    ResolvedRef, UniquePointLimits, UniquePointOutcome, UniquePointResolution,
};
use core::{
    fmt,
    num::{NonZeroU64, NonZeroUsize},
};

/// Explicit input,candidate and integer limits for an original-point workspace. No default.
/// Counts and conservative operand/result widths do not certify allocator overhead or CPU time.
///
/// ```compile_fail
/// use sc_core::ontology::OriginalPointLimits;
/// let limits: OriginalPointLimits = Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OriginalPointLimits {
    current: UniquePointLimits,
    candidates: NonZeroUsize,
    integer_bits: NonZeroU64,
}
impl OriginalPointLimits {
    /// Choose every limit explicitly. Candidate storage includes all simultaneously retained states.
    #[must_use]
    pub const fn new(
        current: UniquePointLimits,
        candidates: NonZeroUsize,
        integer_bits: NonZeroU64,
    ) -> Self {
        Self {
            current,
            candidates,
            integer_bits,
        }
    }
    /// Whole current-point input admission limits.
    #[must_use]
    pub const fn current(self) -> UniquePointLimits {
        self.current
    }
    /// Maximum simultaneously retained numeric candidate states,including intermediate states.
    #[must_use]
    pub const fn candidates(self) -> NonZeroUsize {
        self.candidates
    }
    /// Maximum conservative integer operand/result width,including temporary products.
    #[must_use]
    pub const fn integer_bits(self) -> NonZeroU64 {
        self.integer_bits
    }
}

/// Actual current-source,target creation or integer-admission refusal. No wide integer is allocated.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OriginalPointAdmissionError {
    /// Current held reference is unavailable;one fixed-size diagnostic box preserves its full outcome.
    Current {
        /// Requested original source edge.
        source: EdgeRef,
        /// Original held reference and exact current refusal;no full choice tree is retained.
        resolution: Box<UniquePointResolution>,
    },
    /// Requested original source edge was never created in this journal.
    UnknownSource {
        /// Requested original source.
        source: EdgeRef,
        /// Actual held reference.
        reference: EdgeRef,
        /// Actual held parameter.
        param: Param,
    },
    /// Conservative integer operand/result reservation exceeds the explicit limit.
    IntegerBits {
        /// Requested original source.
        source: EdgeRef,
        /// Actual held reference.
        reference: EdgeRef,
        /// Actual held parameter.
        param: Param,
        /// Complete derived reservation,not a minimal necessary width.
        required: u64,
        /// Explicit selected limit.
        limit: u64,
    },
    /// Journal-count-derived reservation cannot be represented.
    BoundOverflow {
        /// Requested original source.
        source: EdgeRef,
        /// Actual held reference.
        reference: EdgeRef,
        /// Actual held parameter.
        param: Param,
        /// Actual complete journal population.
        journal_entries: usize,
    },
}
impl fmt::Display for OriginalPointAdmissionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Current { source, resolution } => write!(f, "original source {source}: current reference {} at {} unavailable: {:?}", resolution.reference(), resolution.param(), resolution.outcome()),
            Self::UnknownSource { source, reference, param } => write!(f, "original source {source} was never created; held {reference} at {param}"),
            Self::IntegerBits { source, reference, param, required, limit } => write!(f, "original source {source},held {reference} at {param}: integer reservation {required} bits exceeds {limit}"),
            Self::BoundOverflow { source, reference, param, journal_entries } => write!(f, "original source {source},held {reference} at {param}: reservation overflows for {journal_entries} journal entries"),
        }
    }
}
impl std::error::Error for OriginalPointAdmissionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let Self::Current { resolution, .. } = self {
            if let UniquePointOutcome::Repair {
                cause: OrphaningEdit::RecomputationFailed { cause, .. },
                ..
            } = resolution.outcome()
            {
                return Some(cause);
            }
        }
        None
    }
}

/// Borrowed original-point workspace admission. No wide arithmetic or inverse result exists yet.
/// Known source creation and current uniqueness are prerequisites,not proof of an original preimage.
/// The immutable ledger borrow keeps this admission tied to the exact observed journal.
///
/// ```compile_fail
/// use core::num::{NonZeroU64, NonZeroUsize};
/// use sc_core::ontology::{DeterministicIdGenerator, IdentityLedger, OriginalPointLimits, Param, UniquePointLimits};
/// let mut ledger = IdentityLedger::new();
/// let mut ids = DeterministicIdGenerator::new();
/// let edge = ledger.declare_edges(&mut ids, 1)?.remove(0);
/// let one = NonZeroUsize::new(1).ok_or("positive")?;
/// let limits = OriginalPointLimits::new(UniquePointLimits::new(one, one), one, NonZeroU64::new(508).ok_or("positive")?);
/// let token = ledger.admit_original_point(edge, Param::START, edge, limits)?;
/// ledger.reverse(&mut ids, edge)?;
/// assert_eq!(token.current().edge(), edge);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Clone, Copy)]
pub struct OriginalPointAdmission<'a> {
    ledger: &'a IdentityLedger,
    reference: EdgeRef,
    param: Param,
    source: EdgeRef,
    creation: &'a JournalEntry,
    creation_index: usize,
    current: ResolvedRef,
    limits: OriginalPointLimits,
    component_bits: u64,
    integer_bits: u64,
}
impl fmt::Debug for OriginalPointAdmission<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            ledger,
            reference,
            param,
            source,
            creation,
            creation_index,
            current,
            limits,
            component_bits,
            integer_bits,
        } = self;
        f.debug_struct("OriginalPointAdmission")
            .field("journal_entries", &ledger.journal().len())
            .field("reference", reference)
            .field("param", param)
            .field("source", source)
            .field("source_creation", &creation.operation())
            .field("creation_index", creation_index)
            .field("current", current)
            .field("limits", limits)
            .field("component_bits", component_bits)
            .field("integer_bits", integer_bits)
            .finish()
    }
}
impl OriginalPointAdmission<'_> {
    /// Original held reference edge.
    #[must_use]
    pub const fn reference(self) -> EdgeRef {
        self.reference
    }
    /// Original held reference parameter.
    #[must_use]
    pub const fn param(self) -> Param {
        self.param
    }
    /// Named source edge whose creation frame is requested.
    #[must_use]
    pub const fn source_edge(self) -> EdgeRef {
        self.source
    }
    /// Actual operation that created the requested source edge.
    #[must_use]
    pub const fn source_creation(self) -> EntityId {
        self.creation.operation()
    }
    /// Actual known-source unique current point;does not identify an original preimage.
    #[must_use]
    pub const fn current(self) -> ResolvedRef {
        self.current
    }
    /// Explicit selected limits.
    #[must_use]
    pub const fn limits(self) -> OriginalPointLimits {
        self.limits
    }
    /// Conservative width of each reduced coordinate component through all admitted edits.
    #[must_use]
    pub const fn component_bit_bound(self) -> u64 {
        self.component_bits
    }
    /// Conservative operand/result reservation,including inverse-map scratch and comparisons.
    #[must_use]
    pub const fn integer_bit_bound(self) -> u64 {
        self.integer_bits
    }
}

impl IdentityLedger {
    /// Admit current uniqueness,actual target creation and wide inverse workspace before arithmetic.
    /// A successful token does not claim that this current point has a preimage in the named source.
    /// The inverse query/candidate production is a separately owned implementation stage.
    ///
    /// ```
    /// use core::num::{NonZeroU64, NonZeroUsize};
    /// use sc_core::ontology::{DeterministicIdGenerator, IdentityLedger, OriginalPointLimits, Param, UniquePointLimits};
    /// let mut ledger = IdentityLedger::new();
    /// let mut ids = DeterministicIdGenerator::new();
    /// let edge = ledger.declare_edges(&mut ids, 1)?.remove(0);
    /// let one = NonZeroUsize::new(1).ok_or("positive")?;
    /// let limits = OriginalPointLimits::new(UniquePointLimits::new(one, one), one, NonZeroU64::new(508).ok_or("positive")?);
    /// let token = ledger.admit_original_point(edge, Param::START, edge, limits)?;
    /// assert_eq!(token.current().edge(), edge);
    /// assert_eq!(token.integer_bit_bound(), 508);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn admit_original_point(
        &self,
        reference: EdgeRef,
        param: Param,
        source: EdgeRef,
        limits: OriginalPointLimits,
    ) -> Result<OriginalPointAdmission<'_>, OriginalPointAdmissionError> {
        let resolution = self.resolve_unique_point(reference, param, limits.current);
        let Some(current) = resolution.resolved() else {
            return Err(OriginalPointAdmissionError::Current {
                source,
                resolution: Box::new(resolution),
            });
        };
        let Some((creation_index, creation)) = self
            .journal()
            .iter()
            .enumerate()
            .find(|(_, entry)| unique_point::creates(entry.edit(), source))
        else {
            return Err(OriginalPointAdmissionError::UnknownSource {
                source,
                reference,
                param,
            });
        };
        let count = self.journal().len();
        let bounds = u64::try_from(count).ok().and_then(integer_bound);
        let Some((component_bits, integer_bits)) = bounds else {
            return Err(OriginalPointAdmissionError::BoundOverflow {
                source,
                reference,
                param,
                journal_entries: count,
            });
        };
        if integer_bits > limits.integer_bits.get() {
            return Err(OriginalPointAdmissionError::IntegerBits {
                source,
                reference,
                param,
                required: integer_bits,
                limit: limits.integer_bits.get(),
            });
        }
        Ok(OriginalPointAdmission {
            ledger: self,
            reference,
            param,
            source,
            creation,
            creation_index,
            current,
            limits,
            component_bits,
            integer_bits,
        })
    }
}

fn integer_bound(journal_entries: u64) -> Option<(u64, u64)> {
    // Offset q*(to-from)+from has complete components at most W+127 bits for 63-bit bounds.
    // Split adds at most64,positive Length merge at most32,and reverse adds none. Starting W=63,
    // 63+127J therefore covers all reduced components. 2W+128 also covers cross-products and even
    // sequential unreduced offset products (at most W+190);W is always at least63.
    let component = journal_entries.checked_mul(127)?.checked_add(63)?;
    let workspace = component.checked_mul(2)?.checked_add(128)?;
    Some((component, workspace))
}

#[cfg(test)]
mod tests {
    #[test]
    fn integer_reservation_matches_independent_exact_integer_fixture() {
        let mut seen = 0;
        for line in include_str!("../../../tests/fixtures/projection_workspace.tsv")
            .lines()
            .filter(|line| !line.starts_with('#'))
        {
            let mut row = line.split('\t');
            let count = row.next().expect("count").parse().expect("u64 count");
            let component = row
                .next()
                .expect("component")
                .parse::<u64>()
                .expect("u64 component");
            let expected = row.next().expect("workspace");
            let actual = super::integer_bound(count);
            if expected == "Overflow" {
                assert!(actual.is_none());
            } else {
                assert_eq!(
                    actual,
                    Some((component, expected.parse().expect("u64 workspace")))
                );
            }
            seen += 1;
        }
        assert_eq!(seen, 6);
    }
}
