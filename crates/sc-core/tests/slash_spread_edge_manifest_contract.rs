//! Candidate provenance and active roles only;no actual contour/transform/reference map is executed.
#![allow(clippy::expect_used)]
use sc_core::{
    construction::*, name::MachineToken, ontology::*, recipe::FormulaExpressionDocument,
};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn parameter(n: i64, d: i64) -> Param {
    Param::new(Rational::new(n, d).expect("rational")).expect("unit interval")
}
fn range(creator: u128) -> DirectedRange {
    DirectedRange {
        range: EdgeRange::new(
            EdgeRef::new(id(creator), LocalTag::new(7)),
            parameter(1, 4),
            parameter(3, 4),
        )
        .expect("positive source"),
        direction: Direction::Reversed,
    }
}
fn intent(
    mode: SlashSpreadOpeningMode,
    policy: SlashSpreadBoundaryPolicy,
) -> SlashSpreadOutputIntent {
    let cut = range(20);
    let input = match mode {
        SlashSpreadOpeningMode::Rotation => SlashSpreadConstruction::rotation(
            id(10),
            id(50),
            cut,
            SlashSpreadSide::Left,
            PointRef::new(id(30), LocalTag::new(4)),
            FormulaExpressionDocument::new("10 deg".into()).expect("angle"),
        ),
        SlashSpreadOpeningMode::Separation => SlashSpreadConstruction::separation(
            id(10),
            id(50),
            cut,
            SlashSpreadSide::Right,
            range(40),
            FormulaExpressionDocument::new("10 mm".into()).expect("distance"),
        ),
    }
    .expect("input roles");
    SlashSpreadOutputIntent::new(input, policy, SlashSpreadFullnessIntent::Added)
}
fn raw_catalog() -> EdgeOutputRoleCatalog {
    EdgeOutputRoleCatalog::new(
        id(10),
        [(9, "main"), (2, "secondary"), (7, "latent")]
            .into_iter()
            .map(|(tag, key)| {
                EdgeOutputRoleDefinition::new(
                    LocalTag::new(tag),
                    MachineToken::new(key).expect("key"),
                )
            })
            .collect(),
        vec![LocalTag::new(2), LocalTag::new(9)],
    )
}
fn catalogue_limits() -> EdgeOutputRoleCatalogLimits {
    EdgeOutputRoleCatalogLimits::new(3, 2, 19)
}
fn limits(n: usize) -> SlashSpreadEdgeManifestLimits {
    SlashSpreadEdgeManifestLimits::new(n, catalogue_limits())
}
fn source(creator: u128, motion: SlashSpreadSourceMotion) -> SlashSpreadEdgeOrigin {
    SlashSpreadEdgeOrigin::Source {
        range: range(creator),
        motion,
    }
}
fn bridge() -> SlashSpreadEdgeOrigin {
    SlashSpreadEdgeOrigin::StraightBridge {
        end: SlashSpreadCutEnd::Lower,
        direction: SlashSpreadBridgeDirection::StationaryToMoving,
    }
}
fn candidate(tag: u32, origin: SlashSpreadEdgeOrigin) -> SlashSpreadEdgeCandidate {
    SlashSpreadEdgeCandidate::new(LocalTag::new(tag), origin)
}
fn edges() -> Vec<SlashSpreadEdgeCandidate> {
    vec![
        candidate(9, source(20, SlashSpreadSourceMotion::Moving)),
        candidate(2, bridge()),
    ]
}
#[test]
fn both_modes_and_explicit_policies_keep_actual_candidate_source_and_active_field_orders() {
    for mode in [
        SlashSpreadOpeningMode::Rotation,
        SlashSpreadOpeningMode::Separation,
    ] {
        for policy in [
            SlashSpreadBoundaryPolicy::StraightBridges,
            SlashSpreadBoundaryPolicy::TrimOverlaps,
        ] {
            let intent = intent(mode, policy);
            let raw_catalog = raw_catalog();
            let catalog = raw_catalog.admit(catalogue_limits()).expect("catalogue");
            let bound = intent
                .bind_edge_roles(&catalog, catalogue_limits())
                .expect("operation association");
            let second = if policy == SlashSpreadBoundaryPolicy::StraightBridges {
                bridge()
            } else {
                source(20, SlashSpreadSourceMotion::Stationary)
            };
            let raw = SlashSpreadEdgeManifest::new(
                id(10),
                vec![
                    candidate(9, source(20, SlashSpreadSourceMotion::Moving)),
                    candidate(2, second),
                ],
            );
            let admitted = raw.admit(&bound, limits(2)).expect("complete metadata");
            assert!(std::ptr::eq(admitted.manifest(), &raw));
            assert!(std::ptr::eq(admitted.bound_roles(), &bound));
            assert_eq!(admitted.limits(), limits(2));
            assert_eq!(admitted.limits().candidates(), 2);
            assert_eq!(admitted.limits().catalog(), catalogue_limits());
            assert_eq!(admitted.workspace_entries(), 4);
            assert_eq!(admitted.outputs().len(), 2);
            assert_eq!(raw.operation(), id(10));
            for (view, (index, active_index)) in admitted.outputs().zip([(0, 1), (1, 0)]) {
                assert!(std::ptr::eq(view.admitted(), &admitted));
                assert_eq!(view.index(), index);
                assert!(std::ptr::eq(
                    view.candidate(),
                    raw.candidates().get(index).expect("original candidate")
                ));
                assert!(std::ptr::eq(view.role().catalog(), &catalog));
                assert_eq!(view.role().index(), active_index);
                assert!(std::ptr::eq(
                    view.role().tag(),
                    raw_catalog
                        .active_tags()
                        .get(active_index)
                        .expect("original active field")
                ));
                assert_eq!(*view.role().tag(), view.candidate().tag());
                assert_eq!(
                    view.logical_output(),
                    EdgeRef::new(id(10), view.candidate().tag())
                );
            }
            assert_eq!(
                admitted
                    .outputs()
                    .rev()
                    .map(|v| v.candidate().tag().get())
                    .collect::<Vec<_>>(),
                vec![2, 9]
            );
            assert_eq!(
                raw.candidates().first().expect("source").origin(),
                &source(20, SlashSpreadSourceMotion::Moving)
            );
            assert_eq!(raw.candidates().get(1).expect("second").origin(), &second);
        }
    }
}
#[test]
fn complete_quotas_then_empty_owner_and_original_candidate_causes_retain_whole_inputs() {
    use SlashSpreadEdgeManifestRefusal as R;
    use SlashSpreadSourceMotion::Moving;
    let intent = intent(
        SlashSpreadOpeningMode::Rotation,
        SlashSpreadBoundaryPolicy::TrimOverlaps,
    );
    let raw_catalog = raw_catalog();
    let catalog = raw_catalog.admit(catalogue_limits()).expect("catalogue");
    let bound = intent
        .bind_edge_roles(&catalog, catalogue_limits())
        .expect("same operation");
    let low = |n, d, a, b| {
        SlashSpreadEdgeManifestLimits::new(n, EdgeOutputRoleCatalogLimits::new(d, a, b))
    };
    let rows = [
        (
            999,
            edges(),
            low(1, 0, 0, 0),
            R::Candidates {
                required: 2,
                allowed: 1,
            },
        ),
        (
            999,
            edges(),
            low(2, 2, 0, 0),
            R::Definitions {
                required: 3,
                allowed: 2,
            },
        ),
        (
            999,
            edges(),
            low(2, 3, 1, 0),
            R::Active {
                required: 2,
                allowed: 1,
            },
        ),
        (
            999,
            edges(),
            low(2, 3, 2, 18),
            R::KeyBytes {
                required: 19,
                allowed: 18,
            },
        ),
        (999, vec![], limits(0), R::Empty),
        (999, edges(), limits(2), R::Operation),
        (
            10,
            vec![
                candidate(9, source(20, Moving)),
                candidate(9, source(10, Moving)),
            ],
            limits(2),
            R::DuplicateTag {
                first: 0,
                second: 1,
            },
        ),
        (
            10,
            vec![
                candidate(9, source(20, Moving)),
                candidate(99, source(10, Moving)),
            ],
            limits(2),
            R::UndefinedRole {
                candidate: 1,
                tag: LocalTag::new(99),
            },
        ),
        (
            10,
            vec![
                candidate(9, source(20, Moving)),
                candidate(7, source(10, Moving)),
            ],
            limits(2),
            R::InactiveRole {
                candidate: 1,
                tag: LocalTag::new(7),
            },
        ),
        (
            10,
            vec![candidate(9, source(10, Moving))],
            limits(1),
            R::SelfSource { candidate: 0 },
        ),
        (10, edges(), limits(2), R::BridgePolicy { candidate: 1 }),
    ];
    for (operation, candidates, limits, cause) in rows {
        let raw = SlashSpreadEdgeManifest::new(id(operation), candidates);
        let original = raw.clone();
        let error = raw.admit(&bound, limits).expect_err("whole refusal");
        assert!(std::ptr::eq(error.manifest(), &raw));
        assert!(std::ptr::eq(error.bound_roles(), &bound));
        assert_eq!(error.limits(), limits);
        assert_eq!(error.refusal(), cause);
        assert_eq!(raw, original);
        assert!(format!("{error}").contains("edge manifest admission"));
        assert!(format!("{error:?}").contains("SlashSpreadEdgeManifestError"));
    }
}
#[test]
fn repeated_provenance_is_not_identity_deduplicated_and_absence_never_retires_roles() {
    let intent = intent(
        SlashSpreadOpeningMode::Separation,
        SlashSpreadBoundaryPolicy::StraightBridges,
    );
    let raw_catalog = raw_catalog();
    let original = raw_catalog.clone();
    let catalog = raw_catalog.admit(catalogue_limits()).expect("catalogue");
    let bound = intent
        .bind_edge_roles(&catalog, catalogue_limits())
        .expect("operation");
    let origin = source(20, SlashSpreadSourceMotion::Moving);
    for candidates in [
        vec![candidate(9, origin)],
        vec![candidate(9, origin), candidate(2, origin)],
        vec![candidate(
            9,
            SlashSpreadEdgeOrigin::StraightBridge {
                end: SlashSpreadCutEnd::Upper,
                direction: SlashSpreadBridgeDirection::MovingToStationary,
            },
        )],
    ] {
        let raw = SlashSpreadEdgeManifest::new(id(10), candidates);
        let admitted = raw.admit(&bound, limits(2)).expect("explicit metadata");
        assert_eq!(admitted.outputs().len(), raw.candidates().len());
        assert_eq!(raw_catalog, original);
        assert!(catalog.is_active(LocalTag::new(2)));
        assert!(catalog.is_active(LocalTag::new(9)));
        assert!(catalog.definition(LocalTag::new(7)).is_some());
        assert!(!catalog.is_active(LocalTag::new(7)));
    }
}
