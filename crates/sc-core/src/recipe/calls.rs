//! Exact call lookup, with envelope precedence and source-bearing metadata refusals.
use super::FormulaBuiltin;
use crate::name::MachineToken;
use core::fmt;

/// Actual call lookup domains; these are distinct from data-name origins.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaCallLookupSource {
    /// The six envelope-owned aliases are checked before any built-in or argument.
    Envelope,
    /// The closed function/selector catalog, including metadata for the if special form.
    BuiltinCatalog,
}
impl FormulaCallLookupSource {
    /// Declared diagnostic metadata spelling, not a callable or data origin.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Envelope => "envelope",
            Self::BuiltinCatalog => "builtin_catalog",
        }
    }
}

/// Closed reason for refusing an exact callee before reading arguments.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaCallRefusalKind {
    /// Neither searched call domain declares the name.
    Unbound,
    /// The requested construct belongs to the deferred NURBS family.
    Nurbs,
    /// The requested construct belongs to deferred implicit sketch solving.
    SketchConstraints,
}

/// Declared alternatives for an envelope request; none constructs geometry or adds syntax.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaCallAlternative {
    /// Two-endpoint line primitive from the units contract.
    LineSegment,
    /// Circular arc primitive from the units contract.
    CircularArc,
    /// Four-control-point cubic primitive from the units contract.
    CubicBezier,
    /// Ordered drafting operations, outside implicit expression solving.
    OrderedConstructionRecipe,
}
impl FormulaCallAlternative {
    /// Declared diagnostic metadata spelling, not a new built-in name.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::LineSegment => "line_segment",
            Self::CircularArc => "circular_arc",
            Self::CubicBezier => "cubic_bezier",
            Self::OrderedConstructionRecipe => "ordered_construction_recipe",
        }
    }
}

/// Immutable refusal borrowing the real callee, without invented expression/recipe context.
/// For an envelope case, name is the requested construct in scope formula_call; no geometric
/// constraint parameters or entity are claimed. Explicit accessors expose the payload; ordinary
/// formatting omits it. Only lookup can construct this evidence.
/// ```compile_fail
/// fn forge() -> sc_core::recipe::FormulaCallRefusal<'static> {
///     sc_core::recipe::FormulaCallRefusal { name: "missing" }
/// }
/// ```
/// ```compile_fail
/// fn escape() -> sc_core::recipe::FormulaCallRefusal<'static> {
///     let name = sc_core::name::MachineToken::new("missing").unwrap();
///     sc_core::recipe::FormulaBuiltin::resolve_call(&name).unwrap_err()
/// }
/// ```
#[derive(Clone, Copy)]
pub struct FormulaCallRefusal<'a> {
    name: &'a str,
    kind: FormulaCallRefusalKind,
}
impl<'a> FormulaCallRefusal<'a> {
    /// Exact borrowed query; also the actual requested kind for an envelope alias.
    #[must_use]
    pub const fn name(self) -> &'a str {
        self.name
    }

    /// Closed refusal reason, independently of input values or context availability.
    #[must_use]
    pub const fn kind(self) -> FormulaCallRefusalKind {
        self.kind
    }

    /// Actual lookup/request scope; data-name reads have their own origin contract.
    #[must_use]
    pub const fn lookup_scope(self) -> &'static str {
        "formula_call"
    }

    /// Stable internal diagnostic token, localized by the command layer.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self.kind {
            FormulaCallRefusalKind::Unbound => "formula_unbound_name",
            FormulaCallRefusalKind::Nurbs => "env_nurbs",
            FormulaCallRefusalKind::SketchConstraints => "env_sketch_constraints",
        }
    }

    /// Domains actually searched, in order. Envelope refusal stops before the catalog.
    #[must_use]
    pub const fn origins_searched(self) -> &'static [FormulaCallLookupSource] {
        use FormulaCallLookupSource::{BuiltinCatalog, Envelope};
        match self.kind {
            FormulaCallRefusalKind::Unbound => &[Envelope, BuiltinCatalog],
            FormulaCallRefusalKind::Nurbs | FormulaCallRefusalKind::SketchConstraints => {
                &[Envelope]
            }
        }
    }

    /// Exact supported curve set or recipe alternative; unbound names have no claimed replacement.
    #[must_use]
    pub const fn alternatives(self) -> &'static [FormulaCallAlternative] {
        use FormulaCallAlternative::{
            CircularArc, CubicBezier, LineSegment, OrderedConstructionRecipe,
        };
        match self.kind {
            FormulaCallRefusalKind::Unbound => &[],
            FormulaCallRefusalKind::Nurbs => &[LineSegment, CircularArc, CubicBezier],
            FormulaCallRefusalKind::SketchConstraints => &[OrderedConstructionRecipe],
        }
    }
}
impl fmt::Debug for FormulaCallRefusal<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaCallRefusal")
            .field("kind", &self.kind)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for FormulaCallRefusal<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.token())
    }
}
impl std::error::Error for FormulaCallRefusal<'_> {}

impl FormulaBuiltin {
    /// Resolve an exact validated ordinary call name, before inspecting any argument.
    /// A namespace binding never supplies a callable. Success grants signature metadata only,
    /// not accepted operands, an expression proof, a value or geometry. The keyword if cannot
    /// enter a MachineToken; its existing special-form syntax is checked separately.
    /// # Errors
    /// Returns a source-bearing envelope refusal or an unbound-callee refusal borrowing name.
    pub fn resolve_call(name: &MachineToken) -> Result<Self, FormulaCallRefusal<'_>> {
        lookup_call_name(name.as_str())
    }
}

// The expression checker may project only a parser-validated callee through this private API.
pub(super) fn lookup_call_name(name: &str) -> Result<FormulaBuiltin, FormulaCallRefusal<'_>> {
    let kind = match name {
        "nurbs" | "spline" | "bspline" => FormulaCallRefusalKind::Nurbs,
        "solve" | "constraint" | "fixpoint" => FormulaCallRefusalKind::SketchConstraints,
        _ => {
            return FormulaBuiltin::from_token(name).ok_or(FormulaCallRefusal {
                name,
                kind: FormulaCallRefusalKind::Unbound,
            });
        }
    };
    Err(FormulaCallRefusal { name, kind })
}
