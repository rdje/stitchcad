//! The garment ontology — every first-class object in a StitchCAD design, and the identity that ties them
//! together.
//!
//! Normative source: `docs/book/src/spec/ontology.md` (gate G0, specified by `G0-CONTRACT.3`). Implemented
//! across three ordered slices: **`.3a`** the identity layer, **`.3b`** the persistent-identity contract
//! that resolves references under split/merge/reverse/delete (both landed), and **`.3c`** the
//! geometry-bearing object types (pieces, copy plans, notches, grainlines, allowances and sewing graphs landed;
//! other marks and constructions follow).
//!
//! The identity layer is the foundation the other two consume: a reference is meaningless without a stable
//! id to point at, and the contract is meaningless without a reference to resolve. It is dependency-free and
//! builds for `wasm32-unknown-unknown`, like the rest of `sc-core`'s core.
//!
//! | Submodule | Contents | Leaf |
//! | --- | --- | --- |
//! | [`allowance`] | per-edge width origins, corner intent and symbolic profile inclusion | `.3c.3c` |
//! | [`anchor`] | shared born-valid semantic anchor checks and typed refusals | `.3c.3a`/`.3c.2b.2` |
//! | [`cut`] | explicit physical-copy identities, complete quantity/orientation validation | `.3c.2b.1` |
//! | [`grain`] | directed grainline and independent stripe/plaid references, deferred angles | `.3c.3b` |
//! | [`notch`] | immutable semantic anchors and symbolic profile bindings, deferred physical validation | `.3c.3a` |
//! | [`piece`] | immutable structurally validated `Piece`, complete label view, deferred geometry | `.3c.1` |
//! | [`id`] | `EntityId` (a ULID) and the injected `IdGenerator` | `.3a` |
//! | [`range`] | whole-interval coverage and visible range repairs, distinct from endpoint queries | `.3c.2a` |
//! | [`rational`] | the bounded exact rational a parameter is stored in | `.3a` |
//! | [`reference`] | `EdgeRef`, `PointRef`, `LocalTag` and the `[0, 1]` `Param` | `.3a` |
//! | [`sewing`] | copy-addressed spans, disjoint self-seams, declared ease and semantic stops | `.3c.2b.2` |
//! | [`topology`] | the persistent-identity contract: the edit journal, resolution, `RepairTask`s | `.3b` |

pub mod allowance;
pub mod anchor;
pub mod cut;
pub mod grain;
pub mod id;
pub mod notch;
pub mod piece;
pub mod range;
pub mod rational;
pub mod reference;
pub mod sewing;
pub mod topology;

pub use id::{DeterministicIdGenerator, EntityId, IdError, IdGenerator, MAX_TIMESTAMP_MS};
pub use rational::Rational;
pub use reference::{EdgeRef, LocalTag, Param, ParamError, PointRef};
pub use topology::{
    Direction, IdentityLedger, JournalEntry, LedgerError, OffsetFragment, OffsetInterval,
    OpenRepair, OrphaningEdit, Registration, ReleaseReadiness, RepairTask, Resolution, ResolvedRef,
    SplitFragments, SplitSide, TopologyEdit, MAX_EDGES_PER_OPERATION,
};

pub use piece::{
    CuttingSide, DirectedEdge, GeometricValidation, Handedness, LabelField, LabelText,
    LoopLocation, MaterialAssignment, Mirroring, Piece, PieceDefinition, PieceError, PrintedLabel,
};

pub use range::{
    EdgeRange, RangeError, RangeIssue, RangePortion, RangeRepairTask, RangeResolution,
    ResolvedRange,
};

pub use notch::{
    EdgeAnchor, Notch, NotchDefinition, NotchDimensionBindings, NotchEncoding, NotchError,
    NotchProfileBindings, NotchStyle, ProfileBindingValidation, ProfileParameterRef,
};

pub use cut::{CopyOrientation, CutCopy, CutCopyDefinition, CutPlan, CutPlanError};

pub use anchor::AnchorError;

pub use sewing::{
    DeclaredEase, EaseAmount, EaseDistribution, EaseValidation, SeamDirection, SeamSide,
    SeamSideId, SeamSpan, SeamSpanDefinition, SewingError, SewingGraph, SewingGraphDefinition,
    SewingLandmark, StopLandmark, TurnPoint, WeightIssue, WeightedEaseRegion,
};

pub use grain::{
    DirectedRange, DirectedRangePortion, DirectedRangeResolution, GrainAlignment, GrainAngle,
    GrainReferenceRole, Grainline, GrainlineDefinition, GrainlineError,
};

pub use allowance::{
    AllowanceInclusion, AllowanceWidth, CornerTreatment, SeamAllowance, SeamAllowanceDefinition,
    SeamAllowanceError,
};
