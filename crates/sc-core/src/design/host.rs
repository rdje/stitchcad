//! One-way integration with immutable metadata owners; canonical values stay in sc-core.
use crate::{
    ontology::EntityId,
    value::{CanonicalInputs, LengthDeclaration},
};
use core::fmt;
use std::{collections::BTreeMap, sync::Arc};

/// Metadata families currently supported by the Design source host.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DesignHostEntityKind {
    /// Measurement landmark.
    Landmark,
    /// Documented measurement procedure.
    Procedure,
    /// Measurement carrying an explicit canonical length binding.
    Measurement,
    /// Named measurement table.
    MeasurementTable,
    /// Body-to-garment Ease mapping.
    Ease,
    /// Named Ease set.
    EaseSet,
    /// External SizeSet identity from its canonical membership.
    SizeSet,
    /// Actual SizeSet member.
    SizeMember,
    /// Authored size axis.
    SizeAxis,
    /// Authored axis coordinate value metadata, separate from referenced canonical scalars.
    SizeAxisValue,
    /// Authored signed break declaration.
    SizeBreak,
    /// Actual garment or MTM chart.
    SizeChart,
    /// Actual garment chart observation.
    SizeChartObservation,
}

/// Owned metadata identity and its original location, without copied input state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DesignHostEntity {
    /// Actual metadata identity, excluding referenced declarations/evidence.
    pub id: EntityId,
    /// Actual owning family.
    pub kind: DesignHostEntityKind,
    /// Zero-based authored position within that family.
    pub index: usize,
}

/// Readonly integration contract implemented by a higher-level immutable metadata owner.
///
/// Implementations must enumerate every owned metadata entity in stable family/authored order,
/// exclude referenced canonical inputs, retain exact bindings and validate actual current objects.
/// Shared access must not mutate that snapshot. Core checks identity disjointness and actual
/// canonical object membership; it relies on the implementation's semantic validation. This is
/// an integration contract, not a security boundary against a dishonest implementation.
/// No core dependency on the implementing crate or stored borrowed context is required.
pub trait DesignMetadataHost {
    /// Original typed current-reference or binding refusal.
    type Error: std::error::Error + 'static;

    /// Complete owned metadata census, preserving original family/position information.
    fn entities(&self) -> Vec<DesignHostEntity>;

    /// Validate every actual owned record against these explicit canonical inputs.
    ///
    /// # Errors
    /// Returns the implementation's actual current-reference refusal, without fallback values.
    fn validate_current(&self, canonical: &CanonicalInputs) -> Result<(), Self::Error>;

    /// Resolve the measurement's explicit binding to an actual dedicated canonical length.
    /// Unknown/Derived states must remain unchanged; matching numeric ids alone prove no binding.
    ///
    /// # Errors
    /// Returns the implementation's missing measurement/binding/current-reference refusal.
    fn measurement_declaration<'a>(
        &'a self,
        measurement: EntityId,
        canonical: &'a CanonicalInputs,
    ) -> Result<&'a LengthDeclaration, Self::Error>;
}

/// Additional readonly external-SizeSet facet for actual combined source owners.
///
/// Implementations validate the requested exact reference against their actual current complete
/// source inventory. Presence/coverage/numeric availability remain distinct. This shares the
/// metadata port's immutable-provider obligation; it supplies no Factory Profile resolution.
pub trait DesignSizeSetHost: DesignMetadataHost {
    /// Require the exact Design-authored set id/revision after current source validation.
    ///
    /// # Errors
    /// Returns original current inventory, missing-id or revision-mismatch cause, with no fallback.
    fn validate_size_set(
        &self,
        reference: super::DesignSizeSetReference,
        canonical: &CanonicalInputs,
    ) -> Result<(), Self::Error>;
}

/// Exact authored source references; None records an explicitly unprovided draft field.
/// This carrier performs no lookup, selection, currentness or completeness proof and has no Default.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DesignSourceReferences {
    /// Selected logical measurement table identity, if provided.
    pub measurement_table: Option<EntityId>,
    /// Selected Ease set identity, if provided; its garment table must match a provided table.
    pub ease_set: Option<EntityId>,
    /// Selected exact external SizeSet reference, if provided.
    pub size_set: Option<super::DesignSizeSetReference>,
}

/// Readonly actual provided-reference and joint-scope validation, extending the exact set facet.
pub trait DesignReferenceHost: DesignSizeSetHost {
    /// Validate complete current sources and every provided reference, including mutual scope.
    /// Unprovided fields stay unprovided; no single-candidate, label or latest-value fallback.
    ///
    /// # Errors
    /// Returns original current/missing/reference mismatch cause without mutating references.
    fn validate_references(
        &self,
        references: DesignSourceReferences,
        canonical: &CanonicalInputs,
    ) -> Result<(), Self::Error>;
}

/// Readonly formula metadata from the actual provided measurement table and Ease set.
///
/// Implementations first validate complete current sources and reference joins, then enumerate
/// only selected table entries followed by selected Ease entries, in authored order. Each
/// declaration borrows its actual token and canonical length, retaining the metadata identity
/// and Measurement/Ease origin. Unprovided selections yield no names; values are never inferred.
pub trait DesignFormulaHost: DesignReferenceHost {
    /// Borrow actual selected source declarations without reading numeric availability.
    ///
    /// # Errors
    /// Returns original current source, selected reference or declaration-binding refusal.
    fn formula_declarations<'a>(
        &'a self,
        references: DesignSourceReferences,
        canonical: &'a CanonicalInputs,
    ) -> Result<Vec<crate::recipe::FormulaDeclaration<'a>>, Self::Error>;
}

/// Checked source-host refusal, preserving original identities, positions and typed host causes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DesignMetadataError<E> {
    /// Two owned metadata objects claim one identity.
    DuplicateIdentity {
        /// Repeated identity.
        identity: EntityId,
        /// First actual position.
        first: DesignHostEntity,
        /// First repeated position.
        duplicate: DesignHostEntity,
    },
    /// Metadata attempts to own a canonical input identity.
    CanonicalIdentityCollision {
        /// Actual conflicting metadata position.
        entity: DesignHostEntity,
    },
    /// A host changed its observable owned census after construction, violating the contract.
    ChangedEntities,
    /// Original host semantic validation/binding cause.
    Host(E),
    /// No measurement metadata with the requested identity exists in this host.
    MissingMeasurement {
        /// Exact requested identity.
        measurement: EntityId,
    },
    /// The requested identity belongs to a different metadata family.
    WrongEntityKind {
        /// Actual owned object.
        entity: DesignHostEntity,
    },
    /// The host returned a detached declaration rather than the supplied canonical object.
    NoncanonicalDeclaration {
        /// Measurement being queried.
        measurement: EntityId,
        /// Returned declaration identity.
        declaration: EntityId,
    },
}
impl<E: fmt::Display> fmt::Display for DesignMetadataError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateIdentity { identity, first, duplicate } => write!(f, "Design metadata identity {identity} repeats at {duplicate:?}, first supplied at {first:?}"),
            Self::CanonicalIdentityCollision { entity } => write!(f, "Design metadata {entity:?} owns a canonical input identity"),
            Self::ChangedEntities => f.write_str("Design metadata host changed its owned entity census"),
            Self::Host(issue) => write!(f, "Design metadata host: {issue}"),
            Self::MissingMeasurement { measurement } => write!(f, "Design metadata has no measurement {measurement}"),
            Self::WrongEntityKind { entity } => write!(f, "Design metadata {entity:?} is not a measurement"),
            Self::NoncanonicalDeclaration { measurement, declaration } => write!(f, "Design measurement {measurement} returned noncanonical declaration {declaration}"),
        }
    }
}
impl<E: std::error::Error + 'static> std::error::Error for DesignMetadataError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let Self::Host(issue) = self {
            Some(issue)
        } else {
            None
        }
    }
}

/// Private immutable metadata owner, checked against explicitly supplied canonical inputs.
///
/// Only the identity census is retained alongside the host; no declaration or scalar state is
/// copied. Model snapshots share this exact immutable host instance without invoking `H::clone`.
/// The captured census is copied separately. Accounting still charges the source contents in each
/// logical model; shared backing does not deduplicate logical demand. This component does not select
/// a Design/table/SizeSet/profile or prove readiness. Later Design composition must also check
/// identities against its other owned components. Cross-thread sharing requires `H: Send + Sync`;
/// single-threaded hosts need neither bound.
///
/// ```
/// fn snapshot<H>(metadata: &sc_core::design::DesignMetadata<H>) -> sc_core::design::DesignMetadata<H> {
///     metadata.clone() // no foreign H: Clone requirement or callback
/// }
/// ```
/// ```compile_fail
/// fn change<H: sc_core::design::DesignMetadataHost>(metadata: &mut sc_core::design::DesignMetadata<H>) {
///     metadata.entities.clear();
/// }
/// ```
/// ```compile_fail
/// fn change<H: sc_core::design::DesignMetadataHost>(metadata: &mut sc_core::design::DesignMetadata<H>) {
///     let _: &mut H = metadata.host();
/// }
/// ```
/// ```compile_fail
/// fn replace<H: sc_core::design::DesignMetadataHost>(
///     metadata: &sc_core::design::DesignMetadata<H>,
///     canonical: &mut sc_core::value::CanonicalInputs,
///     replacement: &mut sc_core::value::CanonicalInputs,
///     measurement: sc_core::ontology::EntityId,
/// ) {
///     if let Ok(declaration) = metadata.measurement_declaration(measurement, canonical) {
///         core::mem::swap(canonical, replacement);
///         let _ = declaration.id();
///     }
/// }
/// ```
#[derive(PartialEq, Eq)]
pub struct DesignMetadata<H> {
    host: Arc<H>,
    entities: Vec<DesignHostEntity>,
}
impl<H> Clone for DesignMetadata<H> {
    fn clone(&self) -> Self {
        let Self { host, entities } = self;
        Self {
            host: Arc::clone(host),
            entities: entities.clone(),
        }
    }
}
impl<H> DesignMetadata<H> {
    // Borrow the retained census; do not regenerate it through the semantic host callback.
    pub(crate) fn resource_parts(&self) -> (&H, &[DesignHostEntity]) {
        let Self { host, entities } = self;
        (host, entities)
    }
}
impl<H> fmt::Debug for DesignMetadata<H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DesignMetadata")
            .field("entity_count", &self.entities.len())
            .finish_non_exhaustive()
    }
}
impl<H: DesignMetadataHost> DesignMetadata<H> {
    /// Own the host after census and actual current semantic checks. Empty explicit hosts are legal.
    ///
    /// # Errors
    /// Returns the first metadata duplicate/canonical collision, then original host validation error.
    pub fn new(
        host: H,
        canonical: &CanonicalInputs,
    ) -> Result<Self, DesignMetadataError<H::Error>> {
        let entities = host.entities();
        let metadata = Self {
            host: Arc::new(host),
            entities,
        };
        metadata.validate_current(canonical)?;
        Ok(metadata)
    }

    /// Original immutable host shared by all snapshots; this port supplies no mutable access.
    /// This is an ordinary borrowed accessor, not a const-evaluation interface.
    #[must_use]
    pub fn host(&self) -> &H {
        self.host.as_ref()
    }

    /// Exact captured metadata identities/positions, excluding referenced canonical inputs.
    #[must_use]
    pub fn entities(&self) -> &[DesignHostEntity] {
        &self.entities
    }

    /// Recheck the stable census and actual host semantics against current canonical declarations.
    ///
    /// # Errors
    /// Returns changed census, first duplicate/canonical collision, then original host refusal.
    pub fn validate_current(
        &self,
        canonical: &CanonicalInputs,
    ) -> Result<(), DesignMetadataError<H::Error>> {
        if self.host.entities() != self.entities {
            return Err(DesignMetadataError::ChangedEntities);
        }
        let mut seen = BTreeMap::new();
        for entity in &self.entities {
            if let Some(first) = seen.insert(entity.id, *entity) {
                return Err(DesignMetadataError::DuplicateIdentity {
                    identity: entity.id,
                    first,
                    duplicate: *entity,
                });
            }
            if canonical.length(entity.id).is_some() || canonical.scalar(entity.id).is_some() {
                return Err(DesignMetadataError::CanonicalIdentityCollision { entity: *entity });
            }
        }
        self.host
            .validate_current(canonical)
            .map_err(DesignMetadataError::Host)
    }

    /// Validate this whole metadata scope, then borrow the measurement's actual canonical length.
    /// The returned borrow keeps both host and supplied canonical context alive.
    ///
    /// # Errors
    /// Returns current scope refusal, absent/wrong-role metadata, original host binding refusal,
    /// or a detached declaration, even when its identity/content equals a canonical record.
    pub fn measurement_declaration<'a>(
        &'a self,
        measurement: EntityId,
        canonical: &'a CanonicalInputs,
    ) -> Result<&'a LengthDeclaration, DesignMetadataError<H::Error>> {
        self.validate_current(canonical)?;
        let entity = self
            .entities
            .iter()
            .find(|entity| entity.id == measurement)
            .ok_or(DesignMetadataError::MissingMeasurement { measurement })?;
        if entity.kind != DesignHostEntityKind::Measurement {
            return Err(DesignMetadataError::WrongEntityKind { entity: *entity });
        }
        let declaration = self
            .host
            .measurement_declaration(measurement, canonical)
            .map_err(DesignMetadataError::Host)?;
        if !canonical
            .length(declaration.id())
            .is_some_and(|actual| core::ptr::eq(actual, declaration))
        {
            return Err(DesignMetadataError::NoncanonicalDeclaration {
                measurement,
                declaration: declaration.id(),
            });
        }
        Ok(declaration)
    }
}
