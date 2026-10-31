//! Whole original EdgeRef candidate attribution,distinct from complete occurrence coverage.
#![allow(clippy::expect_used)]
#[path = "support/mirror_edges.rs"]
mod common;
use common::metadata::source as sources;
use common::with_edges;
use sc_core::{construction::*, name::MachineToken, ontology::*};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn source(creator: u128, tag: u32) -> EdgeRef {
    EdgeRef::new(id(creator), LocalTag::new(tag))
}
fn candidate(tag: u32, edge: EdgeRef) -> MirrorPieceEdgeCandidate {
    MirrorPieceEdgeCandidate::new(LocalTag::new(tag), edge)
}

#[test]
fn actual_candidates_and_roles_keep_original_fields_order_and_declared_logical_tags() {
    with_edges(|identity| {
        let original_source = source(90, 7);
        let raw = MirrorPieceEdgeManifest::new(
            id(10),
            vec![candidate(2, original_source), candidate(9, original_source)],
        );
        let original = raw.clone();
        let limits = MirrorPieceEdgeManifestLimits::new(2, 4);
        let edges = raw
            .admit(&identity, limits)
            .expect("distinct output roles may share a declared source");
        assert!(std::ptr::eq(edges.manifest(), &raw));
        assert!(std::ptr::eq(edges.identity(), &identity));
        assert_eq!(edges.limits(), limits);
        assert_eq!((limits.candidates(), limits.workspace_entries()), (2, 4));
        assert_eq!(edges.workspace_entries(), 4);
        assert_eq!(edges.edge_count(), 2);
        assert_eq!(edges.manifest().operation(), id(10));
        for (view, index) in edges.edges().zip([0, 1]) {
            let field = raw.candidates().get(index).expect("actual candidate");
            assert!(std::ptr::eq(view.owner(), &edges));
            assert_eq!(view.index(), index);
            assert!(std::ptr::eq(view.candidate(), field));
            assert!(std::ptr::eq(view.source(), field.source()));
            assert!(std::ptr::eq(
                view.role().catalog(),
                identity.roles().catalog()
            ));
            assert_eq!(view.role().definition().tag(), field.tag());
            assert_eq!(view.logical_output(), EdgeRef::new(id(10), field.tag()));
            assert_eq!(*view.source(), original_source);
        }
        assert_eq!(
            edges.edges().rev().map(|p| p.index()).collect::<Vec<_>>(),
            vec![1, 0]
        );
        assert_eq!(
            edges
                .edge(LocalTag::new(9))
                .expect("explicit tag lookup")
                .index(),
            1
        );
        assert!(edges.edge(LocalTag::new(77)).is_none());
        assert_eq!(raw, original);
        let reversed_raw = MirrorPieceEdgeManifest::new(
            id(10),
            vec![candidate(9, original_source), candidate(2, original_source)],
        );
        let reversed = reversed_raw
            .admit(&identity, limits)
            .expect("candidate order differs from active order");
        assert_eq!(
            reversed
                .edges()
                .map(|p| p.candidate().tag().get())
                .collect::<Vec<_>>(),
            vec![9, 2]
        );
        assert_eq!(
            reversed
                .edge(LocalTag::new(9))
                .expect("persistent lookup")
                .index(),
            0
        );
    });
}

#[test]
fn whole_count_and_workspace_precede_owner_then_original_role_and_source_checks() {
    use MirrorPieceEdgeManifestRefusal as R;
    with_edges(|identity| {
        let raw = MirrorPieceEdgeManifest::new(
            id(99),
            vec![candidate(88, source(10, 42)), candidate(88, source(10, 42))],
        );
        let original = raw.clone();
        for (limits, refusal) in [
            (
                MirrorPieceEdgeManifestLimits::new(1, 0),
                R::Candidates {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                MirrorPieceEdgeManifestLimits::new(2, 3),
                R::Workspace {
                    required: 4,
                    allowed: 3,
                },
            ),
            (MirrorPieceEdgeManifestLimits::new(2, 4), R::Operation),
        ] {
            let error = raw
                .admit(&identity, limits)
                .expect_err("whole refusal before invalid candidate prefix");
            assert!(std::ptr::eq(error.manifest(), &raw));
            assert!(std::ptr::eq(error.identity(), &identity));
            assert_eq!(error.limits(), limits);
            assert_eq!(error.refusal(), refusal);
            assert!(format!("{error}").contains("mirror edge candidate manifest"));
            assert_eq!(raw, original);
        }
        for (tag, refusal) in [
            (
                88,
                R::UndefinedRole {
                    index: 0,
                    tag: LocalTag::new(88),
                },
            ),
            (
                7,
                R::InactiveRole {
                    index: 0,
                    tag: LocalTag::new(7),
                },
            ),
        ] {
            let raw = MirrorPieceEdgeManifest::new(id(10), vec![candidate(tag, source(10, 42))]);
            assert_eq!(
                raw.admit(&identity, MirrorPieceEdgeManifestLimits::new(1, 2))
                    .expect_err("role before self source")
                    .refusal(),
                refusal
            );
        }
    });
}

#[test]
fn repeated_output_tag_then_original_self_and_piece_sources_keep_their_actual_fields() {
    use MirrorPieceEdgeManifestRefusal as R;
    with_edges(|identity| {
        let duplicate = MirrorPieceEdgeManifest::new(
            id(10),
            vec![candidate(9, source(90, 7)), candidate(9, source(10, 42))],
        );
        assert_eq!(
            duplicate
                .admit(&identity, MirrorPieceEdgeManifestLimits::new(2, 4))
                .expect_err("duplicate before second self source")
                .refusal(),
            R::DuplicateTag {
                first: 0,
                second: 1,
                tag: LocalTag::new(9)
            }
        );
        for creator in [10, 100, 200] {
            let edge = source(creator, 42);
            let raw = MirrorPieceEdgeManifest::new(id(10), vec![candidate(9, edge)]);
            let expected = if creator == 10 {
                R::SelfSource {
                    index: 0,
                    source: edge,
                }
            } else {
                R::PieceSource {
                    index: 0,
                    source: edge,
                    piece: id(creator),
                }
            };
            assert_eq!(
                raw.admit(&identity, MirrorPieceEdgeManifestLimits::new(1, 2))
                    .expect_err("actual typed source conflict")
                    .refusal(),
                expected
            );
        }
    });
}

#[test]
fn empty_edge_candidate_metadata_does_not_retire_canonical_roles_or_imply_a_no_op() {
    with_edges(|identity| {
        let raw = MirrorPieceEdgeManifest::new(id(10), vec![]);
        let edges = raw
            .admit(&identity, MirrorPieceEdgeManifestLimits::new(0, 0))
            .expect("explicit empty proposal");
        assert_eq!(edges.workspace_entries(), 0);
        assert_eq!(edges.edge_count(), 0);
        assert_eq!(edges.edges().len(), 0);
        assert!(edges.edge(LocalTag::new(9)).is_none());
        assert!(identity.roles().catalog().is_active(LocalTag::new(9)));
        assert!(identity
            .roles()
            .catalog()
            .definition(LocalTag::new(7))
            .is_some());
        let wrong = MirrorPieceEdgeManifest::new(id(99), vec![]);
        assert_eq!(
            wrong
                .admit(&identity, MirrorPieceEdgeManifestLimits::new(0, 0))
                .expect_err("even empty metadata keeps owner")
                .refusal(),
            MirrorPieceEdgeManifestRefusal::Operation
        );
    });
}

#[test]
fn source_and_selected_material_ids_cannot_supply_required_original_edges() {
    let (ledger, piece) = sources::fixture();
    let mut definition = piece.definition().clone();
    definition.material = MaterialAssignment::Assigned(id(700));
    let piece = Piece::new(definition, &ledger).expect("actual typed source material");
    let input = sources::input(10, 100, sources::axis());
    let source_limits = MirrorPieceSourceLimits::new(2, 8);
    let bound = input
        .bind_piece(&piece, &source_limits)
        .expect("actual source");
    let policy = MirrorPieceCutPolicy::new(
        1,
        Mirroring::Single,
        MirrorPieceFoldIntent::NoFold,
        CuttingSide::Face,
    );
    let cut = bound.bind_cut_policy(&policy).expect("explicit cut");
    let fields = MirrorPieceMetadataPolicy::new(
        MirrorPieceFieldIntent::Replace(MaterialAssignment::Assigned(id(800))),
        MirrorPieceFieldIntent::PreserveSource,
        MirrorPieceFieldIntent::PreserveSource,
    );
    let field_limits = MirrorPieceMetadataLimits::new(27);
    let metadata = cut
        .bind_metadata(&fields, &field_limits)
        .expect("actual result material");
    let edge_raw = EdgeOutputRoleCatalog::new(
        id(10),
        vec![EdgeOutputRoleDefinition::new(
            LocalTag::new(9),
            MachineToken::new("main").expect("key"),
        )],
        vec![LocalTag::new(9)],
    );
    let edge_limits = EdgeOutputRoleCatalogLimits::new(1, 1, 4);
    let edges = edge_raw.admit(edge_limits).expect("edge metadata");
    let edges = metadata
        .bind_edge_roles(&edges, edge_limits)
        .expect("actual edge owner");
    let piece_id = id(200);
    let identity = edges
        .declare_piece_identity(&piece_id)
        .expect("actual Piece declaration");
    for material in [700, 800] {
        let edge = source(material, 42);
        let raw = MirrorPieceEdgeManifest::new(id(10), vec![candidate(9, edge)]);
        assert_eq!(
            raw.admit(&identity, MirrorPieceEdgeManifestLimits::new(1, 2))
                .expect_err("Material is not an edge operation")
                .refusal(),
            MirrorPieceEdgeManifestRefusal::MaterialSource {
                index: 0,
                source: edge,
                material: id(material)
            }
        );
    }
}
