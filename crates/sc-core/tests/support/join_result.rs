//! Actual retained construction partitions,two holes and explicit fold/copy parents for result matching.
#[path = "join_candidate.rs"]
pub mod candidate;
pub use candidate::id;
use candidate::pairing::copies::fold::edges;
use sc_core::{construction::*, ontology::*};

pub fn with_result(
    change: impl FnOnce(&mut PieceDefinition),
    run: impl FnOnce(BoundJoinInventoryFields<'_>),
) {
    use Direction::{Original as O, Reversed as R};
    let proposed = [
        (11, O),
        (5, R),
        (4, R),
        (12, O),
        (4, O),
        (11, R),
        (5, O),
        (12, R),
    ]
    .into_iter()
    .map(|(tag, direction)| JoinConstructionUse::new(LocalTag::new(tag), direction))
    .collect();
    edges::with_partitioned_sources(proposed, |sources| {
        use SeamSideId::{A, B};
        let groups = [
            (A, 0, [4, 5]),
            (A, 1, [5, 4]),
            (B, 0, [11, 12]),
            (B, 1, [12, 11]),
        ]
        .into_iter()
        .map(|(side, index, tags)| {
            JoinConstructionMappingGroup::new(
                side,
                index,
                tags.into_iter().map(LocalTag::new).collect(),
            )
        })
        .collect();
        let raw = JoinConstructionGroupManifest::new(id(1000), groups);
        let groups = raw
            .admit(&sources, JoinConstructionGroupLimits::new(4, 4, 8, 16))
            .expect("actual retained whole-field groups");
        let coverage = groups
            .require_source_coverage(JoinConstructionCoverageLimits::new(4, 8))
            .expect("actual complete original partitions");
        let rows = [
            (A, 0, [4, 1]),
            (A, 1, [6, 2]),
            (B, 0, [5, 3]),
            (B, 1, [7, 0]),
        ]
        .into_iter()
        .map(|(side, index, uses)| {
            JoinConstructionTargetAssignment::new(side, index, uses.to_vec())
        })
        .collect();
        let raw = JoinConstructionTargetManifest::new(
            id(1000),
            rows,
            JoinAdditionalConstructionIntent::SourceMappedOnly,
        );
        let declared = raw
            .admit(&coverage, JoinConstructionTargetLimits::new(4, 8, 8, 20))
            .expect("complete actual retained occurrence assignments");
        let construction = declared
            .require_target_coverage(JoinConstructionTargetCoverageLimits::new(8, 8))
            .expect("every declared construction occurrence assigned");
        let raw = JoinFoldRequirement::new(
            id(1000),
            JoinFoldIntent::CutOnFold {
                tag: LocalTag::new(9),
            },
            id(0),
        );
        let fold = construction
            .require_fold(&raw, JoinFoldLimits::new(10, 10, 4))
            .expect("explicit outer fold request");
        let sources = fold
            .construction()
            .targets()
            .coverage()
            .groups()
            .sources()
            .uses()
            .connectivity()
            .contours()
            .directions()
            .coverage()
            .edges()
            .identity()
            .roles()
            .banks()
            .sources();
        let rows = sources
            .occurrences()
            .filter_map(|f| match f.location() {
                JoinSourceOccurrenceLocation::Fold(index) => Some(JoinFoldSourceAssignment::new(
                    f.side(),
                    index,
                    *f.field().edge(),
                    JoinFoldSourceIntent::Omit {
                        evidence_source: id(0),
                    },
                )),
                _ => None,
            })
            .collect();
        let raw = JoinFoldSourceManifest::new(id(1000), rows);
        let folds = raw
            .admit(&fold, JoinFoldSourceLimits::new(10, 2, 4))
            .expect("both original fold omission requests stay visible");
        let raw = JoinResultCopyManifest::new(
            id(1000),
            vec![CutCopyDefinition {
                id: id(6000),
                piece: id(5000),
                orientation: CopyOrientation::Authored,
            }],
        );
        let results = raw
            .admit(&folds, JoinResultCopyLimits::new(2, 2, 1, 5))
            .expect("explicit one-copy result");
        let rows = [200, 201]
            .into_iter()
            .map(|bits| {
                JoinCopySourceAssignment::new(
                    *results
                        .source_copy(id(bits))
                        .expect("actual original copy")
                        .definition(),
                    JoinCopySourceIntent::JoinTo {
                        result_copy: id(6000),
                    },
                )
            })
            .collect();
        let raw = JoinCopySourceManifest::new(id(1000), rows);
        let actions = raw
            .admit(&results, JoinCopySourceLimits::new(2, 2, 4))
            .expect("complete original physical-copy actions");
        let pairs = actions
            .require_pairs(JoinCopyPairLimits::new(2, 1, 3))
            .expect("exact selected A/B pair");
        let mut d = results
            .source_piece(id(100))
            .expect("actual original A selected metadata")
            .definition()
            .clone();
        d.id = id(5000);
        d.quantity = 1;
        d.mirroring = Mirroring::Single;
        let edge = |tag, direction| DirectedEdge {
            edge: EdgeRef::new(id(1000), LocalTag::new(tag)),
            direction,
        };
        use Direction::{Original as O, Reversed as R};
        d.boundary = vec![edge(9, R), edge(8, O)];
        d.holes = vec![vec![edge(10, O)], vec![edge(2, R)]];
        d.construction_lines = vec![
            edge(11, O),
            edge(5, R),
            edge(4, R),
            edge(12, O),
            edge(4, O),
            edge(11, R),
            edge(5, O),
            edge(12, R),
        ];
        d.cut_on_fold = true;
        d.fold_edges = vec![EdgeRef::new(id(1000), LocalTag::new(9))];
        change(&mut d);
        let mut ledger = IdentityLedger::new();
        for (operation, count) in [(1, 2), (2, 2), (1000, 14)] {
            ledger
                .declare_edges(&mut DeterministicIdGenerator::from_start(operation), count)
                .expect("fixture published identity ledger");
        }
        let inventory = PieceInventory::new(vec![
            Piece::new(d, &ledger).expect("structurally born supplied candidate")
        ])
        .expect("whole actual candidate inventory");
        let plan = CutPlan::new(
            vec![*results
                .copy(id(6000))
                .expect("actual explicit result request")
                .definition()],
            inventory.pieces(),
        )
        .expect("canonical complete candidate copy plan");
        let plan = pairs
            .bind_candidate_plan(
                &inventory,
                &plan,
                JoinCandidatePlanLimits::new(2, 2, 1, 1, 1, 4),
            )
            .expect("exact candidate copy definitions");
        let fields = plan
            .admit_piece_fields(JoinInventoryFieldLimits::new(2, 1, 4, 23, 1000))
            .expect("whole original/candidate fields before equality");
        run(fields);
    });
}
