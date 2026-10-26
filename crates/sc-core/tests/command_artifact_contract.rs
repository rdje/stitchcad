//! Concrete artifact/closure requests bind the same exact inputs under distinct command authority.
use sc_core::{
    command::{
        ArtifactPayload, CommandActorClaim, CommandArtifact, CommandAuthority, CommandClass,
        CommandId, CommandQuery, CommandReversibility, CommandSnapshotContext,
        CommandUndoGranularity, DesignRevision, DxfGrading, DxfTarget, ExportDxf,
        GradedSizeSelection, GradedSizeSelectionError, QueryPayload,
    },
    ontology::{EntityId as Id, SelectedProfilePin, SelectedProfilePinDefinition},
};
use sc_units::Count;

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

#[test]
fn graded_selection_keeps_authored_order_exact_set_pin_and_first_duplicate_positions() {
    let set = Id::from_bits(20);
    let members = vec![Id::from_bits(32), Id::from_bits(31)];
    let selection = GradedSizeSelection::new(set, Count::new(u32::MAX), members.clone())
        .expect("explicit members");
    assert_eq!(selection.size_set(), set);
    assert_eq!(selection.revision(), Count::new(u32::MAX));
    assert_eq!(selection.members(), members);
    assert_eq!(
        GradedSizeSelection::new(set, Count::new(0), vec![]),
        Err(GradedSizeSelectionError::NoMembers)
    );
    assert_eq!(
        GradedSizeSelection::new(
            set,
            Count::new(0),
            vec![
                Id::from_bits(32),
                Id::from_bits(31),
                Id::from_bits(32),
                Id::from_bits(31)
            ]
        ),
        Err(GradedSizeSelectionError::DuplicateMember {
            member: Id::from_bits(32),
            first: 0,
            duplicate: 2
        })
    );
}

#[test]
fn graded_carriages_fix_registered_pairings_and_closure_retains_base_rules_and_ordered_sizes() {
    let pin = SelectedProfilePin::new(SelectedProfilePinDefinition {
        profile_id: "acme-cmt-pt".to_owned(),
        schema_version: "2.0".to_owned(),
        content_version: "1.3.0".to_owned(),
    })
    .expect("selected profile");
    let sizes = GradedSizeSelection::new(
        Id::from_bits(10),
        Count::new(7),
        vec![Id::from_bits(12), Id::from_bits(11)],
    )
    .expect("explicit range");
    let base = Id::from_bits(20);
    let rules = Id::from_bits(30);
    let parameters = Id::from_bits(40);
    let snapshot = CommandSnapshotContext::new(
        Id::from_bits(1),
        CommandActorClaim::Human(Id::from_bits(2)),
        DesignRevision::initial(),
    );
    let registry = include_str!("../../../docs/book/src/spec/interchange-dialects.md");
    for (input, target, carriage, grading) in [
        (
            ExportDxf::base_plus_rules(base, rules, sizes.clone(), pin.clone(), parameters),
            DxfTarget::AamaNamedR12,
            "base plus rules",
            DxfGrading::BasePlusRules {
                rule_table: rules,
                sizes: sizes.clone(),
            },
        ),
        (
            ExportDxf::embedded_rules(base, rules, sizes.clone(), pin.clone(), parameters),
            DxfTarget::AstmNumberedR13,
            "embedded rules",
            DxfGrading::EmbeddedRules {
                rule_table: rules,
                sizes: sizes.clone(),
            },
        ),
    ] {
        let row = registry
            .lines()
            .find(|row| row.starts_with(&format!("| `{}` |", target.token())))
            .expect("registered row");
        assert!(row.contains(carriage));
        assert_eq!(input.target(), target);
        assert_eq!(input.instance(), base);
        assert_eq!(input.selected_profile(), &pin);
        assert_eq!(input.parameters(), parameters);
        assert_eq!(input.grading(), &grading);
        let generation = CommandArtifact::new(snapshot.clone(), ArtifactPayload::ExportDxf(input));
        let query = CommandQuery::new(
            snapshot.clone(),
            QueryPayload::GetUncertaintyClosure {
                artifact: generation.payload().clone(),
            },
        );
        assert!(
            matches!(query.payload(), QueryPayload::GetUncertaintyClosure { artifact } if artifact == generation.payload())
        );
        assert_eq!(query.context(), generation.context());
        assert_eq!(generation.command_id(), CommandId::ExportDxf);
        assert_eq!(
            query.command_id().required_authority(),
            CommandAuthority::Inspect
        );
    }
    let single = ExportDxf::single_instance(base, DxfTarget::AamaNamedR12, pin, parameters);
    assert_eq!(single.grading(), &DxfGrading::SingleSize);
}

#[test]
fn every_normative_artifact_command_has_a_concrete_constructible_payload(
) -> Result<(), Box<dyn std::error::Error>> {
    use sc_core::command::{
        EvaluationProfile, ExportPdf, ExportPlt, GenerateTechPack, InstanceSelection, PdfLayout,
        PdfOrientation, PdfPrintSettings,
    };
    use sc_units::Length;
    let instance = Id::from_bits(3);
    let pin = SelectedProfilePin::new(SelectedProfilePinDefinition {
        profile_id: "acme-cmt-pt".to_owned(),
        schema_version: "2.0".to_owned(),
        content_version: "1.3.0".to_owned(),
    })?;
    let parameters = Id::from_bits(4);
    let print = PdfPrintSettings::new(
        PdfLayout::A0,
        PdfOrientation::Portrait,
        Length::from_micrometres(100_000)?,
    )?;
    let actual = [
        ArtifactPayload::ExportDxf(ExportDxf::single_instance(
            instance,
            DxfTarget::AamaNamedR12,
            pin.clone(),
            parameters,
        )),
        ArtifactPayload::ExportPlt(ExportPlt::new(instance, pin, parameters)),
        ArtifactPayload::ExportPdf(ExportPdf::new(instance, print, EvaluationProfile::Logical)),
        ArtifactPayload::GenerateTechPack(GenerateTechPack::new(
            InstanceSelection::new(vec![instance])?,
            EvaluationProfile::Logical,
        )),
    ];
    let book = include_str!("../../../docs/book/src/spec/command-layer.md")
        .split("### 1.1 The commands")
        .nth(1)
        .ok_or("command table absent")?
        .split("## 2.")
        .next()
        .ok_or("command table boundary absent")?;
    let expected: Vec<_> = book
        .lines()
        .filter(|line| line.contains("| `artifact` |"))
        .map(|line| line.split('`').nth(1).ok_or("artifact identity absent"))
        .collect::<Result<_, _>>()?;
    let identities: Vec<_> = actual
        .iter()
        .map(|payload| payload.command_id().token())
        .collect();
    assert_eq!(identities, expected);
    assert!(actual
        .iter()
        .all(|payload| payload.command_id().class() == CommandClass::Artifact));
    Ok(())
}
