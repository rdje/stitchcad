//! A drawn pattern: what an exporter needs, in whole micrometres, with nothing about how it was drafted.
use sc_geometry::Point;
use sc_store::Value;
use std::collections::BTreeMap;

/// A straight internal line: a dart leg, a fold or a grainline.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Line {
    /// Where it starts.
    pub from: Point,
    /// Where it ends.
    pub to: Point,
}

/// What a mark on a piece is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MarkKind {
    /// A V notch in the cut edge.
    VNotch,
    /// A slit notch in the cut edge.
    SlitNotch,
    /// A drilled hole.
    Drill,
}

/// A mark at a point, with the direction it faces when it has one.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mark {
    /// What it is.
    pub kind: MarkKind,
    /// Where it is.
    pub at: Point,
    /// A point it aims at, inward from the edge for a notch.
    pub toward: Point,
}

/// One line of label text.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Text {
    /// Where it starts.
    pub at: Point,
    /// What it says.
    pub text: String,
}

/// What kind of closure takes up cloth on a piece.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClosureKind {
    /// Two legs meeting at an apex, stitched together.
    Dart,
    /// A fold stitched down part of its length.
    Tuck,
    /// A fold pressed and held at the seam.
    Pleat,
    /// A range drawn up to a shorter length.
    Gather,
}

/// A closure with its meaning, not its line art: where it takes cloth from and how much.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Closure {
    /// What kind it is.
    pub kind: ClosureKind,
    /// Its name on this piece, which a seam or a report refers to.
    pub name: String,
    /// Where the range it takes up starts on the sew line. For a dart, its near leg's top.
    pub from: Point,
    /// Where that range ends. For a dart, its far leg's top.
    pub to: Point,
    /// A dart's apex.
    pub apex: Option<Point>,
    /// How much it removes from the sewn length of the edge it sits on, in micrometres.
    pub intake: i64,
}

/// One drawn piece.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DrawnPiece {
    /// The piece's name, as in `front`.
    pub name: String,
    /// The size it was drawn at, as in `M`.
    pub size: String,
    /// How many to cut.
    pub quantity: u32,
    /// The cut boundary, allowances included, as a closed polyline.
    pub cut: Vec<Point>,
    /// The sew line, as a closed polyline.
    pub sew: Vec<Point>,
    /// The grainline, directed.
    pub grain: Line,
    /// Internal lines.
    pub internal: Vec<Line>,
    /// Notches and drill holes.
    pub marks: Vec<Mark>,
    /// Label lines.
    pub labels: Vec<Text>,
    /// Darts, tucks, pleats and gathers, which a seam's length accounts for.
    pub closures: Vec<Closure>,
    /// The points a grading engine moves, each by name.
    pub grade_points: Vec<GradePoint>,
}

/// A named point on a piece's sew line, which a grade rule moves.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GradePoint {
    /// Its name on this piece.
    pub name: String,
    /// Where it is.
    pub at: Point,
}

/// A length a drawing solved for: the value of an input that makes a named seam walk to what it declares.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fit {
    /// The input it varies.
    pub vary: String,
    /// The seam that decides it.
    pub seam: String,
    /// What it came to, in micrometres.
    pub value: i64,
}

impl Fit {
    /// The fit as a value.
    #[must_use]
    pub fn to_value(&self) -> Value {
        let mut fields = BTreeMap::new();
        fields.insert("vary".to_owned(), Value::Text(self.vary.clone()));
        fields.insert("seam".to_owned(), Value::Text(self.seam.clone()));
        fields.insert("value".to_owned(), Value::Int(self.value));
        Value::Object(fields)
    }
}

/// A reference an edit left with nowhere to go. It is shown and it blocks a release, and nothing moves it onto another edge for you.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Repair {
    /// The piece it is on.
    pub piece: String,
    /// What holds the reference: a notch, a grade point, or one end of a seam.
    pub carrier: String,
    /// What it holds, as written: an edge and a fraction of it.
    pub reference: String,
    /// The edit that orphaned it, in words.
    pub orphaned_by: String,
    /// Where it might go instead, which is empty when nothing can be suggested honestly.
    pub candidates: Vec<String>,
}

impl Repair {
    /// The task as a value.
    #[must_use]
    pub fn to_value(&self) -> Value {
        let mut fields = BTreeMap::new();
        fields.insert("piece".to_owned(), Value::Text(self.piece.clone()));
        fields.insert("carrier".to_owned(), Value::Text(self.carrier.clone()));
        fields.insert("reference".to_owned(), Value::Text(self.reference.clone()));
        fields.insert(
            "orphaned_by".to_owned(),
            Value::Text(self.orphaned_by.clone()),
        );
        fields.insert(
            "candidates".to_owned(),
            Value::List(self.candidates.iter().cloned().map(Value::Text).collect()),
        );
        Value::Object(fields)
    }
}

/// One side of a seam: a stretch of a piece's sew line, in the direction the seam is walked.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SeamSide {
    /// The piece.
    pub piece: String,
    /// Where the seam starts on it.
    pub from: Point,
    /// Where it ends.
    pub to: Point,
}

/// Two stretches sewn together, with the ease declared between them.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Seam {
    /// The seam's name.
    pub name: String,
    /// One side.
    pub a: SeamSide,
    /// The other side.
    pub b: SeamSide,
    /// Side A's length less side B's that is intended, in micrometres. Negative means side B is the longer.
    pub ease: i64,
}

/// A whole drawn pattern at one size.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pattern {
    /// The style's name.
    pub style: String,
    /// The Design's identity.
    pub design: String,
    /// The Design's revision.
    pub revision: String,
    /// The pieces.
    pub pieces: Vec<DrawnPiece>,
    /// The sewing graph: which stretches are sewn to which, and with what ease.
    pub seams: Vec<Seam>,
    /// References an edit orphaned. While any is open the drawing can be inspected and cannot be released.
    pub repairs: Vec<Repair>,
    /// What the drawing solved for, which is empty when it solved for nothing.
    pub fits: Vec<Fit>,
}
