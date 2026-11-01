//! Ordered exact retained construction-field partitions,not target occurrence or geometry proof.
use super::{
    DeclaredJoinConstructionGroups, JoinConstructionGroupOutput, JoinConstructionOutputGroup,
};
use crate::ontology::{Direction, Param};
use core::fmt;

/// Complete actual retained group G and original reference L scan allowances.
/// ```compile_fail
/// let limits:sc_core::construction::JoinConstructionCoverageLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinConstructionCoverageLimits {
    groups: usize,
    references: usize,
}
impl JoinConstructionCoverageLimits {
    /// Admit every actual group/reference before directions or interval fields;no new workspace.
    #[must_use]
    pub const fn new(groups: usize, references: usize) -> Self {
        Self { groups, references }
    }
    /// Complete actual retained group population.
    #[must_use]
    pub const fn groups(self) -> usize {
        self.groups
    }
    /// Complete original tag-reference population,including repetitions.
    #[must_use]
    pub const fn references(self) -> usize {
        self.references
    }
}
/// First complete quota or exact original ordered interval partition cause.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinConstructionCoverageRefusal {
    /// Complete group population exceeds explicit allowance.
    Groups {
        /// Actual whole G.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete reference population exceeds explicit allowance.
    References {
        /// Actual whole L.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Uncovered positive original interval;bounds remain ascending regardless of field traversal.
    Gap {
        /// Original group ordinal.
        group: usize,
        /// Next original reference,or None for the uncovered final suffix in field traversal.
        index: Option<usize>,
        /// First uncovered ascending parameter.
        from: Param,
        /// Last uncovered ascending parameter.
        to: Param,
    },
    /// Reference starts behind the exact traversal cursor;no sorting or repeated coverage is allowed.
    Overlap {
        /// Original group ordinal.
        group: usize,
        /// Original reference position.
        index: usize,
        /// Exact parameter already reached in original field traversal.
        cursor: Param,
    },
}
/// Actual whole grouping parent,policy and original first refusal;no partial certificate escapes.
pub struct JoinConstructionCoverageError<'a> {
    groups: &'a DeclaredJoinConstructionGroups<'a>,
    limits: JoinConstructionCoverageLimits,
    refusal: JoinConstructionCoverageRefusal,
}
impl<'a> JoinConstructionCoverageError<'a> {
    /// Actual whole original groups/source actions/proposed-use context.
    #[must_use]
    pub const fn groups(&self) -> &'a DeclaredJoinConstructionGroups<'a> {
        self.groups
    }
    /// Original explicit complete scan policy.
    #[must_use]
    pub const fn limits(&self) -> JoinConstructionCoverageLimits {
        self.limits
    }
    /// First whole quota or original ordered interval cause.
    #[must_use]
    pub const fn refusal(&self) -> JoinConstructionCoverageRefusal {
        self.refusal
    }
    /// Actual original group for interval causes;whole quota causes return None.
    #[must_use]
    pub fn refused_group(&self) -> Option<JoinConstructionOutputGroup<'a>> {
        let index = match self.refusal {
            JoinConstructionCoverageRefusal::Gap { group, .. }
            | JoinConstructionCoverageRefusal::Overlap { group, .. } => group,
            _ => return None,
        };
        self.groups.groups().nth(index)
    }
    /// Actual next/offending original reference;final gaps and whole quota causes return None.
    #[must_use]
    pub fn refused_output(&self) -> Option<JoinConstructionGroupOutput<'a>> {
        let index = match self.refusal {
            JoinConstructionCoverageRefusal::Gap {
                index: Some(index), ..
            }
            | JoinConstructionCoverageRefusal::Overlap { index, .. } => index,
            _ => return None,
        };
        self.refused_group()?.outputs().nth(index)
    }
}
impl fmt::Debug for JoinConstructionCoverageError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinConstructionCoverageError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for JoinConstructionCoverageError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "join retained construction coverage: {:?}", self.refusal)
    }
}
impl std::error::Error for JoinConstructionCoverageError<'_> {}
impl<'a> DeclaredJoinConstructionGroups<'a> {
    /// Require every actual Retain field's complete0..1 source interval exactly once,in original
    /// field traversal and explicit reference order. Original fields walk0→1,reversed fields1→0;
    /// source ranges remain positive ascending. Admit full G/L before any direction/range scan.
    /// Work O(G+L+1),constant cursor state,no allocation/sort/rescale/provider/geometry/evidence call.
    /// Prior complete input/action/group policies and storage remain. This checks source parameter
    /// bookkeeping only:actual target presence/traversal/multiplicity,additional intent,omission
    /// justification,currentness,achieved maps,geometry and atomic effects remain required separately.
    /// # Errors
    /// Whole G/L quotas,then first original group's first gap/overlap,including final traversal gap.
    pub fn require_source_coverage(
        &'a self,
        limits: JoinConstructionCoverageLimits,
    ) -> Result<CoveredJoinConstructionGroups<'a>, JoinConstructionCoverageError<'a>> {
        use JoinConstructionCoverageRefusal as R;
        let refuse = |refusal| JoinConstructionCoverageError {
            groups: self,
            limits,
            refusal,
        };
        let g = self.group_count();
        let l = self.reference_count();
        if g > limits.groups {
            return Err(refuse(R::Groups {
                required: g,
                allowed: limits.groups,
            }));
        }
        if l > limits.references {
            return Err(refuse(R::References {
                required: l,
                allowed: limits.references,
            }));
        }
        for group in self.groups() {
            let reversed = group.source_action().source_field().direction == Direction::Reversed;
            let (mut cursor, end) = if reversed {
                (Param::END, Param::START)
            } else {
                (Param::START, Param::END)
            };
            for output in group.outputs() {
                let range = output.source();
                let (entry, exit) = if reversed {
                    (range.to(), range.from())
                } else {
                    (range.from(), range.to())
                };
                if entry != cursor {
                    let overlaps = if reversed {
                        entry > cursor
                    } else {
                        entry < cursor
                    };
                    return Err(refuse(if overlaps {
                        R::Overlap {
                            group: group.index(),
                            index: output.index(),
                            cursor,
                        }
                    } else {
                        R::Gap {
                            group: group.index(),
                            index: Some(output.index()),
                            from: cursor.min(entry),
                            to: cursor.max(entry),
                        }
                    }));
                }
                cursor = exit;
            }
            if cursor != end {
                return Err(refuse(R::Gap {
                    group: group.index(),
                    index: None,
                    from: cursor.min(end),
                    to: cursor.max(end),
                }));
            }
        }
        Ok(CoveredJoinConstructionGroups {
            groups: self,
            limits,
        })
    }
}
/// Authentic ordered whole retained source-field partitions;no target/geometry/omission authority.
/// ```compile_fail
/// fn forge<'a>(mut proof:sc_core::construction::CoveredJoinConstructionGroups<'a>,other:&'a sc_core::construction::DeclaredJoinConstructionGroups<'a>) {proof.groups=other;}
/// ```
/// ```compile_fail
/// fn escape(groups:sc_core::construction::DeclaredJoinConstructionGroups<'_>)->sc_core::construction::CoveredJoinConstructionGroups<'static> {
///     groups.require_source_coverage(sc_core::construction::JoinConstructionCoverageLimits::new(0,0)).unwrap()
/// }
#[derive(Debug)]
pub struct CoveredJoinConstructionGroups<'a> {
    groups: &'a DeclaredJoinConstructionGroups<'a>,
    limits: JoinConstructionCoverageLimits,
}
impl<'a> CoveredJoinConstructionGroups<'a> {
    /// Actual whole original grouping/source/action/proposed-use parents.
    #[must_use]
    pub const fn groups(&self) -> &'a DeclaredJoinConstructionGroups<'a> {
        self.groups
    }
    /// Original complete scan policy.
    #[must_use]
    pub const fn limits(&self) -> JoinConstructionCoverageLimits {
        self.limits
    }
    /// Complete covered Retain group population;Omit scopes remain in the source-action parent.
    #[must_use]
    pub fn group_count(&self) -> usize {
        self.groups.group_count()
    }
    /// Complete original references,including distinct repeated source scopes.
    #[must_use]
    pub fn reference_count(&self) -> usize {
        self.groups.reference_count()
    }
}
