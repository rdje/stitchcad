//! Authentic whole contour and point proposals sharing one actual canonical Piece declaration.
#[path = "mirror_edges.rs"]
pub mod edges;
use edges::metadata::source as sources;
use sc_core::{construction::*, name::MachineToken, ontology::*};

pub fn with_context(
    axis: MirrorAxis,
    point_candidates: Vec<MirrorPiecePointCandidate>,
    run: impl FnOnce(MirrorPieceContours<'_>, AdmittedMirrorPiecePoints<'_>),
) {
    if axis == sources::axis() {
        return edges::with_edges(|identity| complete_context(identity, point_candidates, run));
    }
    let (_, piece) = sources::fixture();
    with_context_piece(&piece, axis, point_candidates, run);
}

pub fn with_context_piece(
    piece: &Piece,
    axis: MirrorAxis,
    point_candidates: Vec<MirrorPiecePointCandidate>,
    run: impl FnOnce(MirrorPieceContours<'_>, AdmittedMirrorPiecePoints<'_>),
) {
    let input = MirrorPieceCopy::new(EntityId::from_bits(10), piece.id(), axis)
        .expect("explicit source/axis");
    let source_limits = MirrorPieceSourceLimits::new(2, 8);
    let source = input
        .bind_piece(piece, &source_limits)
        .expect("whole source");
    let cut_policy = MirrorPieceCutPolicy::new(
        1,
        Mirroring::Single,
        MirrorPieceFoldIntent::NoFold,
        CuttingSide::Face,
    );
    let cut = source.bind_cut_policy(&cut_policy).expect("explicit cut");
    let fields = MirrorPieceMetadataPolicy::new(
        MirrorPieceFieldIntent::PreserveSource,
        MirrorPieceFieldIntent::PreserveSource,
        MirrorPieceFieldIntent::PreserveSource,
    );
    let field_limits = MirrorPieceMetadataLimits::new(55);
    let metadata = cut
        .bind_metadata(&fields, &field_limits)
        .expect("actual fields");
    with_context_metadata(metadata, point_candidates, run);
}

pub fn with_context_metadata(
    metadata: BoundMirrorPieceMetadata<'_>,
    point_candidates: Vec<MirrorPiecePointCandidate>,
    run: impl FnOnce(MirrorPieceContours<'_>, AdmittedMirrorPiecePoints<'_>),
) {
    edges::with_edges_metadata(metadata, |identity| {
        complete_context(identity, point_candidates, run)
    });
}

fn complete_context(
    identity: DeclaredMirrorPieceIdentity<'_>,
    point_candidates: Vec<MirrorPiecePointCandidate>,
    run: impl FnOnce(MirrorPieceContours<'_>, AdmittedMirrorPiecePoints<'_>),
) {
    let point_raw = PointOutputRoleCatalog::new(
        EntityId::from_bits(10),
        [(9, "main"), (2, "aux")]
            .into_iter()
            .map(|(tag, key)| {
                PointOutputRoleDefinition::new(
                    LocalTag::new(tag),
                    MachineToken::new(key).expect("key"),
                )
            })
            .collect(),
        [9, 2].into_iter().map(LocalTag::new).collect(),
    );
    let point_limits = PointOutputRoleCatalogLimits::new(2, 2, 7);
    let point_catalog = point_raw.admit(point_limits).expect("point roles");
    let point_roles = identity
        .bind_point_roles(&point_catalog, point_limits)
        .expect("same actual identity");
    let point_manifest = MirrorPiecePointManifest::new(EntityId::from_bits(10), point_candidates);
    let points = point_manifest
        .admit(&point_roles, MirrorPiecePointManifestLimits::new(2, 4))
        .expect("explicit original points");
    let edge_raw = MirrorPieceEdgeManifest::new(
        EntityId::from_bits(10),
        [(6, 43), (2, 41), (9, 40), (4, 42)]
            .into_iter()
            .map(|(tag, id)| {
                MirrorPieceEdgeCandidate::new(
                    LocalTag::new(tag),
                    EdgeRef::new(EntityId::from_bits(id), LocalTag::FIRST),
                )
            })
            .collect(),
    );
    let edge_candidates = edge_raw
        .admit(&identity, MirrorPieceEdgeManifestLimits::new(4, 8))
        .expect("actual edges");
    let coverage = edge_candidates
        .admit_source_coverage(MirrorPieceEdgeCoverageLimits::new(4, 8, 12))
        .expect("complete coverage");
    let policy = MirrorPieceContourPolicy::new(
        EntityId::from_bits(10),
        [4, 9, 6, 2]
            .into_iter()
            .map(|tag| {
                MirrorPieceEdgeParameterization::new(LocalTag::new(tag), Direction::Original)
            })
            .collect(),
    );
    let contours = coverage
        .declare_contours(&policy, MirrorPieceContourLimits::new(4, 2, 8, 18))
        .expect("desired contours");
    run(contours, points);
}
