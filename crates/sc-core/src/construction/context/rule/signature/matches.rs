//! Readonly original matched input/requirement views,without new values or canonical input proof.
use crate::recipe::{
    ButtonholePolicyInput, ButtonholePolicyRequirement, LogicalDerivationInput, SemanticOperation,
    SemanticRuleDeclaration, SemanticRuleInput, SemanticRuleSignature,
};
use core::fmt;
/// Raw pair of original records. Public construction is only a view,not a private match proof.
#[derive(Clone, Copy, Debug)]
pub enum DraftingRuleInputPair<'a> {
    /// Original logical operation input and corresponding original named typed requirement.
    Logical {
        /// Original supplied input,including exact Design/Profile/numeric source representation.
        input: &'a LogicalDerivationInput,
        /// Original corresponding requirement,without a copied local name or kind coercion.
        requirement: &'a SemanticRuleInput,
    },
    /// Original additional numeric Buttonhole input and original corresponding policy requirement.
    Buttonhole {
        /// Original supplied numeric canonical reference,not its resolved value.
        input: &'a ButtonholePolicyInput,
        /// Original corresponding numeric requirement.
        requirement: &'a ButtonholePolicyRequirement,
    },
}
/// Private exact local name/kind match with both original owners,records and authored indices.
/// This is only a local metadata fact;it grants no actual canonical input/source/value binding.
/// ```compile_fail,E0616
/// fn alter(input:&mut sc_core::construction::DraftingRuleInputMatch<'_>) {input.input_index=9;}
/// ```
/// The local match remains tied to its original input/requirement owners:
/// ```compile_fail
/// fn escape<'a>(input:sc_core::construction::DraftingRuleInputMatch<'a>)
///     ->sc_core::construction::DraftingRuleInputMatch<'static> {input}
/// ```
#[derive(Clone, Copy)]
pub struct DraftingRuleInputMatch<'a> {
    operation: &'a SemanticOperation,
    rule: &'a SemanticRuleDeclaration,
    input_index: usize,
    requirement_index: usize,
    pair: DraftingRuleInputPair<'a>,
}
impl<'a> DraftingRuleInputMatch<'a> {
    /// Exact original Root operation owner of the supplied input.
    #[must_use]
    pub const fn operation(self) -> &'a SemanticOperation {
        self.operation
    }
    /// Exact original source declaration owner of the requirement.
    #[must_use]
    pub const fn rule(self) -> &'a SemanticRuleDeclaration {
        self.rule
    }
    /// Original zero-based operation input position.
    #[must_use]
    pub const fn input_index(self) -> usize {
        self.input_index
    }
    /// Original zero-based corresponding signature position;names,not positions,were matched.
    #[must_use]
    pub const fn requirement_index(self) -> usize {
        self.requirement_index
    }
    /// Original input/requirement record loans;no source or selected value is reconstructed.
    #[must_use]
    pub const fn pair(self) -> DraftingRuleInputPair<'a> {
        self.pair
    }
}
impl fmt::Debug for DraftingRuleInputMatch<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingRuleInputMatch")
            .field("input_index", &self.input_index)
            .field("requirement_index", &self.requirement_index)
            .finish_non_exhaustive()
    }
}
pub(super) fn input_count(operation: &SemanticOperation) -> usize {
    match operation {
        SemanticOperation::Buttonhole(operation) => operation.definition().inputs.len(),
        SemanticOperation::LogicalDerivation(operation) => operation.definition().inputs.len(),
    }
}
pub(super) fn input_match<'a>(
    operation: &'a SemanticOperation,
    rule: &'a SemanticRuleDeclaration,
    input_index: usize,
) -> Option<DraftingRuleInputMatch<'a>> {
    let (requirement_index, pair) = match (operation, &rule.definition().signature) {
        (
            SemanticOperation::LogicalDerivation(operation),
            SemanticRuleSignature::Logical { inputs, .. },
        ) => {
            let input = operation.definition().inputs.get(input_index)?;
            let (index, requirement) = inputs
                .iter()
                .enumerate()
                .find(|(_, requirement)| requirement.role == input.role)?;
            (index, DraftingRuleInputPair::Logical { input, requirement })
        }
        (
            SemanticOperation::Buttonhole(operation),
            SemanticRuleSignature::Buttonhole { inputs },
        ) => {
            let input = operation.definition().inputs.get(input_index)?;
            let (index, requirement) = inputs
                .iter()
                .enumerate()
                .find(|(_, requirement)| requirement.role == input.role)?;
            (
                index,
                DraftingRuleInputPair::Buttonhole { input, requirement },
            )
        }
        _ => return None,
    };
    Some(DraftingRuleInputMatch {
        operation,
        rule,
        input_index,
        requirement_index,
        pair,
    })
}
