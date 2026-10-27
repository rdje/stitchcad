//! Public core-host guards; fixture hosts exercise observable integration-contract violations.
#![allow(clippy::expect_used)]
use core::{cell::Cell, fmt};
use sc_core::{
    design::{
        DesignHostEntity as Entity, DesignHostEntityKind as Kind, DesignMetadata,
        DesignMetadataError as Error, DesignMetadataHost,
    },
    ontology::EntityId,
    value::{
        CanonicalInputs, LengthDeclaration, LengthDeclarationDefinition, LengthState,
        ScalarInputDeclaration, ScalarInputDeclarationDefinition, ScalarInputKind,
        ScalarInputState,
    },
};
use std::error::Error as _;

fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn unknown() -> LengthDeclaration {
    LengthDeclaration::new(LengthDeclarationDefinition {
        id: id(1),
        source: id(90),
        state: LengthState::Unknown {
            observation: id(91),
        },
    })
    .expect("explicit unknown")
}
fn canonical() -> CanonicalInputs {
    CanonicalInputs::new(
        vec![unknown()],
        vec![
            ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
                id: id(2),
                source: id(92),
                kind: ScalarInputKind::Boolean,
                state: ScalarInputState::Unknown {
                    observation: id(93),
                },
            })
            .expect("explicit boolean"),
        ],
    )
    .expect("disjoint declaration identities")
}
fn entity(bits: u128, kind: Kind, index: usize) -> Entity {
    Entity {
        id: id(bits),
        kind,
        index,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HostError {
    Refused,
    MissingBinding,
}
impl fmt::Display for HostError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for HostError {}
struct Host {
    entities: Vec<Entity>,
    target: EntityId,
    detached: Option<LengthDeclaration>,
    refuse: bool,
    changed: Cell<bool>,
}
fn host(entities: Vec<Entity>) -> Host {
    Host {
        entities,
        target: id(1),
        detached: None,
        refuse: false,
        changed: Cell::new(false),
    }
}
impl DesignMetadataHost for Host {
    type Error = HostError;
    fn entities(&self) -> Vec<Entity> {
        let mut entries = self.entities.clone();
        if self.changed.get() {
            entries.push(entity(99, Kind::Ease, 0));
        }
        entries
    }
    fn validate_current(&self, canonical: &CanonicalInputs) -> Result<(), HostError> {
        if self.refuse {
            return Err(HostError::Refused);
        }
        if !self.entities.is_empty() && canonical.length(self.target).is_none() {
            return Err(HostError::MissingBinding);
        }
        Ok(())
    }
    fn measurement_declaration<'a>(
        &'a self,
        _: EntityId,
        canonical: &'a CanonicalInputs,
    ) -> Result<&'a LengthDeclaration, HostError> {
        self.detached
            .as_ref()
            .or_else(|| canonical.length(self.target))
            .ok_or(HostError::MissingBinding)
    }
}

#[test]
fn actual_unknown_is_borrowed_without_copying_state_or_evaluating_it() {
    let canonical = canonical();
    let metadata = DesignMetadata::new(host(vec![entity(10, Kind::Measurement, 3)]), &canonical)
        .expect("actual disjoint measurement host");
    let declaration = metadata
        .measurement_declaration(id(10), &canonical)
        .expect("explicit target");
    assert!(core::ptr::eq(
        declaration,
        canonical.length(id(1)).expect("actual canonical object")
    ));
    assert_eq!(
        declaration.state(),
        &LengthState::Unknown {
            observation: id(91)
        }
    );
    assert_eq!(metadata.entities(), &[entity(10, Kind::Measurement, 3)]);
    assert_eq!(canonical.lengths(), &[unknown()]);
    let debug = format!("{metadata:?}");
    assert!(debug.contains("entity_count: 1"));
    assert!(!debug.contains("Unknown"));
}

#[test]
fn empty_explicit_host_does_not_infer_measurement_from_canonical_identity() {
    let canonical = canonical();
    let metadata = DesignMetadata::new(host(vec![]), &canonical).expect("empty authored host");
    assert_eq!(
        metadata.measurement_declaration(id(1), &canonical),
        Err(Error::MissingMeasurement { measurement: id(1) })
    );
    assert!(metadata.entities().is_empty());
}

#[test]
fn duplicate_witnesses_keep_actual_family_and_authored_positions() {
    let first = entity(10, Kind::Measurement, 4);
    let duplicate = entity(10, Kind::EaseSet, 2);
    let mut fixture = host(vec![first, duplicate]);
    fixture.refuse = true;
    assert!(matches!(DesignMetadata::new(fixture, &canonical()),
        Err(Error::DuplicateIdentity { identity, first: a, duplicate: b })
        if identity == id(10) && a == first && b == duplicate));
}

#[test]
fn neither_canonical_representation_family_can_be_owned_as_metadata() {
    for bits in [1, 2] {
        let entity = entity(bits, Kind::MeasurementTable, 7);
        assert!(
            matches!(DesignMetadata::new(host(vec![entity]), &canonical()),
            Err(Error::CanonicalIdentityCollision { entity: actual }) if actual == entity)
        );
    }
}

#[test]
fn wrong_metadata_role_cannot_be_read_as_a_measurement() {
    let canonical = canonical();
    let entity = entity(20, Kind::Ease, 1);
    let metadata = DesignMetadata::new(host(vec![entity]), &canonical).expect("valid other role");
    assert_eq!(
        metadata.measurement_declaration(id(20), &canonical),
        Err(Error::WrongEntityKind { entity })
    );
}

#[test]
fn equal_detached_declaration_is_rejected_despite_matching_id_and_state() {
    let canonical = canonical();
    let mut fixture = host(vec![entity(10, Kind::Measurement, 0)]);
    fixture.detached = Some(unknown());
    assert_eq!(fixture.detached.as_ref(), canonical.length(id(1)));
    let metadata = DesignMetadata::new(fixture, &canonical).expect("valid original host scope");
    assert_eq!(
        metadata.measurement_declaration(id(10), &canonical),
        Err(Error::NoncanonicalDeclaration {
            measurement: id(10),
            declaration: id(1)
        })
    );
}

#[test]
fn changed_context_revalidates_host_and_preserves_original_typed_cause() {
    let canonical = canonical();
    let metadata = DesignMetadata::new(host(vec![entity(10, Kind::Measurement, 0)]), &canonical)
        .expect("valid original binding");
    let replacement = CanonicalInputs::new(vec![], vec![]).expect("empty replacement");
    let error = metadata
        .measurement_declaration(id(10), &replacement)
        .expect_err("stale binding");
    assert_eq!(error, Error::Host(HostError::MissingBinding));
    assert_eq!(
        error
            .source()
            .and_then(|error| error.downcast_ref::<HostError>()),
        Some(&HostError::MissingBinding)
    );
    assert!(metadata.measurement_declaration(id(10), &canonical).is_ok());
}

#[test]
fn observable_host_contract_violation_is_not_silently_accepted() {
    let canonical = canonical();
    let metadata = DesignMetadata::new(host(vec![entity(10, Kind::Measurement, 0)]), &canonical)
        .expect("initially stable census");
    // Deliberately invalid provider: real implementations must expose immutable snapshots.
    metadata.host().changed.set(true);
    assert_eq!(
        metadata.validate_current(&canonical),
        Err(Error::ChangedEntities)
    );
    assert_eq!(
        metadata.measurement_declaration(id(10), &canonical),
        Err(Error::ChangedEntities)
    );
    assert_eq!(metadata.entities(), &[entity(10, Kind::Measurement, 0)]);
}
