//! Original typed Buttonhole recipe intent; no physical formula or independently authored length.
use crate::{
    name::MachineToken,
    ontology::{EntityId, NotionSize},
    value::CanonicalInputTarget,
};
use core::fmt;
mod limits;
pub use limits::{
    ButtonholeOperationLimits, ButtonholeOperationResource, ButtonholeOperationShape,
};

/// One original named policy dependency, with no independently owned value or uncertainty state.
#[derive(Clone, PartialEq, Eq)]
pub struct ButtonholePolicyInput {
    /// Exact local policy role, not a global formula/geometry name or localized label.
    pub role: MachineToken,
    /// Actual numeric canonical target representation and expected scalar kind where applicable.
    /// This records a reference; membership, currentness and policy-signature checks come later.
    pub target: CanonicalInputTarget,
}
impl fmt::Debug for ButtonholePolicyInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ButtonholePolicyInput")
            .field("target", &self.target)
            .finish_non_exhaustive()
    }
}
/// Editable original semantic recipe operation. Policy identity and roles are explicitly authored;
/// core supplies no physical formula, clearance, vendor-code dimensions or default dependency.
#[derive(Clone, PartialEq, Eq)]
pub struct ButtonholeOperationDefinition {
    /// Stable semantic operation identity, distinct from any later ordered position.
    pub id: EntityId,
    /// Existing canonical button-size origin, retained exactly as Declaration or Profile.
    pub button: NotionSize,
    /// Explicit original physical-policy source reference; its existence/kind is checked separately.
    pub policy: EntityId,
    /// Complete original named numeric policy references, in authored order; empty is explicit.
    pub inputs: Vec<ButtonholePolicyInput>,
}
impl fmt::Debug for ButtonholeOperationDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ButtonholeOperationDefinition")
            .field("id", &self.id)
            .field("input_count", &self.inputs.len())
            .finish_non_exhaustive()
    }
}
/// First complete population/arithmetic refusal or original repeated local role.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonholeOperationRefusal {
    /// Complete role-byte or P*P comparison arithmetic cannot fit usize; no prefix is admitted.
    PopulationOverflow,
    /// Complete original requirement exceeds the explicit selected allowance.
    Limit {
        /// Exact complete allowance being checked.
        resource: ButtonholeOperationResource,
        /// Selected maximum.
        allowed: usize,
        /// Complete original requirement, including repeated invalid roles.
        required: usize,
    },
    /// Two policy dependencies use the same local role; equal targets do not excuse repetition.
    DuplicateRole {
        /// First original zero-based role position.
        first: usize,
        /// First repeated original zero-based role position.
        duplicate: usize,
    },
}
/// Original operation identity, explicit policy and first complete shape/role refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonholeOperationError {
    /// Exact original operation identity.
    pub operation: EntityId,
    /// Explicit selected constructor bounds, without source or aggregate accounting credit.
    pub limits: ButtonholeOperationLimits,
    /// Original first refusal; authored role text is omitted from diagnostics.
    pub refusal: ButtonholeOperationRefusal,
}
impl fmt::Display for ButtonholeOperationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "buttonhole operation {}: {:?}",
            self.operation, self.refusal
        )
    }
}
impl std::error::Error for ButtonholeOperationError {}

/// Immutable standalone original Buttonhole operation. The button origin is its sole hardware-size
/// dependency; there is no authored/cached hole length or executed result. This constructor grants
/// no ordered recipe membership, Root/Profile/policy source binding, scalar/native step, geometry,
/// source truth or positive physical length. G3 owns physical policy execution and its proof.
/// ```compile_fail,E0616
/// fn alter(operation:&mut sc_core::recipe::ButtonholeOperation) { operation.definition.inputs.clear(); }
/// ```
/// ```compile_fail,E0560
/// use sc_core::{recipe::ButtonholeOperationDefinition,ontology::{EntityId,NotionSize}};
/// let definition=ButtonholeOperationDefinition {
///     id:EntityId::from_bits(1),button:NotionSize::Declaration(EntityId::from_bits(2)),
///     policy:EntityId::from_bits(3),inputs:vec![],length:sc_units::Length::ZERO,
/// };
/// ```
#[derive(Clone)]
pub struct ButtonholeOperation {
    definition: ButtonholeOperationDefinition,
    limits: ButtonholeOperationLimits,
    shape: ButtonholeOperationShape,
}
impl PartialEq for ButtonholeOperation {
    fn eq(&self, other: &Self) -> bool {
        self.definition == other.definition
    }
}
impl Eq for ButtonholeOperation {}
impl fmt::Debug for ButtonholeOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ButtonholeOperation")
            .field("id", &self.definition.id)
            .field("shape", &self.shape)
            .finish_non_exhaustive()
    }
}
impl ButtonholeOperation {
    /// Derive complete original P/role-byte/P*P shape before input,text,comparison quotas,then reject
    /// the first repeated role by borrowed prefix scan. Validation allocates no workspace,queries no
    /// values/sources and retains the original input vector unchanged. Input allocation is caller-owned;
    /// logical counts/comparisons do not bound allocator capacity, comparison bytes, CPU or execution.
    /// Distinct roles may reference the same target; an explicitly empty dependency vector is legal.
    /// # Errors
    /// Complete arithmetic/first quota refusal,then DuplicateRole with both original indices.
    pub fn new(
        definition: ButtonholeOperationDefinition,
        limits: ButtonholeOperationLimits,
    ) -> Result<Self, ButtonholeOperationError> {
        let refuse = |refusal| ButtonholeOperationError {
            operation: definition.id,
            limits,
            refusal,
        };
        let shape = ButtonholeOperationShape::admit(&definition, limits).map_err(refuse)?;
        for (duplicate, input) in definition.inputs.iter().enumerate() {
            if let Some(first) = definition
                .inputs
                .iter()
                .take(duplicate)
                .position(|prior| prior.role == input.role)
            {
                return Err(refuse(ButtonholeOperationRefusal::DuplicateRole {
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
    /// Exact original immutable definition, without an evaluated or default physical length.
    #[must_use]
    pub const fn definition(&self) -> &ButtonholeOperationDefinition {
        &self.definition
    }
    /// Exact stable operation identity, not a recipe/native/scalar position.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.definition.id
    }
    /// Original selected constructor policy, not a content/source/runtime grant.
    #[must_use]
    pub const fn limits(&self) -> ButtonholeOperationLimits {
        self.limits
    }
    /// Complete original populations and conservative comparison bound.
    #[must_use]
    pub const fn shape(&self) -> ButtonholeOperationShape {
        self.shape
    }
    // Exhaustive original owner view: new retained fields must extend the accounting contract.
    pub(crate) fn resource_parts(
        &self,
    ) -> (
        &ButtonholeOperationDefinition,
        ButtonholeOperationLimits,
        ButtonholeOperationShape,
    ) {
        let Self {
            definition,
            limits,
            shape,
        } = self;
        (definition, *limits, *shape)
    }
}
