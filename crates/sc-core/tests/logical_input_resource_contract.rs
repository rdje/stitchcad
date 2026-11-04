//! Complete standalone logical content retains original locations and cumulative first refusals.
use core::num::{NonZeroU64, NonZeroUsize};
use sc_core::{design::*, name::MachineToken, ontology::EntityId, value::*};
use std::error::Error;
type TestResult<T = ()> = Result<T, Box<dyn Error>>;
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn limits(entries: usize, text: usize, bits: u64) -> TestResult<DesignResourceLimits> {
    Ok(DesignResourceLimits::new(
        NonZeroUsize::new(entries).ok_or("positive entries")?,
        NonZeroUsize::new(text).ok_or("positive text")?,
        NonZeroU64::new(bits).ok_or("positive bits")?,
    ))
}
fn member() -> TestResult<MachineToken> {
    Ok(MachineToken::new("small")?)
}
fn fixture() -> TestResult<LogicalInputs> {
    let states = vec![
        LogicalInputState::Known {
            value: member()?,
            evidence: vec![id(2002), id(2001)],
        },
        LogicalInputState::Assumed {
            value: member()?,
            assumption: id(2010),
        },
        LogicalInputState::Unknown {
            observation: id(2020),
        },
        LogicalInputState::Choice {
            value: member()?,
            decision: id(2030),
        },
        LogicalInputState::Preference {
            value: member()?,
            provenance: id(2040),
        },
        LogicalInputState::Derived {
            operation: id(2050),
        },
    ];
    let declarations = states
        .into_iter()
        .zip(1000_u128..)
        .map(|(state, n)| -> TestResult<LogicalInputDeclaration> {
            Ok(LogicalInputDeclaration::new(
                LogicalInputDeclarationDefinition {
                    id: id(n),
                    source: id(900),
                    kind: if n % 2 == 0 {
                        LogicalInputKind::PocketOpening
                    } else {
                        LogicalInputKind::NotionSize
                    },
                    domain: vec![MachineToken::new("small")?, MachineToken::new("large")?],
                    state,
                },
                LogicalInputLimits::new(100, 100, 10000, 100000),
            )?)
        })
        .collect::<TestResult<Vec<_>>>()?;
    Ok(LogicalInputs::new(
        declarations,
        LogicalInputInventoryLimits::new(100, 10000),
    )?)
}
fn at(field: &'static str, owner: Option<u128>, index: Option<usize>) -> DesignResourceLocation {
    DesignResourceLocation {
        scope: DesignResourceScope::Structure,
        owner: owner.map(id),
        field,
        index,
    }
}
#[test]
fn exhaustive_actual_domains_selections_evidence_and_fixed_proofs_count_without_policy_credit(
) -> TestResult {
    let inputs = fixture()?;
    let mut meter = DesignResourceMeter::new(limits(20, 80, 1)?);
    charge_logical_input_resources(&inputs, &mut meter)?;
    let demand = meter.finish()?;
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (20, 80, 0)
    );
    assert_eq!(
        inputs
            .declarations()
            .iter()
            .map(|r| r.definition().state.tag())
            .collect::<Vec<_>>(),
        [
            LogicalInputStateTag::Known,
            LogicalInputStateTag::Assumed,
            LogicalInputStateTag::Unknown,
            LogicalInputStateTag::Choice,
            LogicalInputStateTag::Preference,
            LogicalInputStateTag::Derived,
        ]
    );
    let prior = at("prior", None, Some(7));
    let mut meter = DesignResourceMeter::new(limits(22, 83, 1)?);
    meter.charge_entries(2, prior)?;
    meter.charge_text("old", prior)?;
    meter.charge_counter_bits(1, prior)?;
    charge_logical_input_resources(&inputs, &mut meter)?;
    let demand = meter.finish()?;
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (22, 83, 1)
    );
    let known = inputs.declarations().first().ok_or("actual Known record")?;
    let mut meter = DesignResourceMeter::new(limits(4, 15, 1)?);
    charge_logical_declaration_resources(known, &mut meter)?;
    let demand = meter.finish()?;
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (4, 15, 0)
    );
    assert_eq!(known.definition().state.tag(), LogicalInputStateTag::Known);
    Ok(())
}
#[test]
fn actual_collection_precharges_and_every_selected_state_refusal_keep_owner_and_original_index(
) -> TestResult {
    let inputs = fixture()?;
    use DesignResourceCount as U;
    // Exact first fields distinguish the original raw walk from a cached aggregate charge.
    for (entries, text, unit, maximum, consumed, requested, location) in [
        (
            5,
            80,
            U::CollectionEntries,
            5,
            0,
            6,
            at("canonical.logicals", None, None),
        ),
        (
            7,
            80,
            U::CollectionEntries,
            7,
            6,
            2,
            at("canonical.logicals.domain", Some(1000), Some(0)),
        ),
        (
            20,
            4,
            U::TextBytes,
            4,
            0,
            5,
            at("canonical.logicals.domain.member", Some(1000), Some(0)),
        ),
        (
            20,
            9,
            U::TextBytes,
            9,
            5,
            5,
            at("canonical.logicals.domain.member", Some(1000), Some(1)),
        ),
        (
            20,
            14,
            U::TextBytes,
            14,
            10,
            5,
            at("canonical.logicals.value", Some(1000), Some(0)),
        ),
        (
            9,
            80,
            U::CollectionEntries,
            9,
            8,
            2,
            at("canonical.logicals.evidence", Some(1000), Some(0)),
        ),
        (
            11,
            80,
            U::CollectionEntries,
            11,
            10,
            2,
            at("canonical.logicals.domain", Some(1001), Some(1)),
        ),
        (
            20,
            29,
            U::TextBytes,
            29,
            25,
            5,
            at("canonical.logicals.value", Some(1001), Some(1)),
        ),
        (
            20,
            54,
            U::TextBytes,
            54,
            50,
            5,
            at("canonical.logicals.value", Some(1003), Some(3)),
        ),
        (
            20,
            69,
            U::TextBytes,
            69,
            65,
            5,
            at("canonical.logicals.value", Some(1004), Some(4)),
        ),
        (
            20,
            79,
            U::TextBytes,
            79,
            75,
            5,
            at("canonical.logicals.domain.member", Some(1005), Some(1)),
        ),
    ] {
        let mut meter = DesignResourceMeter::new(limits(entries, text, 1)?);
        let expected = DesignResourceLimitError::Count {
            unit,
            maximum,
            consumed,
            requested,
            location,
        };
        assert_eq!(
            charge_logical_input_resources(&inputs, &mut meter),
            Err(expected)
        );
        assert_eq!(meter.finish(), Err(expected));
    }
    let known = inputs.declarations().first().ok_or("actual Known record")?;
    let mut meter = DesignResourceMeter::new(limits(3, 15, 1)?);
    let expected = DesignResourceLimitError::Count {
        unit: U::CollectionEntries,
        maximum: 3,
        consumed: 2,
        requested: 2,
        location: at("canonical.logicals.evidence", Some(1000), None),
    };
    assert_eq!(
        charge_logical_declaration_resources(known, &mut meter),
        Err(expected)
    );
    assert_eq!(meter.finish(), Err(expected));
    Ok(())
}
#[test]
fn empty_inventory_and_all_producers_preserve_a_previously_latched_refusal() -> TestResult {
    let inputs = fixture()?;
    let empty = LogicalInputs::new(vec![], LogicalInputInventoryLimits::new(0, 0))?;
    let mut meter = DesignResourceMeter::new(limits(1, 1, 1)?);
    charge_logical_input_resources(&empty, &mut meter)?;
    let demand = meter.finish()?;
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (0, 0, 0)
    );
    for inventory in [&inputs, &empty] {
        let mut meter = DesignResourceMeter::new(limits(1, 1, 1)?);
        let location = at("original.first", None, Some(7));
        let expected = DesignResourceLimitError::Count {
            unit: DesignResourceCount::CollectionEntries,
            maximum: 1,
            consumed: 0,
            requested: 2,
            location,
        };
        assert_eq!(meter.charge_entries(2, location), Err(expected));
        assert_eq!(
            charge_logical_input_resources(inventory, &mut meter),
            Err(expected)
        );
        assert_eq!(
            charge_logical_declaration_resources(
                inputs.declarations().first().ok_or("actual declaration")?,
                &mut meter
            ),
            Err(expected)
        );
        assert_eq!(meter.finish(), Err(expected));
    }
    Ok(())
}
