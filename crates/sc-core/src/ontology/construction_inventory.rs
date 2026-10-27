//! Immutable ownership of the six remaining construction-source families; current scope is separate.
use super::{Dart, EntityId, Gather, Grainline, Pleat, SeamAllowance, Tuck};
use core::fmt;
use std::collections::BTreeMap;

/// Exact semantic family in deterministic Dart/Tuck/Pleat/Gather/Grainline/SeamAllowance order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConstructionKind {
    /// Dart intake and closing intent.
    Dart,
    /// Distinct tuck intent.
    Tuck,
    /// Distinct pleat intent.
    Pleat,
    /// Physical-side gathering intent.
    Gather,
    /// Directed grain and print intent.
    Grainline,
    /// Per-edge allowance and inclusion intent.
    SeamAllowance,
}

/// Original family and zero-based authored position; positions never create entity identities.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConstructionInventoryPosition {
    /// Actual distinct family.
    pub kind: ConstructionKind,
    /// Original authored position in that family.
    pub index: usize,
}

/// Borrow of an actual distinct stored construction; inspection is no current-validation certificate.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ConstructionRef<'a> {
    /// Actual stored Dart.
    Dart(&'a Dart),
    /// Actual stored Tuck.
    Tuck(&'a Tuck),
    /// Actual stored Pleat.
    Pleat(&'a Pleat),
    /// Actual stored Gather.
    Gather(&'a Gather),
    /// Actual stored Grainline.
    Grainline(&'a Grainline),
    /// Actual stored SeamAllowance.
    SeamAllowance(&'a SeamAllowance),
}
impl fmt::Debug for ConstructionRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConstructionRef")
            .field("kind", &self.kind())
            .finish_non_exhaustive()
    }
}
impl ConstructionRef<'_> {
    /// Original semantic family, never inferred from label or numeric id.
    #[must_use]
    pub const fn kind(self) -> ConstructionKind {
        match self {
            Self::Dart(_) => ConstructionKind::Dart,
            Self::Tuck(_) => ConstructionKind::Tuck,
            Self::Pleat(_) => ConstructionKind::Pleat,
            Self::Gather(_) => ConstructionKind::Gather,
            Self::Grainline(_) => ConstructionKind::Grainline,
            Self::SeamAllowance(_) => ConstructionKind::SeamAllowance,
        }
    }
    /// Exact original owned identity.
    #[must_use]
    pub const fn id(self) -> EntityId {
        match self {
            Self::Dart(value) => value.id(),
            Self::Tuck(value) => value.id(),
            Self::Pleat(value) => value.id(),
            Self::Gather(value) => value.id(),
            Self::Grainline(value) => value.id(),
            Self::SeamAllowance(value) => value.id(),
        }
    }
    /// Original held source Piece, without lookup or retargeting.
    #[must_use]
    pub const fn piece(self) -> EntityId {
        match self {
            Self::Dart(value) => value.piece(),
            Self::Tuck(value) => value.piece(),
            Self::Pleat(value) => value.piece(),
            Self::Gather(value) => value.piece(),
            Self::Grainline(value) => value.piece(),
            Self::SeamAllowance(value) => value.piece(),
        }
    }
}

/// Original duplicate ownership refusal, independent of source/metadata/geometry validation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConstructionInventoryError {
    /// Two objects across any stored families claim the same owned id.
    DuplicateIdentity {
        /// Actual repeated id.
        identity: EntityId,
        /// Original first owning location.
        first: ConstructionInventoryPosition,
        /// Original first repeated location.
        duplicate: ConstructionInventoryPosition,
    },
}
impl fmt::Display for ConstructionInventoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateIdentity { identity, first, duplicate } => write!(
                f, "construction identity {identity} repeats at {duplicate:?}, first supplied at {first:?}"
            ),
        }
    }
}
impl std::error::Error for ConstructionInventoryError {}

/// Private ordered Dart/Tuck/Pleat/Gather/Grainline/SeamAllowance owners.
///
/// Construction checks owned ids only and preserves exact objects, including held repairs. Referenced
/// Pieces, operations, parameters and profile declarations are uses, not additional owners here.
/// Pocket/notion/Closure/layer/Hem inventories remain separate. This component alone proves no current
/// graph/copy/Piece sources, declaration roles, global Design identity or physical readiness.
///
/// ```compile_fail
/// fn edit(value: &mut sc_core::ontology::ConstructionInventory) { value.darts.clear(); }
/// ```
/// ```compile_fail
/// fn edit(value: &mut sc_core::ontology::ConstructionInventory) { value.gathers().get_mut(0); }
/// ```
/// ```compile_fail
/// fn escape(value: sc_core::ontology::ConstructionInventory)
///     -> sc_core::ontology::ConstructionRef<'static> {
///     value.entries().next().unwrap().1
/// }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct ConstructionInventory {
    darts: Vec<Dart>,
    tucks: Vec<Tuck>,
    pleats: Vec<Pleat>,
    gathers: Vec<Gather>,
    grainlines: Vec<Grainline>,
    allowances: Vec<SeamAllowance>,
}
impl fmt::Debug for ConstructionInventory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConstructionInventory")
            .field("dart_count", &self.darts.len())
            .field("tuck_count", &self.tucks.len())
            .field("pleat_count", &self.pleats.len())
            .field("gather_count", &self.gathers.len())
            .field("grainline_count", &self.grainlines.len())
            .field("allowance_count", &self.allowances.len())
            .finish_non_exhaustive()
    }
}
impl ConstructionInventory {
    /// Own exact family orders without sorting, rewriting, choosing or repairing held content.
    /// # Errors
    /// Returns the first duplicate id across all six families with both original positions.
    pub fn new(
        darts: Vec<Dart>,
        tucks: Vec<Tuck>,
        pleats: Vec<Pleat>,
        gathers: Vec<Gather>,
        grainlines: Vec<Grainline>,
        allowances: Vec<SeamAllowance>,
    ) -> Result<Self, ConstructionInventoryError> {
        let inventory = Self {
            darts,
            tucks,
            pleats,
            gathers,
            grainlines,
            allowances,
        };
        let mut seen = BTreeMap::new();
        for (duplicate, value) in inventory.entries() {
            if let Some(first) = seen.insert(value.id(), duplicate) {
                return Err(ConstructionInventoryError::DuplicateIdentity {
                    identity: value.id(),
                    first,
                    duplicate,
                });
            }
        }
        Ok(inventory)
    }
    /// Original authored Dart order, without current-source certification.
    #[must_use]
    pub fn darts(&self) -> &[Dart] {
        &self.darts
    }
    /// Original authored Tuck order, without current-source certification.
    #[must_use]
    pub fn tucks(&self) -> &[Tuck] {
        &self.tucks
    }
    /// Original authored Pleat order, without current-source certification.
    #[must_use]
    pub fn pleats(&self) -> &[Pleat] {
        &self.pleats
    }
    /// Original authored Gather order, without current-source certification.
    #[must_use]
    pub fn gathers(&self) -> &[Gather] {
        &self.gathers
    }
    /// Original authored Grainline order, without current-source certification.
    #[must_use]
    pub fn grainlines(&self) -> &[Grainline] {
        &self.grainlines
    }
    /// Original authored SeamAllowance order, without current-source certification.
    #[must_use]
    pub fn allowances(&self) -> &[SeamAllowance] {
        &self.allowances
    }
    /// Typed actual borrows in documented family order, then original authored family order.
    pub fn entries(
        &self,
    ) -> impl Iterator<Item = (ConstructionInventoryPosition, ConstructionRef<'_>)> {
        macro_rules! family {
            ($values:expr, $kind:ident) => {
                $values.iter().enumerate().map(|(index, value)| {
                    (
                        ConstructionInventoryPosition {
                            kind: ConstructionKind::$kind,
                            index,
                        },
                        ConstructionRef::$kind(value),
                    )
                })
            };
        }
        family!(self.darts, Dart)
            .chain(family!(self.tucks, Tuck))
            .chain(family!(self.pleats, Pleat))
            .chain(family!(self.gathers, Gather))
            .chain(family!(self.grainlines, Grainline))
            .chain(family!(self.allowances, SeamAllowance))
    }
}
