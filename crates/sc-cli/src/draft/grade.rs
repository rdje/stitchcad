//! Path 2: a sample size moved by a table of rules. The recipe is not consulted. Each point goes where its rule says, the geometry between them is rebuilt, and allowances, notches and seams are worked out again by the same drawing as anything else.
use super::{
    boundary, drawing, length, point, ratio, required, seam_side, whole, Fields, Lengths, Tables,
    WALK_TOLERANCE,
};
use sc_artifacts::{stretch, stretch_length, Pattern};
use sc_geometry::Point;
use sc_grading::{Attributes, Convention, GradeTable, Provenance, Rule, ZeroState};
use sc_store::Value;
use std::collections::BTreeMap;

type Visitor<'a> = &'a mut dyn FnMut(String, &mut Value) -> Result<(), String>;

fn object_of(value: &mut Value) -> Option<&mut Fields> {
    match value {
        Value::Object(fields) => Some(fields),
        _ => None,
    }
}

fn list_of(value: &mut Value) -> Option<&mut Vec<Value>> {
    match value {
        Value::List(list) => Some(list),
        _ => None,
    }
}

fn is_point(value: &Value) -> bool {
    matches!(value.as_list(), Some([x, y]) if x.as_text().is_some() && y.as_text().is_some())
}

// Every point of one piece a rule can move, named by what it is rather than by where it is in a list, except for those a list is all that names.
fn visit_piece(piece: &str, fields: &mut Fields, visit: Visitor<'_>) -> Result<(), String> {
    if let Some(labels) = fields.get_mut("labels").and_then(list_of) {
        for (at, label) in labels.iter_mut().enumerate() {
            if let Some(place) = object_of(label).and_then(|label| label.get_mut("at")) {
                visit(format!("{piece}/label/{at}/at"), place)?;
            }
        }
    }
    if fields.contains_key("mirror_of") {
        return Ok(());
    }
    let mut last = None;
    if let Some(edges) = fields.get_mut("edges").and_then(list_of) {
        for edge in edges.iter_mut() {
            let edge = object_of(edge).ok_or("an edge must be an object")?;
            let name = edge
                .get("name")
                .and_then(Value::as_text)
                .ok_or_else(|| {
                    format!("{piece} has an edge with no name, and a rule has nothing to hold")
                })?
                .to_owned();
            if let Some(to) = edge.get_mut("to") {
                visit(format!("{piece}/{name}/to"), to)?;
            }
            if let Some(through) = edge.get_mut("through") {
                visit(format!("{piece}/{name}/through"), through)?;
            }
            last = Some(name);
        }
    }
    if let (Some(last), Some(start)) = (last, fields.get_mut("start")) {
        visit(format!("{piece}/{last}/to"), start)?;
    }
    if let Some(grains) = fields.get_mut("grain").and_then(list_of) {
        for grain in grains.iter_mut() {
            if let Some([from, to]) = list_of(grain).map(Vec::as_mut_slice) {
                visit(format!("{piece}/grain/from"), from)?;
                visit(format!("{piece}/grain/to"), to)?;
            }
        }
    }
    if let Some(lines) = fields.get_mut("internal").and_then(list_of) {
        for (at, line) in lines.iter_mut().enumerate() {
            match line {
                Value::List(ends) => {
                    if let [from, to] = ends.as_mut_slice() {
                        visit(format!("{piece}/internal/{at}/from"), from)?;
                        visit(format!("{piece}/internal/{at}/to"), to)?;
                    }
                }
                Value::Object(arc) => {
                    for end in ["from", "through", "to"] {
                        if let Some(place) = arc.get_mut(end) {
                            visit(format!("{piece}/internal/{at}/{end}"), place)?;
                        }
                    }
                }
                _ => {}
            }
        }
    }
    for kind in ["darts", "tucks", "pleats", "gathers", "slash_spreads"] {
        if let Some(items) = fields.get_mut(kind).and_then(list_of) {
            for item in items.iter_mut() {
                let item = object_of(item).ok_or("a closure must be an object")?;
                let name = item
                    .get("name")
                    .and_then(Value::as_text)
                    .unwrap_or("unnamed")
                    .to_owned();
                for (field, value) in item.iter_mut() {
                    if is_point(value) {
                        visit(format!("{piece}/{kind}/{name}/{field}"), value)?;
                    }
                }
            }
        }
    }
    if let Some(notches) = fields.get_mut("notches").and_then(list_of) {
        for (at, notch) in notches.iter_mut().enumerate() {
            if let Value::List(ends) = notch {
                if let [place, toward] = ends.as_mut_slice() {
                    visit(format!("{piece}/notch/{at}/at"), place)?;
                    visit(format!("{piece}/notch/{at}/toward"), toward)?;
                }
            }
        }
    }
    Ok(())
}

// Every point of a whole draft a rule can move.
fn visit_all(description: &mut Fields, visit: Visitor<'_>) -> Result<(), String> {
    if let Some(pieces) = description.get_mut("pieces").and_then(list_of) {
        for piece in pieces.iter_mut() {
            let piece = object_of(piece).ok_or("a piece must be an object")?;
            let name = piece
                .get("name")
                .and_then(Value::as_text)
                .ok_or("a piece has a name")?
                .to_owned();
            visit_piece(&name, piece, visit)?;
        }
    }
    if let Some(seams) = description.get_mut("seams").and_then(list_of) {
        for seam in seams.iter_mut() {
            let seam = object_of(seam).ok_or("a seam must be an object")?;
            let name = seam
                .get("name")
                .and_then(Value::as_text)
                .unwrap_or("unnamed")
                .to_owned();
            for side_name in ["a", "b"] {
                let Some(side) = seam.get_mut(side_name).and_then(object_of) else {
                    continue;
                };
                let piece = side
                    .get("piece")
                    .and_then(Value::as_text)
                    .unwrap_or("unnamed")
                    .to_owned();
                for end in ["from", "to"] {
                    if let Some(place) = side.get_mut(end) {
                        if is_point(place) {
                            visit(format!("{piece}/seam/{name}/{side_name}/{end}"), place)?;
                        }
                    }
                }
            }
        }
    }
    if let Some(buttons) = description.get_mut("buttons").and_then(list_of) {
        for pair in buttons.iter_mut() {
            let pair = object_of(pair).ok_or("a button must be an object")?;
            let name = pair
                .get("name")
                .and_then(Value::as_text)
                .unwrap_or("unnamed")
                .to_owned();
            for (part, ends) in [("button", &["at"][..]), ("hole", &["at", "toward"][..])] {
                let Some(side) = pair.get_mut(part).and_then(object_of) else {
                    continue;
                };
                let piece = side
                    .get("piece")
                    .and_then(Value::as_text)
                    .unwrap_or("unnamed")
                    .to_owned();
                for end in ends {
                    if let Some(place) = side.get_mut(*end) {
                        visit(format!("{piece}/{part}/{name}/{end}"), place)?;
                    }
                }
            }
        }
    }
    Ok(())
}

/// Where every point a rule can move is, in this description at these lengths.
///
/// # Errors
///
/// A length has no value, an edge has no name, or a boundary does not close.
pub fn points(lengths: &Lengths, description: &Fields) -> Result<BTreeMap<String, Point>, String> {
    let mut found: BTreeMap<String, Point> = BTreeMap::new();
    let mut copy = description.clone();
    visit_all(&mut copy, &mut |key, value| {
        let at = point(lengths, value)?;
        match found.insert(key.clone(), at) {
            Some(before) if before != at => Err(format!(
                "{key} is in two places, ({}, {}) and ({}, {})",
                before.x(),
                before.y(),
                at.x(),
                at.y()
            )),
            _ => Ok(()),
        }
    })?;
    Ok(found)
}

fn piece_of(key: &str) -> &str {
    key.split('/').next().unwrap_or(key)
}

/// A table worked out from a drawing at each size. Each piece is held to its stack point, so that a delta is the piece's change and not its drift.
///
/// # Errors
///
/// The draft does not say how it stacks, the sizes do not have the same points, or the table would be refused.
pub fn derive_table(
    description: &Fields,
    at_sizes: &[(String, Lengths)],
    sample: &str,
) -> Result<GradeTable, String> {
    let grading = description
        .get("grading")
        .and_then(Value::as_object)
        .ok_or("this draft does not say how it grades: name its stack points, in grading")?;
    let name = grading
        .get("table")
        .or_else(|| description.get("style"))
        .and_then(Value::as_text)
        .ok_or("a table has a name")?
        .to_owned();
    let stack: BTreeMap<String, String> = grading
        .get("stack")
        .and_then(Value::as_object)
        .ok_or("grading names a stack point for each piece, in stack")?
        .iter()
        .filter_map(|(piece, point)| {
            point
                .as_text()
                .map(|point| (piece.clone(), format!("{piece}/{point}")))
        })
        .collect();
    // A mirror has no boundary of its own, so what is on it moves with the piece it mirrors.
    let mut anchors = stack.clone();
    for item in required(description, "pieces")?
        .as_list()
        .ok_or("pieces must be a list")?
        .iter()
        .filter_map(Value::as_object)
    {
        if let (Some(name), Some(source)) = (
            item.get("name").and_then(Value::as_text),
            item.get("mirror_of").and_then(Value::as_text),
        ) {
            if let Some(anchor) = stack.get(source) {
                anchors.insert(name.to_owned(), anchor.clone());
            }
        }
    }
    let by_size: Vec<BTreeMap<String, Point>> = at_sizes
        .iter()
        .map(|(_, lengths)| points(lengths, description))
        .collect::<Result<_, _>>()?;
    let sizes: Vec<String> = at_sizes.iter().map(|(size, _)| size.clone()).collect();
    let sample_at = sizes
        .iter()
        .position(|size| size == sample)
        .ok_or_else(|| format!("the sample size {sample} is not among the sizes"))?;
    let at_sample = by_size.get(sample_at).ok_or("no sample")?;
    let moved = |key: &str, at: usize| -> Result<(i64, i64), String> {
        let (here, there) = (
            by_size.get(at).and_then(|points| points.get(key)),
            at_sample.get(key),
        );
        match (here, there) {
            (Some(here), Some(there)) => Ok((here.x() - there.x(), here.y() - there.y())),
            _ => Err(format!(
                "the size {} has no point {key}",
                sizes.get(at).map_or("", String::as_str)
            )),
        }
    };
    let mut rules = Vec::with_capacity(at_sample.len());
    for (ordinal, key) in at_sample.keys().enumerate() {
        let piece = piece_of(key);
        let anchor = anchors.get(piece).ok_or_else(|| {
            format!("grading names no stack point for {piece}, so it would drift")
        })?;
        let mut deltas = Vec::with_capacity(sizes.len());
        for at in 0..sizes.len() {
            let (here, drift) = (moved(key, at)?, moved(anchor, at)?);
            deltas.push((here.0 - drift.0, here.1 - drift.1));
        }
        rules.push(Rule {
            point: key.clone(),
            id: i64::try_from(ordinal).map_err(|error| error.to_string())? + 1,
            deltas,
            zero: ZeroState::Declared,
        });
    }
    let attributes = Attributes {
        stack: stack.values().cloned().collect(),
        fixed: grading
            .get("fixed")
            .and_then(Value::as_list)
            .map(<[Value]>::to_vec)
            .unwrap_or_default(),
        smoothing: grading
            .get("smoothing")
            .and_then(Value::as_list)
            .unwrap_or(&[])
            .iter()
            .filter_map(Value::as_text)
            .map(str::to_owned)
            .collect(),
    };
    GradeTable::new(
        name,
        (sample.to_owned(), sizes),
        Convention::Cumulative,
        Provenance::Regenerations,
        rules,
        attributes,
    )
    .map_err(|error| error.to_string())
}

/// A contour a table says must not change, that did.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Conflict {
    /// The contour.
    pub contour: String,
    /// The size.
    pub size: String,
    /// Its length at the sample size.
    pub at_sample: i64,
    /// Its length here.
    pub here: i64,
}

impl Conflict {
    /// The conflict as a value.
    #[must_use]
    pub fn to_value(&self) -> Value {
        let mut fields = Fields::new();
        fields.insert("contour".to_owned(), Value::Text(self.contour.clone()));
        fields.insert("size".to_owned(), Value::Text(self.size.clone()));
        fields.insert("at_sample".to_owned(), Value::Int(self.at_sample));
        fields.insert("here".to_owned(), Value::Int(self.here));
        Value::Object(fields)
    }
}

/// A size rebuilt from a table.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rebuilt {
    /// The drawing.
    pub pattern: Pattern,
    /// Points of this design the table does not mention, each moved in proportion to its neighbours or left where it was.
    pub unmentioned: Vec<String>,
    /// Rules for points this design does not have, which a table from somewhere else may well hold.
    pub unmatched: Vec<String>,
    /// What was only approximated, or could not be rebuilt as it was.
    pub notes: Vec<String>,
    /// Contours the table says must not change, and did.
    pub conflicts: Vec<Conflict>,
}

// Where a place on an edge is, as a fraction, so that it keeps its place on the edge rather than its distance along it.
fn as_fraction(
    tables: &Tables,
    lengths: &Lengths,
    piece: &str,
    place: &mut Fields,
    what: &str,
) -> Result<(), String> {
    if !place.contains_key("along") && !place.contains_key("before_end") {
        return Ok(());
    }
    let (edges, _) = tables
        .get(piece)
        .ok_or_else(|| format!("no piece is named {piece}"))?;
    let held = edges.held(lengths, place, what)?;
    place.remove("along");
    place.remove("before_end");
    place.insert(
        "at".to_owned(),
        Value::Text(held.param.as_rational().to_string()),
    );
    Ok(())
}

// A boundary corner a table does not mention moves in proportion to the nearest corners it does, along the boundary, and with none on the piece it stays where it is.
fn fill_corners(corners: &[(String, Point)], deltas: &mut BTreeMap<String, (i64, i64)>) {
    let ruled: Vec<usize> = corners
        .iter()
        .enumerate()
        .filter(|(_, (key, _))| deltas.contains_key(key))
        .map(|(at, _)| at)
        .collect();
    let count = corners.len();
    let chord = |a: usize, b: usize| -> f64 {
        match (corners.get(a), corners.get(b)) {
            (Some((_, a)), Some((_, b))) => {
                sc_units::pure::hypot(ratio(a.x() - b.x()), ratio(a.y() - b.y()))
            }
            _ => 0.0,
        }
    };
    for at in 0..count {
        let Some((key, _)) = corners.get(at) else {
            continue;
        };
        if deltas.contains_key(key) {
            continue;
        }
        if ruled.is_empty() {
            deltas.insert(key.clone(), (0, 0));
            continue;
        }
        let mut before = at;
        let mut behind = 0.0;
        while !ruled.contains(&before) {
            let next = (before + count - 1) % count;
            behind += chord(before, next);
            before = next;
        }
        let mut after = at;
        let mut ahead = 0.0;
        while !ruled.contains(&after) {
            let next = (after + 1) % count;
            ahead += chord(after, next);
            after = next;
        }
        let share = if behind + ahead > 0.0 {
            behind / (behind + ahead)
        } else {
            0.0
        };
        let delta_of = |index: usize, deltas: &BTreeMap<String, (i64, i64)>| {
            corners
                .get(index)
                .and_then(|(key, _)| deltas.get(key))
                .copied()
                .unwrap_or((0, 0))
        };
        let (a, b) = (delta_of(before, deltas), delta_of(after, deltas));
        let moved = (
            whole(ratio(a.0) + (ratio(b.0) - ratio(a.0)) * share),
            whole(ratio(a.1) + (ratio(b.1) - ratio(a.1)) * share),
        );
        deltas.insert(key.clone(), moved);
    }
}

// The length of a stretch declared as a seam's side is, on the line this description draws.
fn contour_length(lengths: &Lengths, description: &Fields, side: &Value) -> Result<i64, String> {
    let fields = side
        .as_object()
        .ok_or("a contour is a piece, from and to")?;
    let piece = required(fields, "piece")?
        .as_text()
        .ok_or("a contour names its piece")?;
    let listed = required(description, "pieces")?
        .as_list()
        .ok_or("pieces must be a list")?;
    let drawn = listed
        .iter()
        .filter_map(Value::as_object)
        .find(|item| item.get("name").and_then(Value::as_text) == Some(piece))
        .ok_or_else(|| format!("a contour is on {piece}, and no piece has that name"))?;
    let (edges, _) = boundary(lengths, drawn, piece, &[])?;
    let line = sc_geometry::Outline::new(edges.curves.clone())
        .map_err(|error| error.to_string())?
        .polyline(super::TOLERANCE);
    let mut tables: Tables = BTreeMap::new();
    tables.insert(piece.to_owned(), (edges, false));
    let mut book = super::Book::default();
    let found = seam_side(
        lengths,
        &tables,
        side,
        &BTreeMap::new(),
        "contour",
        &mut book,
    )?
    .ok_or("a contour's end has nowhere to go")?;
    let walked = stretch(&line, found.from, found.to).ok_or("a contour's end is off the line")?;
    Ok(whole(stretch_length(&walked)))
}

/// A size, from the sample's lengths and a table. The recipe is not consulted.
///
/// # Errors
///
/// The draft holds what a rule cannot follow yet, the table has no such size, or the result cannot be drawn.
#[allow(clippy::too_many_lines)]
pub fn reconstruct(
    lengths: &Lengths,
    description: &Fields,
    table: &GradeTable,
    size: &str,
    (design, revision): (&str, &str),
) -> Result<Rebuilt, String> {
    if description
        .get("trues")
        .and_then(Value::as_list)
        .is_some_and(|listed| !listed.is_empty())
    {
        return Err(
            "this draft trues a seam, and a true is worked out from the drawing, so a rule cannot follow it yet"
                .to_owned(),
        );
    }
    let base = points(lengths, description)?;
    let mut deltas: BTreeMap<String, (i64, i64)> = BTreeMap::new();
    for key in base.keys() {
        if let Some(rule) = table.rule(key) {
            deltas.insert(
                key.clone(),
                table
                    .delta_at(rule, size)
                    .map_err(|error| error.to_string())?,
            );
        }
    }
    let unmentioned: Vec<String> = base
        .keys()
        .filter(|key| !deltas.contains_key(*key))
        .cloned()
        .collect();
    let unmatched: Vec<String> = table
        .rules()
        .iter()
        .filter(|rule| !base.contains_key(&rule.point))
        .map(|rule| rule.point.clone())
        .collect();
    let mut notes = Vec::new();
    let listed = required(description, "pieces")?
        .as_list()
        .ok_or("pieces must be a list")?;
    // The boundary of each piece at the sample, and what it holds.
    let mut tables: Tables = BTreeMap::new();
    for item in listed.iter().filter_map(Value::as_object) {
        let name = required(item, "name")?
            .as_text()
            .ok_or("a piece has a name")?
            .to_owned();
        if let Some(source) = item.get("mirror_of").and_then(Value::as_text) {
            if let Some((edges, mirrored)) = tables.get(source) {
                let copy = (edges.clone(), !*mirrored);
                tables.insert(name, copy);
            }
            continue;
        }
        let (edges, _) = boundary(lengths, item, &name, &[])?;
        let corners: Vec<(String, Point)> = item
            .get("edges")
            .and_then(Value::as_list)
            .unwrap_or(&[])
            .iter()
            .filter_map(Value::as_object)
            .filter_map(|edge| {
                let edge_name = edge.get("name")?.as_text()?;
                let key = format!("{name}/{edge_name}/to");
                base.get(&key).map(|at| (key.clone(), *at))
            })
            .collect();
        fill_corners(&corners, &mut deltas);
        tables.insert(name, (edges, false));
    }
    let mut extended = lengths.clone();
    let mut described = description.clone();
    for name in ["fits", "sizes", "grading"] {
        described.remove(name);
    }
    described.insert("size".to_owned(), Value::Text(size.to_owned()));
    // Each cubic keeps its parameterization, and its control points move by the interpolation of its two ends.
    if let Some(pieces) = described.get_mut("pieces").and_then(list_of) {
        for piece in pieces.iter_mut() {
            let piece = object_of(piece).ok_or("a piece must be an object")?;
            let name = piece
                .get("name")
                .and_then(Value::as_text)
                .ok_or("a piece has a name")?
                .to_owned();
            let Some(edges) = piece.get_mut("edges").and_then(list_of) else {
                continue;
            };
            let names: Vec<String> = edges
                .iter()
                .filter_map(|edge| edge.as_object()?.get("name")?.as_text().map(str::to_owned))
                .collect();
            let count = names.len();
            for (at, edge) in edges.iter_mut().enumerate() {
                let edge = object_of(edge).ok_or("an edge must be an object")?;
                let edge_name = names.get(at).cloned().unwrap_or_default();
                let from_key = names
                    .get((at + count - 1) % count.max(1))
                    .map(|before| format!("{name}/{before}/to"))
                    .unwrap_or_default();
                let to_key = format!("{name}/{edge_name}/to");
                let (from_delta, to_delta) = (
                    deltas.get(&from_key).copied().unwrap_or((0, 0)),
                    deltas.get(&to_key).copied().unwrap_or((0, 0)),
                );
                for (field, near, far) in [("first", 2.0, 1.0), ("second", 1.0, 2.0)] {
                    let Some(control) = edge.get_mut(field) else {
                        continue;
                    };
                    let was = point(lengths, control)?;
                    let share = |a: i64, b: i64| whole((near * ratio(a) + far * ratio(b)) / 3.0);
                    let (x_name, y_name) = (
                        format!("@{name}/{edge_name}/{field}/x"),
                        format!("@{name}/{edge_name}/{field}/y"),
                    );
                    extended.insert(x_name.clone(), was.x() + share(from_delta.0, to_delta.0));
                    extended.insert(y_name.clone(), was.y() + share(from_delta.1, to_delta.1));
                    *control = Value::List(vec![Value::Text(x_name), Value::Text(y_name)]);
                    if field == "first" {
                        notes.push(format!(
                            "{name}/{edge_name}: a cubic's control points moved by the interpolation of its ends"
                        ));
                    }
                }
                if let Some(through) = edge.get("through") {
                    let middle = point(lengths, through)?;
                    let key = format!("{name}/{edge_name}/through");
                    let delta = deltas.get(&key).copied().unwrap_or((0, 0));
                    let (from, to) = (
                        base.get(&from_key).copied().unwrap_or(middle),
                        base.get(&to_key).copied().unwrap_or(middle),
                    );
                    let at_point = |at: Point, delta: (i64, i64)| {
                        (ratio(at.x() + delta.0), ratio(at.y() + delta.1))
                    };
                    let (ax, ay) = at_point(from, from_delta);
                    let (bx, by) = at_point(middle, delta);
                    let (cx, cy) = at_point(to, to_delta);
                    // Twice the area of the triangle over the chord is the sagitta's measure: with nothing there, no arc is consistent with the three.
                    let chord = sc_units::pure::hypot(cx - ax, cy - ay);
                    let height =
                        ((bx - ax) * (cy - ay) - (by - ay) * (cx - ax)).abs() / chord.max(1.0);
                    if height < 1.0 {
                        notes.push(format!(
                            "{name}/{edge_name}: no arc goes through its three moved points, so it is drawn straight"
                        ));
                    }
                }
            }
        }
    }
    // Everywhere else a point goes where its rule says. An unmentioned point on a straight edge stays on it, in proportion, and any other stays where it is.
    let mut counter = 0_u32;
    visit_all(&mut described, &mut |key, value| {
        let was = point(lengths, value)?;
        let delta = match deltas.get(&key) {
            Some(delta) => *delta,
            None => between_corners(was, &key, &base, &deltas),
        };
        counter += 1;
        let (x_name, y_name) = (format!("@{counter}/x"), format!("@{counter}/y"));
        extended.insert(x_name.clone(), was.x() + delta.0);
        extended.insert(y_name.clone(), was.y() + delta.1);
        *value = Value::List(vec![Value::Text(x_name), Value::Text(y_name)]);
        Ok(())
    })?;
    // A notch, a grade point, a seam's end or a pocket keeps its place on its edge rather than its distance along it.
    if let Some(pieces) = described.get_mut("pieces").and_then(list_of) {
        for piece in pieces.iter_mut() {
            let piece = object_of(piece).ok_or("a piece must be an object")?;
            let name = piece
                .get("name")
                .and_then(Value::as_text)
                .ok_or("a piece has a name")?
                .to_owned();
            for carriers in ["notches", "grade_points"] {
                if let Some(items) = piece.get_mut(carriers).and_then(list_of) {
                    for item in items.iter_mut() {
                        if let Some(item) = object_of(item) {
                            as_fraction(&tables, lengths, &name, item, carriers)?;
                        }
                    }
                }
            }
        }
    }
    if let Some(seams) = described.get_mut("seams").and_then(list_of) {
        for seam in seams.iter_mut() {
            let seam = object_of(seam).ok_or("a seam must be an object")?;
            for side_name in ["a", "b"] {
                let Some(side) = seam.get_mut(side_name).and_then(object_of) else {
                    continue;
                };
                let piece = side
                    .get("piece")
                    .and_then(Value::as_text)
                    .unwrap_or_default()
                    .to_owned();
                for end in ["from", "to"] {
                    if let Some(place) = side.get_mut(end).and_then(object_of) {
                        as_fraction(&tables, lengths, &piece, place, "a seam's end")?;
                    }
                }
            }
        }
    }
    if let Some(pockets) = described.get_mut("pockets").and_then(list_of) {
        for pocket in pockets.iter_mut() {
            let pocket = object_of(pocket).ok_or("a pocket must be an object")?;
            let piece = pocket
                .get("piece")
                .and_then(Value::as_text)
                .unwrap_or_default()
                .to_owned();
            if pocket.contains_key("before_end") {
                pocket.insert("runs".to_owned(), Value::Text("back".to_owned()));
            }
            as_fraction(&tables, lengths, &piece, pocket, "a pocket")?;
        }
    }
    // A balance notch keeps its share of the seam, and it is told that share rather than a distance.
    let shares = balance_shares(lengths, description)?;
    if let Some(items) = described.get_mut("balance_notches").and_then(list_of) {
        for (item, share) in items.iter_mut().zip(shares) {
            if let Some(item) = object_of(item) {
                item.remove("at");
                item.insert("share".to_owned(), Value::Text(share));
            }
        }
    }
    let pattern = drawing(&extended, &described, design, revision)?.pattern;
    let mut conflicts = Vec::new();
    for contour in &table.attributes().fixed {
        let name = contour
            .as_object()
            .and_then(|item| item.get("name"))
            .and_then(Value::as_text)
            .unwrap_or("unnamed")
            .to_owned();
        let (at_sample, here) = (
            contour_length(lengths, description, contour)?,
            contour_length(
                &extended,
                &described,
                &as_rewritten(contour, &tables, lengths)?,
            )?,
        );
        if (at_sample - here).abs() > WALK_TOLERANCE {
            conflicts.push(Conflict {
                contour: name,
                size: size.to_owned(),
                at_sample,
                here,
            });
        }
    }
    notes.sort();
    notes.dedup();
    Ok(Rebuilt {
        pattern,
        unmentioned,
        unmatched,
        notes,
        conflicts,
    })
}

// A contour with its ends held as fractions, as everything else is once a rule has moved them.
fn as_rewritten(contour: &Value, tables: &Tables, lengths: &Lengths) -> Result<Value, String> {
    let mut copy = contour.clone();
    let side = object_of(&mut copy).ok_or("a contour is a piece, from and to")?;
    let piece = side
        .get("piece")
        .and_then(Value::as_text)
        .unwrap_or_default()
        .to_owned();
    for end in ["from", "to"] {
        if let Some(place) = side.get_mut(end).and_then(object_of) {
            as_fraction(tables, lengths, &piece, place, "a contour")?;
        }
    }
    Ok(copy)
}

fn between_corners(
    was: Point,
    key: &str,
    base: &BTreeMap<String, Point>,
    deltas: &BTreeMap<String, (i64, i64)>,
) -> (i64, i64) {
    let piece = piece_of(key);
    let prefix = format!("{piece}/");
    // A straight run between any two corners of the piece that this point lies on, within T1.
    let corners: Vec<(&String, &Point)> = base
        .iter()
        .filter(|(other, _)| other.starts_with(&prefix) && other.ends_with("/to"))
        .collect();
    for (a_key, a) in &corners {
        for (b_key, b) in &corners {
            if a_key >= b_key {
                continue;
            }
            let (dx, dy) = (ratio(b.x() - a.x()), ratio(b.y() - a.y()));
            let reach = dx * dx + dy * dy;
            if reach == 0.0 {
                continue;
            }
            let t = (ratio(was.x() - a.x()) * dx + ratio(was.y() - a.y()) * dy) / reach;
            if !(0.0..=1.0).contains(&t) {
                continue;
            }
            let (px, py) = (ratio(a.x()) + t * dx, ratio(a.y()) + t * dy);
            if sc_units::pure::hypot(ratio(was.x()) - px, ratio(was.y()) - py) <= 1.5 {
                let of = |key: &String| deltas.get(key).copied().unwrap_or((0, 0));
                let (from, to) = (of(a_key), of(b_key));
                return (
                    whole(ratio(from.0) + t * ratio(to.0 - from.0)),
                    whole(ratio(from.1) + t * ratio(to.1 - from.1)),
                );
            }
        }
    }
    (0, 0)
}

fn balance_shares(lengths: &Lengths, description: &Fields) -> Result<Vec<String>, String> {
    let listed = description
        .get("balance_notches")
        .and_then(Value::as_list)
        .unwrap_or(&[]);
    if listed.is_empty() {
        return Ok(Vec::new());
    }
    let mut plain = description.clone();
    plain.remove("balance_notches");
    let pattern = super::draw(lengths, &plain, "", "", &BTreeMap::new())?.pattern;
    let mut shares = Vec::with_capacity(listed.len());
    for item in listed.iter().filter_map(Value::as_object) {
        let name = required(item, "seam")?
            .as_text()
            .ok_or("a seam's name is text")?;
        let at = length(
            lengths,
            required(item, "at")?.as_text().ok_or("at names a length")?,
        )?;
        let seam = pattern
            .seams
            .iter()
            .find(|seam| seam.name == name)
            .ok_or_else(|| format!("no seam is named {name}"))?;
        let piece = pattern
            .pieces
            .iter()
            .find(|piece| piece.name == seam.a.piece)
            .ok_or_else(|| format!("no piece is named {}", seam.a.piece))?;
        let walked = stretch(&piece.sew, seam.a.from, seam.a.to)
            .ok_or_else(|| format!("the seam {name} has an end off the line"))?;
        shares.push(format!("{at}/{}", whole(stretch_length(&walked))));
    }
    Ok(shares)
}
