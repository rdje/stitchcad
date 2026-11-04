//! Original logical ownership, usable standalone or inside canonical Design inputs.
use super::{LogicalInputDeclaration, LogicalInputKind};
use crate::ontology::EntityId;
use core::fmt;

/// Explicit complete standalone ownership-validation allowances, without a default policy.
/// These are declaration counts and conservative identity comparisons, not held-content bytes,
/// allocator capacity, source lookup work, currentness or runtime limits.
/// ```compile_fail
/// let policy:sc_core::value::LogicalInputInventoryLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LogicalInputInventoryLimits {
    declarations: usize,
    inspections: usize,
}
impl LogicalInputInventoryLimits {
    /// Select both complete allowances; zero admits an explicitly empty inventory.
    #[must_use]
    pub const fn new(declarations: usize, validation_inspections: usize) -> Self {
        Self {
            declarations,
            inspections: validation_inspections,
        }
    }
    /// Maximum complete original declarations, including repeated invalid identities.
    #[must_use]
    pub const fn declarations(self) -> usize {
        self.declarations
    }
    /// Maximum conservative L*L identity-comparison allowance.
    #[must_use]
    pub const fn validation_inspections(self) -> usize {
        self.inspections
    }
}
/// Factory-derived complete declaration count and conservative ownership-comparison bound.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LogicalInputInventoryShape {
    declarations: usize,
    inspections: usize,
}
impl LogicalInputInventoryShape {
    /// Actual complete original declaration population.
    #[must_use]
    pub const fn declarations(self) -> usize {
        self.declarations
    }
    /// Conservative L*L comparisons, not measured CPU or query work.
    #[must_use]
    pub const fn validation_inspections(self) -> usize {
        self.inspections
    }
}
/// First complete allowance or original authored ownership refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogicalInputInventoryRefusal {
    /// Complete conservative comparison arithmetic cannot fit usize, before quota checks.
    PopulationOverflow,
    /// Complete original declaration population exceeds its selected allowance.
    Declarations {
        /// Selected maximum.
        allowed: usize,
        /// Complete requirement.
        required: usize,
    },
    /// Conservative complete L*L comparison bound exceeds its selected allowance.
    ValidationInspections {
        /// Selected maximum.
        allowed: usize,
        /// Complete requirement.
        required: usize,
    },
    /// Original repeated declaration identity, even for identical definitions or another kind.
    DuplicateIdentity {
        /// Original repeated identity; referenced provenance ids are not owners.
        identity: EntityId,
        /// First zero-based authored owner.
        first: usize,
        /// First repeated zero-based authored owner.
        duplicate: usize,
    },
}
/// Original explicit selected policy and first whole-population/ownership refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LogicalInputInventoryError {
    /// Actual selected constructor allowances, without claiming aggregate content admission.
    pub limits: LogicalInputInventoryLimits,
    /// First complete-bound or original identity/position cause.
    pub refusal: LogicalInputInventoryRefusal,
}
impl fmt::Display for LogicalInputInventoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "logical input ownership: {:?}", self.refusal)
    }
}
impl std::error::Error for LogicalInputInventoryError {}

/// Immutable standalone original logical owner. Existing records keep their original domains,
/// state/provenance and authored order. Construction alone does not attach them to a Design/Profile Root, admit
/// aggregate held content, validate sources/evidence or supply runtime/physical readiness.
/// ```compile_fail,E0616
/// fn alter(inputs:&mut sc_core::value::LogicalInputs) {inputs.declarations.clear();}
/// ```
#[derive(Clone)]
pub struct LogicalInputs {
    declarations: Vec<LogicalInputDeclaration>,
    limits: LogicalInputInventoryLimits,
    shape: LogicalInputInventoryShape,
}
impl PartialEq for LogicalInputs {
    fn eq(&self, other: &Self) -> bool {
        self.declarations == other.declarations
    }
}
impl Eq for LogicalInputs {}
impl fmt::Debug for LogicalInputs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LogicalInputs")
            .field("shape", &self.shape)
            .finish_non_exhaustive()
    }
}
/// First lookup refusal; a kind mismatch retains the actual original record loan.
#[derive(Clone, Copy, Debug)]
pub enum LogicalInputLookupError<'a> {
    /// No declaration owns this identity; a referenced source/provenance ID is not membership.
    Missing {
        /// Original requested declaration identity.
        declaration: EntityId,
        /// Original requested logical role, retained even when the record is absent.
        expected: LogicalInputKind,
    },
    /// The actual original member has a different logical role.
    Kind {
        /// Original requested role.
        expected: LogicalInputKind,
        /// Actual original record, without a copied definition/state.
        actual: &'a LogicalInputDeclaration,
    },
}
impl fmt::Display for LogicalInputLookupError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing {
                declaration,
                expected,
            } => {
                write!(
                    f,
                    "logical {expected:?} declaration {declaration} is absent"
                )
            }
            Self::Kind { expected, actual } => write!(
                f,
                "logical declaration {} needs {expected:?}, has {:?}",
                actual.id(),
                actual.kind()
            ),
        }
    }
}
impl std::error::Error for LogicalInputLookupError<'_> {}
/// Privately bound original inventory member and exact logical kind, without a value read.
/// Equal definitions in a different actual inventory cannot replace this owner or record.
/// ```compile_fail,E0616
/// fn alter(binding:&mut sc_core::value::LogicalInputBinding<'_>) {binding.index=0;}
/// ```
/// ```compile_fail
/// fn escape(inputs:&sc_core::value::LogicalInputs)->sc_core::value::LogicalInputBinding<'static> {
///     inputs.resolve(sc_core::ontology::EntityId::from_bits(1),sc_core::value::LogicalInputKind::PocketOpening).unwrap()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct LogicalInputBinding<'a> {
    inputs: &'a LogicalInputs,
    declaration: &'a LogicalInputDeclaration,
    index: usize,
}
impl<'a> LogicalInputBinding<'a> {
    /// Actual original containing inventory, not a matching-ID or equal-data replacement.
    #[must_use]
    pub const fn inputs(self) -> &'a LogicalInputs {
        self.inputs
    }
    /// Actual original kind-checked declaration and all original source/domain/state/provenance.
    #[must_use]
    pub const fn declaration(self) -> &'a LogicalInputDeclaration {
        self.declaration
    }
    /// Zero-based original authored membership position; identity is never an array index.
    #[must_use]
    pub const fn index(self) -> usize {
        self.index
    }
}
impl LogicalInputs {
    // The numeric-only canonical constructor supplies no logical data or implicit declaration.
    // This is the exact zero-population proof, not a default policy for nonempty inventories.
    pub(crate) const fn empty() -> Self {
        Self {
            declarations: Vec::new(),
            limits: LogicalInputInventoryLimits::new(0, 0),
            shape: LogicalInputInventoryShape {
                declarations: 0,
                inspections: 0,
            },
        }
    }
    // Preserve the complete actual owned vector and fixed proof fields without lookup or cloning.
    pub(crate) fn resource_parts(
        &self,
    ) -> (
        &[LogicalInputDeclaration],
        LogicalInputInventoryLimits,
        LogicalInputInventoryShape,
    ) {
        let Self {
            declarations,
            limits,
            shape,
        } = self;
        (declarations, *limits, *shape)
    }
    /// Own the complete supplied vector after checked L*L arithmetic and declaration/comparison
    /// quotas, then original-order duplicate checks. Validation allocates no workspace and reads
    /// no selected member, currentness or source value. Prior record proofs grant no aggregate
    /// held-content allowance. Input allocation is the caller's, outside these logical bounds.
    ///
    /// ```
    /// use sc_core::{name::MachineToken,ontology::EntityId,value::*};
    /// # fn main()->Result<(),Box<dyn std::error::Error>> {
    /// let record=LogicalInputDeclaration::new(LogicalInputDeclarationDefinition {
    ///     id:EntityId::from_bits(1),source:EntityId::from_bits(2),
    ///     kind:LogicalInputKind::PocketOpening,domain:vec![MachineToken::new("option_a")?],
    ///     state:LogicalInputState::Unknown {observation:EntityId::from_bits(3)},
    /// },LogicalInputLimits::new(1,0,8,2))?;
    /// let inputs=LogicalInputs::new(vec![record],LogicalInputInventoryLimits::new(1,1))?;
    /// let binding=inputs.resolve(EntityId::from_bits(1),LogicalInputKind::PocketOpening).map_err(|e|e.to_string())?;
    /// assert!(std::ptr::eq(binding.inputs(),&inputs));
    /// assert_eq!(binding.declaration().definition().state.tag(),LogicalInputStateTag::Unknown);
    /// # Ok(()) }
    /// ```
    /// # Errors
    /// Complete arithmetic, declaration then comparison quota, then first duplicate identity with
    /// both original positions. No inventory or accepted ownership prefix escapes a refusal.
    pub fn new(
        declarations: Vec<LogicalInputDeclaration>,
        limits: LogicalInputInventoryLimits,
    ) -> Result<Self, LogicalInputInventoryError> {
        use LogicalInputInventoryRefusal as R;
        let refuse = |refusal| LogicalInputInventoryError { limits, refusal };
        let count = declarations.len();
        let inspections = count
            .checked_mul(count)
            .ok_or_else(|| refuse(R::PopulationOverflow))?;
        if count > limits.declarations {
            return Err(refuse(R::Declarations {
                allowed: limits.declarations,
                required: count,
            }));
        }
        if inspections > limits.inspections {
            return Err(refuse(R::ValidationInspections {
                allowed: limits.inspections,
                required: inspections,
            }));
        }
        for (duplicate, record) in declarations.iter().enumerate() {
            if let Some(first) = declarations
                .iter()
                .take(duplicate)
                .position(|prior| prior.id() == record.id())
            {
                return Err(refuse(R::DuplicateIdentity {
                    identity: record.id(),
                    first,
                    duplicate,
                }));
            }
        }
        Ok(Self {
            declarations,
            limits,
            shape: LogicalInputInventoryShape {
                declarations: count,
                inspections,
            },
        })
    }
    /// Original declaration vector, without sorting, cloning or state/value evaluation.
    #[must_use]
    pub fn declarations(&self) -> &[LogicalInputDeclaration] {
        &self.declarations
    }
    /// Original explicit constructor policy, not a source/currentness or aggregate content grant.
    #[must_use]
    pub const fn limits(&self) -> LogicalInputInventoryLimits {
        self.limits
    }
    /// Complete original population and conservative validation-work bound.
    #[must_use]
    pub const fn shape(&self) -> LogicalInputInventoryShape {
        self.shape
    }
    /// Inspect an actual original member by owned identity, without kind or value conversion.
    #[must_use]
    pub fn declaration(&self, id: EntityId) -> Option<&LogicalInputDeclaration> {
        self.declarations.iter().find(|record| record.id() == id)
    }
    /// Bind one actual original member and its exact logical kind, without callbacks,allocation,
    /// values or a fallback declaration. Original-source consumers must still bind this inventory
    /// to their actual Root/Profile owner; equal IDs/data alone cannot establish that relation.
    ///
    /// ```
    /// use sc_core::{value::*,ontology::EntityId};
    /// fn original<'a>(inputs:&'a LogicalInputs,id:EntityId,kind:LogicalInputKind)
    ///     ->Result<&'a LogicalInputDeclaration,LogicalInputLookupError<'a>> {
    ///     let binding=inputs.resolve(id,kind)?;
    ///     assert!(std::ptr::eq(binding.inputs(),inputs));
    ///     Ok(binding.declaration())
    /// }
    /// ```
    /// # Errors
    /// Original Missing identity or wrong Kind with the actual original member loan.
    pub fn resolve(
        &self,
        id: EntityId,
        expected: LogicalInputKind,
    ) -> Result<LogicalInputBinding<'_>, LogicalInputLookupError<'_>> {
        let (index, declaration) = self
            .declarations
            .iter()
            .enumerate()
            .find(|(_, record)| record.id() == id)
            .ok_or(LogicalInputLookupError::Missing {
                declaration: id,
                expected,
            })?;
        if declaration.kind() != expected {
            return Err(LogicalInputLookupError::Kind {
                expected,
                actual: declaration,
            });
        }
        Ok(LogicalInputBinding {
            inputs: self,
            declaration,
            index,
        })
    }
}
