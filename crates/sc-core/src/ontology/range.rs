//! Whole-interval reference resolution (ontology §1.1 / §4.2, defect D55).
//!
//! Two surviving endpoints do not prove a surviving interior. This fold carries each positive-length
//! portion through every topology edit and preserves lost portions as visible repair tasks. It uses
//! exact interval intersection rather than sampling. Neither coverage nor endpoint resolution proves
//! geometric continuity, and neither grants release approval.

use core::fmt;

use sc_units::UnitError;

use super::{
    Direction, EdgeRef, IdentityLedger, JournalEntry, OrphaningEdit, Param, Rational, Resolution,
    TopologyEdit,
};

/// An authored positive-length interval along an edge, with exact ascending bounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EdgeRange {
    edge: EdgeRef,
    from: Param,
    to: Param,
}

/// A range must have positive parameter length; zero-length anchors use point references.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RangeError {
    /// The requested lower bound.
    pub from: Param,
    /// The requested upper bound, which was not greater than `from`.
    pub to: Param,
}

impl fmt::Display for RangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "edge range requires from < to, got {} through {}",
            self.from, self.to
        )
    }
}

impl std::error::Error for RangeError {}

impl EdgeRange {
    /// Build a positive-length interval; existence is checked by the consuming object or query.
    ///
    /// # Errors
    /// Returns [`RangeError`] for equal or descending bounds.
    pub fn new(edge: EdgeRef, from: Param, to: Param) -> Result<Self, RangeError> {
        if from >= to {
            Err(RangeError { from, to })
        } else {
            Ok(Self { edge, from, to })
        }
    }

    /// The entire edge, in its authored parameter frame.
    #[must_use]
    pub const fn whole(edge: EdgeRef) -> Self {
        Self {
            edge,
            from: Param::START,
            to: Param::END,
        }
    }

    /// Held edge identity, never rewritten by a query.
    #[must_use]
    pub const fn edge(self) -> EdgeRef {
        self.edge
    }

    /// Lower parameter bound.
    #[must_use]
    pub const fn from(self) -> Param {
        self.from
    }

    /// Upper parameter bound.
    #[must_use]
    pub const fn to(self) -> Param {
        self.to
    }
}

/// A surviving interval and its direction relative to the held range's forward traversal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResolvedRange {
    range: EdgeRange,
    direction: Direction,
}

impl ResolvedRange {
    /// The surviving edge and ascending local parameter bounds.
    #[must_use]
    pub const fn range(self) -> EdgeRange {
        self.range
    }

    /// Whether traversing the held range runs forward or backward on this fragment.
    #[must_use]
    pub const fn direction(self) -> Direction {
        self.direction
    }
}

/// Why a range portion cannot resolve.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RangeIssue {
    /// The held edge was never created in this ledger; no edit can be cited.
    UnknownSource,
    /// A recorded topology edit orphaned the interval or refused exact recomputation.
    Orphaned(OrphaningEdit),
}

/// A visible repair for a positive-length portion of a held range.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RangeRepairTask {
    held: EdgeRange,
    affected: ResolvedRange,
    issue: RangeIssue,
}

impl RangeRepairTask {
    /// The original authored range, rather than an intermediate identity.
    #[must_use]
    pub const fn held(&self) -> EdgeRange {
        self.held
    }

    /// The affected local portion at the edit that orphaned it.
    #[must_use]
    pub const fn affected(&self) -> ResolvedRange {
        self.affected
    }

    /// Candidate interval mappings. All current range-repair causes have no unambiguous candidate;
    /// endpoint ambiguity carries candidates through the separate point-resolution contract.
    #[must_use]
    pub const fn candidates(&self) -> &[ResolvedRange] {
        &[]
    }

    /// Missing-source or edit-specific typed cause.
    #[must_use]
    pub const fn issue(&self) -> RangeIssue {
        self.issue
    }
}

impl fmt::Display for RangeRepairTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "range repair: {} [{} through {}], affected {} [{} through {}]: ",
            self.held.edge,
            self.held.from,
            self.held.to,
            self.affected.range.edge,
            self.affected.range.from,
            self.affected.range.to
        )?;
        match self.issue {
            RangeIssue::UnknownSource => f.write_str("source edge was never created in the ledger"),
            RangeIssue::Orphaned(edit) => write!(f, "{edit}"),
        }
    }
}

/// One interval portion, retained in the held range's original forward-traversal order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RangePortion {
    /// A live interval, with direction relative to the original traversal.
    Resolved(ResolvedRange),
    /// Lost or uncomputable content; later edits never silently reattach it.
    Unresolved(RangeRepairTask),
}

/// Whole-interval evidence and the separate point-resolution answers for its endpoints.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RangeResolution {
    held: EdgeRange,
    portions: Vec<RangePortion>,
    start: Resolution,
    end: Resolution,
}

impl RangeResolution {
    /// Original authored reference.
    #[must_use]
    pub const fn held(&self) -> EdgeRange {
        self.held
    }

    /// Ordered live portions and visible repairs; no deleted gap is elided.
    #[must_use]
    pub fn portions(&self) -> &[RangePortion] {
        &self.portions
    }

    /// No positive-length portion was lost or failed arithmetic.
    ///
    /// This does not mean endpoints have unique resolutions or that geometry is valid.
    #[must_use]
    pub fn has_full_coverage(&self) -> bool {
        self.portions
            .iter()
            .all(|part| matches!(part, RangePortion::Resolved(_)))
    }

    /// Original lower endpoint, including split choices or offset ambiguity.
    #[must_use]
    pub const fn start(&self) -> &Resolution {
        &self.start
    }

    /// Original upper endpoint, including split choices or offset ambiguity.
    #[must_use]
    pub const fn end(&self) -> &Resolution {
        &self.end
    }

    /// Every unresolved interval, derived from the journal.
    pub fn repairs(&self) -> impl Iterator<Item = &RangeRepairTask> {
        self.portions.iter().filter_map(|part| match part {
            RangePortion::Unresolved(task) => Some(task),
            RangePortion::Resolved(_) => None,
        })
    }
}

impl IdentityLedger {
    /// Fold the entire range through every edit, without sampling or mutating authored content.
    ///
    /// Endpoint point queries retain the existing point contract. For an unknown source they are
    /// still point-fold pass-through answers; the range's [`RangeIssue::UnknownSource`] blocks coverage.
    #[must_use]
    pub fn resolve_range(&self, held: EdgeRange) -> RangeResolution {
        let original = ResolvedRange {
            range: held,
            direction: Direction::Original,
        };
        let known = self.journal().iter().any(|entry| match entry.edit() {
            TopologyEdit::Declared { edges } => edges.contains(&held.edge),
            TopologyEdit::Split { first, second, .. } => {
                *first == held.edge || *second == held.edge
            }
            TopologyEdit::Merge { merged, .. } => *merged == held.edge,
            TopologyEdit::Offset { fragments, .. } => {
                fragments.iter().any(|item| item.edge() == held.edge)
            }
            TopologyEdit::Reverse { .. } | TopologyEdit::Delete { .. } => false,
        });
        let mut portions = if known {
            vec![RangePortion::Resolved(original)]
        } else {
            vec![repair(held, original, RangeIssue::UnknownSource)]
        };
        for entry in self.journal() {
            let mut next = Vec::new();
            for part in portions {
                match part {
                    RangePortion::Unresolved(_) => next.push(part),
                    RangePortion::Resolved(current) => match step(held, current, entry) {
                        Ok(mapped) => next.extend(mapped),
                        Err(cause) => next.push(repair(
                            held,
                            current,
                            RangeIssue::Orphaned(OrphaningEdit::RecomputationFailed {
                                operation: entry.operation(),
                                edge: current.range.edge,
                                cause,
                            }),
                        )),
                    },
                }
            }
            portions = next;
        }
        RangeResolution {
            held,
            portions,
            start: self.resolve(held.edge, held.from),
            end: self.resolve(held.edge, held.to),
        }
    }
}

fn repair(held: EdgeRange, affected: ResolvedRange, issue: RangeIssue) -> RangePortion {
    RangePortion::Unresolved(RangeRepairTask {
        held,
        affected,
        issue,
    })
}

fn param(value: Rational) -> Result<Param, UnitError> {
    Param::new(value).map_err(|_| UnitError::DomainExceeded {
        operation: "IdentityLedger::resolve_range",
        kind: "range parameter",
        value: i128::from(value.numerator()),
        limit: i128::from(value.denominator()),
    })
}

fn mapped(
    edge: EdgeRef,
    from: Rational,
    to: Rational,
    direction: Direction,
) -> Result<RangePortion, UnitError> {
    let from = param(from)?;
    let to = param(to)?;
    let range = EdgeRange::new(edge, from, to).map_err(|_| UnitError::DomainExceeded {
        operation: "IdentityLedger::resolve_range",
        kind: "positive range width",
        value: 0,
        limit: 1,
    })?;
    Ok(RangePortion::Resolved(ResolvedRange { range, direction }))
}

fn affine(value: Param, from: Param, to: Param) -> Result<Rational, UnitError> {
    value
        .as_rational()
        .checked_sub(from.as_rational())?
        .checked_div(to.as_rational().checked_sub(from.as_rational())?)
}

fn step(
    held: EdgeRange,
    current: ResolvedRange,
    entry: &JournalEntry,
) -> Result<Vec<RangePortion>, UnitError> {
    let EdgeRange { edge, from, to } = current.range;
    let direction = current.direction;
    match entry.edit() {
        TopologyEdit::Split {
            source,
            at,
            first,
            second,
        } if *source == edge => {
            let mut parts = Vec::new();
            if from < *at {
                parts.push(mapped(
                    *first,
                    from.as_rational().checked_div(at.as_rational())?,
                    to.min(*at).as_rational().checked_div(at.as_rational())?,
                    direction,
                )?);
            }
            if to > *at {
                parts.push(mapped(
                    *second,
                    affine(from.max(*at), *at, Param::END)?,
                    affine(to, *at, Param::END)?,
                    direction,
                )?);
            }
            if direction == Direction::Reversed {
                parts.reverse();
            }
            Ok(parts)
        }
        TopologyEdit::Merge {
            first,
            first_length,
            second,
            second_length,
            merged,
        } if *first == edge || *second == edge => {
            let l1 = Rational::from_integer(first_length.as_micrometres());
            let l2 = Rational::from_integer(second_length.as_micrometres());
            let total = l1.checked_add(l2)?;
            let map = |t: Param| {
                if *first == edge {
                    t.as_rational().checked_mul(l1)?.checked_div(total)
                } else {
                    l1.checked_add(t.as_rational().checked_mul(l2)?)?
                        .checked_div(total)
                }
            };
            Ok(vec![mapped(*merged, map(from)?, map(to)?, direction)?])
        }
        TopologyEdit::Reverse { edge: reversed } if *reversed == edge => Ok(vec![mapped(
            edge,
            to.as_rational().one_minus()?,
            from.as_rational().one_minus()?,
            direction.reversed(),
        )?]),
        TopologyEdit::Delete { edge: deleted } if *deleted == edge => Ok(vec![repair(
            held,
            current,
            RangeIssue::Orphaned(OrphaningEdit::Deleted {
                operation: entry.operation(),
                edge,
            }),
        )]),
        TopologyEdit::Offset { source, fragments } if *source == edge => {
            let mut parts = Vec::new();
            let mut cursor = from;
            let gap = |low, high| {
                repair(
                    held,
                    ResolvedRange {
                        range: EdgeRange {
                            edge,
                            from: low,
                            to: high,
                        },
                        direction,
                    },
                    RangeIssue::Orphaned(OrphaningEdit::OffsetUnmapped {
                        operation: entry.operation(),
                        source: edge,
                    }),
                )
            };
            for fragment in fragments {
                let low = from.max(fragment.from());
                let high = to.min(fragment.to());
                if low >= high {
                    continue;
                }
                if cursor < low {
                    parts.push(gap(cursor, low));
                }
                parts.push(mapped(
                    fragment.edge(),
                    affine(low, fragment.from(), fragment.to())?,
                    affine(high, fragment.from(), fragment.to())?,
                    direction,
                )?);
                cursor = high;
            }
            if cursor < to {
                parts.push(gap(cursor, to));
            }
            if direction == Direction::Reversed {
                parts.reverse();
            }
            Ok(parts)
        }
        _ => Ok(vec![RangePortion::Resolved(current)]),
    }
}

// Shared structural ownership: endpoints alone cannot certify a requested material interval.
pub(crate) fn range_is_owned(
    range: &RangeResolution,
    piece: &super::Piece,
    ledger: &IdentityLedger,
) -> bool {
    let mut owned = Vec::new();
    for (_, resolution) in piece.range_resolutions(ledger) {
        for portion in resolution.portions() {
            if let RangePortion::Resolved(part) = portion {
                owned.push(part.range());
            }
        }
    }
    range.portions().iter().all(|part| {
        let RangePortion::Resolved(part) = part else {
            return false;
        };
        let requested = part.range();
        let mut intervals = owned
            .iter()
            .filter(|range| range.edge() == requested.edge())
            .copied()
            .collect::<Vec<_>>();
        intervals.sort_by_key(|range| range.from());
        let mut covered = requested.from();
        for interval in intervals {
            if interval.from() > covered {
                break;
            }
            covered = covered.max(interval.to());
            if covered >= requested.to() {
                return true;
            }
        }
        false
    })
}

#[cfg(test)]
mod domain_context_contracts {
    use super::*;
    use crate::ontology::{EntityId, LocalTag};

    #[test]
    #[allow(clippy::expect_used)] // This exact rational is valid; only Param's narrower domain refuses it.
    fn range_parameter_totality_guard_names_the_resolver() {
        assert_eq!(
            param(Rational::new(2, 1).expect("valid rational")).err(),
            Some(UnitError::DomainExceeded {
                operation: "IdentityLedger::resolve_range",
                kind: "range parameter",
                value: 2,
                limit: 1,
            })
        );
    }
    #[test]
    fn zero_range_width_totality_guard_names_the_resolver() {
        let edge = EdgeRef::new(EntityId::from_bits(1), LocalTag::FIRST);
        assert_eq!(
            mapped(edge, Rational::ZERO, Rational::ZERO, Direction::Original).err(),
            Some(UnitError::DomainExceeded {
                operation: "IdentityLedger::resolve_range",
                kind: "positive range width",
                value: 0,
                limit: 1,
            })
        );
    }
}
