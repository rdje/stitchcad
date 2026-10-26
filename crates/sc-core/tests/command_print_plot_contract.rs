//! Authored print choices and exact profile-result claims remain concrete artifact-specific inputs.
use sc_core::{
    command::{
        ArtifactPayload, CommandActorClaim, CommandArtifact, CommandAuthority, CommandClass,
        CommandId, CommandQuery, CommandSnapshotContext, DesignRevision, EvaluationProfile,
        ExportPdf, ExportPlt, PdfLayout, PdfOrientation, PdfPrintSettings, PdfPrintSettingsError,
        PdfTileOrder, PdfTilePaper, QueryPayload,
    },
    ontology::{EntityId as Id, SelectedProfilePin, SelectedProfilePinDefinition},
};
use sc_units::Length;

// Invalid authored fixture data must fail this test; this helper is never production code.
#[allow(clippy::expect_used)]
fn length(um: i64) -> Length {
    Length::from_micrometres(um).expect("admitted authored length")
}

#[test]
fn every_admitted_paper_orientation_and_tile_order_retains_exact_authored_print_choices() {
    let mut layouts = vec![PdfLayout::A0];
    for paper in [PdfTilePaper::A3, PdfTilePaper::A4] {
        for order in [PdfTileOrder::RowMajor, PdfTileOrder::ColumnMajor] {
            layouts.push(PdfLayout::Tiled {
                paper,
                order,
                overlap: length(10_001),
            });
        }
    }
    for layout in layouts {
        for orientation in [PdfOrientation::Portrait, PdfOrientation::Landscape] {
            let square = length(100_001);
            let settings = PdfPrintSettings::new(layout, orientation, square)
                .expect("explicit positive print choices");
            assert_eq!(settings.layout(), layout);
            assert_eq!(settings.orientation(), orientation);
            assert_eq!(settings.scale_square(), square);
        }
    }
    assert!(PdfPrintSettings::new(
        PdfLayout::Tiled {
            paper: PdfTilePaper::A4,
            overlap: length(1),
            order: PdfTileOrder::RowMajor,
        },
        PdfOrientation::Portrait,
        length(1)
    )
    .is_ok());
}

#[test]
fn zero_negative_square_and_overlap_refuse_actual_values_without_clamping_or_arbitrary_minimum() {
    for invalid in [Length::ZERO, length(-1)] {
        assert_eq!(
            PdfPrintSettings::new(PdfLayout::A0, PdfOrientation::Portrait, invalid),
            Err(PdfPrintSettingsError::NonPositiveScaleSquare(invalid))
        );
        for paper in [PdfTilePaper::A3, PdfTilePaper::A4] {
            let layout = PdfLayout::Tiled {
                paper,
                order: PdfTileOrder::ColumnMajor,
                overlap: invalid,
            };
            assert_eq!(
                PdfPrintSettings::new(layout, PdfOrientation::Landscape, length(100_000)),
                Err(PdfPrintSettingsError::NonPositiveOverlap(invalid))
            );
            assert_eq!(
                PdfPrintSettings::new(layout, PdfOrientation::Landscape, invalid),
                Err(PdfPrintSettingsError::NonPositiveScaleSquare(invalid))
            );
        }
    }
}

#[test]
fn plot_profile_and_logical_or_factory_print_inputs_survive_unchanged_in_inspect_closures() {
    let pin = SelectedProfilePin::new(SelectedProfilePinDefinition {
        profile_id: " House/É ".to_owned(),
        schema_version: "02.0".to_owned(),
        content_version: "release+001".to_owned(),
    })
    .expect("exact selected labels");
    let source = Id::from_bits(3);
    let result = Id::from_bits(4);
    let plot = ExportPlt::new(source, pin.clone(), result);
    assert_eq!(plot.instance(), source);
    assert_eq!(plot.selected_profile(), &pin);
    assert_eq!(plot.parameters(), result);
    assert_registered(plot.target_token());
    assert_artifact(ArtifactPayload::ExportPlt(plot), CommandId::ExportPlt);
    for profile in [
        EvaluationProfile::Logical,
        EvaluationProfile::FactoryResolved {
            selected: pin,
            parameters: result,
        },
    ] {
        let settings = PdfPrintSettings::new(
            PdfLayout::Tiled {
                paper: PdfTilePaper::A3,
                overlap: length(15_000),
                order: PdfTileOrder::RowMajor,
            },
            PdfOrientation::Landscape,
            length(100_000),
        )
        .expect("explicit tiled print settings");
        let print = ExportPdf::new(source, settings, profile.clone());
        assert_eq!(print.instance(), source);
        assert_eq!(print.settings(), settings);
        assert_eq!(print.profile(), &profile);
        assert_registered(print.target_token());
        assert_artifact(ArtifactPayload::ExportPdf(print), CommandId::ExportPdf);
    }
}

// Missing normative registry markers must fail the contract rather than select another section.
#[allow(clippy::expect_used)]
fn assert_registered(token: &str) {
    let registry = include_str!("../../../docs/book/src/spec/interchange-dialects.md")
        .split("## 2. The target registry")
        .nth(1)
        .expect("actual registry")
        .split("## 3.")
        .next()
        .expect("registry boundary");
    assert!(registry
        .lines()
        .any(|row| row.starts_with(&format!("| `{token}` |"))));
}
fn assert_artifact(payload: ArtifactPayload, identity: CommandId) {
    let revision = DesignRevision::from_counter(u128::MAX).successor();
    let snapshot = CommandSnapshotContext::new(
        Id::from_bits(1),
        CommandActorClaim::Agent {
            identity: Id::from_bits(7),
            principal: Id::from_bits(2),
        },
        revision.clone(),
    );
    let generation = CommandArtifact::new(snapshot.clone(), payload.clone());
    let closure = CommandQuery::new(
        snapshot.clone(),
        QueryPayload::GetUncertaintyClosure {
            artifact: payload.clone(),
        },
    );
    assert_eq!(generation.context(), &snapshot);
    assert_eq!(closure.context().expected_revision(), &revision);
    assert_eq!(generation.payload(), &payload);
    assert!(
        matches!(closure.payload(), QueryPayload::GetUncertaintyClosure { artifact } if artifact == generation.payload())
    );
    assert_eq!(generation.command_id(), identity);
    assert_eq!(identity.class(), CommandClass::Artifact);
    assert_eq!(identity.required_authority(), CommandAuthority::Generate);
    assert_eq!(closure.command_id(), CommandId::GetUncertaintyClosure);
    assert_eq!(
        closure.command_id().required_authority(),
        CommandAuthority::Inspect
    );
}
