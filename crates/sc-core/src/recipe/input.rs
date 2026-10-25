//! Explicit authored reads from canonical length inputs; no namespace, source-truth or replay proof.
use super::{
    FormulaDeclaration, FormulaDeclarationSource, FormulaInputOrigin, FormulaNumericValue,
};
use crate::ontology::EntityId;
use crate::value::{LengthDeclaration, LengthState, LengthValueError, ValueProvenanceValidation};
use core::fmt;

mod scalar;
pub use scalar::{
    FormulaScalarInput, FormulaScalarInputAdapterError, FormulaScalarInputAdapterRule,
    FormulaScalarInputError, FormulaScalarInputValue,
};

/// Read adapter for an actual borrowed canonical length declaration, with its original metadata.
///
/// General scalar metadata claiming Length cannot construct this adapter. Construction does not
/// fetch a value or validate source registries, evidence scope, operation order or human approval.
#[derive(Clone, Copy)]
pub struct FormulaLengthInput<'a> {
    name: &'a str,
    origin: FormulaInputOrigin,
    input: EntityId,
    declaration: &'a LengthDeclaration,
}

impl<'a> FormulaLengthInput<'a> {
    /// Selects an actual LengthInput source; every other metadata variant returns None.
    #[must_use]
    pub fn from_declaration(metadata: FormulaDeclaration<'a>) -> Option<Self> {
        let FormulaDeclarationSource::LengthInput {
            origin,
            input,
            declaration,
        } = metadata.source()
        else {
            return None;
        };
        Some(Self {
            name: metadata.name(),
            origin,
            input,
            declaration,
        })
    }

    /// Original name, origin and metadata identity, reconstructed from the sealed source view.
    #[must_use]
    pub const fn metadata(self) -> FormulaDeclaration<'a> {
        FormulaDeclaration::from_length_input_view(self)
    }

    /// Actual borrowed name from the original validated metadata.
    #[must_use]
    pub const fn name(self) -> &'a str {
        self.name
    }

    /// Actual source domain from the original metadata.
    #[must_use]
    pub const fn origin(self) -> FormulaInputOrigin {
        self.origin
    }

    /// Metadata record identity, distinct from the canonical value declaration.
    #[must_use]
    pub const fn input_id(self) -> EntityId {
        self.input
    }

    /// Actual canonical record; its identity/source/value/state remain singly owned there.
    #[must_use]
    pub const fn canonical_declaration(self) -> &'a LengthDeclaration {
        self.declaration
    }

    /// Complete borrowed state, including the original evidence/assumption/preference references.
    #[must_use]
    pub const fn state(self) -> &'a LengthState {
        self.declaration.state()
    }

    /// Readability is separate from registry, source-truth and artifact-policy validation.
    #[must_use]
    pub const fn provenance_validation(self) -> ValueProvenanceValidation {
        self.declaration.provenance_validation()
    }

    /// Reads only a present authored value and preserves the canonical source beside the result.
    ///
    /// # Errors
    /// Unknown retains its observation request. Derived retains the formula requiring evaluation;
    /// this adapter never supplies zero, a cached result or an independently authored derived value.
    pub fn read_authored(self) -> Result<FormulaLengthInputValue<'a>, FormulaLengthInputError<'a>> {
        let value =
            self.declaration
                .authored_value()
                .map_err(|reason| FormulaLengthInputError {
                    input: self,
                    reason,
                })?;
        Ok(FormulaLengthInputValue {
            input: self,
            value: value.into(),
        })
    }
}

impl fmt::Debug for FormulaLengthInput<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The metadata Debug view intentionally omits customer names and canonical state/value.
        f.debug_struct("FormulaLengthInput")
            .field("metadata", &self.metadata())
            .finish_non_exhaustive()
    }
}

/// A present exact Length value beside its actual canonical authored state and source metadata.
/// This is a read result, without evaluation, binding publication or source/export approval.
#[derive(Clone, Copy, Debug)]
pub struct FormulaLengthInputValue<'a> {
    input: FormulaLengthInput<'a>,
    value: FormulaNumericValue,
}

impl<'a> FormulaLengthInputValue<'a> {
    /// Retained input identity, origin and complete authored state/provenance.
    #[must_use]
    pub const fn input(self) -> FormulaLengthInput<'a> {
        self.input
    }

    /// Canonical signed micrometre integer imported exactly, without another rounding boundary.
    #[must_use]
    pub const fn value(self) -> FormulaNumericValue {
        self.value
    }
}

/// An unavailable authored value with its original metadata and exact canonical request.
/// RequiresEvaluation is a request for the evaluator, distinct from an unknown observation.
#[derive(Clone, Copy, Debug)]
pub struct FormulaLengthInputError<'a> {
    input: FormulaLengthInput<'a>,
    reason: LengthValueError,
}

impl<'a> FormulaLengthInputError<'a> {
    /// Actual input whose canonical authored value could not be read.
    #[must_use]
    pub const fn input(self) -> FormulaLengthInput<'a> {
        self.input
    }

    /// Observation or formula identity supplied by the canonical declaration itself.
    #[must_use]
    pub const fn reason(self) -> LengthValueError {
        self.reason
    }
}

impl fmt::Display for FormulaLengthInputError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "length input {}: {}", self.input.name(), self.reason)
    }
}

impl std::error::Error for FormulaLengthInputError<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.reason)
    }
}
