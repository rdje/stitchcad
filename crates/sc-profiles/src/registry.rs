//! The parameters this schema knows. A name not listed here does nothing anywhere, so a file naming one is refused.

/// What kind of value a parameter holds. The tokens are the roadmap's, and no floating point is behind them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// One token from a list.
    Enum,
    /// A notch's physical shape.
    NotchSpec,
    /// Two exact numbers.
    Vec2f,
    /// One exact number.
    F32,
    /// Yes or no.
    Bool,
    /// A pen number for each plotter function.
    PenMap,
}

impl Kind {
    /// Its name in a profile file.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Enum => "enum",
            Self::NotchSpec => "notch_spec",
            Self::Vec2f => "vec2f",
            Self::F32 => "f32",
            Self::Bool => "bool",
            Self::PenMap => "pen_map",
        }
    }
}

/// One parameter the schema knows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParameterSpec {
    /// Its name in a profile file.
    pub name: &'static str,
    /// What it holds.
    pub kind: Kind,
    /// For an enum, the tokens anything here understands. A profile's domain may narrow them and cannot add to them.
    pub tokens: &'static [&'static str],
    /// The number must be above zero.
    pub positive: bool,
    /// It is a transformation applied at a declared stage.
    pub transformation: bool,
}

const fn spec(name: &'static str, kind: Kind, tokens: &'static [&'static str]) -> ParameterSpec {
    ParameterSpec {
        name,
        kind,
        tokens,
        positive: false,
        transformation: false,
    }
}

/// Every parameter this schema knows, with the roadmap's ten first.
pub const PARAMETERS: [ParameterSpec; 10] = [
    spec(
        "seam_allowance_policy",
        Kind::Enum,
        &["included", "excluded"],
    ),
    spec("notch_geometry", Kind::NotchSpec, &[]),
    spec("units", Kind::Enum, &["mm", "cm"]),
    spec("dxf_version_target", Kind::Enum, &["r12", "r13", "r2000"]),
    spec(
        "spline_policy",
        Kind::Enum,
        &["explode_bulge", "flatten_chord_0.1mm"],
    ),
    ParameterSpec {
        transformation: true,
        ..spec("shrinkage_xy_pct", Kind::Vec2f, &[])
    },
    ParameterSpec {
        positive: true,
        ..spec("max_cutting_width_mm", Kind::F32, &[])
    },
    spec("piece_block_required", Kind::Bool, &[]),
    spec("cut_sew_swap", Kind::Bool, &[]),
    spec("hpgl_pens", Kind::PenMap, &[]),
];

/// The artifact targets a profile can say it accepts, by the names the artifact contract registers.
pub const ACCEPTS: [&str; 4] = ["dxf-aama-named", "dxf-astm-num-r13", "plt", "pdf-a0"];

/// The notch shapes a `notch_spec` can name.
pub const NOTCH_CODES: [&str; 6] = ["slit", "V", "T", "castle", "check", "U"];

/// The plotter functions a `pen_map` can give a pen to.
pub const PEN_FUNCTIONS: [&str; 4] = ["cut", "draw", "notch", "drill"];

pub(crate) fn spec_of(name: &str) -> Option<&'static ParameterSpec> {
    PARAMETERS.iter().find(|spec| spec.name == name)
}
