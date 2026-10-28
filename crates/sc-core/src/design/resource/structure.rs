//! Actual structural root composition, without higher metadata or whole-garment admission.
use super::{
    charge_cut_plan_resources, charge_topology_resources, DesignResourceLimitError,
    DesignResourceLocation, DesignResourceScope, DesignResourceSink, ResourceProjection,
};
use crate::design::{DesignStructure, DesignStructureDefinition};
/// Charge the actual structural root under the caller's cumulative meter, in schema order.
/// Exact revision magnitude bits precede canonical inputs, Pieces, copies and topology. Zero has zero
/// magnitude bits. No formatting, counter advance, model copy, current validation or value lookup occurs.
/// Exhaustive private owner/definition views expose future root-field accounting obligations.
/// This covers DesignStructure only; higher Inputs/Sewing/Garment and actual source-host work compose
/// separately. Success is not whole-model, runtime, source-currentness, physical or allocator admission.
/// # Errors
/// Returns the original first refusal; revision refusal carries the actual Design identity.
pub fn charge_design_structure_resources(
    structure: &DesignStructure,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    charge_structure_projected(structure, ResourceProjection::Current, sink)
}
pub(super) fn charge_structure_projected(
    structure: &DesignStructure,
    projection: ResourceProjection<'_>,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let DesignStructureDefinition {
        id,
        revision,
        inputs,
        pieces,
        cut_plan,
        topology,
    } = structure.resource_definition();
    let bits = if let ResourceProjection::SuccessorRevision(maximum_issued) = projection {
        maximum_issued.successor_resource_bits()
    } else {
        Some(revision.resource_bits())
    };
    charge_revision_bits(
        bits,
        sink,
        DesignResourceLocation {
            scope: DesignResourceScope::Structure,
            owner: Some(*id),
            field: "design.revision",
            index: None,
        },
    )?;
    let (length, material) = match projection {
        ResourceProjection::Current => (None, None),
        ResourceProjection::Length(replacement) => (Some(replacement), None),
        ResourceProjection::AppendedNotch(_) => (None, None),
        ResourceProjection::SuccessorRevision(_) => (None, None),
        ResourceProjection::Material { pieces, assignment } => (None, Some((pieces, assignment))),
    };
    super::canonical::charge_canonical_with_length(inputs, length, sink)?;
    super::pieces::charge_pieces_projected(pieces, material, sink)?;
    charge_cut_plan_resources(cut_plan, sink)?;
    charge_topology_resources(topology, sink)
}

// None denotes exactly u64::MAX + one bits. Charge two exact chunks instead of wrapping, clipping
// or imposing a counter cap. No finite u64 policy can admit both, and the sink latches the first error.
pub(crate) fn charge_revision_bits(
    bits: Option<u64>,
    sink: &mut dyn DesignResourceSink,
    location: DesignResourceLocation,
) -> Result<(), DesignResourceLimitError> {
    if let Some(bits) = bits {
        sink.charge_counter_bits(bits, location)
    } else {
        sink.charge_counter_bits(u64::MAX, location)?;
        sink.charge_counter_bits(1, location)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::design::{DesignResourceLimits, DesignResourceMeter};
    use core::num::{NonZeroU64, NonZeroUsize};
    #[test]
    fn magnitude_count_beyond_u64_policy_refuses_exact_carry_without_wrap_or_partial_demand() {
        let location = DesignResourceLocation {
            scope: DesignResourceScope::Structure,
            owner: None,
            field: "design.revision",
            index: None,
        };
        let mut meter = DesignResourceMeter::new(DesignResourceLimits::new(
            NonZeroUsize::MAX,
            NonZeroUsize::MAX,
            NonZeroU64::MAX,
        ));
        let expected = DesignResourceLimitError::CounterBits {
            maximum: u64::MAX,
            consumed: u64::MAX,
            requested: 1,
            location,
        };
        assert_eq!(u128::from(u64::MAX) + 1, 1_u128 << 64);
        assert_eq!(
            charge_revision_bits(None, &mut meter, location),
            Err(expected)
        );
        assert_eq!(meter.finish(), Err(expected));
    }
}
