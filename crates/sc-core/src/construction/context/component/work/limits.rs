//! Explicit complete current component content and prospective matching bounds.
use super::DraftingComponentWorkRefusal;
use crate::design::DesignResourceLimits;
/// Explicit fresh content/work limits,without a default or geometry/execution policy.
/// ```compile_fail
/// let limits:sc_core::construction::DraftingComponentBindingLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DraftingComponentBindingLimits {
    content: DesignResourceLimits,
    inspections: usize,
}
impl DraftingComponentBindingLimits {
    /// Select complete fresh content and conservative (E+8)² matching inspections explicitly.
    #[must_use]
    pub const fn new(content: DesignResourceLimits, matching_inspections: usize) -> Self {
        Self {
            content,
            inspections: matching_inspections,
        }
    }
    /// Fresh complete current garment/provider content allowance.
    #[must_use]
    pub const fn content(self) -> DesignResourceLimits {
        self.content
    }
    /// Conservative original member/creator/header inspection maximum.
    #[must_use]
    pub const fn matching_inspections(self) -> usize {
        self.inspections
    }
}
/// Fresh actual held population and factory-derived conservative prospective matching work.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DraftingComponentBindingShape {
    entries: usize,
    inspections: usize,
}
impl DraftingComponentBindingShape {
    /// E,the complete freshly walked original collection population,including unused sources.
    #[must_use]
    pub const fn held_collection_entries(self) -> usize {
        self.entries
    }
    /// Checked conservative (E+8)²,without CPU,string-byte,allocator or execution credit.
    #[must_use]
    pub const fn matching_inspection_bound(self) -> usize {
        self.inspections
    }
    pub(super) fn admit<'a>(
        entries: usize,
        limits: DraftingComponentBindingLimits,
    ) -> Result<Self, DraftingComponentWorkRefusal<'a>> {
        let inspections = bound(entries).ok_or(DraftingComponentWorkRefusal::PopulationOverflow)?;
        if inspections > limits.inspections {
            return Err(DraftingComponentWorkRefusal::MatchingInspections {
                allowed: limits.inspections,
                required: inspections,
            });
        }
        Ok(Self {
            entries,
            inspections,
        })
    }
}
fn bound(entries: usize) -> Option<usize> {
    let n = entries.checked_add(8)?;
    n.checked_mul(n)
}
#[cfg(test)]
mod tests {
    use super::bound;
    #[test]
    fn component_checked_bound_refuses_addition_and_square_overflow() {
        assert_eq!(bound(0), Some(64));
        assert_eq!(bound(12), Some(400));
        assert_eq!(bound(usize::MAX), None);
        assert_eq!(bound(usize::MAX - 8), None);
    }
}
