//! Exact original Pocket/Closure fields and locations,without source or mixed-order acceptance.
use super::{
    garment_source::{garment, garment_with_closures, garment_with_opening},
    TestResult,
};
use sc_core::{construction::*, ontology::*};
use sc_units::Length;
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
#[test]
fn pocket_fields_keep_actual_garment_owner_nested_indices_and_source_buffers() -> TestResult {
    let garment = garment(false)?;
    let pocket = garment.pockets().pockets().first().ok_or("actual Pocket")?;
    let definition = pocket.definition();
    let mut fields = garment.pocket_closure_dependencies();
    for (expected, field) in [
        DraftingPocketSelectionField::Served,
        DraftingPocketSelectionField::Position,
        DraftingPocketSelectionField::Orientation,
        DraftingPocketSelectionField::Opening,
        DraftingPocketSelectionField::Component { index: 0 },
    ]
    .into_iter()
    .enumerate()
    {
        let use_ = fields.next().ok_or("complete Pocket fields")?;
        assert_eq!(
            use_.selection_location(),
            DraftingSemanticSelectionLocation::Pocket { pocket: 0, field }
        );
        let expected_need = if expected == 3 {
            DraftingSemanticSelectionNeed::Required(DraftingSemanticSelectionKey::Profile(
                DraftingSemanticSelectionLocation::Pocket {
                    pocket: 0,
                    field: DraftingPocketSelectionField::Opening,
                },
            ))
        } else {
            DraftingSemanticSelectionNeed::NoDirectSelection
        };
        assert_eq!(use_.selection_need(), expected_need);
        assert!(core::ptr::eq(use_.garment(), &garment));
        assert!(
            matches!(use_.owner(),DraftingGarmentSemanticOwner::Pocket{index:0,pocket:actual} if core::ptr::eq(actual,pocket))
        );
        let matches = match (expected, use_.field()) {
            (0, DraftingGarmentSemanticField::PocketServed(actual)) => {
                core::ptr::eq(actual, &definition.served)
            }
            (1, DraftingGarmentSemanticField::PocketPosition(actual)) => {
                core::ptr::eq(actual, &definition.position)
            }
            (2, DraftingGarmentSemanticField::PocketOrientation(actual)) => {
                core::ptr::eq(actual, &definition.orientation)
            }
            (3, DraftingGarmentSemanticField::PocketOpening(actual)) => {
                core::ptr::eq(actual, &definition.opening)
            }
            (
                4,
                DraftingGarmentSemanticField::PocketComponent {
                    index: 0,
                    reference,
                },
            ) => core::ptr::eq(
                reference,
                definition.components.first().ok_or("actual component")?,
            ),
            _ => false,
        };
        assert!(matches);
    }
    assert!(fields.next().is_none());
    assert!(fields.next().is_none());
    Ok(())
}
#[test]
fn declaration_opening_keeps_root_source_without_profile_selection() -> TestResult {
    let garment = garment_with_opening(PocketOpening::Declaration(id(700)))?;
    let uses: Vec<_> = garment.pocket_closure_dependencies().collect();
    assert_eq!(uses.len(), 5);
    for use_ in uses {
        assert_eq!(
            use_.selection_need(),
            DraftingSemanticSelectionNeed::NoDirectSelection
        );
        assert!(core::ptr::eq(use_.garment(), &garment));
    }
    Ok(())
}
#[test]
fn all_constructible_closure_kinds_keep_actual_sources_and_physical_instances() -> TestResult {
    let kinds = vec![
        ClosureKind::CentredZipper {
            length: ClosureLength::Explicit {
                parameter: id(701),
                value: Length::from_micrometres(12345)?,
            },
        },
        ClosureKind::CentredZipper {
            length: ClosureLength::Formula(id(702)),
        },
        ClosureKind::CentredZipper {
            length: ClosureLength::Profile(ProfileParameterRef::new(id(703))),
        },
        ClosureKind::HookAndBar {
            hook: NotionSize::Declaration(id(704)),
            bar: NotionSize::Declaration(id(704)),
        },
        ClosureKind::ButtonAndButtonhole {
            button: NotionSize::Profile(ProfileParameterRef::new(id(705))),
            hole: ButtonholeDerivation { operation: id(706) },
        },
    ];
    let garment = garment_with_closures(kinds)?;
    use DraftingClosureSelectionField as C;
    let expected_locations = [
        (0, C::Length),
        (0, C::Instance { index: 0 }),
        (1, C::Length),
        (1, C::Instance { index: 0 }),
        (2, C::Length),
        (2, C::Instance { index: 0 }),
        (3, C::Size(DraftingClosureSizeRole::Hook)),
        (3, C::Size(DraftingClosureSizeRole::Bar)),
        (3, C::Instance { index: 0 }),
        (4, C::Size(DraftingClosureSizeRole::Button)),
        (4, C::Buttonhole),
        (4, C::Instance { index: 0 }),
    ];
    let locations: Vec<_> = garment
        .pocket_closure_dependencies()
        .skip(5)
        .map(|use_| use_.selection_location())
        .collect();
    assert_eq!(
        locations,
        expected_locations
            .map(|(closure, field)| DraftingSemanticSelectionLocation::Closure { closure, field })
    );
    let needs: Vec<_> = garment
        .pocket_closure_dependencies()
        .skip(5)
        .map(|use_| use_.selection_need())
        .collect();
    let expected_needs = expected_locations.map(|(closure, field)| {
        if matches!(
            (closure, field),
            (2, C::Length) | (4, C::Size(DraftingClosureSizeRole::Button))
        ) {
            DraftingSemanticSelectionNeed::Required(DraftingSemanticSelectionKey::Profile(
                DraftingSemanticSelectionLocation::Closure { closure, field },
            ))
        } else {
            DraftingSemanticSelectionNeed::NoDirectSelection
        }
    });
    assert_eq!(needs, expected_needs);
    let mut fields = garment.pocket_closure_dependencies().skip(5);
    for (index, closure) in garment.closures().closures().iter().enumerate() {
        let mut next = || -> TestResult<DraftingGarmentSemanticField<'_>> {
            let use_ = fields.next().ok_or("original Closure occurrence")?;
            assert!(core::ptr::eq(use_.garment(), &garment));
            assert!(
                matches!(use_.owner(),DraftingGarmentSemanticOwner::Closure{index:actual,closure:member} if actual==index && core::ptr::eq(member,closure))
            );
            Ok(use_.field())
        };
        match &closure.definition().kind {
            ClosureKind::CentredZipper { length } => assert!(
                matches!(next()?,DraftingGarmentSemanticField::ClosureLength(actual) if core::ptr::eq(actual,length))
            ),
            ClosureKind::HookAndBar { hook, bar } => {
                assert!(
                    matches!(next()?,DraftingGarmentSemanticField::ClosureSize{role:DraftingClosureSizeRole::Hook,source} if core::ptr::eq(source,hook))
                );
                assert!(
                    matches!(next()?,DraftingGarmentSemanticField::ClosureSize{role:DraftingClosureSizeRole::Bar,source} if core::ptr::eq(source,bar))
                );
                assert_eq!(hook, bar);
                assert!(
                    !core::ptr::eq(hook, bar),
                    "equal sources remain separate original fields"
                );
            }
            ClosureKind::ButtonAndButtonhole { button, hole } => {
                assert!(
                    matches!(next()?,DraftingGarmentSemanticField::ClosureSize{role:DraftingClosureSizeRole::Button,source} if core::ptr::eq(source,button))
                );
                assert!(
                    matches!(next()?,DraftingGarmentSemanticField::ClosureButtonhole(actual) if core::ptr::eq(actual,hole))
                );
            }
            ClosureKind::Fly { .. } => return Err("unsupported constructor kind".into()),
        }
        assert!(
            matches!(next()?,DraftingGarmentSemanticField::ClosureInstance{index:0,instance} if core::ptr::eq(instance,closure.definition().instances.first().ok_or("actual instance")?))
        );
    }
    assert!(fields.next().is_none());
    assert!(fields.next().is_none());
    assert_eq!(garment.pocket_closure_dependencies().count(), 17);
    Ok(())
}
