//! Complete original native semantic targets/Profile references;no lookup or acceptance.
use super::DraftingOperationInput;
use crate::{
    construction::WalkBalancePair,
    ontology::{
        EntityId, NotchDimensionBindings, NotchProfileBindings, ProfileParameterRef, SeamSideId,
    },
};
use core::{fmt, iter::FusedIterator};
/// Actual semantic target role,without a registry/kind/value or source certificate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingNativeSemanticTargetRole {
    /// Original Offset SeamAllowance target.
    OffsetAllowance,
    /// Original SlashSpread source Piece.
    SlashPiece,
    /// Original canonical Dart target.
    Dart,
    /// Original Notch owning Piece.
    NotchPiece,
    /// Original canonical Grainline target.
    Grainline,
    /// Original Walk canonical span.
    WalkSpan,
    /// Original constrained truing span.
    TruingSpan,
    /// Original constrained truing adjusted Piece.
    TruingPiece,
    /// Original constrained truing fit-tolerance source.
    TruingFitTolerance,
    /// Original Mirror source Piece.
    MirrorPiece,
    /// Original Join canonical span.
    JoinSpan,
    /// Original Join source Piece A.
    JoinPieceA,
    /// Original Join source Piece B.
    JoinPieceB,
}
/// Original symbolic Notch Profile field;sample/production remain separate occurrences.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingNotchProfileRole {
    /// Original style source.
    Style,
    /// Original sample depth source.
    SampleDepth,
    /// Original sample width source.
    SampleWidth,
    /// Original production depth source.
    ProductionDepth,
    /// Original production width source.
    ProductionWidth,
    /// Original encoding source.
    Encoding,
}
/// Actual borrowed native semantic field. This public view alone binds no source or owner.
#[derive(Clone, Copy)]
pub enum DraftingNativeSemanticField<'a> {
    /// Original canonical/source identity field,without reinterpretation as geometry.
    Target {
        /// Exact original role.
        role: DraftingNativeSemanticTargetRole,
        /// Exact original field loan.
        reference: &'a EntityId,
    },
    /// Original requested Notch Profile reference.
    NotchProfile {
        /// Actual original symbolic field role.
        role: DraftingNotchProfileRole,
        /// Exact original Profile field.
        reference: &'a ProfileParameterRef,
    },
    /// Every original Walk pair/A/B/Profile field,including repeated references.
    WalkNotchProfile {
        /// Zero-based original pair position.
        pair_index: usize,
        /// Original explicit canonical side.
        side: SeamSideId,
        /// Actual original symbolic field role.
        role: DraftingNotchProfileRole,
        /// Exact original Profile field loan.
        reference: &'a ProfileParameterRef,
    },
}
impl fmt::Debug for DraftingNativeSemanticField<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Target { role, .. } => f.debug_tuple("Target").field(role).finish(),
            Self::NotchProfile { role, .. } => f.debug_tuple("NotchProfile").field(role).finish(),
            Self::WalkNotchProfile {
                pair_index,
                side,
                role,
                ..
            } => f
                .debug_struct("WalkNotchProfile")
                .field("pair_index", pair_index)
                .field("side", side)
                .field("role", role)
                .finish_non_exhaustive(),
        }
    }
}
/// Original native input and its actual semantic target/Profile field. No synthetic mixed position,
/// canonical/source/currentness,Profile pin,value,resource allowance or executable proof is supplied.
/// ```compile_fail,E0616
/// fn alter(use_:&mut sc_core::construction::DraftingNativeSemanticUse<'_>) {use_.field=todo!();}
/// ```
/// Original input storage cannot escape:
/// ```compile_fail
/// use sc_core::construction::{DraftingNativeSemanticUse,DraftingOperationInput};
/// fn escape<'a>(input:DraftingOperationInput<'a>)->Option<DraftingNativeSemanticUse<'static>> {
///     input.semantic_dependencies().next()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct DraftingNativeSemanticUse<'a> {
    input: DraftingOperationInput<'a>,
    field: DraftingNativeSemanticField<'a>,
}
impl<'a> DraftingNativeSemanticUse<'a> {
    /// Exact original native input,including explicit command-history identity where authored.
    #[must_use]
    pub const fn input(self) -> DraftingOperationInput<'a> {
        self.input
    }
    /// Exact original semantic field and any pair/side/Profile role metadata.
    #[must_use]
    pub const fn field(self) -> DraftingNativeSemanticField<'a> {
        self.field
    }
}
type Classified<'a> = (
    [Option<DraftingNativeSemanticField<'a>>; 3],
    Option<&'a NotchProfileBindings>,
    Option<&'a [WalkBalancePair]>,
);
fn target(
    role: DraftingNativeSemanticTargetRole,
    reference: &EntityId,
) -> Option<DraftingNativeSemanticField<'_>> {
    Some(DraftingNativeSemanticField::Target { role, reference })
}
impl<'a> DraftingOperationInput<'a> {
    /// Classify all13 native schemas and stream every original semantic target/Profile field.
    /// Independent exhaustive owner guards distinguish numeric/direct geometry from these fields.
    /// Walk order is span,then original pair order,A/B,and style/sample/production/encoding fields.
    /// No deduplication,query,value,clone,result vector,work allowance or mixed position occurs.
    /// Empty output for a direct point/curve family discharges no numeric/geometry obligations.
    /// ```
    /// use sc_core::construction::DraftingOperationInput;
    /// fn count(input:DraftingOperationInput<'_>)->usize {input.semantic_dependencies().count()}
    /// ```
    pub fn semantic_dependencies(
        self,
    ) -> impl FusedIterator<Item = DraftingNativeSemanticUse<'a>> + 'a {
        use DraftingNativeSemanticTargetRole as R;
        let (targets, notch, walk): Classified<'a> = match self {
            Self::ConstructPoint(input) => {
                input.semantic_input_guard();
                ([None; 3], None, None)
            }
            Self::DrawLine(input) => {
                input.semantic_input_guard();
                ([None; 3], None, None)
            }
            Self::DrawArc(input) => {
                input.semantic_input_guard();
                ([None; 3], None, None)
            }
            Self::DrawBezier(input) => {
                input.semantic_input_guard();
                ([None; 3], None, None)
            }
            Self::OffsetEdge(input) => (
                [
                    target(R::OffsetAllowance, input.semantic_allowance()),
                    None,
                    None,
                ],
                None,
                None,
            ),
            Self::SlashSpread(input) => (
                [target(R::SlashPiece, input.semantic_piece()), None, None],
                None,
                None,
            ),
            Self::CloseDart { operation, request } => {
                let _: EntityId = operation;
                (
                    [target(R::Dart, request.semantic_dart()), None, None],
                    None,
                    None,
                )
            }
            Self::AddNotch { operation, request } => {
                let _: EntityId = operation;
                let (piece, profile) = request.semantic_fields();
                (
                    [target(R::NotchPiece, piece), None, None],
                    Some(profile),
                    None,
                )
            }
            Self::PlaceGrainline(input) => (
                [target(R::Grainline, input.semantic_grainline()), None, None],
                None,
                None,
            ),
            Self::WalkSeam(input) => {
                let (span, pairs) = input.semantic_fields();
                ([target(R::WalkSpan, span), None, None], None, Some(pairs))
            }
            Self::TrueSeam(input) => {
                let [span, piece, tolerance] = input.semantic_targets();
                (
                    [
                        target(R::TruingSpan, span),
                        target(R::TruingPiece, piece),
                        target(R::TruingFitTolerance, tolerance),
                    ],
                    None,
                    None,
                )
            }
            Self::MirrorPiece(input) => (
                [target(R::MirrorPiece, input.semantic_piece()), None, None],
                None,
                None,
            ),
            Self::JoinPieces(input) => {
                let [span, a, b] = input.semantic_targets();
                (
                    [
                        target(R::JoinSpan, span),
                        target(R::JoinPieceA, a),
                        target(R::JoinPieceB, b),
                    ],
                    None,
                    None,
                )
            }
        };
        targets
            .into_iter()
            .flatten()
            .chain(notch.into_iter().flat_map(|profile| {
                profile_fields(profile)
                    .into_iter()
                    .map(
                        |(role, reference)| DraftingNativeSemanticField::NotchProfile {
                            role,
                            reference,
                        },
                    )
            }))
            .chain(
                walk.into_iter()
                    .flat_map(|pairs| pairs.iter().enumerate())
                    .flat_map(|(pair_index, pair)| {
                        pair.semantic_profiles()
                            .into_iter()
                            .flat_map(move |(side, profile)| {
                                profile_fields(profile)
                                    .into_iter()
                                    .map(move |(role, reference)| {
                                        DraftingNativeSemanticField::WalkNotchProfile {
                                            pair_index,
                                            side,
                                            role,
                                            reference,
                                        }
                                    })
                            })
                    }),
            )
            .map(move |field| DraftingNativeSemanticUse { input: self, field })
            .fuse()
    }
}
fn profile_fields(
    profile: &NotchProfileBindings,
) -> [(DraftingNotchProfileRole, &ProfileParameterRef); 6] {
    use DraftingNotchProfileRole as R;
    let NotchProfileBindings {
        style,
        sample,
        production,
        encoding,
    } = profile;
    let NotchDimensionBindings {
        depth: sample_depth,
        width: sample_width,
    } = sample;
    let NotchDimensionBindings {
        depth: production_depth,
        width: production_width,
    } = production;
    [
        (R::Style, style),
        (R::SampleDepth, sample_depth),
        (R::SampleWidth, sample_width),
        (R::ProductionDepth, production_depth),
        (R::ProductionWidth, production_width),
        (R::Encoding, encoding),
    ]
}
