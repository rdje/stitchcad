//! Immutable original rule signatures; no body, source binding or execution certificate.
use super::SemanticOperationKind;
use crate::{
    name::MachineToken,
    ontology::EntityId,
    value::{LogicalInputKind, ScalarInputKind},
};
use core::fmt;
mod limits;
pub use limits::{SemanticRuleLimits, SemanticRuleResource, SemanticRuleShape};

/// Exact numeric canonical representation required by a local rule input.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CanonicalNumericInputKind {
    /// Dedicated canonical length declaration, distinct from a general Length scalar.
    Length,
    /// General canonical scalar declaration of this exact kind.
    Scalar(ScalarInputKind),
}
/// A logical rule's typed requirement; source origin and identity are supplied separately.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemanticRuleInputKind {
    /// Finite canonical logical declaration of this exact role, without a selected value.
    Logical(LogicalInputKind),
    /// Exact numeric canonical representation, without coercion or a copied value.
    Numeric(CanonicalNumericInputKind),
}
/// Original named logical-rule input requirement, in declaration order.
#[derive(Clone, PartialEq, Eq)]
pub struct SemanticRuleInput {
    /// Exact local role, not a global formula name or localized label.
    pub role: MachineToken,
    /// Actual declared input kind; Design/Profile origin remains separate.
    pub kind: SemanticRuleInputKind,
}
impl fmt::Debug for SemanticRuleInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SemanticRuleInput")
            .field("kind", &self.kind)
            .finish_non_exhaustive()
    }
}
/// Original named numeric Buttonhole policy requirement, separate from its mandatory button.
#[derive(Clone, PartialEq, Eq)]
pub struct ButtonholePolicyRequirement {
    /// Exact local policy role, not a global name or an inferred positional meaning.
    pub role: MachineToken,
    /// Exact numeric canonical representation; logical requirements cannot be supplied here.
    pub kind: CanonicalNumericInputKind,
}
impl fmt::Debug for ButtonholePolicyRequirement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ButtonholePolicyRequirement")
            .field("kind", &self.kind)
            .finish_non_exhaustive()
    }
}
/// Original complete signature. Neither family supplies a body, domain selection or result value.
#[derive(Clone, PartialEq, Eq)]
pub enum SemanticRuleSignature {
    /// Logical derivation with an explicit logical result kind and complete named requirements.
    Logical {
        /// Actual logical output kind; canonical output domain/state remain in their owner.
        result: LogicalInputKind,
        /// Original local requirements; an explicitly empty vector is legal.
        inputs: Vec<SemanticRuleInput>,
    },
    /// Physical Buttonhole policy with a mandatory canonical button-size source and a later
    /// physical Length result. Neither is an optional named input or an authored/cached hole length.
    Buttonhole {
        /// Complete additional numeric requirements; empty does not remove the mandatory button.
        inputs: Vec<ButtonholePolicyRequirement>,
    },
}
impl fmt::Debug for SemanticRuleSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Logical { result, inputs } => f
                .debug_struct("Logical")
                .field("result", result)
                .field("input_count", &inputs.len())
                .finish_non_exhaustive(),
            Self::Buttonhole { inputs } => f
                .debug_struct("Buttonhole")
                .field("input_count", &inputs.len())
                .finish_non_exhaustive(),
        }
    }
}
impl SemanticRuleSignature {
    fn roles(&self) -> impl Iterator<Item = &MachineToken> {
        let (logical, buttonhole): (&[SemanticRuleInput], &[ButtonholePolicyRequirement]) =
            match self {
                Self::Logical { inputs, result: _ } => (inputs, &[]),
                Self::Buttonhole { inputs } => (&[], inputs),
            };
        logical
            .iter()
            .map(|input| &input.role)
            .chain(buttonhole.iter().map(|input| &input.role))
    }
}
/// Editable declaration identity and complete signature, separate from original body/source data.
#[derive(Clone, PartialEq, Eq)]
pub struct SemanticRuleDefinition {
    /// Actual original rule/policy identity, not an operation or canonical input identity.
    pub id: EntityId,
    /// Original complete family, result kind and local input requirements.
    pub signature: SemanticRuleSignature,
}
impl fmt::Debug for SemanticRuleDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SemanticRuleDefinition")
            .field("id", &self.id)
            .field("signature", &self.signature)
            .finish_non_exhaustive()
    }
}
/// First complete shape/quota refusal, then first repeated original role.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemanticRuleRefusal {
    /// Whole role text or conservative P*P comparisons cannot fit usize.
    PopulationOverflow,
    /// Complete original requirement exceeds its explicit selected allowance.
    Limit {
        /// Complete allowance being checked.
        resource: SemanticRuleResource,
        /// Selected maximum.
        allowed: usize,
        /// Complete original requirement, including invalid repeated roles.
        required: usize,
    },
    /// First repeated role, even if both requirements have the same kind.
    DuplicateRole {
        /// First original zero-based role position.
        first: usize,
        /// First repeated original zero-based role position.
        duplicate: usize,
    },
}
/// Original declaration identity, explicit policy and first constructor refusal; no private text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SemanticRuleDeclarationError {
    /// Actual original rule/policy identity.
    pub rule: EntityId,
    /// Explicit complete constructor allowances, without source/content credit.
    pub limits: SemanticRuleLimits,
    /// Original first complete-bound or duplicate-role cause.
    pub refusal: SemanticRuleRefusal,
}
impl fmt::Display for SemanticRuleDeclarationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "semantic rule {}: {:?}", self.rule, self.refusal)
    }
}
impl std::error::Error for SemanticRuleDeclarationError {}

/// Immutable original typed signature, without original body, Root/Profile/snapshot association,
/// membership, source truth, value, execution or whole-program acceptance. Caller-owned buffers
/// are moved unchanged. Body/signature conformance remains the integration/execution contract.
/// ```compile_fail,E0616
/// fn alter(rule:&mut sc_core::recipe::SemanticRuleDeclaration) { rule.definition.id=sc_core::ontology::EntityId::from_bits(9); }
/// ```
#[derive(Clone)]
pub struct SemanticRuleDeclaration {
    definition: SemanticRuleDefinition,
    limits: SemanticRuleLimits,
    shape: SemanticRuleShape,
}
impl PartialEq for SemanticRuleDeclaration {
    fn eq(&self, other: &Self) -> bool {
        self.definition == other.definition
    }
}
impl Eq for SemanticRuleDeclaration {}
impl fmt::Debug for SemanticRuleDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SemanticRuleDeclaration")
            .field("id", &self.definition.id)
            .field("kind", &self.kind())
            .field("shape", &self.shape)
            .finish_non_exhaustive()
    }
}
impl SemanticRuleDeclaration {
    /// Derive complete input/text/P*P requirements before quotas and duplicate-role comparison.
    /// Validation allocates no workspace and queries no source/value. Distinct roles may require
    /// the same kind. Counts do not bound buffer capacity, comparison bytes, CPU or body execution.
    /// # Errors
    /// Complete arithmetic/first quota refusal, then first repeated role in original order.
    pub fn new(
        definition: SemanticRuleDefinition,
        limits: SemanticRuleLimits,
    ) -> Result<Self, SemanticRuleDeclarationError> {
        let refuse = |refusal| SemanticRuleDeclarationError {
            rule: definition.id,
            limits,
            refusal,
        };
        let shape = SemanticRuleShape::admit(&definition.signature, limits).map_err(refuse)?;
        for (duplicate, role) in definition.signature.roles().enumerate() {
            if let Some(first) = definition
                .signature
                .roles()
                .take(duplicate)
                .position(|prior| prior == role)
            {
                return Err(refuse(SemanticRuleRefusal::DuplicateRole {
                    first,
                    duplicate,
                }));
            }
        }
        Ok(Self {
            definition,
            limits,
            shape,
        })
    }
    /// Exact original definition; no body, source or result is reconstructed from it.
    #[must_use]
    pub const fn definition(&self) -> &SemanticRuleDefinition {
        &self.definition
    }
    /// Actual original declaration identity.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.definition.id
    }
    /// Actual operation family served by this declaration, without source binding.
    #[must_use]
    pub const fn kind(&self) -> SemanticOperationKind {
        match self.definition.signature {
            SemanticRuleSignature::Logical { .. } => SemanticOperationKind::LogicalDerivation,
            SemanticRuleSignature::Buttonhole { .. } => SemanticOperationKind::Buttonhole,
        }
    }
    /// Original explicit constructor policy, not later aggregate accounting credit.
    #[must_use]
    pub const fn limits(&self) -> SemanticRuleLimits {
        self.limits
    }
    /// Complete original populations and conservative comparison bound.
    #[must_use]
    pub const fn shape(&self) -> SemanticRuleShape {
        self.shape
    }
    pub(crate) fn resource_parts(
        &self,
    ) -> (
        &SemanticRuleDefinition,
        SemanticRuleLimits,
        SemanticRuleShape,
    ) {
        let Self {
            definition,
            limits,
            shape,
        } = self;
        (definition, *limits, *shape)
    }
}
