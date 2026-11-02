//! Checked immutable initial declarations; reserved kinds do not require runtime value providers.
use super::{FormulaDeclaration, FormulaDeclarationSource, FormulaOrigin, FormulaReservedName};
use crate::name::MachineToken;
use core::fmt;
use std::collections::BTreeMap;

mod ordered;
pub use ordered::{FormulaNameCursor, FormulaStatementNameScope};
mod scope;
pub use scope::{FormulaGeometryDeclaration, FormulaNameScope};

/// Authored input or prior-operation metadata admitted to an initial namespace.
/// Recipe bindings and language-owned reserved sources cannot seed a recipe's inputs.
/// The retained declaration remains borrowed and grants no canonical registry/geometry proof.
/// ```compile_fail
/// fn forge(d: sc_core::recipe::FormulaDeclaration<'_>) {
///     let _ = sc_core::recipe::FormulaInitialDeclaration(d);
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct FormulaInitialDeclaration<'a>(FormulaDeclaration<'a>);
impl<'a> FormulaInitialDeclaration<'a> {
    /// Original immutable declaration, with its exact name and source identities.
    #[must_use]
    pub const fn declaration(self) -> FormulaDeclaration<'a> {
        self.0
    }
}
impl<'a> TryFrom<FormulaDeclaration<'a>> for FormulaInitialDeclaration<'a> {
    type Error = FormulaDeclaration<'a>;

    /// Admit authored inputs and geometry references without reading state or resolving geometry.
    /// Returns the original rejected declaration for a recipe or reserved source.
    fn try_from(declaration: FormulaDeclaration<'a>) -> Result<Self, Self::Error> {
        match declaration.source() {
            FormulaDeclarationSource::Input { .. }
            | FormulaDeclarationSource::LengthInput { .. }
            | FormulaDeclarationSource::Point(_)
            | FormulaDeclarationSource::Edge(_) => Ok(Self(declaration)),
            FormulaDeclarationSource::Recipe { .. } | FormulaDeclarationSource::Reserved(_) => {
                Err(declaration)
            }
        }
    }
}

/// Binding-name collision, retaining both real sources without copied values.
/// Debug omits authored names/identities through the declaration's opaque formatter.
#[derive(Clone, Debug)]
pub enum FormulaNamespaceError<'a> {
    /// A language-owned reserved name cannot be authored by any source domain.
    ReservedBinding {
        /// Fixed kind/origin/required context; there is no prior recipe statement.
        reserved: FormulaReservedName,
        /// Actual attempted authored source; there is no invented recipe ordinal.
        attempted: FormulaDeclaration<'a>,
    },
    /// Two authored declarations collide, including two from the same origin.
    AmbiguousName {
        /// Earlier and attempted declarations in authored order; boxed to keep the error compact.
        /// Only metadata handles are copied; canonical records remain borrowed.
        sources: Box<[FormulaDeclaration<'a>; 2]>,
    },
    /// A recipe let repeats a name bound by an earlier let in this same recipe.
    RecipeRebinding {
        /// Earlier and attempted actual recipe declarations, including both ordinals and spans.
        sources: Box<[FormulaDeclaration<'a>; 2]>,
    },
}
impl<'a> FormulaNamespaceError<'a> {
    /// Stable internal diagnostic token; localized user presentation belongs to the command layer.
    #[must_use]
    pub const fn token(&self) -> &'static str {
        match self {
            Self::ReservedBinding { .. } => "formula_rebinding",
            Self::AmbiguousName { .. } => "formula_ambiguous_name",
            Self::RecipeRebinding { .. } => "formula_rebinding",
        }
    }

    /// Explicit collision name. Default diagnostic formatting does not expose it.
    #[must_use]
    pub fn name(&self) -> &'a str {
        match self {
            Self::ReservedBinding { attempted, .. } => attempted.name(),
            Self::AmbiguousName { sources } | Self::RecipeRebinding { sources } => {
                let [_, attempted] = **sources;
                attempted.name()
            }
        }
    }

    /// Prior/reserved and attempted source declarations in that order.
    /// Reserved metadata is available without fetching a runtime provider or inventing an ordinal.
    #[must_use]
    pub fn binding_sources(&self) -> [FormulaDeclaration<'a>; 2] {
        match self {
            Self::ReservedBinding {
                reserved,
                attempted,
            } => [FormulaDeclaration::reserved(*reserved), *attempted],
            Self::AmbiguousName { sources } => **sources,
            Self::RecipeRebinding { sources } => **sources,
        }
    }
}
impl fmt::Display for FormulaNamespaceError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.token())
    }
}
impl std::error::Error for FormulaNamespaceError<'_> {}

/// An exact machine name is absent from the current flat metadata namespace.
/// This static refusal carries no state, value or guessed recipe/expression context.
/// ```compile_fail
/// fn forge() -> sc_core::recipe::FormulaUnboundName<'static> {
///     sc_core::recipe::FormulaUnboundName { name: "missing" }
/// }
/// ```
/// ```compile_fail
/// fn escaped() -> sc_core::recipe::FormulaUnboundName<'static> {
///     let name = sc_core::name::MachineToken::new("missing").unwrap();
///     sc_core::recipe::FormulaNamespace::new([]).unwrap().resolve(&name).unwrap_err()
/// }
/// ```
#[derive(Clone, Copy)]
pub struct FormulaUnboundName<'a> {
    name: &'a str,
}
impl<'a> FormulaUnboundName<'a> {
    /// Exact borrowed query. Default formatting omits this authored payload.
    #[must_use]
    pub const fn name(self) -> &'a str {
        self.name
    }

    /// Stable internal diagnostic token; a runtime unknown-value refusal is a separate condition.
    #[must_use]
    pub const fn token(self) -> &'static str {
        "formula_unbound_name"
    }

    /// Complete flat namespace domains, including the empty recipe domain before statement one.
    /// Their metadata is searched without consulting any runtime value provider.
    #[must_use]
    pub const fn origins_searched(self) -> &'static [FormulaOrigin; 9] {
        &FormulaOrigin::ALL
    }
}
impl fmt::Debug for FormulaUnboundName<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaUnboundName").finish_non_exhaustive()
    }
}
impl fmt::Display for FormulaUnboundName<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.token())
    }
}
impl std::error::Error for FormulaUnboundName<'_> {}

/// Immutable checked initial namespace: authored pairs plus all eight fixed reserved sources.
/// No partial namespace escapes a collision. Construction reads metadata only, without values,
/// availability, expression checking or operation-order/registry validation.
/// ```compile_fail
/// fn alter(ns: &mut sc_core::recipe::FormulaNamespace<'_>) { ns.entries.clear(); }
/// ```
/// ```compile_fail
/// fn escaped() -> sc_core::recipe::FormulaNamespace<'static> {
///     use sc_core::{name::MachineToken, ontology::{EntityId, LocalTag, PointRef}, recipe::{
///         FormulaDeclaration, FormulaInitialDeclaration, FormulaNamespace,
///     }};
///     let name = MachineToken::new("constructed_point").unwrap();
///     let declaration = FormulaDeclaration::point(&name, PointRef::new(EntityId::from_bits(1), LocalTag::FIRST));
///     FormulaNamespace::new([FormulaInitialDeclaration::try_from(declaration).unwrap()]).unwrap()
/// }
/// ```
#[derive(Clone)]
pub struct FormulaNamespace<'a> {
    entries: BTreeMap<&'a str, FormulaDeclaration<'a>>,
}
impl<'a> FormulaNamespace<'a> {
    /// Consume declarations in authored order and reject the first collision before insertion.
    /// Required runtime contexts do not affect whether reserved metadata is declared.
    /// # Errors
    /// Returns [`FormulaNamespaceError`] for a reserved-name attempt or two authored declarations
    /// with the same name. Both sources remain intact; no input is overwritten or shadowed.
    pub fn new(
        declarations: impl IntoIterator<Item = FormulaInitialDeclaration<'a>>,
    ) -> Result<Self, FormulaNamespaceError<'a>> {
        let mut entries: BTreeMap<&'a str, FormulaDeclaration<'a>> = BTreeMap::new();
        for reserved in FormulaReservedName::ALL {
            let declaration = FormulaDeclaration::reserved(reserved);
            entries.insert(declaration.name(), declaration);
        }
        let mut namespace = Self { entries };
        for initial in declarations {
            namespace.insert_declaration(initial.declaration())?;
        }
        Ok(namespace)
    }

    // Shared original-source collision rules; a metadata insertion never grants program acceptance.
    fn check_declaration(
        &self,
        attempted: FormulaDeclaration<'a>,
    ) -> Result<(), FormulaNamespaceError<'a>> {
        let Some(prior) = self.entries.get(attempted.name()).copied() else {
            return Ok(());
        };
        Err(match (prior.source(), attempted.source()) {
            (FormulaDeclarationSource::Reserved(reserved), _) => {
                FormulaNamespaceError::ReservedBinding {
                    reserved,
                    attempted,
                }
            }
            (FormulaDeclarationSource::Recipe { .. }, FormulaDeclarationSource::Recipe { .. }) => {
                FormulaNamespaceError::RecipeRebinding {
                    sources: Box::new([prior, attempted]),
                }
            }
            _ => FormulaNamespaceError::AmbiguousName {
                sources: Box::new([prior, attempted]),
            },
        })
    }
    fn insert_declaration(
        &mut self,
        attempted: FormulaDeclaration<'a>,
    ) -> Result<(), FormulaNamespaceError<'a>> {
        self.check_declaration(attempted)?;
        self.entries.insert(attempted.name(), attempted);
        Ok(())
    }

    /// Extend actual initial metadata in supplied order using the same reserved/collision rules.
    /// This consumes the original map; no extended prefix escapes a later collision. It reads
    /// no values and grants no containing owner,geometry,currentness or whole-program admission.
    /// # Errors
    /// Original first reserved/ambiguous pair, retaining both actual declaration sources.
    pub fn with_initial(
        mut self,
        declarations: impl IntoIterator<Item = FormulaInitialDeclaration<'a>>,
    ) -> Result<Self, FormulaNamespaceError<'a>> {
        for initial in declarations {
            self.insert_declaration(initial.declaration())?;
        }
        Ok(self)
    }

    /// Borrow readonly resolution/checking without exposing a mutable or cloneable map handle.
    #[must_use]
    pub const fn visible_names(&self) -> FormulaNameScope<'_, 'a> {
        FormulaNameScope::new(self)
    }

    /// Inspect exact immutable declarations in lexical name order, independently of recipe order.
    pub fn declarations(&self) -> impl ExactSizeIterator<Item = FormulaDeclaration<'a>> + '_ {
        self.entries.values().copied()
    }

    /// Resolve an exact validated machine name to its original declaration metadata.
    /// Declared unknown values and absent optional contexts retain their known kinds.
    /// # Errors
    /// Returns [`FormulaUnboundName`] borrowing the query when no declaration has this spelling.
    /// No value, state, context availability or geometry is read, and no name is repaired.
    pub fn resolve<'n>(
        &self,
        name: &'n MachineToken,
    ) -> Result<FormulaDeclaration<'a>, FormulaUnboundName<'n>> {
        self.resolve_name(name.as_str())
    }

    /// Parser-validated name lookup for later semantic checking; external queries use MachineToken.
    pub(super) fn resolve_name<'n>(
        &self,
        name: &'n str,
    ) -> Result<FormulaDeclaration<'a>, FormulaUnboundName<'n>> {
        self.entries
            .get(name)
            .copied()
            .ok_or(FormulaUnboundName { name })
    }
}
impl fmt::Debug for FormulaNamespace<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaNamespace")
            .field("declaration_count", &self.entries.len())
            .finish_non_exhaustive()
    }
}
