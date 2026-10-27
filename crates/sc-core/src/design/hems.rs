//! Actual Hem depth/fold declarations; numeric, logical-domain and physical validation stay separate.
use super::{DesignInputs, DesignInputsError, DesignParameter, DesignReferenceHost};
use crate::{
    ontology::{
        EntityId, Hem, HemDepth, HemFoldType, HemInventory, ProfileBindingValidation,
        ProfileParameterRef,
    },
    recipe::{FormulaDeclaration, FormulaKind},
    value::CanonicalInputs,
};
use core::fmt;
use sc_units::Length;

/// Original Hem field role, in depth-then-fold order for each authored Hem.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DesignHemDeclarationRole {
    /// Nonnegative depth intent, with authored zero distinct from missing input.
    Depth,
    /// Logical fold-type intent; its recipe/Profile owner supplies domain, selection and state.
    FoldType,
}
/// Original borrowed intent or actual current Length-kind metadata, never a computed fold/depth.
#[derive(Clone, Copy)]
pub enum DesignHemDeclarationSource<'a> {
    /// Original inline parameter and nonnegative length intent, without a second canonical value.
    ExplicitDepth {
        /// Original authored parameter identity; global identity joins remain separate.
        parameter: &'a EntityId,
        /// Original authored intent, not a canonical input read or evaluation certificate.
        value: &'a Length,
    },
    /// Actual Design parameter and its current canonical Length-kind declaration.
    FormulaDepth {
        /// Exact parameter metadata, not a same-number canonical input.
        parameter: &'a DesignParameter,
        /// Current dedicated/general Length metadata, with canonical state unchanged.
        declaration: FormulaDeclaration<'a>,
    },
    /// Original logical profile depth, deferred to G4.
    ProfileDepth(&'a ProfileParameterRef),
    /// Original logical fold declaration, without an inferred physical vocabulary or default.
    FoldType(&'a HemFoldType),
}
impl fmt::Debug for DesignHemDeclarationSource<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::ExplicitDepth { .. } => "ExplicitDepth",
            Self::FormulaDepth { .. } => "FormulaDepth",
            Self::ProfileDepth(_) => "ProfileDepth",
            Self::FoldType(_) => "FoldType",
        })
    }
}

/// Actual Hem/current canonical owner and one borrowed field role; not current garment readiness.
///
/// ```compile_fail
/// fn escape<H: sc_core::design::DesignReferenceHost>(state: sc_core::design::DesignInputs<H>,
///     hems: sc_core::ontology::HemInventory)
///     -> Vec<sc_core::design::DesignHemDeclaration<'static>> {
///     state.hem_declarations(&hems).unwrap()
/// }
/// ```
#[derive(Clone, Copy)]
pub struct DesignHemDeclaration<'a> {
    hem: &'a Hem,
    canonical: &'a CanonicalInputs,
    role: DesignHemDeclarationRole,
    source: DesignHemDeclarationSource<'a>,
}
impl fmt::Debug for DesignHemDeclaration<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DesignHemDeclaration")
            .field("role", &self.role)
            .field("source", &self.source)
            .finish_non_exhaustive()
    }
}
impl<'a> DesignHemDeclaration<'a> {
    /// Original actual Hem, without copied depth/fold/Facing intent.
    #[must_use]
    pub const fn hem(self) -> &'a Hem {
        self.hem
    }
    /// Actual single canonical owner used for formula metadata checking.
    #[must_use]
    pub const fn canonical_inputs(self) -> &'a CanonicalInputs {
        self.canonical
    }
    /// Exact field role, independent of numeric identity or selected value.
    #[must_use]
    pub const fn role(self) -> DesignHemDeclarationRole {
        self.role
    }
    /// Original intent or checked formula metadata, without evaluating or defaulting a Hem value.
    #[must_use]
    pub const fn source(self) -> DesignHemDeclarationSource<'a> {
        self.source
    }
    /// Present logical Profile fields remain deferred; None only means this role has no such field.
    #[must_use]
    pub const fn profile_validation(self) -> Option<ProfileBindingValidation> {
        match self.source {
            DesignHemDeclarationSource::ProfileDepth(_)
            | DesignHemDeclarationSource::FoldType(HemFoldType::Profile(_)) => {
                Some(ProfileBindingValidation::DeferredToG4)
            }
            _ => None,
        }
    }
}

/// Original current root or actual Hem formula-metadata refusal, without a partial view collection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DesignHemDeclarationError<E> {
    /// Original whole current root cause, before publishing any role views.
    Inputs(Box<DesignInputsError<E>>),
    /// Formula depth identity names no actual Design parameter metadata.
    MissingFormulaParameter {
        /// Actual Hem requiring the source.
        hem: EntityId,
        /// Exact original parameter identity.
        parameter: EntityId,
    },
    /// Actual current parameter metadata does not have Length kind.
    FormulaKind {
        /// Actual Hem requiring nonnegative depth intent.
        hem: EntityId,
        /// Actual parameter identity.
        parameter: EntityId,
        /// Original current declared kind, without reading its value.
        actual: FormulaKind,
    },
}
impl<E: fmt::Display> fmt::Display for DesignHemDeclarationError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inputs(issue) => write!(f, "Hem declarations current inputs: {issue}"),
            Self::MissingFormulaParameter { hem, parameter } => {
                write!(f, "Hem {hem} has no formula parameter {parameter}")
            }
            Self::FormulaKind {
                hem,
                parameter,
                actual,
            } => write!(
                f,
                "Hem {hem} requires Length parameter {parameter}, found {actual:?}"
            ),
        }
    }
}
impl<E: std::error::Error + 'static> std::error::Error for DesignHemDeclarationError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Inputs(issue) => Some(issue.as_ref()),
            Self::MissingFormulaParameter { .. } | Self::FormulaKind { .. } => None,
        }
    }
}
impl<H: DesignReferenceHost> DesignInputs<H> {
    /// Borrow every actual Hem's depth/fold roles after complete current root validation.
    ///
    /// Formula depths select actual Design parameter metadata, then require Length kind through its
    /// current canonical binding. Unknown/Derived values remain unread. Inline fields retain original
    /// intent; logical fold domains, profile resolution and nonnegative evaluated depths retain their
    /// existing recipe/G2/G3/G4 owners. This checks metadata, not Hem/layer geometry, Facing targets,
    /// global garment ids or physical/release readiness. No partial view collection escapes a refusal.
    ///
    /// # Errors
    /// Returns original current root causes, then first missing/wrong-kind formula parameter in order.
    pub fn hem_declarations<'a>(
        &'a self,
        hems: &'a HemInventory,
    ) -> Result<Vec<DesignHemDeclaration<'a>>, DesignHemDeclarationError<H::Error>> {
        self.validate_current()
            .map_err(|issue| DesignHemDeclarationError::Inputs(Box::new(issue)))?;
        let mut views = Vec::new();
        for hem in hems.hems() {
            let source = match &hem.definition().depth {
                HemDepth::Explicit { parameter, value } => {
                    DesignHemDeclarationSource::ExplicitDepth { parameter, value }
                }
                HemDepth::Profile(reference) => DesignHemDeclarationSource::ProfileDepth(reference),
                HemDepth::Formula(identity) => {
                    let parameter = self.parameters().parameter(*identity).ok_or(
                        DesignHemDeclarationError::MissingFormulaParameter {
                            hem: hem.id(),
                            parameter: *identity,
                        },
                    )?;
                    let declaration = parameter.declaration(self.inputs()).map_err(|issue| {
                        DesignHemDeclarationError::Inputs(Box::new(DesignInputsError::Parameters(
                            Box::new(issue),
                        )))
                    })?;
                    let actual = declaration.kind();
                    if actual != FormulaKind::Length {
                        return Err(DesignHemDeclarationError::FormulaKind {
                            hem: hem.id(),
                            parameter: *identity,
                            actual,
                        });
                    }
                    DesignHemDeclarationSource::FormulaDepth {
                        parameter,
                        declaration,
                    }
                }
            };
            views.push(DesignHemDeclaration {
                hem,
                canonical: self.inputs(),
                role: DesignHemDeclarationRole::Depth,
                source,
            });
            views.push(DesignHemDeclaration {
                hem,
                canonical: self.inputs(),
                role: DesignHemDeclarationRole::FoldType,
                source: DesignHemDeclarationSource::FoldType(&hem.definition().fold_type),
            });
        }
        Ok(views)
    }
}
