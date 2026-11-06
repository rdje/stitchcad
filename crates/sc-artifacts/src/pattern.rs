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
}
