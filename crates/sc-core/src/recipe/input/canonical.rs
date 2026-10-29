//! Actual canonical inventory joins, before authored reads or expression execution.
use super::{FormulaLengthInput, FormulaScalarInput, FormulaScalarInputAdapterError};
use crate::{
    ontology::EntityId,
    recipe::{FormulaDeclaration, FormulaDeclarationSource},
    value::CanonicalInputs,
};
use core::fmt;

/// Inspect the existing source/state-preserving adapter selected by an actual inventory join.
/// This enum alone is not an inventory membership certificate; use FormulaCanonicalInput.
#[derive(Clone, Copy, Debug)]
pub enum FormulaCanonicalInputAdapter<'a> {
    /// Actual dedicated length, retaining its exact original canonical borrow.
    Length(FormulaLengthInput<'a>),
    /// Actual general scalar, retaining its canonical identity and checked kind.
    Scalar(FormulaScalarInput<'a>),
}

/// Actual inventory/source refusal; no fallback value, family conversion or language error.
#[derive(Clone, Copy, Debug)]
pub enum FormulaCanonicalInputRefusal<'a> {
    /// Prior let, reserved or geometry metadata is not a canonical input source.
    NotCanonicalInput,
    /// Dedicated length target is absent; a general Length-kind scalar cannot substitute.
    MissingLength {
        /// Actual declared canonical target identity.
        declaration: EntityId,
        /// Original zero-based scalar position if this identity belongs to the other family.
        scalar_index: Option<usize>,
    },
    /// General scalar target is absent; a dedicated length cannot substitute.
    MissingScalar {
        /// Actual declared canonical target identity.
        declaration: EntityId,
        /// Original zero-based dedicated length position if owned by the other family.
        length_index: Option<usize>,
    },
    /// The metadata's borrowed length is not the actual inventory record, even if content is equal.
    LengthOwner {
        /// Shared identity cannot substitute an equal-looking independently owned record.
        declaration: EntityId,
    },
    /// Original general-scalar adapter refusal with its actual candidate and typed cause.
    Scalar(FormulaScalarInputAdapterError<'a>),
}

/// Refusal with the original declaration and actual supplied canonical inventory.
/// Explicit metadata/refusal inspection exposes original source names; Debug omits them.
#[derive(Clone, Copy, Debug)]
pub struct FormulaCanonicalInputError<'a> {
    metadata: FormulaDeclaration<'a>,
    inputs: &'a CanonicalInputs,
    issue: JoinIssue,
}
// Keep the immutable source/owner once. Detailed borrowed witnesses are reconstructed on explicit
// inspection, rather than duplicating metadata and a full scalar error in every Result's Err slot.
#[derive(Clone, Copy, Debug)]
enum JoinIssue {
    NotCanonicalInput,
    MissingLength { scalar_index: Option<usize> },
    MissingScalar { length_index: Option<usize> },
    LengthOwner,
    Scalar,
}
impl<'a> FormulaCanonicalInputError<'a> {
    /// Exact original metadata, including canonical target and any actual length borrow.
    #[must_use]
    pub const fn metadata(self) -> FormulaDeclaration<'a> {
        self.metadata
    }
    /// Actual supplied inventory; records and original family positions remain inspectable.
    #[must_use]
    pub const fn inputs(self) -> &'a CanonicalInputs {
        self.inputs
    }
    /// Original source/family/owner refusal, including the existing scalar adapter error.
    #[must_use]
    pub fn refusal(self) -> FormulaCanonicalInputRefusal<'a> {
        use FormulaCanonicalInputRefusal as Refusal;
        match (self.issue, self.metadata.source()) {
            (JoinIssue::NotCanonicalInput, _) => Refusal::NotCanonicalInput,
            (
                JoinIssue::MissingLength { scalar_index },
                FormulaDeclarationSource::LengthInput { declaration, .. },
            ) => Refusal::MissingLength {
                declaration: declaration.id(),
                scalar_index,
            },
            (
                JoinIssue::MissingScalar { length_index },
                FormulaDeclarationSource::Input { declaration, .. },
            ) => Refusal::MissingScalar {
                declaration,
                length_index,
            },
            (JoinIssue::LengthOwner, FormulaDeclarationSource::LengthInput { declaration, .. }) => {
                Refusal::LengthOwner {
                    declaration: declaration.id(),
                }
            }
            (JoinIssue::Scalar, FormulaDeclarationSource::Input { declaration, .. }) => {
                let Some(actual) = self.inputs.scalar(declaration) else {
                    unreachable!("private scalar issue retains its immutable inventory target");
                };
                let Err(error) = FormulaScalarInput::from_declaration(self.metadata, actual) else {
                    unreachable!("same immutable metadata/target reproduces its scalar refusal");
                };
                Refusal::Scalar(error)
            }
            _ => unreachable!("private join issue retains its exact source family"),
        }
    }
}
impl fmt::Display for FormulaCanonicalInputError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "formula canonical input join: {:?}", self.refusal())
    }
}
impl std::error::Error for FormulaCanonicalInputError<'_> {
    // A nested scalar cause borrows the actual record and is exposed through refusal(), not
    // erased into Error::source's 'static requirement.
}

/// Private source-to-inventory join retaining the existing authored-read adapter.
/// It proves membership in the supplied immutable CanonicalInputs, not a containing Design,
/// revision/currentness, metadata ownership, evidence truth, resource envelope or execution.
/// No allocation, clone, name-only search, value read or registry substitution occurs.
/// ```compile_fail
/// fn substitute<'a>(joined: &mut sc_core::recipe::FormulaCanonicalInput<'a>,
///     other: &'a sc_core::value::CanonicalInputs) {
///     joined.inputs = other;
/// }
/// ```
/// The joined view cannot outlive its actual inventory:
/// ```compile_fail
/// fn escape(metadata: sc_core::recipe::FormulaDeclaration<'static>)
///     -> sc_core::recipe::FormulaCanonicalInput<'static> {
///     let inputs = sc_core::value::CanonicalInputs::new(vec![], vec![]).unwrap();
///     sc_core::recipe::FormulaCanonicalInput::resolve(metadata, &inputs).unwrap()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct FormulaCanonicalInput<'a> {
    inputs: &'a CanonicalInputs,
    adapter: FormulaCanonicalInputAdapter<'a>,
}
impl<'a> FormulaCanonicalInput<'a> {
    /// Resolve the metadata's actual family/identity before selecting its existing read adapter.
    /// Unknown/Derived states are valid matches and are not read or defaulted by this join.
    ///
    /// ```
    /// use sc_core::{name::MachineToken, ontology::EntityId, recipe::*, value::*};
    /// let name = MachineToken::new("body_width")?;
    /// let record = LengthDeclaration::new(LengthDeclarationDefinition {
    ///     id: EntityId::from_bits(1), source: EntityId::from_bits(2),
    ///     state: LengthState::Unknown { observation: EntityId::from_bits(3) },
    /// })?;
    /// let inputs = CanonicalInputs::new(vec![record], vec![])?;
    /// let record = inputs.length(EntityId::from_bits(1)).expect("actual owned record");
    /// let metadata = FormulaDeclaration::length_input(&name, FormulaInputOrigin::Measurement,
    ///     EntityId::from_bits(4), record);
    /// let joined = FormulaCanonicalInput::resolve(metadata, &inputs).expect("actual owner");
    /// assert!(std::ptr::eq(joined.inputs(), &inputs));
    /// if let FormulaCanonicalInputAdapter::Length(input) = joined.adapter() {
    ///     assert!(std::ptr::eq(input.canonical_declaration(), record));
    ///     assert!(input.read_authored().is_err()); // Observation remains necessary.
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    /// # Errors
    /// Source family precedes inventory lookup. A missing family retains an actual other-family
    /// position if present; length owner matching precedes any read. Scalars reuse the existing
    /// target/kind checker and preserve its borrowed error. No name lookup or numeric read occurs.
    pub fn resolve(
        metadata: FormulaDeclaration<'a>,
        inputs: &'a CanonicalInputs,
    ) -> Result<Self, FormulaCanonicalInputError<'a>> {
        let refuse = |issue| FormulaCanonicalInputError {
            metadata,
            inputs,
            issue,
        };
        let adapter = match metadata.source() {
            FormulaDeclarationSource::LengthInput { declaration, .. } => {
                let id = declaration.id();
                let actual = inputs.length(id).ok_or_else(|| {
                    refuse(JoinIssue::MissingLength {
                        scalar_index: inputs.scalars().iter().position(|input| input.id() == id),
                    })
                })?;
                if !core::ptr::eq(actual, declaration) {
                    return Err(refuse(JoinIssue::LengthOwner));
                }
                let Some(input) = FormulaLengthInput::from_declaration(metadata) else {
                    unreachable!("actual LengthInput match selects the dedicated adapter");
                };
                FormulaCanonicalInputAdapter::Length(input)
            }
            FormulaDeclarationSource::Input { declaration, .. } => {
                let actual = inputs.scalar(declaration).ok_or_else(|| {
                    refuse(JoinIssue::MissingScalar {
                        length_index: inputs
                            .lengths()
                            .iter()
                            .position(|input| input.id() == declaration),
                    })
                })?;
                let input = FormulaScalarInput::from_declaration(metadata, actual)
                    .map_err(|_| refuse(JoinIssue::Scalar))?;
                FormulaCanonicalInputAdapter::Scalar(input)
            }
            FormulaDeclarationSource::Point(_)
            | FormulaDeclarationSource::Edge(_)
            | FormulaDeclarationSource::Recipe { .. }
            | FormulaDeclarationSource::Reserved(_) => {
                return Err(refuse(JoinIssue::NotCanonicalInput));
            }
        };
        Ok(Self { inputs, adapter })
    }

    /// Exact actual canonical inventory establishing this join, without a copied record/state.
    #[must_use]
    pub const fn inputs(self) -> &'a CanonicalInputs {
        self.inputs
    }
    /// Existing dedicated/general authored-read view, preserving original source/state labels.
    /// Extracting the adapter does not itself retain this view's inventory-membership certificate.
    #[must_use]
    pub const fn adapter(self) -> FormulaCanonicalInputAdapter<'a> {
        self.adapter
    }
    /// Exact original metadata reconstructed from the sealed existing adapter.
    #[must_use]
    pub fn metadata(self) -> FormulaDeclaration<'a> {
        match self.adapter {
            FormulaCanonicalInputAdapter::Length(input) => input.metadata(),
            FormulaCanonicalInputAdapter::Scalar(input) => input.metadata(),
        }
    }
}
