//! Declared source eligibility only;no geometry,currentness,ledger fold or output Param certificate.
#![allow(clippy::expect_used)]
use sc_core::{construction::*, ontology::*, recipe::FormulaExpressionDocument};
#[path = "support/slash_sources.rs"]
mod sources;
use sources::*;
fn parameter(n: i64, d: i64) -> Param {
    Param::new(Rational::new(n, d).expect("rational")).expect("unit interval")
}
fn query_limits() -> SlashSpreadReferenceQueryLimits {
    SlashSpreadReferenceQueryLimits::new(2, 2, 7)
}
fn repeated_contours(f: &Fixture) -> SlashSpreadContours {
    let raw = contours(f);
    let mut construction = raw.construction_lines().to_vec();
    construction.push(SlashSpreadContourEdge::Retained(DirectedEdge {
        direction: Direction::Reversed,
        ..f.second
    }));
    SlashSpreadContours::new(raw.boundary().to_vec(), raw.holes().to_vec(), construction)
}
fn repeated_limits() -> SlashSpreadContourLimits {
    SlashSpreadContourLimits::new(2, 2, 0, 1, 7)
}
fn reversed_intent(f: &Fixture) -> SlashSpreadOutputIntent {
    let input = SlashSpreadConstruction::rotation(
        id(10),
        id(100),
        DirectedRange {
            range: EdgeRange::whole(f.first.edge),
            direction: Direction::Reversed,
        },
        SlashSpreadSide::Left,
        PointRef::new(id(60), LocalTag::FIRST),
        FormulaExpressionDocument::new("10 deg".into()).expect("angle"),
    )
    .expect("spatial inputs");
    SlashSpreadOutputIntent::new(
        input,
        SlashSpreadBoundaryPolicy::StraightBridges,
        SlashSpreadFullnessIntent::Added,
    )
}
#[test]
fn all_original_source_images_and_both_bridge_endpoints_stay_distinct_in_both_directions() {
    use SlashSpreadBridgeEndpoint as E;
    use SlashSpreadSourceMotion as M;
    for (end, direction, at, banks) in [
        (
            SlashSpreadCutEnd::Upper,
            SlashSpreadBridgeDirection::MovingToStationary,
            Param::END,
            [M::Moving, M::Stationary],
        ),
        (
            SlashSpreadCutEnd::Lower,
            SlashSpreadBridgeDirection::StationaryToMoving,
            Param::START,
            [M::Stationary, M::Moving],
        ),
    ] {
        let f = fixture();
        let intent = reversed_intent(&f);
        let raw_catalog = catalog();
        let catalog = raw_catalog.admit(catalogue_limits()).expect("catalogue");
        let bound = intent
            .bind_edge_roles(&catalog, catalogue_limits())
            .expect("operation");
        let base = manifest(&f);
        let raw_edges = SlashSpreadEdgeManifest::new(
            id(10),
            vec![
                *base.candidates().first().expect("source"),
                SlashSpreadEdgeCandidate::new(
                    LocalTag::new(2),
                    SlashSpreadEdgeOrigin::StraightBridge { end, direction },
                ),
            ],
        );
        let edges = raw_edges
            .admit(
                &bound,
                SlashSpreadEdgeManifestLimits::new(2, catalogue_limits()),
            )
            .expect("edges");
        let raw = repeated_contours(&f);
        let policy = repeated_limits();
        let contours = raw.admit(&edges, &f.piece, &policy).expect("contours");
        let policy = query_limits();
        let anchor = EdgeAnchor {
            edge: f.first.edge,
            param: at,
        };
        let query = contours
            .query_source_reference(anchor, &policy)
            .expect("original cut bound");
        assert!(std::ptr::eq(query.contours(), &contours));
        assert!(std::ptr::eq(query.limits(), &policy));
        assert_eq!(query.anchor(), &anchor);
        assert_eq!(
            (policy.candidates(), policy.source_edges(), policy.uses()),
            (2, 2, 7)
        );
        assert_eq!(query.maximum_match_slots(), 5);
        assert_eq!(query.kind(), SlashSpreadReferenceMatchKind::Ambiguous);
        let matches = query.matches().collect::<Vec<_>>();
        assert_eq!(matches.len(), 4);
        let mut actual = matches.iter().copied();
        let retained = actual.next().expect("retained first");
        assert!(matches!(
            retained.origin(),
            SlashSpreadReferenceOrigin::RetainedOriginal
        ));
        assert_eq!(retained.edge(), f.first.edge);
        assert_eq!(retained.retained_uses().count(), 1);
        let source = actual.next().expect("source candidate");
        let source_output = match source.origin() {
            SlashSpreadReferenceOrigin::Source(output) => Some(output),
            _ => None,
        }
        .expect("original source match required");
        assert!(std::ptr::eq(source_output.admitted(), &edges));
        assert!(std::ptr::eq(
            source_output.candidate(),
            raw_edges.candidates().first().expect("source field")
        ));
        assert_eq!(source.retained_uses().count(), 0);
        for (matched, (bank, endpoint)) in actual.zip(banks.into_iter().zip([E::Start, E::End])) {
            assert!(std::ptr::eq(matched.query(), &query));
            assert_eq!(matched.edge(), EdgeRef::new(id(10), LocalTag::new(2)));
            let (output, actual_bank, actual_endpoint) = match matched.origin() {
                SlashSpreadReferenceOrigin::BridgeEndpoint {
                    output,
                    bank,
                    endpoint,
                } => Some((output, bank, endpoint)),
                _ => None,
            }
            .expect("bridge occurrence required");
            assert_eq!((actual_bank, actual_endpoint), (bank, endpoint));
            assert!(std::ptr::eq(
                output.candidate(),
                raw_edges.candidates().get(1).expect("bridge field")
            ));
            assert_eq!(matched.retained_uses().count(), 0);
        }
        assert_eq!(
            query.matches().rev().map(|m| m.edge()).collect::<Vec<_>>(),
            matches.iter().rev().map(|m| m.edge()).collect::<Vec<_>>()
        );
        let second = contours
            .query_source_reference(
                EdgeAnchor {
                    edge: f.second.edge,
                    param: parameter(1, 2),
                },
                &policy,
            )
            .expect("original construction source");
        assert_eq!(second.kind(), SlashSpreadReferenceMatchKind::Unique);
        let second_match = second.matches().next().expect("one retained identity");
        assert_eq!(
            second_match
                .retained_uses()
                .map(|u| u.location())
                .collect::<Vec<_>>(),
            vec![
                SlashSpreadContourLocation::Construction(2),
                SlashSpreadContourLocation::Construction(3)
            ]
        );
    }
}
#[test]
fn quotas_precede_exact_external_source_frame_and_scope_refusals_with_all_original_inputs() {
    use SlashSpreadReferenceQueryRefusal as R;
    let f = fixture();
    let intent = intent(&f);
    let raw_catalog = catalog();
    let catalog = raw_catalog.admit(catalogue_limits()).expect("catalogue");
    let bound = intent
        .bind_edge_roles(&catalog, catalogue_limits())
        .expect("operation");
    let base = manifest(&f);
    let external = DirectedRange {
        range: EdgeRange::new(
            EdgeRef::new(id(55), LocalTag::new(17)),
            parameter(1, 4),
            parameter(3, 4),
        )
        .expect("declared external source"),
        direction: Direction::Reversed,
    };
    let raw_edges = SlashSpreadEdgeManifest::new(
        id(10),
        vec![
            SlashSpreadEdgeCandidate::new(
                LocalTag::new(9),
                SlashSpreadEdgeOrigin::Source {
                    range: external,
                    motion: SlashSpreadSourceMotion::Moving,
                },
            ),
            *base.candidates().get(1).expect("bridge"),
        ],
    );
    let edges = raw_edges
        .admit(
            &bound,
            SlashSpreadEdgeManifestLimits::new(2, catalogue_limits()),
        )
        .expect("provenance metadata,not source ownership");
    let raw = repeated_contours(&f);
    let policy = repeated_limits();
    let contours = raw.admit(&edges, &f.piece, &policy).expect("contours");
    let unknown = EdgeAnchor {
        edge: EdgeRef::new(id(999), LocalTag::FIRST),
        param: parameter(1, 2),
    };
    let outside = EdgeAnchor {
        edge: external.range.edge(),
        param: parameter(1, 8),
    };
    for (anchor, policy, cause) in [
        (
            unknown,
            SlashSpreadReferenceQueryLimits::new(1, 0, 0),
            R::Candidates {
                required: 2,
                allowed: 1,
            },
        ),
        (
            unknown,
            SlashSpreadReferenceQueryLimits::new(2, 1, 0),
            R::SourceEdges {
                required: 2,
                allowed: 1,
            },
        ),
        (
            unknown,
            SlashSpreadReferenceQueryLimits::new(2, 2, 6),
            R::Uses {
                required: 7,
                allowed: 6,
            },
        ),
        (unknown, query_limits(), R::SourceEdge),
        (outside, query_limits(), R::SourcePosition),
    ] {
        let error = contours
            .query_source_reference(anchor, &policy)
            .expect_err("whole query refused");
        assert!(std::ptr::eq(error.contours(), &contours));
        assert!(std::ptr::eq(error.limits(), &policy));
        assert_eq!(error.anchor(), &anchor);
        assert_eq!(error.refusal(), cause);
        assert!(format!("{error}").contains("source eligibility"));
        assert!(format!("{error:?}").contains("SlashSpreadReferenceQueryError"));
    }
    let policy = query_limits();
    let inside = contours
        .query_source_reference(
            EdgeAnchor {
                edge: external.range.edge(),
                param: parameter(1, 2),
            },
            &policy,
        )
        .expect("declared external domain");
    assert_eq!(inside.kind(), SlashSpreadReferenceMatchKind::Unique);
    assert_eq!(
        inside.matches().next().expect("source").edge(),
        EdgeRef::new(id(10), LocalTag::new(9))
    );
}
#[test]
fn gaps_and_later_journal_aliases_neither_discharge_repairs_nor_mutate_original_metadata() {
    let mut f = fixture();
    let intent = intent(&f);
    let raw_catalog = catalog();
    let original_catalog = raw_catalog.clone();
    let catalog = raw_catalog.admit(catalogue_limits()).expect("catalogue");
    let bound = intent
        .bind_edge_roles(&catalog, catalogue_limits())
        .expect("operation");
    let raw_edges = manifest(&f);
    let edges = raw_edges
        .admit(
            &bound,
            SlashSpreadEdgeManifestLimits::new(2, catalogue_limits()),
        )
        .expect("edges");
    let base = contours(&f);
    let raw = SlashSpreadContours::new(
        base.boundary().to_vec(),
        base.holes().to_vec(),
        vec![
            output(9, Direction::Original),
            output(9, Direction::Reversed),
        ],
    );
    let original = raw.clone();
    let policy = limits();
    let contours = raw
        .admit(&edges, &f.piece, &policy)
        .expect("all actual outputs used");
    let policy = query_limits();
    let gap = contours
        .query_source_reference(
            EdgeAnchor {
                edge: f.second.edge,
                param: parameter(1, 2),
            },
            &policy,
        )
        .expect("known original source scope");
    assert_eq!(gap.kind(), SlashSpreadReferenceMatchKind::Gap);
    assert_eq!(gap.matches().count(), 0);
    assert!(f.ledger.is_live(f.second.edge));
    let mut ids = DeterministicIdGenerator::from_start(1000);
    let children = f
        .ledger
        .split(&mut ids, f.first.edge, parameter(1, 2))
        .expect("later actual split");
    let old = contours
        .query_source_reference(
            EdgeAnchor {
                edge: f.first.edge,
                param: parameter(1, 4),
            },
            &policy,
        )
        .expect("original frame remains inspectable");
    assert_eq!(old.kind(), SlashSpreadReferenceMatchKind::Ambiguous);
    assert_eq!(old.matches().count(), 2);
    assert!(!f.ledger.is_live(f.first.edge));
    let alias = EdgeAnchor {
        edge: children.first(),
        param: parameter(1, 2),
    };
    assert_eq!(
        contours
            .query_source_reference(alias, &policy)
            .expect_err("no silent alias substitution")
            .refusal(),
        SlashSpreadReferenceQueryRefusal::SourceEdge
    );
    assert_eq!(raw, original);
    assert_eq!(raw_catalog, original_catalog);
}
#[test]
fn source_quota_counts_original_repetitions_not_deduplicated_membership_entries() {
    let f = fixture();
    let intent = intent(&f);
    let raw_catalog = catalog();
    let catalog = raw_catalog.admit(catalogue_limits()).expect("catalogue");
    let bound = intent
        .bind_edge_roles(&catalog, catalogue_limits())
        .expect("operation");
    let raw_edges = manifest(&f);
    let edges = raw_edges
        .admit(
            &bound,
            SlashSpreadEdgeManifestLimits::new(2, catalogue_limits()),
        )
        .expect("edges");
    let mut definition = f.piece.definition().clone();
    definition.construction_lines.push(f.second);
    let source =
        Piece::new(definition, &f.ledger).expect("legal repeated source construction occurrence");
    let raw = contours(&f);
    let policy = SlashSpreadContourLimits::new(2, 3, 0, 1, 6);
    let contours = raw
        .admit(&edges, &source, &policy)
        .expect("all source occurrences admitted");
    assert_eq!(contours.source_edge_count(), 3);
    let policy = SlashSpreadReferenceQueryLimits::new(2, 2, 6);
    assert_eq!(
        contours
            .query_source_reference(
                EdgeAnchor {
                    edge: f.second.edge,
                    param: parameter(1, 2)
                },
                &policy
            )
            .expect_err("complete R3 required")
            .refusal(),
        SlashSpreadReferenceQueryRefusal::SourceEdges {
            required: 3,
            allowed: 2
        }
    );
}
