//! A drawn pattern: what an exporter needs, in whole micrometres, with nothing about how it was drafted.
use sc_geometry::Point;

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
}
