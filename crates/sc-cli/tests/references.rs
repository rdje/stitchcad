//! References under edit: every edit the contract names, against every carrier it names. A carrier stays on the same cloth, or it becomes a repair task that says why, and nothing moves anywhere else.
#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
use sc_artifacts::from_canonical_text;
use sc_cli::{replay, Caller, CommandAuthority, Running};
use sc_store::{parse, FilePath, MemoryRepository, Repository, Value};
use std::collections::BTreeMap;

const SKIRT: &str = include_str!("../../../conformance/reference-skirt/session.json");
// T2 twice over: a part of a curve and the whole curve are each within T2 of it, so two flattenings of one place can differ by this much.
const SAME_PLACE: i64 = 20;
const TRIALS: u64 = 48;

// A small random sequence with a stated seed, so a failure names the case that made it. The same seed gives the same case on every machine.
struct Dice(u64);

impl Dice {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (self.0 >> 33) ^ self.0
    }

    fn below(&mut self, bound: u64) -> u64 {
        self.next() % bound
    }

    // A fraction strictly inside an edge, not near an end, in lowest-terms-free form.
    fn fraction(&mut self) -> String {
        format!("{}/1000", 50 + self.below(900))
    }
}

fn opening(width: i64) -> String {
    format!(
        r#"{{"format":1,"design":"01ARZ3NDEKTSV4RRFFQ69G5FAV",
        "inputs":[{{"token":"wide","um":{width}}},{{"token":"tall","um":600000}},{{"token":"sa","um":10000}},{{"token":"wider","um":200000}},{{"token":"over","um":60000}}],
        "recipe":"let depth: length = 5 mm\nlet control_a: length = wide - 100 mm\nlet control_b: length = 100 mm\nlet rise: length = tall + 50 mm\nlet half: length = tall / 2\n",
        "steps":[]}}"#
    )
}

fn edge(name: &str, to: (&str, &str)) -> String {
    format!(
        r#"{{"name":"{name}","to":["{}","{}"],"width":"sa","corner":"miter"}}"#,
        to.0, to.1
    )
}

// A panel with three straight edges and one cubic, and a carrier of each kind on every edge.
fn panel(name: &str, places: &[String; 4]) -> String {
    let notches: Vec<String> = ["bottom", "right", "top", "left"]
        .iter()
        .zip(places)
        .map(|(edge, at)| {
            format!(r#"{{"name":"n_{edge}","edge":"{edge}","at":"{at}","depth":"depth"}}"#)
        })
        .collect();
    let grade: Vec<String> = ["bottom", "right", "top", "left"]
        .iter()
        .zip(places)
        .map(|(edge, at)| format!(r#"{{"name":"g_{edge}","edge":"{edge}","at":"{at}"}}"#))
        .collect();
    format!(
        r#"{{"name":"{name}","quantity":1,"start":["zero","zero"],
        "edges":[{},{},
          {{"name":"top","to":["zero","tall"],"first":["control_a","rise"],"second":["control_b","rise"],"width":"sa","corner":"miter"}},
          {}],
        "grain":[[["over","over"],["over","half"]]],
        "notches":[{}],"grade_points":[{}]}}"#,
        edge("bottom", ("wide", "zero")),
        edge("right", ("wide", "tall")),
        edge("left", ("zero", "zero")),
        notches.join(","),
        grade.join(",")
    )
}

fn seams() -> String {
    let side = |piece: &str, edge: &str| {
        format!(
            r#"{{"piece":"{piece}","from":{{"edge":"{edge}","at":"0"}},"to":{{"edge":"{edge}","at":"1"}}}}"#
        )
    };
    ["bottom", "right", "top", "left"]
        .iter()
        .map(|edge| {
            format!(
                r#"{{"name":"s_{edge}","a":{},"b":{}}}"#,
                side("panel", edge),
                side("twin", edge)
            )
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn draft_step(places: &[String; 4]) -> String {
    format!(
        r#"{{"do":"draft","style":"two","size":"M","pieces":[{},{}],"seams":[{}]}}"#,
        panel("panel", places),
        panel("twin", places),
        seams()
    )
}

fn step(running: &mut Running<&mut MemoryRepository>, text: &str) -> Result<Value, String> {
    running
        .step(&parse(text).expect("a step"))
        .map_err(|failure| {
            failure.to_value().as_object().expect("a diagnostic")["message"]
                .as_text()
                .expect("text")
                .to_owned()
        })
}

fn result(record: &Value) -> Value {
    record.as_object().expect("a record")["result"].clone()
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Landing {
    now: String,
    direction: Option<String>,
    point: Option<(i64, i64)>,
}

#[derive(Clone, Debug)]
struct Seen {
    places: BTreeMap<(String, String), Landing>,
    repairs: Vec<BTreeMap<String, Value>>,
}

fn seen(running: &mut Running<&mut MemoryRepository>) -> Seen {
    let record = result(&step(running, r#"{"do":"references"}"#).expect("lists"));
    let fields = record.as_object().expect("a report");
    let mut places = BTreeMap::new();
    for item in fields["references"].as_list().expect("a list") {
        let item = item.as_object().expect("a reference");
        let point = item.get("point").map(|point| {
            let point = point.as_list().expect("a point");
            (point[0].as_int().expect("x"), point[1].as_int().expect("y"))
        });
        places.insert(
            (
                item["piece"].as_text().expect("text").to_owned(),
                item["carrier"].as_text().expect("text").to_owned(),
            ),
            Landing {
                now: item["now"].as_text().expect("text").to_owned(),
                direction: item
                    .get("direction")
                    .and_then(Value::as_text)
                    .map(str::to_owned),
                point,
            },
        );
    }
    let repairs = fields["repairs"]
        .as_list()
        .expect("a list")
        .iter()
        .map(|item| item.as_object().expect("a repair").clone())
        .collect();
    Seen { places, repairs }
}

fn away(a: (i64, i64), b: (i64, i64)) -> i64 {
    let (dx, dy) = ((a.0 - b.0) as f64, (a.1 - b.1) as f64);
    (dx * dx + dy * dy).sqrt().ceil() as i64
}

// The contract, whole: every carrier that was there before is still answered for. It is on the same cloth, or it is listed as a repair with the edit that did it. Only carriers the
// caller expected may be orphaned, because orphaning something else is a reassignment too, and no answer may be a different place.
fn holds(before: &Seen, after: &Seen, orphaned: &[(&str, &str)], case: &str) {
    assert_eq!(
        before.places.len(),
        after.places.len(),
        "{case}: a carrier vanished"
    );
    for (key, was) in &before.places {
        let now = after
            .places
            .get(key)
            .unwrap_or_else(|| panic!("{case}: {key:?} is gone"));
        let expected = orphaned
            .iter()
            .any(|(piece, carrier)| key.0 == *piece && key.1.starts_with(carrier));
        if expected {
            assert_eq!(now.now, "unresolved", "{case}: {key:?} should be orphaned");
            assert!(now.point.is_none(), "{case}: {key:?} still has a place");
            assert!(
                after.repairs.iter().any(|repair| {
                    repair["piece"].as_text() == Some(&key.0)
                        && repair["carrier"].as_text() == Some(&key.1)
                }),
                "{case}: {key:?} has no repair task"
            );
        } else {
            let (was, is) = (
                was.point
                    .unwrap_or_else(|| panic!("{case}: {key:?} began unresolved")),
                now.point
                    .unwrap_or_else(|| panic!("{case}: {key:?} was orphaned: {:?}", after.repairs)),
            );
            assert!(
                away(was, is) <= SAME_PLACE,
                "{case}: {key:?} moved from {was:?} to {is:?}"
            );
        }
    }
    assert_eq!(
        after.repairs.len(),
        after
            .places
            .values()
            .filter(|landing| landing.now == "unresolved")
            .count(),
        "{case}: every task is one unresolved carrier, and the other way round"
    );
}

fn start<'a>(
    places: &[String; 4],
    disk: &'a mut MemoryRepository,
) -> Running<&'a mut MemoryRepository> {
    let session = parse(&opening(400_000)).expect("valid");
    let mut running = Running::open(&session, disk).expect("opens");
    step(&mut running, &draft_step(places)).expect("draws");
    running
}

fn places(dice: &mut Dice) -> [String; 4] {
    [
        dice.fraction(),
        dice.fraction(),
        dice.fraction(),
        dice.fraction(),
    ]
}

const EDGES: [&str; 4] = ["bottom", "right", "top", "left"];

#[test]
fn a_split_keeps_every_carrier_on_the_same_cloth() {
    let mut dice = Dice(1);
    for trial in 0..TRIALS {
        let held = places(&mut dice);
        let target = EDGES[dice.below(4) as usize];
        let at = dice.fraction();
        let mut disk = MemoryRepository::new();
        let mut running = start(&held, &mut disk);
        let before = seen(&mut running);
        let report = result(
            &step(
                &mut running,
                &format!(r#"{{"do":"edit_edge","piece":"panel","op":"split","edge":"{target}","at":"{at}","into":["p","q"]}}"#),
            )
            .expect("splits"),
        );
        let after = seen(&mut running);
        let case = format!("trial {trial}: split {target} at {at}, holding {held:?}");
        // A carrier exactly at the split is the contract's own case, and has its own test.
        let exact = held[EDGES
            .iter()
            .position(|edge| *edge == target)
            .expect("an edge")]
            == at;
        if !exact {
            holds(&before, &after, &[], &case);
            assert!(
                report.as_object().expect("a report")["repairs"]
                    .as_list()
                    .expect("a list")
                    .is_empty(),
                "{case}"
            );
        }
        step(&mut running, r#"{"do":"draft"}"#).expect("draws again");
    }
}

#[test]
fn a_carrier_exactly_at_a_split_is_asked_which_side_it_wants_and_is_never_guessed() {
    let held = [
        "1/2".to_owned(),
        "1/4".to_owned(),
        "1/3".to_owned(),
        "3/4".to_owned(),
    ];
    let mut disk = MemoryRepository::new();
    let mut running = start(&held, &mut disk);
    let before = seen(&mut running);
    step(
        &mut running,
        r#"{"do":"edit_edge","piece":"panel","op":"split","edge":"bottom","at":"1/2","into":["p","q"]}"#,
    )
    .expect("splits");
    let after = seen(&mut running);
    holds(
        &before,
        &after,
        &[
            ("panel", "notch n_bottom"),
            ("panel", "grade point g_bottom"),
        ],
        "at the split",
    );
    let task = after
        .repairs
        .iter()
        .find(|repair| repair["carrier"].as_text() == Some("notch n_bottom"))
        .expect("a task");
    assert_eq!(
        task["candidates"].as_list().expect("candidates"),
        &[
            Value::Text("p at 1".to_owned()),
            Value::Text("q at 0".to_owned())
        ]
    );
    assert!(task["orphaned_by"]
        .as_text()
        .expect("text")
        .contains("choose"));
    step(
        &mut running,
        r#"{"do":"repair_reference","piece":"panel","notch":"n_bottom","choose":"second"}"#,
    )
    .expect("chooses");
    step(
        &mut running,
        r#"{"do":"repair_reference","piece":"panel","grade_point":"g_bottom","choose":"first"}"#,
    )
    .expect("chooses");
    let chosen = seen(&mut running);
    holds(&before, &chosen, &[], "after choosing");
    assert_eq!(
        chosen.places[&("panel".to_owned(), "notch n_bottom".to_owned())].now,
        "q at 0"
    );
    assert_eq!(
        chosen.places[&("panel".to_owned(), "grade point g_bottom".to_owned())].now,
        "p at 1"
    );
}

#[test]
fn a_merge_keeps_every_carrier_and_recomputes_its_fraction_by_length() {
    let mut dice = Dice(2);
    for trial in 0..TRIALS {
        let held = places(&mut dice);
        let target = EDGES[dice.below(4) as usize];
        let at = dice.fraction();
        let mut disk = MemoryRepository::new();
        let mut running = start(&held, &mut disk);
        let before = seen(&mut running);
        step(
            &mut running,
            &format!(r#"{{"do":"edit_edge","piece":"panel","op":"split","edge":"{target}","at":"{at}","into":["p","q"]}}"#),
        )
        .expect("splits");
        step(
            &mut running,
            r#"{"do":"edit_edge","piece":"panel","op":"merge","first":"p","second":"q","into":"whole"}"#,
        )
        .expect("merges");
        let after = seen(&mut running);
        // Merged back, a carrier that sat exactly at the split has one place again, so there is nothing to ask.
        holds(
            &before,
            &after,
            &[],
            &format!("trial {trial}: {target} at {at}"),
        );
        let name = format!("notch n_{target}");
        assert!(
            after.places[&("panel".to_owned(), name)]
                .now
                .starts_with("whole at "),
            "trial {trial}"
        );
    }
}

#[test]
fn a_merge_of_neighbours_across_the_list_end_works_and_of_strangers_is_refused() {
    let held = [
        "1/2".to_owned(),
        "1/4".to_owned(),
        "1/3".to_owned(),
        "3/4".to_owned(),
    ];
    let mut disk = MemoryRepository::new();
    let mut running = start(&held, &mut disk);
    let before = seen(&mut running);
    let refused = step(
        &mut running,
        r#"{"do":"edit_edge","piece":"panel","op":"merge","first":"bottom","second":"top","into":"m"}"#,
    )
    .expect_err("not neighbours");
    assert!(refused.contains("does not end where"), "{refused}");
    // `left` ends where `bottom` starts, and `left` is last in the list.
    step(
        &mut running,
        r#"{"do":"edit_edge","piece":"panel","op":"merge","first":"left","second":"bottom","into":"l"}"#,
    )
    .expect("merges across the end");
    holds(&before, &seen(&mut running), &[], "across the end");
}

#[test]
fn a_reverse_keeps_every_carrier_and_tells_a_directed_consumer() {
    let mut dice = Dice(3);
    for trial in 0..TRIALS {
        let held = places(&mut dice);
        let target = EDGES[dice.below(4) as usize];
        let mut disk = MemoryRepository::new();
        let mut running = start(&held, &mut disk);
        let before = seen(&mut running);
        let reverse =
            format!(r#"{{"do":"edit_edge","piece":"panel","op":"reverse","edge":"{target}"}}"#);
        step(&mut running, &reverse).expect("reverses");
        let after = seen(&mut running);
        let case = format!("trial {trial}: reverse {target}");
        holds(&before, &after, &[], &case);
        let key = ("panel".to_owned(), format!("notch n_{target}"));
        assert_eq!(
            after.places[&key].direction.as_deref(),
            Some("reversed"),
            "{case}"
        );
        let fraction = held[EDGES
            .iter()
            .position(|edge| *edge == target)
            .expect("an edge")]
        .split_once('/')
        .map(|(top, _)| top.parse::<i64>().expect("a number"))
        .expect("a fraction");
        // Parameters become 1 − t, exactly, in lowest terms or not.
        let now = &after.places[&key].now;
        let (top, bottom) = now
            .rsplit_once(' ')
            .and_then(|(_, at)| at.split_once('/'))
            .map(|(top, bottom)| {
                (
                    top.parse::<i64>().expect("a number"),
                    bottom.parse::<i64>().expect("a number"),
                )
            })
            .expect("a fraction");
        assert_eq!(top * 1000, (1000 - fraction) * bottom, "{case}: {now}");
        step(&mut running, &reverse).expect("reverses back");
        let twice = seen(&mut running);
        holds(&before, &twice, &[], &case);
        assert_eq!(
            twice.places[&key].direction.as_deref(),
            Some("original"),
            "{case}"
        );
    }
}

#[test]
fn a_delete_orphans_exactly_what_it_held_and_moves_nothing_else() {
    let mut dice = Dice(4);
    for trial in 0..TRIALS {
        let held = places(&mut dice);
        let target = EDGES[dice.below(4) as usize];
        let mut disk = MemoryRepository::new();
        let mut running = start(&held, &mut disk);
        let before = seen(&mut running);
        let report = result(
            &step(
                &mut running,
                &format!(r#"{{"do":"edit_edge","piece":"panel","op":"delete","edge":"{target}","bridge":"stand_in"}}"#),
            )
            .expect("deletes"),
        );
        let case = format!("trial {trial}: delete {target}");
        let notch = format!("notch n_{target}");
        let grade = format!("grade point g_{target}");
        let seam = format!("seam s_{target}");
        let orphaned = [
            ("panel", notch.as_str()),
            ("panel", grade.as_str()),
            ("panel", seam.as_str()),
        ];
        let after = seen(&mut running);
        holds(&before, &after, &orphaned, &case);
        // A notch, a grade point, and both ends of one seam side.
        assert_eq!(after.repairs.len(), 4, "{case}: {:?}", after.repairs);
        assert_eq!(
            report.as_object().expect("a report")["repairs"]
                .as_list()
                .expect("a list")
                .len(),
            4,
            "{case}"
        );
        for repair in &after.repairs {
            assert!(
                repair["candidates"].as_list().expect("a list").is_empty(),
                "{case}: a deletion has nothing honest to suggest"
            );
            assert!(
                repair["orphaned_by"]
                    .as_text()
                    .expect("text")
                    .contains(&format!("delete {target}")),
                "{case}: {repair:?}"
            );
        }
        // The stand-in is a new identity: nothing landed on it.
        assert!(
            !after
                .places
                .values()
                .any(|landing| landing.now.starts_with("stand_in")),
            "{case}: a silent reassignment"
        );
    }
}

#[test]
fn a_reference_a_delete_orphaned_can_be_saved_and_looked_at_and_cannot_be_released_until_it_is_repaired(
) {
    let held = [
        "1/2".to_owned(),
        "1/4".to_owned(),
        "1/3".to_owned(),
        "3/4".to_owned(),
    ];
    let mut disk = MemoryRepository::new();
    let mut running = start(&held, &mut disk);
    let before = seen(&mut running);
    step(
        &mut running,
        r#"{"do":"edit_edge","piece":"panel","op":"delete","edge":"right","bridge":"stand_in"}"#,
    )
    .expect("deletes");
    step(&mut running, r#"{"do":"draft"}"#).expect("still draws");
    let pattern = running.pattern().expect("a drawing").clone();
    let panel = pattern
        .pieces
        .iter()
        .find(|piece| piece.name == "panel")
        .expect("the panel");
    assert_eq!(
        panel.marks.len(),
        3,
        "a notch with nowhere to go is not drawn somewhere else"
    );
    assert_eq!(panel.grade_points.len(), 3);
    assert!(
        !pattern.seams.iter().any(|seam| seam.name == "s_right"),
        "a seam with no end is out of the graph"
    );
    assert_eq!(pattern.repairs.len(), 4);
    let refused = step(
        &mut running,
        r#"{"do":"export_pattern","format":"dxf","semantics":"cut_as_1","path":"p.dxf"}"#,
    )
    .expect_err("not released");
    assert!(
        refused.contains("need repair before it can be released"),
        "{refused}"
    );
    assert!(refused.contains("notch n_right"), "{refused}");
    let refused = step(
        &mut running,
        r#"{"do":"export_pattern","format":"pdf","layout":"a0","path":"p.pdf"}"#,
    )
    .expect_err("not released");
    assert!(refused.contains("need repair"), "{refused}");
    // The canonical form is how it is inspected, and it says what is open.
    let canonical = sc_artifacts::to_canonical_text(&pattern);
    assert_eq!(
        from_canonical_text(&canonical).expect("reads").repairs,
        pattern.repairs
    );
    step(&mut running, r#"{"do":"save"}"#).expect("saves");

    // Repaired by someone: the notch to the stand-in, the grade point retired, the seam's two ends to the stand-in.
    step(
        &mut running,
        r#"{"do":"repair_reference","piece":"panel","notch":"n_right","edge":"stand_in","at":"1/2"}"#,
    )
    .expect("repoints");
    step(
        &mut running,
        r#"{"do":"repair_reference","piece":"panel","grade_point":"g_right","retire":true}"#,
    )
    .expect("retires");
    step(
        &mut running,
        r#"{"do":"repair_reference","seam":"s_right","side":"a","end":"from","edge":"stand_in","at":"0"}"#,
    )
    .expect("repoints");
    step(
        &mut running,
        r#"{"do":"repair_reference","seam":"s_right","side":"a","end":"to","edge":"stand_in","at":"1"}"#,
    )
    .expect("repoints");
    let repaired = seen(&mut running);
    assert!(repaired.repairs.is_empty(), "{:?}", repaired.repairs);
    assert_eq!(repaired.places.len(), before.places.len() - 1);
    step(&mut running, r#"{"do":"draft"}"#).expect("draws");
    step(
        &mut running,
        r#"{"do":"export_pattern","format":"dxf","semantics":"cut_as_1","path":"p.dxf"}"#,
    )
    .expect("released");
}

#[test]
fn an_edit_and_its_repairs_undo() {
    let held = [
        "1/2".to_owned(),
        "1/4".to_owned(),
        "1/3".to_owned(),
        "3/4".to_owned(),
    ];
    let mut disk = MemoryRepository::new();
    let mut running = start(&held, &mut disk);
    let before = seen(&mut running);
    let revision = running.revision();
    step(
        &mut running,
        r#"{"do":"edit_edge","piece":"panel","op":"delete","edge":"right","bridge":"stand_in"}"#,
    )
    .expect("deletes");
    assert_ne!(running.revision(), revision, "an edit is a new revision");
    step(&mut running, r#"{"do":"undo"}"#).expect("undoes");
    assert_eq!(running.revision(), revision);
    holds(&before, &seen(&mut running), &[], "undone");
    step(&mut running, r#"{"do":"redo"}"#).expect("redoes");
    assert_eq!(seen(&mut running).repairs.len(), 4);
}

#[test]
fn an_edit_that_makes_no_sense_changes_nothing_and_says_why() {
    let held = [
        "1/2".to_owned(),
        "1/4".to_owned(),
        "1/3".to_owned(),
        "3/4".to_owned(),
    ];
    let mut disk = MemoryRepository::new();
    let mut running = start(&held, &mut disk);
    let before = seen(&mut running);
    let revision = running.revision();
    for (edit, says) in [
        (
            r#""op":"split","edge":"nowhere","at":"1/2","into":["p","q"]"#,
            "no edge is named nowhere",
        ),
        (
            r#""op":"split","edge":"right","at":"3/2","into":["p","q"]"#,
            "a fraction from 0 to 1",
        ),
        (
            r#""op":"split","edge":"right","at":"0","into":["p","q"]"#,
            "zero-length fragment",
        ),
        (
            r#""op":"split","edge":"right","at":"1/2","into":["top","q"]"#,
            "already named top",
        ),
        (
            r#""op":"split","edge":"right","at":"1/2","into":["p","p"]"#,
            "both parts",
        ),
        (
            r#""op":"reverse","edge":"nowhere""#,
            "no edge is named nowhere",
        ),
        (r#""op":"delete","edge":"right""#, "names its bridge"),
        (r#""op":"fold","edge":"right""#, "no edit is named fold"),
    ] {
        let text = format!(r#"{{"do":"edit_edge","piece":"panel",{edit}}}"#);
        let refused = step(&mut running, &text).expect_err(edit);
        assert!(refused.contains(says), "{edit}: {refused}");
        assert_eq!(running.revision(), revision, "{edit}");
    }
    holds(&before, &seen(&mut running), &[], "after refusals");

    // An edge edited away cannot be edited again, and a carrier can still hold it, since that is what a reference is.
    step(
        &mut running,
        r#"{"do":"edit_edge","piece":"panel","op":"split","edge":"right","at":"1/5","into":["p","q"]}"#,
    )
    .expect("splits");
    let refused = step(
        &mut running,
        r#"{"do":"edit_edge","piece":"panel","op":"reverse","edge":"right"}"#,
    )
    .expect_err("edited away");
    assert!(refused.contains("already been edited away"), "{refused}");
    let refused = step(
        &mut running,
        r#"{"do":"edit_edge","piece":"twin_of_nothing","op":"reverse","edge":"right"}"#,
    )
    .expect_err("no piece");
    assert!(refused.contains("no piece is named"), "{refused}");
}

#[test]
fn a_changed_measurement_keeps_the_edge_and_the_fraction_and_moves_the_cloth() {
    let mut dice = Dice(5);
    for trial in 0..TRIALS {
        let held = places(&mut dice);
        let wider = 300_000 + 5000 * dice.below(100) as i64;
        let mut disk = MemoryRepository::new();
        let mut running = start(&held, &mut disk);
        let before = seen(&mut running);
        step(
            &mut running,
            &format!(r#"{{"do":"change","token":"wide","um":{wider}}}"#),
        )
        .expect("changes");
        let after = seen(&mut running);
        assert!(after.repairs.is_empty(), "trial {trial}");
        for (key, was) in &before.places {
            let now = &after.places[key];
            assert_eq!(
                was.now, now.now,
                "trial {trial}: {key:?} changed what it lands on"
            );
        }
        // `bottom` runs along x, so the fraction can be read straight off the new width.
        let key = ("panel".to_owned(), "notch n_bottom".to_owned());
        let mark = after.places[&key].point.expect("a place");
        let share = held[0]
            .split_once('/')
            .map(|(top, _)| top.parse::<f64>().expect("a number"))
            .expect("a fraction")
            / 1000.0;
        assert!(
            (mark.0 as f64 - share * wider as f64).abs() <= 1.0,
            "trial {trial}: {mark:?} at {share} of {wider}"
        );
    }
}

// An L, whose inside corner stops the wide allowance on `ledge` from reaching its far end.
fn ell(along_ledge: &str, width: &str) -> String {
    format!(
        r#"{{"do":"draft","style":"ell","size":"M","pieces":[{{"name":"ell","quantity":1,"start":["zero","zero"],
        "edges":[{{"name":"floor","to":["wider","zero"],"width":"sa","corner":"miter"}},
                 {{"name":"wall","to":["wider","over"],"width":"sa","corner":"miter"}},
                 {{"name":"ledge","to":["over","over"],"width":"{width}","corner":"miter"}},
                 {{"name":"post","to":["over","wider"],"width":"{width}","corner":"miter"}},
                 {{"name":"cap","to":["zero","wider"],"width":"sa","corner":"miter"}},
                 {{"name":"back","to":["zero","zero"],"width":"sa","corner":"miter"}}],
        "grain":[[["sa","sa"],["sa","over"]]],
        "notches":[{{"name":"near_end","edge":"ledge","at":"1/10","depth":"depth"}},{{"name":"far_end","edge":"ledge","at":"{along_ledge}","depth":"depth"}}],
        "grade_points":[{{"name":"on_ledge","edge":"ledge","at":"{along_ledge}"}}]}}]}}"#
    )
}

#[test]
fn a_place_the_allowance_trims_away_is_a_repair_task_and_the_rest_of_the_edge_is_not() {
    let session = parse(&opening(400_000)).expect("valid");
    let mut disk = MemoryRepository::new();
    let mut running = Running::open(&session, &mut disk).expect("opens");
    // `ledge` is 14 cm long and its allowance here is 6 cm: `post`'s allowance overlaps its last 6 cm, so 9/10 of the way along is under it and 1/10 is not.
    step(&mut running, &ell("9/10", "over")).expect("draws");
    let found = seen(&mut running);
    let ell = "ell".to_owned();
    assert_eq!(
        found.places[&(ell.clone(), "notch far_end".to_owned())].now,
        "unresolved"
    );
    assert!(found.places[&(ell.clone(), "notch near_end".to_owned())]
        .point
        .is_some());
    assert!(
        found.places[&(ell.clone(), "grade point on_ledge".to_owned())]
            .point
            .is_some(),
        "a grade point is on the sew line, which an allowance does not touch"
    );
    assert_eq!(found.repairs.len(), 1, "{:?}", found.repairs);
    assert!(
        found.repairs[0]["orphaned_by"]
            .as_text()
            .expect("text")
            .contains("the allowance"),
        "{:?}",
        found.repairs[0]
    );
    let piece = running.pattern().expect("drawn").pieces[0].clone();
    assert_eq!(
        piece.marks.len(),
        1,
        "it is not moved to where the cut line happens to be"
    );

    // Narrow the allowance by an edit, and the same reference has a place again: nothing was rewritten.
    step(
        &mut running,
        r#"{"do":"edit_edge","piece":"ell","op":"allowance","edge":"ledge","width":"sa","corner":"miter"}"#,
    )
    .expect("edits");
    step(
        &mut running,
        r#"{"do":"edit_edge","piece":"ell","op":"allowance","edge":"post","width":"sa","corner":"miter"}"#,
    )
    .expect("edits");
    let found = seen(&mut running);
    assert!(found.repairs.is_empty(), "{:?}", found.repairs);
    assert!(found.places[&(ell, "notch far_end".to_owned())]
        .point
        .is_some());
}

#[test]
fn a_mirror_holds_the_same_edges_and_lands_on_the_mirrored_cloth() {
    let mut dice = Dice(6);
    for trial in 0..TRIALS {
        let held = places(&mut dice);
        let target = EDGES[dice.below(4) as usize];
        let session = parse(&opening(400_000)).expect("valid");
        let mut disk = MemoryRepository::new();
        let mut running = Running::open(&session, &mut disk).expect("opens");
        let draft = format!(
            r#"{{"do":"draft","style":"two","size":"M","pieces":[{},{{"name":"copy","quantity":1,"mirror_of":"panel"}}],
            "seams":[{{"name":"m","a":{{"piece":"panel","from":{{"edge":"{target}","at":"0"}},"to":{{"edge":"{target}","at":"{}"}}}},
                               "b":{{"piece":"copy","from":{{"edge":"{target}","at":"0"}},"to":{{"edge":"{target}","at":"{}"}}}}}}]}}"#,
            panel("panel", &held),
            held[0],
            held[0]
        );
        step(&mut running, &draft).expect("draws");
        let before = seen(&mut running);
        let case = format!("trial {trial}: {target}");
        for end in ["from", "to"] {
            let on = |piece: &str| {
                (
                    piece.to_owned(),
                    format!(
                        "seam m, side {}, {end}",
                        if piece == "panel" { "a" } else { "b" }
                    ),
                )
            };
            let source = before.places[&on("panel")].point.expect("a place");
            let copy = &before.places[&on("copy")];
            assert_eq!(copy.point, Some((-source.0, source.1)), "{case}");
            assert_eq!(
                copy.direction.as_deref(),
                Some("reversed"),
                "{case}: a mirror runs the other way, and a directed consumer is told"
            );
        }
        // Each side measures the same, so the mirror walks to a match.
        step(&mut running, r#"{"do":"draft"}"#).expect("draws");
        let walked = result(&step(&mut running, r#"{"do":"walk"}"#).expect("walks"));
        let report = walked.as_list().expect("reports")[0]
            .as_object()
            .expect("a report")
            .clone();
        assert_eq!(
            report["finding"].as_text(),
            Some("matches"),
            "{case}: {report:?}"
        );
        // An edit to the source reaches the copy, on the same cloth.
        step(
            &mut running,
            &format!(r#"{{"do":"edit_edge","piece":"panel","op":"split","edge":"{target}","at":"{}","into":["p","q"]}}"#, dice.fraction()),
        )
        .expect("splits");
        let refused = step(
            &mut running,
            r#"{"do":"edit_edge","piece":"copy","op":"reverse","edge":"bottom"}"#,
        )
        .expect_err("a mirror has no edits of its own");
        assert!(refused.contains("is a mirror"), "{refused}");
        let after = seen(&mut running);
        for end in ["from", "to"] {
            let key = ("copy".to_owned(), format!("seam m, side b, {end}"));
            let was = before.places[&key].point.expect("a place");
            let now = after.places[&key].point.expect("a place");
            assert!(away(was, now) <= SAME_PLACE, "{case}: {was:?} to {now:?}");
        }
    }
}

#[test]
fn a_slash_and_spread_carries_a_reference_with_the_cloth_it_turned() {
    let session = parse(&opening(400_000)).expect("valid");
    let mut disk = MemoryRepository::new();
    let mut running = Running::open(&session, &mut disk).expect("opens");
    let draft = |spread: &str| {
        format!(
            r#"{{"do":"draft","style":"slash","size":"M","pieces":[{{"name":"p","quantity":1,"start":["zero","zero"],
            "edges":[{{"name":"bottom","to":["wide","zero"],"width":"sa","corner":"miter"}},
                     {{"name":"right","to":["wide","tall"],"width":"sa","corner":"miter"}},
                     {{"name":"top","to":["zero","tall"],"width":"sa","corner":"miter"}},
                     {{"name":"left","to":["zero","zero"],"width":"sa","corner":"miter"}}],
            "grain":[[["over","over"],["over","half"]]],
            {spread}
            "notches":[{{"name":"on_bottom","edge":"bottom","at":"1/2","depth":"depth"}},{{"name":"on_top","edge":"top","at":"1/2","depth":"depth"}}]}}]}}"#
        )
    };
    step(&mut running, &draft("")).expect("draws");
    let flat = seen(&mut running);
    let mut disk = MemoryRepository::new();
    let mut running = Running::open(&session, &mut disk).expect("opens");
    step(
        &mut running,
        &draft(r#""slash_spreads":[{"name":"s","pivot":["wide","half"],"slash_to":["zero","half"],"spread":"over"}],"#),
    )
    .expect("draws");
    let spread = seen(&mut running);
    assert!(spread.repairs.is_empty(), "{:?}", spread.repairs);
    // The chain from the slash's end to the pivot turns `left`'s lower part, `bottom` and `right`'s lower part. `top` is on the part that stays.
    let bottom = ("p".to_owned(), "notch on_bottom".to_owned());
    let (was, now) = (
        flat.places[&bottom].point.expect("a place"),
        spread.places[&bottom].point.expect("a place"),
    );
    assert_ne!(was, now, "the bottom turned with the part it is on");
    let top = ("p".to_owned(), "notch on_top".to_owned());
    assert_eq!(
        flat.places[&top].point, spread.places[&top].point,
        "the top did not turn"
    );
    // A turn keeps the distance from the pivot.
    let pivot = (400_000, 300_000);
    assert!(
        (away(was, pivot) - away(now, pivot)).abs() <= SAME_PLACE,
        "{was:?} and {now:?} about {pivot:?}"
    );
}

#[test]
fn any_sequence_of_edits_keeps_every_carrier_or_lists_it() {
    let mut dice = Dice(7);
    for trial in 0..TRIALS {
        let held = places(&mut dice);
        let mut disk = MemoryRepository::new();
        let mut running = start(&held, &mut disk);
        let before = seen(&mut running);
        let mut live: Vec<String> = EDGES.iter().map(|edge| (*edge).to_owned()).collect();
        let mut fresh = 0;
        let mut name = || {
            fresh += 1;
            format!("e{fresh}")
        };
        let mut said = Vec::new();
        for _ in 0..(3 + dice.below(5)) {
            let which = dice.below(live.len() as u64) as usize;
            let edge = live[which].clone();
            let text = match dice.below(3) {
                0 => {
                    let (p, q) = (name(), name());
                    let at = dice.fraction();
                    live.splice(which..=which, [p.clone(), q.clone()]);
                    format!(r#""op":"split","edge":"{edge}","at":"{at}","into":["{p}","{q}"]"#)
                }
                1 => format!(r#""op":"reverse","edge":"{edge}""#),
                _ => {
                    format!(r#""op":"allowance","edge":"{edge}","width":"sa","corner":"miter""#)
                }
            };
            said.push(text.clone());
            step(
                &mut running,
                &format!(r#"{{"do":"edit_edge","piece":"panel",{text}}}"#),
            )
            .unwrap_or_else(|message| panic!("trial {trial}: {said:?}: {message}"));
        }
        let after = seen(&mut running);
        // Nothing here deletes, and a carrier exactly at a split is its own case, so everything keeps its place or asks which side.
        for (key, was) in &before.places {
            let now = &after.places[key];
            if now.now == "unresolved" {
                let task = after
                    .repairs
                    .iter()
                    .find(|repair| repair["carrier"].as_text() == Some(&key.1))
                    .unwrap_or_else(|| panic!("trial {trial}: {key:?} has no task"));
                assert!(
                    task["orphaned_by"]
                        .as_text()
                        .expect("text")
                        .contains("choose"),
                    "trial {trial}"
                );
            } else {
                let (was, is) = (was.point.expect("a place"), now.point.expect("a place"));
                assert!(
                    away(was, is) <= SAME_PLACE,
                    "trial {trial}: {key:?} moved from {was:?} to {is:?} after {said:?}"
                );
            }
        }
        step(&mut running, r#"{"do":"draft"}"#)
            .unwrap_or_else(|message| panic!("trial {trial}: {said:?}: {message}"));
    }
}

#[test]
fn a_caller_without_commit_is_refused_by_the_bus_itself() {
    let session = parse(&opening(400_000)).expect("valid");
    let caller = Caller {
        agent: true,
        authorities: vec![CommandAuthority::Inspect, CommandAuthority::Generate],
    };
    let held = [
        "1/2".to_owned(),
        "1/4".to_owned(),
        "1/3".to_owned(),
        "3/4".to_owned(),
    ];
    let mut disk = MemoryRepository::new();
    let mut running = Running::open_as(&session, &mut disk, &caller).expect("opens");
    step(&mut running, &draft_step(&held)).expect("draws");
    for edit in [
        r#"{"do":"edit_edge","piece":"panel","op":"reverse","edge":"right"}"#,
        r#"{"do":"repair_reference","piece":"panel","notch":"n_right","retire":true}"#,
    ] {
        let refused = step(&mut running, edit).expect_err("no commit");
        assert!(
            refused.to_lowercase().contains("commit")
                || refused.contains("authority")
                || refused.contains("not authorized"),
            "{refused}"
        );
    }
    assert!(seen(&mut running).repairs.is_empty());
}

#[test]
fn the_reference_skirt_holds_edges_and_a_split_of_any_of_them_changes_no_seam() {
    let session = parse(SKIRT).expect("valid");
    let steps = session.as_object().expect("a session")["steps"]
        .as_list()
        .expect("steps")
        .to_vec();
    let draft_step = steps
        .iter()
        .find(|item| item.as_object().expect("a step")["do"].as_text() == Some("draft"))
        .expect("a draft");
    let mut opening = session.as_object().expect("a session").clone();
    opening.insert("steps".to_owned(), Value::List(Vec::new()));
    let mut disk = MemoryRepository::new();
    let mut running = Running::open(&Value::Object(opening), &mut disk).expect("opens");
    running.step(draft_step).expect("draws");
    let before = seen(&mut running);
    assert!(before.repairs.is_empty(), "{:?}", before.repairs);
    assert!(
        before.places.len() >= 30,
        "{} references",
        before.places.len()
    );
    for (piece, edge) in [
        ("skirt_front", "side_lower"),
        ("skirt_front", "side_upper"),
        ("skirt_front", "waist"),
        ("skirt_back_right", "cb"),
        ("waistband", "band_lower"),
    ] {
        step(
            &mut running,
            &format!(r#"{{"do":"edit_edge","piece":"{piece}","op":"split","edge":"{edge}","at":"7/23","into":["{edge}_p","{edge}_q"]}}"#),
        )
        .expect("splits");
    }
    holds(&before, &seen(&mut running), &[], "the skirt, split");
    step(&mut running, r#"{"do":"draft"}"#).expect("draws");
    let walked = result(&step(&mut running, r#"{"do":"walk"}"#).expect("walks"));
    for report in walked.as_list().expect("reports") {
        let report = report.as_object().expect("a report");
        assert_eq!(report["finding"].as_text(), Some("matches"), "{report:?}");
    }
}

#[test]
fn replaying_the_same_edits_gives_the_same_bytes() {
    let held = [
        "1/3".to_owned(),
        "1/4".to_owned(),
        "2/3".to_owned(),
        "3/4".to_owned(),
    ];
    let session = |edits: &str| {
        format!(
            r#"{{"format":1,"design":"01ARZ3NDEKTSV4RRFFQ69G5FAV",
            "inputs":[{{"token":"wide","um":400000}},{{"token":"tall","um":600000}},{{"token":"sa","um":10000}},{{"token":"wider","um":200000}},{{"token":"over","um":60000}}],
            "recipe":"let depth: length = 5 mm\nlet control_a: length = wide - 100 mm\nlet control_b: length = 100 mm\nlet rise: length = tall + 50 mm\nlet half: length = tall / 2\n",
            "steps":[{},{edits}{{"do":"draft"}},{{"do":"export_pattern","format":"canonical","path":"exports/c.json"}},{{"do":"save"}}]}}"#,
            draft_step(&held)
        )
    };
    let edits = r#"{"do":"edit_edge","piece":"panel","op":"split","edge":"top","at":"2/7","into":["t1","t2"]},
        {"do":"edit_edge","piece":"panel","op":"reverse","edge":"t2"},
        {"do":"edit_edge","piece":"twin","op":"delete","edge":"right","bridge":"stand_in"},"#;
    let run = || {
        let mut disk = MemoryRepository::new();
        replay(&parse(&session(edits)).expect("valid"), &mut disk).expect("replays");
        disk.read(&FilePath::new("exports/c.json").expect("a path"))
            .expect("reads")
            .expect("written")
    };
    assert_eq!(run(), run());
    let pattern = from_canonical_text(std::str::from_utf8(&run()).expect("text")).expect("reads");
    assert_eq!(pattern.repairs.len(), 4, "{:?}", pattern.repairs);
}

// D404: a reference is a fraction of its edge, and a true moves corners, so a true on a reference in the middle of an edge, or on a mirror, moves nothing. It is refused rather than reported as done.
#[test]
fn a_true_that_could_move_nothing_is_refused_and_changes_nothing() {
    let rectangle = |name: &str, width: &str| {
        format!(
            r#"{{"name":"{name}","quantity":1,"start":["zero","zero"],
            "edges":[{{"name":"low","to":["{width}","zero"],"width":"zero","corner":"miter"}},
                     {{"name":"side","to":["{width}","tall"],"width":"zero","corner":"miter"}},
                     {{"name":"high","to":["zero","tall"],"width":"zero","corner":"miter"}},
                     {{"name":"back","to":["zero","zero"],"width":"zero","corner":"miter"}}],
            "grain":[[["zero","zero"],["zero","tall"]]]}}"#
        )
    };
    let session = parse(
        r#"{"format":1,"design":"01ARZ3NDEKTSV4RRFFQ69G5FAV",
        "inputs":[{"token":"wide","um":400000},{"token":"narrow","um":397000},{"token":"tall","um":100000},{"token":"part","um":100000},{"token":"more","um":103000}],
        "recipe":"let same: length = tall\n","steps":[]}"#,
    )
    .expect("valid");
    let in_the_middle_of_an_edge = format!(
        r#"{{"do":"draft","style":"two","size":"M","pieces":[{},{}],
        "seams":[{{"name":"out","a":{{"piece":"one","from":{{"edge":"high","at":"1"}},"to":{{"edge":"high","at":"3/4"}}}},"b":{{"piece":"two","from":{{"edge":"high","at":"1"}},"to":{{"edge":"high","at":"3/4"}}}}}}]}}"#,
        rectangle("one", "wide"),
        rectangle("two", "narrow")
    );
    let on_a_mirror = format!(
        r#"{{"do":"draft","style":"two","size":"M","pieces":[{},{{"name":"copy","quantity":1,"mirror_of":"one"}},{}],
        "seams":[{{"name":"out","a":{{"piece":"copy","from":{{"edge":"high","at":"1"}},"to":{{"edge":"high","at":"0"}}}},"b":{{"piece":"two","from":{{"edge":"high","at":"1"}},"to":{{"edge":"high","at":"0"}}}}}}]}}"#,
        rectangle("one", "wide"),
        rectangle("two", "narrow")
    );
    for (case, draft) in [
        ("mid-edge", in_the_middle_of_an_edge),
        ("mirror", on_a_mirror),
    ] {
        let mut disk = MemoryRepository::new();
        let mut running = Running::open(&session, &mut disk).expect("opens");
        step(&mut running, &draft).unwrap_or_else(|message| panic!("{case}: {message}"));
        let revision = running.revision();
        let refused = step(
            &mut running,
            r#"{"do":"true_seam","seam":"out","side":"a","limit_um":10000}"#,
        )
        .expect_err(case);
        assert!(
            refused.contains("is not a corner of a piece that can move"),
            "{case}: {refused}"
        );
        assert_eq!(running.revision(), revision, "{case}: nothing changed");
        let walked = result(&step(&mut running, r#"{"do":"walk"}"#).expect("walks"));
        let report = walked.as_list().expect("reports")[0]
            .as_object()
            .expect("a report")
            .clone();
        assert_eq!(
            report["finding"].as_text(),
            Some("defect"),
            "{case}: {report:?}"
        );
    }
}

// An end written as a position is its own place, so truing one moves that place along the line and the walk reports the new stretch. No corner moves, and the outline is what it was.
#[test]
fn a_true_on_a_position_in_the_middle_of_an_edge_moves_that_position_and_no_corner() {
    let rectangle = |name: &str| {
        format!(
            r#"{{"name":"{name}","quantity":1,"start":["zero","zero"],
            "edges":[{{"to":["wide","zero"],"width":"zero","corner":"miter"}},{{"to":["wide","tall"],"width":"zero","corner":"miter"}},
                     {{"to":["zero","tall"],"width":"zero","corner":"miter"}},{{"to":["zero","zero"],"width":"zero","corner":"miter"}}],
            "grain":[[["zero","zero"],["zero","tall"]]]}}"#
        )
    };
    let session = parse(
        r#"{"format":1,"design":"01ARZ3NDEKTSV4RRFFQ69G5FAV",
        "inputs":[{"token":"wide","um":400000},{"token":"tall","um":100000},{"token":"part","um":100000},{"token":"more","um":103000}],
        "recipe":"let same: length = tall\n","steps":[]}"#,
    )
    .expect("valid");
    let draft = format!(
        r#"{{"do":"draft","style":"two","size":"M","pieces":[{},{}],
        "seams":[{{"name":"out","a":{{"piece":"one","from":["zero","tall"],"to":["part","tall"]}},"b":{{"piece":"two","from":["zero","tall"],"to":["more","tall"]}}}}]}}"#,
        rectangle("one"),
        rectangle("two")
    );
    let mut disk = MemoryRepository::new();
    let mut running = Running::open(&session, &mut disk).expect("opens");
    step(&mut running, &draft).expect("draws");
    let outline = running.pattern().expect("drawn").pieces[0].sew.clone();
    let trued = result(
        &step(
            &mut running,
            r#"{"do":"true_seam","seam":"out","side":"a","limit_um":10000}"#,
        )
        .expect("trues"),
    );
    let after = trued.as_object().expect("a report")["after"].clone();
    assert_eq!(
        after.as_object().expect("a report")["a_length"].as_int(),
        Some(103_000)
    );
    step(&mut running, r#"{"do":"draft"}"#).expect("draws");
    assert_eq!(running.pattern().expect("drawn").pieces[0].sew, outline);
}
