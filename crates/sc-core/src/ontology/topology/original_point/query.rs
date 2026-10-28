//! Exact backward numeric projection on an already admitted immutable journal.
use super::super::{Direction, JournalEntry, OffsetFragment, Rational, TopologyEdit};
use super::{EdgeRef, EntityId, OriginalPointAdmission, Param, ResolvedRef};
use core::{cmp::Ordering, fmt};
use sc_units::UnitError;
mod wide;
use wide::{Wide, Workspace};

/// Opaque canonical exact coordinate in the named source edge's creation frame.
/// No dependency integer type escapes. Equality is exact;Debug reports only component bit counts.
#[derive(PartialEq, Eq)]
pub struct OriginalCoordinate {
    exact: Wide,
}
impl OriginalCoordinate {
    /// Actual numerator/positive-denominator bit counts,including zero numerator's zero bits.
    #[must_use]
    pub fn component_bit_counts(&self) -> (u64, u64) {
        (self.exact.num.bits(), self.exact.den.bits())
    }
    /// Explicit exact u128 representation;refuses only if either original component exceeds it.
    pub fn as_u128_ratio(&self) -> Result<(u128, u128), UnitError> {
        let overflow = |_| UnitError::Overflow {
            operation: "OriginalCoordinate::as_u128_ratio",
        };
        Ok((
            u128::try_from(&self.exact.num).map_err(overflow)?,
            u128::try_from(&self.exact.den).map_err(overflow)?,
        ))
    }
    /// Explicit bounded reference representation;wide valid coordinates remain valid on refusal.
    pub fn as_param(&self) -> Result<Param, UnitError> {
        let overflow = || UnitError::Overflow {
            operation: "OriginalCoordinate::as_param",
        };
        let n = i64::try_from(&self.exact.num).map_err(|_| overflow())?;
        let d = i64::try_from(&self.exact.den).map_err(|_| overflow())?;
        Param::new(Rational::new(n, d)?).map_err(|_| UnitError::DomainExceeded {
            operation: "OriginalCoordinate::as_param",
            kind: "edge parameter",
            value: i128::from(n),
            limit: i128::from(d),
        })
    }
}
impl fmt::Debug for OriginalCoordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            exact: Wide { num, den },
        } = self;
        f.debug_struct("OriginalCoordinate")
            .field("numerator_bits", &num.bits())
            .field("denominator_bits", &den.bits())
            .finish()
    }
}

/// One numeric original coordinate and its mapping direction relative to the current edge frame.
/// This direction is distinct from the held reference's own current-resolution direction.
#[derive(Debug, PartialEq, Eq)]
pub struct OriginalPointCandidate {
    coordinate: OriginalCoordinate,
    direction: Direction,
}
impl OriginalPointCandidate {
    /// Exact coordinate in the named original source's creation frame.
    #[must_use]
    pub const fn coordinate(&self) -> &OriginalCoordinate {
        &self.coordinate
    }
    /// Source-creation-frame mapping direction relative to the observed current edge frame.
    #[must_use]
    pub const fn direction(&self) -> Direction {
        self.direction
    }
}

/// Complete original-coordinate candidate set tied to the actual admitted immutable journal.
pub struct OriginalPointProjection<'a> {
    admission: OriginalPointAdmission<'a>,
    candidates: Vec<OriginalPointCandidate>,
}
impl<'a> OriginalPointProjection<'a> {
    /// Actual source/held/current/creation/limits evidence used by this projection.
    #[must_use]
    pub const fn admission(&self) -> OriginalPointAdmission<'a> {
        self.admission
    }
    /// All distinct numeric coordinate/direction candidates in stable traversal order.
    #[must_use]
    pub fn candidates(&self) -> &[OriginalPointCandidate] {
        &self.candidates
    }
    /// The sole coordinate/direction candidate,if exactly one exists;no branch is selected silently.
    #[must_use]
    pub fn unique(&self) -> Option<&OriginalPointCandidate> {
        match self.candidates.as_slice() {
            [only] => Some(only),
            _ => None,
        }
    }
}
impl fmt::Debug for OriginalPointProjection<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            admission,
            candidates,
        } = self;
        f.debug_struct("OriginalPointProjection")
            .field("admission", admission)
            .field("candidate_count", &candidates.len())
            .finish()
    }
}

/// Concrete inverse candidate population or arithmetic refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OriginalPointProjectionIssue {
    /// No predecessor in the named source creation frame;the current reference itself remains valid.
    NoOrigin,
    /// New branch would exceed simultaneously retained state limit,before allocating its state.
    Candidates {
        /// Actual inverse branch operation.
        operation: EntityId,
        /// States already retained,including the active input state.
        retained: usize,
        /// Additional states requested by the branch.
        requested: usize,
        /// Explicit selected limit.
        limit: usize,
    },
    /// Candidate population cannot be represented.
    CountOverflow {
        /// Actual inverse operation.
        operation: EntityId,
    },
    /// Actual exact-arithmetic or integer-reservation refusal.
    Numeric {
        /// Actual inverse operation.
        operation: EntityId,
        /// Current predecessor edge being mapped.
        edge: EdgeRef,
        /// Concrete numeric cause,also the standard error source.
        cause: UnitError,
    },
    /// A deleted predecessor was encountered;never fabricate an inverse through it.
    DeletedPredecessor {
        /// Actual deletion operation.
        operation: EntityId,
        /// Actual deleted predecessor.
        edge: EdgeRef,
    },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FailureContext {
    source: EdgeRef,
    reference: EdgeRef,
    param: Param,
    current: ResolvedRef,
}
/// Original source/held/current evidence plus the concrete first projection failure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OriginalPointProjectionError {
    inner: Box<Failure>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Failure {
    context: FailureContext,
    issue: OriginalPointProjectionIssue,
}
impl OriginalPointProjectionError {
    /// Named original source edge.
    #[must_use]
    pub fn source_edge(&self) -> EdgeRef {
        self.inner.context.source
    }
    /// Original held reference edge.
    #[must_use]
    pub fn reference(&self) -> EdgeRef {
        self.inner.context.reference
    }
    /// Original held parameter.
    #[must_use]
    pub fn param(&self) -> Param {
        self.inner.context.param
    }
    /// Actual unique current point observed by admission.
    #[must_use]
    pub fn current(&self) -> ResolvedRef {
        self.inner.context.current
    }
    /// Concrete failure;no partial candidate set is published.
    #[must_use]
    pub fn issue(&self) -> &OriginalPointProjectionIssue {
        &self.inner.issue
    }
}
impl fmt::Display for OriginalPointProjectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "original source {},held {} at {}: {:?}",
            self.inner.context.source,
            self.inner.context.reference,
            self.inner.context.param,
            self.inner.issue
        )
    }
}
impl std::error::Error for OriginalPointProjectionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.inner.issue {
            OriginalPointProjectionIssue::Numeric { cause, .. } => Some(cause),
            _ => None,
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
struct State {
    edge: EdgeRef,
    coordinate: OriginalCoordinate,
    direction: Direction,
}

impl<'a> OriginalPointAdmission<'a> {
    /// Project the admitted current point backward to the named source edge's actual creation frame.
    /// Retain every distinct coordinate/direction state. Equal numeric states coalesce without
    /// rewriting held references;current reference choices were already refused by admission.
    /// Candidate caps cover both work queues and the active state;checks precede new branch storage.
    /// No partial result escapes on failure. Physical/copy/range/quantity checks remain separate.
    ///
    /// ```
    /// use core::num::{NonZeroU64, NonZeroUsize};
    /// use sc_core::ontology::{DeterministicIdGenerator, IdentityLedger, OriginalPointLimits,
    ///     Param, Rational, UniquePointLimits};
    /// let mut ledger = IdentityLedger::new();
    /// let mut ids = DeterministicIdGenerator::new();
    /// let source = ledger.declare_edges(&mut ids, 1)?.remove(0);
    /// let half = Param::new(Rational::new(1, 2)?)?;
    /// let fragment = ledger.split(&mut ids, source, half)?.first();
    /// let one = NonZeroUsize::new(1).ok_or("positive")?;
    /// let two = NonZeroUsize::new(2).ok_or("positive")?;
    /// let limits = OriginalPointLimits::new(UniquePointLimits::new(two, one), one,
    ///     NonZeroU64::new(762).ok_or("positive")?);
    /// let current = Param::new(Rational::new(1, i64::MAX)?)?;
    /// let result = ledger.admit_original_point(fragment, current, source, limits)?.project()?;
    /// let original = result.unique().ok_or("one coordinate/direction origin")?.coordinate();
    /// assert_eq!(original.as_u128_ratio()?, (1, 18446744073709551614));
    /// assert!(original.as_param().is_err());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// ```compile_fail
    /// use core::num::{NonZeroU64, NonZeroUsize};
    /// use sc_core::ontology::{DeterministicIdGenerator, IdentityLedger, OriginalPointLimits, Param, UniquePointLimits};
    /// let mut ledger = IdentityLedger::new();
    /// let mut ids = DeterministicIdGenerator::new();
    /// let edge = ledger.declare_edges(&mut ids, 1)?.remove(0);
    /// let one = NonZeroUsize::new(1).ok_or("positive")?;
    /// let limits = OriginalPointLimits::new(UniquePointLimits::new(one, one), one,
    ///     NonZeroU64::new(508).ok_or("positive")?);
    /// let result = ledger.admit_original_point(edge, Param::START, edge, limits)?.project()?;
    /// ledger.reverse(&mut ids, edge)?;
    /// assert_eq!(result.candidates().len(), 1);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn project(self) -> Result<OriginalPointProjection<'a>, OriginalPointProjectionError> {
        let Self {
            ledger,
            reference,
            param,
            source,
            creation: _,
            creation_index,
            current,
            limits,
            component_bits,
            integer_bits,
        } = self;
        let fail = |issue| OriginalPointProjectionError {
            inner: Box::new(Failure {
                context: FailureContext {
                    source,
                    reference,
                    param,
                    current,
                },
                issue,
            }),
        };
        let workspace = Workspace {
            component_bits,
            integer_bits,
            operation: "OriginalPointAdmission::project",
        };
        let initial = workspace.reference(current.param()).map_err(|cause| {
            fail(OriginalPointProjectionIssue::Numeric {
                operation: current.edge().creator(),
                edge: current.edge(),
                cause,
            })
        })?;
        let mut states = vec![State {
            edge: current.edge(),
            coordinate: OriginalCoordinate { exact: initial },
            direction: Direction::Original,
        }];
        for (_, entry) in ledger
            .journal()
            .iter()
            .enumerate()
            .rev()
            .take_while(|(index, _)| *index > creation_index)
        {
            states.reverse();
            let mut next = Vec::new();
            while let Some(state) = states.pop() {
                let retained = states
                    .len()
                    .checked_add(next.len())
                    .and_then(|n| n.checked_add(1))
                    .ok_or_else(|| {
                        fail(OriginalPointProjectionIssue::CountOverflow {
                            operation: entry.operation(),
                        })
                    })?;
                let mapped = inverse_step(
                    state,
                    entry,
                    workspace,
                    retained,
                    limits.candidates().get(),
                    fail,
                )?;
                for candidate in mapped.into_iter().flatten() {
                    if !next.iter().any(|prior| prior == &candidate) {
                        next.push(candidate);
                    }
                }
            }
            states = next;
            if states.is_empty() {
                break;
            }
        }
        let candidates = states
            .into_iter()
            .filter(|state| state.edge == source)
            .map(|state| OriginalPointCandidate {
                coordinate: state.coordinate,
                direction: state.direction,
            })
            .collect::<Vec<_>>();
        if candidates.is_empty() {
            return Err(fail(OriginalPointProjectionIssue::NoOrigin));
        }
        Ok(OriginalPointProjection {
            admission: self,
            candidates,
        })
    }
}

fn inverse_step<F>(
    mut state: State,
    entry: &JournalEntry,
    workspace: Workspace,
    retained: usize,
    limit: usize,
    fail: F,
) -> Result<[Option<State>; 2], OriginalPointProjectionError>
where
    F: Fn(OriginalPointProjectionIssue) -> OriginalPointProjectionError + Copy,
{
    let numeric = |cause| {
        fail(OriginalPointProjectionIssue::Numeric {
            operation: entry.operation(),
            edge: state.edge,
            cause,
        })
    };
    match entry.edit() {
        TopologyEdit::Declared { edges } => {
            if edges.contains(&state.edge) {
                return Ok([None, None]);
            }
        }
        TopologyEdit::Split {
            source,
            at,
            first,
            second,
        } => {
            if state.edge == *first || state.edge == *second {
                state.coordinate.exact = workspace
                    .split(state.coordinate.exact, *at, state.edge == *second)
                    .map_err(numeric)?;
                state.edge = *source;
            }
        }
        TopologyEdit::Merge {
            first,
            first_length,
            second,
            second_length,
            merged,
        } => {
            if state.edge == *merged {
                let l1 = first_length.as_micrometres().unsigned_abs();
                let l2 = second_length.as_micrometres().unsigned_abs();
                match workspace
                    .merge_side(&state.coordinate.exact, l1, l2)
                    .map_err(numeric)?
                {
                    Ordering::Equal => {
                        let required = retained.checked_add(1).ok_or_else(|| {
                            fail(OriginalPointProjectionIssue::CountOverflow {
                                operation: entry.operation(),
                            })
                        })?;
                        if required > limit {
                            return Err(fail(OriginalPointProjectionIssue::Candidates {
                                operation: entry.operation(),
                                retained,
                                requested: 1,
                                limit,
                            }));
                        }
                        let other = State {
                            edge: *second,
                            coordinate: OriginalCoordinate {
                                exact: workspace.endpoint(false).map_err(numeric)?,
                            },
                            direction: state.direction,
                        };
                        state.coordinate.exact = workspace.endpoint(true).map_err(numeric)?;
                        state.edge = *first;
                        return Ok([Some(state), Some(other)]);
                    }
                    side => {
                        let second_side = side == Ordering::Greater;
                        state.coordinate.exact = workspace
                            .merge(state.coordinate.exact, l1, l2, second_side)
                            .map_err(numeric)?;
                        state.edge = if second_side { *second } else { *first };
                    }
                }
            }
        }
        TopologyEdit::Reverse { edge } => {
            if state.edge == *edge {
                state.coordinate.exact =
                    workspace.reverse(state.coordinate.exact).map_err(numeric)?;
                state.direction = state.direction.reversed();
            }
        }
        TopologyEdit::Delete { edge } => {
            if state.edge == *edge {
                return Err(fail(OriginalPointProjectionIssue::DeletedPredecessor {
                    operation: entry.operation(),
                    edge: *edge,
                }));
            }
        }
        TopologyEdit::Offset { source, fragments } => {
            if let Some(fragment) = fragments
                .iter()
                .find(|fragment| fragment.edge() == state.edge)
            {
                let OffsetFragment { edge: _, from, to } = *fragment;
                state.coordinate.exact = workspace
                    .offset(state.coordinate.exact, from, to)
                    .map_err(numeric)?;
                state.edge = *source;
            }
        }
    }
    Ok([Some(state), None])
}

#[cfg(test)]
mod contracts {
    use crate::ontology::{
        DeterministicIdGenerator, IdentityLedger, OffsetInterval, OriginalPointLimits, Param,
        Rational, UniquePointLimits,
    };
    use core::num::{NonZeroU64, NonZeroUsize};

    #[test]
    fn actual_offset_projection_matches_independent_fraction_components_beyond_u128() {
        let mut seen = 0;
        let mut beyond = false;
        let mut widest = 0;
        for line in include_str!("../../../../tests/fixtures/original_projection.tsv")
            .lines()
            .filter(|line| !line.starts_with('#'))
        {
            let fields = line.split('\t').collect::<Vec<_>>();
            let field = |index: usize| *fields.get(index).expect("frozen column");
            let position = |n, d| {
                Param::new(
                    Rational::new(
                        field(n).parse().expect("i64 numerator"),
                        field(d).parse().expect("i64 denominator"),
                    )
                    .expect("exact input"),
                )
                .expect("unit input")
            };
            let mut ledger = IdentityLedger::new();
            let mut ids = DeterministicIdGenerator::new();
            let source = *ledger
                .declare_edges(&mut ids, 1)
                .expect("actual source")
                .first()
                .expect("one edge");
            let fragment = *ledger
                .offset(
                    &mut ids,
                    source,
                    &[OffsetInterval::new(position(3, 4), position(5, 6))
                        .expect("actual interval")],
                )
                .expect("actual offset")
                .first()
                .expect("one fragment");
            let one = NonZeroUsize::new(1).expect("positive");
            let two = NonZeroUsize::new(2).expect("positive");
            let limits = OriginalPointLimits::new(
                UniquePointLimits::new(two, two),
                one,
                NonZeroU64::new(762).expect("positive"),
            );
            let projection = ledger
                .admit_original_point(fragment, position(1, 2), source, limits)
                .expect("admission")
                .project()
                .expect("actual exact inverse");
            let coordinate = projection.unique().expect("one origin").coordinate();
            assert_eq!(coordinate.exact.num.to_str_radix(10), field(7));
            assert_eq!(coordinate.exact.den.to_str_radix(10), field(8));
            widest = widest
                .max(coordinate.component_bit_counts().0)
                .max(coordinate.component_bit_counts().1);
            if coordinate.component_bit_counts().0 > 128
                || coordinate.component_bit_counts().1 > 128
            {
                assert!(coordinate.as_u128_ratio().is_err());
                beyond = true;
            }
            seen += 1;
        }
        assert_eq!(seen, 6);
        assert_eq!(widest, 188, "published wide-coordinate control");
        assert!(
            beyond,
            "u128 narrowing cannot implement this original-coordinate contract"
        );
    }
    #[test]
    fn admitted_original_coordinate_ordering_matches_independent_fraction_cases() {
        use super::wide::Workspace;
        use core::cmp::Ordering;
        let mut seen = 0;
        for row in include_str!("../../../../tests/fixtures/original_comparison.tsv")
            .lines()
            .filter(|row| !row.starts_with('#'))
        {
            let fields = row.split('\t').collect::<Vec<_>>();
            assert_eq!(fields.len(), 17);
            let field = |index: usize| *fields.get(index).expect("frozen comparison column");
            let param = |n, d| {
                Param::new(
                    Rational::new(
                        field(n).parse().expect("i64 numerator"),
                        field(d).parse().expect("i64 denominator"),
                    )
                    .expect("exact input"),
                )
                .expect("unit input")
            };
            let order = |name| {
                [
                    ("Less", Ordering::Less),
                    ("Equal", Ordering::Equal),
                    ("Greater", Ordering::Greater),
                ]
                .into_iter()
                .find_map(|(label, value)| (label == name).then_some(value))
                .expect("frozen exact ordering")
            };
            let mut ledger = IdentityLedger::new();
            let mut ids = DeterministicIdGenerator::new();
            let source = ledger
                .declare_edges(&mut ids, 1)
                .expect("actual source")
                .remove(0);
            let reversed = field(1) == "1";
            if reversed {
                ledger
                    .reverse(&mut ids, source)
                    .expect("actual frame reversal");
            }
            let fragment = ledger
                .offset(
                    &mut ids,
                    source,
                    &[OffsetInterval::new(param(2, 3), param(4, 5)).expect("actual interval")],
                )
                .expect("actual offset")
                .remove(0);
            let four = NonZeroUsize::new(4).expect("positive");
            let limits = OriginalPointLimits::new(
                UniquePointLimits::new(four, four),
                NonZeroUsize::new(1).expect("positive"),
                NonZeroU64::new(10000).expect("positive"),
            );
            let project = |at| {
                ledger
                    .admit_original_point(fragment, at, source, limits)
                    .expect("actual immutable admission")
                    .project()
                    .expect("actual original coordinate")
            };
            let left = project(param(6, 7));
            let right = project(param(8, 9));
            assert!(core::ptr::eq(left.admission.ledger, right.admission.ledger));
            assert_eq!(
                left.admission.source_creation(),
                right.admission.source_creation()
            );
            assert_eq!(left.admission.source_edge(), right.admission.source_edge());
            let workspace = Workspace {
                component_bits: left.admission.component_bit_bound(),
                integer_bits: left.admission.integer_bit_bound(),
                operation: "Workspace::compare",
            };
            let a = left.unique().expect("unique actual source coordinate");
            let b = right.unique().expect("unique actual source coordinate");
            assert_eq!(
                workspace
                    .compare(&a.coordinate.exact, &b.coordinate.exact)
                    .expect("guarded exact ordering"),
                order(field(12)),
                "{}",
                field(0)
            );
            assert_eq!(
                workspace
                    .compare_reference(&a.coordinate.exact, param(10, 11))
                    .expect("guarded source-bound ordering"),
                order(field(13)),
                "{}",
                field(0)
            );
            assert_eq!(
                workspace
                    .compare_reference(&b.coordinate.exact, param(10, 11))
                    .expect("guarded source-bound ordering"),
                order(field(14)),
                "{}",
                field(0)
            );
            assert_eq!(a.coordinate.as_param().is_ok(), field(15) == "1");
            assert_eq!(a.coordinate.as_u128_ratio().is_ok(), field(16) == "1");
            let direction = if reversed {
                super::Direction::Reversed
            } else {
                super::Direction::Original
            };
            assert_eq!(a.direction(), direction);
            assert_eq!(b.direction(), direction);
            if field(0) == "wide-half" {
                // Private guard control uses actual admitted/projected values;no public token is forged.
                let required = (a.coordinate.exact.num.bits() + b.coordinate.exact.den.bits())
                    .max(b.coordinate.exact.num.bits() + a.coordinate.exact.den.bits());
                let under = Workspace {
                    component_bits: workspace.component_bits,
                    integer_bits: required - 1,
                    operation: workspace.operation,
                };
                assert_eq!(
                    under.compare(&a.coordinate.exact, &b.coordinate.exact),
                    Err(sc_units::UnitError::DomainExceeded {
                        operation: "Workspace::compare",
                        kind: "integer workspace bits",
                        value: i128::from(required),
                        limit: i128::from(required - 1),
                    })
                );
                let exact = Workspace {
                    component_bits: workspace.component_bits,
                    integer_bits: required,
                    operation: workspace.operation,
                };
                assert_eq!(
                    exact
                        .compare(&a.coordinate.exact, &b.coordinate.exact)
                        .expect("exact conservative quota"),
                    Ordering::Less
                );
            }
            seen += 1;
        }
        assert_eq!(seen, 6);
    }
}
