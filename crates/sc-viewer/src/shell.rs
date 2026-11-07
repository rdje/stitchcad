//! The dev shell's logic: what a viewer and editor does, with no drawing in it, so it runs and is tested without a browser.
//!
//! Every edit is a step of the same grammar a session file uses, handed to the same command bus, and recorded. The recording
//! replays through the command-line program to the same result, which is the proof that this has no other way to change a Design.
//! Looking around changes nothing and records nothing. A page only forwards events to this and draws the lines it returns.
use sc_artifacts::{DrawnPiece, Line, Pattern};
use sc_cli::{Failure, Running};
use sc_store::{parse, sha256_hex, FilePath, MemoryRepository, Repository, RepositoryError, Value};
use sc_viewport::{pick, EdgeRef, Piece, Point, View};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

/// How the shell reaches each step a session can take. The book prints this table, and a test holds the two together.
pub const COVERAGE: [(&str, &str); 21] = [
    (
        "evaluate",
        "On opening and after every edit, for the values panel",
    ),
    (
        "preview",
        "Not offered: an edit here is applied and can be undone",
    ),
    (
        "closures",
        "Not offered yet: the viewer shows a dart's legs and not what closing it does",
    ),
    ("walk", "Not offered yet: the viewer draws no seams"),
    (
        "plant_balance_notch",
        "Not offered yet: the viewer draws no seams",
    ),
    ("true_seam", "Not offered yet: the viewer draws no seams"),
    (
        "edit_edge",
        "Not offered yet: the viewer does not name a piece's edges",
    ),
    ("regenerate", "Not offered yet: the viewer draws one size"),
    ("grade", "Not offered yet: the viewer draws one size"),
    (
        "references",
        "Not offered yet: the viewer does not name a piece's edges",
    ),
    (
        "repair_reference",
        "Not offered yet: the viewer does not name a piece's edges",
    ),
    ("change", "A measurement field"),
    ("undo", "The Undo button"),
    ("redo", "The Redo button"),
    (
        "solve",
        "Not offered: nothing in the shell states a constraint yet",
    ),
    (
        "render",
        "Not offered: it draws the schematic that predates drafting",
    ),
    ("draft", "On opening and after every edit"),
    ("export_pattern", "The export buttons"),
    (
        "export",
        "Not offered: it writes the schematic that predates drafting",
    ),
    ("save", "The Save button, and after each export"),
    ("reopen", "The Reopen button"),
];

/// Fixed-point pixels: a coordinate crosses to the page as a whole number of these per pixel, because a session's text has no fractions.
pub const PIXEL_UNITS: i64 = 256;

// The zooms a wheel steps through, relative to 4 pixels per millimetre. Each is exact, so zooming back returns exactly.
const LADDER: [(i64, i64); 27] = [
    (1, 100),
    (1, 64),
    (1, 50),
    (1, 40),
    (1, 32),
    (1, 25),
    (1, 20),
    (1, 16),
    (1, 12),
    (1, 10),
    (1, 8),
    (1, 6),
    (1, 5),
    (1, 4),
    (1, 3),
    (1, 2),
    (2, 3),
    (1, 1),
    (3, 2),
    (2, 1),
    (3, 1),
    (4, 1),
    (5, 1),
    (8, 1),
    (10, 1),
    (20, 1),
    (40, 1),
];
const PIXELS_PER_MILLIMETRE: i64 = 4;
const MICROMETRES_PER_MILLIMETRE: i64 = 1000;
// Pieces on screen sit in rows, in name order. This is arrangement for looking, not a marker, and nothing is saved.
const ROW_WIDTH: i64 = 1_200_000;
const PIECE_GAP: i64 = 20_000;
const FIT_MARGIN: i64 = 16;
// How near a click must be, in pixels.
const PICK_REACH: i64 = 8;

// One handle to the project, shared between the store inside the session and the shell that reads an export back.
#[derive(Clone, Debug, Default)]
struct Shared(Rc<RefCell<MemoryRepository>>);

impl Repository for Shared {
    fn read(&self, path: &FilePath) -> Result<Option<Vec<u8>>, RepositoryError> {
        self.0.borrow().read(path)
    }
    fn write(&mut self, path: &FilePath, bytes: &[u8]) -> Result<(), RepositoryError> {
        self.0.borrow_mut().write(path, bytes)
    }
    fn rename(&mut self, from: &FilePath, to: &FilePath) -> Result<(), RepositoryError> {
        self.0.borrow_mut().rename(from, to)
    }
    fn remove(&mut self, path: &FilePath) -> Result<(), RepositoryError> {
        self.0.borrow_mut().remove(path)
    }
    fn list(&self) -> Result<Vec<FilePath>, RepositoryError> {
        self.0.borrow().list()
    }
}

#[derive(Clone, Debug)]
struct Drawn {
    kind: &'static str,
    points: Vec<Point>,
}

#[derive(Clone, Debug, Default)]
struct Scene {
    names: Vec<String>,
    pieces: Vec<Piece>,
    lines: Vec<Drawn>,
    labels: Vec<(Point, String)>,
    width: i64,
    height: i64,
}

fn extent(points: &[sc_geometry::Point]) -> (i64, i64, i64, i64) {
    let (mut low_x, mut low_y, mut high_x, mut high_y) = (i64::MAX, i64::MAX, i64::MIN, i64::MIN);
    for at in points {
        low_x = low_x.min(at.x());
        low_y = low_y.min(at.y());
        high_x = high_x.max(at.x());
        high_y = high_y.max(at.y());
    }
    (low_x, low_y, high_x, high_y)
}

// A pattern's y runs up and a screen's runs down. Each piece keeps its own shape, and only the page's axis turns over.
fn scene_of(pattern: &Pattern) -> Scene {
    let mut ordered: Vec<&DrawnPiece> = pattern.pieces.iter().collect();
    ordered.sort_by(|a, b| a.name.cmp(&b.name));
    let mut scene = Scene::default();
    let (mut x, mut y, mut row_height) = (0, 0, 0);
    for piece in ordered {
        let (low_x, low_y, high_x, high_y) = extent(&piece.cut);
        let (width, height) = (high_x - low_x, high_y - low_y);
        if x > 0 && x + width > ROW_WIDTH {
            y += row_height + PIECE_GAP;
            x = 0;
            row_height = 0;
        }
        let place = |at: sc_geometry::Point| Point {
            x: x + at.x() - low_x,
            y: y + high_y - at.y(),
        };
        let polyline = |points: &[sc_geometry::Point]| -> Vec<Point> {
            points.iter().map(|at| place(*at)).collect()
        };
        let closed = |points: &[sc_geometry::Point]| -> Vec<Point> {
            let mut out = polyline(points);
            if let Some(first) = out.first().copied() {
                out.push(first);
            }
            out
        };
        let line = |item: &Line| vec![place(item.from), place(item.to)];
        scene.lines.push(Drawn {
            kind: "cut",
            points: closed(&piece.cut),
        });
        scene.lines.push(Drawn {
            kind: "sew",
            points: closed(&piece.sew),
        });
        scene.lines.push(Drawn {
            kind: "grain",
            points: line(&piece.grain),
        });
        for item in &piece.internal {
            scene.lines.push(Drawn {
                kind: "internal",
                points: line(item),
            });
        }
        for mark in &piece.marks {
            scene.lines.push(Drawn {
                kind: "notch",
                points: vec![place(mark.at), place(mark.toward)],
            });
        }
        for label in &piece.labels {
            scene.labels.push((place(label.at), label.text.clone()));
        }
        scene.names.push(piece.name.clone());
        scene.pieces.push(Piece {
            outline: polyline(&piece.cut),
            annotations: Vec::new(),
        });
        scene.width = scene.width.max(x + width);
        scene.height = scene.height.max(y + height);
        x += width + PIECE_GAP;
        row_height = row_height.max(height);
    }
    scene
}

fn text(value: &str) -> Value {
    Value::Text(value.to_owned())
}

fn number(value: usize) -> Value {
    Value::Int(i64::try_from(value).unwrap_or(i64::MAX))
}

// A step's own diagnostic, as the status line says it.
fn said(failure: &Failure) -> String {
    let value = failure.to_value();
    value
        .as_object()
        .and_then(|fields| fields.get("message"))
        .and_then(Value::as_text)
        .unwrap_or("the step failed")
        .to_owned()
}

fn step_named(name: &str) -> Value {
    let mut fields = BTreeMap::new();
    fields.insert("do".to_owned(), text(name));
    Value::Object(fields)
}

/// A session open in the shell.
#[derive(Debug)]
pub struct Shell {
    running: Running<Shared>,
    disk: Shared,
    opening: BTreeMap<String, Value>,
    draft_step: Option<Value>,
    recorded: Vec<Value>,
    bindings: Value,
    scene: Scene,
    view: View,
    rung: usize,
    size: (i64, i64),
    selected: Option<EdgeRef>,
    status: String,
    failed: bool,
    file: Option<Value>,
}

impl Shell {
    /// Opens a session file. Its `draft` step is what the shell redraws with, and the file's other steps are not run.
    ///
    /// # Errors
    /// The session is malformed, has no `draft` step, or does not draft.
    pub fn open(session: &str, size: (i64, i64)) -> Result<Self, String> {
        let parsed = parse(session).map_err(|error| format!("the session: {error}"))?;
        let mut opening = parsed
            .as_object()
            .ok_or("a session must be an object")?
            .clone();
        let steps = opening
            .remove("steps")
            .and_then(|steps| steps.as_list().map(<[Value]>::to_vec))
            .ok_or("a session must list steps")?;
        let is_draft = |step: &&Value| {
            step.as_object()
                .and_then(|fields| fields.get("do"))
                .and_then(Value::as_text)
                == Some("draft")
        };
        let draft_step = steps
            .iter()
            .find(is_draft)
            .ok_or("the shell draws a session's draft step, and this one has none")?
            .clone();
        let disk = Shared::default();
        let mut fields = opening.clone();
        fields.insert("steps".to_owned(), Value::List(Vec::new()));
        let running = Running::open(&Value::Object(fields), disk.clone()).map_err(|f| said(&f))?;
        let mut shell = Self {
            running,
            disk,
            opening,
            draft_step: Some(draft_step),
            recorded: Vec::new(),
            bindings: Value::List(Vec::new()),
            scene: Scene::default(),
            view: View::new((1, 1)).map_err(|error| error.to_string())?,
            rung: 17,
            size,
            selected: None,
            status: "opened".to_owned(),
            failed: false,
            file: None,
        };
        shell.redraw().map_err(|f| said(&f))?;
        shell.fit().map_err(|error| error.to_string())?;
        Ok(shell)
    }

    // The one way anything reaches the Design: a step, handed to the session, and recorded if it was taken.
    fn issue(&mut self, step: &Value) -> Result<Value, Failure> {
        let record = self.running.step(step)?;
        self.recorded.push(step.clone());
        Ok(record)
    }

    // The first draft names its pieces, and the session keeps them, so every later one is the same plain step a project can replay.
    fn redraw(&mut self) -> Result<(), Failure> {
        if let Some(first) = self.draft_step.take() {
            self.issue(&first)?;
        } else {
            self.issue(&step_named("draft"))?;
        }
        let record = self.issue(&step_named("evaluate"))?;
        self.bindings = record
            .as_object()
            .and_then(|fields| fields.get("result"))
            .cloned()
            .unwrap_or(Value::List(Vec::new()));
        if let Some(pattern) = self.running.pattern() {
            self.scene = scene_of(pattern);
        }
        self.selected = None;
        Ok(())
    }

    fn say(&mut self, message: String, failed: bool) {
        self.status = message;
        self.failed = failed;
    }

    // An edit, then the drawing it makes. A value the draft cannot draw is taken back through the bus, so the shell never holds a Design it cannot show.
    fn edit(&mut self, step: &Value, what: &str) {
        if let Err(failure) = self.issue(step) {
            self.say(format!("{what}: {}", said(&failure)), true);
            return;
        }
        match self.redraw() {
            Ok(()) => self.say(
                format!("{what}: revision {}", self.running.revision()),
                false,
            ),
            Err(failure) => {
                let reason = said(&failure);
                let back = self.issue(&step_named("undo")).and_then(|_| self.redraw());
                match back {
                    Ok(()) => self.say(format!("{what} was taken back: {reason}"), true),
                    Err(next) => self.say(format!("{what}: {reason}; then {}", said(&next)), true),
                }
            }
        }
    }

    fn export(&mut self, step: &Value) {
        let path = step
            .as_object()
            .and_then(|fields| fields.get("path"))
            .and_then(Value::as_text)
            .unwrap_or("")
            .to_owned();
        let written = self
            .issue(step)
            .and_then(|_| self.issue(&step_named("save")));
        if let Err(failure) = written {
            self.say(format!("export: {}", said(&failure)), true);
            return;
        }
        let bytes = FilePath::new(&path)
            .ok()
            .and_then(|at| self.disk.read(&at).ok().flatten());
        let Some(bytes) = bytes else {
            self.say(format!("export: {path} is not in the project"), true);
            return;
        };
        let mut file = BTreeMap::new();
        file.insert("path".to_owned(), text(&path));
        file.insert("sha256".to_owned(), Value::Text(sha256_hex(&bytes)));
        file.insert(
            "hex".to_owned(),
            Value::Text(bytes.iter().map(|byte| format!("{byte:02x}")).collect()),
        );
        self.file = Some(Value::Object(file));
        self.say(format!("exported {path}, {} bytes", bytes.len()), false);
    }

    fn rung_for(&self) -> usize {
        // The largest zoom at which the whole drawing fits, in exact arithmetic.
        let room = |side: i64| i128::from((side - 2 * FIT_MARGIN).max(1));
        let fits = |zoom: &(i64, i64), extent: i64, side: i64| {
            i128::from(extent) * i128::from(PIXELS_PER_MILLIMETRE) * i128::from(zoom.0)
                <= room(side) * i128::from(zoom.1) * i128::from(MICROMETRES_PER_MILLIMETRE)
        };
        LADDER
            .iter()
            .rposition(|zoom| {
                fits(zoom, self.scene.width, self.size.0)
                    && fits(zoom, self.scene.height, self.size.1)
            })
            .unwrap_or(0)
    }

    fn fit(&mut self) -> Result<(), sc_viewport::ViewError> {
        self.rung = self.rung_for();
        let zoom = LADDER.get(self.rung).copied().unwrap_or((1, 1));
        self.view = View::new(zoom)?.pan(-FIT_MARGIN, -FIT_MARGIN)?;
        Ok(())
    }

    fn zoom(&mut self, closer: bool, x: i64, y: i64) -> Result<(), sc_viewport::ViewError> {
        let rung = if closer {
            (self.rung + 1).min(LADDER.len() - 1)
        } else {
            self.rung.saturating_sub(1)
        };
        let zoom = LADDER.get(rung).copied().unwrap_or((1, 1));
        self.view = self.view.zoom_about(zoom, x, y)?;
        self.rung = rung;
        Ok(())
    }

    fn pick_at(&mut self, x: i64, y: i64) -> Result<(), sc_viewport::ViewError> {
        let point = self.view.point_of_pixel(x, y)?;
        let reach = self.view.point_of_pixel(x + PICK_REACH, y)?.x - point.x;
        self.selected = pick(&self.scene.pieces, point, reach);
        self.say(
            if self.selected.is_some() {
                "picked an edge".to_owned()
            } else {
                "nothing there".to_owned()
            },
            false,
        );
        Ok(())
    }

    /// Does what the page asks: an edit through the bus, or a change of view. Returns the whole state to draw.
    pub fn act(&mut self, action: &str) -> Value {
        self.file = None;
        match parse(action) {
            Ok(action) => self.dispatch(&action),
            Err(error) => self.say(format!("the action: {error}"), true),
        }
        self.state()
    }

    fn dispatch(&mut self, action: &Value) {
        let Some(fields) = action.as_object() else {
            return self.say("an action must be an object".to_owned(), true);
        };
        let what = fields.get("do").and_then(Value::as_text).unwrap_or("");
        let int = |name: &str| fields.get(name).and_then(Value::as_int).unwrap_or(0);
        let looked = match what {
            "change" => {
                let token = fields.get("token").and_then(Value::as_text).unwrap_or("?");
                return self.edit(action, &format!("{token} changed"));
            }
            "undo" => return self.edit(action, "undone"),
            "redo" => return self.edit(action, "redone"),
            "reopen" => return self.edit(action, "reopened"),
            "save" => {
                return match self.issue(action) {
                    Ok(_) => self.say(
                        format!("saved at revision {}", self.running.revision()),
                        false,
                    ),
                    Err(failure) => self.say(format!("save: {}", said(&failure)), true),
                };
            }
            "export_pattern" => return self.export(action),
            "zoom" => self.zoom(
                !matches!(fields.get("closer"), Some(Value::Bool(false))),
                int("x"),
                int("y"),
            ),
            "pan" => self.view.pan(int("x"), int("y")).map(|view| {
                self.view = view;
            }),
            "pick" => self.pick_at(int("x"), int("y")),
            "fit" => self.fit(),
            "resize" => {
                self.size = (int("width").max(1), int("height").max(1));
                self.fit()
            }
            other => return self.say(format!("no action is named {other}"), true),
        };
        if let Err(error) = looked {
            self.say(error.to_string(), true);
        }
    }

    fn pixels(&self, points: &[Point]) -> Value {
        let mut out = Vec::with_capacity(points.len() * 2);
        for point in points {
            if let Ok((x, y)) = self.view.pixel_of(*point) {
                out.push(fixed(x));
                out.push(fixed(y));
            }
        }
        Value::List(out)
    }

    fn selection(&self) -> Value {
        let Some(edge) = self.selected else {
            return Value::Null;
        };
        let piece = self.scene.pieces.get(edge.piece);
        let outline = piece.map_or(&[][..], |piece| piece.outline.as_slice());
        let (Some(from), Some(to)) = (
            outline.get(edge.edge).copied(),
            outline.get((edge.edge + 1) % outline.len().max(1)).copied(),
        ) else {
            return Value::Null;
        };
        let mut fields = BTreeMap::new();
        let name = self.scene.names.get(edge.piece).map_or("", String::as_str);
        fields.insert("piece".to_owned(), text(name));
        fields.insert("edge".to_owned(), number(edge.edge));
        fields.insert("length_um".to_owned(), Value::Int(rounded_length(from, to)));
        fields.insert("points".to_owned(), self.pixels(&[from, to]));
        Value::Object(fields)
    }

    /// The state a page draws: revision, inputs, derived values, status, and every line in pixels.
    #[must_use]
    pub fn state(&self) -> Value {
        let mut fields = BTreeMap::new();
        fields.insert("revision".to_owned(), Value::Text(self.running.revision()));
        fields.insert("can_undo".to_owned(), Value::Bool(self.running.can_undo()));
        fields.insert("can_redo".to_owned(), Value::Bool(self.running.can_redo()));
        fields.insert("status".to_owned(), text(&self.status));
        fields.insert("failed".to_owned(), Value::Bool(self.failed));
        fields.insert("steps".to_owned(), number(self.recorded.len()));
        fields.insert("pixel_units".to_owned(), Value::Int(PIXEL_UNITS));
        let inputs = self
            .running
            .inputs()
            .into_iter()
            .map(|(token, um)| {
                let mut input = BTreeMap::new();
                input.insert("token".to_owned(), Value::Text(token));
                input.insert("um".to_owned(), Value::Int(um));
                Value::Object(input)
            })
            .collect();
        fields.insert("inputs".to_owned(), Value::List(inputs));
        fields.insert("bindings".to_owned(), self.bindings.clone());
        let lines = self
            .scene
            .lines
            .iter()
            .map(|line| {
                let mut drawn = BTreeMap::new();
                drawn.insert("kind".to_owned(), text(line.kind));
                drawn.insert("points".to_owned(), self.pixels(&line.points));
                Value::Object(drawn)
            })
            .collect();
        fields.insert("lines".to_owned(), Value::List(lines));
        let labels = self
            .scene
            .labels
            .iter()
            .map(|(at, label)| {
                let mut drawn = BTreeMap::new();
                drawn.insert("at".to_owned(), self.pixels(&[*at]));
                drawn.insert("text".to_owned(), text(label));
                Value::Object(drawn)
            })
            .collect();
        fields.insert("labels".to_owned(), Value::List(labels));
        fields.insert("selected".to_owned(), self.selection());
        if let Some(file) = &self.file {
            fields.insert("file".to_owned(), file.clone());
        }
        Value::Object(fields)
    }

    /// Everything done to the Design so far, as a session the command-line program can replay.
    #[must_use]
    pub fn recording(&self) -> Value {
        let mut fields = self.opening.clone();
        fields.insert("steps".to_owned(), Value::List(self.recorded.clone()));
        Value::Object(fields)
    }
}

#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
fn fixed(pixel: f64) -> Value {
    Value::Int((pixel * PIXEL_UNITS as f64).round() as i64)
}

#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
fn rounded_length(from: Point, to: Point) -> i64 {
    let (x, y) = ((to.x - from.x) as f64, (to.y - from.y) as f64);
    (x * x + y * y).sqrt().round() as i64
}
