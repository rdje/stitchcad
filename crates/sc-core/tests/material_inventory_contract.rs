//! Actual Material ownership and Piece targets remain distinct from numeric/physical readiness.
#![allow(clippy::expect_used)]
use sc_core::{
    ontology::{
        CuttingSide, DeterministicIdGenerator, DirectedEdge, Direction, EntityId, IdentityLedger,
        LabelText, Material, MaterialAssignment, MaterialDefinition, MaterialError,
        MaterialInventory as Inventory, MaterialInventoryError as Error, MaterialProperty,
        Mirroring, Piece, PieceDefinition, PieceInventory, PieceMaterialView as View,
    },
    value::{
        CanonicalInputTarget as Target, CanonicalInputs, LengthDeclaration,
        LengthDeclarationDefinition, LengthState, ScalarInputDeclaration,
        ScalarInputDeclarationDefinition, ScalarInputKind as Kind, ScalarInputState as State,
    },
};
use sc_units::Length;

fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn scalar(identity: u128, kind: Kind, state: State) -> ScalarInputDeclaration {
    ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
        id: id(identity),
        source: id(500),
        kind,
        state,
    })
    .expect("explicit fixture scalar")
}
fn canonical(second_width: State) -> CanonicalInputs {
    let lengths = vec![LengthDeclaration::new(LengthDeclarationDefinition {
        id: id(1),
        source: id(501),
        state: LengthState::Unknown {
            observation: id(502),
        },
    })
    .expect("explicit unknown width")];
    let mut scalars = (2..=5)
        .map(|identity| {
            scalar(
                identity,
                Kind::Boolean,
                State::Unknown {
                    observation: id(503),
                },
            )
        })
        .collect::<Vec<_>>();
    scalars.push(scalar(8, Kind::Length, second_width));
    CanonicalInputs::new(lengths, scalars).expect("unique canonical fixture inputs")
}
fn unknown() -> CanonicalInputs {
    canonical(State::Unknown {
        observation: id(504),
    })
}
fn materials(canonical: &CanonicalInputs) -> Vec<Material> {
    [
        (
            200,
            Target::Scalar {
                declaration: id(8),
                kind: Kind::Length,
            },
        ),
        (100, Target::Length(id(1))),
    ]
    .into_iter()
    .map(|(identity, width)| {
        let flag = |identity| Target::Scalar {
            declaration: id(identity),
            kind: Kind::Boolean,
        };
        Material::new(
            MaterialDefinition {
                id: id(identity),
                source: id(505),
                name: "identical private fabric label".into(),
                composition: None,
                width,
                nap: flag(2),
                directional_print: flag(3),
                striped: flag(4),
                checked: flag(5),
                shrinkage: None,
            },
            canonical,
        )
        .expect("valid draft Material")
    })
    .collect()
}
fn pieces(rows: &[(u128, MaterialAssignment)]) -> PieceInventory {
    let mut ledger = IdentityLedger::new();
    let mut ids = DeterministicIdGenerator::from_start(1_000);
    let pieces = rows
        .iter()
        .map(|(identity, assignment)| {
            let edge = ledger
                .declare_edges(&mut ids, 1)
                .expect("fixture edge declaration")
                .into_iter()
                .next()
                .expect("one requested edge");
            Piece::new(
                PieceDefinition {
                    id: id(*identity),
                    boundary: vec![DirectedEdge {
                        edge,
                        direction: Direction::Original,
                    }],
                    holes: vec![],
                    construction_lines: vec![],
                    quantity: 1,
                    mirroring: Mirroring::Single,
                    cut_on_fold: false,
                    fold_edges: vec![],
                    cutting_side: CuttingSide::Face,
                    material: assignment.clone(),
                    layer_index: 0,
                    label: LabelText {
                        name: "private_piece_label".into(),
                        size: "fixture".into(),
                        fabric: "identical private fabric label".into(),
                        colorway: "undetermined".into(),
                    },
                },
                &ledger,
            )
            .expect("valid local fixture Piece")
        })
        .collect();
    PieceInventory::new(pieces).expect("pair-consistent fixture inventory")
}
fn missing_second_width(canonical: &CanonicalInputs) -> CanonicalInputs {
    CanonicalInputs::new(
        canonical.lengths().to_vec(),
        canonical
            .scalars()
            .iter()
            .filter(|value| value.id() != id(8))
            .cloned()
            .collect(),
    )
    .expect("current missing second width")
}

#[test]
fn authored_order_exact_objects_and_unresolved_reasons_survive_with_canonical_borrows() {
    let canonical = unknown();
    let supplied = materials(&canonical);
    let original = supplied.clone();
    let inventory = Inventory::new(supplied, &canonical).unwrap();
    assert_eq!(inventory.materials(), original);
    assert!(core::ptr::eq(
        inventory.material(id(100)).unwrap(),
        inventory.materials().last().unwrap()
    ));
    assert!(inventory.material(id(999)).is_none());
    let pieces = pieces(&[
        (12, MaterialAssignment::Assigned(id(100))),
        (
            11,
            MaterialAssignment::Unresolved("  private_reason  ".into()),
        ),
        (10, MaterialAssignment::Assigned(id(200))),
    ]);
    let views = inventory.assignments(&pieces, &canonical).unwrap();
    assert_eq!(
        views,
        vec![
            View::Assigned {
                piece: pieces.piece(id(12)).unwrap(),
                material: inventory.material(id(100)).unwrap()
            },
            View::Unresolved {
                piece: pieces.piece(id(11)).unwrap(),
                reason: "  private_reason  "
            },
            View::Assigned {
                piece: pieces.piece(id(10)).unwrap(),
                material: inventory.material(id(200)).unwrap()
            }
        ]
    );
    for view in &views {
        match view {
            View::Assigned { piece, material } => {
                assert!(core::ptr::eq(*piece, pieces.piece(piece.id()).unwrap()));
                assert!(core::ptr::eq(
                    *material,
                    inventory.material(material.id()).unwrap()
                ));
            }
            View::Unresolved { piece, reason } => {
                assert!(core::ptr::eq(*piece, pieces.piece(piece.id()).unwrap()));
                if let MaterialAssignment::Unresolved(original) = &piece.definition().material {
                    assert_eq!(reason.as_ptr(), original.as_ptr());
                }
            }
        }
    }
}

#[test]
fn identical_duplicate_ids_report_original_positions_before_current_property_failures() {
    let canonical = unknown();
    let mut materials = materials(&canonical);
    materials.push(materials.first().unwrap().clone());
    let current = missing_second_width(&canonical);
    assert_eq!(
        Inventory::new(materials, &current),
        Err(Error::DuplicateMaterial {
            material: id(200),
            first: 0,
            duplicate: 2
        })
    );
}

#[test]
fn absent_assigned_ids_never_transfer_to_a_same_label_peer() {
    let canonical = unknown();
    let inventory = Inventory::new(materials(&canonical), &canonical).unwrap();
    let pieces = pieces(&[(10, MaterialAssignment::Assigned(id(999)))]);
    assert_eq!(
        pieces.piece(id(10)).unwrap().definition().label.fabric,
        inventory.material(id(100)).unwrap().definition().name
    );
    let expected = Error::MissingMaterial {
        piece: id(10),
        material: id(999),
    };
    assert_eq!(
        inventory.assignment(pieces.piece(id(10)).unwrap(), &canonical),
        Err(expected.clone())
    );
    assert_eq!(inventory.assignments(&pieces, &canonical), Err(expected));
    assert_eq!(
        pieces.piece(id(10)).unwrap().definition().material,
        MaterialAssignment::Assigned(id(999))
    );
}

#[test]
fn whole_queries_check_piece_identity_collisions_before_missing_targets() {
    let canonical = unknown();
    let inventory = Inventory::new(materials(&canonical), &canonical).unwrap();
    let pieces = pieces(&[
        (10, MaterialAssignment::Assigned(id(999))),
        (
            100,
            MaterialAssignment::Unresolved("awaiting selection".into()),
        ),
    ]);
    assert_eq!(
        inventory.assignments(&pieces, &canonical),
        Err(Error::PieceIdentityCollision { identity: id(100) })
    );
    assert_eq!(
        inventory.assignment(pieces.piece(id(10)).unwrap(), &canonical),
        Err(Error::MissingMaterial {
            piece: id(10),
            material: id(999)
        })
    );
}

#[test]
fn current_property_or_canonical_identity_failures_keep_the_actual_material_and_original_cause() {
    let canonical = unknown();
    let inventory = Inventory::new(materials(&canonical), &canonical).unwrap();
    let current = self::canonical(State::Preference {
        value: Length::from_micrometres(0).unwrap().into(),
        provenance: id(506),
    });
    let error = inventory.validate_current(&current).unwrap_err();
    assert_eq!(
        error,
        Error::InvalidMaterial {
            material: id(200),
            issue: Box::new(MaterialError::NonPositiveWidth {
                material: id(200),
                target: Target::Scalar {
                    declaration: id(8),
                    kind: Kind::Length
                },
                actual_micrometres: 0
            })
        }
    );
    assert!(std::error::Error::source(&error).is_some());
    let mut scalars = canonical.scalars().to_vec();
    scalars.push(scalar(
        100,
        Kind::Boolean,
        State::Unknown {
            observation: id(507),
        },
    ));
    let current = CanonicalInputs::new(canonical.lengths().to_vec(), scalars).unwrap();
    assert_eq!(
        inventory.validate_current(&current),
        Err(Error::InvalidMaterial {
            material: id(100),
            issue: Box::new(MaterialError::IdentityCollision { material: id(100) })
        })
    );
}

#[test]
fn targeted_inspection_does_not_claim_complete_inventory_validation() {
    let canonical = unknown();
    let inventory = Inventory::new(materials(&canonical), &canonical).unwrap();
    let current = missing_second_width(&canonical);
    let pieces = pieces(&[
        (10, MaterialAssignment::Assigned(id(100))),
        (
            11,
            MaterialAssignment::Unresolved("awaiting selection".into()),
        ),
    ]);
    assert_eq!(
        inventory
            .assignment(pieces.piece(id(10)).unwrap(), &current)
            .unwrap(),
        View::Assigned {
            piece: pieces.piece(id(10)).unwrap(),
            material: inventory.material(id(100)).unwrap()
        }
    );
    assert!(matches!(
        inventory.assignment(pieces.piece(id(11)).unwrap(), &current),
        Ok(View::Unresolved { .. })
    ));
    assert_eq!(
        inventory.assignments(&pieces, &current),
        Err(Error::InvalidMaterial {
            material: id(200),
            issue: Box::new(MaterialError::MissingDeclaration {
                material: id(200),
                property: MaterialProperty::Width,
                target: Target::Scalar {
                    declaration: id(8),
                    kind: Kind::Length
                }
            })
        })
    );
}

#[test]
fn empty_drafts_mint_no_material_and_unresolved_pieces_remain_explicit() {
    let canonical = CanonicalInputs::new(vec![], vec![]).unwrap();
    let inventory = Inventory::new(vec![], &canonical).unwrap();
    assert!(inventory.materials().is_empty());
    assert!(inventory
        .assignments(&PieceInventory::new(vec![]).unwrap(), &canonical)
        .unwrap()
        .is_empty());
    let pieces = pieces(&[(
        10,
        MaterialAssignment::Unresolved("no chosen fabric".into()),
    )]);
    assert_eq!(
        inventory.assignments(&pieces, &canonical).unwrap(),
        vec![View::Unresolved {
            piece: pieces.piece(id(10)).unwrap(),
            reason: "no chosen fabric"
        }]
    );
    assert!(inventory.materials().is_empty());
}

#[test]
fn detached_replacements_preserve_original_objects_and_default_debug_omits_customer_content() {
    let canonical = unknown();
    let inventory = Inventory::new(materials(&canonical), &canonical).unwrap();
    let original = inventory.materials().to_vec();
    let mut detached = inventory.material(id(100)).unwrap().definition().clone();
    detached.name = "explicit replacement".into();
    assert_ne!(
        Material::new(detached, &canonical).unwrap(),
        *inventory.material(id(100)).unwrap()
    );
    assert_eq!(inventory.materials(), original);
    let pieces = pieces(&[
        (10, MaterialAssignment::Assigned(id(100))),
        (11, MaterialAssignment::Unresolved("private_reason".into())),
    ]);
    let views = inventory.assignments(&pieces, &canonical).unwrap();
    for debug in [format!("{inventory:?}"), format!("{views:?}")] {
        assert!(!debug.contains("private fabric"));
        assert!(!debug.contains("private_piece_label"));
        assert!(!debug.contains("private_reason"));
    }
}
