//! Exact normalized owner/mixed position and original raw native semantic field loans.
use super::{
    limits, native_geometry::same_input, native_semantic::with_native, root_with_recipe, scope,
    Source, TestResult,
};
use sc_core::construction::*;
fn same_field(a: DraftingNativeSemanticField<'_>, b: DraftingNativeSemanticField<'_>) -> bool {
    match (a, b) {
        (
            DraftingNativeSemanticField::Target {
                role: a,
                reference: ar,
            },
            DraftingNativeSemanticField::Target {
                role: b,
                reference: br,
            },
        ) => a == b && core::ptr::eq(ar, br),
        (
            DraftingNativeSemanticField::NotchProfile {
                role: a,
                reference: ar,
            },
            DraftingNativeSemanticField::NotchProfile {
                role: b,
                reference: br,
            },
        ) => a == b && core::ptr::eq(ar, br),
        (
            DraftingNativeSemanticField::WalkNotchProfile {
                pair_index: ai,
                side: as_,
                role: a,
                reference: ar,
            },
            DraftingNativeSemanticField::WalkNotchProfile {
                pair_index: bi,
                side: bs,
                role: b,
                reference: br,
            },
        ) => ai == bi && as_ == bs && a == b && core::ptr::eq(ar, br),
        _ => false,
    }
}
#[test]
fn mixed_native_fields_keep_exact_program_step_and_every_original_raw_loan() -> TestResult {
    for rotated in [false, true] {
        with_native(rotated, |native, _| {
            let root = root_with_recipe("let before:length=1 mm\nlet after:length=2 mm")?;
            let source = Source::new(&root);
            let context = scope(&root)?
                .bind_drafting_context(&source, limits(10000, 10000, 10000)?)
                .map_err(|e| e.to_string())?;
            let mut steps = vec![DraftingProgramStep::Scalar { ordinal: 1 }];
            for (index, input) in native.iter().enumerate() {
                if index == 8 {
                    steps.push(DraftingProgramStep::Scalar { ordinal: 2 });
                }
                steps.push(DraftingProgramStep::Drafting(*input));
            }
            // The same actual Grainline loan appears at another mixed position.
            let grainline = *native.get(8).ok_or("original Grainline input")?;
            steps.push(DraftingProgramStep::Drafting(grainline));
            let raw = DraftingProgramInputs::new(context, steps.clone());
            let other = DraftingProgramInputs::new(context, steps);
            let policy =
                DraftingProgramLimits::new(limits(10000, 10000, 10000)?, 1000, 1000, 10000);
            let admitted = raw.admit(&policy).map_err(|e| e.to_string())?;
            let program = admitted.normalized().map_err(|e| e.to_string())?;
            let other_admitted = other.admit(&policy).map_err(|e| e.to_string())?;
            let other_program = other_admitted.normalized().map_err(|e| e.to_string())?;
            source.trace.set(0);
            use DraftingNativeSelectionField as N;
            use DraftingNativeSemanticTargetRole as R;
            use DraftingNotchProfileRole as P;
            let mut expected_locations = vec![
                (6, N::Target(R::OffsetAllowance)),
                (7, N::Target(R::SlashPiece)),
                (8, N::Target(R::Dart)),
                (9, N::Target(R::NotchPiece)),
            ];
            let roles = [
                P::Style,
                P::SampleDepth,
                P::SampleWidth,
                P::ProductionDepth,
                P::ProductionWidth,
                P::Encoding,
            ];
            expected_locations.extend(roles.map(|role| (9, N::NotchProfile(role))));
            expected_locations
                .extend([(11, N::Target(R::Grainline)), (12, N::Target(R::WalkSpan))]);
            for pair_index in 0..2 {
                for side in [
                    sc_core::ontology::SeamSideId::A,
                    sc_core::ontology::SeamSideId::B,
                ] {
                    expected_locations.extend(roles.map(|role| {
                        (
                            12,
                            N::WalkNotchProfile {
                                pair_index,
                                side,
                                role,
                            },
                        )
                    }));
                }
            }
            expected_locations.extend([
                (13, N::Target(R::TruingSpan)),
                (13, N::Target(R::TruingPiece)),
                (13, N::Target(R::TruingFitTolerance)),
                (14, N::Target(R::MirrorPiece)),
                (15, N::Target(R::JoinSpan)),
                (15, N::Target(R::JoinPieceA)),
                (15, N::Target(R::JoinPieceB)),
                (16, N::Target(R::Grainline)),
            ]);
            let locations: Vec<_> = program
                .native_semantic_dependencies()
                .map(|use_| use_.selection_location())
                .collect();
            assert_eq!(
                locations,
                expected_locations
                    .into_iter()
                    .map(
                        |(position, field)| DraftingSemanticSelectionLocation::Native {
                            position,
                            field
                        }
                    )
                    .collect::<Vec<_>>()
            );
            let mut uses = program.native_semantic_dependencies();
            let mut count = 0;
            for (input, (position, expected_count)) in native.iter().copied().zip([
                (2, 0),
                (3, 0),
                (4, 0),
                (5, 0),
                (6, 1),
                (7, 1),
                (8, 1),
                (9, 7),
                (11, 1),
                (12, 25),
                (13, 3),
                (14, 1),
                (15, 3),
            ]) {
                let mut actual = 0;
                for original in input.semantic_dependencies() {
                    let use_ = uses.next().ok_or("complete mixed native fields")?;
                    let copied = use_;
                    assert!(core::ptr::eq(copied.program(), &program));
                    assert!(!core::ptr::eq(copied.program(), &other_program));
                    assert_eq!(use_.position(), position);
                    assert_eq!(use_.step().position(), position);
                    assert!(
                        matches!(use_.step().kind(),DraftingProgramStepKind::Drafting(actual) if same_input(actual.input(),input))
                    );
                    assert!(same_input(use_.native().input(), input));
                    assert!(same_field(use_.native().field(), original.field()));
                    actual += 1;
                    count += 1;
                }
                assert_eq!(actual, expected_count);
            }
            assert_eq!(count, 43);
            let repeated = uses.next().ok_or("repeated original Grainline loan")?;
            assert_eq!(repeated.position(), 16);
            assert!(same_input(repeated.native().input(), grainline));
            assert_eq!(
                repeated.selection_location(),
                DraftingSemanticSelectionLocation::Native {
                    position: 16,
                    field: N::Target(R::Grainline),
                }
            );
            assert!(uses.next().is_none());
            assert!(uses.next().is_none());
            assert_eq!(other_program.native_semantic_dependencies().count(), 44);
            assert_eq!(source.trace.get(), 0);
            Ok(())
        })?;
    }
    Ok(())
}
#[test]
fn actual_scalar_only_program_has_no_native_semantic_fields() -> TestResult {
    let root = root_with_recipe("let only:length=1 mm/0")?;
    let source = Source::new(&root);
    let context = scope(&root)?
        .bind_drafting_context(&source, limits(10000, 10000, 10000)?)
        .map_err(|e| e.to_string())?;
    let raw = DraftingProgramInputs::new(context, vec![DraftingProgramStep::Scalar { ordinal: 1 }]);
    let policy = DraftingProgramLimits::new(limits(10000, 10000, 10000)?, 100, 100, 1000);
    let admitted = raw.admit(&policy).map_err(|e| e.to_string())?;
    let program = admitted.normalized().map_err(|e| e.to_string())?;
    source.trace.set(0);
    let mut uses = program.native_semantic_dependencies();
    assert!(uses.next().is_none());
    assert!(uses.next().is_none());
    assert_eq!(source.trace.get(), 0);
    Ok(())
}
