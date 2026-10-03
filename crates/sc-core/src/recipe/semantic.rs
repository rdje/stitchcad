//! Closed semantic vocabulary; declared kinds and required contexts carry no numeric values.
use super::{FormulaBindingKind, FormulaToleranceName};

/// Every kind accepted as a formula operand. Geometry references are not bindable scalars.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FormulaKind {
    /// Distance or coordinate, measured in micrometres when bound.
    Length,
    /// Signed angle retaining complete turns when bound.
    Angle,
    /// Derived square-micrometre area.
    Area,
    /// Scale factor in parts per million when bound.
    Ratio,
    /// Nonnegative count when bound.
    Count,
    /// Boolean condition.
    Boolean,
    /// Existing constructed point reference.
    Point,
    /// Existing constructed edge reference.
    Edge,
}
impl FormulaKind {
    /// Complete closed population in normative table order.
    pub const ALL: [Self; 8] = [
        Self::Length,
        Self::Angle,
        Self::Area,
        Self::Ratio,
        Self::Count,
        Self::Boolean,
        Self::Point,
        Self::Edge,
    ];

    /// Exact machine token; no presentation localization is implied.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Length => "length",
            Self::Angle => "angle",
            Self::Area => "area",
            Self::Ratio => "ratio",
            Self::Count => "count",
            Self::Boolean => "boolean",
            Self::Point => "point",
            Self::Edge => "edge",
        }
    }

    /// Recognize exactly one normative kind token, without trimming or case conversion.
    #[must_use]
    pub fn from_token(token: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.token() == token)
    }

    /// Scalar annotation for a bindable kind; geometry cannot be bound by let.
    #[must_use]
    pub const fn binding_kind(self) -> Option<FormulaBindingKind> {
        match self {
            Self::Length => Some(FormulaBindingKind::Length),
            Self::Angle => Some(FormulaBindingKind::Angle),
            Self::Area => Some(FormulaBindingKind::Area),
            Self::Ratio => Some(FormulaBindingKind::Ratio),
            Self::Count => Some(FormulaBindingKind::Count),
            Self::Boolean => Some(FormulaBindingKind::Boolean),
            Self::Point | Self::Edge => None,
        }
    }
}
impl From<FormulaBindingKind> for FormulaKind {
    fn from(kind: FormulaBindingKind) -> Self {
        match kind {
            FormulaBindingKind::Length => Self::Length,
            FormulaBindingKind::Angle => Self::Angle,
            FormulaBindingKind::Area => Self::Area,
            FormulaBindingKind::Ratio => Self::Ratio,
            FormulaBindingKind::Count => Self::Count,
            FormulaBindingKind::Boolean => Self::Boolean,
        }
    }
}

/// Sole source domain for a declared formula name, independently of value availability.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FormulaOrigin {
    /// Selected measurement table.
    Measurement,
    /// Selected Ease set.
    Ease,
    /// Authored Design parameter.
    Parameter,
    /// Instantiation Factory Profile.
    Profile,
    /// Piece Material property.
    Material,
    /// Prior operation's point or edge output.
    Geometry,
    /// Prior recipe let statement.
    Recipe,
    /// Instance size context.
    Size,
    /// Reserved tolerance class.
    Tolerance,
}
impl FormulaOrigin {
    /// Complete closed population in normative table order.
    pub const ALL: [Self; 9] = [
        Self::Measurement,
        Self::Ease,
        Self::Parameter,
        Self::Profile,
        Self::Material,
        Self::Geometry,
        Self::Recipe,
        Self::Size,
        Self::Tolerance,
    ];

    /// Exact declared machine token.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Measurement => "measurement",
            Self::Ease => "ease",
            Self::Parameter => "parameter",
            Self::Profile => "profile",
            Self::Material => "material",
            Self::Geometry => "geometry",
            Self::Recipe => "recipe",
            Self::Size => "size",
            Self::Tolerance => "tolerance",
        }
    }

    /// Recognize a source domain exactly, without aliases or a fallback origin.
    #[must_use]
    pub fn from_token(token: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|origin| origin.token() == token)
    }
}

/// Context that must supply a reserved name's runtime value, not evidence it is available.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaReservedContext {
    /// Language-defined numerical or geometric tolerance.
    Always,
    /// Target format or receiver export context.
    Export,
    /// Factory Profile supplying its own physical tolerance.
    Profile,
    /// Instance size membership and authored size order.
    Size,
}

/// Closed built-in namespace. A spelling remains visible even when its required context is absent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaReservedName {
    /// One of the existing five symbolic tolerance annotations.
    Tolerance(FormulaToleranceName),
    /// One-based ordinal in authored size order.
    SizeIndex,
    /// Number of members in the selected size set.
    SizeCount,
    /// Whether the instance is its size set's base member.
    IsBaseSize,
}
impl FormulaReservedName {
    /// Complete closed population in normative table order.
    pub const ALL: [Self; 8] = [
        Self::Tolerance(FormulaToleranceName::Numerical),
        Self::Tolerance(FormulaToleranceName::Geometric),
        Self::Tolerance(FormulaToleranceName::Format),
        Self::Tolerance(FormulaToleranceName::Importer),
        Self::Tolerance(FormulaToleranceName::Physical),
        Self::SizeIndex,
        Self::SizeCount,
        Self::IsBaseSize,
    ];

    /// Exact built-in spelling, sharing tolerance annotation tokens with statement syntax.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Tolerance(name) => name.token(),
            Self::SizeIndex => "size_index",
            Self::SizeCount => "size_count",
            Self::IsBaseSize => "is_base_size",
        }
    }

    /// Recognize only the eight exact built-in names; ordinary identifiers are not reserved.
    #[must_use]
    pub fn from_token(token: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|name| name.token() == token)
    }

    /// Statically declared kind, irrespective of runtime context availability.
    #[must_use]
    pub const fn kind(self) -> FormulaKind {
        match self {
            Self::Tolerance(_) => FormulaKind::Length,
            Self::SizeIndex | Self::SizeCount => FormulaKind::Count,
            Self::IsBaseSize => FormulaKind::Boolean,
        }
    }

    /// Semantic origin: physical tolerance is a tolerance name supplied by a profile context.
    #[must_use]
    pub const fn origin(self) -> FormulaOrigin {
        match self {
            Self::Tolerance(_) => FormulaOrigin::Tolerance,
            Self::SizeIndex | Self::SizeCount | Self::IsBaseSize => FormulaOrigin::Size,
        }
    }

    /// Required value provider; does not inspect, synthesize or fetch that context's value.
    #[must_use]
    pub const fn required_context(self) -> FormulaReservedContext {
        match self {
            Self::Tolerance(FormulaToleranceName::Numerical | FormulaToleranceName::Geometric) => {
                FormulaReservedContext::Always
            }
            Self::Tolerance(FormulaToleranceName::Format | FormulaToleranceName::Importer) => {
                FormulaReservedContext::Export
            }
            Self::Tolerance(FormulaToleranceName::Physical) => FormulaReservedContext::Profile,
            Self::SizeIndex | Self::SizeCount | Self::IsBaseSize => FormulaReservedContext::Size,
        }
    }

    /// Symbolic tolerance role; size names are never a tolerance class.
    #[must_use]
    pub const fn tolerance_name(self) -> Option<FormulaToleranceName> {
        match self {
            Self::Tolerance(name) => Some(name),
            Self::SizeIndex | Self::SizeCount | Self::IsBaseSize => None,
        }
    }
}
