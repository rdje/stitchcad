//! Scoped native result Piece declaration,not trusted issuance or complete registry freshness.
#![allow(clippy::expect_used)]
#[allow(dead_code)]
#[path = "support/join_banks.rs"]
mod common;
use common::{id, with_custom_banks, BankFixture, SpanFixture};
use sc_core::{construction::*, ontology::*};
fn with_roles(options: BankFixture, run: impl FnOnce(BoundJoinEdgeRoles<'_>)) {
    with_custom_banks(options, |banks| {
        let raw = EdgeOutputRoleCatalog::new(id(1000), vec![], vec![]);
        let limits = EdgeOutputRoleCatalogLimits::new(0, 0, 0);
        let catalog = raw.admit(limits).expect("actual empty metadata");
        let roles = banks
            .bind_edge_roles(&catalog, limits)
            .expect("actual original owner");
        run(roles);
    });
}
#[test]
fn unlisted_and_equal_external_source_values_remain_local_declarations_with_actual_field_borrows() {
    with_roles(BankFixture::basic(), |roles| {
        for wanted in [0, 5000, 8000] {
            let field = id(wanted);
            let limits = JoinPieceIdentityLimits::new(1);
            let declared = roles
                .declare_piece_identity(&field, limits)
                .expect("listed local exclusions only");
            assert!(std::ptr::eq(declared.roles(), &roles));
            assert!(std::ptr::eq(declared.piece_id(), &field));
            assert_eq!(declared.limits(), limits);
            assert_eq!(declared.stop_occurrences(), 1);
            assert_eq!(*declared.piece_id(), field);
            assert!(format!("{declared:?}").contains("DeclaredJoinPieceIdentity"));
        }
        assert_eq!(
            roles
                .banks()
                .sources()
                .population()
                .metadata()
                .splice()
                .source()
                .span()
                .definition()
                .ease
                .declaration()
                .amount,
            EaseAmount::Profile(ProfileParameterRef::new(id(8000)))
        );
    });
}
#[test]
fn complete_stop_quota_precedes_listed_owner_aliases_and_refusals_lend_original_sources() {
    use JoinPieceIdentityRefusal as R;
    with_roles(BankFixture::basic(), |roles| {
        let field = id(1000);
        let error = roles
            .declare_piece_identity(&field, JoinPieceIdentityLimits::new(0))
            .expect_err("whole stops before operation field");
        assert_eq!(
            error.refusal(),
            R::Stops {
                required: 1,
                allowed: 0
            }
        );
        assert!(error.refused_source().is_none() && error.refused_stop().is_none());
        for (wanted, cause) in [
            (1000, R::Operation),
            (100, R::InventoryPiece { index: 0 }),
            (101, R::InventoryPiece { index: 1 }),
            (300, R::Graph),
            (301, R::Span),
            (201, R::Copy { index: 0 }),
            (200, R::Copy { index: 1 }),
            (
                1,
                R::SourceOperation {
                    side: SeamSideId::A,
                    location: JoinSourceOccurrenceLocation::Boundary(0),
                },
            ),
            (
                401,
                R::Stop {
                    origin: JoinIdentityStopOrigin::SelectedSpan,
                    index: 0,
                },
            ),
        ] {
            let field = id(wanted);
            let limits = JoinPieceIdentityLimits::new(1);
            let error = roles
                .declare_piece_identity(&field, limits)
                .expect_err("actual local alias");
            assert!(std::ptr::eq(error.roles(), &roles));
            assert!(std::ptr::eq(error.piece_id(), &field));
            assert_eq!(error.limits(), limits);
            assert_eq!(error.refusal(), cause);
            if wanted == 1 {
                let original = &roles
                    .banks()
                    .sources()
                    .piece(SeamSideId::A)
                    .definition()
                    .boundary
                    .first()
                    .expect("original field")
                    .edge;
                assert!(std::ptr::eq(
                    error.refused_source().expect("source cause"),
                    original
                ));
            } else {
                assert!(error.refused_source().is_none());
            }
            if wanted == 401 {
                let stop = roles
                    .banks()
                    .sources()
                    .population()
                    .metadata()
                    .splice()
                    .source()
                    .span()
                    .definition()
                    .stops
                    .first()
                    .expect("original stop");
                assert!(std::ptr::eq(
                    error.refused_stop().expect("stop cause"),
                    stop
                ));
            } else {
                assert!(error.refused_stop().is_none());
            }
            assert!(error.to_string().starts_with("join result Piece"));
            assert!(format!("{error:?}").contains("JoinPieceIdentityError"));
        }
    });
}
#[test]
fn unrelated_inventory_copy_and_material_owners_and_requested_companion_cannot_be_the_result_piece()
{
    use JoinPieceIdentityRefusal as R;
    let options = BankFixture {
        source_materials: Some([id(600), id(601)]),
        result_material: Some(id(700)),
        extra_piece: true,
        ..BankFixture::basic()
    };
    with_roles(options, |roles| {
        for (wanted, cause) in [
            (102, R::InventoryPiece { index: 2 }),
            (202, R::Copy { index: 2 }),
            (600, R::SourceMaterial { index: 0 }),
            (601, R::SourceMaterial { index: 1 }),
            (602, R::SourceMaterial { index: 2 }),
            (700, R::ResultMaterial),
        ] {
            assert_eq!(
                roles
                    .declare_piece_identity(&id(wanted), JoinPieceIdentityLimits::new(1))
                    .expect_err("whole provided context alias")
                    .refusal(),
                cause
            );
        }
    });
    let options = BankFixture {
        mirroring: Mirroring::PairMember {
            handedness: Handedness::Left,
            companion: id(500),
        },
        ..BankFixture::basic()
    };
    with_roles(options, |roles| {
        assert_eq!(
            roles
                .declare_piece_identity(&id(500), JoinPieceIdentityLimits::new(1))
                .expect_err("self companion")
                .refusal(),
            R::SelfCompanion
        );
        let field = id(501);
        assert_eq!(
            *roles
                .declare_piece_identity(&field, JoinPieceIdentityLimits::new(1))
                .expect("companion truth remains later")
                .piece_id(),
            field
        );
    });
}
#[test]
fn retained_ease_span_and_all_original_stops_keep_their_actual_sources_and_count_duplicate_occurrences(
) {
    use JoinPieceIdentityRefusal as R;
    for (form, count, origin) in [
        (
            SpanFixture::RetainedWhole,
            2,
            JoinIdentityStopOrigin::SelectedSpan,
        ),
        (
            SpanFixture::RetainedTail,
            1,
            JoinIdentityStopOrigin::RetainedEaseSource,
        ),
    ] {
        with_roles(
            BankFixture {
                span: form,
                ..BankFixture::basic()
            },
            |roles| {
                let field = id(5000);
                assert_eq!(
                    roles
                        .declare_piece_identity(&field, JoinPieceIdentityLimits::new(count - 1))
                        .expect_err("complete selected plus source stops")
                        .refusal(),
                    R::Stops {
                        required: count,
                        allowed: count - 1
                    }
                );
                let limits = JoinPieceIdentityLimits::new(count);
                assert_eq!(
                    roles
                        .declare_piece_identity(&field, limits)
                        .expect("local id")
                        .stop_occurrences(),
                    count
                );
                assert_eq!(
                    roles
                        .declare_piece_identity(&id(302), limits)
                        .expect_err("selected span alias")
                        .refusal(),
                    R::Span
                );
                assert_eq!(
                    roles
                        .declare_piece_identity(&id(301), limits)
                        .expect_err("retained original span alias")
                        .refusal(),
                    R::RetainedSpan
                );
                let landmark = id(401);
                let error = roles
                    .declare_piece_identity(&landmark, limits)
                    .expect_err("actual original landmark alias");
                assert_eq!(error.refusal(), R::Stop { origin, index: 0 });
                let span = roles
                    .banks()
                    .sources()
                    .population()
                    .metadata()
                    .splice()
                    .source()
                    .span();
                let expected = match origin {
                    JoinIdentityStopOrigin::SelectedSpan => {
                        span.definition().stops.first().expect("selected stop")
                    }
                    JoinIdentityStopOrigin::RetainedEaseSource => span
                        .definition()
                        .ease
                        .portion()
                        .expect("portion")
                        .source()
                        .stops()
                        .first()
                        .expect("original source stop"),
                };
                assert!(std::ptr::eq(
                    error.refused_stop().expect("actual refused stop"),
                    expected
                ));
            },
        );
    }
}
