//! Actual construction intent and checked formula metadata; execution/source readiness is separate.
use super::{DesignInputs, DesignInputsError, DesignParameter, DesignReferenceHost};
use crate::{
    ontology::{
        ConstructionInventory, ConstructionInventoryPosition, ConstructionRef, EntityId,
        IntakeAmount, ProfileBindingValidation, ProfileParameterRef,
    },
    recipe::{FormulaDeclaration, FormulaKind},
    value::CanonicalInputs,
};
use core::fmt;
use sc_units::Length;

/// Exact original field role, independent of identity spelling or numeric availability.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DesignConstructionDeclarationRole {
    /// Dart/Tuck/Pleat intake intent.
    Intake,
    /// Required logical closing-operation identity, without operation lookup or scalar conversion.
    ClosingOperation,
}
/// Actual borrowed intent or checked current parameter metadata; no evaluated construction values.
#[derive(Clone, Copy)]
pub enum DesignConstructionDeclarationSource<'a> {
    /// Original inline parameter id and authored nonnegative intake, including explicit zero.
    ExplicitIntake {
        /// Original inline identity; this does not invent Design parameter metadata.
        parameter: &'a EntityId,
        /// Exact original intent, not another canonical value.
        value: &'a Length,
    },
    /// Actual Design parameter and current canonical Length-kind metadata, without reading its state.
    FormulaIntake {
        /// Original actual Design metadata, not a same-number bare canonical record.
        parameter: &'a DesignParameter,
        /// Actual dedicated/general Length declaration with its original state intact.
        declaration: FormulaDeclaration<'a>,
    },
    /// Original logical Profile intake, deferred to G4 even if its id matches a numeric input.
    ProfileIntake(&'a ProfileParameterRef),
    /// Exact original operation intent; recipe existence/kind/dependencies remain separate.
    ClosingOperation(&'a EntityId),
}
impl fmt::Debug for DesignConstructionDeclarationSource<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::ExplicitIntake { .. } => "ExplicitIntake",
            Self::FormulaIntake { .. } => "FormulaIntake",
            Self::ProfileIntake(_) => "ProfileIntake",
            Self::ClosingOperation(_) => "ClosingOperation",
        })
    }
}
/// Original actual construction/current canonical owner and one unexecuted field role.
///
/// ```compile_fail
/// fn escape<H: sc_core::design::DesignReferenceHost>(state: sc_core::design::DesignInputs<H>,
///     constructions: sc_core::ontology::ConstructionInventory)
///     -> Vec<sc_core::design::DesignConstructionDeclaration<'static>> {
///     state.construction_intake_declarations(&constructions).unwrap()
/// }
/// ```
#[derive(Clone, Copy)]
pub struct DesignConstructionDeclaration<'a> {
    construction: ConstructionRef<'a>,
    position: ConstructionInventoryPosition,
    canonical: &'a CanonicalInputs,
    role: DesignConstructionDeclarationRole,
    source: DesignConstructionDeclarationSource<'a>,
}
impl fmt::Debug for DesignConstructionDeclaration<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DesignConstructionDeclaration")
            .field("kind", &self.construction.kind())
            .field("role", &self.role)
            .field("source", &self.source)
            .finish_non_exhaustive()
    }
}
impl<'a> DesignConstructionDeclaration<'a> {
    /// Exact distinct stored source, without copied content or a source-validity certificate.
    #[must_use]
    pub const fn construction(self) -> ConstructionRef<'a> {
        self.construction
    }
    /// Original family and zero-based authored index, without renumbering filtered roles.
    #[must_use]
    pub const fn position(self) -> ConstructionInventoryPosition {
        self.position
    }
    /// Actual containing root's single current canonical owner.
    #[must_use]
    pub const fn canonical_inputs(self) -> &'a CanonicalInputs {
        self.canonical
    }
    /// Original semantic field role.
    #[must_use]
    pub const fn role(self) -> DesignConstructionDeclarationRole {
        self.role
    }
    /// Exact intent or current typed formula metadata, without reading/evaluating/defaulting values.
    #[must_use]
    pub const fn source(self) -> DesignConstructionDeclarationSource<'a> {
        self.source
    }
    /// Present Profile roles stay deferred; None means only that this role has no Profile field.
    #[must_use]
    pub const fn profile_validation(self) -> Option<ProfileBindingValidation> {
        match self.source {
            DesignConstructionDeclarationSource::ProfileIntake(_) => {
                Some(ProfileBindingValidation::DeferredToG4)
            }
            _ => None,
        }
    }
}
/// Original current root or contextual formula-metadata refusal; no partial views escape.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DesignConstructionDeclarationError<E> {
    /// Whole current root's original concrete cause, before even empty declaration scope.
    Inputs(Box<DesignInputsError<E>>),
    /// Formula identity names no actual Design parameter metadata.
    MissingFormulaParameter {
        /// Actual construction requiring the declaration.
        construction: EntityId,
        /// Original family and authored index.
        position: ConstructionInventoryPosition,
        /// Original semantic field role.
        role: DesignConstructionDeclarationRole,
        /// Actual held formula identity.
        parameter: EntityId,
    },
    /// Actual current parameter declaration has an incompatible kind.
    FormulaKind {
        /// Actual construction requiring the declaration.
        construction: EntityId,
        /// Original family and authored index.
        position: ConstructionInventoryPosition,
        /// Original semantic field role.
        role: DesignConstructionDeclarationRole,
        /// Actual held formula identity.
        parameter: EntityId,
        /// Required semantic kind, independent of available values.
        expected: FormulaKind,
        /// Actual current declared kind, without reading its value.
        actual: FormulaKind,
    },
}
impl<E: fmt::Display> fmt::Display for DesignConstructionDeclarationError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inputs(issue) => write!(f, "construction declarations current inputs: {issue}"),
            Self::MissingFormulaParameter { construction, position, role, parameter } => write!(
                f, "construction {construction} at {position:?} {role:?} has no formula parameter {parameter}"
            ),
            Self::FormulaKind { construction, position, role, parameter, expected, actual } => write!(
                f, "construction {construction} at {position:?} {role:?} requires {expected:?} parameter {parameter}, found {actual:?}"
            ),
        }
    }
}
impl<E: std::error::Error + 'static> std::error::Error for DesignConstructionDeclarationError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Inputs(issue) => Some(issue.as_ref()),
            Self::MissingFormulaParameter { .. } | Self::FormulaKind { .. } => None,
        }
    }
}
impl<H: DesignReferenceHost> DesignInputs<H> {
    /// Borrow Dart/Tuck/Pleat intake then closing intent in original family/authored order.
    ///
    /// Validates the complete current root first, even when these families are empty. Formula intake
    /// requires actual Design parameter metadata and its current Length-kind canonical binding;
    /// Unknown/Derived states remain unread. Explicit and logical Profile/operation origins stay
    /// borrowed unchanged. Gather ease belongs to its existing graph span and is not copied here.
    /// Current construction sources, operation existence/kind/dependencies, global garment identities,
    /// nonnegative evaluated intake, conservation and physical/release readiness remain separate.
    /// # Errors
    /// Returns the original current-root cause, then first missing/wrong-kind parameter in role order.
    /// No partial view collection is returned on failure.
    pub fn construction_intake_declarations<'a>(
        &'a self,
        constructions: &'a ConstructionInventory,
    ) -> Result<Vec<DesignConstructionDeclaration<'a>>, DesignConstructionDeclarationError<H::Error>>
    {
        self.validate_current()
            .map_err(|issue| DesignConstructionDeclarationError::Inputs(Box::new(issue)))?;
        let mut views = Vec::new();
        for (position, construction) in constructions.entries() {
            let (intake, closing) = match construction {
                ConstructionRef::Dart(value) => (
                    &value.definition().intake,
                    &value.definition().closing_operation,
                ),
                ConstructionRef::Tuck(value) => (
                    &value.definition().intake,
                    &value.definition().closing_operation,
                ),
                ConstructionRef::Pleat(value) => (
                    &value.definition().intake,
                    &value.definition().closing_operation,
                ),
                ConstructionRef::Gather(_)
                | ConstructionRef::Grainline(_)
                | ConstructionRef::SeamAllowance(_) => continue,
            };
            let source = match intake {
                IntakeAmount::Explicit { parameter, value } => {
                    DesignConstructionDeclarationSource::ExplicitIntake { parameter, value }
                }
                IntakeAmount::Profile(reference) => {
                    DesignConstructionDeclarationSource::ProfileIntake(reference)
                }
                IntakeAmount::Formula(identity) => {
                    let parameter = self.parameters().parameter(*identity).ok_or(
                        DesignConstructionDeclarationError::MissingFormulaParameter {
                            construction: construction.id(),
                            position,
                            role: DesignConstructionDeclarationRole::Intake,
                            parameter: *identity,
                        },
                    )?;
                    let declaration = parameter.declaration(self.inputs()).map_err(|issue| {
                        DesignConstructionDeclarationError::Inputs(Box::new(
                            DesignInputsError::Parameters(Box::new(issue)),
                        ))
                    })?;
                    let actual = declaration.kind();
                    if actual != FormulaKind::Length {
                        return Err(DesignConstructionDeclarationError::FormulaKind {
                            construction: construction.id(),
                            position,
                            role: DesignConstructionDeclarationRole::Intake,
                            parameter: *identity,
                            expected: FormulaKind::Length,
                            actual,
                        });
                    }
                    DesignConstructionDeclarationSource::FormulaIntake {
                        parameter,
                        declaration,
                    }
                }
            };
            views.push(DesignConstructionDeclaration {
                construction,
                position,
                canonical: self.inputs(),
                role: DesignConstructionDeclarationRole::Intake,
                source,
            });
            views.push(DesignConstructionDeclaration {
                construction,
                position,
                canonical: self.inputs(),
                role: DesignConstructionDeclarationRole::ClosingOperation,
                source: DesignConstructionDeclarationSource::ClosingOperation(closing),
            });
        }
        Ok(views)
    }
}
