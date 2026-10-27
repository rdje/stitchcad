//! Actual Closure declaration roles; numeric, logical-domain and operation execution stay separate.
use super::{DesignInputs, DesignInputsError, DesignParameter, DesignReferenceHost};
use crate::{
    ontology::{
        ButtonholeLengthSource, ButtonholeValidation, Closure, ClosureEnvelopeError,
        ClosureInventory, ClosureKind, ClosureLength, EntityId, NotionSize,
        ProfileBindingValidation, ProfileParameterRef,
    },
    recipe::{FormulaDeclaration, FormulaKind},
    value::CanonicalInputs,
};
use core::fmt;
use sc_units::Length;

/// Exact field role in deterministic authored Closure order; no inferred hardware vocabulary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DesignClosureDeclarationRole {
    /// Positive zipper-length intent.
    ZipperLength,
    /// Logical hook-size intent.
    HookSize,
    /// Logical bar-size intent, following the hook.
    BarSize,
    /// Logical canonical button-size intent.
    ButtonSize,
    /// One canonical button-size/operation source, following that button's size.
    Buttonhole,
}
/// Borrowed original intent or actual current Length-kind metadata; never a computed result.
#[derive(Clone, Copy)]
pub enum DesignClosureDeclarationSource<'a> {
    /// Original inline parameter identity and positive authored length, without a second value.
    ExplicitLength {
        /// Original authored parameter identity; global identity joins remain separate.
        parameter: &'a EntityId,
        /// Original Closure's length intent, not a canonical input read or evaluation certificate.
        value: &'a Length,
    },
    /// Actual Design parameter and its current canonical Length-kind declaration.
    FormulaLength {
        /// Exact parameter metadata, not a same-number canonical input.
        parameter: &'a DesignParameter,
        /// Current metadata from its existing canonical target; state is not read.
        declaration: FormulaDeclaration<'a>,
    },
    /// Original logical profile zipper-length role, deferred to G4.
    ProfileLength(&'a ProfileParameterRef),
    /// Original logical hardware size; recipe/Profile owns its domain, value and state.
    Size(&'a NotionSize),
    /// Existing canonical button-size/operation view, without an independently authored hole length.
    Buttonhole(ButtonholeLengthSource<'a>),
}
impl fmt::Debug for DesignClosureDeclarationSource<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::ExplicitLength { .. } => "ExplicitLength",
            Self::FormulaLength { .. } => "FormulaLength",
            Self::ProfileLength(_) => "ProfileLength",
            Self::Size(_) => "Size",
            Self::Buttonhole(_) => "Buttonhole",
        })
    }
}

/// Actual Closure/current canonical owner and one borrowed field role; not whole garment readiness.
///
/// ```compile_fail
/// fn escape<H: sc_core::design::DesignReferenceHost>(state: sc_core::design::DesignInputs<H>,
///     closures: sc_core::ontology::ClosureInventory)
///     -> Vec<sc_core::design::DesignClosureDeclaration<'static>> {
///     state.closure_declarations(&closures).unwrap()
/// }
/// ```
#[derive(Clone, Copy)]
pub struct DesignClosureDeclaration<'a> {
    closure: &'a Closure,
    canonical: &'a CanonicalInputs,
    role: DesignClosureDeclarationRole,
    source: DesignClosureDeclarationSource<'a>,
}
impl fmt::Debug for DesignClosureDeclaration<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DesignClosureDeclaration")
            .field("role", &self.role)
            .field("source", &self.source)
            .finish_non_exhaustive()
    }
}
impl<'a> DesignClosureDeclaration<'a> {
    /// Original actual Closure; no copied instance, size or source definition.
    #[must_use]
    pub const fn closure(self) -> &'a Closure {
        self.closure
    }
    /// Actual single canonical input owner used for formula metadata checking.
    #[must_use]
    pub const fn canonical_inputs(self) -> &'a CanonicalInputs {
        self.canonical
    }
    /// Exact field's semantic role, independent of numeric id or source value.
    #[must_use]
    pub const fn role(self) -> DesignClosureDeclarationRole {
        self.role
    }
    /// Original inline/logical intent or actual checked formula metadata, without defaulting.
    #[must_use]
    pub const fn source(self) -> DesignClosureDeclarationSource<'a> {
        self.source
    }
    /// Present profile fields remain deferred; None only means this role has no profile binding.
    #[must_use]
    pub const fn profile_validation(self) -> Option<ProfileBindingValidation> {
        match self.source {
            DesignClosureDeclarationSource::ProfileLength(_)
            | DesignClosureDeclarationSource::Size(NotionSize::Profile(_)) => {
                Some(ProfileBindingValidation::DeferredToG4)
            }
            DesignClosureDeclarationSource::Buttonhole(source) => match source.button_size() {
                NotionSize::Profile(_) => Some(ProfileBindingValidation::DeferredToG4),
                NotionSize::Declaration(_) => None,
            },
            _ => None,
        }
    }
    /// Only the canonical hole-source role exposes its separate unexecuted G3 derivation state.
    #[must_use]
    pub const fn buttonhole_validation(self) -> Option<ButtonholeValidation> {
        match self.source {
            DesignClosureDeclarationSource::Buttonhole(source) => Some(source.validation()),
            _ => None,
        }
    }
}

/// Original current root or actual zipper formula-metadata refusal, with no partial view result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DesignClosureDeclarationError<E> {
    /// Original whole current root cause, before any declaration views escape.
    Inputs(Box<DesignInputsError<E>>),
    /// Original named Closure execution-envelope refusal.
    OutsideEnvelope(ClosureEnvelopeError),
    /// Zipper formula identity names no actual Design parameter metadata.
    MissingFormulaParameter {
        /// Actual Closure requiring the source.
        closure: EntityId,
        /// Original authored parameter identity.
        parameter: EntityId,
    },
    /// Actual current parameter metadata has a kind other than Length.
    FormulaKind {
        /// Actual Closure requiring positive length intent.
        closure: EntityId,
        /// Actual parameter identity.
        parameter: EntityId,
        /// Original current declared kind, without reading its value.
        actual: FormulaKind,
    },
}
impl<E: fmt::Display> fmt::Display for DesignClosureDeclarationError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inputs(issue) => write!(f, "Closure declarations current inputs: {issue}"),
            Self::OutsideEnvelope(issue) => issue.fmt(f),
            Self::MissingFormulaParameter { closure, parameter } => {
                write!(f, "Closure {closure} has no formula parameter {parameter}")
            }
            Self::FormulaKind {
                closure,
                parameter,
                actual,
            } => write!(
                f,
                "Closure {closure} requires Length parameter {parameter}, found {actual:?}"
            ),
        }
    }
}
impl<E: std::error::Error + 'static> std::error::Error for DesignClosureDeclarationError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Inputs(issue) => Some(issue.as_ref()),
            Self::OutsideEnvelope(issue) => Some(issue),
            Self::MissingFormulaParameter { .. } | Self::FormulaKind { .. } => None,
        }
    }
}
impl<H: DesignReferenceHost> DesignInputs<H> {
    /// Borrow every actual Closure's known field roles after complete current root validation.
    ///
    /// Formula zipper lengths select actual Design parameter metadata and require Length kind through
    /// its current canonical binding. Unknown/Derived states are not read or evaluated. Inline fields
    /// retain original intent; logical size domains, recipe operation existence/kind/dependencies and
    /// positive resolved lengths retain their existing recipe/G3/G4 owners. Notion/current geometry
    /// and global garment identities are separate checks. No partial collection escapes a refusal.
    ///
    /// # Errors
    /// Returns the original current root cause, then first Closure scope/formula-metadata refusal.
    pub fn closure_declarations<'a>(
        &'a self,
        closures: &'a ClosureInventory,
    ) -> Result<Vec<DesignClosureDeclaration<'a>>, DesignClosureDeclarationError<H::Error>> {
        self.validate_current()
            .map_err(|issue| DesignClosureDeclarationError::Inputs(Box::new(issue)))?;
        let mut views = Vec::new();
        for closure in closures.closures() {
            let mut push = |role, source| {
                views.push(DesignClosureDeclaration {
                    closure,
                    canonical: self.inputs(),
                    role,
                    source,
                })
            };
            use DesignClosureDeclarationRole as Role;
            use DesignClosureDeclarationSource as Source;
            match &closure.definition().kind {
                ClosureKind::CentredZipper { length } => {
                    let source = match length {
                        ClosureLength::Explicit { parameter, value } => {
                            Source::ExplicitLength { parameter, value }
                        }
                        ClosureLength::Profile(reference) => Source::ProfileLength(reference),
                        ClosureLength::Formula(identity) => {
                            let parameter = self.parameters().parameter(*identity).ok_or(
                                DesignClosureDeclarationError::MissingFormulaParameter {
                                    closure: closure.id(),
                                    parameter: *identity,
                                },
                            )?;
                            let declaration =
                                parameter.declaration(self.inputs()).map_err(|issue| {
                                    DesignClosureDeclarationError::Inputs(Box::new(
                                        DesignInputsError::Parameters(Box::new(issue)),
                                    ))
                                })?;
                            let actual = declaration.kind();
                            if actual != FormulaKind::Length {
                                return Err(DesignClosureDeclarationError::FormulaKind {
                                    closure: closure.id(),
                                    parameter: *identity,
                                    actual,
                                });
                            }
                            Source::FormulaLength {
                                parameter,
                                declaration,
                            }
                        }
                    };
                    push(Role::ZipperLength, source);
                }
                ClosureKind::HookAndBar { hook, bar } => {
                    push(Role::HookSize, Source::Size(hook));
                    push(Role::BarSize, Source::Size(bar));
                }
                ClosureKind::ButtonAndButtonhole { button, .. } => {
                    push(Role::ButtonSize, Source::Size(button));
                    if let Some(source) = closure.buttonhole_length_source() {
                        push(Role::Buttonhole, Source::Buttonhole(source));
                    }
                }
                ClosureKind::Fly { .. } => closure
                    .definition()
                    .require_in_scope()
                    .map_err(DesignClosureDeclarationError::OutsideEnvelope)?,
            }
        }
        Ok(views)
    }
}
