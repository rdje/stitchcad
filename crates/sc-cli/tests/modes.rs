//! The three ways grading travels, each read back from the written file by a small reader here, and each with what it loses said out loud.
#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss
)]
use sc_cli::replay;
use sc_grading::{from_rul, GradeTable};
use sc_store::{parse, FilePath, MemoryRepository, Repository, Value};
use std::collections::BTreeMap;

const SKIRT: &str = include_str!("../../../conformance/reference-skirt/session.json");
const BODICE: &str = include_str!("../../../conformance/reference-bodice/session.json");
const TROUSERS: &str = include_str!("../../../conformance/reference-trousers/session.json");
const BOOK: &str = include_str!("../../../docs/book/src/spec/grading.md");
const DIGESTS: &str = include_str!("../../../conformance/goldens/grading/digests.tsv");
const SIZES: [&str; 5] = ["XS", "S", "M", "L", "XL"];
const MODES: [&str; 3] = ["base_plus_rules", "embedded_rules", "all_contours"];

fn fields_of(value: &Value) -> &BTreeMap<String, Value> {
    value.as_object().expect("an object")
}

fn with_steps(session: &str, extra: &str) -> Value {
    let whole = parse(session).expect("valid");
    let mut fields = fields_of(&whole).clone();
    let mut steps: Vec<Value> = fields["steps"]
        .as_list()
        .expect("steps")
        .iter()
        .filter(|step| fields_of(step)["do"].as_text() != Some("save"))
        .cloned()
        .collect();
    steps.extend(
        parse(extra)
            .expect("valid")
            .as_list()
            .expect("steps")
            .iter()
            .cloned(),
    );
    fields.insert("steps".to_owned(), Value::List(steps));
    Value::Object(fields)
}

fn exported(session: &str) -> MemoryRepository {
    let mut disk = MemoryRepository::new();
    let mut steps = vec![r#"{"do":"grade","rules_path":"exports/rules.json"}"#.to_owned()];
    for mode in MODES {
        steps.push(format!(
            r#"{{"do":"export_graded","mode":"{mode}","semantics":"sew_as_1","path":"exports/{mode}"}}"#
        ));
    }
    steps.push(r#"{"do":"save"}"#.to_owned());
    let plan = with_steps(session, &format!("[{}]", steps.join(",")));
    replay(&plan, &mut disk).unwrap_or_else(|failure| panic!("{failure:?}"));
    disk
}

fn file(disk: &MemoryRepository, name: &str) -> String {
    String::from_utf8(
        disk.read(&FilePath::new(name).expect("a path"))
            .expect("reads")
            .unwrap_or_else(|| panic!("{name} was not written")),
    )
    .expect("text")
}

fn refused(session: &str, step: &str) -> String {
    let plan = with_steps(session, &format!("[{step}]"));
    let failure = replay(&plan, MemoryRepository::new())
        .expect_err(step)
        .to_value();
    fields_of(&failure)["message"]
        .as_text()
        .expect("text")
        .to_owned()
}

// A plain reader of what these files hold, in millimetres to micrometres. It knows nothing of the writer but the format.
type At = (i64, i64);

#[derive(Default, Debug)]
struct Block {
    lines: BTreeMap<String, Vec<Vec<At>>>,
    straight: BTreeMap<String, Vec<(At, At)>>,
    points: BTreeMap<String, Vec<At>>,
    texts: Vec<(String, (i64, i64), String)>,
}

fn micrometres(text: &str) -> i64 {
    let value: f64 = text.trim().parse().expect("a number");
    (value * 1000.0).round() as i64
}

struct Read {
    version: String,
    layers: Vec<String>,
    blocks: Vec<(String, Block)>,
    entities: Vec<(String, (i64, i64), String)>,
}

fn read(text: &str) -> Read {
    let lines: Vec<&str> = text.lines().collect();
    let pairs: Vec<(i32, &str)> = lines
        .chunks(2)
        .map(|pair| {
            (
                pair[0].trim().parse().expect("a group code"),
                pair[1].trim(),
            )
        })
        .collect();
    let mut out = Read {
        version: String::new(),
        layers: Vec::new(),
        blocks: Vec::new(),
        entities: Vec::new(),
    };
    let mut section = "";
    let mut index = 0;
    while index < pairs.len() {
        let (code, value) = pairs[index];
        if code == 0 && value == "SECTION" {
            section = pairs[index + 1].1;
            index += 2;
            continue;
        }
        if code == 9 && value == "$ACADVER" {
            out.version = pairs[index + 1].1.to_owned();
        }
        if section == "TABLES" && code == 0 && value == "LAYER" {
            out.layers.push(pairs[index + 1].1.to_owned());
        }
        if code == 0 && value == "BLOCK" {
            let name = pairs[index + 2].1.to_owned();
            out.blocks.push((name, Block::default()));
        }
        if code == 0 && value == "POINT" {
            let mut cursor = index + 1;
            let (mut layer, mut x, mut y) = ("", 0, 0);
            while pairs[cursor].0 != 0 {
                match pairs[cursor].0 {
                    8 => layer = pairs[cursor].1,
                    10 => x = micrometres(pairs[cursor].1),
                    20 => y = micrometres(pairs[cursor].1),
                    _ => {}
                }
                cursor += 1;
            }
            let block = &mut out.blocks.last_mut().expect("a block").1;
            block
                .points
                .entry(layer.to_owned())
                .or_default()
                .push((x, y));
            index = cursor;
            continue;
        }
        if code == 0 && matches!(value, "POLYLINE" | "LINE" | "TEXT") {
            let mut fields: BTreeMap<i32, Vec<&str>> = BTreeMap::new();
            let mut cursor = index + 1;
            let kind = value;
            let mut vertices = Vec::new();
            if kind == "POLYLINE" {
                let layer = pairs[cursor].1;
                loop {
                    let (code, value) = pairs[cursor];
                    if code == 0 && value == "SEQEND" {
                        break;
                    }
                    if code == 10 {
                        vertices.push((micrometres(value), micrometres(pairs[cursor + 1].1)));
                    }
                    cursor += 1;
                }
                let block = &mut out.blocks.last_mut().expect("a block").1;
                block
                    .lines
                    .entry(layer.to_owned())
                    .or_default()
                    .push(vertices);
            } else {
                while pairs[cursor].0 != 0 {
                    fields
                        .entry(pairs[cursor].0)
                        .or_default()
                        .push(pairs[cursor].1);
                    cursor += 1;
                }
                let layer = fields[&8][0];
                let at = (micrometres(fields[&10][0]), micrometres(fields[&20][0]));
                if kind == "LINE" {
                    let to = (micrometres(fields[&11][0]), micrometres(fields[&21][0]));
                    let block = &mut out.blocks.last_mut().expect("a block").1;
                    block
                        .straight
                        .entry(layer.to_owned())
                        .or_default()
                        .push((at, to));
                } else if section == "ENTITIES" {
                    out.entities
                        .push((layer.to_owned(), at, fields[&1][0].to_owned()));
                } else {
                    let block = &mut out.blocks.last_mut().expect("a block").1;
                    block
                        .texts
                        .push((layer.to_owned(), at, fields[&1][0].to_owned()));
                }
            }
            index = cursor;
            continue;
        }
        index += 1;
    }
    out
}

fn table_of(disk: &MemoryRepository) -> GradeTable {
    GradeTable::from_value(&parse(&file(disk, "exports/rules.json")).expect("valid"))
        .expect("reads")
}

fn close(a: (i64, i64), b: (i64, i64), within: i64) -> bool {
    (a.0 - b.0).abs() <= within && (a.1 - b.1).abs() <= within
}

// Every identifier sits on a point a receiver can find, on the layer that point is on: the standard's own requirement.
fn identifiers_sit_on_points(read: &Read, within: i64, boundary_layer: &str) {
    let mut seen = 0;
    for (name, block) in &read.blocks {
        for (layer, at, value) in &block.texts {
            if !value.starts_with("# ") {
                continue;
            }
            seen += 1;
            let on_a_vertex = block
                .lines
                .get(layer)
                .into_iter()
                .flatten()
                .flatten()
                .any(|vertex| close(*vertex, *at, within));
            let on_a_line = block
                .straight
                .get(layer)
                .into_iter()
                .flatten()
                .any(|(from, to)| close(*from, *at, within) || close(*to, *at, within));
            let at_a_drill = block.lines.get(layer).into_iter().flatten().any(|hole| {
                hole.len() == 2 && close(((hole[0].0 + hole[1].0) / 2, hole[0].1), *at, within)
            }) || block
                .points
                .get(layer)
                .into_iter()
                .flatten()
                .any(|point| close(*point, *at, within));
            assert!(
                on_a_vertex || on_a_line || at_a_drill,
                "{name}: {value} at {at:?} on {layer} is on nothing"
            );
        }
        let corners = block
            .texts
            .iter()
            .filter(|(layer, _, value)| layer == boundary_layer && value.starts_with("# "))
            .count();
        assert!(
            corners >= 3,
            "{name} has {corners} identifiers on its boundary"
        );
    }
    assert!(seen > 10, "{seen} identifiers");
}

#[test]
fn base_plus_rules_is_named_layers_identifiers_at_points_and_a_table_for_every_one() {
    for session in [SKIRT, BODICE, TROUSERS] {
        let disk = exported(session);
        let text = file(&disk, "exports/base_plus_rules.dxf");
        let got = read(&text);
        assert_eq!(got.version, "AC1009");
        for layer in &got.layers {
            assert!(
                ["CUT", "DRAW", "DRILL", "INTCUT", "NOTCH", "TEXT"].contains(&layer.as_str()),
                "{layer}"
            );
        }
        identifiers_sit_on_points(&got, 0, "CUT");
        // A rule file comes with it, and every identifier in the drawing has a rule in it.
        let rules = from_rul(&file(&disk, "exports/base_plus_rules.rul")).expect("reads");
        for (name, block) in &got.blocks {
            for (_, _, value) in &block.texts {
                if let Some(id) = value.strip_prefix("# ") {
                    let id: i64 = id.parse().expect("a number");
                    assert!(
                        rules.rules.iter().any(|rule| rule.id == id),
                        "{name}: {id} has no rule"
                    );
                }
            }
        }
        // A mirror is told by its pair's quantity, and is not a block of its own.
        for (name, block) in &got.blocks {
            assert!(!name.contains("_left_"), "{name}");
            assert!(
                block
                    .texts
                    .iter()
                    .any(|(_, _, value)| value.starts_with("Quantity: ")),
                "{name}"
            );
        }
    }
}

#[test]
fn a_pair_is_told_by_its_quantity() {
    let got = read(&file(&exported(BODICE), "exports/base_plus_rules.dxf"));
    let quantity = |name: &str| {
        got.blocks
            .iter()
            .find(|(block, _)| block == name)
            .and_then(|(_, block)| {
                block
                    .texts
                    .iter()
                    .find(|(_, _, value)| value.starts_with("Quantity: "))
            })
            .map(|(_, _, value)| value.clone())
            .unwrap_or_else(|| panic!("{name}"))
    };
    assert_eq!(quantity("bodice_back_right_M"), "Quantity: 1,1");
    assert_eq!(quantity("collar_right_M"), "Quantity: 2,2");
    assert_eq!(quantity("bodice_front_M"), "Quantity: 1,0");
}

#[test]
fn embedded_rules_is_numbered_layers_with_style_text_and_two_places() {
    for session in [SKIRT, BODICE, TROUSERS] {
        let disk = exported(session);
        let text = file(&disk, "exports/embedded_rules.dxf");
        let got = read(&text);
        assert_eq!(got.version, "AC1012");
        for layer in &got.layers {
            assert!(layer.chars().all(|c| c.is_ascii_digit()), "{layer}");
            assert_ne!(layer, "12", "layer 12 is left blank");
        }
        identifiers_sit_on_points(&got, 10, "1");
        for wanted in [
            "Style Name: ",
            "Sample Size: M",
            "Grade Rule Table: reference-",
            "Units: METRIC",
            "ASTM/D13 Proposal 1 Version: D 6673-10",
        ] {
            assert!(
                got.entities
                    .iter()
                    .any(|(layer, _, value)| layer == "1" && value.starts_with(wanted)),
                "{wanted}"
            );
        }
        // Every coordinate has two places, which is what its units say.
        let mut lines = text.lines();
        while let (Some(code), Some(value)) = (lines.next(), lines.next()) {
            if matches!(code.trim(), "10" | "20") && value != "0.0" {
                assert_eq!(
                    value.trim().rsplit('.').next().expect("places").len(),
                    2,
                    "{value}"
                );
            }
        }
        for (name, block) in &got.blocks {
            assert!(block.lines.contains_key("1"), "{name}");
            assert!(block.lines.contains_key("14"), "{name}");
            // The convention's own restriction: these layers cannot hold a polyline.
            for layer in ["5", "6", "7", "9", "10", "13"] {
                assert!(
                    !block.lines.contains_key(layer),
                    "{name}: a polyline on layer {layer}"
                );
            }
            // Layers 2 and 3 hold every turn point and curve point of the boundary between them.
            let (turn, curve) = (&block.points["2"], block.points.get("3"));
            for vertex in &block.lines["1"][0] {
                assert!(
                    turn.iter().any(|point| close(*point, *vertex, 10))
                        || curve.is_some_and(|curve| curve
                            .iter()
                            .any(|point| close(*point, *vertex, 10))),
                    "{name}: {vertex:?} is on neither"
                );
            }
        }
    }
}

#[test]
fn a_rule_moves_a_point_to_where_that_size_has_it() {
    // Corners, read from the file, moved by the file's own rules, and compared with the same corners in the file of contours, stacked on their reference lines. Two files agreeing that neither was written from the other.
    for session in [SKIRT, BODICE, TROUSERS] {
        let disk = exported(session);
        let base = read(&file(&disk, "exports/embedded_rules.dxf"));
        let nest = read(&file(&disk, "exports/all_contours.dxf"));
        let rules = from_rul(&file(&disk, "exports/embedded_rules.rul")).expect("reads");
        for (name, block) in &base.blocks {
            let piece = name.strip_suffix("_M").expect("the sample");
            for (layer, at, value) in &block.texts {
                let Some(id) = value.strip_prefix("# ") else {
                    continue;
                };
                if layer != "1" {
                    continue;
                }
                let id: i64 = id.parse().expect("a number");
                let rule = rules
                    .rules
                    .iter()
                    .find(|rule| rule.id == id)
                    .expect("the rule");
                for (at_size, size) in SIZES.iter().enumerate() {
                    let delta = rule.deltas[at_size];
                    let target = nest
                        .blocks
                        .iter()
                        .find(|(block, _)| *block == format!("{piece}_{size}"))
                        .unwrap_or_else(|| panic!("{piece}_{size}"));
                    let sample = nest
                        .blocks
                        .iter()
                        .find(|(block, _)| *block == format!("{piece}_M"))
                        .expect("the sample");
                    // Stack the size on the sample by their grade reference lines.
                    let reference = |block: &Block| block.straight["5"][0].0;
                    let (here, there) = (reference(&target.1), reference(&sample.1));
                    let wanted = (
                        at.0 + delta.0 + here.0 - there.0,
                        at.1 + delta.1 + here.1 - there.1,
                    );
                    let found = target.1.lines["1"][0]
                        .iter()
                        .any(|vertex| close(*vertex, wanted, 30));
                    assert!(found, "{piece} at {size}: rule {id} puts a corner at {wanted:?}, and the nest has none there");
                }
            }
        }
    }
}

#[test]
fn all_contours_is_every_size_in_order_with_the_same_points_and_only_the_sample_carries_the_rest() {
    for session in [SKIRT, BODICE, TROUSERS] {
        let disk = exported(session);
        let got = read(&file(&disk, "exports/all_contours.dxf"));
        assert_eq!(got.version, "AC1012");
        let table = table_of(&disk);
        let pieces: Vec<String> = got
            .blocks
            .iter()
            .filter_map(|(name, _)| name.strip_suffix("_M").map(str::to_owned))
            .collect();
        assert!(pieces.len() >= 3, "{pieces:?}");
        let order: Vec<&str> = got.blocks.iter().map(|(name, _)| name.as_str()).collect();
        assert_eq!(order.len(), pieces.len() * SIZES.len());
        // The order of the blocks is the order of the sizes.
        for piece in &pieces {
            let positions: Vec<usize> = SIZES
                .iter()
                .map(|size| {
                    order
                        .iter()
                        .position(|name| *name == format!("{piece}_{size}"))
                        .unwrap_or_else(|| panic!("{piece}_{size}"))
                })
                .collect();
            assert!(
                positions.windows(2).all(|pair| pair[0] < pair[1]),
                "{piece}: {positions:?}"
            );
            let block = |size: &str| {
                &got.blocks
                    .iter()
                    .find(|(name, _)| *name == format!("{piece}_{size}"))
                    .expect("the block")
                    .1
            };
            let points = block("M").lines["1"][0].len();
            for size in SIZES {
                let here = block(size);
                assert_eq!(
                    here.lines["1"][0].len(),
                    points,
                    "{piece} at {size}: the same quantity of points"
                );
                assert_eq!(
                    here.straight["5"].len(),
                    1,
                    "{piece} at {size} has a grade reference line"
                );
                assert!(here
                    .texts
                    .iter()
                    .any(|(layer, _, value)| layer == "1"
                        && *value == format!("Piece Name: {piece}")));
                assert!(here.texts.iter().any(
                    |(layer, _, value)| layer == "1" && *value == format!("Size Name: {size}")
                ));
                let carries_more = size == table.sample();
                for layer in ["14", "15"] {
                    assert_eq!(
                        here.lines.contains_key(layer)
                            || here.texts.iter().any(|(on, _, _)| on == layer),
                        carries_more,
                        "{piece} at {size}: layer {layer}"
                    );
                }
                // Notches, turn points and curve points are not repeated, as the standard says.
                for layer in ["2", "3", "4"] {
                    assert!(
                        carries_more || !here.points.contains_key(layer),
                        "{piece} at {size}: layer {layer}"
                    );
                }
                assert!(
                    carries_more || !here.lines.contains_key("8"),
                    "{piece} at {size}"
                );
            }
        }
        // No rules travel in this mode, and nothing beside it says otherwise.
        for (name, block) in &got.blocks {
            assert!(
                !block
                    .texts
                    .iter()
                    .any(|(_, _, value)| value.starts_with("# ")),
                "{name}"
            );
        }
        assert!(disk
            .read(&FilePath::new("exports/all_contours.rul").expect("a path"))
            .expect("reads")
            .is_none());
    }
}

#[test]
fn even_points_stay_within_what_the_target_allows() {
    // A nest's points are evenly spaced so that sizes correspond, and they are still within the tessellation bound of the line drawn normally.
    let disk = exported(BODICE);
    let got = read(&file(&disk, "exports/all_contours.dxf"));
    let base = read(&file(&disk, "exports/embedded_rules.dxf"));
    for (name, block) in &base.blocks {
        let fine = &block.lines["1"][0];
        let coarse = &got
            .blocks
            .iter()
            .find(|(other, _)| other == name)
            .expect("the block")
            .1
            .lines["1"][0];
        let to_segment = |at: (i64, i64), a: (i64, i64), b: (i64, i64)| {
            let (dx, dy) = ((b.0 - a.0) as f64, (b.1 - a.1) as f64);
            let reach = dx * dx + dy * dy;
            let t = if reach == 0.0 {
                0.0
            } else {
                ((((at.0 - a.0) as f64) * dx + ((at.1 - a.1) as f64) * dy) / reach).clamp(0.0, 1.0)
            };
            (((at.0 - a.0) as f64 - t * dx).powi(2) + ((at.1 - a.1) as f64 - t * dy).powi(2)).sqrt()
        };
        for at in fine {
            let away = coarse
                .iter()
                .enumerate()
                .map(|(index, start)| to_segment(*at, *start, coarse[(index + 1) % coarse.len()]))
                .fold(f64::MAX, f64::min);
            assert!(away <= 100.0, "{name}: {at:?} is {away} from the even line");
        }
    }
}

#[test]
fn each_mode_has_its_own_record_saying_what_it_loses_and_that_no_receiver_has_read_it() {
    let disk = exported(BODICE);
    let mut seen: Vec<Vec<String>> = Vec::new();
    for mode in MODES {
        let record =
            parse(&file(&disk, &format!("exports/{mode}.dxf.record.json"))).expect("valid");
        let record = fields_of(&record);
        assert_eq!(
            fields_of(&record["axes"])["grading_mode"].as_text(),
            Some(mode)
        );
        assert_eq!(
            fields_of(&record["axes"])["cut_sew_swap"].as_text(),
            Some("sew_as_1")
        );
        assert_eq!(
            fields_of(&record["axes"])["seam_allowance_policy"].as_text(),
            Some("generated_downstream")
        );
        assert!(
            record["receiver"]
                .as_text()
                .expect("text")
                .contains("no receiver has read"),
            "{mode}"
        );
        assert_eq!(record["approval"].as_text(), Some("unapproved"));
        let losses: Vec<String> = record["losses"]
            .as_list()
            .expect("losses")
            .iter()
            .filter_map(Value::as_text)
            .map(str::to_owned)
            .collect();
        assert!(losses.len() >= 3, "{mode}");
        for loss in &losses {
            assert!(
                BOOK.contains(&format!("`{loss}`")),
                "the book does not state {loss}"
            );
        }
        assert!(!seen.contains(&losses), "each mode loses its own");
        seen.push(losses);
    }
    assert!(!BOOK.to_lowercase().contains("universal package exists"));
}

#[test]
fn a_mode_that_cannot_be_written_is_refused_by_name() {
    for (step, says) in [
        (
            r#"{"do":"export_graded","mode":"universal","semantics":"sew_as_1","path":"exports/x"}"#,
            "no universal one",
        ),
        (
            r#"{"do":"export_graded","semantics":"sew_as_1","path":"exports/x"}"#,
            "names its mode",
        ),
        (
            r#"{"do":"export_graded","mode":"embedded_rules","semantics":"cut_as_1","path":"exports/x"}"#,
            "needs the net line",
        ),
        (
            r#"{"do":"export_graded","mode":"embedded_rules","path":"exports/x"}"#,
            "names its semantics",
        ),
        (
            r#"{"do":"export_graded","mode":"embedded_rules","semantics":"sew_as_1"}"#,
            "needs a path",
        ),
    ] {
        let said = refused(SKIRT, step);
        assert!(said.contains(says), "{step}: {said}");
    }
    // An open repair blocks a graded file as it does any cutting file.
    let said = refused(
        SKIRT,
        r#"{"do":"edit_edge","piece":"skirt_front","op":"delete","edge":"side_lower","bridge":"patch"},{"do":"draft"},{"do":"export_graded","mode":"embedded_rules","semantics":"sew_as_1","path":"exports/x"}"#,
    );
    assert!(said.contains("need repair"), "{said}");
}

// Each mode is frozen for each garment, by digest, and the skirt's in full so that a change can be read.
#[test]
fn every_file_of_every_mode_is_what_was_frozen() {
    let frozen: BTreeMap<&str, &str> = DIGESTS
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.split_once('\t'))
        .collect();
    let mut checked = 0;
    for (name, session) in [("skirt", SKIRT), ("bodice", BODICE), ("trousers", TROUSERS)] {
        let disk = exported(session);
        for mode in MODES {
            let mut files = vec![format!("{mode}.dxf"), format!("{mode}.dxf.record.json")];
            if mode != "all_contours" {
                files.push(format!("{mode}.rul"));
                files.push(format!("{mode}.rul.record.json"));
            }
            for part in files {
                let digest =
                    sc_store::sha256_hex(file(&disk, &format!("exports/{part}")).as_bytes());
                let key = format!("{name}/{part}");
                assert_eq!(
                    frozen.get(key.as_str()).copied(),
                    Some(digest.as_str()),
                    "{key} is not what was frozen: {digest}"
                );
                checked += 1;
            }
        }
    }
    assert_eq!(checked, frozen.len(), "nothing frozen that nothing checks");
    let disk = exported(SKIRT);
    assert_eq!(
        file(&disk, "exports/base_plus_rules.rul"),
        include_str!("../../../conformance/goldens/grading/reference-skirt/base_plus_rules.rul")
    );
    assert_eq!(
        file(&disk, "exports/embedded_rules.dxf"),
        include_str!("../../../conformance/goldens/grading/reference-skirt/embedded_rules.dxf")
    );
}
