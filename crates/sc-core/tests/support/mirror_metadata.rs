//! Actual immutable source/cut/field association for focused mirror manifest contracts.
#[path = "mirror_source.rs"]
mod source;
use sc_core::{construction::*, ontology::*};

pub fn with_metadata(run: impl FnOnce(BoundMirrorPieceMetadata<'_>)) {
    let (_, piece) = source::fixture();
    let input = source::input(10, 100, source::axis());
    let source_limits = MirrorPieceSourceLimits::new(2, 8);
    let source = input
        .bind_piece(&piece, &source_limits)
        .expect("actual source");
    let cut_policy = MirrorPieceCutPolicy::new(
        1,
        Mirroring::Single,
        MirrorPieceFoldIntent::NoFold,
        CuttingSide::Face,
    );
    let cut = source.bind_cut_policy(&cut_policy).expect("explicit cut");
    let policy = MirrorPieceMetadataPolicy::new(
        MirrorPieceFieldIntent::PreserveSource,
        MirrorPieceFieldIntent::PreserveSource,
        MirrorPieceFieldIntent::PreserveSource,
    );
    let limits = MirrorPieceMetadataLimits::new(55);
    let metadata = cut
        .bind_metadata(&policy, &limits)
        .expect("actual selected fields");
    run(metadata);
}
