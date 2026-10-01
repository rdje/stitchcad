//! The persistent-identity contract: reference resolution under split, merge, reverse, delete and
//! offset-fragmentation, and the [`RepairTask`] an orphaned reference becomes.
//!
//! The normative source is `docs/book/src/spec/ontology.md` §1.1 — the five-row edit table and the rule
//! under it: **no silent reassignment**; an unresolved reference is a first-class, visible repair task
//! naming the reference, the edit that orphaned it and the candidate resolutions, and a design holding one
//! is savable and inspectable but cannot be released. The glossary owns `repair task` and `unresolved
//! reference`; the release gate this feeds is `docs/book/src/spec/release-contract.md` §8.
//!
//! The coupled decision is `docs/decisions/decision_reference-resolution-journal-fold.md`: a stored
//! reference is **never rewritten by an edit**. An [`IdentityLedger`] holds an append-only journal of typed
//! [`TopologyEdit`]s, and resolution is a pure fold of that journal — the same reference over the same
//! journal always resolves the same way, so a replay reproduces every fragment identity byte-for-byte (the
//! `.3a` replay property, extended from ids to topology). Repair state is a derived view, never stored
//! mutation state, so it cannot drift from the journal it is derived from.
//!
//! Boundaries this module keeps, so a later slice does not rediscover them:
//!
//! - **Edges, not points.** §1.1's table governs edge topology. A [`PointRef`](super::reference::PointRef)
//!   is an entity in its own right; no edit in the G1 vocabulary deletes a point, so point stability is not
//!   a fold case — that a referenced point exists is `.3c`'s structural invariant.
//! - **Lengths are evidence, not state.** The only edit needing arc length is merge, and it carries the two
//!   lengths as caller-declared [`Length`] values (geometry owns them; the ledger stores none). The
//!   recomputation is exact [`Rational`] arithmetic — never a rounding.
//! - **Offset consumes declared intervals.** Which fragments an offset produces, over which
//!   source-parameter intervals, is G2's geometry (the offset engine). This module decides the contract
//!   given the intervals: an unambiguous position maps exactly, a shared boundary is refused with both
//!   fragments as candidates, a trimmed-away position is refused with none.
//! - **Serialization and commands are later leaves.** The journal is private and append-only; `sc-store`
//!   (`.7`) will deserialize behind the same validation, and the command bus (`.6`) is the product's only
//!   mutation path. [`ReleaseReadiness`] is this contract's one input to the release decision, not the
//!   whole of it — the artifact policy matrix (release contract §8, tuned at G4) remains the authority.

use core::cmp::Ordering;
use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use sc_units::{Length, UnitError};

use super::id::{EntityId, IdGenerator};
use super::rational::Rational;
use super::reference::{EdgeRef, LocalTag, Param};

/// The most edges one operation may create — the declared domain of [`IdentityLedger::declare_edges`] and
/// [`IdentityLedger::offset`], enforced at the boundary so no downstream code re-checks it.
///
/// Deliberately generous and deliberately finite: the supported envelope's drafting operations create a
/// handful of edges each (a line one, a full piece boundary tens), so a count past a million is a
/// caller bug — reported as a typed [`LedgerError::TooManyEdges`], in the same spirit as `sc-units`'
/// declared length domain, and it keeps every allocation the ledger makes bounded.
pub const MAX_EDGES_PER_OPERATION: usize = 1 << 20;

/// The direction of a resolved edge relative to the frame its reference was created in.
///
/// §1.1's reverse row: "references survive; parameters become `1 − t`; **directed consumers are told**".
/// This is how they are told: a fold accumulates every [`TopologyEdit::Reverse`] it passes through, so a
/// directed consumer (a grainline, a seam walk) sees [`Reversed`](Direction::Reversed) after an odd number
/// of reversals and [`Original`](Direction::Original) after an even one. It is deliberately not called
/// "orientation": the glossary owns that word for a boundary loop's winding (ontology §4.1), and one word
/// keeps one meaning.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    /// The edge still runs the way it did when the reference was created.
    Original,
    /// The edge has been reversed an odd number of times since; `t` counts from the other end.
    Reversed,
}

impl Direction {
    /// The direction after one more reversal.
    #[must_use]
    pub const fn reversed(self) -> Self {
        match self {
            Self::Original => Self::Reversed,
            Self::Reversed => Self::Original,
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Original => "original direction",
            Self::Reversed => "reversed",
        })
    }
}

/// Which fragment of a split a consumer wants when its position is exactly the split point.
///
/// §1.1's split row: "references at the split point resolve to both, and **the consumer states which it
/// wants**". Stating it is explicit — a [`SplitSide`] passed to [`Resolution::choose`] or recorded on a
/// registration through [`IdentityLedger::state_split_choice`] — because picking one silently is the
/// reassignment the contract forbids.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SplitSide {
    /// The fragment holding `[0, s)` — where the edge's parameter was below the split point.
    First,
    /// The fragment holding `(s, 1]` — where the edge's parameter was above the split point.
    Second,
}

impl fmt::Display for SplitSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::First => "first fragment",
            Self::Second => "second fragment",
        })
    }
}

/// Where a parameterized reference lives now: the live edge, the exact recomputed parameter, and the
/// direction relative to the frame the reference was created in.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResolvedRef {
    edge: EdgeRef,
    param: Param,
    direction: Direction,
}

impl ResolvedRef {
    /// Builds a resolved position. Contract-internal: the ledger mints these from a fold and consumers
    /// receive them; nothing outside this module constructs one, so a public constructor would only
    /// invite a fabricated resolution.
    const fn new(edge: EdgeRef, param: Param, direction: Direction) -> Self {
        Self {
            edge,
            param,
            direction,
        }
    }

    /// The live edge the reference resolves to.
    #[must_use]
    pub const fn edge(self) -> EdgeRef {
        self.edge
    }

    /// The exact recomputed parameter along that edge.
    #[must_use]
    pub const fn param(self) -> Param {
        self.param
    }

    /// Whether the edge runs opposite to the frame the reference was created in.
    #[must_use]
    pub const fn direction(self) -> Direction {
        self.direction
    }
}

impl fmt::Display for ResolvedRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at {}, {}", self.edge, self.param, self.direction)
    }
}

/// The edit that orphaned a reference, as a [`RepairTask`] names it.
///
/// Every variant carries the `operation` — the [`EntityId`] of the edit that did it — so a repair task
/// points at a replayable journal entry, not at prose.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrphaningEdit {
    /// The edge the fold had reached was deleted (§1.1's delete row: "every reference to it becomes
    /// unresolved"). `edge` is the edge the delete removed, which after earlier edits need not be the
    /// edge the holder names.
    Deleted {
        /// The delete operation.
        operation: EntityId,
        /// The edge the operation removed.
        edge: EdgeRef,
    },
    /// An offset fragmentation put the position on a boundary two fragments share, so the mapping is
    /// ambiguous (§1.1's offset row: "otherwise unresolved"). The two candidates are on the task.
    OffsetAmbiguous {
        /// The offset operation.
        operation: EntityId,
        /// The source edge that was fragmented.
        source: EdgeRef,
    },
    /// An offset fragmentation left the position in no fragment — it was trimmed away. No candidates:
    /// nothing in the fragments corresponds to it, so any resolution is a human or agent decision.
    OffsetUnmapped {
        /// The offset operation.
        operation: EntityId,
        /// The source edge that was fragmented.
        source: EdgeRef,
    },
    /// The exact recomputation an edit demanded does not fit the bounded rational — a typed
    /// [`UnitError`], per `decision_edge-parameter-bounded-exact-rational.md`. The reference is
    /// unresolved rather than rounded: an approximated position is a silent reassignment by arithmetic.
    RecomputationFailed {
        /// The operation whose recomputation failed.
        operation: EntityId,
        /// The edge the fold was on when the arithmetic refused.
        edge: EdgeRef,
        /// The typed arithmetic diagnostic.
        cause: UnitError,
    },
}

impl OrphaningEdit {
    /// The operation that orphaned the reference — an id into the ledger's journal.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        match self {
            Self::Deleted { operation, .. }
            | Self::OffsetAmbiguous { operation, .. }
            | Self::OffsetUnmapped { operation, .. }
            | Self::RecomputationFailed { operation, .. } => *operation,
        }
    }
}

impl fmt::Display for OrphaningEdit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Deleted { operation, edge } => {
                write!(f, "the deletion of {edge} by operation {operation}")
            }
            Self::OffsetAmbiguous { operation, source } => write!(
                f,
                "the offset fragmentation of {source} by operation {operation} (the position sits on a boundary two fragments share)"
            ),
            Self::OffsetUnmapped { operation, source } => write!(
                f,
                "the offset fragmentation of {source} by operation {operation} (no fragment covers the position)"
            ),
            Self::RecomputationFailed {
                operation,
                edge,
                cause,
            } => write!(
                f,
                "the recomputation {edge} owed operation {operation}, which refused exactly: {cause}"
            ),
        }
    }
}

/// An unresolved reference: the first-class, visible object §1.1 requires instead of a silent
/// reassignment.
///
/// It names exactly what the spec's rule demands — **the reference** (as its holder states it, not an
/// intermediate the fold passed through), **the edit that orphaned it**, and **the candidate
/// resolutions** (empty where the contract has nothing to suggest; a deletion leaves no candidates,
/// because inventing one would be the silent reassignment the contract forbids). It is data: savable,
/// inspectable, enumerable through [`IdentityLedger::open_repairs`], and while one is open the design
/// cannot be released ([`IdentityLedger::release_readiness`]).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RepairTask {
    reference: EdgeRef,
    param: Param,
    orphaned_by: OrphaningEdit,
    candidates: Vec<ResolvedRef>,
}

impl RepairTask {
    /// Builds a repair task. Contract-internal, like [`ResolvedRef::new`]: a task is what a fold returns,
    /// never something a caller fabricates.
    fn new(
        reference: EdgeRef,
        param: Param,
        orphaned_by: OrphaningEdit,
        candidates: Vec<ResolvedRef>,
    ) -> Self {
        Self {
            reference,
            param,
            orphaned_by,
            candidates,
        }
    }

    /// The orphaned reference, exactly as its holder states it.
    #[must_use]
    pub const fn reference(&self) -> EdgeRef {
        self.reference
    }

    /// The parameter along that reference that no longer resolves.
    #[must_use]
    pub const fn param(&self) -> Param {
        self.param
    }

    /// The edit that orphaned it.
    #[must_use]
    pub const fn orphaned_by(&self) -> &OrphaningEdit {
        &self.orphaned_by
    }

    /// The candidate resolutions, in the fragments' order. Empty where the contract has nothing to
    /// suggest.
    #[must_use]
    pub fn candidates(&self) -> &[ResolvedRef] {
        &self.candidates
    }
}

impl fmt::Display for RepairTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "repair task: {} at {} was orphaned by {}; {} candidate resolution(s)",
            self.reference,
            self.param,
            self.orphaned_by,
            self.candidates.len()
        )
    }
}

/// The answer to "where does this parameterized reference live now?".
///
/// Exactly one of the three happens, which is §1.1's "exactly one of two things" made total: the split
/// point is not a third fate but the split row's own both-fragments case, kept visible so the consumer —
/// not the ledger — states which it wants.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Resolution {
    /// The reference resolves to exactly one live position.
    Resolved(ResolvedRef),
    /// The reference sits exactly at a split point: both fragments contain it, each side fully resolved
    /// through the rest of the journal. Choose with [`choose`](Resolution::choose).
    SplitPoint {
        /// The `[0, s)` fragment's resolution.
        first: Box<Resolution>,
        /// The `(s, 1]` fragment's resolution.
        second: Box<Resolution>,
    },
    /// The reference no longer resolves; the repair task names the reference, the orphaning edit and
    /// the candidates.
    Unresolved(RepairTask),
}

impl Resolution {
    /// States which fragment the consumer wants at a split point.
    ///
    /// On anything but a [`SplitPoint`](Resolution::SplitPoint) this is the identity: a unique resolution
    /// has nothing to choose, so a consumer may apply a recorded side unconditionally. It is not a
    /// silent pick in the forbidden sense — the forbidden pick is the *ledger* choosing; here the
    /// consumer has spoken.
    #[must_use]
    pub fn choose(self, side: SplitSide) -> Self {
        match (self, side) {
            (Self::SplitPoint { first, .. }, SplitSide::First) => *first,
            (Self::SplitPoint { second, .. }, SplitSide::Second) => *second,
            (other, _) => other,
        }
    }

    /// The unique resolved position, when this resolution is one.
    #[must_use]
    pub const fn resolved(&self) -> Option<ResolvedRef> {
        match self {
            Self::Resolved(position) => Some(*position),
            Self::SplitPoint { .. } | Self::Unresolved(_) => None,
        }
    }
}

/// One journal entry: an operation's id and the topology edit it performed.
///
/// The operation id is the identity ontology §1 gives every creating operation; for the edits that
/// create edges it is also the `creator` half of every [`EdgeRef`] they mint.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JournalEntry {
    operation: EntityId,
    edit: TopologyEdit,
}

impl JournalEntry {
    /// The id of the operation that performed this edit.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }

    /// The edit itself.
    #[must_use]
    pub const fn edit(&self) -> &TopologyEdit {
        &self.edit
    }
}

/// One fragment of an offset, as the journal records it: the fragment's identity and the closed
/// `[from, to]` interval of the source edge it corresponds to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OffsetFragment {
    edge: EdgeRef,
    from: Param,
    to: Param,
}

impl OffsetFragment {
    /// The fragment's identity (created by the offset operation).
    #[must_use]
    pub const fn edge(self) -> EdgeRef {
        self.edge
    }

    /// Where the fragment's correspondence with the source starts.
    #[must_use]
    pub const fn from(self) -> Param {
        self.from
    }

    /// Where the fragment's correspondence with the source ends.
    #[must_use]
    pub const fn to(self) -> Param {
        self.to
    }
}

/// A typed topology edit: exactly the vocabulary of ontology §1.1's table, plus the declaration a
/// drafting operation's edges enter the journal with.
///
/// The created edges' identities ride in the entry (`first`/`second`, `merged`, the fragments'
/// [`EdgeRef`]s): the journal is the sole source of truth for who created what, which is what makes a
/// replay reproduce every identity byte-for-byte.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TopologyEdit {
    /// A drafting operation created these edges (tags `0..n` of one operation id).
    Declared {
        /// The edges the operation created, in tag order.
        edges: Vec<EdgeRef>,
    },
    /// An edge was split at `at` into `first` (`[0, at)`) and `second` (`(at, 1]`); the source is gone.
    /// `at` is strictly inside `(0, 1)` — an endpoint split would create a zero-length fragment, whose
    /// parameter space ontology §1 defines as a fraction of the edge's *own length*.
    Split {
        /// The edge that was split.
        source: EdgeRef,
        /// The split point, in the source's frame at edit time.
        at: Param,
        /// The `[0, at)` fragment.
        first: EdgeRef,
        /// The `(at, 1]` fragment.
        second: EdgeRef,
    },
    /// Two live edges were merged end to end, first then second, into `merged`; both sources are gone.
    /// The lengths are the caller-declared arc-length evidence the parameter recomputation divides by —
    /// the ledger stores no length of its own.
    Merge {
        /// The edge that comes first in the merged direction.
        first: EdgeRef,
        /// Its arc length, as evidence.
        first_length: Length,
        /// The edge that comes second.
        second: EdgeRef,
        /// Its arc length, as evidence.
        second_length: Length,
        /// The merged edge (created by the merge operation).
        merged: EdgeRef,
    },
    /// An edge was reversed in place: its identity survives, its direction does not.
    Reverse {
        /// The edge that was reversed.
        edge: EdgeRef,
    },
    /// An edge was deleted; every reference resolving onto it becomes unresolved.
    Delete {
        /// The edge that was deleted.
        edge: EdgeRef,
    },
    /// An edge was fragmented by offsetting into `fragments`, in source-parameter order; the source is
    /// gone. Coverage may be partial — a trimmed-away position is an unmapped orphan, not an error.
    Offset {
        /// The edge that was fragmented.
        source: EdgeRef,
        /// The fragments, each with the source interval it corresponds to.
        fragments: Vec<OffsetFragment>,
    },
}

/// A source-parameter interval an offset fragment corresponds to, as the caller declares it.
///
/// The bounds are [`Param`]s, so "inside `[0, 1]`" is enforced by the type; `new` enforces the one
/// remaining invariant — a fragment covers a non-empty interval — at construction, where this crate puts
/// its invariants.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OffsetInterval {
    from: Param,
    to: Param,
}

impl OffsetInterval {
    /// Builds an interval, rejecting an empty or reversed one.
    ///
    /// # Errors
    ///
    /// Returns [`LedgerError::IntervalEmpty`] when `from >= to`: a zero-width fragment has no parameter
    /// space to map a reference into.
    pub fn new(from: Param, to: Param) -> Result<Self, LedgerError> {
        if from >= to {
            return Err(LedgerError::IntervalEmpty { from, to });
        }
        Ok(Self { from, to })
    }

    /// Where the interval starts.
    #[must_use]
    pub const fn from(self) -> Param {
        self.from
    }

    /// Where the interval ends.
    #[must_use]
    pub const fn to(self) -> Param {
        self.to
    }
}

/// The two fragments a split created, in parameter order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SplitFragments {
    first: EdgeRef,
    second: EdgeRef,
}

impl SplitFragments {
    /// The `[0, s)` fragment.
    #[must_use]
    pub const fn first(self) -> EdgeRef {
        self.first
    }

    /// The `(s, 1]` fragment.
    #[must_use]
    pub const fn second(self) -> EdgeRef {
        self.second
    }
}

/// A reference a consumer holds, as registered with the ledger: the owning entity plus the parameterized
/// edge reference, and the split side the consumer stated if it ever had to.
///
/// This is the `.3b`-level stand-in for the ownership `.3c`'s objects will bring: a notch, a seam span or
/// a grainline registers the references it holds under its own [`EntityId`], and the ledger can then
/// answer §1.1's design-level questions — what is unresolved, may this be released — over the whole set.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Registration {
    owner: EntityId,
    edge: EdgeRef,
    param: Param,
    split_choice: Option<SplitSide>,
}

impl Registration {
    /// The entity that holds the reference.
    #[must_use]
    pub const fn owner(self) -> EntityId {
        self.owner
    }

    /// The referenced edge, exactly as the holder states it — never rewritten by an edit.
    #[must_use]
    pub const fn edge(self) -> EdgeRef {
        self.edge
    }

    /// The parameter along that edge, exactly as the holder states it.
    #[must_use]
    pub const fn param(self) -> Param {
        self.param
    }

    /// The split side the consumer stated, if any.
    #[must_use]
    pub const fn split_choice(self) -> Option<SplitSide> {
        self.split_choice
    }
}

/// One open repair task and the entity whose reference it repairs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpenRepair {
    owner: EntityId,
    task: RepairTask,
}

impl OpenRepair {
    /// The entity holding the unresolved reference.
    #[must_use]
    pub const fn owner(&self) -> EntityId {
        self.owner
    }

    /// The repair task itself.
    #[must_use]
    pub const fn task(&self) -> &RepairTask {
        &self.task
    }
}

impl fmt::Display for OpenRepair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} held by {}", self.task, self.owner)
    }
}

/// Whether the identity contract's release rule permits a release.
///
/// §1.1: "A design with unresolved references can be saved and inspected; it cannot be released." This is
/// that rule and nothing more — one input to the release decision, whose full authority is the artifact
/// policy matrix (release contract §8, tuned at G4).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReleaseReadiness {
    /// No registered reference is unresolved.
    Releasable,
    /// Registered references are unresolved; each is an open [`RepairTask`].
    Blocked {
        /// How many registrations are unresolved.
        unresolved: usize,
    },
}

impl ReleaseReadiness {
    /// How many registrations are unresolved (`0` when releasable).
    #[must_use]
    pub const fn unresolved(self) -> usize {
        match self {
            Self::Releasable => 0,
            Self::Blocked { unresolved } => unresolved,
        }
    }
}

impl fmt::Display for ReleaseReadiness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Releasable => f.write_str("releasable: no unresolved references"),
            Self::Blocked { unresolved } => write!(
                f,
                "not releasable: {unresolved} unresolved reference(s) carry open repair tasks (ontology §1.1)"
            ),
        }
    }
}

/// Every way the ledger can refuse an operation. Typed, named, and carrying the offending value, per the
/// house diagnostic convention (`sc-units`' `UnitError`, `.3a`'s `IdError`/`ParamError`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LedgerError {
    /// The edit named an edge that is not live — never declared, or already consumed by a split, a
    /// merge, an offset or a delete.
    NotLive {
        /// The edge that is not live.
        edge: EdgeRef,
    },
    /// A split at `0` or `1` would create a zero-length fragment, which has no parameter space (ontology
    /// §1 defines the parameter as a fraction of the edge's own length).
    SplitAtEndpoint {
        /// The offending split point.
        at: Param,
    },
    /// A merge of an edge with itself.
    MergeSameEdge {
        /// The edge named on both sides.
        edge: EdgeRef,
    },
    /// A merge length was zero or negative: arc-length evidence for a recomputation must be a real
    /// length, and a zero length would divide by zero rather than diagnose.
    NonPositiveLength {
        /// Which side of the merge, for the diagnostic.
        side: &'static str,
        /// The offending length.
        length: Length,
    },
    /// An operation was asked to create no edges at all — a declaration of zero, or an offset with no
    /// intervals. An operation that creates nothing has no identity to mint and nothing to resolve
    /// through.
    NoEdges,
    /// More edges than one operation may create ([`MAX_EDGES_PER_OPERATION`]).
    TooManyEdges {
        /// The requested count.
        count: usize,
    },
    /// An offset interval was empty or reversed (`from >= to`).
    IntervalEmpty {
        /// The interval start.
        from: Param,
        /// The interval end.
        to: Param,
    },
    /// Offset intervals were not ascending and non-overlapping: a fragment may share at most a boundary
    /// point with its predecessor, so `next.from < previous.to` is an overlap and a descending pair is
    /// unordered — both refuse, because the fold's ambiguity rule counts interval containment.
    IntervalsUnordered {
        /// The previous interval's end.
        previous_to: Param,
        /// The next interval's start.
        next_from: Param,
    },
    /// An operation named a registration that does not exist.
    UnknownRegistration {
        /// The owner named.
        owner: EntityId,
        /// The edge named.
        edge: EdgeRef,
        /// The parameter named.
        param: Param,
    },
}

impl fmt::Display for LedgerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotLive { edge } => write!(
                f,
                "{edge} is not a live edge: never declared, or already consumed by an edit (ontology §1.1)"
            ),
            Self::SplitAtEndpoint { at } => write!(
                f,
                "a split at {at} would create a zero-length fragment; the split point must be strictly inside (0, 1)"
            ),
            Self::MergeSameEdge { edge } => write!(f, "{edge} cannot be merged with itself"),
            Self::NonPositiveLength { side, length } => write!(
                f,
                "the {side} side of a merge must carry a positive arc length; {length} does not"
            ),
            Self::NoEdges => f.write_str("an operation must create at least one edge"),
            Self::TooManyEdges { count } => {
                write!(f, "{count} edges exceed the {MAX_EDGES_PER_OPERATION} one operation may create")
            }
            Self::IntervalEmpty { from, to } => {
                write!(f, "an offset interval must be non-empty; [{from}, {to}] is not")
            }
            Self::IntervalsUnordered {
                previous_to,
                next_from,
            } => write!(
                f,
                "offset intervals must ascend without overlapping; the next starts at {next_from}, before the previous ends at {previous_to}"
            ),
            Self::UnknownRegistration {
                owner,
                edge,
                param,
            } => write!(f, "{owner} holds no registration of {edge} at {param}"),
        }
    }
}

impl std::error::Error for LedgerError {}

/// The identity contract's state: an append-only journal of topology edits, the live-edge set that
/// journal implies, and the references consumers have registered against it.
///
/// The journal is the sole source of truth; the live set is its maintained index (kept equal to a replay
/// of the journal, which the property suite proves rather than assumes). Registrations are how §1.1's
/// design-level sentences get a subject before `.3c`'s objects exist: a consumer registers the
/// parameterized references it holds, and the ledger can then enumerate what an edit orphaned
/// ([`open_repairs`](IdentityLedger::open_repairs)) and answer the release rule
/// ([`release_readiness`](IdentityLedger::release_readiness)).
///
/// Every mutation validates before it mints: a refused operation consumes no [`EntityId`], so a script
/// with failures replays as deterministically as one without.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct IdentityLedger {
    journal: Vec<JournalEntry>,
    live: BTreeSet<EdgeRef>,
    registrations: BTreeMap<(EntityId, EdgeRef, Param), Option<SplitSide>>,
}

impl IdentityLedger {
    /// An empty ledger: no edges, no edits, no registrations.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The journal, in the order the edits were applied. Append-only; the sole source of truth.
    #[must_use]
    pub fn journal(&self) -> &[JournalEntry] {
        &self.journal
    }

    /// Whether an edge is live right now.
    #[must_use]
    pub fn is_live(&self, edge: EdgeRef) -> bool {
        self.live.contains(&edge)
    }

    /// The live edges, in a deterministic order.
    pub fn live_edges(&self) -> impl Iterator<Item = EdgeRef> + '_ {
        self.live.iter().copied()
    }

    /// The registered references, in a deterministic order.
    pub fn registrations(&self) -> impl Iterator<Item = Registration> + '_ {
        self.registrations
            .iter()
            .map(|((owner, edge, param), choice)| Registration {
                owner: *owner,
                edge: *edge,
                param: *param,
                split_choice: *choice,
            })
    }

    /// Records a drafting operation creating `count` edges (tags `0..count` of one operation id).
    ///
    /// This is how edges enter the contract before the recipe exists (`G1-SLICE.5` will drive it from
    /// real drafting operations); every later edit consumes what a declaration introduced.
    ///
    /// # Errors
    ///
    /// [`LedgerError::NoEdges`] when `count` is zero, [`LedgerError::TooManyEdges`] when it exceeds
    /// [`MAX_EDGES_PER_OPERATION`].
    pub fn declare_edges(
        &mut self,
        ids: &mut impl IdGenerator,
        count: usize,
    ) -> Result<Vec<EdgeRef>, LedgerError> {
        if count == 0 {
            return Err(LedgerError::NoEdges);
        }
        if count > MAX_EDGES_PER_OPERATION {
            return Err(LedgerError::TooManyEdges { count });
        }
        let operation = ids.next_id();
        let mut edges = Vec::new();
        for slot in 0..count {
            // `count` is bounded by `MAX_EDGES_PER_OPERATION`, far inside `u32`, so the conversion
            // cannot fail; the map keeps the bound enforced in one place instead of trusted in two.
            let tag = u32::try_from(slot).map_err(|_| LedgerError::TooManyEdges { count })?;
            edges.push(EdgeRef::new(operation, LocalTag::new(tag)));
        }
        self.live.extend(edges.iter().copied());
        self.push(
            operation,
            TopologyEdit::Declared {
                edges: edges.clone(),
            },
        );
        Ok(edges)
    }

    /// Splits a live edge at `at` into two fragments created by the split operation: `first` holds
    /// `[0, at)`, `second` holds `(at, 1]`. The source leaves the live set; its references resolve
    /// through the split (§1.1's first row).
    ///
    /// # Errors
    ///
    /// [`LedgerError::NotLive`] when `source` is not live, [`LedgerError::SplitAtEndpoint`] when `at`
    /// is `0` or `1`.
    pub fn split(
        &mut self,
        ids: &mut impl IdGenerator,
        source: EdgeRef,
        at: Param,
    ) -> Result<SplitFragments, LedgerError> {
        if !self.is_live(source) {
            return Err(LedgerError::NotLive { edge: source });
        }
        if at == Param::START || at == Param::END {
            return Err(LedgerError::SplitAtEndpoint { at });
        }
        let operation = ids.next_id();
        let fragments = SplitFragments {
            first: EdgeRef::new(operation, LocalTag::FIRST),
            second: EdgeRef::new(operation, LocalTag::new(1)),
        };
        self.live.remove(&source);
        self.live.insert(fragments.first);
        self.live.insert(fragments.second);
        self.push(
            operation,
            TopologyEdit::Split {
                source,
                at,
                first: fragments.first,
                second: fragments.second,
            },
        );
        Ok(fragments)
    }

    /// Merges two distinct live edges end to end — `first` then `second` — into one edge created by the
    /// merge operation. Both sources leave the live set; their references survive onto the merged edge
    /// with their parameters recomputed by the declared arc lengths (§1.1's second row).
    ///
    /// The lengths are evidence the caller (geometry, later the command layer) declares; the ledger
    /// stores none of its own. If the second edge runs opposite to the merged direction, reverse it
    /// first — the journal records what happened, and the fold composes it exactly.
    ///
    /// # Errors
    ///
    /// [`LedgerError::NotLive`] for a source that is not live, [`LedgerError::MergeSameEdge`] when both
    /// sides name one edge, [`LedgerError::NonPositiveLength`] for a length that is zero or negative.
    pub fn merge(
        &mut self,
        ids: &mut impl IdGenerator,
        first: EdgeRef,
        first_length: Length,
        second: EdgeRef,
        second_length: Length,
    ) -> Result<EdgeRef, LedgerError> {
        if !self.is_live(first) {
            return Err(LedgerError::NotLive { edge: first });
        }
        if !self.is_live(second) {
            return Err(LedgerError::NotLive { edge: second });
        }
        if first == second {
            return Err(LedgerError::MergeSameEdge { edge: first });
        }
        if first_length <= Length::ZERO {
            return Err(LedgerError::NonPositiveLength {
                side: "first",
                length: first_length,
            });
        }
        if second_length <= Length::ZERO {
            return Err(LedgerError::NonPositiveLength {
                side: "second",
                length: second_length,
            });
        }
        let operation = ids.next_id();
        let merged = EdgeRef::new(operation, LocalTag::FIRST);
        self.live.remove(&first);
        self.live.remove(&second);
        self.live.insert(merged);
        self.push(
            operation,
            TopologyEdit::Merge {
                first,
                first_length,
                second,
                second_length,
                merged,
            },
        );
        Ok(merged)
    }

    /// Reverses a live edge in place: its identity survives, its direction does not, and every
    /// reference's parameter becomes `1 − t`, with directed consumers told through
    /// [`Direction::Reversed`] (§1.1's third row). Returns the operation id.
    ///
    /// # Errors
    ///
    /// [`LedgerError::NotLive`] when `edge` is not live.
    pub fn reverse(
        &mut self,
        ids: &mut impl IdGenerator,
        edge: EdgeRef,
    ) -> Result<EntityId, LedgerError> {
        if !self.is_live(edge) {
            return Err(LedgerError::NotLive { edge });
        }
        let operation = ids.next_id();
        self.push(operation, TopologyEdit::Reverse { edge });
        Ok(operation)
    }

    /// Deletes a live edge. Every reference resolving onto it becomes unresolved and surfaces as a
    /// visible [`RepairTask`] (§1.1's fourth row); nothing is reassigned. Returns the operation id —
    /// the id every resulting task names as the orphaning edit.
    ///
    /// # Errors
    ///
    /// [`LedgerError::NotLive`] when `edge` is not live.
    pub fn delete(
        &mut self,
        ids: &mut impl IdGenerator,
        edge: EdgeRef,
    ) -> Result<EntityId, LedgerError> {
        if !self.is_live(edge) {
            return Err(LedgerError::NotLive { edge });
        }
        let operation = ids.next_id();
        self.live.remove(&edge);
        self.push(operation, TopologyEdit::Delete { edge });
        Ok(operation)
    }

    /// Fragments a live edge by offsetting (§1.1's fifth row): the source leaves the live set and the
    /// offset operation creates one edge per interval, in source-parameter order (tags `0..n`).
    ///
    /// The intervals are the caller's declaration of the correspondence geometry established; at G1 they
    /// are test and fixture data, from G2 they are the offset engine's output. A reference maps to the
    /// one fragment whose interval contains its parameter; a parameter on a shared boundary or in an
    /// uncovered gap is refused into a [`RepairTask`] rather than guessed at.
    ///
    /// # Errors
    ///
    /// [`LedgerError::NotLive`] when `source` is not live, [`LedgerError::NoEdges`] when `intervals` is
    /// empty, [`LedgerError::TooManyEdges`] beyond [`MAX_EDGES_PER_OPERATION`],
    /// [`LedgerError::IntervalsUnordered`] when a pair overlaps or descends. (An empty interval is
    /// refused earlier, by [`OffsetInterval::new`].)
    pub fn offset(
        &mut self,
        ids: &mut impl IdGenerator,
        source: EdgeRef,
        intervals: &[OffsetInterval],
    ) -> Result<Vec<EdgeRef>, LedgerError> {
        if !self.is_live(source) {
            return Err(LedgerError::NotLive { edge: source });
        }
        if intervals.is_empty() {
            return Err(LedgerError::NoEdges);
        }
        if intervals.len() > MAX_EDGES_PER_OPERATION {
            return Err(LedgerError::TooManyEdges {
                count: intervals.len(),
            });
        }
        for (previous, next) in intervals.iter().zip(intervals.iter().skip(1)) {
            if next.from() < previous.to() {
                return Err(LedgerError::IntervalsUnordered {
                    previous_to: previous.to(),
                    next_from: next.from(),
                });
            }
        }
        let operation = ids.next_id();
        let mut fragments = Vec::with_capacity(intervals.len());
        for (slot, interval) in intervals.iter().enumerate() {
            // Bounded above, as in `declare_edges`: the conversion cannot fail, and the map keeps the
            // bound enforced in one place instead of trusted in two.
            let tag = u32::try_from(slot).map_err(|_| LedgerError::TooManyEdges {
                count: intervals.len(),
            })?;
            fragments.push(OffsetFragment {
                edge: EdgeRef::new(operation, LocalTag::new(tag)),
                from: interval.from(),
                to: interval.to(),
            });
        }
        self.live.remove(&source);
        self.live
            .extend(fragments.iter().map(|fragment| fragment.edge()));
        let edges = fragments.iter().map(|fragment| fragment.edge()).collect();
        self.push(operation, TopologyEdit::Offset { source, fragments });
        Ok(edges)
    }

    /// Resolves a parameterized reference against the journal: the pure fold §1.1's table defines.
    ///
    /// The reference is any `(edge, param)` pair — the fold is total and needs no registration. Edits the
    /// journal recorded *before* an edge existed cannot mention it (every apply validates against the
    /// live set), so an edge this ledger never saw passes through resolved: whether a reference names a
    /// real edge is a structural invariant of the object holding it (`.3c`), not of the fold.
    #[must_use]
    pub fn resolve(&self, edge: EdgeRef, param: Param) -> Resolution {
        self.fold(0, edge, param, edge, param, Direction::Original)
    }

    /// Registers a reference a consumer holds, so the design-level questions — what is unresolved, may
    /// this be released — have a complete set to answer over. Re-registering an identical reference is
    /// idempotent and keeps a stated split choice.
    ///
    /// Liveness is the whole born-valid check, and that is a theorem, not a shortcut: of the five
    /// §1.1 edits, four consume their source edge, so only a reverse can ever mention a live one — and
    /// `1 − t` is exact for every `t` in `[0, 1]`. A live edge therefore always resolves; references are
    /// orphaned by edits that come *after* registration, which is precisely what §1.1 governs and what
    /// [`open_repairs`](IdentityLedger::open_repairs) derives.
    ///
    /// # Errors
    ///
    /// [`LedgerError::NotLive`] when `edge` is not live.
    pub fn register(
        &mut self,
        owner: EntityId,
        edge: EdgeRef,
        param: Param,
    ) -> Result<(), LedgerError> {
        if !self.is_live(edge) {
            return Err(LedgerError::NotLive { edge });
        }
        self.registrations
            .entry((owner, edge, param))
            .or_insert(None);
        Ok(())
    }

    /// Records the side a consumer wants when its registered reference sits at a split point — the
    /// "consumer states which it wants" half of §1.1's first row. The choice is applied at every split
    /// point the fold meets, so a stated preference survives a chain of edits.
    ///
    /// # Errors
    ///
    /// [`LedgerError::UnknownRegistration`] when no such registration exists.
    pub fn state_split_choice(
        &mut self,
        owner: EntityId,
        edge: EdgeRef,
        param: Param,
        side: SplitSide,
    ) -> Result<(), LedgerError> {
        let choice = self
            .registrations
            .get_mut(&(owner, edge, param))
            .ok_or(LedgerError::UnknownRegistration { owner, edge, param })?;
        *choice = Some(side);
        Ok(())
    }

    /// Resolves a registered reference, applying the split choice its consumer stated, if any.
    ///
    /// # Errors
    ///
    /// [`LedgerError::UnknownRegistration`] when no such registration exists.
    pub fn resolve_registered(
        &self,
        owner: EntityId,
        edge: EdgeRef,
        param: Param,
    ) -> Result<Resolution, LedgerError> {
        let choice = self
            .registrations
            .get(&(owner, edge, param))
            .copied()
            .ok_or(LedgerError::UnknownRegistration { owner, edge, param })?;
        Ok(self.resolve_with_choice(edge, param, choice))
    }

    /// Withdraws a registration — the consumer lets the reference go (the object holding it was itself
    /// deleted, or a repair retires it). An open repair task for it closes with it, because the task set
    /// is derived from the registrations.
    ///
    /// # Errors
    ///
    /// [`LedgerError::UnknownRegistration`] when no such registration exists.
    pub fn retire(
        &mut self,
        owner: EntityId,
        edge: EdgeRef,
        param: Param,
    ) -> Result<(), LedgerError> {
        if self.registrations.remove(&(owner, edge, param)).is_none() {
            return Err(LedgerError::UnknownRegistration { owner, edge, param });
        }
        Ok(())
    }

    /// Repoints a registration at a new parameterized reference — the explicit repair of an orphaned
    /// one, and the only way a registration's `(edge, param)` ever changes: a human or agent decision,
    /// never an edit's side effect. Any stated split choice resets, because it was a statement about the
    /// old reference's frame. The new edge must be live, which by [`register`](IdentityLedger::register)'s
    /// theorem means the new reference resolves.
    ///
    /// # Errors
    ///
    /// [`LedgerError::UnknownRegistration`] when the old registration does not exist,
    /// [`LedgerError::NotLive`] when the new edge is not live.
    pub fn repoint(
        &mut self,
        owner: EntityId,
        old_edge: EdgeRef,
        old_param: Param,
        new_edge: EdgeRef,
        new_param: Param,
    ) -> Result<(), LedgerError> {
        let old = (owner, old_edge, old_param);
        if !self.registrations.contains_key(&old) {
            return Err(LedgerError::UnknownRegistration {
                owner,
                edge: old_edge,
                param: old_param,
            });
        }
        if !self.is_live(new_edge) {
            return Err(LedgerError::NotLive { edge: new_edge });
        }
        self.registrations.remove(&old);
        self.registrations
            .insert((owner, new_edge, new_param), None);
        Ok(())
    }

    /// Every open repair task, with the entity holding it, in a deterministic order. Derived from the
    /// journal and the registrations on every call — never stored, so it cannot drift from either.
    /// This is §1.1's "an unresolved reference is a first-class, visible object", enumerable.
    #[must_use]
    pub fn open_repairs(&self) -> Vec<OpenRepair> {
        self.registrations
            .iter()
            .filter_map(|((owner, edge, param), choice)| {
                match self.resolve_with_choice(*edge, *param, *choice) {
                    Resolution::Unresolved(task) => Some(OpenRepair {
                        owner: *owner,
                        task,
                    }),
                    Resolution::Resolved(_) | Resolution::SplitPoint { .. } => None,
                }
            })
            .collect()
    }

    /// The identity contract's release rule: [`Blocked`](ReleaseReadiness::Blocked) while any registered
    /// reference is unresolved, [`Releasable`](ReleaseReadiness::Releasable) when none is. Saving and
    /// inspecting are untouched by it — every query on this ledger answers identically with open
    /// repairs, which is the "savable and inspectable" half of §1.1's rule.
    #[must_use]
    pub fn release_readiness(&self) -> ReleaseReadiness {
        let unresolved = self.open_repairs().len();
        if unresolved == 0 {
            ReleaseReadiness::Releasable
        } else {
            ReleaseReadiness::Blocked { unresolved }
        }
    }

    /// Appends one entry. The only writer of the journal; every caller validated and minted first.
    fn push(&mut self, operation: EntityId, edit: TopologyEdit) {
        self.journal.push(JournalEntry { operation, edit });
    }

    /// Resolves and peels a stated split choice at every split point the fold meets.
    fn resolve_with_choice(
        &self,
        edge: EdgeRef,
        param: Param,
        choice: Option<SplitSide>,
    ) -> Resolution {
        let mut resolution = self.resolve(edge, param);
        if let Some(side) = choice {
            // Each `choose` unboxes one split-point layer, and the journal is finite, so this peels to a
            // non-split resolution in at most as many steps as the fold met split points.
            while matches!(resolution, Resolution::SplitPoint { .. }) {
                resolution = resolution.choose(side);
            }
        }
        resolution
    }

    /// The fold itself: walks the journal from `from`, carrying the holder's original reference
    /// (`held_edge`, `held_param` — what a [`RepairTask`] must name) and the current position (`edge`,
    /// `param`, `direction`).
    ///
    /// Total by construction: every arithmetic step is checked, and a step that refuses becomes a
    /// `RecomputationFailed` repair task rather than a panic, a wrap or a round.
    fn fold(
        &self,
        from: usize,
        held_edge: EdgeRef,
        held_param: Param,
        mut edge: EdgeRef,
        mut param: Param,
        mut direction: Direction,
    ) -> Resolution {
        for (index, entry) in self.journal.iter().enumerate().skip(from) {
            // A repair task for a failed recomputation, in the one shape every arithmetic refusal uses:
            // the reference as its holder states it, the operation that demanded the recomputation, and
            // the typed diagnostic — never a rounded fallback. `edge` is a parameter, not a capture: the
            // fold mutates it as it goes, and a task must name the edge the fold was on when the
            // arithmetic refused.
            let failed = |edge: EdgeRef, cause: UnitError| {
                Resolution::Unresolved(RepairTask::new(
                    held_edge,
                    held_param,
                    OrphaningEdit::RecomputationFailed {
                        operation: entry.operation(),
                        edge,
                        cause,
                    },
                    Vec::new(),
                ))
            };
            match entry.edit() {
                TopologyEdit::Split {
                    source,
                    at,
                    first,
                    second,
                } if *source == edge => {
                    let t = param.as_rational();
                    let s = at.as_rational();
                    match t.cmp(&s) {
                        Ordering::Less => {
                            // `s > 0` (validated at apply), so the division is defined; exactness is
                            // `Rational`'s, and a refusal is a visible task, never a rounding.
                            match t.checked_div(s).and_then(to_param) {
                                Ok(local) => {
                                    param = local;
                                    edge = *first;
                                }
                                Err(cause) => return failed(edge, cause),
                            }
                        }
                        Ordering::Greater => {
                            // `s < 1` (validated at apply), so `1 − s > 0` and the division is defined.
                            let recomputed = t
                                .checked_sub(s)
                                .and_then(|shifted| {
                                    s.one_minus().and_then(|rest| shifted.checked_div(rest))
                                })
                                .and_then(to_param);
                            match recomputed {
                                Ok(local) => {
                                    param = local;
                                    edge = *second;
                                }
                                Err(cause) => return failed(edge, cause),
                            }
                        }
                        Ordering::Equal => {
                            // The split-point case §1.1 spells out: resolve to BOTH, each side folded
                            // through the rest of the journal, and let the consumer state which it wants.
                            return Resolution::SplitPoint {
                                first: Box::new(self.fold(
                                    index + 1,
                                    held_edge,
                                    held_param,
                                    *first,
                                    Param::END,
                                    direction,
                                )),
                                second: Box::new(self.fold(
                                    index + 1,
                                    held_edge,
                                    held_param,
                                    *second,
                                    Param::START,
                                    direction,
                                )),
                            };
                        }
                    }
                }
                TopologyEdit::Merge {
                    first,
                    first_length,
                    second,
                    second_length,
                    merged,
                } if *first == edge || *second == edge => {
                    let on_first = *first == edge;
                    let l1 = Rational::from_integer(first_length.as_micrometres());
                    let l2 = Rational::from_integer(second_length.as_micrometres());
                    let t = param.as_rational();
                    // Recomputed BY ARC LENGTH, exactly: `t·L₁/(L₁+L₂)` from the first edge,
                    // `(L₁ + t·L₂)/(L₁+L₂)` from the second. Both lengths are positive (validated at
                    // apply), so the total is non-zero and every division is defined.
                    let recomputed = l1.checked_add(l2).and_then(|total| {
                        if on_first {
                            t.checked_mul(l1)
                                .and_then(|scaled| scaled.checked_div(total))
                        } else {
                            t.checked_mul(l2)
                                .and_then(|scaled| l1.checked_add(scaled))
                                .and_then(|shifted| shifted.checked_div(total))
                        }
                    });
                    match recomputed.and_then(to_param) {
                        Ok(local) => {
                            param = local;
                            edge = *merged;
                        }
                        Err(cause) => return failed(edge, cause),
                    }
                }
                TopologyEdit::Reverse { edge: reversed } if *reversed == edge => {
                    match param.as_rational().one_minus().and_then(to_param) {
                        Ok(reflected) => {
                            param = reflected;
                            direction = direction.reversed();
                        }
                        Err(cause) => return failed(edge, cause),
                    }
                }
                TopologyEdit::Delete { edge: deleted } if *deleted == edge => {
                    return Resolution::Unresolved(RepairTask::new(
                        held_edge,
                        held_param,
                        OrphaningEdit::Deleted {
                            operation: entry.operation(),
                            edge,
                        },
                        Vec::new(),
                    ));
                }
                TopologyEdit::Offset { source, fragments } if *source == edge => {
                    // Validated intervals ascend without overlapping and each is non-empty, so a
                    // parameter sits in at most two of them — exactly two only on a shared boundary.
                    // The count decides, per §1.1's fifth row: one maps exactly, two are refused with
                    // both as candidates, none is refused with none.
                    let t = param.as_rational();
                    let containing: Vec<&OffsetFragment> = fragments
                        .iter()
                        .filter(|fragment| fragment.from() <= param && param <= fragment.to())
                        .collect();
                    let mut candidates: Vec<ResolvedRef> = Vec::with_capacity(containing.len());
                    for fragment in &containing {
                        // The affine map into the fragment's own parameter space:
                        // `(t − from)/(to − from)`, exact; `to > from` is the interval's invariant.
                        let mapped = fragment
                            .to()
                            .as_rational()
                            .checked_sub(fragment.from().as_rational())
                            .and_then(|width| {
                                t.checked_sub(fragment.from().as_rational())
                                    .and_then(|shifted| shifted.checked_div(width))
                            })
                            .and_then(to_param)
                            .map(|local| ResolvedRef::new(fragment.edge(), local, direction));
                        match mapped {
                            Ok(position) => candidates.push(position),
                            Err(cause) => return failed(edge, cause),
                        }
                    }
                    if candidates.len() > 1 {
                        return Resolution::Unresolved(RepairTask::new(
                            held_edge,
                            held_param,
                            OrphaningEdit::OffsetAmbiguous {
                                operation: entry.operation(),
                                source: *source,
                            },
                            candidates,
                        ));
                    }
                    match candidates.into_iter().next() {
                        Some(position) => {
                            param = position.param();
                            edge = position.edge();
                        }
                        None => {
                            return Resolution::Unresolved(RepairTask::new(
                                held_edge,
                                held_param,
                                OrphaningEdit::OffsetUnmapped {
                                    operation: entry.operation(),
                                    source: *source,
                                },
                                Vec::new(),
                            ))
                        }
                    }
                }
                _ => {}
            }
        }
        Resolution::Resolved(ResolvedRef::new(edge, param, direction))
    }
}

/// Lifts an exact rational into a [`Param`], mapping the out-of-range refusal into the same typed-
/// diagnostic channel as an arithmetic refusal, so the fold stays total without an `unwrap` the
/// workspace forbids. Every fold step mathematically preserves `[0, 1]`, so the mapped arm is
/// unreachable through a validated journal; it exists because "unreachable" is a claim, and a claim in a
/// total function needs a typed consequence, not a trust me.
fn to_param(value: Rational) -> Result<Param, UnitError> {
    Param::new(value).map_err(|_| UnitError::DomainExceeded {
        kind: "edge parameter",
        value: i128::from(value.numerator()),
        limit: i128::from(value.denominator()),
    })
}

#[cfg(test)]
mod tests {
    //! Unit tests for the persistent-identity contract. Every §1.1 row has its mechanics proven here on
    //! exact rationals; the randomized stability properties live in `tests/identity_contract_property.rs`.

    // The workspace denies `unwrap_used` and `panic` because production code returns a diagnostic rather
    // than aborting a session; a test is the opposite case, so both are allowed here (`.clippy.toml`'s
    // `allow-unwrap-in-tests` reaches the `unwrap`s, and the match-arm `panic!`s are the loud fallback).
    #![allow(clippy::unwrap_used, clippy::panic)]

    use super::{
        Direction, IdentityLedger, LedgerError, OffsetInterval, OpenRepair, OrphaningEdit,
        ReleaseReadiness, Resolution, SplitSide, TopologyEdit, MAX_EDGES_PER_OPERATION,
    };
    use crate::ontology::id::{DeterministicIdGenerator, EntityId, IdGenerator};
    use crate::ontology::rational::Rational;
    use crate::ontology::reference::{EdgeRef, LocalTag, Param};
    use sc_units::{Length, UnitError};

    /// An exact parameter `num/den` inside `[0, 1]`.
    fn param(num: i64, den: i64) -> Param {
        Param::new(Rational::new(num, den).unwrap()).unwrap()
    }

    /// A length in micrometres, inside the declared domain.
    fn length(micrometres: i64) -> Length {
        Length::from_micrometres(micrometres).unwrap()
    }

    /// A ledger whose first operation declared `count` edges.
    fn ledger_with_edges(count: usize) -> (IdentityLedger, DeterministicIdGenerator, Vec<EdgeRef>) {
        let mut ledger = IdentityLedger::new();
        let mut ids = DeterministicIdGenerator::new();
        let edges = ledger.declare_edges(&mut ids, count).unwrap();
        (ledger, ids, edges)
    }

    /// The one live edge of a fresh ledger.
    fn one_edge() -> (IdentityLedger, DeterministicIdGenerator, EdgeRef) {
        let (ledger, ids, edges) = ledger_with_edges(1);
        (ledger, ids, *edges.first().unwrap())
    }

    /// A parameterization no fold can recompute: `t/at` whose reduced numerator is `~2^124`. Both values
    /// are honest `[0, 1]` rationals with `t < at`, so the refusal is the arithmetic's, exactly where the
    /// contract says it must surface — as a visible task, never a rounding. Registered *before* the split
    /// (a live edge always resolves), the pair orphans through the edit that follows.
    fn unresolvable_pair() -> (Param, Param) {
        let two62 = 1i64 << 62;
        (
            param(two62 + 1, two62 + 5), // t
            param(two62 + 3, two62 + 7), // at, with t < at and gcd(num, den) = 1 on the quotient
        )
    }

    #[test]
    fn declared_edges_are_live_and_carry_their_creating_operation() {
        let (ledger, _ids, edges) = ledger_with_edges(3);
        assert_eq!(edges.len(), 3);
        assert!(edges.iter().all(|edge| ledger.is_live(*edge)));
        let creator = edges.first().unwrap().creator();
        assert!(
            edges.iter().all(|edge| edge.creator() == creator),
            "one drafting operation, one creator id"
        );
        let tags: Vec<LocalTag> = edges.iter().map(|edge| edge.tag()).collect();
        assert_eq!(
            tags,
            vec![LocalTag::new(0), LocalTag::new(1), LocalTag::new(2)]
        );
        assert_eq!(ledger.journal().len(), 1);
        assert!(matches!(
            ledger.journal().first().unwrap().edit(),
            TopologyEdit::Declared { edges: declared } if declared.len() == 3
        ));
    }

    #[test]
    fn a_declaration_outside_its_domain_is_refused_and_mints_nothing() {
        let mut ledger = IdentityLedger::new();
        let mut ids = DeterministicIdGenerator::new();
        assert_eq!(
            ledger.declare_edges(&mut ids, 0).unwrap_err(),
            LedgerError::NoEdges
        );
        assert!(matches!(
            ledger
                .declare_edges(&mut ids, MAX_EDGES_PER_OPERATION + 1)
                .unwrap_err(),
            LedgerError::TooManyEdges { count } if count == MAX_EDGES_PER_OPERATION + 1
        ));
        assert!(
            ledger.journal().is_empty(),
            "a refused edit records nothing"
        );
        assert_eq!(
            ids.next_id(),
            EntityId::from_bits(1),
            "a refused edit consumes no identity, so a failing script replays like a passing one"
        );
    }

    #[test]
    fn an_edit_on_an_edge_that_is_not_live_is_refused() {
        let (mut ledger, mut ids, _edges) = ledger_with_edges(1);
        let stranger = EdgeRef::new(EntityId::from_bits(999), LocalTag::FIRST);
        assert!(matches!(
            ledger.split(&mut ids, stranger, param(1, 2)).unwrap_err(),
            LedgerError::NotLive { edge } if edge == stranger
        ));
        assert!(matches!(
            ledger.reverse(&mut ids, stranger).unwrap_err(),
            LedgerError::NotLive { .. }
        ));
        assert!(matches!(
            ledger.delete(&mut ids, stranger).unwrap_err(),
            LedgerError::NotLive { .. }
        ));
        assert!(matches!(
            ledger
                .offset(
                    &mut ids,
                    stranger,
                    &[OffsetInterval::new(param(0, 1), param(1, 1)).unwrap()]
                )
                .unwrap_err(),
            LedgerError::NotLive { .. }
        ));
        assert_eq!(
            ids.next_id(),
            EntityId::from_bits(2),
            "only the declaration minted; every refusal consumed nothing"
        );
    }

    #[test]
    fn split_replaces_the_source_with_two_fragments_of_the_split_operation() {
        let (mut ledger, mut ids, edge) = one_edge();
        let fragments = ledger.split(&mut ids, edge, param(1, 3)).unwrap();
        assert!(!ledger.is_live(edge), "the source is consumed");
        assert!(ledger.is_live(fragments.first()));
        assert!(ledger.is_live(fragments.second()));
        assert_eq!(
            fragments.first().creator(),
            fragments.second().creator(),
            "both fragments were created by the split operation"
        );
        assert_ne!(fragments.first().creator(), edge.creator());
        assert_eq!(fragments.first().tag(), LocalTag::FIRST);
        assert_eq!(fragments.second().tag(), LocalTag::new(1));
    }

    #[test]
    fn split_at_an_endpoint_is_refused() {
        let (mut ledger, mut ids, edge) = one_edge();
        assert_eq!(
            ledger.split(&mut ids, edge, Param::START).unwrap_err(),
            LedgerError::SplitAtEndpoint { at: Param::START }
        );
        assert_eq!(
            ledger.split(&mut ids, edge, Param::END).unwrap_err(),
            LedgerError::SplitAtEndpoint { at: Param::END }
        );
        assert!(ledger.is_live(edge), "a refused split changes nothing");
    }

    #[test]
    fn split_resolves_a_reference_into_the_fragment_that_contains_its_parameter() {
        let (mut ledger, mut ids, edge) = one_edge();
        let fragments = ledger.split(&mut ids, edge, param(1, 3)).unwrap();
        // t = 1/6 < 1/3: first fragment, at (1/6)/(1/3) = 1/2 exactly.
        let below = ledger.resolve(edge, param(1, 6)).resolved().unwrap();
        assert_eq!(below.edge(), fragments.first());
        assert_eq!(below.param(), param(1, 2));
        assert_eq!(below.direction(), Direction::Original);
        // t = 2/3 > 1/3: second fragment, at (2/3 − 1/3)/(1 − 1/3) = 1/2 exactly.
        let above = ledger.resolve(edge, param(2, 3)).resolved().unwrap();
        assert_eq!(above.edge(), fragments.second());
        assert_eq!(above.param(), param(1, 2));
    }

    #[test]
    fn a_reference_at_the_split_point_resolves_to_both_and_the_consumer_states_which_it_wants() {
        let (mut ledger, mut ids, edge) = one_edge();
        let fragments = ledger.split(&mut ids, edge, param(1, 3)).unwrap();
        let at_point = ledger.resolve(edge, param(1, 3));
        assert!(
            matches!(at_point, Resolution::SplitPoint { .. }),
            "the boundary case is visible, not silently assigned: {at_point:?}"
        );
        let first = at_point
            .clone()
            .choose(SplitSide::First)
            .resolved()
            .unwrap();
        assert_eq!(first.edge(), fragments.first());
        assert_eq!(first.param(), Param::END);
        let second = at_point
            .clone()
            .choose(SplitSide::Second)
            .resolved()
            .unwrap();
        assert_eq!(second.edge(), fragments.second());
        assert_eq!(second.param(), Param::START);
        // A stated side on a unique resolution is the identity — nothing to choose, nothing reassigned.
        let unique = ledger.resolve(edge, param(1, 6));
        assert_eq!(unique.clone().choose(SplitSide::Second), unique);
    }

    #[test]
    fn merge_requires_two_distinct_live_edges() {
        let (mut ledger, mut ids, edges) = ledger_with_edges(2);
        let (e0, e1) = (*edges.first().unwrap(), *edges.get(1).unwrap());
        assert_eq!(
            ledger
                .merge(&mut ids, e0, length(1), e0, length(1))
                .unwrap_err(),
            LedgerError::MergeSameEdge { edge: e0 }
        );
        ledger.delete(&mut ids, e1).unwrap();
        assert!(matches!(
            ledger
                .merge(&mut ids, e0, length(1), e1, length(1))
                .unwrap_err(),
            LedgerError::NotLive { .. }
        ));
    }

    #[test]
    fn merge_refuses_a_non_positive_arc_length() {
        let (mut ledger, mut ids, edges) = ledger_with_edges(2);
        let (e0, e1) = (*edges.first().unwrap(), *edges.get(1).unwrap());
        assert!(matches!(
            ledger
                .merge(&mut ids, e0, Length::ZERO, e1, length(10))
                .unwrap_err(),
            LedgerError::NonPositiveLength { side: "first", .. }
        ));
        assert!(matches!(
            ledger
                .merge(&mut ids, e0, length(10), e1, length(-5))
                .unwrap_err(),
            LedgerError::NonPositiveLength { side: "second", .. }
        ));
    }

    #[test]
    fn merge_recomputes_the_parameter_by_arc_length() {
        let (mut ledger, mut ids, edges) = ledger_with_edges(2);
        let (e0, e1) = (*edges.first().unwrap(), *edges.get(1).unwrap());
        // 1 cm then 3 cm: the seam point sits at 1/4 of the merged edge.
        let merged = ledger
            .merge(&mut ids, e0, length(10_000), e1, length(30_000))
            .unwrap();
        assert!(!ledger.is_live(e0) && !ledger.is_live(e1) && ledger.is_live(merged));
        // From the first edge: t·L₁/(L₁+L₂) = (1/2)·(1/4) = 1/8.
        let from_first = ledger.resolve(e0, param(1, 2)).resolved().unwrap();
        assert_eq!(from_first.edge(), merged);
        assert_eq!(from_first.param(), param(1, 8));
        assert_eq!(from_first.direction(), Direction::Original);
        // From the second edge: (L₁ + t·L₂)/(L₁+L₂) = (1 + 3/2)/4 = 5/8.
        let from_second = ledger.resolve(e1, param(1, 2)).resolved().unwrap();
        assert_eq!(from_second.edge(), merged);
        assert_eq!(from_second.param(), param(5, 8));
        // The seam point is one position from both sides: END of the first, START of the second.
        assert_eq!(
            ledger.resolve(e0, Param::END).resolved().unwrap().param(),
            param(1, 4)
        );
        assert_eq!(
            ledger.resolve(e1, Param::START).resolved().unwrap().param(),
            param(1, 4)
        );
    }

    #[test]
    fn reverse_keeps_the_identity_maps_t_to_one_minus_t_and_tells_directed_consumers() {
        let (mut ledger, mut ids, edge) = one_edge();
        ledger.reverse(&mut ids, edge).unwrap();
        let reversed = ledger.resolve(edge, param(1, 4)).resolved().unwrap();
        assert_eq!(reversed.edge(), edge, "the identity survives");
        assert_eq!(reversed.param(), param(3, 4));
        assert_eq!(reversed.direction(), Direction::Reversed);
        ledger.reverse(&mut ids, edge).unwrap();
        let twice = ledger.resolve(edge, param(1, 4)).resolved().unwrap();
        assert_eq!(twice.param(), param(1, 4), "1 − (1 − t) = t, exactly");
        assert_eq!(twice.direction(), Direction::Original);
    }

    #[test]
    fn the_fold_composes_edits_in_journal_order() {
        let (mut ledger, mut ids, edge) = one_edge();
        ledger.reverse(&mut ids, edge).unwrap();
        let fragments = ledger.split(&mut ids, edge, param(1, 4)).unwrap();
        // t = 5/8 reverses to 3/8, which is above the split point 1/4:
        // (3/8 − 1/4)/(3/4) = 1/6 on the second fragment, direction still reversed.
        let resolved = ledger.resolve(edge, param(5, 8)).resolved().unwrap();
        assert_eq!(resolved.edge(), fragments.second());
        assert_eq!(resolved.param(), param(1, 6));
        assert_eq!(resolved.direction(), Direction::Reversed);
    }

    #[test]
    fn delete_orphans_a_reference_into_a_visible_repair_task() {
        let (mut ledger, mut ids, edge) = one_edge();
        let holder = EntityId::from_bits(77);
        ledger.register(holder, edge, param(1, 2)).unwrap();
        let deleting = ledger.delete(&mut ids, edge).unwrap();
        let unresolved = ledger.resolve(edge, param(1, 2));
        let task = match &unresolved {
            Resolution::Unresolved(task) => task,
            other => panic!("a deleted edge's reference must be unresolved, got {other:?}"),
        };
        assert_eq!(task.reference(), edge, "the task names the held reference");
        assert_eq!(task.param(), param(1, 2));
        assert_eq!(
            *task.orphaned_by(),
            OrphaningEdit::Deleted {
                operation: deleting,
                edge
            },
            "and the edit that orphaned it"
        );
        assert!(
            task.candidates().is_empty(),
            "a deletion leaves nothing to suggest; inventing a candidate would be the silent reassignment"
        );
    }

    #[test]
    fn an_orphan_through_a_chain_names_the_held_reference_not_the_intermediate() {
        let (mut ledger, mut ids, edge) = one_edge();
        let fragments = ledger.split(&mut ids, edge, param(1, 2)).unwrap();
        let deleting = ledger.delete(&mut ids, fragments.second()).unwrap();
        // The holder states (edge, 3/4); the fold carried it onto the second fragment, which died.
        let unresolved = ledger.resolve(edge, param(3, 4));
        let task = match &unresolved {
            Resolution::Unresolved(task) => task,
            other => panic!("expected an unresolved reference, got {other:?}"),
        };
        assert_eq!(task.reference(), edge);
        assert_eq!(task.param(), param(3, 4));
        assert_eq!(
            *task.orphaned_by(),
            OrphaningEdit::Deleted {
                operation: deleting,
                edge: fragments.second()
            },
            "the orphaning edit names the edge that actually died"
        );
    }

    #[test]
    fn offset_intervals_are_validated_where_they_are_made_and_where_they_are_used() {
        assert_eq!(
            OffsetInterval::new(param(1, 2), param(1, 2)).unwrap_err(),
            LedgerError::IntervalEmpty {
                from: param(1, 2),
                to: param(1, 2)
            }
        );
        assert!(matches!(
            OffsetInterval::new(param(3, 4), param(1, 4)).unwrap_err(),
            LedgerError::IntervalEmpty { .. }
        ));
        let (mut ledger, mut ids, edge) = one_edge();
        assert_eq!(
            ledger.offset(&mut ids, edge, &[]).unwrap_err(),
            LedgerError::NoEdges
        );
        // Overlap: the next starts before the previous ends.
        let overlapping = [
            OffsetInterval::new(param(0, 1), param(1, 2)).unwrap(),
            OffsetInterval::new(param(1, 4), param(3, 4)).unwrap(),
        ];
        assert_eq!(
            ledger.offset(&mut ids, edge, &overlapping).unwrap_err(),
            LedgerError::IntervalsUnordered {
                previous_to: param(1, 2),
                next_from: param(1, 4)
            }
        );
        assert!(ledger.is_live(edge), "a refused offset changes nothing");
        // The TooManyEdges arm of `offset` shares MAX_EDGES_PER_OPERATION with `declare_edges`, whose
        // refusal the declaration test proves without allocating a million-interval vector here.
    }

    #[test]
    fn offset_maps_an_unambiguous_position_onto_its_fragment_exactly() {
        let (mut ledger, mut ids, edge) = one_edge();
        let intervals = [
            OffsetInterval::new(Param::START, param(1, 2)).unwrap(),
            OffsetInterval::new(param(1, 2), Param::END).unwrap(),
        ];
        let fragments = ledger.offset(&mut ids, edge, &intervals).unwrap();
        assert!(!ledger.is_live(edge), "the source is fragmented away");
        assert_eq!(fragments.len(), 2);
        assert!(fragments.iter().all(|fragment| ledger.is_live(*fragment)));
        // t = 1/4 lies in [0, 1/2]: (1/4 − 0)/(1/2 − 0) = 1/2 on the first fragment.
        let low = ledger.resolve(edge, param(1, 4)).resolved().unwrap();
        assert_eq!(low.edge(), *fragments.first().unwrap());
        assert_eq!(low.param(), param(1, 2));
        // t = 3/4 lies in [1/2, 1]: (3/4 − 1/2)/(1 − 1/2) = 1/2 on the second fragment.
        let high = ledger.resolve(edge, param(3, 4)).resolved().unwrap();
        assert_eq!(high.edge(), *fragments.get(1).unwrap());
        assert_eq!(high.param(), param(1, 2));
    }

    #[test]
    fn offset_at_a_shared_boundary_is_ambiguous_with_both_fragments_as_candidates() {
        let (mut ledger, mut ids, edge) = one_edge();
        let intervals = [
            OffsetInterval::new(Param::START, param(1, 2)).unwrap(),
            OffsetInterval::new(param(1, 2), Param::END).unwrap(),
        ];
        let fragments = ledger.offset(&mut ids, edge, &intervals).unwrap();
        let offset_op = fragments.first().unwrap().creator();
        let unresolved = ledger.resolve(edge, param(1, 2));
        let task = match &unresolved {
            Resolution::Unresolved(task) => task,
            other => panic!("a shared boundary must be refused, got {other:?}"),
        };
        assert_eq!(
            *task.orphaned_by(),
            OrphaningEdit::OffsetAmbiguous {
                operation: offset_op,
                source: edge
            }
        );
        assert_eq!(task.reference(), edge);
        let candidates = task.candidates();
        assert_eq!(candidates.len(), 2, "both fragments, and only those two");
        assert_eq!(
            candidates.first().unwrap().edge(),
            *fragments.first().unwrap()
        );
        assert_eq!(candidates.first().unwrap().param(), Param::END);
        assert_eq!(
            candidates.get(1).unwrap().edge(),
            *fragments.get(1).unwrap()
        );
        assert_eq!(candidates.get(1).unwrap().param(), Param::START);
    }

    #[test]
    fn offset_in_a_trimmed_gap_is_unmapped_without_candidates() {
        let (mut ledger, mut ids, edge) = one_edge();
        let intervals = [
            OffsetInterval::new(Param::START, param(1, 4)).unwrap(),
            OffsetInterval::new(param(3, 4), Param::END).unwrap(),
        ];
        ledger.offset(&mut ids, edge, &intervals).unwrap();
        let unresolved = ledger.resolve(edge, param(1, 2));
        let task = match &unresolved {
            Resolution::Unresolved(task) => task,
            other => panic!("a trimmed-away position must be unresolved, got {other:?}"),
        };
        assert!(matches!(
            task.orphaned_by(),
            OrphaningEdit::OffsetUnmapped { source, .. } if *source == edge
        ));
        assert!(task.candidates().is_empty());
    }

    #[test]
    fn registration_requires_a_live_edge() {
        let (mut ledger, _ids, edge) = one_edge();
        let holder = EntityId::from_bits(7);
        let stranger = EdgeRef::new(EntityId::from_bits(999), LocalTag::FIRST);
        assert!(matches!(
            ledger.register(holder, stranger, param(1, 2)).unwrap_err(),
            LedgerError::NotLive { .. }
        ));
        assert!(ledger.register(holder, edge, param(1, 2)).is_ok());
    }

    #[test]
    fn an_edit_that_cannot_recompute_a_held_reference_exactly_orphans_it_visibly() {
        let (mut ledger, mut ids, edge) = one_edge();
        let (t, at) = unresolvable_pair();
        let holder = EntityId::from_bits(7);
        // Registered while the edge is live: born resolved, as the liveness theorem guarantees.
        ledger.register(holder, edge, t).unwrap();
        assert!(ledger.open_repairs().is_empty());
        // The split's exact recomputation t/at does not fit the bounded rational. The edit is valid and
        // lands; the reference becomes an open repair instead of being rounded.
        ledger.split(&mut ids, edge, at).unwrap();
        let repairs = ledger.open_repairs();
        assert_eq!(repairs.len(), 1);
        let task = repairs.first().unwrap().task();
        assert_eq!(task.reference(), edge, "the task names the held reference");
        assert_eq!(task.param(), t);
        match task.orphaned_by() {
            OrphaningEdit::RecomputationFailed {
                operation,
                edge: failed_on,
                cause,
            } => {
                assert_eq!(*operation, ledger.journal().last().unwrap().operation());
                assert_eq!(*failed_on, edge);
                assert!(
                    matches!(cause, UnitError::Overflow { .. }),
                    "the exact cause rides along: {cause}"
                );
            }
            other => panic!("expected RecomputationFailed, got {other:?}"),
        }
        assert!(matches!(
            ledger.release_readiness(),
            ReleaseReadiness::Blocked { unresolved: 1 }
        ));
    }

    #[test]
    fn an_unresolvable_recomputation_becomes_a_repair_task_naming_the_cause() {
        let (mut ledger, mut ids, edge) = one_edge();
        let (t, at) = unresolvable_pair();
        ledger.split(&mut ids, edge, at).unwrap();
        let split_operation = ledger.journal().last().unwrap().operation();
        let unresolved = ledger.resolve(edge, t);
        let task = match &unresolved {
            Resolution::Unresolved(task) => task,
            other => panic!("expected an unresolved reference, got {other:?}"),
        };
        match task.orphaned_by() {
            OrphaningEdit::RecomputationFailed {
                operation,
                edge: failed_on,
                cause,
            } => {
                assert_eq!(*operation, split_operation);
                assert_eq!(*failed_on, edge);
                assert!(
                    matches!(cause, UnitError::Overflow { .. }),
                    "the exact cause rides along: {cause}"
                );
            }
            other => panic!("expected RecomputationFailed, got {other:?}"),
        }
    }

    #[test]
    fn an_edge_the_ledger_never_saw_passes_through_resolved() {
        let (ledger, _ids, _edge) = one_edge();
        let stranger = EdgeRef::new(EntityId::from_bits(555), LocalTag::new(3));
        let resolved = ledger.resolve(stranger, param(2, 5)).resolved().unwrap();
        assert_eq!(resolved.edge(), stranger);
        assert_eq!(resolved.param(), param(2, 5));
        assert_eq!(resolved.direction(), Direction::Original);
    }

    #[test]
    fn open_repairs_enumerate_exactly_the_unresolved_registrations() {
        let (mut ledger, mut ids, edges) = ledger_with_edges(3);
        let (e0, e1, e2) = (
            *edges.first().unwrap(),
            *edges.get(1).unwrap(),
            *edges.get(2).unwrap(),
        );
        let alice = EntityId::from_bits(101);
        let bob = EntityId::from_bits(102);
        ledger.register(alice, e0, param(1, 2)).unwrap();
        ledger.register(bob, e1, param(1, 4)).unwrap();
        ledger.register(alice, e2, param(3, 4)).unwrap();
        assert!(ledger.open_repairs().is_empty());
        ledger.delete(&mut ids, e1).unwrap();
        let repairs = ledger.open_repairs();
        assert_eq!(repairs.len(), 1, "exactly bob's reference died");
        let repair: &OpenRepair = repairs.first().unwrap();
        assert_eq!(repair.owner(), bob);
        assert_eq!(repair.task().reference(), e1);
        assert_eq!(repair.task().param(), param(1, 4));
    }

    #[test]
    fn a_design_with_unresolved_references_is_inspectable_but_not_releasable() {
        let (mut ledger, mut ids, edges) = ledger_with_edges(2);
        let (e0, e1) = (*edges.first().unwrap(), *edges.get(1).unwrap());
        let holder = EntityId::from_bits(42);
        ledger.register(holder, e0, param(1, 2)).unwrap();
        assert_eq!(ledger.release_readiness(), ReleaseReadiness::Releasable);
        ledger.delete(&mut ids, e0).unwrap();
        // Not releasable —
        assert_eq!(
            ledger.release_readiness(),
            ReleaseReadiness::Blocked { unresolved: 1 }
        );
        // — yet every inspection still answers: this is the "savable and inspectable" half of §1.1.
        assert_eq!(ledger.journal().len(), 2);
        assert_eq!(ledger.live_edges().count(), 1);
        assert_eq!(ledger.registrations().count(), 1);
        assert_eq!(ledger.open_repairs().len(), 1);
        assert!(matches!(
            ledger.resolve(holder_and_param_edge(&ledger, holder), param(1, 2)),
            Resolution::Unresolved(_)
        ));
        // Retiring the orphaned reference — the holder lets it go — restores releasability.
        ledger.retire(holder, e0, param(1, 2)).unwrap();
        assert_eq!(ledger.release_readiness(), ReleaseReadiness::Releasable);
        assert!(ledger.is_live(e1));
    }

    /// The edge a holder's single registration names (inspection stays available under open repairs).
    fn holder_and_param_edge(ledger: &IdentityLedger, holder: EntityId) -> EdgeRef {
        ledger
            .registrations()
            .find(|registration| registration.owner() == holder)
            .unwrap()
            .edge()
    }

    #[test]
    fn repoint_is_the_explicit_repair_and_restores_releasability() {
        let (mut ledger, mut ids, edges) = ledger_with_edges(2);
        let (e0, e1) = (*edges.first().unwrap(), *edges.get(1).unwrap());
        let holder = EntityId::from_bits(42);
        ledger.register(holder, e0, param(1, 2)).unwrap();
        ledger.delete(&mut ids, e0).unwrap();
        assert!(matches!(
            ledger.release_readiness(),
            ReleaseReadiness::Blocked { unresolved: 1 }
        ));
        ledger
            .repoint(holder, e0, param(1, 2), e1, param(1, 4))
            .unwrap();
        assert_eq!(ledger.release_readiness(), ReleaseReadiness::Releasable);
        let repaired = ledger
            .resolve_registered(holder, e1, param(1, 4))
            .unwrap()
            .resolved()
            .unwrap();
        assert_eq!(repaired.edge(), e1);
        assert_eq!(repaired.param(), param(1, 4));
        // The repointed reference's registration carries no split choice, and the old one is gone.
        assert!(matches!(
            ledger.retire(holder, e0, param(1, 2)).unwrap_err(),
            LedgerError::UnknownRegistration { .. }
        ));
        // Repoint refuses an unknown old registration and a dead new edge.
        assert!(matches!(
            ledger
                .repoint(holder, e1, param(9, 10), e1, param(1, 4))
                .unwrap_err(),
            LedgerError::UnknownRegistration { .. }
        ));
        assert!(matches!(
            ledger
                .repoint(holder, e1, param(1, 4), e0, param(1, 4))
                .unwrap_err(),
            LedgerError::NotLive { .. }
        ));
    }

    #[test]
    fn a_stated_split_choice_is_applied_to_a_registered_reference() {
        let (mut ledger, mut ids, edge) = one_edge();
        let holder = EntityId::from_bits(42);
        // The consumer registers while the edge is live; the split comes after and does not touch the
        // registration — references are never rewritten.
        ledger.register(holder, edge, param(1, 2)).unwrap();
        let fragments = ledger.split(&mut ids, edge, param(1, 2)).unwrap();
        assert!(matches!(
            ledger
                .resolve_registered(holder, edge, param(1, 2))
                .unwrap(),
            Resolution::SplitPoint { .. }
        ));
        ledger
            .state_split_choice(holder, edge, param(1, 2), SplitSide::Second)
            .unwrap();
        let chosen = ledger
            .resolve_registered(holder, edge, param(1, 2))
            .unwrap()
            .resolved()
            .unwrap();
        assert_eq!(chosen.edge(), fragments.second());
        assert_eq!(chosen.param(), Param::START);
        let registration = ledger.registrations().next().unwrap();
        assert_eq!(registration.split_choice(), Some(SplitSide::Second));
        // An unknown registration is a typed refusal on every operation that names one.
        let nobody = EntityId::from_bits(43);
        assert!(matches!(
            ledger
                .resolve_registered(nobody, edge, param(1, 2))
                .unwrap_err(),
            LedgerError::UnknownRegistration { .. }
        ));
        assert!(matches!(
            ledger
                .state_split_choice(nobody, edge, param(1, 2), SplitSide::First)
                .unwrap_err(),
            LedgerError::UnknownRegistration { .. }
        ));
        assert!(matches!(
            ledger.retire(nobody, edge, param(1, 2)).unwrap_err(),
            LedgerError::UnknownRegistration { .. }
        ));
    }

    #[test]
    fn duplicate_registration_is_idempotent_and_keeps_the_stated_choice() {
        let (mut ledger, _ids, edge) = one_edge();
        let holder = EntityId::from_bits(42);
        ledger.register(holder, edge, param(1, 2)).unwrap();
        // A split choice may be stated ahead of the split that needs it: it is a preference about a
        // frame, applied only where a split point actually arises.
        ledger
            .state_split_choice(holder, edge, param(1, 2), SplitSide::First)
            .unwrap();
        ledger.register(holder, edge, param(1, 2)).unwrap();
        assert_eq!(
            ledger.registrations().count(),
            1,
            "one reference, one entry"
        );
        assert_eq!(
            ledger.registrations().next().unwrap().split_choice(),
            Some(SplitSide::First),
            "re-registering does not silently withdraw a stated choice"
        );
    }

    #[test]
    fn retire_withdraws_the_registration() {
        let (mut ledger, _ids, edge) = one_edge();
        let holder = EntityId::from_bits(42);
        ledger.register(holder, edge, param(1, 2)).unwrap();
        ledger.retire(holder, edge, param(1, 2)).unwrap();
        assert_eq!(ledger.registrations().count(), 0);
        assert!(matches!(
            ledger.retire(holder, edge, param(1, 2)).unwrap_err(),
            LedgerError::UnknownRegistration { .. }
        ));
    }

    #[test]
    fn the_journal_records_every_edit_in_order_with_increasing_operations() {
        let (mut ledger, mut ids, edges) = ledger_with_edges(3);
        let (e0, e1, e2) = (
            *edges.first().unwrap(),
            *edges.get(1).unwrap(),
            *edges.get(2).unwrap(),
        );
        ledger.split(&mut ids, e0, param(1, 2)).unwrap();
        ledger.reverse(&mut ids, e1).unwrap();
        ledger.delete(&mut ids, e2).unwrap();
        let journal = ledger.journal();
        assert_eq!(journal.len(), 4, "declare, split, reverse, delete");
        let mut previous = Option::<EntityId>::None;
        for entry in journal {
            if let Some(before) = previous {
                assert!(before < entry.operation(), "operations increase");
            }
            previous = Some(entry.operation());
        }
        assert!(matches!(
            journal.first().unwrap().edit(),
            TopologyEdit::Declared { .. }
        ));
        assert!(matches!(
            journal.get(1).unwrap().edit(),
            TopologyEdit::Split { source, .. } if *source == e0
        ));
        assert!(matches!(
            journal.get(2).unwrap().edit(),
            TopologyEdit::Reverse { edge } if *edge == e1
        ));
        assert!(matches!(
            journal.last().unwrap().edit(),
            TopologyEdit::Delete { edge } if *edge == e2
        ));
    }

    #[test]
    fn a_split_point_whose_side_died_offers_the_survivor_and_shows_the_task_on_the_dead_side() {
        let (mut ledger, mut ids, edge) = one_edge();
        let holder = EntityId::from_bits(42);
        ledger.register(holder, edge, param(1, 2)).unwrap();
        let fragments = ledger.split(&mut ids, edge, param(1, 2)).unwrap();
        let deleting = ledger.delete(&mut ids, fragments.second()).unwrap();
        // The split point still exists geometrically — as the surviving fragment's end — so the
        // reference is not orphaned: the fold offers the survivor AND the task on the dead side.
        // Nothing is collapsed silently in either direction.
        match ledger.resolve(edge, param(1, 2)) {
            Resolution::SplitPoint { first, second } => {
                let live = first.resolved().unwrap();
                assert_eq!(live.edge(), fragments.first());
                assert_eq!(live.param(), Param::END);
                match &*second {
                    Resolution::Unresolved(task) => {
                        assert_eq!(
                            *task.orphaned_by(),
                            OrphaningEdit::Deleted {
                                operation: deleting,
                                edge: fragments.second()
                            }
                        );
                    }
                    other => panic!("the dead side must carry its task, got {other:?}"),
                }
            }
            other => panic!("expected a split point, got {other:?}"),
        }
        // With no side stated the reference resolves, so it is not an open repair.
        assert!(ledger.open_repairs().is_empty());
        assert_eq!(ledger.release_readiness(), ReleaseReadiness::Releasable);
        // Stating the dead side surfaces the task; stating the survivor clears it. The consumer's
        // statement — never the ledger — decides which fate the reference has.
        ledger
            .state_split_choice(holder, edge, param(1, 2), SplitSide::Second)
            .unwrap();
        assert_eq!(ledger.open_repairs().len(), 1);
        assert!(matches!(
            ledger.release_readiness(),
            ReleaseReadiness::Blocked { unresolved: 1 }
        ));
        ledger
            .state_split_choice(holder, edge, param(1, 2), SplitSide::First)
            .unwrap();
        assert!(ledger.open_repairs().is_empty());
    }

    #[test]
    fn diagnostics_name_the_offender_when_displayed() {
        let (mut ledger, mut ids, edge) = one_edge();
        let holder = EntityId::from_bits(42);
        ledger.register(holder, edge, param(1, 2)).unwrap();
        ledger.delete(&mut ids, edge).unwrap();
        let err = ledger.delete(&mut ids, edge).unwrap_err();
        assert!(
            err.to_string().contains(&edge.to_string()),
            "the NotLive diagnostic names the edge: {err}"
        );
        let task = &ledger.open_repairs().first().unwrap().task().clone();
        let text = task.to_string();
        assert!(text.contains("repair task"), "{text}");
        assert!(text.contains(&edge.to_string()), "{text}");
        assert!(
            ledger
                .release_readiness()
                .to_string()
                .contains("not releasable"),
            "{}",
            ledger.release_readiness()
        );
        assert_eq!(
            Direction::Reversed.reversed(),
            Direction::Original,
            "the direction accessor round-trips"
        );
        assert_eq!(SplitSide::First.to_string(), "first fragment");
        assert!(ledger
            .open_repairs()
            .first()
            .unwrap()
            .to_string()
            .contains("held by"));
    }
}
