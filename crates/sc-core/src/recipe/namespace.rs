//! Checked immutable initial declarations; reserved kinds do not require runtime value providers.
use super::{FormulaDeclaration, FormulaDeclarationSource, FormulaReservedName};
use core::fmt;
use std::collections::{btree_map::Entry, BTreeMap};

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

/// First initial-name collision, retaining both real binding sources without copied values.
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
}
impl<'a> FormulaNamespaceError<'a> {
    /// Stable internal diagnostic token; localized user presentation belongs to the command layer.
    #[must_use]
    pub const fn token(&self) -> &'static str {
        match self {
            Self::ReservedBinding { .. } => "formula_rebinding",
            Self::AmbiguousName { .. } => "formula_ambiguous_name",
        }
    }

    /// Explicit collision name. Default diagnostic formatting does not expose it.
    #[must_use]
    pub fn name(&self) -> &'a str {
        match self {
            Self::ReservedBinding { attempted, .. } => attempted.name(),
            Self::AmbiguousName { sources } => {
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
        }
    }
}
impl fmt::Display for FormulaNamespaceError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.token())
    }
}
impl std::error::Error for FormulaNamespaceError<'_> {}

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
        for initial in declarations {
            let attempted = initial.declaration();
            match entries.entry(attempted.name()) {
                Entry::Vacant(slot) => {
                    slot.insert(attempted);
                }
                Entry::Occupied(slot) => {
                    return Err(match slot.get().source() {
                        FormulaDeclarationSource::Reserved(reserved) => {
                            FormulaNamespaceError::ReservedBinding {
                                reserved,
                                attempted,
                            }
                        }
                        _ => FormulaNamespaceError::AmbiguousName {
                            sources: Box::new([*slot.get(), attempted]),
                        },
                    });
                }
            }
        }
        Ok(Self { entries })
    }

    /// Inspect exact immutable declarations in lexical name order, independently of recipe order.
    #[must_use]
    pub fn declarations(&self) -> impl ExactSizeIterator<Item = FormulaDeclaration<'a>> + '_ {
        self.entries.values().copied()
    }
}
impl fmt::Debug for FormulaNamespace<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaNamespace")
            .field("declaration_count", &self.entries.len())
            .finish_non_exhaustive()
    }
}
