//! Explicit fresh content and conservative prospective original matching inspections.
use super::DraftingRuleWorkRefusal;
use crate::design::DesignResourceLimits;
/// Explicit content and logical matching allowance;no default policy or runtime estimate.
/// ```compile_fail
/// let limits:sc_core::construction::DraftingRuleBindingLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DraftingRuleBindingLimits {
    content: DesignResourceLimits,
    inspections: usize,
}
impl DraftingRuleBindingLimits {
    /// Select complete fresh content and conservative original matching inspections explicitly.
    #[must_use]
    pub const fn new(content: DesignResourceLimits, matching_inspections: usize) -> Self {
        Self {
            content,
            inspections: matching_inspections,
        }
    }
    /// Complete fresh Root/provider/request-pin logical content allowance.
    #[must_use]
    pub const fn content(self) -> DesignResourceLimits {
        self.content
    }
    /// Maximum conservative (E+8)² original member/header/role inspection bound.
    #[must_use]
    pub const fn matching_inspections(self) -> usize {
        self.inspections
    }
}
/// Factory-derived complete held collection population and conservative matching bound.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DraftingRuleBindingShape {
    entries: usize,
    inspections: usize,
}
impl DraftingRuleBindingShape {
    /// E, every complete actual Root/provider/request dynamic collection entry.
    #[must_use]
    pub const fn held_collection_entries(self) -> usize {
        self.entries
    }
    /// Checked (E+8)²,not measured CPU,string byte work or allocator capacity.
    #[must_use]
    pub const fn matching_inspection_bound(self) -> usize {
        self.inspections
    }
    pub(super) fn admit<'a>(
        entries: usize,
        limits: DraftingRuleBindingLimits,
    ) -> Result<Self, DraftingRuleWorkRefusal<'a>> {
        let inspections = bound(entries).ok_or(DraftingRuleWorkRefusal::PopulationOverflow)?;
        if inspections > limits.inspections {
            return Err(DraftingRuleWorkRefusal::MatchingInspections {
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
    let padded = entries.checked_add(8)?;
    padded.checked_mul(padded)
}
#[cfg(test)]
mod tests {
    use super::bound;
    #[test]
    fn complete_checked_bound_refuses_addition_and_square_overflow() {
        assert_eq!(bound(0), Some(64));
        assert_eq!(bound(13), Some(441));
        assert_eq!(bound(usize::MAX), None);
        assert_eq!(bound(usize::MAX - 8), None);
    }
}
