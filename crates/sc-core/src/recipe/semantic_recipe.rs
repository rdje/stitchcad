//! Original ordered semantic declarations,without mixed-program positions or source execution.
use super::{ButtonholeOperation, LogicalDerivationOperation};
use crate::ontology::EntityId;
use core::fmt;

/// Implemented semantic declaration families,distinct from scalar statements and native inputs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemanticOperationKind {
    /// Derive physical buttonhole length from its canonical button origin and explicit policy.
    Buttonhole,
    /// Derive a finite logical output through an explicit rule and original typed dependencies.
    LogicalDerivation,
}
/// Exact immutable original semantic declaration,not an executed operation or native drafting slot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SemanticOperation {
    /// Original validated standalone Buttonhole declaration and complete constructor metadata.
    Buttonhole(ButtonholeOperation),
    /// Original validated standalone logical declaration and complete constructor metadata.
    LogicalDerivation(LogicalDerivationOperation),
}
impl SemanticOperation {
    /// Actual original operation identity,not a declaration/source reference or ordered position.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        match self {
            Self::Buttonhole(p) => p.id(),
            Self::LogicalDerivation(p) => p.id(),
        }
    }
    /// Actual implemented declaration family,without checking a rule/policy signature or value.
    #[must_use]
    pub const fn kind(&self) -> SemanticOperationKind {
        match self {
            Self::Buttonhole(_) => SemanticOperationKind::Buttonhole,
            Self::LogicalDerivation(_) => SemanticOperationKind::LogicalDerivation,
        }
    }
}
/// Explicit whole standalone ownership bounds,not Root content,query or runtime allowances.
/// ```compile_fail
/// let limits:sc_core::recipe::SemanticRecipeLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SemanticRecipeLimits {
    operations: usize,
    inspections: usize,
}
impl SemanticRecipeLimits {
    /// Select complete operation count and conservative N*N identity comparisons explicitly.
    /// Zero admits an explicitly empty semantic scope without an invented declaration.
    #[must_use]
    pub const fn new(operations: usize, validation_inspections: usize) -> Self {
        Self {
            operations,
            inspections: validation_inspections,
        }
    }
    /// Maximum complete original operation population,including repeated invalid owners.
    #[must_use]
    pub const fn operations(self) -> usize {
        self.operations
    }
    /// Maximum conservative complete N*N identity comparison allowance.
    #[must_use]
    pub const fn validation_inspections(self) -> usize {
        self.inspections
    }
}
/// Factory-derived original complete ownership population and comparison bound.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SemanticRecipeShape {
    operations: usize,
    inspections: usize,
}
impl SemanticRecipeShape {
    /// Complete original operation population.
    #[must_use]
    pub const fn operations(self) -> usize {
        self.operations
    }
    /// Conservative complete N*N identity comparisons,not measured execution or CPU time.
    #[must_use]
    pub const fn validation_inspections(self) -> usize {
        self.inspections
    }
}
/// First complete arithmetic/quota refusal or original cross-family ownership collision.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemanticRecipeRefusal {
    /// Complete N*N arithmetic cannot fit usize,before quotas or duplicate checks.
    PopulationOverflow,
    /// Complete actual operation count exceeds the selected maximum.
    Operations {
        /// Selected maximum.
        allowed: usize,
        /// Whole original requirement,including invalid repeats.
        required: usize,
    },
    /// Conservative complete comparison requirement exceeds its selected maximum.
    ValidationInspections {
        /// Selected maximum.
        allowed: usize,
        /// Complete actual N*N requirement.
        required: usize,
    },
    /// Two owned operations have the same identity,including identical or different-family records.
    DuplicateIdentity {
        /// Exact repeated original operation identity;references do not become owners.
        identity: EntityId,
        /// First zero-based original semantic operation position.
        first: usize,
        /// First repeated zero-based original semantic operation position.
        duplicate: usize,
    },
}
/// Selected complete constructor policy and original first allowance/ownership refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SemanticRecipeError {
    /// Explicit complete selected constructor limits,not a source/content grant.
    pub limits: SemanticRecipeLimits,
    /// Original first complete arithmetic/quota/ownership refusal.
    pub refusal: SemanticRecipeRefusal,
}
impl fmt::Display for SemanticRecipeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "semantic recipe ownership: {:?}", self.refusal)
    }
}
impl std::error::Error for SemanticRecipeError {}
/// Immutable original ordered owner of the implemented semantic operation declarations.
/// Scalar recipe statements and native inputs retain their separate actual owners. Construction
/// grants no combined-program position,Root membership,rule/policy signature,source/domain/value,
/// currentness,graph or execution proof. Referenced output/rule/input IDs remain references.
/// ```compile_fail,E0616
/// fn alter(recipe:&mut sc_core::recipe::SemanticRecipe) {recipe.operations.clear();}
/// ```
#[derive(Clone)]
pub struct SemanticRecipe {
    operations: Vec<SemanticOperation>,
    limits: SemanticRecipeLimits,
    shape: SemanticRecipeShape,
}
impl PartialEq for SemanticRecipe {
    fn eq(&self, other: &Self) -> bool {
        self.operations == other.operations
    }
}
impl Eq for SemanticRecipe {}
impl fmt::Debug for SemanticRecipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SemanticRecipe")
            .field("shape", &self.shape)
            .finish_non_exhaustive()
    }
}
/// Exact original lookup refusal;wrong-kind errors loan the actual original member.
#[derive(Clone, Copy, Debug)]
pub enum SemanticOperationLookupError<'a> {
    /// No operation owns the requested ID;references and equal-kind alternatives do not substitute.
    Missing {
        /// Original requested identity.
        operation: EntityId,
        /// Original expected semantic family,retained even without a member.
        expected: SemanticOperationKind,
    },
    /// An actual original member exists with another family.
    Kind {
        /// Original expected family.
        expected: SemanticOperationKind,
        /// Actual original declaration loan,without copied fields or an invented replacement.
        actual: &'a SemanticOperation,
    },
}
impl fmt::Display for SemanticOperationLookupError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing {
                operation,
                expected,
            } => write!(f, "semantic {expected:?} operation {operation} is absent"),
            Self::Kind { expected, actual } => write!(
                f,
                "semantic operation {} needs {expected:?},has {:?}",
                actual.id(),
                actual.kind()
            ),
        }
    }
}
impl std::error::Error for SemanticOperationLookupError<'_> {}
/// Private original owner/member association. This semantic index is not a scalar ordinal,native
/// step or mixed-program position. Binding reads no value or source and cannot replace its owner.
/// ```compile_fail
/// fn escape(recipe:sc_core::recipe::SemanticRecipe,id:sc_core::ontology::EntityId)
///     ->sc_core::recipe::SemanticOperationBinding<'static> {
///     recipe.resolve(id,sc_core::recipe::SemanticOperationKind::Buttonhole).unwrap()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct SemanticOperationBinding<'a> {
    recipe: &'a SemanticRecipe,
    operation: &'a SemanticOperation,
    index: usize,
}
impl<'a> SemanticOperationBinding<'a> {
    /// Actual supplied original semantic recipe owner.
    #[must_use]
    pub const fn recipe(self) -> &'a SemanticRecipe {
        self.recipe
    }
    /// Actual exact original operation declaration,not a copied definition.
    #[must_use]
    pub const fn operation(self) -> &'a SemanticOperation {
        self.operation
    }
    /// Original zero-based semantic-vector index,without a mixed-program or creator-order proof.
    #[must_use]
    pub const fn index(self) -> usize {
        self.index
    }
}
impl SemanticRecipe {
    /// Explicitly select an empty semantic cohort with exact zero operation/comparison allowances.
    /// This invents no declaration or default for nonempty data. Root construction still requires
    /// the caller to supply this owner explicitly; no omitted field is filled automatically.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            operations: Vec::new(),
            limits: SemanticRecipeLimits::new(0, 0),
            shape: SemanticRecipeShape {
                operations: 0,
                inspections: 0,
            },
        }
    }
    /// Move exact original operations after complete N*N arithmetic,count then comparison quotas,
    /// then first duplicate identity in authored order. Validation uses borrowed prefix scans and
    /// allocates no workspace. Prior member proofs grant no whole-source content credit. Input
    /// allocation is caller-owned;logical counts/comparisons do not bound allocator capacity or CPU.
    /// # Errors
    /// Whole arithmetic/first quota refusal,then DuplicateIdentity with both original positions.
    pub fn new(
        operations: Vec<SemanticOperation>,
        limits: SemanticRecipeLimits,
    ) -> Result<Self, SemanticRecipeError> {
        use SemanticRecipeRefusal as R;
        let refuse = |refusal| SemanticRecipeError { limits, refusal };
        let count = operations.len();
        let inspections = count
            .checked_mul(count)
            .ok_or_else(|| refuse(R::PopulationOverflow))?;
        if count > limits.operations {
            return Err(refuse(R::Operations {
                allowed: limits.operations,
                required: count,
            }));
        }
        if inspections > limits.inspections {
            return Err(refuse(R::ValidationInspections {
                allowed: limits.inspections,
                required: inspections,
            }));
        }
        for (duplicate, operation) in operations.iter().enumerate() {
            if let Some(first) = operations
                .iter()
                .take(duplicate)
                .position(|prior| prior.id() == operation.id())
            {
                return Err(refuse(R::DuplicateIdentity {
                    identity: operation.id(),
                    first,
                    duplicate,
                }));
            }
        }
        Ok(Self {
            operations,
            limits,
            shape: SemanticRecipeShape {
                operations: count,
                inspections,
            },
        })
    }
    /// Complete original readonly semantic sequence,without sorting,cloning or source evaluation.
    #[must_use]
    pub fn operations(&self) -> &[SemanticOperation] {
        &self.operations
    }
    /// Actual original operation by owned identity;referenced rule/output/input IDs are not members.
    #[must_use]
    pub fn operation(&self, id: EntityId) -> Option<&SemanticOperation> {
        self.operations.iter().find(|p| p.id() == id)
    }
    /// Original explicit selected constructor limits,not an aggregate content/source policy.
    #[must_use]
    pub const fn limits(&self) -> SemanticRecipeLimits {
        self.limits
    }
    /// Complete original population and conservative ownership-comparison requirement.
    #[must_use]
    pub const fn shape(&self) -> SemanticRecipeShape {
        self.shape
    }
    /// Bind the exact actual original member and expected family,without values,sources or allocation.
    /// Root/source consumers must separately bind this owner to their actual context;equal data does
    /// not supply that association. Lookup is a borrowed linear scan,no cached snapshot is introduced.
    /// # Errors
    /// Original Missing ID/family,or Kind retaining the actual original member loan.
    pub fn resolve(
        &self,
        id: EntityId,
        expected: SemanticOperationKind,
    ) -> Result<SemanticOperationBinding<'_>, SemanticOperationLookupError<'_>> {
        let (index, operation) = self
            .operations
            .iter()
            .enumerate()
            .find(|(_, p)| p.id() == id)
            .ok_or(SemanticOperationLookupError::Missing {
                operation: id,
                expected,
            })?;
        if operation.kind() != expected {
            return Err(SemanticOperationLookupError::Kind {
                expected,
                actual: operation,
            });
        }
        Ok(SemanticOperationBinding {
            recipe: self,
            operation,
            index,
        })
    }
    pub(crate) fn resource_parts(
        &self,
    ) -> (
        &[SemanticOperation],
        SemanticRecipeLimits,
        SemanticRecipeShape,
    ) {
        let Self {
            operations,
            limits,
            shape,
        } = self;
        (operations, *limits, *shape)
    }
}
