//! Explicit whole retained-content and normalization policy, separate from language/runtime caps.
use crate::design::DesignResourceLimits;

/// Explicit whole-program logical-content and prospective normalization allowances.
/// Expressions include all Root source bodies, both scalar assertion operands and all native
/// arguments. Retained records conservatively cover `steps + statements + expressions + 2*nodes`:
/// mixed slots, scalar statement records, expression owner-views, nodes and call-argument indices.
/// This composes existing source/native bounds; inline owners may be conservatively counted twice.
/// Semantic operations stay in the actual Root; their complete content is charged there. Their
/// mixed slots count as steps, without fabricated scalar expressions or extra normalized arenas.
/// No allocator bytes/capacity, parser/literal scratch, CPU, checking or runtime budget is granted.
/// ```compile_fail
/// let limits:sc_core::construction::DraftingProgramLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DraftingProgramLimits {
    content: DesignResourceLimits,
    expressions: usize,
    nodes: usize,
    retained_records: usize,
}
impl DraftingProgramLimits {
    /// Select every allowance explicitly; zero normalization allowances permit empty cohorts.
    #[must_use]
    pub const fn new(
        content: DesignResourceLimits,
        expressions: usize,
        nodes: usize,
        retained_records: usize,
    ) -> Self {
        Self {
            content,
            expressions,
            nodes,
            retained_records,
        }
    }
    /// Cumulative actual Root, context source, mixed vector and native held-field content limits.
    #[must_use]
    pub const fn content(self) -> DesignResourceLimits {
        self.content
    }
    /// Complete expression-operand/source-body/argument population allowance.
    #[must_use]
    pub const fn expressions(self) -> usize {
        self.expressions
    }
    /// Complete factory-derived semantic-node allowance, including all branches and repetitions.
    #[must_use]
    pub const fn nodes(self) -> usize {
        self.nodes
    }
    /// Complete conservative prospective normalized-record allowance.
    #[must_use]
    pub const fn retained_records(self) -> usize {
        self.retained_records
    }
}

/// Complete immutable factory-derived prospective shape after whole admission.
/// Private metadata is not caller totals, a normalized owner, a kind proof or an executable program.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DraftingProgramShape {
    pub(super) steps: usize,
    pub(super) statements: usize,
    pub(super) semantics: usize,
    pub(super) sources: usize,
    pub(super) arguments: usize,
    pub(super) expressions: usize,
    pub(super) nodes: usize,
    pub(super) retained_records: usize,
}
impl DraftingProgramShape {
    /// Complete mixed sequence population.
    #[must_use]
    pub const fn step_count(self) -> usize {
        self.steps
    }
    /// Complete original scalar statement population, including every assertion.
    #[must_use]
    pub const fn statement_count(self) -> usize {
        self.statements
    }
    /// Complete actual Root semantic operation population, independent of scalar statements.
    #[must_use]
    pub const fn semantic_operation_count(self) -> usize {
        self.semantics
    }
    /// All original separately identified Root formula-source bodies, including unused sources.
    #[must_use]
    pub const fn source_count(self) -> usize {
        self.sources
    }
    /// Native argument occurrences, including repeated uses of the same declaration.
    #[must_use]
    pub const fn argument_count(self) -> usize {
        self.arguments
    }
    /// Complete source-body, scalar-operand and native-argument occurrences.
    #[must_use]
    pub const fn expression_count(self) -> usize {
        self.expressions
    }
    /// Complete semantic nodes across all three cohorts, without deduplication.
    #[must_use]
    pub const fn node_count(self) -> usize {
        self.nodes
    }
    /// Conservative `steps + statements + expressions + 2*nodes` bound, not allocator bytes.
    #[must_use]
    pub const fn retained_record_bound(self) -> usize {
        self.retained_records
    }
}
