//! One allocation-free journal step, shared by full and bounded point consumers.

use core::cmp::Ordering;
use sc_units::UnitError;

use super::{
    to_param, JournalEntry, OffsetFragment, OrphaningEdit, Rational, ResolvedRef, TopologyEdit,
};

/// Immediate evidence only; following split branches belongs to the consuming resolver.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PointStep {
    Current(ResolvedRef),
    Split {
        first: ResolvedRef,
        second: ResolvedRef,
    },
    Repair {
        cause: OrphaningEdit,
        candidates: [Option<ResolvedRef>; 2],
    },
}

pub(super) fn step(current: ResolvedRef, entry: &JournalEntry) -> PointStep {
    // Exhaustive private state views and Copy output force new fields/owned state to be reviewed.
    let ResolvedRef {
        edge,
        param,
        direction,
    } = current;
    let JournalEntry { operation, edit } = entry;
    let failed = |cause| PointStep::Repair {
        cause: OrphaningEdit::RecomputationFailed {
            operation: *operation,
            edge,
            cause,
        },
        candidates: [None, None],
    };
    let mapped = |next_edge, value: Result<Rational, UnitError>, next_direction| match value
        .and_then(to_param)
    {
        Ok(local) => PointStep::Current(ResolvedRef::new(next_edge, local, next_direction)),
        Err(cause) => failed(cause),
    };
    match edit {
        TopologyEdit::Declared { edges: _ } => PointStep::Current(current),
        TopologyEdit::Split {
            source,
            at,
            first,
            second,
        } => {
            if *source != edge {
                return PointStep::Current(current);
            }
            let value = param.as_rational();
            let split = at.as_rational();
            match value.cmp(&split) {
                Ordering::Less => mapped(*first, value.checked_div(split), direction),
                Ordering::Greater => mapped(
                    *second,
                    value.checked_relative_progress(split, Rational::ONE),
                    direction,
                ),
                Ordering::Equal => PointStep::Split {
                    first: ResolvedRef::new(*first, super::Param::END, direction),
                    second: ResolvedRef::new(*second, super::Param::START, direction),
                },
            }
        }
        TopologyEdit::Merge {
            first,
            first_length,
            second,
            second_length,
            merged,
        } => {
            if *first != edge && *second != edge {
                return PointStep::Current(current);
            }
            mapped(
                *merged,
                param.as_rational().checked_merge_progress(
                    first_length.as_micrometres(),
                    second_length.as_micrometres(),
                    *first == edge,
                ),
                direction,
            )
        }
        TopologyEdit::Reverse { edge: reversed } => {
            if *reversed != edge {
                return PointStep::Current(current);
            }
            mapped(edge, param.as_rational().one_minus(), direction.reversed())
        }
        TopologyEdit::Delete { edge: deleted } => {
            if *deleted != edge {
                return PointStep::Current(current);
            }
            PointStep::Repair {
                cause: OrphaningEdit::Deleted {
                    operation: *operation,
                    edge,
                },
                candidates: [None, None],
            }
        }
        TopologyEdit::Offset { source, fragments } => {
            if *source != edge {
                return PointStep::Current(current);
            }
            let mut candidate = None;
            for fragment in fragments {
                let OffsetFragment {
                    edge: fragment_edge,
                    from,
                    to,
                } = *fragment;
                if param < from || param > to {
                    continue;
                }
                let local = match param
                    .as_rational()
                    .checked_relative_progress(from.as_rational(), to.as_rational())
                    .and_then(to_param)
                {
                    Ok(local) => local,
                    Err(cause) => return failed(cause),
                };
                let position = ResolvedRef::new(fragment_edge, local, direction);
                if let Some(first) = candidate {
                    // Validated positive nonoverlapping intervals contain a point at most twice,
                    // exactly on a shared boundary. Preserve their authored order without a Vec.
                    return PointStep::Repair {
                        cause: OrphaningEdit::OffsetAmbiguous {
                            operation: *operation,
                            source: *source,
                        },
                        candidates: [Some(first), Some(position)],
                    };
                }
                candidate = Some(position);
            }
            match candidate {
                Some(position) => PointStep::Current(position),
                None => PointStep::Repair {
                    cause: OrphaningEdit::OffsetUnmapped {
                        operation: *operation,
                        source: *source,
                    },
                    candidates: [None, None],
                },
            }
        }
    }
}
