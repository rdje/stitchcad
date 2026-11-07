//! A piece's edges by name, and the references to them that survive an edit.
//!
//! Every edge of a drafted piece is declared in the core's identity ledger, and an edit changes the ledger and the drawing together. A notch, a grade
//! point or a seam's end holds an edge and a fraction of it, and where that lands now is the ledger's answer and nobody else's. So a reference is never
//! rewritten and never reassigned: it lands on the same cloth, or it becomes a repair task that says which edit orphaned it.
use super::{
    corner_named, length, ratio, required, whole, Fields, Lengths, NOTCH_REACH, TOLERANCE,
};
use sc_core::ontology::{
    DeterministicIdGenerator, Direction, EdgeRef, EntityId, IdGenerator, IdentityLedger, LocalTag,
    OffsetInterval, Param, Rational, Resolution, ResolvedRef, SplitSide,
};
use sc_geometry::{Allowance, Curve, Point};
use sc_store::Value;
use sc_units::{pure, Length};
use std::collections::{BTreeMap, BTreeSet};

// How many places along an edge are tried when asking how much of it the cut line still follows, besides every place a notch holds.
const SAMPLES: i64 = 64;

#[derive(Clone, Debug)]
struct Run {
    edge: EdgeRef,
    name: Option<String>,
    // Whether the edge's own frame runs against the boundary, after a reverse.
    flipped: bool,
    live: bool,
    // Its length when it stopped being live, which is the frame its references were written against.
    last_length: i64,
}

/// A position a reference holds: a named edge and a fraction of it, in the frame the edge was born in.
#[derive(Clone, Debug)]
pub(super) struct Held {
    pub(super) edge: String,
    pub(super) param: Param,
    pub(super) choose: Option<SplitSide>,
}

impl Held {
    pub(super) fn words(&self) -> String {
        format!("{} at {}", self.edge, self.param.as_rational())
    }
}

/// Where a reference lands on the net line.
#[derive(Clone, Debug)]
pub(super) struct Located {
    pub(super) point: Point,
    // Out of the piece, square to the edge here.
    pub(super) outward: (f64, f64),
    pub(super) width: i64,
    pub(super) direction: Direction,
    pub(super) now: String,
}

/// The answer to where a reference is now.
#[derive(Clone, Debug)]
pub(super) enum Answer {
    Here(Located),
    Repair {
        orphaned_by: String,
        candidates: Vec<String>,
    },
}

enum Settled {
    Here(ResolvedRef),
    Repair {
        orphaned_by: String,
        candidates: Vec<String>,
    },
}

/// A piece's boundary as it is being described: its curves, each one's allowance, and which edge each one is part of.
#[derive(Clone, Debug)]
pub(super) struct Edges {
    ledger: IdentityLedger,
    ids: DeterministicIdGenerator,
    runs: Vec<Run>,
    pub(super) curves: Vec<Curve>,
    pub(super) allowances: Vec<Allowance>,
    // A spread's opening belongs to no edge.
    pub(super) owners: Vec<Option<usize>>,
    // What each operation in the journal was, in words.
    edits: Vec<(EntityId, String)>,
}

fn chord(a: Point, b: Point) -> f64 {
    pure::hypot(ratio(b.x() - a.x()), ratio(b.y() - a.y()))
}

fn value_of(rational: Rational) -> f64 {
    ratio(rational.numerator()) / ratio(rational.denominator())
}

fn curve_measure(curve: Curve) -> f64 {
    curve
        .flatten(TOLERANCE)
        .windows(2)
        .map(|pair| match pair {
            [a, b] => chord(*a, *b),
            _ => 0.0,
        })
        .sum()
}

// A fraction of an edge, as `1/3`, `0` or `1`.
fn fraction(text: &str, what: &str) -> Result<Param, String> {
    let wrong = || {
        format!("{what} says {text}, and a place on an edge is a fraction from 0 to 1, as in 1/3")
    };
    let (top, bottom) = text.split_once('/').unwrap_or((text, "1"));
    let top: i64 = top.trim().parse().map_err(|_| wrong())?;
    let bottom: i64 = bottom.trim().parse().map_err(|_| wrong())?;
    let rational = Rational::new(top, bottom).map_err(|_| wrong())?;
    Param::new(rational).map_err(|_| wrong())
}

fn text<'a>(fields: &'a Fields, name: &str, what: &str) -> Result<&'a str, String> {
    fields
        .get(name)
        .and_then(Value::as_text)
        .ok_or_else(|| format!("{what} names its {name}"))
}

fn distance_to_line(line: &[Point], at: (f64, f64)) -> f64 {
    let mut least = f64::MAX;
    for (index, a) in line.iter().enumerate() {
        let Some(b) = line.get((index + 1) % line.len()) else {
            continue;
        };
        let (dx, dy) = (ratio(b.x() - a.x()), ratio(b.y() - a.y()));
        let reach = dx * dx + dy * dy;
        let along = if reach > 0.0 {
            (((at.0 - ratio(a.x())) * dx + (at.1 - ratio(a.y())) * dy) / reach).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let away = pure::hypot(
            at.0 - (ratio(a.x()) + along * dx),
            at.1 - (ratio(a.y()) + along * dy),
        );
        if away < least {
            least = away;
        }
    }
    least
}

impl Edges {
    /// A boundary from a piece's description, every edge declared in the ledger.
    pub(super) fn describe(lengths: &Lengths, fields: &Fields) -> Result<Self, String> {
        let start = super::point(lengths, required(fields, "start")?)?;
        let listed = required(fields, "edges")?
            .as_list()
            .ok_or("edges must be a list")?;
        let mut curves = Vec::with_capacity(listed.len());
        let mut allowances = Vec::with_capacity(listed.len());
        let mut names = Vec::with_capacity(listed.len());
        let mut from = start;
        for edge in listed {
            let edge = edge.as_object().ok_or("an edge must be an object")?;
            let to = super::point(lengths, required(edge, "to")?)?;
            let curve = match (edge.get("first"), edge.get("second")) {
                (Some(first), Some(second)) => Curve::Cubic {
                    from,
                    first: super::point(lengths, first)?,
                    second: super::point(lengths, second)?,
                    to,
                },
                (None, None) => Curve::Line { from, to },
                _ => return Err("a curved edge names both control points".to_owned()),
            };
            curves.push(curve);
            allowances.push(Self::allowance_of(lengths, edge)?);
            names.push(edge.get("name").and_then(Value::as_text).map(str::to_owned));
            from = to;
        }
        let mut ledger = IdentityLedger::new();
        let mut ids = DeterministicIdGenerator::new();
        let mut runs = Vec::with_capacity(names.len());
        let mut seen = BTreeSet::new();
        if !names.is_empty() {
            let tags: Vec<LocalTag> = (0..names.len())
                .map(|slot| u32::try_from(slot).map(LocalTag::new))
                .collect::<Result<_, _>>()
                .map_err(|_| "a piece has too many edges".to_owned())?;
            let operation = ids.next_id().map_err(|error| error.to_string())?;
            let edges = ledger
                .declare_tagged_edges(operation, &tags)
                .map_err(|error| error.to_string())?;
            for (edge, name) in edges.into_iter().zip(names) {
                if let Some(name) = &name {
                    if !seen.insert(name.clone()) {
                        return Err(format!("two edges are named {name}"));
                    }
                }
                runs.push(Run {
                    edge,
                    name,
                    flipped: false,
                    live: true,
                    last_length: 0,
                });
            }
        }
        Ok(Self {
            ledger,
            ids,
            owners: (0..runs.len()).map(Some).collect(),
            runs,
            curves,
            allowances,
            edits: Vec::new(),
        })
    }

    fn allowance_of(lengths: &Lengths, edge: &Fields) -> Result<Allowance, String> {
        let width = length(
            lengths,
            required(edge, "width")?
                .as_text()
                .ok_or("an edge's width is named as text")?,
        )?;
        let corner = corner_named(
            required(edge, "corner")?
                .as_text()
                .ok_or("an edge's corner is named as text")?,
        )?;
        Ok(Allowance { width, corner })
    }

    /// Every edit the piece lists, in order, each changing the ledger and the drawing together.
    pub(super) fn edit_all(
        &mut self,
        lengths: &Lengths,
        fields: &Fields,
        piece: &str,
    ) -> Result<(), String> {
        for (index, item) in fields
            .get("edits")
            .and_then(Value::as_list)
            .unwrap_or(&[])
            .iter()
            .enumerate()
        {
            let item = item.as_object().ok_or("an edit must be an object")?;
            let op = text(item, "op", "an edit")?;
            let words = format!(
                "edit {} on {piece}, {op} {}",
                index + 1,
                item.get("edge")
                    .or_else(|| item.get("first"))
                    .and_then(Value::as_text)
                    .unwrap_or("")
            );
            match op {
                "split" => self.split(item, words)?,
                "merge" => self.merge(item, words)?,
                "reverse" => self.reverse(item, words)?,
                "delete" => self.delete(item, words)?,
                "allowance" => self.set_allowance(lengths, item)?,
                other => {
                    return Err(format!(
                        "no edit is named {other}: an edge can be split, merged, reversed, deleted, or given an allowance"
                    ))
                }
            }
        }
        Ok(())
    }

    fn named(&self, name: &str) -> Result<usize, String> {
        self.runs
            .iter()
            .position(|run| run.name.as_deref() == Some(name))
            .ok_or_else(|| format!("no edge is named {name}"))
    }

    fn live(&self, name: &str) -> Result<usize, String> {
        let run = self.named(name)?;
        if self.runs.get(run).is_some_and(|run| run.live) {
            Ok(run)
        } else {
            Err(format!("the edge {name} has already been edited away"))
        }
    }

    fn fresh(&self, name: &str) -> Result<(), String> {
        if self.named(name).is_ok() {
            Err(format!("an edge is already named {name}"))
        } else {
            Ok(())
        }
    }

    fn note(&mut self, words: String) {
        if let Some(entry) = self.ledger.journal().last() {
            self.edits.push((entry.operation(), words));
        }
    }

    fn add_run(&mut self, edge: EdgeRef, name: &str, flipped: bool) -> usize {
        self.runs.push(Run {
            edge,
            name: Some(name.to_owned()),
            flipped,
            live: true,
            last_length: 0,
        });
        self.runs.len() - 1
    }

    fn retire(&mut self, run: usize) {
        let measured = whole(self.measure(run));
        if let Some(run) = self.runs.get_mut(run) {
            run.live = false;
            run.last_length = measured;
        }
    }

    fn owned(&self, run: usize) -> impl Iterator<Item = usize> + '_ {
        self.owners
            .iter()
            .enumerate()
            .filter(move |(_, owner)| **owner == Some(run))
            .map(|(index, _)| index)
    }

    // One edge's line in boundary order: its chords, each with the allowance of the curve it came from.
    fn chords(&self, run: usize) -> Vec<(Point, Point, i64)> {
        let mut out = Vec::new();
        for index in self.owned(run) {
            let Some(curve) = self.curves.get(index) else {
                continue;
            };
            let width = self
                .allowances
                .get(index)
                .map_or(0, |allowance| allowance.width);
            for pair in curve.flatten(TOLERANCE).windows(2) {
                if let [a, b] = pair {
                    out.push((*a, *b, width));
                }
            }
        }
        out
    }

    fn measure(&self, run: usize) -> f64 {
        self.chords(run).iter().map(|(a, b, _)| chord(*a, *b)).sum()
    }

    // The length an `along` is a fraction of: as drawn while the edge is live, and as it last was once it is not.
    fn frame_length(&self, run: usize) -> i64 {
        match self.runs.get(run) {
            Some(found) if !found.live => found.last_length,
            _ => whole(self.measure(run)),
        }
    }

    // The place a fraction along a live edge lands, in that edge's own frame.
    fn place(&self, run: usize, at: Rational) -> Result<(Point, (f64, f64), i64), String> {
        let flipped = self.runs.get(run).is_some_and(|run| run.flipped);
        let share = if flipped {
            1.0 - value_of(at)
        } else {
            value_of(at)
        };
        let chords = self.chords(run);
        let target = share * chords.iter().map(|(a, b, _)| chord(*a, *b)).sum::<f64>();
        let mut walked = 0.0;
        for (index, (a, b, width)) in chords.iter().enumerate() {
            let here = chord(*a, *b);
            if here <= 0.0 {
                continue;
            }
            if walked + here >= target || index + 1 == chords.len() {
                let along = ((target - walked) / here).clamp(0.0, 1.0);
                let (dx, dy) = (ratio(b.x() - a.x()), ratio(b.y() - a.y()));
                let point = Point::new(a.x() + whole(dx * along), a.y() + whole(dy * along))
                    .map_err(|error| error.to_string())?;
                return Ok((point, (dy / here, -dx / here), *width));
            }
            walked += here;
        }
        Err("an edge with no line".to_owned())
    }

    fn name_of(&self, edge: EdgeRef) -> Option<String> {
        self.runs
            .iter()
            .find(|run| run.edge == edge)
            .and_then(|run| run.name.clone())
    }

    fn describe_place(&self, place: ResolvedRef) -> String {
        format!(
            "{} at {}",
            self.name_of(place.edge())
                .unwrap_or_else(|| "a part of this edge".to_owned()),
            place.param().as_rational()
        )
    }

    fn split(&mut self, item: &Fields, words: String) -> Result<(), String> {
        let name = text(item, "edge", "a split")?;
        let run = self.live(name)?;
        let what = format!("the split of {name}");
        let at = fraction(text(item, "at", "a split")?, &what)?;
        let names = item
            .get("into")
            .and_then(Value::as_list)
            .ok_or_else(|| format!("{what} names the two parts, as into"))?;
        let [first_name, second_name] = names else {
            return Err(format!("{what} names the two parts, as into"));
        };
        let (first_name, second_name) = (
            first_name.as_text().ok_or("a part's name is text")?,
            second_name.as_text().ok_or("a part's name is text")?,
        );
        self.fresh(first_name)?;
        self.fresh(second_name)?;
        if first_name == second_name {
            return Err(format!("{what} gives both parts the name {first_name}"));
        }
        let found = self.runs.get(run).ok_or("no such edge")?;
        let (edge, flipped) = (found.edge, found.flipped);
        let total = self.measure(run);
        let share = if flipped {
            1.0 - value_of(at.as_rational())
        } else {
            value_of(at.as_rational())
        };
        let cut = share * total;
        let fragments = self
            .ledger
            .split(&mut self.ids, edge, at)
            .map_err(|error| error.to_string())?;
        self.note(words);
        self.retire(run);
        let first = self.add_run(fragments.first(), first_name, flipped);
        let second = self.add_run(fragments.second(), second_name, flipped);
        let (ahead, behind) = if flipped {
            (second, first)
        } else {
            (first, second)
        };
        let mut walked = 0.0;
        let mut index = 0;
        while index < self.curves.len() {
            if self.owners.get(index).copied().flatten() != Some(run) {
                index += 1;
                continue;
            }
            let curve = *self.curves.get(index).ok_or("no such curve")?;
            let here = curve_measure(curve);
            let owner = if walked + here <= cut + 0.5 {
                Some(ahead)
            } else if walked >= cut - 0.5 {
                Some(behind)
            } else {
                None
            };
            if let Some(owner) = owner {
                if let Some(slot) = self.owners.get_mut(index) {
                    *slot = Some(owner);
                }
                index += 1;
            } else {
                let (near, far) = curve
                    .split_at_length(whole(cut - walked), TOLERANCE)
                    .ok_or_else(|| format!("{what} lands too near an end to make two parts"))?;
                let allowance = *self.allowances.get(index).ok_or("no such allowance")?;
                self.curves.splice(index..=index, [near, far]);
                self.allowances.insert(index, allowance);
                self.owners
                    .splice(index..=index, [Some(ahead), Some(behind)]);
                index += 2;
            }
            walked += here;
        }
        if self.owned(ahead).next().is_none() || self.owned(behind).next().is_none() {
            return Err(format!("{what} lands too near an end to make two parts"));
        }
        Ok(())
    }

    fn merge(&mut self, item: &Fields, words: String) -> Result<(), String> {
        let first_name = text(item, "first", "a merge")?;
        let second_name = text(item, "second", "a merge")?;
        let into = text(item, "into", "a merge")?;
        let (first, second) = (self.live(first_name)?, self.live(second_name)?);
        if first == second {
            return Err(format!(
                "a merge needs two edges, and both are {first_name}"
            ));
        }
        self.fresh(into)?;
        let run_of = |run: usize| self.runs.get(run).ok_or("no such edge");
        let (first_edge, flipped) = (run_of(first)?.edge, run_of(first)?.flipped);
        let (second_edge, second_flipped) = (run_of(second)?.edge, run_of(second)?.flipped);
        if flipped != second_flipped {
            return Err(format!(
                "{first_name} and {second_name} run opposite ways, so one is reversed before they merge"
            ));
        }
        // In the merged edge's direction first comes before second, and on a reversed edge that direction runs against the boundary.
        let (lead, follow) = if flipped {
            (second, first)
        } else {
            (first, second)
        };
        let count = self.curves.len();
        let lead_first = self.owned(lead).next().ok_or("an edge with no line")?;
        let lead_last = self.owned(lead).last().ok_or("an edge with no line")?;
        let follow_first = self.owned(follow).next().ok_or("an edge with no line")?;
        if (lead_last + 1) % count != follow_first {
            return Err(format!(
                "{first_name} does not end where {second_name} starts, so they cannot merge"
            ));
        }
        // An edge is a contiguous stretch of the list, so a merge across the list's end turns the list first.
        if follow_first < lead_first {
            self.curves.rotate_left(lead_first);
            self.allowances.rotate_left(lead_first);
            self.owners.rotate_left(lead_first);
        }
        let evidence =
            |value: i64| Length::from_micrometres(value).map_err(|error| error.to_string());
        let first_length = evidence(whole(self.measure(first)))?;
        let second_length = evidence(whole(self.measure(second)))?;
        let merged = self
            .ledger
            .merge(
                &mut self.ids,
                first_edge,
                first_length,
                second_edge,
                second_length,
            )
            .map_err(|error| error.to_string())?;
        self.note(words);
        self.retire(first);
        self.retire(second);
        let joined = self.add_run(merged, into, flipped);
        for owner in &mut self.owners {
            if *owner == Some(first) || *owner == Some(second) {
                *owner = Some(joined);
            }
        }
        Ok(())
    }

    fn reverse(&mut self, item: &Fields, words: String) -> Result<(), String> {
        let name = text(item, "edge", "a reverse")?;
        let run = self.live(name)?;
        let edge = self.runs.get(run).ok_or("no such edge")?.edge;
        self.ledger
            .reverse(&mut self.ids, edge)
            .map_err(|error| error.to_string())?;
        self.note(words);
        if let Some(found) = self.runs.get_mut(run) {
            found.flipped = !found.flipped;
        }
        Ok(())
    }

    // A boundary stays closed, so what replaces a deleted edge is a straight line with a new identity. Nothing that held the old edge moves onto it.
    fn delete(&mut self, item: &Fields, words: String) -> Result<(), String> {
        let name = text(item, "edge", "a delete")?;
        let bridge = text(item, "bridge", "a delete")?;
        let run = self.live(name)?;
        self.fresh(bridge)?;
        let first = self.owned(run).next().ok_or("an edge with no line")?;
        let last = self.owned(run).last().ok_or("an edge with no line")?;
        let from = self.curves.get(first).ok_or("no such curve")?.from();
        let to = self.curves.get(last).ok_or("no such curve")?.to();
        if from == to {
            return Err(format!(
                "{name} is the whole boundary, and cannot be deleted"
            ));
        }
        let allowance = *self.allowances.get(first).ok_or("no such allowance")?;
        let edge = self.runs.get(run).ok_or("no such edge")?.edge;
        self.ledger
            .delete(&mut self.ids, edge)
            .map_err(|error| error.to_string())?;
        self.note(words);
        self.retire(run);
        let operation = self.ids.next_id().map_err(|error| error.to_string())?;
        let edges = self
            .ledger
            .declare_tagged_edges(operation, &[LocalTag::FIRST])
            .map_err(|error| error.to_string())?;
        let new = edges.first().copied().ok_or("no edge was declared")?;
        let added = self.add_run(new, bridge, false);
        self.curves.splice(first..=last, [Curve::Line { from, to }]);
        self.allowances.splice(first..=last, [allowance]);
        self.owners.splice(first..=last, [Some(added)]);
        Ok(())
    }

    fn set_allowance(&mut self, lengths: &Lengths, item: &Fields) -> Result<(), String> {
        let name = text(item, "edge", "an allowance")?;
        let run = self.live(name)?;
        let allowance = Self::allowance_of(lengths, item)?;
        let owned: Vec<usize> = self.owned(run).collect();
        for index in owned {
            if let Some(slot) = self.allowances.get_mut(index) {
                *slot = allowance;
            }
        }
        Ok(())
    }

    /// A position a carrier states, read and checked. The edge may be one an edit has since replaced, because that is the point of a reference.
    pub(super) fn held(
        &self,
        lengths: &Lengths,
        fields: &Fields,
        what: &str,
    ) -> Result<Held, String> {
        let edge = text(fields, "edge", what)?;
        let run = self
            .named(edge)
            .map_err(|error| format!("{what}: {error}"))?;
        let param = match (fields.get("at"), fields.get("along")) {
            (Some(at), None) => fraction(
                at.as_text().ok_or_else(|| format!("{what}: at is text"))?,
                what,
            )?,
            (None, Some(along)) => {
                let names: Vec<&str> = match along {
                    Value::Text(name) => vec![name.as_str()],
                    Value::List(names) => names.iter().filter_map(Value::as_text).collect(),
                    _ => return Err(format!("{what}: along names lengths")),
                };
                let mut distance = 0;
                for name in names {
                    distance += length(lengths, name)?;
                }
                let whole_length = self.frame_length(run);
                if distance < 0 || distance > whole_length {
                    return Err(format!(
                        "{what} is {distance} along {edge}, which is {whole_length} long"
                    ));
                }
                let share =
                    Rational::new(distance, whole_length).map_err(|error| error.to_string())?;
                Param::new(share).map_err(|error| error.to_string())?
            }
            _ => {
                return Err(format!(
                    "{what} says where along {edge} with at or along, and only one"
                ))
            }
        };
        let choose = match fields.get("choose").and_then(Value::as_text) {
            None => None,
            Some("first") => Some(SplitSide::First),
            Some("second") => Some(SplitSide::Second),
            Some(other) => {
                return Err(format!(
                    "{what} chooses {other}, and a split has a first and a second"
                ))
            }
        };
        Ok(Held {
            edge: edge.to_owned(),
            param,
            choose,
        })
    }

    fn candidates_of(&self, answer: &Resolution) -> Vec<String> {
        match answer {
            Resolution::Resolved(place) => vec![self.describe_place(*place)],
            Resolution::SplitPoint { first, second } => {
                let mut listed = self.candidates_of(first);
                listed.extend(self.candidates_of(second));
                listed
            }
            Resolution::Unresolved(task) => task
                .candidates()
                .iter()
                .map(|place| self.describe_place(*place))
                .collect(),
        }
    }

    // Both sides of a split that has since been merged away are the same place, and there is nothing left to choose between.
    fn settle(&self, answer: Resolution) -> Settled {
        match answer {
            Resolution::Resolved(place) => Settled::Here(place),
            Resolution::SplitPoint { first, second } => {
                let candidates = {
                    let mut listed = self.candidates_of(&first);
                    listed.extend(self.candidates_of(&second));
                    listed
                };
                match (self.settle(*first), self.settle(*second)) {
                    (Settled::Here(a), Settled::Here(b)) if a == b => Settled::Here(a),
                    _ => Settled::Repair {
                        orphaned_by:
                            "a split at exactly this place, so it has to choose first or second"
                                .to_owned(),
                        candidates,
                    },
                }
            }
            Resolution::Unresolved(task) => {
                let operation = task.orphaned_by().operation();
                Settled::Repair {
                    orphaned_by: self
                        .edits
                        .iter()
                        .find(|(listed, _)| *listed == operation)
                        .map_or_else(
                            || task.orphaned_by().to_string(),
                            |(_, words)| words.clone(),
                        ),
                    candidates: task
                        .candidates()
                        .iter()
                        .map(|place| self.describe_place(*place))
                        .collect(),
                }
            }
        }
    }

    fn settled(&self, held: &Held) -> Result<Settled, String> {
        let run = self.named(&held.edge)?;
        let edge = self.runs.get(run).ok_or("no such edge")?.edge;
        let mut answer = self.ledger.resolve(edge, held.param);
        if let Some(side) = held.choose {
            answer = answer.choose(side);
        }
        Ok(self.settle(answer))
    }

    fn locate(&self, place: ResolvedRef) -> Result<Located, String> {
        let run = self
            .runs
            .iter()
            .position(|run| run.edge == place.edge())
            .ok_or("a reference landed on an edge this piece does not draw")?;
        let (point, outward, width) = self.place(run, place.param().as_rational())?;
        Ok(Located {
            point,
            outward,
            width,
            direction: place.direction(),
            now: self.describe_place(place),
        })
    }

    /// Where a reference lands on the net line now.
    pub(super) fn answer(&self, held: &Held) -> Result<Answer, String> {
        Ok(match self.settled(held)? {
            Settled::Here(place) => Answer::Here(self.locate(place)?),
            Settled::Repair {
                orphaned_by,
                candidates,
            } => Answer::Repair {
                orphaned_by,
                candidates,
            },
        })
    }

    /// The same, for a mark on the cut line: a place whose allowance has been trimmed away has no cut line to carry it, which is the contract's fifth row.
    pub(super) fn answer_on_cut(&self, trimmed: &Self, held: &Held) -> Result<Answer, String> {
        if let Settled::Repair {
            orphaned_by,
            candidates,
        } = trimmed.settled(held)?
        {
            return Ok(Answer::Repair {
                orphaned_by,
                candidates,
            });
        }
        self.answer(held)
    }

    fn leaves(answer: &Resolution, out: &mut Vec<ResolvedRef>) {
        match answer {
            Resolution::Resolved(place) => out.push(*place),
            Resolution::SplitPoint { first, second } => {
                Self::leaves(first, out);
                Self::leaves(second, out);
            }
            Resolution::Unresolved(_) => {}
        }
    }

    // Whether the cut line still follows this place on a live edge.
    fn reaches(&self, run: usize, at: Param, cut: &[Point]) -> Result<bool, String> {
        let (point, outward, width) = self.place(run, at.as_rational())?;
        let target = (
            ratio(point.x()) + outward.0 * ratio(width),
            ratio(point.y()) + outward.1 * ratio(width),
        );
        Ok(distance_to_line(cut, target) <= ratio(NOTCH_REACH))
    }

    /// This ledger as the cut line sees it. Each edge a notch holds is fragmented where the offset no longer follows it, using the offset's own result.
    pub(super) fn trimmed(&self, cut: &[Point], held: &[Held]) -> Result<Self, String> {
        let mut wanted: BTreeMap<usize, BTreeSet<Param>> = BTreeMap::new();
        for item in held {
            let run = self.named(&item.edge)?;
            let edge = self.runs.get(run).ok_or("no such edge")?.edge;
            let mut places = Vec::new();
            Self::leaves(&self.ledger.resolve(edge, item.param), &mut places);
            for place in places {
                if let Some(run) = self.runs.iter().position(|run| run.edge == place.edge()) {
                    wanted.entry(run).or_default().insert(place.param());
                }
            }
        }
        let mut frame = self.clone();
        for (run, places) in wanted {
            let mut samples = places;
            for step in 0..=SAMPLES {
                let share = Rational::new(step, SAMPLES).map_err(|error| error.to_string())?;
                samples.insert(Param::new(share).map_err(|error| error.to_string())?);
            }
            let mut kept: Vec<(Param, Param)> = Vec::new();
            let mut open: Option<(Param, Param)> = None;
            for at in samples {
                if self.reaches(run, at, cut)? {
                    open = Some(open.map_or((at, at), |(from, _)| (from, at)));
                } else if let Some(span) = open.take() {
                    kept.push(span);
                }
            }
            kept.extend(open);
            kept.retain(|(from, to)| from < to);
            if kept == [(Param::START, Param::END)] {
                continue;
            }
            let found = self.runs.get(run).ok_or("no such edge")?;
            let words = format!(
                "the allowance, which trims part of {} away",
                found.name.as_deref().unwrap_or("this edge")
            );
            if kept.is_empty() {
                frame
                    .ledger
                    .delete(&mut frame.ids, found.edge)
                    .map_err(|error| error.to_string())?;
            } else {
                let intervals = kept
                    .iter()
                    .map(|(from, to)| OffsetInterval::new(*from, *to))
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|error| error.to_string())?;
                frame
                    .ledger
                    .offset(&mut frame.ids, found.edge, &intervals)
                    .map_err(|error| error.to_string())?;
            }
            frame.note(words);
        }
        Ok(frame)
    }
}
