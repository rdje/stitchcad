# Glossary: profiles, uncertainty and release

> One part of the [glossary](../glossary.md). Rows are alphabetical. The **canonical object** column
> names the chapter and clause that specifies the term normatively — this glossary defines the word,
> it never restates the rule. A ⚠ marks a **safety-relevant** term: one whose mistranslation or
> misreading causes a wrong cut or a factory rejection. Most of these terms are specified by the
> [release and approval](../release-contract.md) chapter, and the profile-side ones by roadmap §8 until
> `G4-PROFILES` writes them.

| Term | What it means | Canonical object | Also called | Machine token |
| --- | --- | --- | --- | --- |
| acceptance state | one rung of the ladder an artifact climbs: generated → checked → inspected → imported → evaluated → approved | [release §5](../release-contract.md) | maturity level, validation stage | — |
| approval ⚠ | a human act, bound to a package's identity; an agent may prepare evidence but never approve | [release §3](../release-contract.md) | signoff, release approval, *Freigabe* | → `approve` |
| artifact policy matrix | the table deciding what an unresolved unknown blocks, per artifact kind | [release §8](../release-contract.md) | export policy, unknown policy | — |
| assumed ⚠ | a value taken on a recorded human assumption: exportable, but it is not evidence | [ontology §5](../ontology.md) | recorded assumption, "taken as" | `assumed` |
| conservative default | a plausible value substituted for an observation — **forbidden** by this product's contract | roadmap §8.3 | safe default, fallback | — |
| candidate package | a package the generator produced and no human has approved; generation makes one, approval binds to its identity | [release §1](../release-contract.md) | candidate, unapproved package | — |
| dependency closure | the set of unknowns that actually affect one requested artifact, computed per artifact | [release §8](../release-contract.md) | impact set, relevance closure | — |
| derived | a value computed from others, whose uncertainty state follows its inputs | [ontology §5](../ontology.md) | computed, calculated | `derived` |
| diagnostic ⚠ | a message with a stable code, typed arguments and units; never prose an agent must parse | roadmap §7.8 · specified by `G0-CONTRACT.16` | error message, finding, warning | diagnostic code |
| disposition | the recorded human decision about an unresolved unknown at release time | [release §8](../release-contract.md) | resolution, waiver (⚠ not the same) | — |
| equivalence report | the per-quantity comparison of the two instantiation paths, carried as release evidence rather than logged | [instantiation paths §6](../instantiation-paths.md) | path comparison, divergence report | — |
| evidence ⚠ | a scoped record: target system, version, import settings, artifact hashes, procedure, observer, date, result | [release §5](../release-contract.md) | proof, validation record, *Nachweis* | `evidence` |
| Factory Profile ⚠ | the versioned bundle of typed parameters and constraints that turns a design into a factory's bytes | [release §2](../release-contract.md) | profile, factory settings, target profile | `profile_id` |
| hard restriction | a constraint that outranks every factory override — a safety or design limit | roadmap §8 | hard constraint, non-negotiable | `hard_restriction` |
| known | a fact with scoped evidence behind it; the only state that needs no qualifier on export | [ontology §5](../ontology.md) | established, verified | `known` |
| manifest | the immutable record of what a release package contains and what was true when it was built | [release §2](../release-contract.md) | bill of the package, release record | — |
| package completeness | the check that every piece, size, multiplicity, material and companion file is actually there | [release §4](../release-contract.md) | completeness check, "nothing missing" | — |
| precedence | the order conflicts are resolved in: hard restriction > factory override > preference > default | roadmap §8 | composition order, priority | `precedence` |
| package identity | the digest of a package's canonical manifest, including every artifact hash — the only thing an approval binds to | [release §3](../release-contract.md) | package hash, identity | — |
| preference | an overridable default that carries its provenance, unlike an invented value | [ontology §5](../ontology.md) | default, house preference | `preference` |
| production release | the artifact class a factory cuts from, and the strictest column of the policy matrix | [release §8](../release-contract.md) | release, production package, *Produktionsfreigabe* | — |
| provenance | where a value came from, recorded with it — a source, a profile, an assumption, a formula | [ontology §2.2](../ontology.md) | origin, lineage, source, *Herkunft* | `provenance` |
| release package | the immutable set of artifacts plus manifest that a factory receives | [release §1](../release-contract.md) | package, delivery, signoff package | — |
| stale-ification ⚠ | any input or artifact change makes a prior approval void; approval is never inherited silently | [release §3](../release-contract.md) | invalidation, approval expiry | — |
| scope (of an approval) | the tuple a claim is true over: garment, material, target system, size range, artifact set | [release §6](../release-contract.md) | claim scope, tested envelope | — |
| sidecar | a provenance file exported beside an artifact, naming what was unresolved and what it would change | [release §8](../release-contract.md) | provenance sidecar, gap report | `sidecar` |
| target system | the specific vendor product and version a profile is written for, with its accepted artifacts | roadmap §8 | receiver, factory system, *Zielsystem* | `system` |
| unknown ⚠ | a fact that requires observation; it carries **no value**, and no solver invents one | [ontology §5](../ontology.md) | unresolved, missing, *unbekannt* | `unknown` |
