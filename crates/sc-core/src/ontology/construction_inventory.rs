//! Exact six-family construction ownership and complete current graph/copy/Piece/source scope.
use super::{
    CutPlan, Dart, DartError, EntityId, FoldError, Gather, GatherError, Grainline, GrainlineError,
    IdentityLedger, Piece, Pleat, SeamAllowance, SeamAllowanceError, SewingError, SewingGraph,
    SewingLandmark, Tuck,
};
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

/// Original ownership or complete current-context/source refusal, without repaired content.
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
    /// Original complete graph/copy/Piece/landmark/distribution context refusal.
    Graph(Box<SewingError>),
    /// An actual construction's original held Piece is absent; no replacement is selected.
    MissingPiece {
        /// Actual construction id.
        construction: EntityId,
        /// Original family and authored index.
        position: ConstructionInventoryPosition,
        /// Original held Piece id.
        piece: EntityId,
    },
    /// Original Dart source refusal.
    Dart {
        /// Actual construction id.
        construction: EntityId,
        /// Original family and authored index.
        position: ConstructionInventoryPosition,
        /// Original source cause and raw evidence.
        issue: Box<DartError>,
    },
    /// Original Tuck source refusal.
    Tuck {
        /// Actual construction id.
        construction: EntityId,
        /// Original family and authored index.
        position: ConstructionInventoryPosition,
        /// Original source cause and raw evidence.
        issue: Box<FoldError>,
    },
    /// Original Pleat source refusal, distinct from a Tuck.
    Pleat {
        /// Actual construction id.
        construction: EntityId,
        /// Original family and authored index.
        position: ConstructionInventoryPosition,
        /// Original source cause and raw evidence.
        issue: Box<FoldError>,
    },
    /// Original Gather selected-copy/attachment/direction refusal.
    Gather {
        /// Actual construction id.
        construction: EntityId,
        /// Original family and authored index.
        position: ConstructionInventoryPosition,
        /// Original source cause and raw evidence.
        issue: Box<GatherError>,
    },
    /// Original grain/print source refusal.
    Grainline {
        /// Actual construction id.
        construction: EntityId,
        /// Original family and authored index.
        position: ConstructionInventoryPosition,
        /// Original source cause and raw evidence.
        issue: Box<GrainlineError>,
    },
    /// Original complete allowance-edge refusal.
    SeamAllowance {
        /// Actual construction id.
        construction: EntityId,
        /// Original family and authored index.
        position: ConstructionInventoryPosition,
        /// Original source cause and raw evidence.
        issue: Box<SeamAllowanceError>,
    },
}
impl fmt::Display for ConstructionInventoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateIdentity { identity, first, duplicate } => write!(
                f, "construction identity {identity} repeats at {duplicate:?}, first supplied at {first:?}"
            ),
            Self::Graph(issue) => write!(f, "construction current graph context: {issue}"),
            Self::MissingPiece { construction, position, piece } => write!(
                f, "construction {construction} at {position:?} lacks original Piece {piece}"
            ),
            Self::Dart { construction, position, issue } => write!(f, "Dart {construction} at {position:?}: {issue}"),
            Self::Tuck { construction, position, issue } => write!(f, "Tuck {construction} at {position:?}: {issue}"),
            Self::Pleat { construction, position, issue } => write!(f, "Pleat {construction} at {position:?}: {issue}"),
            Self::Gather { construction, position, issue } => write!(f, "Gather {construction} at {position:?}: {issue}"),
            Self::Grainline { construction, position, issue } => write!(f, "Grainline {construction} at {position:?}: {issue}"),
            Self::SeamAllowance { construction, position, issue } => write!(f, "SeamAllowance {construction} at {position:?}: {issue}"),
        }
    }
}
impl std::error::Error for ConstructionInventoryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Graph(issue) => Some(issue.as_ref()),
            Self::Dart { issue, .. } => Some(issue.as_ref()),
            Self::Tuck { issue, .. } | Self::Pleat { issue, .. } => Some(issue.as_ref()),
            Self::Gather { issue, .. } => Some(issue.as_ref()),
            Self::Grainline { issue, .. } => Some(issue.as_ref()),
            Self::SeamAllowance { issue, .. } => Some(issue.as_ref()),
            Self::DuplicateIdentity { .. } | Self::MissingPiece { .. } => None,
        }
    }
}

/// Private ordered Dart/Tuck/Pleat/Gather/Grainline/SeamAllowance owners.
///
/// Construction checks owned ids only and preserves exact objects, including held repairs. Referenced
/// Pieces, operations, parameters and profile declarations are uses, not additional owners here.
/// Pocket/notion/Closure/layer/Hem inventories remain separate. This component alone proves no current
/// graph/copy/Piece sources; use `validate_current` for that structural scope. Declaration roles,
/// global Design identity and physical readiness remain separate obligations.
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
pub(crate) struct ConstructionResourceView<'a> {
    pub darts: &'a [Dart],
    pub tucks: &'a [Tuck],
    pub pleats: &'a [Pleat],
    pub gathers: &'a [Gather],
    pub grainlines: &'a [Grainline],
    pub allowances: &'a [SeamAllowance],
}
impl fmt::Debug for ConstructionResourceView<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConstructionResourceView")
            .field("dart_count", &self.darts.len())
            .field("tuck_count", &self.tucks.len())
            .field("pleat_count", &self.pleats.len())
            .field("gather_count", &self.gathers.len())
            .field("grainline_count", &self.grainlines.len())
            .field("allowance_count", &self.allowances.len())
            .finish_non_exhaustive()
    }
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
    pub(crate) fn resource_collections(&self) -> ConstructionResourceView<'_> {
        let Self {
            darts,
            tucks,
            pleats,
            gathers,
            grainlines,
            allowances,
        } = self;
        ConstructionResourceView {
            darts,
            tucks,
            pleats,
            gathers,
            grainlines,
            allowances,
        }
    }
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
    /// Validate the complete current graph context, then every original owned source in entry order.
    ///
    /// Whole graph validation includes CutPlan quantities/orientations, unique Piece context, all
    /// spans, stops and distributions, even for an empty inventory. Original held Pieces are selected
    /// only after that preflight. No references are rewritten, endpoint choices inferred, results
    /// cached or declarations evaluated. Success is not full Design or physical/release readiness.
    /// # Errors
    /// Returns the first original graph cause, missing held Piece, or concrete family source cause
    /// with actual construction id and original family/index. No partial success is returned.
    pub fn validate_current(
        &self,
        pieces: &[Piece],
        plan: &CutPlan,
        graph: &SewingGraph,
        landmarks: &[SewingLandmark],
        ledger: &IdentityLedger,
    ) -> Result<(), ConstructionInventoryError> {
        graph
            .validate_current(plan, pieces, landmarks, ledger)
            .map_err(|issue| ConstructionInventoryError::Graph(Box::new(issue)))?;
        let pieces = pieces
            .iter()
            .map(|piece| (piece.id(), piece))
            .collect::<BTreeMap<_, _>>();
        for (position, source) in self.entries() {
            let construction = source.id();
            let piece =
                pieces
                    .get(&source.piece())
                    .ok_or(ConstructionInventoryError::MissingPiece {
                        construction,
                        position,
                        piece: source.piece(),
                    })?;
            macro_rules! validate {
                ($value:expr, $kind:ident, $($context:expr),+) => {
                    $value.validate_current($($context),+).map_err(|issue| {
                        ConstructionInventoryError::$kind { construction, position, issue: Box::new(issue) }
                    })
                };
            }
            match source {
                ConstructionRef::Dart(value) => validate!(value, Dart, piece, ledger),
                ConstructionRef::Tuck(value) => validate!(value, Tuck, piece, ledger),
                ConstructionRef::Pleat(value) => validate!(value, Pleat, piece, ledger),
                ConstructionRef::Gather(value) => {
                    validate!(value, Gather, piece, plan, graph, ledger)
                }
                ConstructionRef::Grainline(value) => validate!(value, Grainline, piece, ledger),
                ConstructionRef::SeamAllowance(value) => {
                    validate!(value, SeamAllowance, piece, ledger)
                }
            }?;
        }
        Ok(())
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
