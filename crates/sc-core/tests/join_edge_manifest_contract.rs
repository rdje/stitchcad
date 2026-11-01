//! Explicit whole join Edge candidate attribution to actual original occurrence fields.
#![allow(clippy::expect_used)]
#[allow(dead_code)]
#[path = "support/join_edges.rs"]
mod common;
use common::{id, with_identity, BankFixture};
use sc_core::{construction::*, ontology::*};
fn tag(n: u32) -> LocalTag {
    LocalTag::new(n)
}
fn range(identity: &DeclaredJoinPieceIdentity<'_>, side: SeamSideId) -> EdgeRange {
    *identity.roles().banks().bank(side).range()
}
fn candidate(
    t: u32,
    side: SeamSideId,
    location: JoinSourceOccurrenceLocation,
    source: EdgeRange,
) -> JoinEdgeCandidate {
    JoinEdgeCandidate::new(tag(t), side, location, source)
}
#[test]
fn explicit_roles_order_repeated_sources_and_partial_ranges_keep_original_fields_and_parents() {
    with_identity(BankFixture::basic(), |identity| {
        use JoinSourceOccurrenceLocation::Boundary;
        let a = range(&identity, SeamSideId::A);
        let b = range(&identity, SeamSideId::B);
        let half = Param::new(Rational::new(1, 2).expect("half")).expect("parameter");
        let raw = JoinEdgeManifest::new(
            id(1000),
            vec![
                candidate(2, SeamSideId::B, Boundary(0), b),
                candidate(
                    9,
                    SeamSideId::A,
                    Boundary(0),
                    EdgeRange::new(a.edge(), half, Param::END).expect("upper half"),
                ),
                candidate(
                    4,
                    SeamSideId::A,
                    Boundary(0),
                    EdgeRange::new(a.edge(), Param::START, half).expect("lower half"),
                ),
            ],
        );
        let snapshot = raw.clone();
        let limits = JoinEdgeManifestLimits::new(3, 6);
        let edges = raw
            .admit(&identity, limits)
            .expect("literal attribution only");
        assert!(std::ptr::eq(edges.manifest(), &raw));
        assert!(std::ptr::eq(edges.identity(), &identity));
        assert_eq!(edges.limits(), limits);
        assert_eq!(edges.workspace_entries(), 6);
        assert_eq!(edges.edge_count(), 3);
        assert_eq!(raw.operation(), id(1000));
        assert_eq!(limits.candidates(), 3);
        assert_eq!(limits.workspace_entries(), 6);
        assert_eq!(
            edges
                .edges()
                .map(|e| e.candidate().tag())
                .collect::<Vec<_>>(),
            [tag(2), tag(9), tag(4)]
        );
        assert_eq!(
            edges
                .edges()
                .rev()
                .map(|e| e.candidate().tag())
                .collect::<Vec<_>>(),
            [tag(4), tag(9), tag(2)]
        );
        for (index, edge) in edges.edges().enumerate() {
            let field = raw.candidates().get(index).expect("original candidate");
            assert!(std::ptr::eq(edge.owner(), &edges));
            assert!(std::ptr::eq(edge.candidate(), field));
            assert!(std::ptr::eq(edge.source(), field.source()));
            assert_eq!(edge.index(), index);
            assert_eq!(edge.logical_output(), EdgeRef::new(id(1000), field.tag()));
            assert!(std::ptr::eq(
                edge.role().catalog(),
                identity.roles().catalog()
            ));
            assert_eq!(edge.occurrence().side(), field.side());
            assert_eq!(edge.occurrence().location(), field.location());
            let original = identity
                .roles()
                .banks()
                .sources()
                .piece(field.side())
                .definition()
                .boundary
                .first()
                .expect("original field");
            assert!(std::ptr::eq(
                edge.occurrence().field().edge(),
                &original.edge
            ));
            assert!(std::ptr::eq(
                edges.edge(field.tag()).expect("explicit tag").candidate(),
                field
            ));
        }
        assert!(edges.edge(tag(13)).is_none());
        assert!(identity.roles().catalog().active_role(tag(13)).is_some());
        assert_eq!(raw, snapshot);
        assert!(format!("{edges:?}").contains("AdmittedJoinEdges"));
    });
}
#[test]
fn complete_n_and_2n_quotas_precede_operation_and_malformed_candidate_fields() {
    use JoinEdgeManifestRefusal as R;
    with_identity(BankFixture::basic(), |identity| {
        let bad = candidate(
            99,
            SeamSideId::A,
            JoinSourceOccurrenceLocation::Boundary(99),
            range(&identity, SeamSideId::B),
        );
        let raw = JoinEdgeManifest::new(id(1001), vec![bad, bad, bad]);
        let snapshot = raw.clone();
        for (limits, cause) in [
            (
                JoinEdgeManifestLimits::new(2, 0),
                R::Candidates {
                    required: 3,
                    allowed: 2,
                },
            ),
            (
                JoinEdgeManifestLimits::new(3, 5),
                R::Workspace {
                    required: 6,
                    allowed: 5,
                },
            ),
            (JoinEdgeManifestLimits::new(3, 6), R::Operation),
        ] {
            let error = raw
                .admit(&identity, limits)
                .expect_err("whole resources before fields");
            assert!(std::ptr::eq(error.manifest(), &raw));
            assert!(std::ptr::eq(error.identity(), &identity));
            assert_eq!(error.limits(), limits);
            assert_eq!(error.refusal(), cause);
            assert!(error.refused_candidate().is_none() && error.refused_occurrence().is_none());
            assert!(error
                .to_string()
                .starts_with("join edge candidate manifest:"));
            assert!(format!("{error:?}").contains("JoinEdgeManifestError"));
        }
        assert_eq!(raw, snapshot);
    });
}
#[test]
fn first_original_role_location_and_edge_causes_preserve_whole_proposals_without_accepted_prefixes()
{
    use JoinEdgeManifestRefusal as R;
    use JoinSourceOccurrenceLocation as L;
    with_identity(BankFixture::basic(), |identity| {
        let a = range(&identity, SeamSideId::A);
        let b = range(&identity, SeamSideId::B);
        let valid = candidate(9, SeamSideId::A, L::Boundary(0), a);
        let missing = [
            L::Boundary(99),
            L::Hole { hole: 0, index: 0 },
            L::Hole {
                hole: 99,
                index: 99,
            },
            L::Construction(0),
            L::Fold(0),
        ];
        let mut cases = vec![
            (
                candidate(9, SeamSideId::B, L::Boundary(99), a),
                R::DuplicateTag {
                    first: 0,
                    second: 1,
                    tag: tag(9),
                },
            ),
            (
                candidate(99, SeamSideId::B, L::Boundary(99), a),
                R::UndefinedRole {
                    index: 1,
                    tag: tag(99),
                },
            ),
            (
                candidate(7, SeamSideId::B, L::Boundary(99), a),
                R::InactiveRole {
                    index: 1,
                    tag: tag(7),
                },
            ),
            (
                candidate(2, SeamSideId::B, L::Boundary(0), a),
                R::SourceEdge { index: 1 },
            ),
            (
                candidate(2, SeamSideId::A, L::Boundary(0), b),
                R::SourceEdge { index: 1 },
            ),
        ];
        cases.extend(missing.into_iter().map(|location| {
            (
                candidate(2, SeamSideId::B, location, b),
                R::MissingOccurrence { index: 1 },
            )
        }));
        for (bad, cause) in cases {
            let raw = JoinEdgeManifest::new(
                id(1000),
                vec![valid, bad, candidate(4, SeamSideId::B, L::Boundary(0), b)],
            );
            let snapshot = raw.clone();
            let limits = JoinEdgeManifestLimits::new(3, 6);
            let error = raw
                .admit(&identity, limits)
                .expect_err("first original failure");
            assert_eq!(error.refusal(), cause);
            assert_eq!(raw, snapshot);
            assert!(std::ptr::eq(error.manifest(), &raw));
            assert!(std::ptr::eq(error.identity(), &identity));
            assert!(std::ptr::eq(
                error.refused_candidate().expect("candidate cause"),
                raw.candidates().get(1).expect("actual rejected field")
            ));
            if matches!(cause, R::SourceEdge { .. }) {
                let occurrence = error
                    .refused_occurrence()
                    .expect("actual mismatched occurrence");
                let original = identity
                    .roles()
                    .banks()
                    .sources()
                    .piece(bad.side())
                    .definition()
                    .boundary
                    .first()
                    .expect("actual source");
                assert!(std::ptr::eq(occurrence.field().edge(), &original.edge));
            } else {
                assert!(error.refused_occurrence().is_none());
            }
        }
    });
}
#[test]
fn every_original_family_and_repeated_field_binds_without_normalizing_and_empty_metadata_is_explicit(
) {
    with_identity(
        BankFixture {
            extra_occurrences: true,
            ..BankFixture::basic()
        },
        |identity| {
            let sources = identity.roles().banks().sources();
            let fields = sources.occurrences().collect::<Vec<_>>();
            assert_eq!(fields.len(), 10);
            let tags = [9, 2, 4, 5, 6, 8, 10, 11, 12, 13];
            let raw = JoinEdgeManifest::new(
                id(1000),
                fields
                    .iter()
                    .zip(tags)
                    .map(|(field, t)| {
                        candidate(
                            t,
                            field.side(),
                            field.location(),
                            EdgeRange::whole(*field.field().edge()),
                        )
                    })
                    .collect(),
            );
            let edges = raw
                .admit(&identity, JoinEdgeManifestLimits::new(10, 20))
                .expect("every literal original family");
            for (edge, original) in edges.edges().zip(fields.iter()) {
                assert!(std::ptr::eq(
                    edge.occurrence().field().edge(),
                    original.field().edge()
                ));
                let selected = sources
                    .occurrence(original.side(), original.location())
                    .expect("direct source lookup");
                assert!(std::ptr::eq(selected.context(), sources));
                assert!(std::ptr::eq(
                    selected.field().edge(),
                    original.field().edge()
                ));
                match (edge.occurrence().field(), original.field()) {
                    (
                        JoinSourceOccurrenceField::Directed(a),
                        JoinSourceOccurrenceField::Directed(b),
                    ) => {
                        assert!(std::ptr::eq(a, b));
                        assert_eq!(a.direction, b.direction);
                    }
                    (JoinSourceOccurrenceField::Fold(a), JoinSourceOccurrenceField::Fold(b)) => {
                        assert!(std::ptr::eq(a, b))
                    }
                    _ => assert_eq!(
                        std::mem::discriminant(&edge.occurrence().field()),
                        std::mem::discriminant(&original.field())
                    ),
                }
            }
            for location in [
                JoinSourceOccurrenceLocation::Hole { hole: 0, index: 99 },
                JoinSourceOccurrenceLocation::Hole { hole: 99, index: 0 },
            ] {
                assert!(sources.occurrence(SeamSideId::A, location).is_none());
            }
            let raw = JoinEdgeManifest::new(id(1000), vec![]);
            let empty = raw
                .admit(&identity, JoinEdgeManifestLimits::new(0, 0))
                .expect("presence metadata only");
            assert_eq!(empty.edge_count(), 0);
            assert_eq!(empty.edges().len(), 0);
            assert_eq!(empty.workspace_entries(), 0);
            assert_eq!(empty.identity().roles().banks().banks().len(), 2);
            assert_eq!(
                *empty
                    .identity()
                    .roles()
                    .banks()
                    .sources()
                    .population()
                    .quantity(),
                1
            );
        },
    );
}
