//! General scalar reads verify the supplied canonical target, without registry or execution proof.
use super::super::{
    FormulaBindingKind, FormulaDeclaration, FormulaDeclarationSource, FormulaKind, FormulaOrigin,
    FormulaScalarInputOrigin,
};
use crate::{
    ontology::EntityId,
    value::{
        ScalarInputDeclaration, ScalarInputKind, ScalarInputReadError, ScalarInputState,
        ScalarInputValue, ValueProvenanceValidation,
    },
};
use core::fmt;

/// Actual failure selecting or matching a general scalar metadata source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaScalarInputAdapterRule {
    /// Metadata is not a general Input source; its actual kind/origin are retained.
    NotScalarInput {
        /// Metadata's declared kind, without value resolution.
        kind: FormulaKind,
        /// Actual semantic origin.
        origin: FormulaOrigin,
    },
    /// The caller supplied a different canonical declaration from the metadata target.
    CanonicalIdentity {
        /// Target identity named by the metadata.
        declared: EntityId,
        /// Actual supplied canonical identity.
        actual: EntityId,
    },
    /// The metadata annotation differs from the actual canonical kind.
    CanonicalKind {
        /// Annotation claimed by the metadata.
        declared: FormulaBindingKind,
        /// Actual canonical declaration kind.
        actual: ScalarInputKind,
    },
}

/// Refused adapter construction with the real name, candidate canonical target and typed reason.
#[derive(Clone, Copy)]
pub struct FormulaScalarInputAdapterError<'a> {
    name: &'a str,
    canonical: &'a ScalarInputDeclaration,
    rule: FormulaScalarInputAdapterRule,
}

impl fmt::Debug for FormulaScalarInputAdapterError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaScalarInputAdapterError")
            .field("rule", &self.rule)
            .finish_non_exhaustive()
    }
}

impl<'a> FormulaScalarInputAdapterError<'a> {
    /// Actual metadata name supplied by the caller.
    #[must_use]
    pub const fn name(self) -> &'a str {
        self.name
    }

    /// Actual candidate, including its original source and authored state.
    #[must_use]
    pub const fn canonical_declaration(self) -> &'a ScalarInputDeclaration {
        self.canonical
    }

    /// Source, identity or kind refusal; no guessed statement/registry context.
    #[must_use]
    pub const fn rule(self) -> FormulaScalarInputAdapterRule {
        self.rule
    }
}

impl fmt::Display for FormulaScalarInputAdapterError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "scalar input {} adapter: {:?}", self.name, self.rule)
    }
}
impl std::error::Error for FormulaScalarInputAdapterError<'_> {}

/// Compact borrowed view of general scalar metadata matched to its actual canonical target/kind.
/// This proves that match, without proving registry membership, source truth or execution order.
#[derive(Clone, Copy)]
pub struct FormulaScalarInput<'a> {
    name: &'a str,
    origin: FormulaScalarInputOrigin,
    input: EntityId,
    canonical: &'a ScalarInputDeclaration,
}

impl<'a> FormulaScalarInput<'a> {
    /// Checks a general Input source, then canonical identity, then actual kind, without reading it.
    /// # Errors
    /// Returns the actual source/identity/kind mismatch and candidate canonical record.
    pub fn from_declaration(
        metadata: FormulaDeclaration<'a>,
        canonical: &'a ScalarInputDeclaration,
    ) -> Result<Self, FormulaScalarInputAdapterError<'a>> {
        let refusal = |rule| FormulaScalarInputAdapterError {
            name: metadata.name(),
            canonical,
            rule,
        };
        let FormulaDeclarationSource::Input {
            origin,
            input,
            declaration,
            kind,
        } = metadata.source()
        else {
            return Err(refusal(FormulaScalarInputAdapterRule::NotScalarInput {
                kind: metadata.kind(),
                origin: metadata.origin(),
            }));
        };
        if declaration != canonical.id() {
            return Err(refusal(FormulaScalarInputAdapterRule::CanonicalIdentity {
                declared: declaration,
                actual: canonical.id(),
            }));
        }
        if kind != FormulaBindingKind::from(canonical.kind()) {
            return Err(refusal(FormulaScalarInputAdapterRule::CanonicalKind {
                declared: kind,
                actual: canonical.kind(),
            }));
        }
        Ok(Self {
            name: metadata.name(),
            origin,
            input,
            canonical,
        })
    }

    /// Original name and source fields, reconstructed only from this sealed matched view.
    #[must_use]
    pub fn metadata(self) -> FormulaDeclaration<'a> {
        FormulaDeclaration::from_scalar_input_view(self)
    }

    /// Actual borrowed name.
    #[must_use]
    pub const fn name(self) -> &'a str {
        self.name
    }

    /// Actual general scalar origin.
    #[must_use]
    pub const fn origin(self) -> FormulaScalarInputOrigin {
        self.origin
    }

    /// Original metadata identity, distinct from the canonical value declaration.
    #[must_use]
    pub const fn input_id(self) -> EntityId {
        self.input
    }

    /// Matched actual canonical record; its value/state/source remain singly owned there.
    #[must_use]
    pub const fn canonical_declaration(self) -> &'a ScalarInputDeclaration {
        self.canonical
    }

    /// Complete borrowed authored state, independent of numeric/Boolean availability.
    #[must_use]
    pub const fn state(self) -> &'a ScalarInputState {
        self.canonical.state()
    }

    /// Registry, evidence scope/truth and artifact policy remain separate obligations.
    #[must_use]
    pub const fn provenance_validation(self) -> ValueProvenanceValidation {
        self.canonical.provenance_validation()
    }

    /// Reads a present typed canonical scalar beside its matched source and complete state.
    /// # Errors
    /// Unknown requires observation; Derived requires the sole formula's evaluation. No numeric or
    /// Boolean fallback, independent derived value, binding publication or approval is supplied.
    pub fn read_authored(self) -> Result<FormulaScalarInputValue<'a>, FormulaScalarInputError<'a>> {
        let value = self
            .canonical
            .authored_value()
            .map_err(|reason| FormulaScalarInputError {
                input: self,
                reason,
            })?;
        Ok(FormulaScalarInputValue { input: self, value })
    }
}

impl fmt::Debug for FormulaScalarInput<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaScalarInput")
            .field("metadata", &self.metadata())
            .finish_non_exhaustive()
    }
}

/// Present canonical scalar with its actual source/state; separate from expression evaluation.
#[derive(Clone, Copy, Debug)]
pub struct FormulaScalarInputValue<'a> {
    input: FormulaScalarInput<'a>,
    value: ScalarInputValue,
}

impl<'a> FormulaScalarInputValue<'a> {
    /// Matched canonical source and complete authored state.
    #[must_use]
    pub const fn input(self) -> FormulaScalarInput<'a> {
        self.input
    }

    /// Exact canonical typed scalar, including raw turns, numeric zero and Boolean false.
    #[must_use]
    pub const fn value(self) -> ScalarInputValue {
        self.value
    }
}

/// Unavailable authored scalar with its matched canonical source and actual required action.
#[derive(Clone, Copy, Debug)]
pub struct FormulaScalarInputError<'a> {
    input: FormulaScalarInput<'a>,
    reason: ScalarInputReadError,
}

impl<'a> FormulaScalarInputError<'a> {
    /// Actual input whose authored value is unavailable.
    #[must_use]
    pub const fn input(self) -> FormulaScalarInput<'a> {
        self.input
    }

    /// Actual observation/formula requirement from the canonical record.
    #[must_use]
    pub const fn reason(self) -> ScalarInputReadError {
        self.reason
    }
}

impl fmt::Display for FormulaScalarInputError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "scalar input {}: {}", self.input.name(), self.reason)
    }
}
impl std::error::Error for FormulaScalarInputError<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.reason)
    }
}
