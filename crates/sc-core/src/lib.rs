//! `sc-core` — the garment ontology, the construction recipe and the command bus.
//!
//! **Status:** identity, explicit-tag canonical edge declarations, point/range reference resolution, structural pieces/copy plans, semantic
//! notches, pair-consistent Piece inventories, canonical Material property sources, directed grainlines, allowance descriptors and
//! construction/closure/pocket intent with copy-addressed sewing graphs are implemented in [`ontology`]. Geometric correctness,
//! realized ease and target-profile binding validation remain explicit deferred obligations.
//! [`design`] owns structural inputs/Pieces/copies/topology and named parameter metadata with
//! current identity/copy/target checks, private garment composition and authorized metadata execution.
//! Full runtime resource, storage and application integration remain later work.
//! Canonical length declarations retain authored state/source/provenance in [`value`], without
//! default unknowns or cached derived results; registry/evidence proof remains Design/G4.
//! Shared [`name::MachineToken`] validates stable ASCII identifiers without deriving display labels.
//! Borrowed formula lexing/expression/ordered-statement syntax, literal normalization and canonical
//! expression/statement/recipe identity, sourced metadata and initial namespaces, exact reads, ordered scopes, operator/built-in/selector signatures and typed wanted catalogs are
//! implemented in [`recipe`]. Complete static recipe validation, exact numeric operators and binding
//! conversion, typed command requests and authorized Piece-list reads are available. Measurement/Material/
//! Notch metadata previews, atomic groups, replay/audit and bounded actual undo/redo execute through the
//! command bus. Direct/source-backed/Design scalar recipe execution, fresh replay and canonical stored
//! bindings have focused native acceptance. [`construction`] retains typed point/line/center-arc/Bezier declarations and offset/grainline input binding,
//! scalar kind proofs, explicit slash/spread sources/amounts, mirror-copy inputs, admitted walk mark requests,
//! constrained truing/join input association,offset candidate admission/queries/declaration binding and original dependencies; geometry/full integration remain pending. Broader/platform
//! verification stays source-scoped or owned by the integration and pre-push gates.
//!
//! What lands here, and when:
//!
//! | Module | Contents | Leaf |
//! | --- | --- | --- |
//! | `ontology` | identity, exact parameters, point/range topology resolution and repairs; structural pieces/copy plans, semantic notches, grainlines, allowances, construction/closure/pocket intent and sewing graphs; all four structural families complete | `G1-SLICE.3a`/`.3b`/`.3c` |
//! | `value` | canonical length declarations with authored state/source/provenance; no unknown fallback or cached derived result | `G1-SLICE.4a.1` |
//! | `name` | immutable ASCII lower-snake tokens, refusing grammar keywords; binding authority stays with the namespace owner | `G1-SLICE.4a.2a` |
//! | `recipe` | syntax/identity, static checking, exact sourced scalar execution, fresh replay and canonical stored bindings; geometry/full integration pending | `G1-SLICE.5` |
//! | `construction` | drafting inputs/scalar proofs,walk mark-pair admission,truing/join and semantic source binding; no evaluated geometry | `G1-SLICE.5f.1` |
//! | `command` | typed requests, authorization, metadata preview/atomic groups/history/undo; other handlers pending | `G0-CONTRACT.17`, `G1-SLICE.6` |
//! | `uncertainty` | known / assumed / unknown / preference / derived states and their artifact effects | `G0-CONTRACT.4`, `G4-PROFILES.7` |
//!
//! Entity numerical storage uses [`sc_units`]: lengths are signed `i64` micrometres and entity
//! directions are normalized `i64` microdegrees. Formula literals instead retain unsigned `u128`
//! canonical magnitudes with separate unary syntax and raw angle turns. Numeric binding conversion
//! checks signed `i64` after one rounding; admitted scalar recipes reuse this exact binding contract.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod ontology;

/// Actual structural/input ownership and scalar formula execution; whole application readiness remains owned.
pub mod design;

/// Typed inputs, authorization and atomic metadata execution/history; other handlers remain owned.
pub mod command;

/// Stable machine identifiers, distinct from localized display names and scalar text values.
pub mod name;

/// Formula syntax/identity, static checking, sourced scalar execution, replay and stored-binding bytes.
pub mod recipe;

/// Typed drafting declarations and static argument proofs, without geometric execution.
pub mod construction;

/// Canonical length declarations with authored state and required provenance references.
pub mod value;

/// The schema version of the canonical project format this crate will read and write.
///
/// Declared before project-directory serialization exists, so the first project file carries
/// a version and the migration path (roadmap §4.5) has something to migrate *from*. Expression
/// canonical bytes are a separate identity contract; they are not a versioned project envelope.
pub const SCHEMA_VERSION: u32 = 0;

/// The specification this crate implements, as a repository-relative path.
///
/// Kept as a constant so a test can assert the chapter still exists — a spec chapter that moves or is
/// renamed should fail a build, not silently orphan the code that claims to implement it.
pub const ONTOLOGY_SPEC: &str = "docs/book/src/spec/ontology.md";

#[cfg(test)]
mod tests {
    use super::{ONTOLOGY_SPEC, SCHEMA_VERSION};

    #[test]
    fn the_project_schema_starts_at_zero_before_project_serialization() {
        assert_eq!(SCHEMA_VERSION, 0);
    }

    #[test]
    fn the_numerical_contract_is_a_real_dependency() {
        // A smoke assertion that the foundation is wired: 1 cm is 10 000 µm.
        let cm = sc_units::Length::from_rational(1, 1, sc_units::Unit::Centimetre)
            .expect("1 cm is inside the declared domain");
        assert_eq!(cm.as_micrometres(), 10_000);
    }

    #[test]
    fn the_spec_path_is_repository_relative() {
        // Directive §12: no checkout-specific absolute paths, and a crate must not encode one.
        assert!(
            std::path::Path::new(ONTOLOGY_SPEC).is_relative(),
            "{ONTOLOGY_SPEC} must be repository-relative"
        );
        assert!(!ONTOLOGY_SPEC.starts_with('/'));
    }
}
