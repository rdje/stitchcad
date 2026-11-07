//! The DXF writer, read back by a reader written here that shares no code with it.
#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::cast_possible_truncation
)]
use sc_artifacts::{
    write_dxf, DrawnPiece, DxfError, Flattening, Line, LineSemantics, Mark, MarkKind, Pattern, Text,
};
use sc_geometry::Point;
use sc_store::parse;
use std::collections::BTreeMap;

fn p(x: i64, y: i64) -> Point {
    Point::new(x, y).expect("in range")
}

const FLAT: Flattening = Flattening {
    bound: 100,
    achieved: 40,
};

fn front() -> DrawnPiece {
    DrawnPiece {
        name: "front".to_owned(),
        size: "M".to_owned(),
        quantity: 2,
        cut: vec![
            p(-1_000, -1_500),
            p(101_250, -1_500),
            p(101_250, 101_000),
            p(-1_000, 101_000),
        ],
        sew: vec![p(0, 0), p(100_000, 0), p(100_000, 100_000), p(0, 100_000)],
        grain: Line {
            from: p(50_000, 10_000),
            to: p(50_000, 90_000),
        },
        internal: vec![Line {
            from: p(20_000, 100_000),
            to: p(25_000, 80_000),
        }],
        marks: vec![
            Mark {
                kind: MarkKind::VNotch,
                at: p(100_000, 50_000),
                toward: p(99_000, 50_000),
            },
            Mark {
                kind: MarkKind::Drill,
                at: p(25_000, 82_000),
                toward: p(25_000, 82_000),
            },
        ],
        labels: vec![Text {
            at: p(40_000, 40_000),
            text: "front".to_owned(),
        }],
    }
}

fn pattern() -> Pattern {
    Pattern {
        style: "skirt".to_owned(),
        design: "01ARZ3NDEKTSV4RRFFQ69G5FAV".to_owned(),
        revision: "3".to_owned(),
        pieces: vec![
            front(),
            DrawnPiece {
                name: "back".to_owned(),
                ..front()
            },
        ],
    }
}

/// One entity as a reader sees it: its type and its group codes in order.
#[derive(Debug, Clone)]
struct Entity {
    kind: String,
    codes: Vec<(i32, String)>,
}

impl Entity {
    fn get(&self, code: i32) -> Option<&str> {
        self.codes
            .iter()
            .find(|(found, _)| *found == code)
            .map(|(_, value)| value.as_str())
    }
    fn number(&self, code: i32) -> f64 {
        self.get(code)
            .expect("the code is there")
            .parse()
            .expect("a number")
    }
    fn layer(&self) -> &str {
        self.get(8).unwrap_or("")
    }
}

/// Reads a DXF file into its sections' entities, refusing anything not in code and value pairs.
fn read(text: &str) -> BTreeMap<String, Vec<Entity>> {
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len() % 2, 0, "pairs");
    let mut sections = BTreeMap::new();
    let mut section = String::new();
    let mut current: Vec<Entity> = Vec::new();
    for pair in lines.chunks(2) {
        let code: i32 = pair[0].trim().parse().expect("a group code");
        let value = pair[1].to_owned();
        if code == 0 {
            if value == "ENDSEC" {
                sections.insert(core::mem::take(&mut section), core::mem::take(&mut current));
                continue;
            }
            current.push(Entity {
                kind: value,
                codes: Vec::new(),
            });
        } else if let Some(entity) = current.last_mut() {
            if entity.kind == "SECTION" && code == 2 {
                section = value;
                current.clear();
            } else {
                entity.codes.push((code, value));
            }
        }
    }
    sections
}

fn micrometres(value: f64) -> i64 {
    (value * 1000.0).round() as i64
}

/// The vertices of the polyline starting at `at`, in micrometres.
fn vertices(entities: &[Entity], at: usize) -> Vec<(i64, i64)> {
    entities[at + 1..]
        .iter()
        .take_while(|entity| entity.kind == "VERTEX")
        .map(|vertex| {
            (
                micrometres(vertex.number(10)),
                micrometres(vertex.number(20)),
            )
        })
        .collect()
}

#[test]
fn the_file_reads_back_with_its_blocks_layers_and_exact_coordinates() {
    let artifact = write_dxf(&pattern(), LineSemantics::CutAsCut, FLAT).expect("writes");
    let sections = read(&artifact.text);
    assert!(artifact.text.ends_with("  0\nEOF\n"));
    assert!(artifact.text.contains("AC1009"), "release R12");

    // One block per piece, named [piece]_[size], with no nested INSERT.
    let blocks = &sections["BLOCKS"];
    let names: Vec<&str> = blocks
        .iter()
        .filter(|e| e.kind == "BLOCK")
        .map(|e| e.get(2).expect("a name"))
        .collect();
    assert_eq!(names, ["back_M", "front_M"], "in canonical order");
    assert!(blocks.iter().all(|e| e.kind != "INSERT"));
    let inserts: Vec<&str> = sections["ENTITIES"]
        .iter()
        .filter(|e| e.kind == "INSERT")
        .map(|e| e.get(2).expect("a name"))
        .collect();
    assert_eq!(inserts, names);

    // The cut boundary is closed on CUT, and each coordinate is the exact micrometre.
    let cut = blocks
        .iter()
        .position(|e| e.kind == "POLYLINE" && e.layer() == "CUT")
        .expect("a cut line");
    assert_eq!(blocks[cut].get(70), Some("1"), "closed");
    assert_eq!(
        vertices(blocks, cut),
        [
            (-1_000, -1_500),
            (101_250, -1_500),
            (101_250, 101_000),
            (-1_000, 101_000)
        ]
    );
    let sew = blocks
        .iter()
        .position(|e| e.kind == "POLYLINE" && e.layer() == "DRAW")
        .expect("a sew line");
    assert_eq!(
        vertices(blocks, sew),
        [(0, 0), (100_000, 0), (100_000, 100_000), (0, 100_000)]
    );

    // The grainline is a directed line on DRAW.
    let grain = blocks
        .iter()
        .find(|e| e.kind == "LINE" && e.layer() == "DRAW")
        .expect("a grainline");
    assert_eq!(
        (micrometres(grain.number(10)), micrometres(grain.number(20))),
        (50_000, 10_000)
    );
    assert_eq!(
        (micrometres(grain.number(11)), micrometres(grain.number(21))),
        (50_000, 90_000)
    );

    // A notch is on NOTCH, an internal line on INTCUT, and a drill hole is a closed polyline of two half circles.
    assert!(blocks
        .iter()
        .any(|e| e.kind == "LINE" && e.layer() == "NOTCH"));
    assert!(blocks
        .iter()
        .any(|e| e.kind == "POLYLINE" && e.layer() == "INTCUT"));
    let drill = blocks
        .iter()
        .position(|e| e.kind == "POLYLINE" && e.layer() == "DRILL")
        .expect("a drill hole");
    assert_eq!(blocks[drill + 1].get(42), Some("1.000000"));
    assert_eq!(blocks[drill + 2].get(42), Some("1.000000"));
}

#[test]
fn nothing_outside_the_written_set_appears_and_one_layer_keeps_one_meaning() {
    let artifact = write_dxf(&pattern(), LineSemantics::CutAsCut, FLAT).expect("writes");
    let sections = read(&artifact.text);
    let allowed = [
        "BLOCK", "ENDBLK", "POLYLINE", "VERTEX", "SEQEND", "LINE", "TEXT",
    ];
    for entity in &sections["BLOCKS"] {
        assert!(allowed.contains(&entity.kind.as_str()), "{}", entity.kind);
    }
    for banned in ["LWPOLYLINE", "SPLINE", "ARC", "CIRCLE", "MTEXT", "HATCH"] {
        assert!(
            !artifact.text.contains(&format!("\n{banned}\n")),
            "{banned}"
        );
    }
    // Every layer used is declared, and every layer declared is one of the named ones.
    let declared: Vec<String> = sections["TABLES"]
        .iter()
        .filter(|e| e.kind == "LAYER")
        .filter_map(|e| e.get(2).map(str::to_owned))
        .collect();
    assert_eq!(
        declared,
        ["CUT", "DRAW", "DRILL", "INTCUT", "NOTCH", "TEXT"]
    );
    for entity in &sections["BLOCKS"] {
        let layer = entity.layer();
        assert!(
            layer == "0" || declared.iter().any(|name| name == layer),
            "{layer}"
        );
    }
    for entity in sections["BLOCKS"].iter().filter(|e| e.kind == "TEXT") {
        assert_eq!(entity.layer(), "TEXT");
    }
    for entity in sections["BLOCKS"]
        .iter()
        .filter(|e| e.kind == "POLYLINE" && e.get(70) == Some("0"))
    {
        assert_eq!(
            entity.layer(),
            "INTCUT",
            "an open polyline is an internal line"
        );
    }
}

#[test]
fn the_labels_carry_the_piece_data() {
    let artifact = write_dxf(&pattern(), LineSemantics::CutAsCut, FLAT).expect("writes");
    let sections = read(&artifact.text);
    let said: Vec<&str> = sections["BLOCKS"]
        .iter()
        .filter(|e| e.kind == "TEXT")
        .filter_map(|e| e.get(1))
        .collect();
    for wanted in [
        "Piece Name: back",
        "Piece Name: front",
        "Size: M",
        "Quantity: 2",
        "Style Name: skirt",
        "front",
    ] {
        assert!(said.contains(&wanted), "{wanted} in {said:?}");
    }
}

#[test]
fn swapping_the_line_semantics_moves_only_which_layer_each_line_is_on() {
    let cut = write_dxf(&pattern(), LineSemantics::CutAsCut, FLAT).expect("writes");
    let sew = write_dxf(&pattern(), LineSemantics::SewAsCut, FLAT).expect("writes");
    assert_ne!(cut.text, sew.text);
    let (a, b) = (read(&cut.text), read(&sew.text));
    let on = |sections: &BTreeMap<String, Vec<Entity>>, layer: &str| {
        let blocks = &sections["BLOCKS"];
        let at = blocks
            .iter()
            .position(|e| e.kind == "POLYLINE" && e.layer() == layer)
            .expect("a line");
        vertices(blocks, at)
    };
    assert_eq!(
        on(&a, "CUT"),
        on(&b, "DRAW"),
        "the cut boundary is the same geometry"
    );
    assert_eq!(
        on(&a, "DRAW"),
        on(&b, "CUT"),
        "the sew line is the same geometry"
    );
    // Everything but those lines is unchanged: a sorted list of every other entity's codes is the same.
    let others = |sections: &BTreeMap<String, Vec<Entity>>| {
        let mut rest: Vec<String> = sections["BLOCKS"]
            .iter()
            .filter(|e| {
                !matches!(e.kind.as_str(), "POLYLINE" | "VERTEX" | "SEQEND")
                    || e.layer() == "INTCUT"
                    || e.layer() == "DRILL"
            })
            .map(|e| format!("{e:?}"))
            .collect();
        rest.sort();
        rest
    };
    assert_eq!(others(&a), others(&b));
}

#[test]
fn the_record_is_complete_and_states_what_the_named_layers_lose() {
    let artifact = write_dxf(&pattern(), LineSemantics::SewAsCut, FLAT).expect("writes");
    let record = parse(&artifact.record).expect("canonical text");
    let fields = record.as_object().expect("an object");
    for (field, wanted) in [
        ("target", "dxf-aama-named"),
        ("unit", "millimetre"),
        ("writer", "stitchcad-dxf-1"),
        ("design_revision", "3"),
        ("approval", "unapproved"),
    ] {
        assert_eq!(fields[field].as_text(), Some(wanted), "{field}");
    }
    assert_eq!(fields["tessellation_bound_um"].as_int(), Some(100));
    assert_eq!(fields["tessellation_achieved_um"].as_int(), Some(40));
    let axes = fields["axes"].as_object().expect("axes");
    for (axis, wanted) in [
        ("layer_naming", "aama_named"),
        ("dxf_version_target", "r12"),
        ("cut_sew_swap", "sew_as_1"),
        ("seam_allowance_policy", "included_in_contour"),
        ("grading_mode", "single_size"),
        ("entity_policy", "polyline_only"),
    ] {
        assert_eq!(axes[axis].as_text(), Some(wanted), "{axis}");
    }
    assert!(!fields["losses"].as_list().expect("losses").is_empty());
    for forbidden in ["time", "date", "host"] {
        assert!(
            !artifact.record.contains(forbidden) && !artifact.text.contains(forbidden),
            "{forbidden}"
        );
    }
}

#[test]
fn two_runs_give_one_byte_string_whatever_order_the_pieces_came_in() {
    let first = write_dxf(&pattern(), LineSemantics::CutAsCut, FLAT).expect("writes");
    let mut shuffled = pattern();
    shuffled.pieces.reverse();
    shuffled.pieces[0].marks.reverse();
    assert_eq!(
        write_dxf(&shuffled, LineSemantics::CutAsCut, FLAT).expect("writes"),
        first
    );
}

#[test]
fn what_cannot_be_written_properly_is_refused() {
    let with = |change: fn(&mut Pattern)| {
        let mut changed = pattern();
        change(&mut changed);
        write_dxf(&changed, LineSemantics::CutAsCut, FLAT).expect_err("refused")
    };
    assert!(matches!(
        with(|p| p.pieces[0].name = "front piece".to_owned()),
        DxfError::BadBlockName { .. }
    ));
    assert!(matches!(
        with(|p| p.pieces[0].size = String::new()),
        DxfError::BadBlockName { .. }
    ));
    assert!(matches!(
        with(|p| p.pieces[1].name = "front".to_owned()),
        DxfError::DuplicateBlock { .. }
    ));
    assert!(matches!(
        with(|p| p.pieces[0].cut.truncate(2)),
        DxfError::NotClosed { line: "cut", .. }
    ));
    assert!(matches!(
        with(|p| p.pieces[0].labels[0].text = "two\nlines".to_owned()),
        DxfError::BadLabel { .. }
    ));
    assert_eq!(
        write_dxf(
            &pattern(),
            LineSemantics::CutAsCut,
            Flattening {
                bound: 100,
                achieved: 101
            }
        ),
        Err(DxfError::TessellationBound {
            bound: 100,
            achieved: 101
        })
    );
}

#[test]
fn a_closed_line_starting_anywhere_writes_one_file() {
    let first = write_dxf(&pattern(), LineSemantics::CutAsCut, FLAT).expect("writes");
    let mut turned = pattern();
    turned.pieces[0].cut.rotate_left(1);
    turned.pieces[0].sew.rotate_left(2);
    assert_eq!(
        write_dxf(&turned, LineSemantics::CutAsCut, FLAT).expect("writes"),
        first
    );
}
