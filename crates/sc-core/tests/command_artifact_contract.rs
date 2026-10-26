//! Concrete artifact/closure requests bind the same exact inputs under distinct command authority.
use sc_core::{
    command::{
        ArtifactPayload, CommandActorClaim, CommandArtifact, CommandAuthority, CommandClass,
        CommandId, CommandQuery, CommandReversibility, CommandSnapshotContext,
        CommandUndoGranularity, DesignRevision, DxfTarget, ExportDxf, QueryPayload,
    },
    ontology::{EntityId as Id, SelectedProfilePin, SelectedProfilePinDefinition},
};

#[test]
fn named_dxf_targets_match_registry_and_never_select_an_alias_or_universal_mode() {
    let registry = include_str!("../../../docs/book/src/spec/interchange-dialects.md")
        .split("## 2. The target registry")
        .nth(1)
        .expect("registry")
        .split("## 3.")
        .next()
        .expect("registry end");
    let rows: Vec<_> = registry
        .lines()
        .filter(|line| line.starts_with("| `dxf-"))
        .collect();
    assert_eq!(rows.len(), DxfTarget::ALL.len());
    for (target, layers, release) in [
        (DxfTarget::AamaNamedR12, "AAMA named", "R12"),
        (DxfTarget::AstmNumberedR13, "ASTM numbered", "R13"),
    ] {
        let row = rows
            .iter()
            .find(|row| row.contains(target.token()))
            .expect("actual target");
        let columns: Vec<_> = row.split('|').map(str::trim).collect();
        assert_eq!(columns.get(2).copied(), Some(layers));
        assert_eq!(columns.get(3).copied(), Some(release));
        assert!(columns
            .get(4)
            .expect("grading column")
            .starts_with("single size"));
        assert_eq!(columns.get(5).copied(), Some("polyline only"));
        assert_eq!(DxfTarget::from_token(target.token()), Some(target));
    }
    for absent in [
        "dxf",
        "DXF-AAMA-NAMED",
        " dxf-aama-named",
        "dxf-astm-num-r13 ",
        "universal",
        "plt",
        "pdf-a0",
    ] {
        assert_eq!(DxfTarget::from_token(absent), None);
    }
}

#[test]
fn artifact_and_inspect_only_closure_keep_identical_instance_profile_result_and_snapshot() {
    let pin = SelectedProfilePin::new(SelectedProfilePinDefinition {
        profile_id: " House/É ".to_owned(),
        schema_version: "02.0".to_owned(),
        content_version: "content+001".to_owned(),
    })
    .expect("explicit opaque pin");
    let revision = DesignRevision::from_counter(u128::MAX).successor();
    let snapshot = CommandSnapshotContext::new(
        Id::from_bits(1),
        CommandActorClaim::Agent {
            identity: Id::from_bits(2),
            principal: Id::from_bits(3),
        },
        revision.clone(),
    );
    for target in DxfTarget::ALL {
        let input =
            ExportDxf::single_instance(Id::from_bits(4), target, pin.clone(), Id::from_bits(5));
        assert_eq!(input.instance(), Id::from_bits(4));
        assert_eq!(input.target(), target);
        assert_eq!(input.selected_profile(), &pin);
        assert_eq!(input.parameters(), Id::from_bits(5));
        let changed =
            ExportDxf::single_instance(Id::from_bits(4), target, pin.clone(), Id::from_bits(6));
        assert_ne!(input, changed);
        let payload = ArtifactPayload::ExportDxf(input);
        let generation = CommandArtifact::new(snapshot.clone(), payload.clone());
        let closure = CommandQuery::new(
            snapshot.clone(),
            QueryPayload::GetUncertaintyClosure {
                artifact: generation.payload().clone(),
            },
        );
        assert_eq!(generation.context(), closure.context());
        assert_eq!(generation.context().expected_revision(), &revision);
        assert_eq!(generation.payload(), &payload);
        assert!(
            matches!(closure.payload(), QueryPayload::GetUncertaintyClosure { artifact }
            if artifact == generation.payload())
        );
        assert_eq!(generation.command_id(), CommandId::ExportDxf);
        assert_eq!(generation.command_id().class(), CommandClass::Artifact);
        assert_eq!(
            generation.command_id().required_authority(),
            CommandAuthority::Generate
        );
        assert_eq!(closure.command_id(), CommandId::GetUncertaintyClosure);
        assert_eq!(closure.command_id().class(), CommandClass::Query);
        assert_eq!(
            closure.command_id().required_authority(),
            CommandAuthority::Inspect
        );
        for id in [generation.command_id(), closure.command_id()] {
            assert_eq!(id.reversibility(), CommandReversibility::NotApplicable);
            assert_eq!(id.undo_granularity(), CommandUndoGranularity::NotApplicable);
        }
    }
}
