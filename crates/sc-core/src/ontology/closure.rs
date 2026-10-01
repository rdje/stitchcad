//! Stable closure instances and canonical physical placements; realized hardware remains G2/G3.
use super::{
    CutPlan, EntityId, GeometricValidation, IdentityLedger, NotionPlacement, NotionPlacementError,
    Piece, ProfileBindingValidation, ProfileParameterRef,
};
use core::fmt;
use sc_units::{Count, Length};
use std::collections::{BTreeMap, BTreeSet};

/// Positive zipper-length provenance, without cached formula/profile results.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClosureLength {
    /// Authored positive length.
    Explicit {
        /// Stable parameter declaration.
        parameter: EntityId,
        /// Authored length.
        value: Length,
    },
    /// Formula declaration expected to resolve to a positive length.
    Formula(EntityId),
    /// Target-profile declaration expected to resolve to a positive length.
    Profile(ProfileParameterRef),
}
/// Required logical hardware size origin; its owner supplies domain, selection and uncertainty.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotionSize {
    /// Recipe/Design declaration, without an inferred vendor designation.
    Declaration(EntityId),
    /// Target-profile declaration, without a default selection.
    Profile(ProfileParameterRef),
}
/// Recipe operation deriving hole length from the canonical button-size declaration.
/// No independent hole length can be authored through this input.
/// ```compile_fail
/// use sc_core::ontology::{ButtonholeDerivation, EntityId};
/// use sc_units::Length;
/// let derivation = ButtonholeDerivation {
///     operation: EntityId::from_bits(1),
///     length: Length::from_micrometres(20_000).unwrap(),
/// };
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonholeDerivation {
    /// Recipe validates operation existence, kind and dependency on this Closure's button size.
    pub operation: EntityId,
}
/// Deriving physical hole length is separate from retaining its canonical source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonholeValidation {
    /// G3 must execute the typed operation using resolved button size and declared policy inputs.
    DeferredToG3,
}
/// Read-only borrowed provenance view; no independently entered or cached hole length.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonholeLengthSource<'a> {
    closure: EntityId,
    button_size: &'a NotionSize,
    derivation: &'a ButtonholeDerivation,
}
impl<'a> ButtonholeLengthSource<'a> {
    /// Closure owning the canonical button-size and derivation input.
    #[must_use]
    pub const fn closure(self) -> EntityId {
        self.closure
    }
    /// Borrowed canonical size declaration; its owner supplies selected value/state.
    #[must_use]
    pub const fn button_size(self) -> &'a NotionSize {
        self.button_size
    }
    /// Borrowed canonical recipe operation declaration.
    #[must_use]
    pub const fn derivation(self) -> &'a ButtonholeDerivation {
        self.derivation
    }
    /// Actual derived physical length remains uncomputed and unproved at G1.
    #[must_use]
    pub const fn validation(self) -> ButtonholeValidation {
        ButtonholeValidation::DeferredToG3
    }
}
/// Authored closure kind; unsupported requests receive explicit envelope diagnostics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClosureKind {
    /// Centred zipper; the two instance placements address its two sides.
    CentredZipper {
        /// Required zipper length origin.
        length: ClosureLength,
    },
    /// Hook and bar; first placement is the hook, second the bar.
    HookAndBar {
        /// Required hook size declaration.
        hook: NotionSize,
        /// Required bar size declaration.
        bar: NotionSize,
    },
    /// Button/buttonhole pairs; first placement is button, second hole.
    ButtonAndButtonhole {
        /// Required canonical logical button-size origin.
        button: NotionSize,
        /// Recipe operation deriving hole length; no separate physical length field.
        hole: ButtonholeDerivation,
    },
    /// Fly request, refused in the v1 envelope before constructing geometry.
    Fly {
        /// Declared trousers gap this request depends on.
        trousers_gap: EntityId,
    },
}
/// One stable physical closure instance, with no independent count or copied placement definition.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClosureInstance {
    /// Stable instance identity, never its position in a Vec.
    pub id: EntityId,
    /// First stable notion-placement identity.
    pub first: EntityId,
    /// Second stable notion-placement identity.
    pub second: EntityId,
}
/// Component roles in deterministic validation/query order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClosurePlacementRole {
    /// First zipper side, hook or button.
    First,
    /// Second zipper side, bar or buttonhole.
    Second,
}
impl ClosureInstance {
    /// Stable target id for the requested component role.
    #[must_use]
    pub const fn placement(self, role: ClosurePlacementRole) -> EntityId {
        match role {
            ClosurePlacementRole::First => self.first,
            ClosurePlacementRole::Second => self.second,
        }
    }
}
/// Editable authored closure intent; counts derive from the instance list.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClosureDefinition {
    /// Stable closure identity.
    pub id: EntityId,
    /// Explicit kind and required size origins.
    pub kind: ClosureKind,
    /// Nonempty stable instances; each component placement is used at most once.
    pub instances: Vec<ClosureInstance>,
}
/// Named unsupported-envelope refusal, independent of physical validation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClosureEnvelopeError {
    /// Fly construction remains outside v1 execution support.
    FlyDeferred {
        /// Requested closure identity.
        closure: EntityId,
        /// Declared trousers gap the request depends on.
        trousers_gap: EntityId,
    },
}
impl ClosureEnvelopeError {
    /// Canonical feature-matrix diagnostic token.
    #[must_use]
    pub const fn diagnostic(self) -> &'static str {
        match self {
            Self::FlyDeferred { .. } => "env_fly",
        }
    }
    /// Gate that owns this envelope limitation, not a promise of future fly support.
    #[must_use]
    pub const fn proving_gate(self) -> &'static str {
        match self {
            Self::FlyDeferred { .. } => "G7",
        }
    }
}
impl fmt::Display for ClosureEnvelopeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FlyDeferred { closure, trousers_gap } => write!(f, "env_fly: closure {closure} depends on trousers gap {trousers_gap}; G7 owns the limitation"),
        }
    }
}
impl std::error::Error for ClosureEnvelopeError {}
impl ClosureDefinition {
    /// Check requested execution scope before inspecting geometry or supplying substitutes.
    /// # Errors
    /// Fly returns env_fly with requested closure, trousers gap and proving gate G7.
    pub const fn require_in_scope(&self) -> Result<(), ClosureEnvelopeError> {
        match self.kind {
            ClosureKind::Fly { trousers_gap } => Err(ClosureEnvelopeError::FlyDeferred {
                closure: self.id,
                trousers_gap,
            }),
            ClosureKind::CentredZipper { .. }
            | ClosureKind::HookAndBar { .. }
            | ClosureKind::ButtonAndButtonhole { .. } => Ok(()),
        }
    }
}
/// Immutable structural closure, borrowing current placements through their stable ids.
/// ```compile_fail
/// fn change_closure(closure: &mut sc_core::ontology::Closure) {
///     closure.definition.instances.clear();
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Closure {
    definition: ClosureDefinition,
    // Derived once from private immutable instances, never an independently authored/serialized input.
    count: Count,
}
/// Structural or scope refusal with stable target identities.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClosureError {
    /// Unsupported execution request.
    OutsideEnvelope(ClosureEnvelopeError),
    /// At least one physical instance is required.
    NoInstances,
    /// Instance cardinality exceeds the Count domain.
    CountOverflow(usize),
    /// An authored zipper length is not positive.
    NonPositiveLength(Length),
    /// One instance identity appears more than once.
    DuplicateInstance(EntityId),
    /// The same component placement is used more than once, including in one pair.
    ReusedPlacement(EntityId),
    /// The supplied Piece registry has ambiguous identities.
    DuplicatePiece(EntityId),
    /// The supplied notion-placement registry has ambiguous identities.
    DuplicatePlacement(EntityId),
    /// No authored instance has the requested stable identity.
    MissingInstance(EntityId),
    /// A required notion-placement target is absent.
    MissingPlacement {
        /// Stable physical instance.
        instance: EntityId,
        /// Component role.
        role: ClosurePlacementRole,
        /// Held target.
        placement: EntityId,
    },
    /// The target's original source Piece is missing from the supplied current registry.
    MissingPiece {
        /// Placement target.
        placement: EntityId,
        /// Missing source owner.
        piece: EntityId,
    },
    /// A current placement requires repair or has a missing/reassigned copy target.
    InvalidPlacement {
        /// Stable physical instance.
        instance: EntityId,
        /// Component role.
        role: ClosurePlacementRole,
        /// Target identity.
        placement: EntityId,
        /// Current validation refusal with raw evidence.
        error: Box<NotionPlacementError>,
    },
}
impl fmt::Display for ClosureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutsideEnvelope(error) => error.fmt(f),
            Self::NoInstances => f.write_str("closure requires physical instances"),
            Self::CountOverflow(count) => {
                write!(f, "closure instance count {count} exceeds Count domain")
            }
            Self::NonPositiveLength(value) => write!(
                f,
                "closure length {} µm must be positive",
                value.as_micrometres()
            ),
            Self::DuplicateInstance(id) => write!(f, "duplicate closure instance {id}"),
            Self::ReusedPlacement(id) => write!(f, "closure placement {id} is reused"),
            Self::DuplicatePiece(id) => write!(f, "duplicate closure context piece {id}"),
            Self::DuplicatePlacement(id) => write!(f, "duplicate closure context placement {id}"),
            Self::MissingInstance(id) => write!(f, "closure instance {id} is absent"),
            Self::MissingPlacement {
                instance,
                role,
                placement,
            } => write!(
                f,
                "closure instance {instance} {role:?} placement {placement} is missing"
            ),
            Self::MissingPiece { placement, piece } => write!(
                f,
                "closure placement {placement} source piece {piece} is missing"
            ),
            Self::InvalidPlacement {
                instance,
                role,
                placement,
                error,
            } => write!(
                f,
                "closure instance {instance} {role:?} placement {placement}: {error}"
            ),
        }
    }
}
impl std::error::Error for ClosureError {}
impl Closure {
    /// Validate authored instances, sizes, unambiguous current registries and every placement.
    /// # Errors
    /// Returns [`ClosureError`] for scope/domain/count/identity/target failures. Recipe/profile size
    /// resolution, physical coincidence and hardware geometry remain later validation obligations.
    pub fn new(
        definition: ClosureDefinition,
        placements: &[NotionPlacement],
        plan: &CutPlan,
        pieces: &[Piece],
        ledger: &IdentityLedger,
    ) -> Result<Self, ClosureError> {
        definition
            .require_in_scope()
            .map_err(ClosureError::OutsideEnvelope)?;
        if definition.instances.is_empty() {
            return Err(ClosureError::NoInstances);
        }
        let count = instance_count(definition.instances.len())?;
        if let ClosureKind::CentredZipper {
            length: ClosureLength::Explicit { value, .. },
        } = definition.kind
        {
            if value.as_micrometres() <= 0 {
                return Err(ClosureError::NonPositiveLength(value));
            }
        }
        let mut instance_ids = BTreeSet::new();
        let mut placement_ids = BTreeSet::new();
        for instance in &definition.instances {
            if !instance_ids.insert(instance.id) {
                return Err(ClosureError::DuplicateInstance(instance.id));
            }
            for id in [instance.first, instance.second] {
                if !placement_ids.insert(id) {
                    return Err(ClosureError::ReusedPlacement(id));
                }
            }
        }
        let closure = Self { definition, count };
        closure.validate_current(placements, plan, pieces, ledger)?;
        Ok(closure)
    }
    /// Stable closure identity.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.definition.id
    }
    /// Authored definition, exposed without mutation.
    #[must_use]
    pub const fn definition(&self) -> &ClosureDefinition {
        &self.definition
    }
    /// Typed count derived from immutable instances, not independently entered.
    #[must_use]
    pub const fn count(&self) -> Count {
        self.count
    }
    /// Validate all current targets; this neither resolves size declarations nor grants release.
    /// # Errors
    /// Returns [`ClosureError`] for ambiguous registries or missing/invalid placement contexts.
    pub fn validate_current(
        &self,
        placements: &[NotionPlacement],
        plan: &CutPlan,
        pieces: &[Piece],
        ledger: &IdentityLedger,
    ) -> Result<(), ClosureError> {
        let context = PlacementContext::new(placements, pieces)?;
        for instance in &self.definition.instances {
            for role in [ClosurePlacementRole::First, ClosurePlacementRole::Second] {
                context.placement(*instance, role, plan, ledger)?;
            }
        }
        Ok(())
    }
    /// Borrow one canonical current target by stable instance id and component role.
    /// This validates only that target, not the other placements or physical/release readiness.
    /// # Errors
    /// Returns [`ClosureError`] for absent instance or ambiguous/missing/invalid target context.
    pub fn placement<'a>(
        &self,
        instance: EntityId,
        role: ClosurePlacementRole,
        placements: &'a [NotionPlacement],
        plan: &CutPlan,
        pieces: &[Piece],
        ledger: &IdentityLedger,
    ) -> Result<&'a NotionPlacement, ClosureError> {
        let instance = self
            .definition
            .instances
            .iter()
            .find(|held| held.id == instance)
            .ok_or(ClosureError::MissingInstance(instance))?;
        PlacementContext::new(placements, pieces)?.placement(*instance, role, plan, ledger)
    }
    /// Borrow the one canonical source of derived hole length; None means a different closure kind.
    /// This resolves neither size/policy values nor physical length, and does not validate placements.
    #[must_use]
    pub const fn buttonhole_length_source(&self) -> Option<ButtonholeLengthSource<'_>> {
        match &self.definition.kind {
            ClosureKind::ButtonAndButtonhole { button, hole } => Some(ButtonholeLengthSource {
                closure: self.id(),
                button_size: button,
                derivation: hole,
            }),
            _ => None,
        }
    }
    /// Actual hardware/attachment geometry remains unproved.
    #[must_use]
    pub const fn geometric_validation(&self) -> GeometricValidation {
        GeometricValidation::DeferredToG2
    }
    /// Present target-profile size fields stay unresolved; None only means none was authored.
    #[must_use]
    pub const fn profile_binding_validation(&self) -> Option<ProfileBindingValidation> {
        match self.definition.kind {
            ClosureKind::CentredZipper {
                length: ClosureLength::Profile(_),
            }
            | ClosureKind::HookAndBar {
                hook: NotionSize::Profile(_),
                ..
            }
            | ClosureKind::HookAndBar {
                bar: NotionSize::Profile(_),
                ..
            }
            | ClosureKind::ButtonAndButtonhole {
                button: NotionSize::Profile(_),
                ..
            } => Some(ProfileBindingValidation::DeferredToG4),
            _ => None,
        }
    }
}
fn instance_count(length: usize) -> Result<Count, ClosureError> {
    u32::try_from(length)
        .map(Count::new)
        .map_err(|_| ClosureError::CountOverflow(length))
}
struct PlacementContext<'a, 'p> {
    placements: BTreeMap<EntityId, &'a NotionPlacement>,
    pieces: BTreeMap<EntityId, &'p Piece>,
}
impl<'a, 'p> PlacementContext<'a, 'p> {
    fn new(placements: &'a [NotionPlacement], pieces: &'p [Piece]) -> Result<Self, ClosureError> {
        let mut context = Self {
            placements: BTreeMap::new(),
            pieces: BTreeMap::new(),
        };
        for piece in pieces {
            if context.pieces.insert(piece.id(), piece).is_some() {
                return Err(ClosureError::DuplicatePiece(piece.id()));
            }
        }
        for placement in placements {
            if context
                .placements
                .insert(placement.id(), placement)
                .is_some()
            {
                return Err(ClosureError::DuplicatePlacement(placement.id()));
            }
        }
        Ok(context)
    }
    fn placement(
        &self,
        instance: ClosureInstance,
        role: ClosurePlacementRole,
        plan: &CutPlan,
        ledger: &IdentityLedger,
    ) -> Result<&'a NotionPlacement, ClosureError> {
        let id = instance.placement(role);
        let placement =
            self.placements
                .get(&id)
                .copied()
                .ok_or(ClosureError::MissingPlacement {
                    instance: instance.id,
                    role,
                    placement: id,
                })?;
        let piece = self
            .pieces
            .get(&placement.piece())
            .ok_or(ClosureError::MissingPiece {
                placement: id,
                piece: placement.piece(),
            })?;
        placement
            .validate_current(plan, piece, ledger)
            .map_err(|error| ClosureError::InvalidPlacement {
                instance: instance.id,
                role,
                placement: id,
                error: Box::new(error),
            })?;
        Ok(placement)
    }
}
#[cfg(test)]
mod tests {
    use super::{instance_count, ClosureError};
    use sc_units::Count;
    #[test]
    fn count_domain_is_checked_without_allocating_billions_of_instances() {
        assert_eq!(instance_count(1), Ok(Count::new(1)));
        if let Ok(maximum) = usize::try_from(u32::MAX) {
            assert_eq!(instance_count(maximum), Ok(Count::new(u32::MAX)));
        }
        if let Ok(overflow) = usize::try_from(u64::from(u32::MAX) + 1) {
            assert_eq!(
                instance_count(overflow),
                Err(ClosureError::CountOverflow(overflow))
            );
        }
    }
}
