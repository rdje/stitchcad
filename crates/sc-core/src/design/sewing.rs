//! Actual sewing amount metadata; structural graph scope and numeric/profile resolution are separate.
use super::{DesignInputs, DesignInputsError, DesignParameter, DesignReferenceHost};
use crate::{
    ontology::{
        EaseAmount, EntityId, ProfileBindingValidation, ProfileParameterRef, SeamEasePortion,
        SeamSpan, SewingGraph,
    },
    recipe::{FormulaDeclaration, FormulaKind},
    value::CanonicalInputs,
};
use core::fmt;
use sc_units::Length;

/// Exact authored amount source, without evaluating a formula or resolving a Factory Profile.
#[derive(Clone, Copy)]
pub enum DesignSewingAmountSource<'a> {
    /// Original signed value borrowed from the actual span.
    Explicit(&'a Length),
    /// Actual Design parameter and its current canonical Length-kind declaration.
    Formula {
        /// Exact parameter metadata, not a same-number canonical declaration.
        parameter: &'a DesignParameter,
        /// Existing dedicated length or checked general scalar metadata; state remains unchanged.
        declaration: FormulaDeclaration<'a>,
    },
    /// Exact logical target-profile reference, always DeferredToG4 here.
    Profile(ProfileParameterRef),
}
impl fmt::Debug for DesignSewingAmountSource<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Explicit(_) => "Explicit",
            Self::Formula { .. } => "Formula",
            Self::Profile(_) => "Profile",
        })
    }
}

/// Borrowed actual span/canonical context plus its amount source metadata; not graph readiness.
///
/// ```compile_fail
/// fn escape<H: sc_core::design::DesignReferenceHost>(state: sc_core::design::DesignInputs<H>,
///     graph: sc_core::ontology::SewingGraph) -> Vec<sc_core::design::DesignSewingAmount<'static>> {
///     state.sewing_amount_declarations(&graph).unwrap()
/// }
/// ```
#[derive(Clone, Copy)]
pub struct DesignSewingAmount<'a> {
    span: &'a SeamSpan,
    canonical: &'a CanonicalInputs,
    source: DesignSewingAmountSource<'a>,
    portion: Option<&'a SeamEasePortion>,
}
impl fmt::Debug for DesignSewingAmount<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DesignSewingAmount")
            .field("source", &self.source)
            .finish_non_exhaustive()
    }
}
impl<'a> DesignSewingAmount<'a> {
    /// Retained original-domain allocation context;source() is its full original declaration.
    #[must_use]
    pub const fn portion(self) -> Option<&'a SeamEasePortion> {
        self.portion
    }

    /// Original actual span, without a copied amount or allocation.
    #[must_use]
    pub const fn span(self) -> &'a SeamSpan {
        self.span
    }
    /// Actual current canonical owner supplied by the containing Design inputs.
    #[must_use]
    pub const fn canonical_inputs(self) -> &'a CanonicalInputs {
        self.canonical
    }
    /// Exact explicit/formula/logical-profile source; no fallback values.
    #[must_use]
    pub const fn source(self) -> DesignSewingAmountSource<'a> {
        self.source
    }
    /// A logical profile role is visible as deferred, even when its numeric id matches an input.
    #[must_use]
    pub const fn profile_validation(self) -> Option<ProfileBindingValidation> {
        match self.source {
            DesignSewingAmountSource::Profile(_) => Some(ProfileBindingValidation::DeferredToG4),
            DesignSewingAmountSource::Explicit(_) | DesignSewingAmountSource::Formula { .. } => {
                None
            }
        }
    }
}

/// Current root or actual formula parameter metadata/kind refusal at the original span.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DesignSewingDeclarationError<E> {
    /// Original complete current root refusal, before publishing any amount views.
    Inputs(Box<DesignInputsError<E>>),
    /// No actual Design parameter metadata has the span's formula-source identity.
    MissingFormulaParameter {
        /// Original span requiring the source.
        span: EntityId,
        /// Exact authored parameter identity.
        parameter: EntityId,
    },
    /// Actual parameter metadata exists but does not declare Length kind.
    FormulaKind {
        /// Original span requiring a signed length.
        span: EntityId,
        /// Actual parameter identity.
        parameter: EntityId,
        /// Actual current declared kind, without evaluating its value.
        actual: FormulaKind,
    },
}
impl<E: fmt::Display> fmt::Display for DesignSewingDeclarationError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inputs(issue) => write!(f, "Design sewing inputs: {issue}"),
            Self::MissingFormulaParameter { span, parameter } => {
                write!(f, "sewing span {span} has no formula parameter {parameter}")
            }
            Self::FormulaKind {
                span,
                parameter,
                actual,
            } => write!(
                f,
                "sewing span {span} requires Length parameter {parameter}, found {actual:?}"
            ),
        }
    }
}
impl<E: std::error::Error + 'static> std::error::Error for DesignSewingDeclarationError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Inputs(issue) => Some(issue.as_ref()),
            Self::MissingFormulaParameter { .. } | Self::FormulaKind { .. } => None,
        }
    }
}
impl<H: DesignReferenceHost> DesignInputs<H> {
    /// Borrow every actual span's amount metadata after complete current root validation.
    ///
    /// Formula identities select actual Design parameters, then require Length kind through their
    /// original current canonical bindings. Unknown/Derived states are neither read nor evaluated.
    /// Profile references remain logical and deferred to G4. This checks metadata only, not the
    /// graph's copy/range/stop context, global graph ids, sewing geometry or numeric amount readiness.
    ///
    /// # Errors
    /// Returns original current root causes, then first missing/wrong-kind parameter in span order.
    /// No partial amount collection escapes a refusal.
    pub fn sewing_amount_declarations<'a>(
        &'a self,
        graph: &'a SewingGraph,
    ) -> Result<Vec<DesignSewingAmount<'a>>, DesignSewingDeclarationError<H::Error>> {
        self.validate_current()
            .map_err(|issue| DesignSewingDeclarationError::Inputs(Box::new(issue)))?;
        graph
            .spans()
            .iter()
            .map(|span| {
                let source = match &span.definition().ease.declaration().amount {
                    EaseAmount::Explicit(value) => DesignSewingAmountSource::Explicit(value),
                    EaseAmount::Profile(reference) => DesignSewingAmountSource::Profile(*reference),
                    EaseAmount::Formula(identity) => {
                        let parameter = self
                            .parameters()
                            .parameters()
                            .iter()
                            .find(|parameter| parameter.id() == *identity)
                            .ok_or(DesignSewingDeclarationError::MissingFormulaParameter {
                                span: span.id(),
                                parameter: *identity,
                            })?;
                        let declaration =
                            parameter.declaration(self.inputs()).map_err(|issue| {
                                DesignSewingDeclarationError::Inputs(Box::new(
                                    DesignInputsError::Parameters(Box::new(issue)),
                                ))
                            })?;
                        let actual = declaration.source().kind();
                        if actual != FormulaKind::Length {
                            return Err(DesignSewingDeclarationError::FormulaKind {
                                span: span.id(),
                                parameter: *identity,
                                actual,
                            });
                        }
                        DesignSewingAmountSource::Formula {
                            parameter,
                            declaration,
                        }
                    }
                };
                Ok(DesignSewingAmount {
                    span,
                    canonical: self.inputs(),
                    source,
                    portion: span.definition().ease.portion(),
                })
            })
            .collect()
    }
}
