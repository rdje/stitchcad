//! Explicit desired cut policy and actual source/old-result/unrelated inventory for copy intents.
#[path = "mirror_correspondence.rs"]
pub mod context;
use context::edges::metadata::source as sources;
use sc_core::{construction::*, ontology::*};
pub fn with_ids(
    quantity: u32,
    mirroring: Mirroring,
    existing: bool,
    run: impl FnOnce(BoundMirrorPieceCopyIdentities<'_>),
) {
    if quantity == 1 && mirroring == Mirroring::Single {
        return context::with_context(sources::axis(), vec![], |contours, points| {
            inventory(contours, points, existing, run)
        });
    }
    let (_, piece) = sources::fixture();
    let input = sources::input(10, 100, sources::axis());
    let source_limits = MirrorPieceSourceLimits::new(2, 8);
    let source = input
        .bind_piece(&piece, &source_limits)
        .expect("actual source");
    let cut_policy = MirrorPieceCutPolicy::new(
        quantity,
        mirroring,
        MirrorPieceFoldIntent::NoFold,
        CuttingSide::Face,
    );
    let cut = source
        .bind_cut_policy(&cut_policy)
        .expect("explicit desired result cut");
    let fields = MirrorPieceMetadataPolicy::new(
        MirrorPieceFieldIntent::PreserveSource,
        MirrorPieceFieldIntent::PreserveSource,
        MirrorPieceFieldIntent::PreserveSource,
    );
    let field_limits = MirrorPieceMetadataLimits::new(55);
    let metadata = cut
        .bind_metadata(&fields, &field_limits)
        .expect("actual fields");
    context::with_context_metadata(metadata, vec![], |contours, points| {
        inventory(contours, points, existing, run)
    });
}

fn inventory(
    contours: MirrorPieceContours<'_>,
    points: AdmittedMirrorPiecePoints<'_>,
    existing: bool,
    run: impl FnOnce(BoundMirrorPieceCopyIdentities<'_>),
) {
    let (ledger, _) = sources::fixture();
    let piece = contours
        .coverage()
        .edges()
        .identity()
        .roles()
        .metadata()
        .cut()
        .source()
        .piece();
    let id = |bits| EntityId::from_bits(bits);
    let request = MirrorPieceCorrespondenceRequirement::new(
        MirrorPieceAxisEvidenceRequest::ThroughPoints {
            supported_axis: id(600),
        },
        id(601),
        id(602),
        id(603),
        id(604),
        id(605),
        id(606),
    );
    let required = contours
        .require_correspondence(
            &points,
            &request,
            MirrorPieceCorrespondenceLimits::new(4, 0, 8),
        )
        .expect("same actual Piece context");
    let mut pieces = vec![piece.clone()];
    let mut rows = vec![
        CutCopyDefinition {
            id: id(410),
            piece: id(100),
            orientation: CopyOrientation::Authored,
        },
        CutCopyDefinition {
            id: id(207),
            piece: id(100),
            orientation: CopyOrientation::Reflected,
        },
        CutCopyDefinition {
            id: id(900),
            piece: id(100),
            orientation: CopyOrientation::Authored,
        },
        CutCopyDefinition {
            id: id(805),
            piece: id(100),
            orientation: CopyOrientation::Reflected,
        },
    ];
    if existing {
        let mut definition = piece.definition().clone();
        definition.id = id(200);
        definition.quantity = 2;
        definition.mirroring = Mirroring::MirroredPairs;
        pieces.push(Piece::new(definition, &ledger).expect("actual old result birth metadata"));
        rows.extend([
            CutCopyDefinition {
                id: id(550),
                piece: id(200),
                orientation: CopyOrientation::Authored,
            },
            CutCopyDefinition {
                id: id(551),
                piece: id(200),
                orientation: CopyOrientation::Reflected,
            },
        ]);
    }
    let mut definition = piece.definition().clone();
    definition.id = id(300);
    definition.quantity = 1;
    definition.mirroring = Mirroring::Single;
    pieces.push(Piece::new(definition, &ledger).expect("unrelated actual Piece"));
    rows.push(CutCopyDefinition {
        id: id(333),
        piece: id(300),
        orientation: CopyOrientation::Authored,
    });
    let plan = CutPlan::new(rows, &pieces).expect("complete actual inventory");
    let source_limits = MirrorPieceSourceCopyLimits::new(10);
    let source = required
        .preserve_source_copies(&plan, &source_limits)
        .expect("all original source copies");
    let c = plan.copies().len();
    let identity_limits = MirrorPieceCopyIdentityLimits::new(10, 8, 0, 2 * c + 17);
    let identities = source
        .bind_copy_identities(&identity_limits)
        .expect("whole known identity context");
    run(identities);
}
