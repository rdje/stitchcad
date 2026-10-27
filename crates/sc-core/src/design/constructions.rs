//! Actual construction intent and checked formula metadata; execution/source readiness is separate.
use super::{DesignInputs, DesignInputsError, DesignParameter, DesignReferenceHost};
use crate::{
    ontology::{
        AllowanceWidth, ConstructionInventory, ConstructionInventoryPosition, ConstructionRef,
        DirectedRange, EntityId, GrainAlignment, GrainAngle, IntakeAmount,
        ProfileBindingValidation, ProfileParameterRef,
    },
    recipe::{FormulaDeclaration, FormulaKind},
    value::CanonicalInputs,
};
use core::fmt;
use sc_units::{Angle, Length};

/// Exact original field role, independent of identity spelling or numeric availability.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DesignConstructionDeclarationRole {
    /// Dart/Tuck/Pleat intake intent.
    Intake,
    /// Required logical closing-operation identity, without operation lookup or scalar conversion.
    ClosingOperation,
    /// Original parallel or angular grain alignment; no implicit zero angle is generated.
    GrainAlignment,
    /// Original nonnegative allowance width intent.
    AllowanceWidth,
    /// Required logical per-profile allowance inclusion policy.
    AllowanceInclusion,
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
    /// Original parallel directed axis, without inventing an explicit angular value.
    ParallelAlignment(&'a DirectedRange),
    /// Original inline angle, without computing alignment geometry.
    ExplicitAngle(&'a Angle),
    /// Actual Design parameter and current canonical Angle-kind metadata.
    FormulaAngle {
        /// Original actual Design metadata.
        parameter: &'a DesignParameter,
        /// Actual Angle declaration with its original state intact.
        declaration: FormulaDeclaration<'a>,
    },
    /// Original logical Profile angle, deferred to G4.
    ProfileAngle(&'a ProfileParameterRef),
    /// Original inline parameter id and nonnegative allowance width, including authored zero.
    ExplicitWidth {
        /// Original inline identity, without inventing metadata.
        parameter: &'a EntityId,
        /// Exact original Length intent.
        value: &'a Length,
    },
    /// Actual Design parameter and current canonical Length-kind width metadata.
    FormulaWidth {
        /// Original actual Design metadata.
        parameter: &'a DesignParameter,
        /// Actual dedicated/general Length declaration, without reading values.
        declaration: FormulaDeclaration<'a>,
    },
    /// Original logical Profile width, deferred to G4.
    ProfileWidth(&'a ProfileParameterRef),
    /// Original required logical Profile policy, never a scalar or selected inclusion flag.
    AllowanceInclusion(&'a ProfileParameterRef),
}
impl fmt::Debug for DesignConstructionDeclarationSource<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::ExplicitIntake { .. } => "ExplicitIntake",
            Self::FormulaIntake { .. } => "FormulaIntake",
            Self::ProfileIntake(_) => "ProfileIntake",
            Self::ClosingOperation(_) => "ClosingOperation",
            Self::ParallelAlignment(_) => "ParallelAlignment",
            Self::ExplicitAngle(_) => "ExplicitAngle",
            Self::FormulaAngle { .. } => "FormulaAngle",
            Self::ProfileAngle(_) => "ProfileAngle",
            Self::ExplicitWidth { .. } => "ExplicitWidth",
            Self::FormulaWidth { .. } => "FormulaWidth",
            Self::ProfileWidth(_) => "ProfileWidth",
            Self::AllowanceInclusion(_) => "AllowanceInclusion",
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
            DesignConstructionDeclarationSource::ProfileIntake(_)
            | DesignConstructionDeclarationSource::ProfileAngle(_)
            | DesignConstructionDeclarationSource::ProfileWidth(_)
            | DesignConstructionDeclarationSource::AllowanceInclusion(_) => {
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
                    let (parameter, declaration) = self.construction_formula_declaration(
                        construction,
                        position,
                        DesignConstructionDeclarationRole::Intake,
                        *identity,
                        FormulaKind::Length,
                    )?;
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

    /// Resolve actual current metadata with the original contextual role, without a value read.
    fn construction_formula_declaration<'a>(
        &'a self,
        construction: ConstructionRef<'_>,
        position: ConstructionInventoryPosition,
        role: DesignConstructionDeclarationRole,
        identity: EntityId,
        expected: FormulaKind,
    ) -> Result<
        (&'a DesignParameter, FormulaDeclaration<'a>),
        DesignConstructionDeclarationError<H::Error>,
    > {
        let parameter = self.parameters().parameter(identity).ok_or(
            DesignConstructionDeclarationError::MissingFormulaParameter {
                construction: construction.id(),
                position,
                role,
                parameter: identity,
            },
        )?;
        let declaration = parameter.declaration(self.inputs()).map_err(|issue| {
            DesignConstructionDeclarationError::Inputs(Box::new(DesignInputsError::Parameters(
                Box::new(issue),
            )))
        })?;
        let actual = declaration.kind();
        if actual != expected {
            return Err(DesignConstructionDeclarationError::FormulaKind {
                construction: construction.id(),
                position,
                role,
                parameter: identity,
                expected,
                actual,
            });
        }
        Ok((parameter, declaration))
    }

    /// Borrow grain alignment, then allowance width/inclusion in original family/authored order.
    ///
    /// Complete current root validation precedes even empty scope. Parallel alignment retains its
    /// actual directed axis and creates no angular value. Formula Angle/Length sources require actual
    /// Design parameter/current canonical metadata of the exact kind, without reading Unknown/Derived
    /// states. Profile fields stay logical and DeferredToG4; corner and directed-axis intent remain
    /// available on the actual object. Current sources, global identity, resolved width domains,
    /// geometric alignment/offsets, profile policies and physical/release proof remain separate.
    /// # Errors
    /// Returns the original current-root cause, then first missing/wrong-kind formula parameter.
    /// No partial collection escapes a refusal.
    /// ```compile_fail
    /// fn escape<H: sc_core::design::DesignReferenceHost>(state: sc_core::design::DesignInputs<H>,
    ///     sources: sc_core::ontology::ConstructionInventory)
    ///     -> Vec<sc_core::design::DesignConstructionDeclaration<'static>> {
    ///     state.grain_allowance_declarations(&sources).unwrap()
    /// }
    /// ```
    pub fn grain_allowance_declarations<'a>(
        &'a self,
        constructions: &'a ConstructionInventory,
    ) -> Result<Vec<DesignConstructionDeclaration<'a>>, DesignConstructionDeclarationError<H::Error>>
    {
        self.validate_current()
            .map_err(|issue| DesignConstructionDeclarationError::Inputs(Box::new(issue)))?;
        let mut views = Vec::new();
        for (position, construction) in constructions.entries() {
            let (role, source) = match construction {
                ConstructionRef::Grainline(grain) => {
                    let source = match &grain.definition().alignment {
                        GrainAlignment::ParallelTo(axis) => {
                            DesignConstructionDeclarationSource::ParallelAlignment(axis)
                        }
                        GrainAlignment::AtAngle { angle, .. } => match angle {
                            GrainAngle::Explicit(value) => {
                                DesignConstructionDeclarationSource::ExplicitAngle(value)
                            }
                            GrainAngle::Profile(reference) => {
                                DesignConstructionDeclarationSource::ProfileAngle(reference)
                            }
                            GrainAngle::Formula(identity) => {
                                let (parameter, declaration) = self
                                    .construction_formula_declaration(
                                        construction,
                                        position,
                                        DesignConstructionDeclarationRole::GrainAlignment,
                                        *identity,
                                        FormulaKind::Angle,
                                    )?;
                                DesignConstructionDeclarationSource::FormulaAngle {
                                    parameter,
                                    declaration,
                                }
                            }
                        },
                    };
                    (DesignConstructionDeclarationRole::GrainAlignment, source)
                }
                ConstructionRef::SeamAllowance(allowance) => {
                    let source = match &allowance.definition().width {
                        AllowanceWidth::Explicit { parameter, value } => {
                            DesignConstructionDeclarationSource::ExplicitWidth { parameter, value }
                        }
                        AllowanceWidth::Profile(reference) => {
                            DesignConstructionDeclarationSource::ProfileWidth(reference)
                        }
                        AllowanceWidth::Formula(identity) => {
                            let (parameter, declaration) = self.construction_formula_declaration(
                                construction,
                                position,
                                DesignConstructionDeclarationRole::AllowanceWidth,
                                *identity,
                                FormulaKind::Length,
                            )?;
                            DesignConstructionDeclarationSource::FormulaWidth {
                                parameter,
                                declaration,
                            }
                        }
                    };
                    views.push(DesignConstructionDeclaration {
                        construction,
                        position,
                        canonical: self.inputs(),
                        role: DesignConstructionDeclarationRole::AllowanceWidth,
                        source,
                    });
                    (
                        DesignConstructionDeclarationRole::AllowanceInclusion,
                        DesignConstructionDeclarationSource::AllowanceInclusion(
                            &allowance.definition().inclusion,
                        ),
                    )
                }
                _ => continue,
            };
            views.push(DesignConstructionDeclaration {
                construction,
                position,
                canonical: self.inputs(),
                role,
                source,
            });
        }
        Ok(views)
    }
}
