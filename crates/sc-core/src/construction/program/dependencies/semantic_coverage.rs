//! Fresh bounded direct selection-key coverage;authentic target needs and sources remain separate.
use super::{
    AdmittedDraftingProgramSemanticWork, DraftingProgramSemanticInputs,
    DraftingProgramSemanticWorkBudget, DraftingProgramSemanticWorkError,
    DraftingProgramSemanticWorkLimits,
};
use crate::{
    construction::{
        DraftingGarmentProvider, DraftingInitialInputProvider, DraftingSemanticSelectionKey as Key,
        DraftingSemanticSelectionLocation as Location, DraftingSemanticSelectionNeed as Need,
    },
    design::{DesignReferenceHost, DesignResourceSource},
};
use core::fmt;
mod error;
pub use error::{DraftingProgramSelectionCoverageError, DraftingProgramSelectionCoverageRefusal};

/// Classification of one original row after actual key matching,not accepted source consumption.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingProgramSelectionRowKind {
    /// Matches an actual required direct Profile field or explicit rule/policy origin.
    DirectRequirement,
    /// Matches an actual native target;its internal need is not established or consumed yet.
    DeferredTargetContext,
}
/// One actual original field's bounded key association. Its exact source universe is retained by
/// the containing private coverage proof. This record alone supplies no source/member proof.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DraftingProgramSelectionAssociation {
    location: Location,
    need: Need,
    row: Option<usize>,
}
impl DraftingProgramSelectionAssociation {
    /// Actual original field coordinates,not a source certificate.
    #[must_use]
    pub const fn location(self) -> Location {
        self.location
    }
    /// Original direct need or unresolved target context.
    #[must_use]
    pub const fn need(self) -> Need {
        self.need
    }
    /// Original zero-based request row,if required or supplied for a potential target context.
    #[must_use]
    pub const fn row_index(self) -> Option<usize> {
        self.row
    }
}
/// Complete direct key coverage under exact original owners and one fresh aggregate budget.
/// Native context rows are only potential intent. No actual source,Profile body,selected value,
/// complete target-internal need,unused-row/cohort acceptance or executable proof is supplied.
/// ```compile_fail,E0616
/// fn alter<I:?Sized>(p:&mut sc_core::construction::CheckedDraftingProgramDirectSelections<'_,I>) {p.associations.clear();}
/// ```
/// ```compile_fail
/// use sc_core::construction::CheckedDraftingProgramDirectSelections;
/// fn escape<'w,I:?Sized>(p:CheckedDraftingProgramDirectSelections<'w,I>)->CheckedDraftingProgramDirectSelections<'static,I> {p}
/// ```
/// ```compile_fail
/// fn duplicate<I:?Sized>(p:sc_core::construction::CheckedDraftingProgramDirectSelections<'_,I>) {let _=p.clone();}
/// ```
pub struct CheckedDraftingProgramDirectSelections<'w, I: ?Sized> {
    budget: DraftingProgramSemanticWorkBudget<'w, I>,
    associations: Vec<DraftingProgramSelectionAssociation>,
    rows: Vec<DraftingProgramSelectionRowKind>,
}
impl<'w, I: ?Sized> CheckedDraftingProgramDirectSelections<'w, I> {
    /// Exact original joined owner,including actual garment/programme and request/pin loans.
    #[must_use]
    pub const fn inputs(&self) -> &'w I {
        self.budget.work().inputs()
    }
    /// Exact original selected full aggregate policy.
    #[must_use]
    pub const fn limits(&self) -> &'w DraftingProgramSemanticWorkLimits {
        self.budget.work().limits()
    }
    /// Original fresh complete content,population and finite selected work admission.
    #[must_use]
    pub const fn work(&self) -> &AdmittedDraftingProgramSemanticWork<'w, I> {
        self.budget.work()
    }
    /// Every original field once:garment order,then semantic/native fields merged in genuine mixed
    /// order,then original local field order. NoDirectSelection fields remain present with no row.
    #[must_use]
    pub fn associations(&self) -> &[DraftingProgramSelectionAssociation] {
        &self.associations
    }
    /// Every original row in original order. Deferred context is never accepted source consumption.
    #[must_use]
    pub fn row_kinds(&self) -> &[DraftingProgramSelectionRowKind] {
        &self.rows
    }
    /// Remaining selected child-pass capacity after preceding complete-stage reservations.
    #[must_use]
    pub const fn remaining_child_passes(&self) -> usize {
        self.budget.remaining_passes()
    }
    /// Earlier complete-stage pass reservations;coverage itself reserves no child passes.
    #[must_use]
    pub const fn reserved_child_passes(&self) -> usize {
        self.budget.reserved_passes()
    }
    /// Atomically reserve a later complete stage before its child work. The original unique cursor
    /// stays inside this exact coverage owner;no mutable cursor or policy replacement is exposed.
    /// # Errors
    /// Original aggregate capacity/previous reservations and exact owner/policy,unchanged on refusal.
    pub fn reserve_child_passes(
        &mut self,
        passes: usize,
    ) -> Result<(), DraftingProgramSemanticWorkError<'w, I>> {
        self.budget.reserve_child_passes(passes)
    }
}
impl<I: ?Sized> fmt::Debug for CheckedDraftingProgramDirectSelections<'_, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CheckedDraftingProgramDirectSelections")
            .field("fields", &self.associations.len())
            .field("rows", &self.rows.len())
            .finish_non_exhaustive()
    }
}
fn key(need: Need) -> Option<Key> {
    match need {
        Need::NoDirectSelection => None,
        Need::Required(key) => Some(key),
        Need::DeferredTargetProfile(location) => Some(Key::Profile(location)),
    }
}
fn merge_fields(
    semantic: impl Iterator<Item = (usize, Location, Need)>,
    native: impl Iterator<Item = (usize, Location, Need)>,
) -> impl Iterator<Item = (Location, Need)> {
    let mut semantic = semantic.peekable();
    let mut native = native.peekable();
    core::iter::from_fn(move || {
        // Each genuine mixed slot has one kind;all its fields precede the next slot's fields.
        let semantic_first = match (semantic.peek(), native.peek()) {
            (Some((s, _, _)), Some((n, _, _))) => s < n,
            (Some(_), None) => true,
            (None, Some(_)) => false,
            (None, None) => return None,
        };
        let next = if semantic_first {
            semantic.next()
        } else {
            native.next()
        };
        next.map(|(_, location, need)| (location, need))
    })
}
impl<
        's,
        'g,
        'k,
        'j,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignReferenceHost + DesignResourceSource,
        S: DraftingInitialInputProvider<H> + DraftingGarmentProvider<H> + ?Sized,
    > DraftingProgramSemanticInputs<'s, 'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    fn selection_fields(&self) -> impl Iterator<Item = (Location, Need)> + '_ {
        let garment = self
            .garment()
            .source()
            .garment
            .pocket_closure_dependencies()
            .map(|use_| (use_.selection_location(), use_.selection_need()));
        let program = self
            .geometry()
            .work()
            .inputs()
            .inputs()
            .order()
            .inputs()
            .roles()
            .positions()
            .work()
            .inputs()
            .program();
        let semantic = program.semantic_dependencies().map(|use_| {
            (
                use_.position(),
                use_.selection_location(),
                use_.selection_need(),
            )
        });
        let native = program.native_semantic_dependencies().map(|use_| {
            (
                use_.position(),
                use_.selection_location(),
                use_.selection_need(),
            )
        });
        garment.chain(merge_fields(semantic, native))
    }
    /// Fresh complete aggregate content/population/child-envelope admission precedes allocation
    /// and all matching. Then reject the first original unmatched row,before the first missing
    /// direct requirement in garment/genuine mixed/local field order. Native target context is
    /// matched only as potential intent;actual target need and final unused rows remain unaccepted.
    ///
    /// Two bounded scans perform at most 2*O*R fixed-key comparisons. Reserve O field associations
    /// and R row classifications within the admitted O+R bound. They retain original row indices,
    /// avoiding another matching scan in later source stages. No source/member/value query or
    /// child pass runs;only the required fresh provider resource walk occurs. Private partial
    /// vectors are dropped on every refusal. The successful owner retains the unique child cursor.
    /// ```
    /// use sc_core::{construction::*,design::*};
    /// fn direct<H:DesignReferenceHost+DesignResourceSource,
    ///     S:DraftingInitialInputProvider<H>+DraftingGarmentProvider<H>+?Sized>(
    ///     inputs:&DraftingProgramSemanticInputs<'_, '_, '_, '_, '_, '_, '_, '_, '_, '_,H,S>,
    ///     limits:&DraftingProgramSemanticWorkLimits)->Result<(),String> {
    ///     let proof=inputs.check_direct_selections(limits).map_err(|e|e.to_string())?;
    ///     assert!(std::ptr::eq(proof.inputs(),inputs));
    ///     Ok(())
    /// }
    /// ```
    /// # Errors
    /// Original first work cause,fallible reservation,first unmatched row,then first missing direct
    /// key. All original owner/policy and completed fresh metadata survive without accepted prefix.
    pub fn check_direct_selections<'w>(
        &'w self,
        limits: &'w DraftingProgramSemanticWorkLimits,
    ) -> Result<
        CheckedDraftingProgramDirectSelections<'w, Self>,
        DraftingProgramSelectionCoverageError<'w, Self>,
    > {
        use DraftingProgramSelectionCoverageRefusal as R;
        let work = self.admit_semantic_work(limits).map_err(|cause| {
            DraftingProgramSelectionCoverageError::new(
                self,
                limits,
                cause.demand(),
                cause.population_shape(),
                cause.shape(),
                R::Work(cause),
            )
        })?;
        let refuse = |refusal| {
            DraftingProgramSelectionCoverageError::new(
                self,
                limits,
                Some(work.demand()),
                Some(work.population_shape()),
                Some(work.shape()),
                refusal,
            )
        };
        let mut associations = Vec::new();
        associations
            .try_reserve_exact(work.population_shape().occurrence_count())
            .map_err(|cause| {
                refuse(R::Reservation {
                    records: work.population_shape().occurrence_count(),
                    cause,
                })
            })?;
        let rows = self.selections().inputs().rows();
        let mut kinds = Vec::new();
        kinds.try_reserve_exact(rows.len()).map_err(|cause| {
            refuse(R::Reservation {
                records: rows.len(),
                cause,
            })
        })?;
        for (row, selection) in rows.iter().enumerate() {
            let requested = selection.key();
            let found = self
                .selection_fields()
                .find(|(_, need)| key(*need) == Some(requested));
            let kind = match found {
                Some((_, Need::Required(_))) => DraftingProgramSelectionRowKind::DirectRequirement,
                Some((_, Need::DeferredTargetProfile(_))) => {
                    DraftingProgramSelectionRowKind::DeferredTargetContext
                }
                Some((_, Need::NoDirectSelection)) | None => {
                    return Err(refuse(R::UnexpectedSelection {
                        row,
                        key: requested,
                    }))
                }
            };
            kinds.push(kind);
        }
        for (location, need) in self.selection_fields() {
            let row =
                key(need).and_then(|required| rows.iter().position(|row| row.key() == required));
            if let (Need::Required(key), None) = (need, row) {
                return Err(refuse(R::MissingSelection { location, key }));
            }
            associations.push(DraftingProgramSelectionAssociation {
                location,
                need,
                row,
            });
        }
        Ok(CheckedDraftingProgramDirectSelections {
            budget: work.into_budget(),
            associations,
            rows: kinds,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::{merge_fields, Key, Location, Need};
    use crate::construction::{
        DraftingNativeSelectionField as N, DraftingNativeSemanticTargetRole as R,
        DraftingNotchProfileRole as P, DraftingSemanticSelectionField as S,
    };
    #[test]
    fn semantic_selection_merge_agrees_with_stable_sort_and_preserves_local_fields() {
        let rule = Location::Semantic {
            ordinal: 1,
            field: S::LogicalRule,
        };
        let input = Location::Semantic {
            ordinal: 1,
            field: S::LogicalInput { index: 0 },
        };
        let target = Location::Native {
            position: 1,
            field: N::Target(R::Grainline),
        };
        let style = Location::Native {
            position: 3,
            field: N::NotchProfile(P::Style),
        };
        let depth = Location::Native {
            position: 3,
            field: N::NotchProfile(P::SampleDepth),
        };
        let semantic = [
            (2, rule, Need::Required(Key::Rule { ordinal: 1 })),
            (2, input, Need::NoDirectSelection),
        ];
        let native = [
            (1, target, Need::DeferredTargetProfile(target)),
            (3, style, Need::Required(Key::Profile(style))),
            (3, depth, Need::Required(Key::Profile(depth))),
        ];
        for (left, right) in [
            (semantic.as_slice(), native.as_slice()),
            (&[], native.as_slice()),
            (semantic.as_slice(), &[]),
            (&[][..], &[][..]),
        ] {
            let mut oracle: Vec<_> = left.iter().chain(right).copied().collect();
            oracle.sort_by_key(|(position, _, _)| *position);
            let expected: Vec<_> = oracle
                .into_iter()
                .map(|(_, location, need)| (location, need))
                .collect();
            let actual: Vec<_> =
                merge_fields(left.iter().copied(), right.iter().copied()).collect();
            assert_eq!(actual, expected);
        }
    }
}
