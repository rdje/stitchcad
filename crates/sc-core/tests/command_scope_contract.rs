//! Independently authored scope boundaries and expansion attempts; no ambient clock or authenticated grant.
use sc_core::{
    command::{
        CommandId as C, CommandScope, CommandScopeNarrowingError as Narrowing,
        CommandScopeRefusal as Refusal, CommandScopeWindowError,
    },
    ontology::EntityId as Id,
};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
fn at(second: u64) -> SystemTime {
    UNIX_EPOCH + Duration::from_secs(second)
}

#[test]
fn coverage_uses_exact_design_commands_and_half_open_time_without_class_expansion(
) -> Result<(), Box<dyn std::error::Error>> {
    let inventory = vec![C::ExportPdf, C::GetPieceList, C::ExportPdf];
    let scope = CommandScope::new(Id::from_bits(1), inventory.clone(), at(10), at(20))?;
    assert_eq!(scope.design(), Id::from_bits(1));
    assert_eq!(scope.commands(), inventory);
    assert_eq!(scope.valid_from(), at(10));
    assert_eq!(scope.valid_until(), at(20));
    for second in [10, 11, 19] {
        for command in [C::ExportPdf, C::GetPieceList] {
            assert_eq!(scope.check(Id::from_bits(1), command, at(second)), Ok(()));
        }
    }
    assert_eq!(
        scope.check(Id::from_bits(2), C::ExportPdf, at(10)),
        Err(Refusal::Design {
            scoped: Id::from_bits(1),
            requested: Id::from_bits(2)
        })
    );
    for command in [C::ExportPlt, C::ApprovePackage, C::PrepareRelease] {
        assert_eq!(
            scope.check(Id::from_bits(1), command, at(10)),
            Err(Refusal::Command { requested: command })
        );
    }
    assert_eq!(
        scope.check(Id::from_bits(1), C::ExportPdf, at(9)),
        Err(Refusal::NotYetValid {
            at: at(9),
            valid_from: at(10)
        })
    );
    for second in [20, 21] {
        assert_eq!(
            scope.check(Id::from_bits(1), C::ExportPdf, at(second)),
            Err(Refusal::Expired {
                at: at(second),
                valid_until: at(20)
            })
        );
    }
    let empty = CommandScope::new(Id::from_bits(1), vec![], at(10), at(20))?;
    for command in C::ALL {
        assert_eq!(
            empty.check(Id::from_bits(1), command, at(10)),
            Err(Refusal::Command { requested: command })
        );
    }
    Ok(())
}

#[test]
fn invalid_windows_retain_exact_endpoints_without_reordering_or_default_duration() {
    for (from, until) in [(at(10), at(10)), (at(20), at(10))] {
        assert_eq!(
            CommandScope::new(Id::from_bits(1), vec![C::GetPieceList], from, until),
            Err(CommandScopeWindowError {
                valid_from: from,
                valid_until: until
            })
        );
    }
}

#[test]
fn narrowing_preserves_requested_order_and_refuses_each_scope_expansion(
) -> Result<(), Box<dyn std::error::Error>> {
    let parent = CommandScope::new(
        Id::from_bits(1),
        vec![C::ExportPdf, C::GetPieceList],
        at(10),
        at(20),
    )?;
    let original = parent.clone();
    let child = CommandScope::new(
        Id::from_bits(1),
        vec![C::GetPieceList, C::ExportPdf, C::GetPieceList],
        at(12),
        at(19),
    )?;
    assert_eq!(parent.narrow_to(child.clone()), Ok(child));
    assert_eq!(parent.narrow_to(parent.clone()), Ok(parent.clone()));
    let empty = CommandScope::new(Id::from_bits(1), vec![], at(10), at(20))?;
    assert_eq!(parent.narrow_to(empty.clone()), Ok(empty));
    assert_eq!(
        parent.narrow_to(CommandScope::new(
            Id::from_bits(2),
            vec![C::GetPieceList],
            at(10),
            at(20)
        )?),
        Err(Narrowing::Design {
            parent: Id::from_bits(1),
            requested: Id::from_bits(2)
        })
    );
    assert_eq!(
        parent.narrow_to(CommandScope::new(
            Id::from_bits(1),
            vec![C::GetPieceList, C::ExportPlt],
            at(10),
            at(20)
        )?),
        Err(Narrowing::Command {
            requested: C::ExportPlt
        })
    );
    for (from, until) in [(at(9), at(20)), (at(10), at(21)), (at(9), at(21))] {
        assert_eq!(
            parent.narrow_to(CommandScope::new(
                Id::from_bits(1),
                vec![C::ExportPdf],
                from,
                until
            )?),
            Err(Narrowing::Validity {
                parent_from: at(10),
                parent_until: at(20),
                requested_from: from,
                requested_until: until
            })
        );
    }
    assert_eq!(
        parent, original,
        "neither accepted children nor refusals alter parent scope"
    );
    Ok(())
}
