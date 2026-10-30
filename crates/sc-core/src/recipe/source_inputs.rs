//! Complete original canonical Derived references joined to actual source owners, without values.
use super::{FormulaSource, FormulaSources};
use crate::{
    ontology::EntityId,
    value::{
        CanonicalInputPosition, CanonicalInputTarget, CanonicalInputs, LengthDeclaration,
        LengthState, ScalarInputDeclaration, ScalarInputState,
    },
};
use core::fmt;

#[derive(Clone, Copy)]
enum Declaration<'a> {
    Length(&'a LengthDeclaration),
    Scalar(&'a ScalarInputDeclaration),
}
impl Declaration<'_> {
    fn id(self) -> EntityId {
        match self {
            Self::Length(value) => value.id(),
            Self::Scalar(value) => value.id(),
        }
    }
    fn target(self) -> CanonicalInputTarget {
        match self {
            Self::Length(value) => CanonicalInputTarget::Length(value.id()),
            Self::Scalar(value) => CanonicalInputTarget::Scalar {
                declaration: value.id(),
                kind: value.kind(),
            },
        }
    }
    fn formula(self) -> Option<EntityId> {
        match self {
            Self::Length(value) => match value.state() {
                LengthState::Derived { formula } => Some(*formula),
                LengthState::Known { .. }
                | LengthState::Assumed { .. }
                | LengthState::Unknown { .. }
                | LengthState::Preference { .. } => None,
            },
            Self::Scalar(value) => match value.state() {
                ScalarInputState::Derived { formula } => Some(*formula),
                ScalarInputState::Known { .. }
                | ScalarInputState::Assumed { .. }
                | ScalarInputState::Unknown { .. }
                | ScalarInputState::Preference { .. } => None,
            },
        }
    }
}
fn declarations(
    inputs: &CanonicalInputs,
) -> impl DoubleEndedIterator<Item = (CanonicalInputPosition, Declaration<'_>)> {
    inputs
        .lengths()
        .iter()
        .enumerate()
        .map(|(index, value)| {
            (
                CanonicalInputPosition::Length { index },
                Declaration::Length(value),
            )
        })
        .chain(inputs.scalars().iter().enumerate().map(|(index, value)| {
            (
                CanonicalInputPosition::Scalar { index },
                Declaration::Scalar(value),
            )
        }))
}

/// Original source-ownership failure, distinct from formula-language or evaluation diagnostics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaSourceInputRefusal {
    /// An expression entity aliases an actual owned canonical declaration identity.
    IdentityCollision {
        /// Original authored expression-source position.
        source_index: usize,
    },
    /// The actual Derived state's sole formula source is absent; no other field substitutes.
    MissingSource {
        /// Exact original formula reference, not the separate provenance/source identity.
        formula: EntityId,
    },
}

/// Error pinned to the actual immutable owners, original canonical position and original cause.
/// Explicit owner inspection is deliberate; Debug/Display omit source text and authored values.
#[derive(Debug)]
pub struct FormulaSourceInputError<'a> {
    inputs: &'a CanonicalInputs,
    sources: &'a FormulaSources,
    position: CanonicalInputPosition,
    target: CanonicalInputTarget,
    refusal: FormulaSourceInputRefusal,
}
impl<'a> FormulaSourceInputError<'a> {
    /// Actual supplied canonical inventory; no independently owned equal-looking replacement.
    #[must_use]
    pub const fn inputs(&self) -> &'a CanonicalInputs {
        self.inputs
    }
    /// Actual supplied expression owners, including their original order and buffers.
    #[must_use]
    pub const fn sources(&self) -> &'a FormulaSources {
        self.sources
    }
    /// Exact original canonical family and position at which admission refused.
    #[must_use]
    pub const fn position(&self) -> CanonicalInputPosition {
        self.position
    }
    /// Actual canonical identity, representation family and declared scalar kind.
    #[must_use]
    pub const fn target(&self) -> CanonicalInputTarget {
        self.target
    }
    /// Original collision position or missing Derived formula identity.
    #[must_use]
    pub const fn refusal(&self) -> FormulaSourceInputRefusal {
        self.refusal
    }
}
impl fmt::Display for FormulaSourceInputError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "formula source/input join at {:?} ({:?}): {:?}",
            self.position, self.target, self.refusal
        )
    }
}
impl std::error::Error for FormulaSourceInputError<'_> {}

/// Actual canonical Derived declaration and its actual expression-source owner.
/// Private fields prevent pairing another state, target, kind or formula with this join.
#[derive(Clone, Copy)]
pub struct FormulaDerivedInput<'a> {
    position: CanonicalInputPosition,
    declaration: Declaration<'a>,
    source: &'a FormulaSource,
}
impl fmt::Debug for FormulaDerivedInput<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaDerivedInput")
            .field("position", &self.position)
            .field("target", &self.target())
            .field("formula", &self.source.id())
            .finish_non_exhaustive()
    }
}
impl<'a> FormulaDerivedInput<'a> {
    /// Original canonical family and authored position.
    #[must_use]
    pub const fn position(self) -> CanonicalInputPosition {
        self.position
    }
    /// Exact family/id/kind; general Length stays distinct from dedicated Length.
    #[must_use]
    pub fn target(self) -> CanonicalInputTarget {
        self.declaration.target()
    }
    /// Actual dedicated declaration when this target belongs to that representation family.
    #[must_use]
    pub const fn length(self) -> Option<&'a LengthDeclaration> {
        match self.declaration {
            Declaration::Length(value) => Some(value),
            Declaration::Scalar(_) => None,
        }
    }
    /// Actual general scalar, retaining its original kind/state/provenance without a copy.
    #[must_use]
    pub const fn scalar(self) -> Option<&'a ScalarInputDeclaration> {
        match self.declaration {
            Declaration::Scalar(value) => Some(value),
            Declaration::Length(_) => None,
        }
    }
    /// Exact expression owner selected only by the actual Derived state's formula field.
    #[must_use]
    pub const fn source(self) -> &'a FormulaSource {
        self.source
    }
}

/// Complete allocation-free input/source admission over the two supplied immutable owners.
/// Source ids are disjoint from canonical declaration ids and every Derived source exists.
/// This is not a global Design, namespace, result-kind, graph, currentness or value certificate.
/// It neither resolves quantity `Formula(id)` parameters nor adds a numeric binding boundary.
/// ```compile_fail
/// fn replace<'a>(scope: &mut sc_core::recipe::FormulaSourceInputs<'a>,
///     other: &'a sc_core::value::CanonicalInputs) { scope.inputs = other; }
/// ```
/// ```compile_fail
/// fn escape(sources: &sc_core::recipe::FormulaSources)
///     -> sc_core::recipe::FormulaSourceInputs<'_> {
///     let inputs = sc_core::value::CanonicalInputs::new(vec![], vec![]).unwrap();
///     sc_core::recipe::FormulaSourceInputs::new(&inputs, sources).unwrap()
/// }
/// ```
/// ```compile_fail
/// fn escape_sources(inputs: &sc_core::value::CanonicalInputs)
///     -> sc_core::recipe::FormulaSourceInputs<'_> {
///     use sc_core::recipe::{FormulaSources, FormulaSourceInputs, FormulaSourceLimits};
///     let sources = FormulaSources::new(vec![], FormulaSourceLimits::new(0, 0, 0)).unwrap();
///     FormulaSourceInputs::new(inputs, &sources).unwrap()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct FormulaSourceInputs<'a> {
    inputs: &'a CanonicalInputs,
    sources: &'a FormulaSources,
}
impl<'a> FormulaSourceInputs<'a> {
    /// Check the complete identity census first, then every Derived reference, in length/scalar
    /// authored order. Unused targets and unused source entities participate. Provenance ids are
    /// references, not newly owned declarations. Non-Derived states require no formula source.
    /// There is no allocation, value read, state clone, normalization, name inference or fallback.
    ///
    /// ```
    /// use sc_core::{ontology::EntityId, recipe::*, value::*};
    /// let formula = EntityId::from_bits(9001);
    /// let declaration = LengthDeclaration::new(LengthDeclarationDefinition {
    ///     id: EntityId::from_bits(10), source: EntityId::from_bits(20),
    ///     state: LengthState::Derived { formula },
    /// })?;
    /// let inputs = CanonicalInputs::new(vec![declaration], vec![])?;
    /// let document = FormulaExpressionDocument::new("1 mm".into())?;
    /// let limits = FormulaSourceLimits::new(1, document.source().len(),
    ///     document.canonical().as_str().len());
    /// let sources = FormulaSources::new(vec![FormulaSource::new(formula, document)], limits)?;
    /// let joined = FormulaSourceInputs::new(&inputs, &sources).expect("complete actual joins");
    /// let derived = joined.derived_inputs().next().expect("actual Derived target");
    /// assert_eq!(derived.target(), CanonicalInputTarget::Length(EntityId::from_bits(10)));
    /// assert_eq!(derived.source().id(), formula);
    /// assert!(derived.length().unwrap().authored_value().is_err()); // No evaluated value added.
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Errors
    /// Returns the first canonical identity collision before any missing-source refusal. Missing
    /// sources retain actual target family/kind/position and original formula id. No prefix escapes.
    pub fn new(
        inputs: &'a CanonicalInputs,
        sources: &'a FormulaSources,
    ) -> Result<Self, FormulaSourceInputError<'a>> {
        let refuse = |position, declaration: Declaration<'_>, refusal| FormulaSourceInputError {
            inputs,
            sources,
            position,
            target: declaration.target(),
            refusal,
        };
        for (position, declaration) in declarations(inputs) {
            if sources.source(declaration.id()).is_some() {
                let Some(source_index) = sources
                    .sources()
                    .iter()
                    .position(|source| source.id() == declaration.id())
                else {
                    unreachable!("same immutable source registry retains its exact indexed owner");
                };
                return Err(refuse(
                    position,
                    declaration,
                    FormulaSourceInputRefusal::IdentityCollision { source_index },
                ));
            }
        }
        for (position, declaration) in declarations(inputs) {
            if let Some(formula) = declaration.formula() {
                if sources.source(formula).is_none() {
                    return Err(refuse(
                        position,
                        declaration,
                        FormulaSourceInputRefusal::MissingSource { formula },
                    ));
                }
            }
        }
        Ok(Self { inputs, sources })
    }
    /// Actual canonical owner, with every original state/provenance field unchanged.
    #[must_use]
    pub const fn inputs(self) -> &'a CanonicalInputs {
        self.inputs
    }
    /// Actual source inventory; this join never copies a source body or changes its id/order.
    #[must_use]
    pub const fn sources(self) -> &'a FormulaSources {
        self.sources
    }
    /// Borrow all and only Derived targets in original length/scalar order. Multiple declarations
    /// may reference one actual source; no deduplication erases their separate targets or kinds.
    pub fn derived_inputs(self) -> impl DoubleEndedIterator<Item = FormulaDerivedInput<'a>> + 'a {
        declarations(self.inputs).filter_map(move |(position, declaration)| {
            let formula = declaration.formula()?;
            let Some(source) = self.sources.source(formula) else {
                unreachable!("complete immutable admission retains every actual Derived source");
            };
            Some(FormulaDerivedInput {
                position,
                declaration,
                source,
            })
        })
    }
}
