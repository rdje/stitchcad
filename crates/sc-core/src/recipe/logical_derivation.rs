//! Original typed logical derivation sources, separate from scalar statements and selected values.
use crate::{
    name::MachineToken,
    ontology::{EntityId, ProfileParameterRef},
    value::{CanonicalInputTarget, LogicalInputKind, LogicalInputTarget},
};
use core::fmt;
mod limits;
pub use limits::{LogicalDerivationLimits, LogicalDerivationResource, LogicalDerivationShape};

/// Original input representation and expected role, without source lookup or selected values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogicalDerivationInputSource {
    /// Original Design logical declaration reference; actual domain/state stays in its owner.
    Logical(LogicalInputTarget),
    /// Original Profile logical parameter and expected role; selected profile/schema/content loans
    /// must be bound later. This parameter ID supplies no profile pin or currentness certificate.
    Profile {
        /// Original logical parameter reference, without a copied selected value.
        parameter: ProfileParameterRef,
        /// Original expected logical role.
        kind: LogicalInputKind,
    },
    /// Original numeric canonical dependency, including explicit representation and scalar kind.
    /// This does not turn logical inputs into numeric operands or add a formula statement.
    Numeric(CanonicalInputTarget),
}
/// One exact named rule-input reference; no duplicated domain,uncertainty state or cached result.
#[derive(Clone, PartialEq, Eq)]
pub struct LogicalDerivationInput {
    /// Authored local rule role, not a global formula/geometry name or localized label.
    pub role: MachineToken,
    /// Exact original typed input representation and expected kind.
    pub source: LogicalDerivationInputSource,
}
impl fmt::Debug for LogicalDerivationInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LogicalDerivationInput")
            .field("source", &self.source)
            .finish_non_exhaustive()
    }
}
/// Editable actual logical recipe declaration, not rule-language syntax or an evaluated selection.
/// A later original-source binder checks the rule's existence/kind/signature and every actual source;
/// this record supplies none of those proofs merely by retaining matching identities.
#[derive(Clone, PartialEq, Eq)]
pub struct LogicalDerivationDefinition {
    /// Stable original semantic operation identity, never a scalar/native ordinal.
    pub id: EntityId,
    /// Exact canonical logical output declaration and expected role. The actual canonical record
    /// retains its domain and Derived operation reference; they are not copied into this recipe.
    pub output: LogicalInputTarget,
    /// Explicit original rule-source identity, without an inferred rule body or default selection.
    pub rule: EntityId,
    /// Complete original named typed rule inputs in authored order; empty is explicit.
    pub inputs: Vec<LogicalDerivationInput>,
}
impl fmt::Debug for LogicalDerivationDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LogicalDerivationDefinition")
            .field("id", &self.id)
            .field("output", &self.output)
            .field("input_count", &self.inputs.len())
            .finish_non_exhaustive()
    }
}
/// First complete arithmetic/quota refusal,then first repeated original local role.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogicalDerivationRefusal {
    /// Complete role text or conservative P*P comparisons cannot fit usize.
    PopulationOverflow,
    /// Complete original requirement exceeds its explicit selected allowance.
    Limit {
        /// Exact resource being checked.
        resource: LogicalDerivationResource,
        /// Selected maximum.
        allowed: usize,
        /// Whole original requirement, including repeated invalid roles.
        required: usize,
    },
    /// First repeated role in authored input order; matching source references do not excuse it.
    DuplicateRole {
        /// First original zero-based role position.
        first: usize,
        /// First repeated original zero-based role position.
        duplicate: usize,
    },
}
/// Original operation identity and selected constructor limits with the first typed refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LogicalDerivationError {
    /// Original requested operation identity.
    pub operation: EntityId,
    /// Explicit complete selected constructor policy, not source/content admission.
    pub limits: LogicalDerivationLimits,
    /// Original first complete-bound or duplicate-role cause, without authored role text.
    pub refusal: LogicalDerivationRefusal,
}
impl fmt::Display for LogicalDerivationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "logical derivation {}: {:?}",
            self.operation, self.refusal
        )
    }
}
impl std::error::Error for LogicalDerivationError {}

/// Immutable standalone original logical operation with no selected value,default rule or copied
/// domain/state. It grants no canonical output/input binding, actual rule signature, Profile pin,
/// graph/order acceptance, original recipe membership or execution. Equal IDs cannot supply those
/// source loans. Logical values remain outside scalar formula kinds and enum-valued let syntax.
/// ```compile_fail,E0616
/// fn alter(operation:&mut sc_core::recipe::LogicalDerivationOperation) {operation.definition.inputs.clear();}
/// ```
/// ```compile_fail,E0560
/// use sc_core::{recipe::LogicalDerivationDefinition,ontology::EntityId,value::*};
/// let definition=LogicalDerivationDefinition {
///     id:EntityId::from_bits(1),output:LogicalInputTarget {
///         declaration:EntityId::from_bits(2),kind:LogicalInputKind::PocketOpening,
///     },rule:EntityId::from_bits(3),inputs:vec![],value:sc_core::name::MachineToken::new("option_a").unwrap(),
/// };
/// ```
#[derive(Clone)]
pub struct LogicalDerivationOperation {
    definition: LogicalDerivationDefinition,
    limits: LogicalDerivationLimits,
    shape: LogicalDerivationShape,
}
impl PartialEq for LogicalDerivationOperation {
    fn eq(&self, other: &Self) -> bool {
        self.definition == other.definition
    }
}
impl Eq for LogicalDerivationOperation {}
impl fmt::Debug for LogicalDerivationOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LogicalDerivationOperation")
            .field("id", &self.definition.id)
            .field("shape", &self.shape)
            .finish_non_exhaustive()
    }
}
impl LogicalDerivationOperation {
    /// Derive whole original P/role-byte/P*P requirements,then input/text/comparison quotas before
    /// first duplicate-role prefix scan. Validation allocates no workspace,queries no source and
    /// preserves the moved original vector/text. Distinct roles may share sources. Empty is explicit.
    /// Logical allowances do not bound allocator capacity,string-comparison bytes,CPU or execution;
    /// caller-owned input buffers were allocated before this constructor.
    /// # Errors
    /// Whole arithmetic/first quota refusal,then DuplicateRole with both original indices.
    pub fn new(
        definition: LogicalDerivationDefinition,
        limits: LogicalDerivationLimits,
    ) -> Result<Self, LogicalDerivationError> {
        let refuse = |refusal| LogicalDerivationError {
            operation: definition.id,
            limits,
            refusal,
        };
        let shape = LogicalDerivationShape::admit(&definition, limits).map_err(refuse)?;
        for (duplicate, input) in definition.inputs.iter().enumerate() {
            if let Some(first) = definition
                .inputs
                .iter()
                .take(duplicate)
                .position(|prior| prior.role == input.role)
            {
                return Err(refuse(LogicalDerivationRefusal::DuplicateRole {
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
    /// Exact original immutable definition; actual domains and values remain in their source owners.
    #[must_use]
    pub const fn definition(&self) -> &LogicalDerivationDefinition {
        &self.definition
    }
    /// Original semantic operation identity, not its later ordered recipe position.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.definition.id
    }
    /// Original explicit constructor allowances, not a source/currentness or content grant.
    #[must_use]
    pub const fn limits(&self) -> LogicalDerivationLimits {
        self.limits
    }
    /// Complete original populations and conservative role comparison bound.
    #[must_use]
    pub const fn shape(&self) -> LogicalDerivationShape {
        self.shape
    }
    pub(crate) fn resource_parts(
        &self,
    ) -> (
        &LogicalDerivationDefinition,
        LogicalDerivationLimits,
        LogicalDerivationShape,
    ) {
        let Self {
            definition,
            limits,
            shape,
        } = self;
        (definition, *limits, *shape)
    }
}
