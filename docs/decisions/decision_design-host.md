# Design metadata uses a readonly host port without reversing crate dependencies

- **Type:** `decision`
- **Date:** `2026-10-27`
- **Status:** `active`
- **Owner / source:** `G1-SLICE.6.4.2.d.4.1`; core design host, higher measurement input owners.

answers: "how can Design own measurement metadata without a circular dependency?" · "does host admission prove a measurement binding?" · "can a copied canonical declaration authorize a Design source?" · "how do owned metadata objects avoid self-referential contexts?" · "how does core validate an external SizeSet pin without a reverse dependency?" · "what does DesignInputs admission prove?"

sc-core owns canonical declarations and defines DesignMetadataHost. Higher crates implement the port
using their actual immutable metadata. DesignMetadata owns that host and an identity-only census;
it stores no borrowed context, cloned scalar declaration, numeric value or uncertainty state. Shared
access supplies no mutable host. Each real implementation must preserve a stable immutable snapshot,
complete authored ownership census and exact bindings, and invoke its actual current validators.
This is a trusted integration contract: core cannot independently prove arbitrary provider semantics.

Core checks duplicate metadata identities, canonical-input disjointness and observable census changes.
A measurement query validates the whole metadata scope, checks the requested metadata role, asks the
host for its explicit binding, and requires pointer membership in the supplied dedicated canonical
length inventory. Equal identity/content on a detached declaration is insufficient. That guard proves
object membership; semantic correctness of the metadata-to-target relation belongs to the actual host
implementation. Canonical replacement triggers current validation, without remembered success.

MeasurementInputs now implements the port using actual owned records and original validators.
Full Design assembly and remaining cross-component identity scopes retain their task children.
Logical profile refs
and evaluation context pins keep their G4 deferrals even if canonical numeric ids happen to coincide.
No lookup through a label, fallback value, profile selection, readiness or execution follows admission.

Focused public native and compile-fail contracts exercise observable guards. Broader feature and
integration verification remains with G1's existing owners. The ownership decision is durable now;
subsequent implementation must record its narrower actual acceptance scope in the task tree/book.

Common SizeSetMetadata now owns membership/axes/break metadata separately from canonical inputs and
measurement/Ease sources. Its higher-ranked callback reconstructs temporary actual contexts, checks
all source identities/current validators before invocation and prevents borrowed contexts escaping.
GarmentSizeSet and MtmSizeSet now own actual chart-family sources over that common component.
SizeSetInventory now owns explicit family sources with whole-current exact-reference checks.
DesignSources now owns actual measurement/set metadata, exposing its complete typed core catalog.
The DesignSizeSetHost facet delegates exact external pins to the actual current inventory; numeric
identity presence supplies no replacement metadata. Core remains independent of higher schemas.
Full Design composition follows; the common component alone is not a complete SizeSet, and Design
still holds an external reference. Physical Profile resolution remains G4.

DesignInputs now owns implemented core structure/parameters/recipe/Materials and checked higher
metadata over the structure's single canonical inventory. Complete cross-component ownership ids
precede actual current parameter/Material/Piece/source validation. This is an input composition,
not full Design selection/name/sewing/garment/readiness proof; those remain explicit following work.
