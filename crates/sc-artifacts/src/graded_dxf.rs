//! The three ways grading travels. They are never combined, each is its own file with its own record, and none is claimed to serve every receiver.
use crate::dxf::{
    block_name, drill, line, millimetres, millimetres_to_two, pair, polyline, text_on, DxfError,
    Flattening, WRITER,
};
use crate::pattern::{DrawnPiece, Line, Mark, MarkKind, Pattern};
use sc_geometry::Point;
use sc_store::Value;
use std::collections::BTreeMap;

/// How grading travels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GradingMode {
    /// The sample's geometry on named layers, with identifiers at its points, and the deltas in a `.rul` file.
    BasePlusRules,
    /// The sample's geometry on numbered layers, with identifiers at its points. The deltas still need a table.
    EmbeddedRules,
    /// Every size's own contours on numbered layers, with no rules.
    AllContours,
}

impl GradingMode {
    /// Its name.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::BasePlusRules => "base_plus_rules",
            Self::EmbeddedRules => "embedded_rules",
            Self::AllContours => "all_contours",
        }
    }

    /// The registered target that carries it.
    #[must_use]
    pub const fn target(self) -> &'static str {
        match self {
            Self::BasePlusRules => "dxf-aama-named",
            Self::EmbeddedRules | Self::AllContours => "dxf-astm-num-r13",
        }
    }

    /// What it loses, for the record, because a receiver should learn that here.
    #[must_use]
    pub const fn losses(self) -> &'static [&'static str] {
        match self {
            Self::BasePlusRules => &[
                "named_layers_merge_turn_points_curve_points_and_the_boundary",
                "rules_only_on_the_points_listed_and_between_them_a_receiver_interpolates",
                "stack_points_fixed_contours_and_declared_zeros_are_not_carried",
                "the_recipe_is_not_carried",
            ],
            Self::EmbeddedRules => &[
                "the_file_holds_identifiers_and_the_deltas_are_in_a_table_it_names",
                "turn_points_and_curve_points_are_not_written_on_their_own_layers",
                "rules_only_on_the_points_listed_and_between_them_a_receiver_interpolates",
                "stack_points_fixed_contours_and_declared_zeros_are_not_carried",
                "the_recipe_is_not_carried",
                "coordinates_are_held_to_two_decimal_places",
            ],
            Self::AllContours => &[
                "no_rules_so_a_receiver_can_make_no_size_outside_the_nest",
                "only_the_sample_carries_notches_and_labels",
                "the_cut_line_is_only_on_the_sample_since_its_vertices_differ_by_size",
                "the_recipe_is_not_carried",
                "coordinates_are_held_to_two_decimal_places",
            ],
        }
    }
}

/// A point with a rule, as a file places it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RulePoint {
    /// The piece it is on.
    pub piece: String,
    /// Where it is on that piece.
    pub at: Point,
    /// The rule's number.
    pub id: i64,
    /// What it is on: `boundary`, `internal`, `grain` or `drill`.
    pub on: &'static str,
}

/// A piece of a pair: how many of it to cut, and how many of its mirror.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Paired {
    /// The piece.
    pub piece: String,
    /// How many to cut of the mirror.
    pub left: u32,
}

/// A graded file, with what travels beside it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GradedArtifact {
    /// The file.
    pub text: String,
    /// Its record, as canonical text.
    pub record: String,
}

fn record(mode: GradingMode, pattern: &Pattern, flattening: Flattening, unit: &str) -> String {
    let text_value = |value: &str| Value::Text(value.to_owned());
    let mut axes = BTreeMap::new();
    axes.insert(
        "layer_naming".to_owned(),
        text_value(match mode {
            GradingMode::BasePlusRules => "aama_named",
            _ => "astm_numbered",
        }),
    );
    axes.insert(
        "dxf_version_target".to_owned(),
        text_value(match mode {
            GradingMode::BasePlusRules => "r12",
            _ => "r13",
        }),
    );
    axes.insert("cut_sew_swap".to_owned(), text_value("sew_as_1"));
    axes.insert(
        "seam_allowance_policy".to_owned(),
        text_value("generated_downstream"),
    );
    axes.insert("grading_mode".to_owned(), text_value(mode.token()));
    axes.insert("entity_policy".to_owned(), text_value("polyline_only"));
    let mut fields = BTreeMap::new();
    fields.insert("target".to_owned(), text_value(mode.target()));
    fields.insert("axes".to_owned(), Value::Object(axes));
    fields.insert("unit".to_owned(), text_value(unit));
    fields.insert(
        "tessellation_bound_um".to_owned(),
        Value::Int(flattening.bound),
    );
    fields.insert(
        "tessellation_achieved_um".to_owned(),
        Value::Int(flattening.achieved),
    );
    fields.insert("writer".to_owned(), text_value(WRITER));
    fields.insert("design".to_owned(), text_value(&pattern.design));
    fields.insert("design_revision".to_owned(), text_value(&pattern.revision));
    fields.insert("approval".to_owned(), text_value("unapproved"));
    fields.insert(
        "receiver".to_owned(),
        text_value("none: no receiver has read a file of this mode"),
    );
    fields.insert(
        "losses".to_owned(),
        Value::List(mode.losses().iter().map(|loss| text_value(loss)).collect()),
    );
    Value::Object(fields).to_text()
}

// A point's identifier, as text at its location, on the layer the point is on. A rule of zeros is written like any other, because a point that does not move still has to say so.
fn identifier(out: &mut String, layer: &str, at: Point, id: i64, numeric: bool) {
    pair(out, 0, "TEXT");
    pair(out, 8, layer);
    if numeric {
        pair(out, 10, &millimetres_to_two(at.x()));
        pair(out, 20, &millimetres_to_two(at.y()));
    } else {
        pair(out, 10, &millimetres(at.x()));
        pair(out, 20, &millimetres(at.y()));
    }
    pair(out, 30, "0.0");
    pair(out, 40, "0.100");
    pair(out, 1, &format!("# {id}"));
}

fn quantity(piece: &DrawnPiece, paired: &[Paired]) -> String {
    let left = paired
        .iter()
        .find(|pair| pair.piece == piece.name)
        .map_or(0, |pair| pair.left);
    format!("Quantity: {},{left}", piece.quantity)
}

fn header(out: &mut String, version: &str, layers: &[(&str, &str)], comment: &str) {
    pair(out, 999, comment);
    pair(out, 0, "SECTION");
    pair(out, 2, "HEADER");
    pair(out, 9, "$ACADVER");
    pair(out, 1, version);
    pair(out, 0, "ENDSEC");
    pair(out, 0, "SECTION");
    pair(out, 2, "TABLES");
    pair(out, 0, "TABLE");
    pair(out, 2, "LAYER");
    pair(out, 70, &layers.len().to_string());
    for (layer, colour) in layers {
        pair(out, 0, "LAYER");
        pair(out, 2, layer);
        pair(out, 70, "0");
        pair(out, 62, colour);
        pair(out, 6, "CONTINUOUS");
    }
    pair(out, 0, "ENDTAB");
    pair(out, 0, "ENDSEC");
}

fn block_start(out: &mut String, name: &str) {
    pair(out, 0, "BLOCK");
    pair(out, 8, "0");
    pair(out, 2, name);
    pair(out, 70, "0");
    pair(out, 10, "0.0");
    pair(out, 20, "0.0");
    pair(out, 30, "0.0");
    pair(out, 3, name);
}

fn block_end(out: &mut String) {
    pair(out, 0, "ENDBLK");
    pair(out, 8, "0");
}

fn entities(out: &mut String, names: &[String], before: &dyn Fn(&mut String)) {
    pair(out, 0, "SECTION");
    pair(out, 2, "ENTITIES");
    before(out);
    for name in names {
        pair(out, 0, "INSERT");
        pair(out, 8, "0");
        pair(out, 2, name);
        pair(out, 10, "0.0");
        pair(out, 20, "0.0");
        pair(out, 30, "0.0");
    }
    pair(out, 0, "ENDSEC");
    pair(out, 0, "EOF");
}

fn on_named_layer(on: &str) -> &'static str {
    match on {
        "internal" => "INTCUT",
        "drill" => "DRILL",
        "grain" => "DRAW",
        _ => "CUT",
    }
}

fn on_numbered_layer(on: &str) -> &'static str {
    match on {
        "internal" => "8",
        "drill" => "13",
        "grain" => "7",
        _ => "1",
    }
}

fn sorted_marks(piece: &DrawnPiece) -> Vec<Mark> {
    let mut marks = piece.marks.clone();
    marks.sort();
    marks
}

fn sorted_lines(piece: &DrawnPiece) -> Vec<Line> {
    let mut lines = piece.internal.clone();
    lines.sort();
    lines
}

fn check_block(name: &str, line_points: &[Point], which: &'static str) -> Result<(), DxfError> {
    if line_points.len() < 3 {
        return Err(DxfError::NotClosed {
            block: name.to_owned(),
            line: which,
        });
    }
    Ok(())
}

/// The sample on named layers, the sew line on `CUT` so that a rule moves what a receiver reads there, with an identifier at each point. The deltas are in a `.rul` file beside it.
///
/// # Errors
///
/// See [`DxfError`].
pub fn write_base_plus_rules(
    pattern: &Pattern,
    paired: &[Paired],
    points: &[RulePoint],
    flattening: Flattening,
) -> Result<GradedArtifact, DxfError> {
    let mut pieces: Vec<&DrawnPiece> = pattern.pieces.iter().collect();
    pieces.sort_by(|a, b| a.name.cmp(&b.name));
    let mut out = String::new();
    header(
        &mut out,
        "AC1009",
        &[
            ("CUT", "1"),
            ("DRAW", "7"),
            ("DRILL", "6"),
            ("INTCUT", "4"),
            ("NOTCH", "3"),
            ("TEXT", "2"),
        ],
        &format!(
            "{} {} unit=millimetre writer={WRITER}",
            GradingMode::BasePlusRules.target(),
            GradingMode::BasePlusRules.token()
        ),
    );
    pair(&mut out, 0, "SECTION");
    pair(&mut out, 2, "BLOCKS");
    let mut names = Vec::with_capacity(pieces.len());
    for piece in pieces {
        let name = block_name(piece)?;
        check_block(&name, &piece.sew, "sew")?;
        check_block(&name, &piece.cut, "cut")?;
        block_start(&mut out, &name);
        polyline(
            &mut out,
            "CUT",
            &crate::canonical::from_lowest(&piece.sew),
            true,
        );
        polyline(
            &mut out,
            "DRAW",
            &crate::canonical::from_lowest(&piece.cut),
            true,
        );
        line(&mut out, "DRAW", piece.grain.from, piece.grain.to);
        for item in sorted_lines(piece) {
            polyline(&mut out, "INTCUT", &[item.from, item.to], false);
        }
        for mark in sorted_marks(piece) {
            match mark.kind {
                MarkKind::VNotch | MarkKind::SlitNotch => {
                    line(&mut out, "NOTCH", mark.at, mark.toward);
                }
                MarkKind::Drill => drill(&mut out, mark.at),
            }
        }
        for (index, value) in [
            format!("Piece Name: {}", piece.name),
            format!("Size: {}", piece.size),
            quantity(piece, paired),
            format!("Style Name: {}", pattern.style),
        ]
        .iter()
        .enumerate()
        {
            let y = piece.grain.from.y() - i64::try_from(index).unwrap_or(0) * 5_000;
            text_on(
                &mut out,
                "TEXT",
                Point::new(piece.grain.from.x(), y).unwrap_or(piece.grain.from),
                value,
            );
        }
        let mut labels = piece.labels.clone();
        labels.sort();
        for label in &labels {
            text_on(&mut out, "TEXT", label.at, &label.text);
        }
        let mut here: Vec<&RulePoint> = points
            .iter()
            .filter(|point| point.piece == piece.name)
            .collect();
        here.sort_by_key(|point| point.id);
        for point in here {
            identifier(
                &mut out,
                on_named_layer(point.on),
                point.at,
                point.id,
                false,
            );
        }
        block_end(&mut out);
        names.push(name);
    }
    pair(&mut out, 0, "ENDSEC");
    entities(&mut out, &names, &|_| {});
    Ok(GradedArtifact {
        text: out,
        record: record(
            GradingMode::BasePlusRules,
            pattern,
            flattening,
            "millimetre",
        ),
    })
}

const NUMBERED: [(&str, &str); 9] = [
    ("1", "1"),
    ("4", "3"),
    ("5", "5"),
    ("7", "7"),
    ("8", "4"),
    ("13", "6"),
    ("14", "2"),
    ("15", "2"),
    ("0", "7"),
];

fn style_text(out: &mut String, pattern: &Pattern, sample: &str, table: &str) {
    for (index, value) in [
        format!("Style Name: {}", pattern.style),
        "Creation Date:".to_owned(),
        "Creation Time:".to_owned(),
        format!("Author: StitchCAD;{WRITER};1"),
        format!("Sample Size: {sample}"),
        format!("Grade Rule Table: {table}"),
        "Units: METRIC".to_owned(),
        "ASTM/D13 Proposal 1 Version: D 6673-10".to_owned(),
    ]
    .iter()
    .enumerate()
    {
        let y = i64::try_from(index).unwrap_or(0) * 5_000;
        if let Ok(at) = Point::new(0, -y) {
            text_numbered(out, "1", at, value);
        }
    }
}

fn text_numbered(out: &mut String, layer: &str, at: Point, value: &str) {
    pair(out, 0, "TEXT");
    pair(out, 8, layer);
    pair(out, 10, &millimetres_to_two(at.x()));
    pair(out, 20, &millimetres_to_two(at.y()));
    pair(out, 30, "0.0");
    pair(out, 40, "3.000");
    pair(out, 1, value);
}

fn polyline_numbered(out: &mut String, layer: &str, points: &[Point], closed: bool) {
    pair(out, 0, "POLYLINE");
    pair(out, 8, layer);
    pair(out, 66, "1");
    pair(out, 70, if closed { "1" } else { "0" });
    for point in points {
        pair(out, 0, "VERTEX");
        pair(out, 8, layer);
        pair(out, 10, &millimetres_to_two(point.x()));
        pair(out, 20, &millimetres_to_two(point.y()));
        pair(out, 30, "0.0");
    }
    pair(out, 0, "SEQEND");
    pair(out, 8, layer);
}

fn line_numbered(out: &mut String, layer: &str, from: Point, to: Point) {
    pair(out, 0, "LINE");
    pair(out, 8, layer);
    pair(out, 10, &millimetres_to_two(from.x()));
    pair(out, 20, &millimetres_to_two(from.y()));
    pair(out, 30, "0.0");
    pair(out, 11, &millimetres_to_two(to.x()));
    pair(out, 21, &millimetres_to_two(to.y()));
    pair(out, 31, "0.0");
}

fn drill_numbered(out: &mut String, at: Point) {
    pair(out, 0, "POLYLINE");
    pair(out, 8, "13");
    pair(out, 66, "1");
    pair(out, 70, "1");
    for x in [at.x() - 1_000, at.x() + 1_000] {
        pair(out, 0, "VERTEX");
        pair(out, 8, "13");
        pair(out, 10, &millimetres_to_two(x));
        pair(out, 20, &millimetres_to_two(at.y()));
        pair(out, 30, "0.0");
        pair(out, 42, "1.000000");
    }
    pair(out, 0, "SEQEND");
    pair(out, 8, "13");
}

/// The sample on numbered layers, with an identifier at each point. **The standard has no place in this file for a delta,** so a receiver also needs the table its style text names.
///
/// # Errors
///
/// See [`DxfError`].
pub fn write_embedded_rules(
    pattern: &Pattern,
    paired: &[Paired],
    points: &[RulePoint],
    (sample, table): (&str, &str),
    flattening: Flattening,
) -> Result<GradedArtifact, DxfError> {
    let mut pieces: Vec<&DrawnPiece> = pattern.pieces.iter().collect();
    pieces.sort_by(|a, b| a.name.cmp(&b.name));
    let mut out = String::new();
    header(
        &mut out,
        "AC1012",
        &NUMBERED,
        &format!(
            "{} {} unit=millimetre writer={WRITER}",
            GradingMode::EmbeddedRules.target(),
            GradingMode::EmbeddedRules.token()
        ),
    );
    pair(&mut out, 0, "SECTION");
    pair(&mut out, 2, "BLOCKS");
    let mut names = Vec::with_capacity(pieces.len());
    for piece in pieces {
        let name = block_name(piece)?;
        check_block(&name, &piece.sew, "sew")?;
        check_block(&name, &piece.cut, "cut")?;
        block_start(&mut out, &name);
        polyline_numbered(
            &mut out,
            "1",
            &crate::canonical::from_lowest(&piece.sew),
            true,
        );
        polyline_numbered(
            &mut out,
            "14",
            &crate::canonical::from_lowest(&piece.cut),
            true,
        );
        line_numbered(&mut out, "7", piece.grain.from, piece.grain.to);
        for item in sorted_lines(piece) {
            polyline_numbered(&mut out, "8", &[item.from, item.to], false);
        }
        for mark in sorted_marks(piece) {
            match mark.kind {
                MarkKind::VNotch | MarkKind::SlitNotch => {
                    line_numbered(&mut out, "4", mark.at, mark.toward);
                }
                MarkKind::Drill => drill_numbered(&mut out, mark.at),
            }
        }
        for (index, value) in [
            format!("Piece Name: {}", piece.name),
            quantity(piece, paired),
        ]
        .iter()
        .enumerate()
        {
            let y = piece.grain.from.y() - i64::try_from(index).unwrap_or(0) * 5_000;
            text_numbered(
                &mut out,
                "1",
                Point::new(piece.grain.from.x(), y).unwrap_or(piece.grain.from),
                value,
            );
        }
        let mut labels = piece.labels.clone();
        labels.sort();
        for label in &labels {
            text_numbered(&mut out, "15", label.at, &label.text);
        }
        let mut here: Vec<&RulePoint> = points
            .iter()
            .filter(|point| point.piece == piece.name)
            .collect();
        here.sort_by_key(|point| point.id);
        for point in here {
            identifier(
                &mut out,
                on_numbered_layer(point.on),
                point.at,
                point.id,
                true,
            );
        }
        block_end(&mut out);
        names.push(name);
    }
    pair(&mut out, 0, "ENDSEC");
    let style = pattern.clone();
    let sample = sample.to_owned();
    let table = table.to_owned();
    entities(&mut out, &names, &|out| {
        style_text(out, &style, &sample, &table)
    });
    Ok(GradedArtifact {
        text: out,
        record: record(
            GradingMode::EmbeddedRules,
            pattern,
            flattening,
            "millimetre_two_places",
        ),
    })
}

/// One piece at one size in a nest: its net line, with the same number of points at every size.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NestPiece {
    /// The piece.
    pub piece: String,
    /// The net line, in the same order and number at every size.
    pub net: Vec<Point>,
    /// Where it stacks: a straight line whose start is the piece's stack point.
    pub grade_reference: Line,
    /// Its grainline.
    pub grain: Line,
    /// Its drill marks, in order.
    pub drills: Vec<Point>,
}

/// Every size's own contours, a block per piece and size, in the order of the sizes. Only the sample carries what the standard says only it needs.
///
/// # Errors
///
/// See [`DxfError`]. Two sizes of a piece differing in how many points they have is refused here, because the standard says they cannot.
pub fn write_all_contours(
    sample: &Pattern,
    paired: &[Paired],
    sizes: &[(String, Vec<NestPiece>)],
    flattening: Flattening,
) -> Result<GradedArtifact, DxfError> {
    let mut out = String::new();
    header(
        &mut out,
        "AC1012",
        &NUMBERED,
        &format!(
            "{} {} unit=millimetre writer={WRITER}",
            GradingMode::AllContours.target(),
            GradingMode::AllContours.token()
        ),
    );
    pair(&mut out, 0, "SECTION");
    pair(&mut out, 2, "BLOCKS");
    let mut names = Vec::new();
    let sample_name = sample
        .pieces
        .first()
        .map_or("", |piece| piece.size.as_str());
    for (size, pieces) in sizes {
        let mut in_order: Vec<&NestPiece> = pieces.iter().collect();
        in_order.sort_by(|a, b| a.piece.cmp(&b.piece));
        for nest in in_order {
            let Some(drawn) = sample.pieces.iter().find(|piece| piece.name == nest.piece) else {
                continue;
            };
            let name = format!("{}_{size}", nest.piece);
            let at_sample = sizes
                .iter()
                .find(|(name, _)| name == sample_name)
                .and_then(|(_, pieces)| pieces.iter().find(|piece| piece.piece == nest.piece));
            if let Some(at_sample) = at_sample {
                if at_sample.net.len() != nest.net.len() {
                    return Err(DxfError::NotClosed {
                        block: name,
                        line: "net with a different number of points from the sample",
                    });
                }
            }
            check_block(&name, &nest.net, "net")?;
            block_start(&mut out, &name);
            polyline_numbered(&mut out, "1", &nest.net, true);
            line_numbered(
                &mut out,
                "5",
                nest.grade_reference.from,
                nest.grade_reference.to,
            );
            line_numbered(&mut out, "7", nest.grain.from, nest.grain.to);
            for at in &nest.drills {
                drill_numbered(&mut out, *at);
            }
            text_numbered(
                &mut out,
                "1",
                nest.grain.from,
                &format!("Piece Name: {}", nest.piece),
            );
            let base = Point::new(nest.grain.from.x(), nest.grain.from.y() - 5_000)
                .unwrap_or(nest.grain.from);
            text_numbered(&mut out, "1", base, &format!("Size Name: {size}"));
            if size == sample_name {
                let lower = Point::new(nest.grain.from.x(), nest.grain.from.y() - 10_000)
                    .unwrap_or(nest.grain.from);
                text_numbered(&mut out, "1", lower, &quantity(drawn, paired));
                polyline_numbered(
                    &mut out,
                    "14",
                    &crate::canonical::from_lowest(&drawn.cut),
                    true,
                );
                for item in sorted_lines(drawn) {
                    polyline_numbered(&mut out, "8", &[item.from, item.to], false);
                }
                for mark in sorted_marks(drawn) {
                    if matches!(mark.kind, MarkKind::VNotch | MarkKind::SlitNotch) {
                        line_numbered(&mut out, "4", mark.at, mark.toward);
                    }
                }
                let mut labels = drawn.labels.clone();
                labels.sort();
                for label in &labels {
                    text_numbered(&mut out, "15", label.at, &label.text);
                }
            }
            block_end(&mut out);
            names.push(name);
        }
    }
    pair(&mut out, 0, "ENDSEC");
    let style = sample.clone();
    let sample_size = sample_name.to_owned();
    entities(&mut out, &names, &|out| {
        style_text(out, &style, &sample_size, "")
    });
    Ok(GradedArtifact {
        text: out,
        record: record(
            GradingMode::AllContours,
            sample,
            flattening,
            "millimetre_two_places",
        ),
    })
}
