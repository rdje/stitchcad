//! Canonical metadata stays borrowed;fold attribution grants no geometry or new physical-copy ids.
#![allow(clippy::expect_used)]
use sc_core::{construction::*, ontology::*};
#[path = "support/slash_sources.rs"]
mod sources;
use sources::*;
fn with_boundary<T>(
    f: &Fixture,
    raw_contours: &SlashSpreadContours,
    raw_edges: &SlashSpreadEdgeManifest,
    run: impl FnOnce(&RequiredSlashSpreadBoundary<'_>) -> T,
) -> T {
    let intent = intent(f);
    let raw_catalog = catalog();
    let catalog = raw_catalog.admit(catalogue_limits()).expect("catalogue");
    let bound = intent
        .bind_edge_roles(&catalog, catalogue_limits())
        .expect("operation");
    let edges = raw_edges
        .admit(
            &bound,
            SlashSpreadEdgeManifestLimits::new(2, catalogue_limits()),
        )
        .expect("edges");
    let baseline = limits();
    let m = raw_contours.boundary().len()
        + raw_contours.holes().iter().map(Vec::len).sum::<usize>()
        + raw_contours.construction_lines().len();
    let contour_policy = SlashSpreadContourLimits::new(
        baseline.candidates(),
        baseline.source_edges(),
        baseline.source_holes(),
        raw_contours.holes().len(),
        m,
    );
    let contours = raw_contours
        .admit(&edges, &f.piece, &contour_policy)
        .expect("contours");
    let raw_sources = SlashSpreadSourcePlan::new(vec![
        SlashSpreadSourceDisposition::new(f.first.edge, SlashSpreadGlobalSourceIntent::Preserve),
        SlashSpreadSourceDisposition::new(f.second.edge, SlashSpreadGlobalSourceIntent::Preserve),
    ]);
    let source_policy = SlashSpreadSourcePlanLimits::new(2, 2, m, 2);
    let sources = raw_sources
        .bind(&contours, &source_policy)
        .expect("source identities");
    let raw_registrations = SlashSpreadRegistrationPlan::new(vec![]);
    let registration_policy = SlashSpreadRegistrationPlanLimits::new(2, 0, 0);
    let registrations = raw_registrations
        .bind(&sources, &f.ledger, &registration_policy)
        .expect("empty actual registered subset");
    let evidence =
        SlashSpreadConsumerEvidenceSources::new(id(800), id(801), id(802), id(803), id(804));
    let consumers = registrations.require_consumer_evidence(&evidence);
    let request = SlashSpreadBoundaryRequirement::new(
        id(920),
        id(921),
        SlashSpreadBoundaryEvidenceRequest::StraightBridges {
            incidence: id(910),
            closure: id(911),
        },
    );
    let boundary = consumers
        .require_boundary(&request)
        .expect("bridge request");
    run(&boundary)
}
fn requirement(fold: SlashSpreadFoldIntent) -> SlashSpreadPieceMetadataRequirement {
    SlashSpreadPieceMetadataRequirement::preserve_source(fold, id(930), id(931))
}
fn folded_fixture() -> Fixture {
    let mut f = fixture();
    let mut definition = f.piece.definition().clone();
    definition.cut_on_fold = true;
    definition.fold_edges = vec![f.first.edge];
    f.piece = Piece::new(definition, &f.ledger).expect("actual born-valid fold source");
    f
}

#[test]
fn preserve_canonical_source_metadata_and_derived_print_fields_without_second_authority() {
    let mut f = fixture();
    let mut definition = f.piece.definition().clone();
    definition.quantity = 4;
    definition.mirroring = Mirroring::MirroredPairs;
    definition.cutting_side = CuttingSide::WrongSide;
    definition.material = MaterialAssignment::Unresolved("  exact pending assignment  ".into());
    definition.layer_index = -2;
    definition.label.name = "front α".into();
    f.piece = Piece::new(definition, &f.ledger).expect("actual canonical metadata");
    let original = f.piece.clone();
    let journal = f.ledger.journal().len();
    with_boundary(&f, &contours(&f), &manifest(&f), |boundary| {
        let raw = requirement(SlashSpreadFoldIntent::NoFold);
        let metadata = boundary
            .preserve_piece_metadata(&raw)
            .expect("explicit no-fold preservation");
        assert!(std::ptr::eq(metadata.requirement(), &raw));
        assert!(std::ptr::eq(metadata.boundary(), boundary));
        assert!(std::ptr::eq(metadata.source_piece(), &f.piece));
        let source = f.piece.definition();
        assert!(std::ptr::eq(metadata.piece_id(), &source.id));
        assert!(std::ptr::eq(metadata.quantity(), &source.quantity));
        assert!(std::ptr::eq(metadata.mirroring(), &source.mirroring));
        assert!(std::ptr::eq(metadata.cut_on_fold(), &source.cut_on_fold));
        assert!(std::ptr::eq(metadata.cutting_side(), &source.cutting_side));
        assert!(std::ptr::eq(metadata.material(), &source.material));
        assert!(std::ptr::eq(metadata.layer_index(), &source.layer_index));
        assert!(std::ptr::eq(metadata.label(), &source.label));
        assert_eq!(metadata.printed_label(), f.piece.printed_label());
        assert_eq!(
            metadata.requirement().fold(),
            &SlashSpreadFoldIntent::NoFold
        );
        assert_eq!(metadata.requirement().fold_correspondence(), &id(930));
        assert_eq!(metadata.requirement().all_copy_effects(), &id(931));
        assert!(
            metadata.source_fold().is_none()
                && metadata.fold_output().is_none()
                && metadata.fold_edge().is_none()
        );
    });
    assert_eq!(f.piece, original);
    assert_eq!(f.ledger.journal().len(), journal);
}

#[test]
fn explicit_retained_or_derived_fold_keeps_source_state_and_requires_original_fold_attribution() {
    let f = folded_fixture();
    with_boundary(&f, &contours(&f), &manifest(&f), |boundary| {
        let retained = requirement(SlashSpreadFoldIntent::RetainOriginal);
        let metadata = boundary
            .preserve_piece_metadata(&retained)
            .expect("original outer fold");
        assert!(std::ptr::eq(
            metadata.source_fold().expect("old fold field"),
            f.piece.definition().fold_edges.first().expect("original")
        ));
        assert_eq!(metadata.fold_edge(), Some(f.first.edge));
        assert!(metadata.fold_output().is_none());
        let derived = requirement(SlashSpreadFoldIntent::Derived {
            tag: LocalTag::new(9),
        });
        let metadata = boundary
            .preserve_piece_metadata(&derived)
            .expect("explicit output from old fold");
        let output = metadata.fold_output().expect("authentic derived source");
        assert_eq!(metadata.fold_edge(), Some(output.logical_output()));
        assert_eq!(output.candidate().tag(), LocalTag::new(9));
        assert_eq!(metadata.source_fold(), Some(&f.first.edge));
        assert_eq!(
            metadata.printed_label().cut_on_fold,
            f.piece.definition().cut_on_fold
        );
        assert!(!f.ledger.is_live(output.logical_output()));
        assert!(f.ledger.is_live(f.first.edge));
    });
}

#[test]
fn source_fold_state_precedes_output_lookup_and_missing_original_outer_fold_refuses() {
    use SlashSpreadPieceMetadataRefusal as R;
    let f = fixture();
    with_boundary(&f, &contours(&f), &manifest(&f), |boundary| {
        let raw = requirement(SlashSpreadFoldIntent::Derived {
            tag: LocalTag::new(404),
        });
        let error = boundary
            .preserve_piece_metadata(&raw)
            .expect_err("cannot add fold via slash metadata preservation");
        assert_eq!(error.refusal(), R::FoldStatus);
        assert!(std::ptr::eq(error.requirement(), &raw));
        assert!(std::ptr::eq(error.boundary(), boundary));
    });
    let f = folded_fixture();
    with_boundary(&f, &contours(&f), &manifest(&f), |boundary| {
        for (fold, reason) in [
            (SlashSpreadFoldIntent::NoFold, R::FoldStatus),
            (
                SlashSpreadFoldIntent::Derived {
                    tag: LocalTag::new(404),
                },
                R::Output,
            ),
            (
                SlashSpreadFoldIntent::Derived {
                    tag: LocalTag::new(2),
                },
                R::OutputNotBoundary,
            ),
        ] {
            let raw = requirement(fold);
            assert_eq!(
                boundary
                    .preserve_piece_metadata(&raw)
                    .expect_err("original fold association refusal")
                    .refusal(),
                reason
            );
        }
    });
    let base = contours(&f);
    let missing = SlashSpreadContours::new(
        vec![output(9, Direction::Reversed)],
        base.holes().to_vec(),
        base.construction_lines().to_vec(),
    );
    with_boundary(&f, &missing, &manifest(&f), |boundary| {
        let raw = requirement(SlashSpreadFoldIntent::RetainOriginal);
        assert_eq!(
            boundary
                .preserve_piece_metadata(&raw)
                .expect_err("fold lost from outer contour")
                .refusal(),
            R::FoldNotRetained
        );
    });
}

#[test]
fn a_bridge_or_another_source_on_the_outer_boundary_cannot_silently_become_the_original_fold() {
    let f = folded_fixture();
    let base = contours(&f);
    let raw_contours = SlashSpreadContours::new(
        vec![
            SlashSpreadContourEdge::Retained(f.first),
            output(2, Direction::Original),
        ],
        vec![vec![output(9, Direction::Original)]],
        base.construction_lines().to_vec(),
    );
    let bridge = manifest(&f);
    let other_source = SlashSpreadEdgeManifest::new(
        id(10),
        vec![
            *bridge.candidates().first().expect("source"),
            SlashSpreadEdgeCandidate::new(
                LocalTag::new(2),
                SlashSpreadEdgeOrigin::Source {
                    range: DirectedRange {
                        range: EdgeRange::whole(f.second.edge),
                        direction: Direction::Original,
                    },
                    motion: SlashSpreadSourceMotion::Stationary,
                },
            ),
        ],
    );
    for raw_edges in [&bridge, &other_source] {
        with_boundary(&f, &raw_contours, raw_edges, |boundary| {
            let raw = requirement(SlashSpreadFoldIntent::Derived {
                tag: LocalTag::new(2),
            });
            assert_eq!(
                boundary
                    .preserve_piece_metadata(&raw)
                    .expect_err("not attributed to original fold")
                    .refusal(),
                SlashSpreadPieceMetadataRefusal::OutputNotFoldSource
            );
        });
    }
}
