//! Immutable metadata locators; constructing one neither validates a namespace nor reads a value.
use super::{
    FormulaBindingKind, FormulaKind, FormulaNormalizedRecipe, FormulaNormalizedStatementKind,
    FormulaOrigin, FormulaReservedName, FormulaSourceSpan,
};
use crate::{
    name::MachineToken,
    ontology::{EdgeRef, EntityId, PointRef},
    value::LengthDeclaration,
};
use core::fmt;

/// External authored source domains, excluding geometry, recipe bindings and reserved contexts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaInputOrigin {
    /// Selected measurement table entry.
    Measurement,
    /// Selected Ease set entry.
    Ease,
    /// Design parameter.
    Parameter,
    /// Factory Profile parameter.
    Profile,
    /// Material property.
    Material,
}
impl From<FormulaInputOrigin> for FormulaOrigin {
    fn from(origin: FormulaInputOrigin) -> Self {
        match origin {
            FormulaInputOrigin::Measurement => Self::Measurement,
            FormulaInputOrigin::Ease => Self::Ease,
            FormulaInputOrigin::Parameter => Self::Parameter,
            FormulaInputOrigin::Profile => Self::Profile,
            FormulaInputOrigin::Material => Self::Material,
        }
    }
}

/// Domains permitting general scalar metadata. Measurement and Ease use canonical length records.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaScalarInputOrigin {
    /// Design parameter.
    Parameter,
    /// Factory Profile parameter.
    Profile,
    /// Material property.
    Material,
}
impl From<FormulaScalarInputOrigin> for FormulaOrigin {
    fn from(origin: FormulaScalarInputOrigin) -> Self {
        match origin {
            FormulaScalarInputOrigin::Parameter => Self::Parameter,
            FormulaScalarInputOrigin::Profile => Self::Profile,
            FormulaScalarInputOrigin::Material => Self::Material,
        }
    }
}

/// Read-only source locator. Ids and declared metadata do not prove registry or evidence validity.
#[derive(Clone, Copy)]
pub enum FormulaDeclarationSource<'a> {
    /// Caller-authored scalar metadata; the canonical registry must later verify kind and identity.
    Input {
        /// Semantic source domain.
        origin: FormulaScalarInputOrigin,
        /// Metadata record exposing this name, distinct from its value declaration.
        input: EntityId,
        /// Sole canonical value declaration identity; no value is copied here.
        declaration: EntityId,
        /// Declared scalar kind, without inferring or fetching a value.
        kind: FormulaBindingKind,
    },
    /// Adapter borrowing an existing canonical length declaration without consulting its state.
    LengthInput {
        /// Semantic source domain.
        origin: FormulaInputOrigin,
        /// Metadata record exposing this name.
        input: EntityId,
        /// Actual immutable canonical record, whose id/source/state remain singly owned.
        declaration: &'a LengthDeclaration,
    },
    /// Prior operation point reference; no coordinates or new geometry are supplied.
    Point(PointRef),
    /// Prior operation edge reference; no curve or tessellation is supplied.
    Edge(EdgeRef),
    /// Annotation and original location from an actual normalized recipe let statement.
    Recipe {
        /// Actual one-based statement ordinal, not an invented index for detached syntax.
        statement_index: usize,
        /// Authored annotation, not proof that the expression has this kind.
        kind: FormulaBindingKind,
        /// Original whole-statement location.
        span: FormulaSourceSpan,
        /// Original binding-name location.
        name_span: FormulaSourceSpan,
    },
    /// Fixed built-in name with metadata available independently of its value provider.
    Reserved(FormulaReservedName),
}
impl FormulaDeclarationSource<'_> {
    /// Declared kind only; never queries canonical state, numeric value or geometry.
    #[must_use]
    pub fn kind(self) -> FormulaKind {
        match self {
            Self::Input { kind, .. } | Self::Recipe { kind, .. } => kind.into(),
            Self::LengthInput { .. } => FormulaKind::Length,
            Self::Point(_) => FormulaKind::Point,
            Self::Edge(_) => FormulaKind::Edge,
            Self::Reserved(name) => name.kind(),
        }
    }

    /// Semantic origin only, distinct from a reserved name's value-provider context.
    #[must_use]
    pub fn origin(self) -> FormulaOrigin {
        match self {
            Self::Input { origin, .. } => origin.into(),
            Self::LengthInput { origin, .. } => origin.into(),
            Self::Point(_) | Self::Edge(_) => FormulaOrigin::Geometry,
            Self::Recipe { .. } => FormulaOrigin::Recipe,
            Self::Reserved(name) => name.origin(),
        }
    }
}
impl fmt::Debug for FormulaDeclarationSource<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaDeclarationSource")
            .field("kind", &self.kind())
            .field("origin", &self.origin())
            .finish_non_exhaustive()
    }
}

/// Immutable named metadata, borrowing validated names and canonical records where available.
/// Construction grants no collision, ordering, type-checking or runtime authority.
/// Measurement and Ease cannot enter through the generic scalar metadata constructor.
/// ```compile_fail
/// use sc_core::{name::MachineToken, ontology::EntityId, recipe::{
///     FormulaBindingKind, FormulaDeclaration, FormulaInputOrigin,
/// }};
/// let name = MachineToken::new("body_width").unwrap();
/// let invalid = FormulaDeclaration::input(&name, FormulaInputOrigin::Measurement,
///     EntityId::from_bits(1), EntityId::from_bits(2), FormulaBindingKind::Angle);
/// ```
/// ```compile_fail
/// fn mutate(d: &mut sc_core::recipe::FormulaDeclaration<'_>) { d.name = "other"; }
/// ```
/// ```compile_fail
/// fn escaped() -> sc_core::recipe::FormulaDeclaration<'static> {
///     let name = sc_core::name::MachineToken::new("width").unwrap();
///     sc_core::recipe::FormulaDeclaration::point(&name,
///         sc_core::ontology::PointRef::new(sc_core::ontology::EntityId::from_bits(1),
///             sc_core::ontology::LocalTag::FIRST))
/// }
/// ```
/// ```compile_fail
/// fn detached(source: &str) -> sc_core::recipe::FormulaDeclaration<'_> {
///     let recipe = sc_core::recipe::FormulaRecipe::parse(source).unwrap()
///         .normalize_literals().unwrap();
///     sc_core::recipe::FormulaDeclaration::recipe(&recipe, 1).unwrap()
/// }
/// ```
/// ```compile_fail
/// fn detached<'a>(name: &'a sc_core::name::MachineToken) -> sc_core::recipe::FormulaDeclaration<'a> {
///     use sc_core::{ontology::EntityId, value::{LengthDeclaration, LengthDeclarationDefinition, LengthState}};
///     let record = LengthDeclaration::new(LengthDeclarationDefinition {
///         id: EntityId::from_bits(1), source: EntityId::from_bits(2),
///         state: LengthState::Unknown { observation: EntityId::from_bits(3) },
///     }).unwrap();
///     sc_core::recipe::FormulaDeclaration::length_input(name,
///         sc_core::recipe::FormulaInputOrigin::Measurement, EntityId::from_bits(4), &record)
/// }
/// ```
#[derive(Clone, Copy)]
pub struct FormulaDeclaration<'a> {
    name: &'a str,
    source: FormulaDeclarationSource<'a>,
}
impl<'a> FormulaDeclaration<'a> {
    /// Carry an explicit input metadata claim; canonical target/kind/state validation remains separate.
    #[must_use]
    pub fn input(
        name: &'a MachineToken,
        origin: FormulaScalarInputOrigin,
        input: EntityId,
        declaration: EntityId,
        kind: FormulaBindingKind,
    ) -> Self {
        Self {
            name: name.as_str(),
            source: FormulaDeclarationSource::Input {
                origin,
                input,
                declaration,
                kind,
            },
        }
    }

    /// Borrow the canonical length record, preserving its complete identity without reading its value.
    #[must_use]
    pub fn length_input(
        name: &'a MachineToken,
        origin: FormulaInputOrigin,
        input: EntityId,
        declaration: &'a LengthDeclaration,
    ) -> Self {
        Self {
            name: name.as_str(),
            source: FormulaDeclarationSource::LengthInput {
                origin,
                input,
                declaration,
            },
        }
    }

    /// Declare the exact output reference of a prior point operation; ordering is checked later.
    #[must_use]
    pub fn point(name: &'a MachineToken, point: PointRef) -> Self {
        Self {
            name: name.as_str(),
            source: FormulaDeclarationSource::Point(point),
        }
    }

    /// Declare the exact output reference of a prior edge operation; no curve is resolved.
    #[must_use]
    pub fn edge(name: &'a MachineToken, edge: EdgeRef) -> Self {
        Self {
            name: name.as_str(),
            source: FormulaDeclarationSource::Edge(edge),
        }
    }

    /// Inspect an actual one-based normalized recipe position, without binding or inferring its RHS.
    /// Returns None for zero, absent positions or assertion labels, which declare no scalar value.
    #[must_use]
    pub fn recipe(recipe: &'a FormulaNormalizedRecipe<'_>, statement_index: usize) -> Option<Self> {
        let statement = recipe.statements().get(statement_index.checked_sub(1)?)?;
        let FormulaNormalizedStatementKind::Let { declared_kind, .. } = statement.kind() else {
            return None;
        };
        Some(Self {
            name: statement.name(),
            source: FormulaDeclarationSource::Recipe {
                statement_index,
                kind: declared_kind,
                span: statement.span(),
                name_span: statement.name_span(),
            },
        })
    }

    /// Fixed name/kind/origin metadata without any size, export or profile value context.
    #[must_use]
    pub const fn reserved(name: FormulaReservedName) -> FormulaDeclaration<'static> {
        FormulaDeclaration {
            name: name.token(),
            source: FormulaDeclarationSource::Reserved(name),
        }
    }

    /// Explicit authored name; Debug omits it to avoid accidental customer-data logging.
    #[must_use]
    pub const fn name(self) -> &'a str {
        self.name
    }

    /// Immutable source view; explicit inspection is separate from fetching a numerical value.
    #[must_use]
    pub const fn source(self) -> FormulaDeclarationSource<'a> {
        self.source
    }

    /// Kind known without evaluating or reading this declaration's source.
    #[must_use]
    pub fn kind(self) -> FormulaKind {
        self.source.kind()
    }

    /// Semantic source domain without inspecting availability or context values.
    #[must_use]
    pub fn origin(self) -> FormulaOrigin {
        self.source.origin()
    }
}
impl fmt::Debug for FormulaDeclaration<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaDeclaration")
            .field("kind", &self.kind())
            .field("origin", &self.origin())
            .finish_non_exhaustive()
    }
}
