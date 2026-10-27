//! Joined original Gather ease and closing intent over real higher-source/core contexts.
#![allow(clippy::expect_used, clippy::panic)]
use super::{fixture, id, sewing_owner_definition, with_sewing_graph};
use sc_core::{
    design::{
        DesignGatherDeclarationError as Error, DesignInputs, DesignInputsDefinition,
        DesignParameterDefinition, DesignParameterTarget, DesignParameters, DesignSewing,
        DesignSewingAmountSource, DesignSewingDefinition, DesignStructure,
    },
    name::MachineToken,
    ontology::*,
    recipe::FormulaLengthInput,
    value::ScalarInputKind,
};
use sc_measure::DesignSources;
use sc_units::Length;

fn empty() -> ConstructionInventory {
    ConstructionInventory::new(vec![], vec![], vec![], vec![], vec![], vec![])
        .expect("explicit empty construction scope")
}
fn fixture_gathers(
    mut definition: DesignInputsDefinition<DesignSources>,
    amounts: Vec<EaseAmount>,
    selections: Vec<(usize, SeamSideId)>,
) -> (DesignSewing<DesignSources>, ConstructionInventory) {
    definition.parameters = DesignParameters::new(
        vec![DesignParameterDefinition {
            id: id(5001),
            token: MachineToken::new("gather_length").expect("token"),
            target: DesignParameterTarget::Length(id(12)),
        }],
        definition.structure.inputs(),
    )
    .expect("actual Derived Length metadata");
    let (inputs, graph) = with_sewing_graph(definition, amounts);
    let gathers = selections
        .into_iter()
        .enumerate()
        .map(|(index, (span_index, side))| {
            let span = graph
                .spans()
                .get(span_index)
                .expect("selected original span");
            let copy = inputs
                .structure()
                .cut_plan()
                .copy(match side {
                    SeamSideId::A => span.definition().a.copy,
                    SeamSideId::B => span.definition().b.copy,
                })
                .expect("actual physical copy");
            let piece = inputs
                .structure()
                .pieces()
                .piece(copy.piece())
                .expect("actual Piece");
            Gather::new(
                GatherDefinition {
                    id: id(114100 - u128::try_from(index).expect("authored descending ordinal")),
                    graph: graph.id(),
                    span: span.id(),
                    side,
                    copy: copy.id(),
                    direction: DirectedRange {
                        range: EdgeRange::whole(
                            piece.edges().next().expect("owned direction").edge,
                        ),
                        direction: Direction::Reversed,
                    },
                    closing_operation: id(2000),
                },
                piece,
                inputs.structure().cut_plan(),
                &graph,
                inputs.structure().topology(),
            )
            .expect("actual born Gather")
        })
        .collect();
    let inventory = ConstructionInventory::new(vec![], vec![], vec![], gathers, vec![], vec![])
        .expect("exact Gather order");
    let sewing = DesignSewing::new(DesignSewingDefinition {
        inputs,
        graph,
        landmarks: vec![],
    })
    .expect("actual sewing owner");
    (sewing, inventory)
}

#[test]
fn joined_views_borrow_original_signed_side_span_amount_and_closing_intent_in_authored_order() {
    let (sewing, inventory) = fixture_gathers(
        fixture(),
        vec![
            EaseAmount::Explicit(Length::from_micrometres(5000).expect("positive signed ease")),
            EaseAmount::Explicit(Length::from_micrometres(-3000).expect("negative signed ease")),
            EaseAmount::Formula(id(5001)),
            EaseAmount::Profile(ProfileParameterRef::new(id(10))),
            EaseAmount::Explicit(Length::ZERO),
        ],
        vec![
            (3, SeamSideId::B),
            (0, SeamSideId::A),
            (1, SeamSideId::B),
            (2, SeamSideId::A),
            (4, SeamSideId::A),
            (4, SeamSideId::B),
        ],
    );
    let saved = (sewing.inputs().inputs().clone(), inventory.clone());
    let views = sewing
        .gather_declarations(&inventory)
        .expect("complete current joins");
    assert_eq!(views.len(), 6);
    for (index, view) in views.into_iter().enumerate() {
        let gather = inventory
            .gathers()
            .get(index)
            .expect("original authored object");
        assert!(core::ptr::eq(view.gather(), gather));
        assert_eq!(
            view.position(),
            ConstructionInventoryPosition {
                kind: ConstructionKind::Gather,
                index
            }
        );
        assert!(core::ptr::eq(
            view.closing_operation(),
            &gather.definition().closing_operation
        ));
        assert!(sewing
            .inputs()
            .inputs()
            .scalar(*view.closing_operation())
            .is_some());
        assert!(core::ptr::eq(
            view.canonical_inputs(),
            sewing.inputs().inputs()
        ));
        let intake = view.intake_source();
        let amount = view.amount_declaration();
        let span = sewing
            .graph()
            .spans()
            .iter()
            .find(|span| span.id() == gather.definition().span)
            .expect("same original span");
        assert!(core::ptr::eq(amount.span(), span));
        assert!(core::ptr::eq(intake.declaration, &span.definition().ease));
        assert_eq!(intake.gathered_side, gather.definition().side);
        match amount.source() {
            DesignSewingAmountSource::Explicit(value) => {
                let EaseAmount::Explicit(original) = &intake.declaration.amount else {
                    panic!("explicit ease")
                };
                assert!(core::ptr::eq(value, original));
                if gather.definition().span == id(97001) {
                    assert_eq!(value.as_micrometres(), -3000);
                    assert_eq!(intake.gathered_side, SeamSideId::B);
                }
                assert_eq!(view.profile_validation(), None);
            }
            DesignSewingAmountSource::Formula {
                parameter,
                declaration,
            } => {
                assert!(core::ptr::eq(
                    parameter,
                    sewing
                        .inputs()
                        .parameters()
                        .parameter(id(5001))
                        .expect("actual metadata")
                ));
                let actual =
                    FormulaLengthInput::from_declaration(declaration).expect("Derived Length");
                assert!(core::ptr::eq(
                    actual.canonical_declaration(),
                    sewing
                        .inputs()
                        .inputs()
                        .length(id(12))
                        .expect("canonical Length")
                ));
                assert!(actual.read_authored().is_err());
                assert_eq!(view.profile_validation(), None);
            }
            DesignSewingAmountSource::Profile(reference) => {
                assert_eq!(reference, ProfileParameterRef::new(id(10)));
                assert_eq!(
                    view.profile_validation(),
                    Some(ProfileBindingValidation::DeferredToG4)
                );
            }
        }
        let debug = format!("{view:?}");
        assert!(
            !debug.contains("EntityId") && !debug.contains("Length") && !debug.contains("5000")
        );
    }
    assert_eq!((sewing.inputs().inputs(), &inventory), (&saved.0, &saved.1));
}

#[test]
fn later_gather_cannot_inherit_a_retargeted_same_piece_copy_after_earlier_success() {
    let (sewing, inventory) = fixture_gathers(
        fixture(),
        vec![EaseAmount::Explicit(Length::ZERO)],
        vec![(0, SeamSideId::B), (0, SeamSideId::A)],
    );
    let mut definition = sewing.definition().clone();
    let mut inputs = definition.inputs.definition().clone();
    let mut structure = inputs.structure.definition().clone();
    let mut copies = structure
        .cut_plan
        .copies()
        .iter()
        .map(|copy| *copy.definition())
        .collect::<Vec<_>>();
    copies.first_mut().expect("original A copy").id = id(96010);
    structure.cut_plan =
        CutPlan::new(copies, structure.pieces.pieces()).expect("valid renamed copy");
    inputs.structure = DesignStructure::new(structure).expect("actual replacement structure");
    definition.inputs = DesignInputs::new(inputs).expect("actual root");
    let root = definition.inputs.structure();
    let mut spans = definition
        .graph
        .spans()
        .iter()
        .map(|span| span.definition().clone())
        .collect::<Vec<_>>();
    spans.first_mut().expect("actual span").a.copy = id(96010);
    definition.graph = SewingGraph::new(
        SewingGraphDefinition {
            id: definition.graph.id(),
            spans,
        },
        root.cut_plan(),
        root.pieces().pieces(),
        &definition.landmarks,
        root.topology(),
    )
    .expect("valid current graph");
    let current = DesignSewing::new(definition).expect("current sewing ownership");
    assert!(current.validate_current().is_ok());
    let root = current.inputs().structure();
    let first = inventory.gathers().first().expect("earlier B Gather");
    assert!(first
        .validate_current(
            root.pieces().piece(first.piece()).expect("B Piece"),
            root.cut_plan(),
            current.graph(),
            root.topology()
        )
        .is_ok());
    let original = inventory
        .validate_current(
            root.pieces().pieces(),
            root.cut_plan(),
            current.graph(),
            current.landmarks(),
            root.topology(),
        )
        .expect_err("original A copy binding");
    assert!(
        matches!(&original,ConstructionInventoryError::Gather {position,issue,..} if position.index==1 && matches!(issue.as_ref(),GatherError::CopyBindingChanged {..}))
    );
    let error = current
        .gather_declarations(&inventory)
        .expect_err("no partial Gather prefix");
    assert_eq!(error, Error::Constructions(Box::new(original.clone())));
    assert_eq!(
        std::error::Error::source(&error)
            .expect("original collection cause")
            .downcast_ref::<ConstructionInventoryError>(),
        Some(&original)
    );
    assert!(std::error::Error::source(&error)
        .expect("collection")
        .source()
        .expect("original Gather")
        .is::<GatherError>());
}

#[test]
fn unused_landmark_and_amount_failures_block_even_empty_gather_declarations_with_original_causes() {
    let mut definition = sewing_owner_definition();
    let mut inputs = definition.inputs.definition().clone();
    let mut structure = inputs.structure.definition().clone();
    structure.pieces = PieceInventory::new(vec![]).expect("removed owned Pieces");
    structure.cut_plan = CutPlan::new(vec![], &[]).expect("empty plan");
    inputs.structure = DesignStructure::new(structure).expect("held historical data");
    definition.inputs = DesignInputs::new(inputs).expect("valid input scope");
    let current = DesignSewing::new(definition).expect("held landmark/graph remain inspectable");
    let original = current
        .validate_current()
        .expect_err("unused landmark current source");
    assert!(matches!(
        original,
        sc_core::design::DesignSewingError::MissingLandmarkPiece { .. }
    ));
    assert_eq!(
        current
            .gather_declarations(&empty())
            .expect_err("even empty scope"),
        Error::Sewing(Box::new(original))
    );
    let mut definition = sewing_owner_definition();
    let mut inputs = definition.inputs.definition().clone();
    inputs.parameters = DesignParameters::new(
        vec![DesignParameterDefinition {
            id: id(5001),
            token: MachineToken::new("gather_count").expect("token"),
            target: DesignParameterTarget::Scalar {
                declaration: id(2000),
                kind: ScalarInputKind::Count,
            },
        }],
        inputs.structure.inputs(),
    )
    .expect("actual current Count metadata");
    definition.inputs = DesignInputs::new(inputs).expect("valid typed root");
    let current = DesignSewing::new(definition).expect("inspectable current owner");
    let original = current.validate_current().expect_err("amount needs Length");
    assert!(matches!(
        original,
        sc_core::design::DesignSewingError::Amount(_)
    ));
    let error = current
        .gather_declarations(&empty())
        .expect_err("whole amount context before empty");
    assert_eq!(error, Error::Sewing(Box::new(original.clone())));
    assert_eq!(
        std::error::Error::source(&error)
            .expect("original sewing")
            .downcast_ref::<sc_core::design::DesignSewingError<sc_measure::DesignSourcesError>>(),
        Some(&original)
    );
}

#[test]
fn an_unused_damaged_construction_blocks_empty_gather_scope_after_healthy_sewing_preflight() {
    let (sewing, _) = fixture_gathers(fixture(), vec![], vec![]);
    let mut definition = sewing.definition().clone();
    let mut inputs = definition.inputs.definition().clone();
    let mut structure = inputs.structure.definition().clone();
    let mut ids = DeterministicIdGenerator::from_start(115000);
    let edge = *structure
        .topology
        .declare_edges(&mut ids, 1)
        .expect("actual private fold source")
        .first()
        .expect("edge");
    let mut pieces = structure.pieces.pieces().to_vec();
    let mut piece = pieces.first().expect("original Piece").definition().clone();
    piece.construction_lines.push(DirectedEdge {
        edge,
        direction: Direction::Original,
    });
    *pieces.first_mut().expect("original position") =
        Piece::new(piece, &structure.topology).expect("owned current fold source");
    let direction = DirectedRange {
        range: EdgeRange::whole(edge),
        direction: Direction::Original,
    };
    let tuck = Tuck::new(
        FoldDefinition {
            id: id(115100),
            intake: IntakeAmount::Explicit {
                parameter: id(115101),
                value: Length::ZERO,
            },
            fold_lines: vec![direction],
            direction,
            closing_operation: id(2000),
        },
        pieces.first().expect("Piece"),
        &structure.topology,
    )
    .expect("born Tuck");
    let inventory = ConstructionInventory::new(vec![], vec![tuck], vec![], vec![], vec![], vec![])
        .expect("unused Tuck owned");
    structure.pieces = PieceInventory::new(pieces).expect("updated source inventory");
    structure
        .topology
        .delete(&mut ids, edge)
        .expect("later fold source loss");
    inputs.structure = DesignStructure::new(structure).expect("held repair retained");
    definition.inputs = DesignInputs::new(inputs).expect("current whole root");
    let current = DesignSewing::new(definition).expect("healthy empty sewing graph");
    assert!(current.validate_current().is_ok());
    let root = current.inputs().structure();
    let original = inventory
        .validate_current(
            root.pieces().pieces(),
            root.cut_plan(),
            current.graph(),
            current.landmarks(),
            root.topology(),
        )
        .expect_err("unused actual Tuck");
    assert!(matches!(original, ConstructionInventoryError::Tuck { .. }));
    assert_eq!(
        current
            .gather_declarations(&inventory)
            .expect_err("no empty shortcut"),
        Error::Constructions(Box::new(original))
    );
}
