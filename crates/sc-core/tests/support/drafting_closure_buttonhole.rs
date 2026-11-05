//! Original Closure/Root operation association; no physical length or placement certificate.
use super::{garment_source, limits, scope, TestResult};
use sc_core::{
    construction::*,
    design::*,
    ontology::*,
    recipe::*,
    value::CanonicalInputs,
    value::{LogicalInputKind, LogicalInputTarget},
};
use std::error::Error;
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn buttonhole(button: NotionSize) -> TestResult<SemanticOperation> {
    buttonhole_at(id(81), button)
}
fn buttonhole_at(operation: EntityId, button: NotionSize) -> TestResult<SemanticOperation> {
    Ok(SemanticOperation::Buttonhole(ButtonholeOperation::new(
        ButtonholeOperationDefinition {
            id: operation,
            button,
            policy: id(82),
            inputs: vec![],
        },
        ButtonholeOperationLimits::new(0, 0, 0),
    )?))
}
fn garment(
    button: NotionSize,
    operations: Vec<SemanticOperation>,
    paired: bool,
) -> TestResult<DesignGarment<super::EmptyHost>> {
    let kind = if paired {
        ClosureKind::ButtonAndButtonhole {
            button,
            hole: ButtonholeDerivation { operation: id(81) },
        }
    } else {
        ClosureKind::HookAndBar {
            hook: button,
            bar: button,
        }
    };
    garment_source::garment_with_semantic_declarations(
        CanonicalInputs::new(vec![], vec![])?,
        vec![],
        vec![kind],
        PocketOpening::Profile(ProfileParameterRef::new(id(700))),
        SemanticRecipe::new(operations, SemanticRecipeLimits::new(10, 100))?,
    )
}
#[test]
fn closure_buttonhole_binds_exact_original_root_member_and_canonical_origin() -> TestResult {
    for button in [
        NotionSize::Declaration(id(80)),
        NotionSize::Profile(ProfileParameterRef::new(id(80))),
    ] {
        let garment = garment_source::garment_with_semantic_declarations(
            CanonicalInputs::new(vec![], vec![])?,
            vec![],
            vec![
                ClosureKind::HookAndBar {
                    hook: button,
                    bar: button,
                },
                ClosureKind::ButtonAndButtonhole {
                    button,
                    hole: ButtonholeDerivation { operation: id(81) },
                },
            ],
            PocketOpening::Profile(ProfileParameterRef::new(id(700))),
            SemanticRecipe::new(
                vec![
                    buttonhole_at(id(79), NotionSize::Declaration(id(88)))?,
                    buttonhole(button)?,
                ],
                SemanticRecipeLimits::new(2, 4),
            )?,
        )?;
        let provider = garment_source::Provider::new(garment.inputs(), &garment, false, false);
        let content = limits(100000, 100000, 100000)?;
        let context = scope(garment.inputs())?
            .bind_drafting_context(&provider, content)
            .map_err(|e| e.to_string())?;
        let admitted = context
            .admit_garment_source(content)
            .map_err(|e| e.to_string())?;
        let selected = DraftingClosureButtonholeLimits::new(content, usize::MAX);
        provider.base.trace.set(0);
        let proof = admitted
            .bind_closure_buttonhole(id(20012), selected)
            .map_err(|e| e.to_string())?;
        assert!(std::ptr::eq(proof.parent(), &admitted));
        assert!(std::ptr::eq(
            proof.closure(),
            &garment.closures().closures()[1]
        ));
        assert_eq!(proof.closure_index(), 1);
        let original = garment.closures().closures()[1]
            .buttonhole_length_source()
            .ok_or("button pair")?;
        assert!(std::ptr::eq(
            proof.source().button_size(),
            original.button_size()
        ));
        assert!(std::ptr::eq(
            proof.source().derivation(),
            original.derivation()
        ));
        let bound = proof.operation();
        assert!(std::ptr::eq(
            bound.recipe(),
            garment.inputs().semantic_recipe()
        ));
        assert!(std::ptr::eq(
            bound.operation(),
            &garment.inputs().semantic_recipe().operations()[1]
        ));
        assert_eq!(bound.index(), 1);
        assert_eq!(
            proof.source().validation(),
            ButtonholeValidation::DeferredToG3
        );
        let mut meter = DesignResourceMeter::new(content);
        admitted.charge_resources(&mut meter)?;
        let expected = meter.finish()?;
        assert_eq!(proof.demand(), expected);
        let e = expected.collection_entries();
        let bound = (e + 8) * (e + 8);
        assert_eq!(proof.shape().matching_inspection_bound(), bound);
        assert_eq!(proof.shape().held_collection_entries(), e);
        assert_eq!(proof.limits(), selected);
        assert_eq!(
            provider.base.trace.get(),
            22,
            "two fresh content walks, no value/current/source query"
        );
        let exact = DraftingClosureButtonholeLimits::new(content, bound);
        assert!(admitted.bind_closure_buttonhole(id(20012), exact).is_ok());
        let error = admitted
            .bind_closure_buttonhole(
                id(20012),
                DraftingClosureButtonholeLimits::new(content, bound - 1),
            )
            .err()
            .ok_or("matching shortage")?;
        assert!(
            matches!(error.refusal(),DraftingClosureButtonholeRefusal::MatchingInspections {allowed,required} if *allowed==bound-1 && *required==bound)
        );
    }
    Ok(())
}
#[test]
fn closure_buttonhole_preserves_first_original_relation_and_fresh_resource_causes() -> TestResult {
    let button = NotionSize::Declaration(id(80));
    for mode in 0..6 {
        let operation = match mode {
            0 => vec![],
            1 => vec![SemanticOperation::LogicalDerivation(
                LogicalDerivationOperation::new(
                    LogicalDerivationDefinition {
                        id: id(81),
                        output: LogicalInputTarget {
                            declaration: id(83),
                            kind: LogicalInputKind::PocketOpening,
                        },
                        rule: id(82),
                        inputs: vec![],
                    },
                    LogicalDerivationLimits::new(0, 0, 0),
                )?,
            )],
            2 => vec![buttonhole(NotionSize::Declaration(id(84)))?],
            3 => vec![buttonhole(NotionSize::Profile(ProfileParameterRef::new(
                id(80),
            )))?],
            _ => vec![buttonhole(button)?],
        };
        let garment = garment(button, operation, mode != 4)?;
        let provider = garment_source::Provider::new(garment.inputs(), &garment, false, false);
        let content = limits(100000, 100000, 100000)?;
        let context = scope(garment.inputs())?
            .bind_drafting_context(&provider, content)
            .map_err(|e| e.to_string())?;
        let admitted = context
            .admit_garment_source(content)
            .map_err(|e| e.to_string())?;
        provider.base.trace.set(0);
        let requested = if mode == 5 { id(20099) } else { id(20002) };
        let selected = DraftingClosureButtonholeLimits::new(content, usize::MAX);
        let error = admitted
            .bind_closure_buttonhole(requested, selected)
            .err()
            .ok_or("relation refusal")?;
        assert!(std::ptr::eq(error.parent(), &admitted));
        assert_eq!(error.closure(), requested);
        assert_eq!(error.limits(), selected);
        assert!(error.demand().is_some());
        assert!(error.shape().is_some());
        match (mode, error.refusal()) {
            (
                0,
                DraftingClosureButtonholeRefusal::Operation(
                    SemanticOperationLookupError::Missing {
                        operation,
                        expected,
                    },
                ),
            ) => {
                assert_eq!(*operation, id(81));
                assert_eq!(*expected, SemanticOperationKind::Buttonhole);
            }
            (
                1,
                DraftingClosureButtonholeRefusal::Operation(SemanticOperationLookupError::Kind {
                    actual,
                    expected,
                }),
            ) => {
                assert!(std::ptr::eq(
                    *actual,
                    &garment.inputs().semantic_recipe().operations()[0]
                ));
                assert_eq!(*expected, SemanticOperationKind::Buttonhole);
            }
            (2 | 3, DraftingClosureButtonholeRefusal::ButtonOrigin { source, operation }) => {
                assert_eq!(*source.button_size(), button);
                assert!(std::ptr::eq(
                    operation.operation(),
                    &garment.inputs().semantic_recipe().operations()[0]
                ));
            }
            (4, DraftingClosureButtonholeRefusal::NotButtonhole { index, closure }) => {
                assert_eq!(*index, 0);
                assert!(std::ptr::eq(*closure, &garment.closures().closures()[0]));
            }
            (5, DraftingClosureButtonholeRefusal::MissingClosure) => {}
            _ => return Err(format!("unexpected mode {mode}: {error:?}").into()),
        }
        assert_eq!(provider.base.trace.get(), 2, "fresh resource walk only");
        // Resource then matching refusals win over every malformed relation above.
        let short = DraftingClosureButtonholeLimits::new(limits(1, 1, 1)?, 0);
        let error = admitted
            .bind_closure_buttonhole(requested, short)
            .err()
            .ok_or("fresh content shortage")?;
        assert!(matches!(
            error.refusal(),
            DraftingClosureButtonholeRefusal::Resources(_)
        ));
        assert!(error
            .source()
            .is_some_and(|c| c.is::<DesignResourceLimitError>()));
        assert_eq!(error.demand(), None);
        assert_eq!(error.shape(), None);
        let error = admitted
            .bind_closure_buttonhole(requested, DraftingClosureButtonholeLimits::new(content, 0))
            .err()
            .ok_or("matching before lookup")?;
        assert!(matches!(
            error.refusal(),
            DraftingClosureButtonholeRefusal::MatchingInspections { .. }
        ));
        assert!(error.demand().is_some());
        assert_eq!(error.shape(), None);
    }
    Ok(())
}
