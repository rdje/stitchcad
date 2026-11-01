//! Explicit proposed construction occurrences,not complete original semantic reconciliation.
#![allow(clippy::expect_used)]
#[allow(dead_code)]
#[path = "support/join_edges.rs"]
mod common;
use common::{id, with_connectivity};
use sc_core::{construction::*, ontology::*};
fn field(tag: u32, direction: Direction) -> JoinConstructionUse {
    JoinConstructionUse::new(LocalTag::new(tag), direction)
}
#[test]
fn ordered_repeated_cut_and_noncut_roles_lend_actual_fields_and_complete_source_direction_parents()
{
    use Direction::{Original as O, Reversed as R};
    with_connectivity(true, |path| {
        let raw = JoinConstructionManifest::new(
            id(1000),
            vec![field(4, O), field(9, R), field(4, R), field(6, O)],
        );
        let snapshot = raw.clone();
        let limits = JoinConstructionUseLimits::new(10, 4, 4);
        let uses = raw
            .admit(&path, limits)
            .expect("explicit proposed occurrences");
        assert_eq!(raw.operation(), id(1000));
        assert!(std::ptr::eq(uses.manifest(), &raw));
        assert!(std::ptr::eq(uses.connectivity(), &path));
        assert_eq!(uses.limits(), limits);
        assert_eq!(
            (limits.outputs(), limits.uses(), limits.workspace_entries()),
            (10, 4, 4)
        );
        assert_eq!(uses.use_count(), 4);
        assert_eq!(uses.workspace_entries(), 4);
        assert_eq!(
            uses.uses().map(|u| *u.declared().tag()).collect::<Vec<_>>(),
            [4, 9, 4, 6].map(LocalTag::new)
        );
        assert_eq!(
            uses.uses().rev().map(|u| u.index()).collect::<Vec<_>>(),
            [3, 2, 1, 0]
        );
        for placed in uses.uses() {
            let original = raw
                .uses()
                .get(placed.index())
                .expect("actual original occurrence");
            assert!(std::ptr::eq(placed.context(), &uses));
            assert!(std::ptr::eq(placed.declared(), original));
            assert!(std::ptr::eq(placed.direction(), original.direction()));
            assert!(std::ptr::eq(
                placed.output().context(),
                path.contours().directions()
            ));
            assert_eq!(
                placed.logical_output(),
                EdgeRef::new(id(1000), *original.tag())
            );
            let actual = path
                .contours()
                .directions()
                .assignment(*original.tag())
                .expect("actual directional output");
            assert!(std::ptr::eq(
                placed.output().assignment(),
                actual.assignment()
            ));
            assert!(std::ptr::eq(
                placed.output().source().candidate(),
                actual.source().candidate()
            ));
        }
        assert_eq!(
            uses.uses_for(LocalTag::new(4))
                .map(|u| u.index())
                .collect::<Vec<_>>(),
            [0, 2]
        );
        assert_eq!(
            uses.uses_for(LocalTag::new(4))
                .rev()
                .map(|u| u.index())
                .collect::<Vec<_>>(),
            [2, 0]
        );
        assert_eq!(uses.uses_for(LocalTag::new(99)).count(), 0);
        assert_eq!(raw, snapshot);
        assert!(format!("{uses:?}").contains("DeclaredJoinConstructionUses"));
    });
}
#[test]
fn full_n_u_and_association_quota_precedes_owner_and_all_original_unknown_fields() {
    use JoinConstructionUseRefusal as R;
    with_connectivity(true, |path| {
        let raw = JoinConstructionManifest::new(id(1001), vec![field(99, Direction::Original); 4]);
        let snapshot = raw.clone();
        for (limits, cause) in [
            (
                JoinConstructionUseLimits::new(9, 0, 0),
                R::Outputs {
                    required: 10,
                    allowed: 9,
                },
            ),
            (
                JoinConstructionUseLimits::new(10, 3, 0),
                R::Uses {
                    required: 4,
                    allowed: 3,
                },
            ),
            (
                JoinConstructionUseLimits::new(10, 4, 3),
                R::Workspace {
                    required: 4,
                    allowed: 3,
                },
            ),
            (JoinConstructionUseLimits::new(10, 4, 4), R::Operation),
        ] {
            let error = raw
                .admit(&path, limits)
                .expect_err("complete resources/owner before fields");
            assert!(std::ptr::eq(error.manifest(), &raw));
            assert!(std::ptr::eq(error.connectivity(), &path));
            assert_eq!(error.limits(), limits);
            assert_eq!(error.refusal(), cause);
            assert!(error.refused_use().is_none());
            assert!(error
                .to_string()
                .starts_with("join proposed construction uses:"));
            assert!(format!("{error:?}").contains("JoinConstructionUseError"));
        }
        assert_eq!(raw, snapshot);
    });
}
#[test]
fn active_but_absent_inactive_and_undefined_roles_refuse_the_actual_first_use_without_prefixes() {
    use JoinConstructionUseRefusal as R;
    with_connectivity(false, |path| {
        assert!(path
            .contours()
            .directions()
            .coverage()
            .edges()
            .identity()
            .roles()
            .catalog()
            .active_role(LocalTag::new(13))
            .is_some());
        for tag in [13, 7, 99] {
            let raw = JoinConstructionManifest::new(
                id(1000),
                vec![
                    field(9, Direction::Original),
                    field(tag, Direction::Reversed),
                    field(2, Direction::Original),
                ],
            );
            let snapshot = raw.clone();
            let limits = JoinConstructionUseLimits::new(2, 3, 3);
            let error = raw
                .admit(&path, limits)
                .expect_err("actual candidate membership required");
            assert_eq!(
                error.refusal(),
                R::UnknownOutput {
                    index: 1,
                    tag: LocalTag::new(tag)
                }
            );
            assert_eq!(error.limits(), limits);
            assert!(std::ptr::eq(error.manifest(), &raw));
            assert!(std::ptr::eq(error.connectivity(), &path));
            assert!(std::ptr::eq(
                error
                    .refused_use()
                    .expect("actual original first refused use"),
                raw.uses().get(1).expect("original field")
            ));
            assert_eq!(raw, snapshot);
        }
    });
}
#[test]
fn explicit_empty_proposal_keeps_all_original_fields_and_does_not_authorize_source_omission() {
    with_connectivity(true, |path| {
        let raw = JoinConstructionManifest::new(id(1000), vec![]);
        let uses = raw
            .admit(&path, JoinConstructionUseLimits::new(10, 0, 0))
            .expect("explicit empty proposal metadata");
        assert_eq!(uses.use_count(), 0);
        assert_eq!(uses.uses().len(), 0);
        assert_eq!(uses.workspace_entries(), 0);
        let sources = uses
            .connectivity()
            .contours()
            .directions()
            .coverage()
            .edges()
            .identity()
            .roles()
            .banks()
            .sources();
        assert_eq!(
            sources
                .piece(SeamSideId::A)
                .definition()
                .construction_lines
                .len(),
            2
        );
        assert_eq!(
            sources
                .piece(SeamSideId::B)
                .definition()
                .construction_lines
                .len(),
            2
        );
        assert_eq!(uses.connectivity().contours().use_count(), 4);
        assert_eq!(uses.connectivity().connection_count(), 4);
        // Complete source-field/fold/consumer reconciliation is a separately owned mandatory stage.
    });
}
