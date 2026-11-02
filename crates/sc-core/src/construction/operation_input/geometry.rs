//! Direct original native geometry fields; semantic targets and containing programmes stay separate.
use super::DraftingOperationInput;
use crate::{
    construction::{
        BezierPointDependency, CenterArcPointDependency, DraftingGeometryReference,
        LinePointDependency, MirrorAxisDependency, MirrorAxisSource, SlashSpreadDependency,
        SlashSpreadSource, WalkBalancePair,
    },
    ontology::{DirectedRange, EdgeAnchor, SeamSideId},
};
use core::{fmt, iter::FusedIterator};

/// Inspect an original native field, retaining existing typed dependency/selector views.
/// This enum alone does not certify an input association, source, frame or programme.
#[derive(Clone, Copy)]
pub enum DraftingNativeGeometryField<'a> {
    /// Actual original ordered line endpoint and declaration.
    Line(LinePointDependency<'a>),
    /// Actual original center field and arc declaration.
    CenterArc(CenterArcPointDependency<'a>),
    /// Actual original quadratic/cubic control role and field borrow.
    Bezier(BezierPointDependency<'a>),
    /// Actual directed source range, including its original bounds and traversal.
    Offset(&'a DirectedRange),
    /// Actual cut/pivot/direction source and original slash/spread declaration.
    SlashSpread(SlashSpreadDependency<'a>),
    /// Actual requested notch anchor, including its original source-frame parameter.
    NotchAnchor(&'a EdgeAnchor),
    /// Actual requested pair, side and original anchor; repeats are distinct occurrences.
    WalkAnchor {
        /// Original zero-based pair position.
        pair_index: usize,
        /// Actual original pair in the raw edit's authored vector.
        pair: &'a WalkBalancePair,
        /// Actual A/B role in that pair.
        side: SeamSideId,
        /// Actual original source-frame anchor field.
        anchor: &'a EdgeAnchor,
    },
    /// Actual explicit axis mode/role and original field borrow.
    Mirror(MirrorAxisDependency<'a>),
}
impl DraftingNativeGeometryField<'_> {
    /// Original persistent reference only; selector bounds/parameters remain on the typed field.
    #[must_use]
    pub fn reference(self) -> DraftingGeometryReference {
        use DraftingGeometryReference::{Edge, Point};
        match self {
            Self::Line(field) => Point(field.point()),
            Self::CenterArc(field) => Point(field.point()),
            Self::Bezier(field) => Point(field.point()),
            Self::Offset(range) => Edge(range.range.edge()),
            Self::SlashSpread(field) => match field.source() {
                SlashSpreadSource::Cut(range) | SlashSpreadSource::Direction(range) => {
                    Edge(range.range.edge())
                }
                SlashSpreadSource::Pivot(point) => Point(*point),
            },
            Self::NotchAnchor(anchor) | Self::WalkAnchor { anchor, .. } => Edge(anchor.edge),
            Self::Mirror(field) => match field.source() {
                MirrorAxisSource::Start(point) | MirrorAxisSource::End(point) => Point(*point),
                MirrorAxisSource::Range(range) => Edge(range.range.edge()),
            },
        }
    }
}
impl fmt::Debug for DraftingNativeGeometryField<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Line(field) => f.debug_tuple("Line").field(&field.endpoint()).finish(),
            Self::CenterArc(_) => f.write_str("CenterArc"),
            Self::Bezier(field) => f.debug_tuple("Bezier").field(&field.role()).finish(),
            Self::Offset(_) => f.write_str("Offset"),
            Self::SlashSpread(field) => f.write_str(match field.source() {
                SlashSpreadSource::Cut(_) => "SlashSpread::Cut",
                SlashSpreadSource::Pivot(_) => "SlashSpread::Pivot",
                SlashSpreadSource::Direction(_) => "SlashSpread::Direction",
            }),
            Self::NotchAnchor(_) => f.write_str("NotchAnchor"),
            Self::WalkAnchor {
                pair_index, side, ..
            } => f
                .debug_struct("WalkAnchor")
                .field("pair_index", pair_index)
                .field("side", side)
                .finish_non_exhaustive(),
            Self::Mirror(field) => f.write_str(match field.source() {
                MirrorAxisSource::Start(_) => "Mirror::Start",
                MirrorAxisSource::End(_) => "Mirror::End",
                MirrorAxisSource::Range(_) => "Mirror::Range",
            }),
        }
    }
}

/// One genuine direct field of an actual raw native input, without containing-programme approval.
/// Its private construction pins the original input, field order and typed selector together.
/// ```compile_fail,E0616
/// fn alter(use_:&mut sc_core::construction::DraftingNativeGeometryUse<'_>) {use_.index=0;}
/// ```
/// ```compile_fail
/// fn escape<'a>(use_:sc_core::construction::DraftingNativeGeometryUse<'a>)
///     ->sc_core::construction::DraftingNativeGeometryUse<'static> {use_}
/// ```
#[derive(Clone, Copy, Debug)]
pub struct DraftingNativeGeometryUse<'a> {
    input: DraftingOperationInput<'a>,
    index: usize,
    field: DraftingNativeGeometryField<'a>,
}
impl<'a> DraftingNativeGeometryUse<'a> {
    /// Exact actual original native input and explicit semantic history-operation identity.
    #[must_use]
    pub const fn input(self) -> DraftingOperationInput<'a> {
        self.input
    }
    /// Zero-based original direct geometry field order, not a mixed position or statement ordinal.
    #[must_use]
    pub const fn index(self) -> usize {
        self.index
    }
    /// Original typed field/role and selector data; copying the view copies only loans/metadata.
    #[must_use]
    pub const fn field(self) -> DraftingNativeGeometryField<'a> {
        self.field
    }
    /// Original persistent reference, without source lookup or selector evaluation.
    #[must_use]
    pub fn reference(self) -> DraftingGeometryReference {
        self.field.reference()
    }
}
impl<'a> DraftingOperationInput<'a> {
    /// Stream every direct original Point/range/anchor field, without allocation or source/value
    /// calls. Existing native typed dependencies remain authoritative; ranges and anchor parameters
    /// stay borrowed intact. Walk retains every original pair in pair/A/B order, including repeats.
    ///
    /// All thirteen raw families are classified through independent exhaustive private schema
    /// views. Point,CloseDart,Grainline,Truing and Join have no direct geometry fields. Their formula
    /// uses and geometry reached through actual semantic records are separate obligations; empty
    /// output here cannot certify an unprovided Dart/Grainline/span/Piece or whole programme.
    /// This grants no content/work quota, native admission, currentness, frame or geometry result.
    ///
    /// ```
    /// use sc_core::construction::{DraftingOperationInput,DraftingGeometryReference};
    /// fn direct_points(input:DraftingOperationInput<'_>) -> usize {
    ///     input.geometry_dependencies().filter(|use_|matches!(use_.reference(),
    ///         DraftingGeometryReference::Point(_))).count()
    /// }
    /// ```
    pub fn geometry_dependencies(
        self,
    ) -> impl FusedIterator<Item = DraftingNativeGeometryUse<'a>> + 'a {
        use DraftingNativeGeometryField as F;
        use DraftingOperationInput as I;
        match self {
            I::ConstructPoint(input) => input.geometry_input_guard(),
            I::DrawLine(input) => input.geometry_input_guard(),
            I::DrawArc(input) => input.geometry_input_guard(),
            I::DrawBezier(input) => input.geometry_input_guard(),
            I::OffsetEdge(input) => input.geometry_input_guard(),
            I::SlashSpread(input) => input.geometry_input_guard(),
            I::CloseDart { operation, request } => {
                let _: crate::ontology::EntityId = operation;
                request.geometry_input_guard();
            }
            I::AddNotch { operation, request } => {
                let _: crate::ontology::EntityId = operation;
                let _ = request.geometry_anchor();
            }
            I::PlaceGrainline(input) => input.geometry_input_guard(),
            I::WalkSeam(input) => {
                let _ = input.geometry_pairs();
            }
            I::TrueSeam(input) => input.geometry_input_guard(),
            I::MirrorPiece(input) => input.geometry_input_guard(),
            I::JoinPieces(input) => input.geometry_input_guard(),
        }
        let line = if let I::DrawLine(input) = self {
            Some(input)
        } else {
            None
        }
        .into_iter()
        .flat_map(|input| input.dependencies().map(F::Line));
        let arc = if let I::DrawArc(input) = self {
            Some(input)
        } else {
            None
        }
        .into_iter()
        .map(|input| F::CenterArc(input.center_dependency()));
        let bezier = if let I::DrawBezier(input) = self {
            Some(input)
        } else {
            None
        }
        .into_iter()
        .flat_map(|input| input.dependencies().map(F::Bezier));
        let offset = if let I::OffsetEdge(input) = self {
            Some(input)
        } else {
            None
        }
        .into_iter()
        .map(|input| F::Offset(input.source()));
        let slash = if let I::SlashSpread(input) = self {
            Some(input)
        } else {
            None
        }
        .into_iter()
        .flat_map(|input| input.dependencies().map(F::SlashSpread));
        let notch = if let I::AddNotch { request, .. } = self {
            Some(request)
        } else {
            None
        }
        .into_iter()
        .map(|request| F::NotchAnchor(request.geometry_anchor()));
        let walk = if let I::WalkSeam(input) = self {
            Some(input)
        } else {
            None
        }
        .into_iter()
        .flat_map(|input| {
            input
                .geometry_pairs()
                .iter()
                .enumerate()
                .flat_map(|(pair_index, pair)| {
                    pair.geometry_anchors()
                        .into_iter()
                        .map(move |(side, anchor)| F::WalkAnchor {
                            pair_index,
                            pair,
                            side,
                            anchor,
                        })
                })
        });
        let mirror = if let I::MirrorPiece(input) = self {
            Some(input)
        } else {
            None
        }
        .into_iter()
        .flat_map(|input| input.dependencies().map(F::Mirror));
        line.chain(arc)
            .chain(bezier)
            .chain(offset)
            .chain(slash)
            .chain(notch)
            .chain(walk)
            .chain(mirror)
            .enumerate()
            .map(move |(index, field)| DraftingNativeGeometryUse {
                input: self,
                index,
                field,
            })
            .fuse()
    }
}
