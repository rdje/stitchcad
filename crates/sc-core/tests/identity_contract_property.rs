//! Property tests for the persistent-identity contract (`G1-SLICE.3b`), as executable evidence for the
//! acceptance criteria: reference stability is a tested property under split, merge and reverse; a
//! reference at the split point resolves to both fragments and the consumer states which it wants; merge
//! recomputes by arc length; reverse maps `t` to `1 − t`; a deleted edge's references become visible
//! repair tasks and nothing else moves.
//!
//! Hand-rolled with a **recorded seed and no dependencies**, per
//! `docs/decisions/decision_property-tests-dependency-free-recorded-seed.md` — the same seed and generator
//! as `sc-units`' and `.3a`'s suites, so every property test in the repository is reproducible from one
//! constant. Where a property needs an oracle, the oracle is a *different derivation* than the code under
//! test: merge is checked against its defining proportion cross-multiplied in `i128`, offset against an
//! integer grid the test walks itself, and the live-edge set against an independent replay of the journal.
//!
//! The workspace denies `clippy::panic` because production code returns a diagnostic rather than aborting a
//! session; a test is the opposite case, so the lint is allowed in this file only.

#![allow(clippy::panic)]

use std::collections::BTreeSet;

use sc_core::ontology::{
    DeterministicIdGenerator, Direction, EntityId, IdentityLedger, OffsetInterval, OrphaningEdit,
    Param, Rational, ReleaseReadiness, Resolution, SplitSide, TopologyEdit,
};
use sc_core::ontology::{EdgeRef, LocalTag};
use sc_units::Length;

/// The seed every randomized case below is generated from. Recorded, per roadmap §6.3 — the same seed the
/// `sc-units` and identity-layer suites use, so the whole repository's property tests are reproducible from
/// one constant.
const SEED: u64 = 0x57_49_54_43_48_43_41_44; // "STITCHCAD"

const CASES: usize = 2_000;
const SCRIPTS: usize = 300;

/// A deterministic xorshift64* generator: no dependency, reproducible, good enough to explore the input
/// space. It is *not* used for anything that reaches production output.
struct Rng(u64);

impl Rng {
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// A value in `0..=bound`.
    fn up_to(&mut self, bound: u64) -> u64 {
        self.next_u64() % (bound + 1)
    }

    /// A rational in `[0, 1]`: a numerator no larger than its denominator.
    #[allow(clippy::expect_used)] // a helper, not a `#[test]` fn, so `.clippy.toml`'s allow-expect-in-tests does not reach it
    fn unit_rational(&mut self) -> Rational {
        let den = i64::try_from(self.up_to(999) + 1).unwrap_or(1);
        let num = i64::try_from(self.up_to(u64::try_from(den).unwrap_or(1))).unwrap_or(0);
        Rational::new(num, den).expect("a small non-zero denominator is always valid")
    }

    /// A rational strictly inside `(0, 1)` — a legal split point or interval bound.
    #[allow(clippy::expect_used)] // as above: a helper, not a `#[test]` fn
    fn open_rational(&mut self) -> Rational {
        let den = i64::try_from(self.up_to(998) + 2).unwrap_or(2);
        let ceiling = u64::try_from(den - 1).unwrap_or(1);
        let num = i64::try_from(self.up_to(ceiling.saturating_sub(1)) + 1).unwrap_or(1);
        Rational::new(num, den).expect("a small non-zero denominator is always valid")
    }

    /// A parameter from a rational in `[0, 1]`.
    #[allow(clippy::expect_used)] // as above: a helper, not a `#[test]` fn
    fn param(&mut self) -> Param {
        Param::new(self.unit_rational()).expect("a unit rational is a valid parameter")
    }

    /// A positive arc length, garment-scale in micrometres and strictly inside `sc-units`' declared
    /// domain — two of which still sum without leaving it.
    #[allow(clippy::expect_used)] // as above: a helper, not a `#[test]` fn
    fn length(&mut self) -> Length {
        let micrometres = i64::try_from(self.up_to(1 << 28) + 1).unwrap_or(1);
        Length::from_micrometres(micrometres).expect("inside the declared domain")
    }

    /// A fresh ledger whose first operation declared `count` edges.
    #[allow(clippy::expect_used)] // as above: a helper, not a `#[test]` fn
    fn ledger_with(
        &mut self,
        count: usize,
    ) -> (IdentityLedger, DeterministicIdGenerator, Vec<EdgeRef>) {
        let mut ledger = IdentityLedger::new();
        let mut ids = DeterministicIdGenerator::new();
        let edges = ledger
            .declare_edges(&mut ids, count)
            .expect("a positive count inside the domain always declares");
        (ledger, ids, edges)
    }
}

/// The unique resolution of a reference, or a panic naming what it was instead — the properties below
/// only ask for resolutions the contract guarantees are unique.
#[allow(clippy::expect_used)] // a helper, not a `#[test]` fn
fn must_resolve(
    ledger: &IdentityLedger,
    edge: EdgeRef,
    param: Param,
    context: &str,
) -> (EdgeRef, Rational, Direction) {
    match ledger.resolve(edge, param) {
        Resolution::Resolved(position) => (
            position.edge(),
            position.param().as_rational(),
            position.direction(),
        ),
        other => panic!("{context}: expected a unique resolution, got {other:?}"),
    }
}

/// Split resolves a reference into exactly the fragment that contains its parameter, recomputed exactly
/// (`t/s` below, `(t − s)/(1 − s)` above), and a reference AT the split point resolves to both fragments
/// — with the consumer, never the ledger, stating which it wants (ontology §1.1's first row).
#[test]
fn split_resolves_by_trichotomy_and_the_split_point_offers_both_sides() {
    let mut rng = Rng(SEED);
    for case in 0..CASES {
        let (mut ledger, mut ids, edges) = rng.ledger_with(1);
        let edge = *edges.first().expect("one edge declared");
        let at = Param::new(rng.open_rational()).expect("an open rational is a valid parameter");
        let fragments = ledger
            .split(&mut ids, edge, at)
            .expect("a live edge splits at an interior parameter");
        // Every fourth case targets the split point exactly; the rest land strictly inside a side.
        let t = if case % 4 == 0 { at } else { rng.param() };
        let s = at.as_rational();
        let t_value = t.as_rational();
        match t_value.cmp(&s) {
            core::cmp::Ordering::Less => {
                let (resolved, param, direction) =
                    must_resolve(&ledger, edge, t, "a reference below the split point");
                assert_eq!(resolved, fragments.first());
                assert_eq!(param, t_value.checked_div(s).expect("s > 0"));
                assert_eq!(direction, Direction::Original);
            }
            core::cmp::Ordering::Greater => {
                let (resolved, param, direction) =
                    must_resolve(&ledger, edge, t, "a reference above the split point");
                assert_eq!(resolved, fragments.second());
                let expected = t_value
                    .checked_sub(s)
                    .and_then(|shifted| s.one_minus().and_then(|rest| shifted.checked_div(rest)))
                    .expect("s < 1");
                assert_eq!(param, expected);
                assert_eq!(direction, Direction::Original);
            }
            core::cmp::Ordering::Equal => match ledger.resolve(edge, t) {
                Resolution::SplitPoint { first, second } => {
                    let chosen_first = first
                        .choose(SplitSide::First)
                        .resolved()
                        .expect("the first side is live");
                    assert_eq!(chosen_first.edge(), fragments.first());
                    assert_eq!(chosen_first.param(), Param::END);
                    let chosen_second = second
                        .choose(SplitSide::Second)
                        .resolved()
                        .expect("the second side is live");
                    assert_eq!(chosen_second.edge(), fragments.second());
                    assert_eq!(chosen_second.param(), Param::START);
                }
                other => panic!("the split point must offer both fragments, got {other:?}"),
            },
        }
    }
}

/// Merge recomputes a surviving parameter BY ARC LENGTH and exactly: the resolved position satisfies the
/// defining proportion — `p·(L₁+L₂) = t·L₁` from the first edge, `p·(L₁+L₂) = L₁ + t·L₂` from the
/// second — checked by cross-multiplication in `i128`, an oracle that shares no code with the fold
/// (§1.1's second row).
#[test]
fn merge_recomputes_parameters_by_arc_length_exactly() {
    let mut rng = Rng(SEED);
    for _ in 0..CASES {
        let (mut ledger, mut ids, edges) = rng.ledger_with(2);
        let (e0, e1) = (
            *edges.first().expect("two edges declared"),
            *edges.get(1).expect("two edges declared"),
        );
        let l1 = rng.length();
        let l2 = rng.length();
        let merged = ledger
            .merge(&mut ids, e0, l1, e1, l2)
            .expect("two distinct live edges with positive lengths merge");
        let t = rng.param();
        let a = i128::from(t.as_rational().numerator());
        let b = i128::from(t.as_rational().denominator());
        let l1v = i128::from(l1.as_micrometres());
        let l2v = i128::from(l2.as_micrometres());
        let total = l1v + l2v;

        let (on_merged, p, direction) =
            must_resolve(&ledger, e0, t, "a reference on the first side");
        assert_eq!(on_merged, merged);
        assert_eq!(direction, Direction::Original);
        // p == t·L₁/(L₁+L₂)  ⟺  p.num · b·(L₁+L₂) == p.den · a·L₁
        assert_eq!(
            i128::from(p.numerator()) * (b * total),
            i128::from(p.denominator()) * (a * l1v),
            "the first side satisfies the defining proportion for t={t}, L₁={l1v}, L₂={l2v}"
        );

        let (on_merged, q, _) = must_resolve(&ledger, e1, t, "a reference on the second side");
        assert_eq!(on_merged, merged);
        // q == (L₁ + t·L₂)/(L₁+L₂)  ⟺  q.num · b·(L₁+L₂) == q.den · (b·L₁ + a·L₂)
        assert_eq!(
            i128::from(q.numerator()) * (b * total),
            i128::from(q.denominator()) * (b * l1v + a * l2v),
            "the second side satisfies the defining proportion for t={t}, L₁={l1v}, L₂={l2v}"
        );
    }
}

/// Reverse maps every parameter to `1 − t`, keeps the identity, tells directed consumers, and is an
/// involution — reversing twice returns the identical rational, with no drift (§1.1's third row).
#[test]
fn reverse_maps_t_to_one_minus_t_and_is_an_involution() {
    let mut rng = Rng(SEED);
    for _ in 0..CASES {
        let (mut ledger, mut ids, edges) = rng.ledger_with(1);
        let edge = *edges.first().expect("one edge declared");
        let t = rng.param();
        ledger
            .reverse(&mut ids, edge)
            .expect("a live edge reverses");
        let (resolved, reversed, direction) = must_resolve(&ledger, edge, t, "after one reverse");
        assert_eq!(resolved, edge, "the identity survives a reverse");
        assert_eq!(
            direction,
            Direction::Reversed,
            "directed consumers are told"
        );
        // The oracle is the defining equation p + t == 1, not a recomputation of `one_minus`.
        assert_eq!(
            reversed.checked_add(t.as_rational()).expect("bounded"),
            Rational::ONE,
            "p + t == 1 exactly for t={t}"
        );
        ledger
            .reverse(&mut ids, edge)
            .expect("a live edge reverses again");
        let (resolved, twice, direction) = must_resolve(&ledger, edge, t, "after two reverses");
        assert_eq!(resolved, edge);
        assert_eq!(
            twice,
            t.as_rational(),
            "an involution: the identical rational"
        );
        assert_eq!(direction, Direction::Original);
    }
}

/// The round trip: merge with lengths `L₁, L₂` and then split at `L₁/(L₁+L₂)` — or split first and merge
/// the fragments with those lengths — returns every parameter to the IDENTICAL reduced rational. This is
/// reference stability in its strongest form: an exact type would drift under a fixed-point parameter,
/// and this property is where that drift would show (`decision_edge-parameter-bounded-exact-rational.md`).
#[test]
fn split_and_merge_round_trip_the_exact_parameter() {
    let mut rng = Rng(SEED);
    for case in 0..CASES {
        let l1 = rng.length();
        let l2 = rng.length();
        let l1v = l1.as_micrometres();
        let l2v = l2.as_micrometres();
        let at = Rational::new(l1v, l1v + l2v).expect("positive lengths");
        let at_param = Param::new(at).expect("a ratio of positive lengths is inside (0, 1)");
        let t = rng.param();

        if case % 2 == 0 {
            // Merge first, then split the merged edge at the length ratio, then merge the fragments back.
            let (mut ledger, mut ids, edges) = rng.ledger_with(2);
            let (e0, e1) = (
                *edges.first().expect("two edges declared"),
                *edges.get(1).expect("two edges declared"),
            );
            let merged = ledger.merge(&mut ids, e0, l1, e1, l2).expect("mergeable");
            let fragments = ledger
                .split(&mut ids, merged, at_param)
                .expect("the ratio of positive lengths is an interior split point");
            ledger
                .merge(&mut ids, fragments.first(), l1, fragments.second(), l2)
                .expect("the fragments merge back");
            match t.as_rational().cmp(&at) {
                core::cmp::Ordering::Equal => {
                    // The seam point round-trips from BOTH sides to the same exact position.
                    match ledger.resolve(merged, t) {
                        Resolution::SplitPoint { first, second } => {
                            for side in [first, second] {
                                let position =
                                    side.resolved().expect("both fragments survive the remerge");
                                assert_eq!(position.param().as_rational(), at);
                            }
                        }
                        other => panic!("the seam point must offer both sides, got {other:?}"),
                    }
                }
                _ => {
                    let (_, back, _) =
                        must_resolve(&ledger, merged, t, "after merge → split → merge");
                    assert_eq!(
                        back,
                        t.as_rational(),
                        "the identical reduced rational, zero drift"
                    );
                }
            }
        } else {
            // Split first, then merge the fragments with lengths proportional to the split.
            let (mut ledger, mut ids, edges) = rng.ledger_with(1);
            let edge = *edges.first().expect("one edge declared");
            let fragments = ledger
                .split(&mut ids, edge, at_param)
                .expect("an interior split point");
            let recomposed = ledger
                .merge(&mut ids, fragments.first(), l1, fragments.second(), l2)
                .expect("the fragments merge");
            match t.as_rational().cmp(&at) {
                core::cmp::Ordering::Equal => {
                    // The split point round-trips from BOTH sides to the seam position of the merge.
                    match ledger.resolve(edge, t) {
                        Resolution::SplitPoint { first, second } => {
                            for side in [first, second] {
                                let position = side.resolved().expect("the merged edge survives");
                                assert_eq!(position.edge(), recomposed);
                                assert_eq!(position.param().as_rational(), at);
                            }
                        }
                        other => panic!("the split point must offer both sides, got {other:?}"),
                    }
                }
                _ => {
                    let (_, back, _) = must_resolve(&ledger, edge, t, "after split → merge");
                    assert_eq!(
                        back,
                        t.as_rational(),
                        "the split-then-merge recomposition is the identity on the parameter"
                    );
                }
            }
        }
    }
}

/// Delete orphans EXACTLY the references that resolved onto the deleted edge — each into a visible repair
/// task naming the held reference, the delete operation and no candidates — and every other reference's
/// resolution is untouched, byte for byte (§1.1's fourth row: no silent reassignment, in either
/// direction).
#[test]
fn a_delete_orphans_exactly_the_references_resolving_onto_the_deleted_edge() {
    let mut rng = Rng(SEED);
    for _ in 0..SCRIPTS {
        let mut ledger = IdentityLedger::new();
        let mut ids = DeterministicIdGenerator::new();
        let script = Script::new(&mut rng);
        script.apply(&mut ledger, &mut ids);
        // Register references over whatever survived the script.
        let live: Vec<EdgeRef> = ledger.live_edges().collect();
        if live.is_empty() {
            continue;
        }
        let mut holders = Vec::new();
        for index in 0..live.len().min(6) {
            let owner = EntityId::from_bits(1_000 + u128::from(u64::try_from(index).unwrap_or(0)));
            let edge = *live.get(index).expect("within bounds");
            let param = rng.param();
            ledger
                .register(owner, edge, param)
                .expect("a live edge registers");
            holders.push((owner, edge, param));
        }
        // Pre-delete oracle: the contract's own answer for every registration, before the delete. A
        // registered LIVE edge always resolves uniquely — of the five edits only a reverse can mention a
        // live edge — so `before` is a resolved position and the oracle is its edge.
        let before: Vec<Resolution> = holders
            .iter()
            .map(|(_, edge, param)| ledger.resolve(*edge, *param))
            .collect();
        let victim_index =
            usize::try_from(rng.up_to(u64::try_from(live.len() - 1).unwrap_or(0))).unwrap_or(0);
        let victim = *live.get(victim_index).expect("within bounds");
        let deleting = ledger
            .delete(&mut ids, victim)
            .expect("a live edge deletes");

        let mut expected_repairs = 0;
        for ((owner, edge, param), before) in holders.iter().zip(before) {
            let after = ledger.resolve(*edge, *param);
            let was_on_victim = match &before {
                Resolution::Resolved(position) => position.edge() == victim,
                _ => false,
            };
            if was_on_victim {
                expected_repairs += 1;
                match &after {
                    Resolution::Unresolved(task) => {
                        assert_eq!(task.reference(), *edge, "the task names the held reference");
                        assert_eq!(task.param(), *param);
                        assert_eq!(
                            *task.orphaned_by(),
                            OrphaningEdit::Deleted {
                                operation: deleting,
                                edge: victim
                            },
                            "and the edit that orphaned it"
                        );
                        assert!(task.candidates().is_empty(), "a deletion suggests nothing");
                    }
                    other => {
                        panic!("a reference on the deleted edge must be unresolved, got {other:?}")
                    }
                }
            } else {
                assert_eq!(
                    before, after,
                    "a reference that was not on the deleted edge is untouched (owner {owner})"
                );
            }
        }
        // The open-repair set is exactly the registrations that became unresolved.
        assert_eq!(ledger.open_repairs().len(), expected_repairs);
        if expected_repairs == 0 {
            assert_eq!(ledger.release_readiness(), ReleaseReadiness::Releasable);
        } else {
            assert_eq!(
                ledger.release_readiness(),
                ReleaseReadiness::Blocked {
                    unresolved: expected_repairs
                }
            );
        }
    }
}

/// Offset maps a position exactly when one fragment's declared interval contains it, refuses a shared
/// boundary with both fragments as candidates, and refuses a trimmed-away position with none (§1.1's
/// fifth row). The oracle is an integer hundredths grid the test walks itself — a different derivation
/// than the fold's rational arithmetic.
#[test]
fn offset_maps_unambiguous_positions_and_refuses_ambiguous_ones() {
    let mut rng = Rng(SEED);
    for _ in 0..CASES {
        // Build 1..=3 ascending, non-overlapping intervals over hundredths, with random gaps and
        // sometimes a shared boundary.
        let mut cursor: i64 = 0;
        let mut grid: Vec<(i64, i64)> = Vec::new();
        let count = rng.up_to(2) + 1;
        for _ in 0..count {
            if cursor >= 99 {
                break;
            }
            let gap = i64::try_from(rng.up_to(if cursor == 0 { 0 } else { 20 })).unwrap_or(0);
            let from = cursor + gap;
            if from >= 99 {
                break;
            }
            let width =
                i64::try_from(rng.up_to(u64::try_from(99 - from).unwrap_or(1))).unwrap_or(1) + 1;
            let to = from + width;
            grid.push((from, to));
            cursor = to; // a zero gap makes the next interval share this boundary: an ambiguity case
        }
        if grid.is_empty() {
            continue;
        }
        #[allow(clippy::expect_used)]
        // the grid is built inside (0, 100) with from < to by construction
        let intervals: Vec<OffsetInterval> = grid
            .iter()
            .map(|(from, to)| {
                OffsetInterval::new(
                    Param::new(Rational::new(*from, 100).expect("hundredths"))
                        .expect("inside [0,1]"),
                    Param::new(Rational::new(*to, 100).expect("hundredths")).expect("inside [0,1]"),
                )
                .expect("from < to by construction")
            })
            .collect();
        let (mut ledger, mut ids, edges) = rng.ledger_with(1);
        let edge = *edges.first().expect("one edge declared");
        let fragments = ledger
            .offset(&mut ids, edge, &intervals)
            .expect("a non-empty ascending interval list offsets a live edge");
        let offset_op = fragments.first().expect("at least one fragment").creator();

        let hundredths = i64::try_from(rng.up_to(100)).unwrap_or(0);
        let t =
            Param::new(Rational::new(hundredths, 100).expect("hundredths")).expect("inside [0,1]");
        // The oracle: walk the grid, count containment, compute the expected position independently.
        let containing: Vec<(usize, i64, i64)> = grid
            .iter()
            .enumerate()
            .filter(|(_, (from, to))| *from <= hundredths && hundredths <= *to)
            .map(|(index, (from, to))| (index, *from, *to))
            .collect();
        match containing.len() {
            1 => {
                let (index, from, to) = *containing.first().expect("one container");
                let (resolved, param, direction) =
                    must_resolve(&ledger, edge, t, "an unambiguous offset position");
                assert_eq!(
                    resolved,
                    *fragments.get(index).expect("fragment for the interval")
                );
                let expected =
                    Rational::new(hundredths - from, to - from).expect("a non-empty interval");
                assert_eq!(
                    param, expected,
                    "(t − from)/(to − from) exactly, for t={hundredths}/100 in [{from},{to}]"
                );
                assert_eq!(direction, Direction::Original);
            }
            2 => match ledger.resolve(edge, t) {
                Resolution::Unresolved(task) => {
                    assert!(matches!(
                        task.orphaned_by(),
                        OrphaningEdit::OffsetAmbiguous { operation, source }
                            if *operation == offset_op && *source == edge
                    ));
                    assert_eq!(task.reference(), edge);
                    assert_eq!(task.param(), t);
                    assert_eq!(task.candidates().len(), 2, "both fragments, and only those");
                    for (candidate, (index, _, _)) in task.candidates().iter().zip(containing) {
                        assert_eq!(candidate.edge(), *fragments.get(index).expect("fragment"));
                    }
                }
                other => panic!("a shared boundary must be refused, got {other:?}"),
            },
            _ => match ledger.resolve(edge, t) {
                Resolution::Unresolved(task) => {
                    assert!(matches!(
                        task.orphaned_by(),
                        OrphaningEdit::OffsetUnmapped { operation, source }
                            if *operation == offset_op && *source == edge
                    ));
                    assert!(
                        task.candidates().is_empty(),
                        "a trimmed-away position has no candidate"
                    );
                }
                other => panic!("an uncovered position must be unresolved, got {other:?}"),
            },
        }
    }
}

/// The maintained live-edge index equals an independent replay of the journal — the ledger's cached half
/// never drifts from its source of truth, which is what makes resolution-by-fold trustworthy.
#[test]
fn the_live_set_equals_the_journal_replayed() {
    let mut rng = Rng(SEED);
    for _ in 0..SCRIPTS {
        let mut ledger = IdentityLedger::new();
        let mut ids = DeterministicIdGenerator::new();
        let script = Script::new(&mut rng);
        script.apply(&mut ledger, &mut ids);
        // The oracle: fold the journal from scratch with the five edits' liveness semantics, written
        // independently of the ledger's maintained set.
        let mut replayed: BTreeSet<EdgeRef> = BTreeSet::new();
        for entry in ledger.journal() {
            match entry.edit() {
                TopologyEdit::Declared { edges } => {
                    replayed.extend(edges.iter().copied());
                }
                TopologyEdit::Split {
                    source,
                    first,
                    second,
                    ..
                } => {
                    assert!(replayed.remove(source), "a split consumes a live source");
                    replayed.insert(*first);
                    replayed.insert(*second);
                }
                TopologyEdit::Merge {
                    first,
                    second,
                    merged,
                    ..
                } => {
                    assert!(replayed.remove(first), "a merge consumes live sources");
                    assert!(replayed.remove(second));
                    replayed.insert(*merged);
                }
                TopologyEdit::Reverse { .. } => {}
                TopologyEdit::Delete { edge } => {
                    assert!(replayed.remove(edge), "a delete consumes a live edge");
                }
                TopologyEdit::Offset { source, fragments } => {
                    assert!(replayed.remove(source), "an offset consumes a live source");
                    replayed.extend(fragments.iter().map(|fragment| fragment.edge()));
                }
            }
        }
        let maintained: BTreeSet<EdgeRef> = ledger.live_edges().collect();
        assert_eq!(maintained, replayed, "the index is the journal, replayed");
    }
}

/// The replay property, extended from ids to topology: the same script over two fresh deterministic
/// generators produces byte-identical journals, identical live sets and identical resolutions — so a
/// recorded session reproduces every fragment identity (`G1-SLICE.10`'s future input).
#[test]
fn replaying_the_same_script_produces_byte_identical_identities() {
    let mut rng = Rng(SEED);
    for _ in 0..SCRIPTS {
        let script = Script::new(&mut rng);
        let mut left = IdentityLedger::new();
        let mut left_ids = DeterministicIdGenerator::new();
        script.apply(&mut left, &mut left_ids);
        let mut right = IdentityLedger::new();
        let mut right_ids = DeterministicIdGenerator::new();
        script.apply(&mut right, &mut right_ids);
        assert_eq!(left.journal(), right.journal(), "byte-identical journals");
        assert_eq!(
            left.journal()
                .iter()
                .map(|entry| entry.operation().to_string())
                .collect::<Vec<String>>(),
            right
                .journal()
                .iter()
                .map(|entry| entry.operation().to_string())
                .collect::<Vec<String>>(),
            "the operation ids agree in their canonical Crockford form"
        );
        let left_live: BTreeSet<EdgeRef> = left.live_edges().collect();
        let right_live: BTreeSet<EdgeRef> = right.live_edges().collect();
        assert_eq!(left_live, right_live);
        // And resolutions of the same probes agree — including the repair tasks, if the script made any.
        for probe in &script.probes {
            assert_eq!(
                left.resolve(probe.edge, probe.param),
                right.resolve(probe.edge, probe.param),
                "the same reference resolves identically in both replays"
            );
        }
    }
}

/// A probe reference the replay property resolves on both ledgers.
#[derive(Clone, Copy, Debug)]
struct Probe {
    edge: EdgeRef,
    param: Param,
}

/// A random but replayable edit script: the operations and their parameters are drawn once, so applying
/// the same `Script` to two ledgers is the replay property's input. Operations always pick from the live
/// set at apply time, in a deterministic order, so the same script is valid on both replays.
#[derive(Clone, Debug)]
struct Script {
    steps: Vec<Step>,
    probes: Vec<Probe>,
}

#[derive(Clone, Copy, Debug)]
enum Step {
    Declare(usize),
    Split(Param),
    Merge(Length, Length),
    Reverse,
    Delete,
    Offset(u64), // a seed for the interval grid, consumed at apply time
}

impl Script {
    #[allow(clippy::expect_used)] // a helper, not a `#[test]` fn, so `.clippy.toml`'s allow-expect-in-tests does not reach it
    fn new(rng: &mut Rng) -> Self {
        let mut steps = vec![Step::Declare(
            usize::try_from(rng.up_to(2) + 1).unwrap_or(1),
        )];
        for _ in 0..rng.up_to(7) {
            steps.push(match rng.up_to(9) {
                0..=2 => Step::Split(
                    Param::new(rng.open_rational()).expect("an open rational is a valid parameter"),
                ),
                3 | 4 => Step::Merge(rng.length(), rng.length()),
                5 => Step::Reverse,
                6 => Step::Delete,
                7 => Step::Declare(usize::try_from(rng.up_to(1) + 1).unwrap_or(1)),
                _ => Step::Offset(rng.next_u64()),
            });
        }
        // Probes reference the FIRST declared operation's edges — identities the deterministic
        // generator reproduces — at random parameters, so both replays resolve the same references.
        let first_operation = EntityId::from_bits(1);
        let probes = (0..3)
            .map(|_| Probe {
                edge: EdgeRef::new(first_operation, LocalTag::FIRST),
                param: rng.param(),
            })
            .collect();
        Self { steps, probes }
    }

    #[allow(clippy::expect_used)] // every step picks from the live set, so each apply is valid by construction
    fn apply(&self, ledger: &mut IdentityLedger, ids: &mut DeterministicIdGenerator) {
        for step in &self.steps {
            let live: Vec<EdgeRef> = ledger.live_edges().collect();
            match step {
                Step::Declare(count) => {
                    ledger
                        .declare_edges(ids, *count)
                        .expect("a positive count inside the domain declares");
                }
                Step::Split(at) => {
                    if let Some(edge) = live.first() {
                        ledger
                            .split(ids, *edge, *at)
                            .expect("a live edge splits at an interior point");
                    }
                }
                Step::Merge(l1, l2) => {
                    if live.len() >= 2 {
                        let (first, second) = (
                            *live.first().expect("two live edges"),
                            *live.get(1).expect("two live edges"),
                        );
                        ledger
                            .merge(ids, first, *l1, second, *l2)
                            .expect("two distinct live edges with positive lengths merge");
                    }
                }
                Step::Reverse => {
                    if let Some(edge) = live.first() {
                        ledger.reverse(ids, *edge).expect("a live edge reverses");
                    }
                }
                Step::Delete => {
                    if let Some(edge) = live.last() {
                        ledger.delete(ids, *edge).expect("a live edge deletes");
                    }
                }
                Step::Offset(grid_seed) => {
                    if let Some(edge) = live.first() {
                        let intervals = offset_grid(*grid_seed);
                        if let Some(intervals) = intervals {
                            ledger
                                .offset(ids, *edge, &intervals)
                                .expect("a valid grid offsets a live edge");
                        }
                    }
                }
            }
        }
    }
}

/// A deterministic interval grid over hundredths from a seed, or `None` when the seed degenerates.
/// Shared boundaries occur by construction, so the ambiguity path is exercised by the replay and
/// live-set scripts, not only by the offset property.
#[allow(clippy::expect_used)] // hundredths with from < to are valid parameters by construction
fn offset_grid(seed: u64) -> Option<Vec<OffsetInterval>> {
    let mut rng = Rng(seed | 1);
    let mut cursor: i64 = 0;
    let mut intervals = Vec::new();
    for _ in 0..rng.up_to(2) + 1 {
        if cursor >= 99 {
            break;
        }
        let from = cursor;
        let width =
            i64::try_from(rng.up_to(u64::try_from(99 - from).unwrap_or(1))).unwrap_or(1) + 1;
        let to = from + width;
        intervals.push(
            OffsetInterval::new(
                Param::new(Rational::new(from, 100).expect("hundredths")).expect("inside [0,1]"),
                Param::new(Rational::new(to, 100).expect("hundredths")).expect("inside [0,1]"),
            )
            .expect("from < to by construction"),
        );
        cursor = to; // adjacent intervals: every shared boundary is an ambiguity case
    }
    if intervals.is_empty() {
        None
    } else {
        Some(intervals)
    }
}
