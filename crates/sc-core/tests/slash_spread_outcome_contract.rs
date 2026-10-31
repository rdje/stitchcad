//! Exact source-preserving intent is distinct from reconciled contours and actual geometry proof.
#![allow(clippy::expect_used)]
use sc_core::{construction::*, ontology::*};
#[path = "support/slash_sources.rs"]
mod sources;
use sources::*;
#[path = "support/slash_zero.rs"]
mod zero_context;
use zero_context::with_zero;
#[path = "support/slash_metadata.rs"]
mod metadata_context;
use metadata_context::with_boundary;
use SlashSpreadOutcomeIntent::{PreserveSource, ReconcileContours};

fn original(f: &Fixture) -> SlashSpreadContours {
    let source = f.piece.definition();
    SlashSpreadContours::new(
        source
            .boundary
            .iter()
            .copied()
            .map(SlashSpreadContourEdge::Retained)
            .collect(),
        source
            .holes
            .iter()
            .map(|hole| {
                hole.iter()
                    .copied()
                    .map(SlashSpreadContourEdge::Retained)
                    .collect()
            })
            .collect(),
        source
            .construction_lines
            .iter()
            .copied()
            .map(SlashSpreadContourEdge::Retained)
            .collect(),
    )
}
fn assert_refusal(effect: &RequiredSlashSpreadEffect<'_>, reason: SlashSpreadOutcomeRefusal) {
    let error = effect
        .require_outcome(PreserveSource)
        .expect_err("source preservation contradiction");
    assert_eq!(error.refusal(), reason);
    assert_eq!(error.intent(), &PreserveSource);
    assert!(std::ptr::eq(error.effect(), effect));
    assert_eq!(
        effect
            .require_outcome(ReconcileContours)
            .expect("actual complete reconciliation requirement")
            .intent(),
        &ReconcileContours
    );
}

#[test]
fn explicit_preservation_retains_actual_complete_context_without_inferred_mode_or_execution() {
    let f = fixture();
    let source = f.piece.clone();
    let journal = f.ledger.journal().len();
    with_zero(
        &f,
        &original(&f),
        SlashSpreadFullnessIntent::Unchanged,
        |effect| {
            let preserve = effect
                .require_outcome(PreserveSource)
                .expect("exact original structural declaration");
            assert!(std::ptr::eq(preserve.effect(), effect));
            assert_eq!(preserve.intent(), &PreserveSource);
            assert_eq!(preserve.new_edge_count(), 0);
            assert!(std::ptr::eq(
                preserve.effect().copies().metadata().source_piece(),
                &f.piece
            ));
            let reconcile = effect
                .require_outcome(ReconcileContours)
                .expect("caller can explicitly request reconciliation too");
            assert_eq!(reconcile.intent(), &ReconcileContours);
            assert_eq!(reconcile.new_edge_count(), 0);
            assert!(std::ptr::eq(reconcile.effect(), effect));
            assert_eq!(
                preserve
                    .effect()
                    .copies()
                    .metadata()
                    .boundary()
                    .consumers()
                    .requirements()
                    .len(),
                5
            );
        },
    );
    assert_eq!(f.piece, source);
    assert_eq!(f.ledger.journal().len(), journal);
}

#[test]
fn preservation_refuses_new_candidates_then_declared_fullness_before_comparing_source_contours() {
    let f = fixture();
    with_boundary(&f, &contours(&f), &manifest(&f), |boundary| {
        let metadata_request = SlashSpreadPieceMetadataRequirement::preserve_source(
            SlashSpreadFoldIntent::NoFold,
            id(930),
            id(931),
        );
        let metadata = boundary
            .preserve_piece_metadata(&metadata_request)
            .expect("metadata");
        let plan = CutPlan::new(
            vec![CutCopyDefinition {
                id: id(410),
                piece: f.piece.id(),
                orientation: CopyOrientation::Authored,
            }],
            core::slice::from_ref(&f.piece),
        )
        .expect("actual copies");
        let policy = SlashSpreadCopyLimits::new(1);
        let copies = metadata
            .preserve_physical_copies(&plan, &policy)
            .expect("copy population");
        let request = SlashSpreadEffectRequirement::new(id(940), id(941), id(942));
        let effect = copies.require_attributed_effect(&request);
        assert_refusal(&effect, SlashSpreadOutcomeRefusal::NewEdges);
        assert_eq!(
            effect
                .require_outcome(ReconcileContours)
                .expect("candidate-bearing declaration")
                .new_edge_count(),
            2
        );
    });
    for fullness in [
        SlashSpreadFullnessIntent::Added,
        SlashSpreadFullnessIntent::Removed,
    ] {
        with_zero(&f, &original(&f), fullness, |effect| {
            assert_refusal(effect, SlashSpreadOutcomeRefusal::Fullness)
        });
    }
}

#[test]
fn identity_membership_cannot_substitute_for_exact_boundary_and_construction_directions_or_occurrences(
) {
    let f = fixture();
    let base = original(&f);
    let reversed_boundary = SlashSpreadContours::new(
        vec![SlashSpreadContourEdge::Retained(DirectedEdge {
            direction: Direction::Reversed,
            ..f.first
        })],
        vec![],
        base.construction_lines().to_vec(),
    );
    with_zero(
        &f,
        &reversed_boundary,
        SlashSpreadFullnessIntent::Unchanged,
        |effect| assert_refusal(effect, SlashSpreadOutcomeRefusal::Boundary),
    );
    let omitted = SlashSpreadContours::new(base.boundary().to_vec(), vec![], vec![]);
    with_zero(
        &f,
        &omitted,
        SlashSpreadFullnessIntent::Unchanged,
        |effect| assert_refusal(effect, SlashSpreadOutcomeRefusal::Construction),
    );
    let reversed = SlashSpreadContours::new(
        base.boundary().to_vec(),
        vec![],
        vec![SlashSpreadContourEdge::Retained(DirectedEdge {
            direction: Direction::Reversed,
            ..f.second
        })],
    );
    with_zero(
        &f,
        &reversed,
        SlashSpreadFullnessIntent::Unchanged,
        |effect| assert_refusal(effect, SlashSpreadOutcomeRefusal::Construction),
    );
    let repeated = SlashSpreadContours::new(
        base.boundary().to_vec(),
        vec![],
        vec![
            SlashSpreadContourEdge::Retained(f.second),
            SlashSpreadContourEdge::Retained(f.second),
        ],
    );
    with_zero(
        &f,
        &repeated,
        SlashSpreadFullnessIntent::Unchanged,
        |effect| assert_refusal(effect, SlashSpreadOutcomeRefusal::Construction),
    );
}

#[test]
fn real_hole_groups_keep_their_original_population_order_and_traversal() {
    let mut f = fixture();
    let edges = f
        .ledger
        .declare_tagged_edges(id(20), &[LocalTag::new(3), LocalTag::new(4)])
        .expect("actual original hole identities");
    let a = DirectedEdge {
        edge: *edges.first().expect("hole A"),
        direction: Direction::Original,
    };
    let b = DirectedEdge {
        edge: *edges.get(1).expect("hole B"),
        direction: Direction::Original,
    };
    let mut definition = f.piece.definition().clone();
    definition.holes = vec![vec![a], vec![b]];
    f.piece = Piece::new(definition, &f.ledger)
        .expect("actual structural source holes,geometry deferred");
    let base = original(&f);
    with_zero(&f, &base, SlashSpreadFullnessIntent::Unchanged, |effect| {
        assert_eq!(
            effect
                .require_outcome(PreserveSource)
                .expect("exact two original groups")
                .new_edge_count(),
            0
        );
    });
    let fewer = SlashSpreadContours::new(
        base.boundary().to_vec(),
        vec![vec![SlashSpreadContourEdge::Retained(a)]],
        base.construction_lines().to_vec(),
    );
    with_zero(&f, &fewer, SlashSpreadFullnessIntent::Unchanged, |effect| {
        assert_refusal(effect, SlashSpreadOutcomeRefusal::HoleGroups)
    });
    let swapped = SlashSpreadContours::new(
        base.boundary().to_vec(),
        vec![
            vec![SlashSpreadContourEdge::Retained(b)],
            vec![SlashSpreadContourEdge::Retained(a)],
        ],
        base.construction_lines().to_vec(),
    );
    with_zero(
        &f,
        &swapped,
        SlashSpreadFullnessIntent::Unchanged,
        |effect| assert_refusal(effect, SlashSpreadOutcomeRefusal::Hole { hole: 0 }),
    );
    let reversed = SlashSpreadContours::new(
        base.boundary().to_vec(),
        vec![
            vec![SlashSpreadContourEdge::Retained(DirectedEdge {
                direction: Direction::Reversed,
                ..a
            })],
            vec![SlashSpreadContourEdge::Retained(b)],
        ],
        base.construction_lines().to_vec(),
    );
    with_zero(
        &f,
        &reversed,
        SlashSpreadFullnessIntent::Unchanged,
        |effect| assert_refusal(effect, SlashSpreadOutcomeRefusal::Hole { hole: 0 }),
    );
}
