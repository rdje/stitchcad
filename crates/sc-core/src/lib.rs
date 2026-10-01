//! `sc-core` — the garment ontology, the construction recipe and the command bus.
//!
//! **Status:** identity, point/range reference resolution, structural pieces/copy plans, semantic
//! notches, directed grainlines, allowance descriptors, construction/closure intent and
//! copy-addressed sewing graphs are implemented in [`ontology`]. Geometric correctness,
//! realized ease and target-profile binding validation remain explicit deferred obligations.
//! The core cross-builds to `wasm32-unknown-unknown`; recipe and command-bus implementation follow.
//!
//! What lands here, and when:
//!
//! | Module | Contents | Leaf |
//! | --- | --- | --- |
//! | `ontology` | identity, exact parameters, point/range topology resolution and repairs; structural pieces/copy plans, semantic notches, grainlines, allowances, construction/closure intent, sewing graphs; other objects follow | `G1-SLICE.3a`/`.3b`/`.3c` |
//! | `recipe` | the formula graph and ordered drafting operations, evaluated in one deterministic pass | `G0-CONTRACT.9`, `G1-SLICE.5` |
//! | `command` | the typed command bus: atomic groups, preview/commit, revision preconditions, idempotency, undo granularity | `G0-CONTRACT.17`, `G1-SLICE.6` |
//! | `uncertainty` | known / assumed / unknown / preference / derived states and their artifact effects | `G0-CONTRACT.4`, `G4-PROFILES.7` |
//!
//! The numerical contract this crate builds on is [`sc_units`], already implemented: every length is
//! an `i64` count of micrometres, every angle an `i64` count of microdegrees, and no comparison is
//! written without naming its tolerance class.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod ontology;

/// The schema version of the canonical project format this crate will read and write.
///
/// Declared now, before any serialization exists, so that the first project file ever written carries
/// a version and the migration path (roadmap §4.5) has something to migrate *from*.
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
    fn the_schema_starts_at_zero_before_any_format_exists() {
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
