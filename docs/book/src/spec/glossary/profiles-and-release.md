# Glossary: profiles, uncertainty and release

> One part of the [glossary](../glossary.md). Rows are alphabetical. The **canonical object** column
> names the chapter and clause that specifies the term normatively — this glossary defines the word,
> it never restates the rule. A ⚠ marks a **safety-relevant** term: one whose mistranslation or
> misreading causes a wrong cut or a factory rejection. Most of these terms are specified by
> `G0-CONTRACT.12` (release and approval); until that chapter lands, the roadmap clause is cited.

| Term | What it means | Canonical object | Also called | Machine token |
| --- | --- | --- | --- | --- |
| acceptance state | one rung of the ladder an artifact climbs: generated → checked → inspected → imported → evaluated → approved | roadmap §9 · specified by `G0-CONTRACT.12` | maturity level, validation stage | — |
| approval ⚠ | a human act, bound to a package's identity; an agent may prepare evidence but never approve | roadmap §9 · specified by `G0-CONTRACT.12` | signoff, release approval, *Freigabe* | → `approve` |
| artifact policy matrix | the table deciding what an unresolved unknown blocks, per artifact kind | roadmap §8.2 · specified by `G0-CONTRACT.12` | export policy, unknown policy | — |
| assumed ⚠ | a value taken on a recorded human assumption: exportable, but it is not evidence | [ontology §5](../ontology.md) | recorded assumption, "taken as" | `assumed` |
| conservative default | a plausible value substituted for an observation — **forbidden** by this product's contract | roadmap §8.3 | safe default, fallback | — |
| dependency closure | the set of unknowns that actually affect one requested artifact, computed per artifact | roadmap §8.2 · specified by `G0-CONTRACT.12` | impact set, relevance closure | — |
| derived | a value computed from others, whose uncertainty state follows its inputs | [ontology §5](../ontology.md) | computed, calculated | `derived` |
| diagnostic ⚠ | a message with a stable code, typed arguments and units; never prose an agent must parse | roadmap §7.8 · specified by `G0-CONTRACT.16` | error message, finding, warning | diagnostic code |
| disposition | the recorded human decision about an unresolved unknown at release time | roadmap §9 · specified by `G0-CONTRACT.12` | resolution, waiver (⚠ not the same) | — |
| equivalence report | the per-quantity comparison of the two instantiation paths, carried as release evidence rather than logged | [instantiation paths §6](../instantiation-paths.md) | path comparison, divergence report | — |
| evidence ⚠ | a scoped record: target system, version, import settings, artifact hashes, procedure, observer, date, result | roadmap §8 · specified by `G0-CONTRACT.12` | proof, validation record, *Nachweis* | `evidence` |
| Factory Profile ⚠ | the versioned bundle of typed parameters and constraints that turns a design into a factory's bytes | roadmap §8 · specified by `G0-CONTRACT.12` | profile, factory settings, target profile | `profile_id` |
| hard restriction | a constraint that outranks every factory override — a safety or design limit | roadmap §8 | hard constraint, non-negotiable | `hard_restriction` |
| known | a fact with scoped evidence behind it; the only state that needs no qualifier on export | [ontology §5](../ontology.md) | established, verified | `known` |
| manifest | the immutable record of what a release package contains and what was true when it was built | roadmap §9 · specified by `G0-CONTRACT.12` | bill of the package, release record | — |
| package completeness | the check that every piece, size, multiplicity, material and companion file is actually there | roadmap §9 · specified by `G0-CONTRACT.12` | completeness check, "nothing missing" | — |
| precedence | the order conflicts are resolved in: hard restriction > factory override > preference > default | roadmap §8 | composition order, priority | `precedence` |
| preference | an overridable default that carries its provenance, unlike an invented value | [ontology §5](../ontology.md) | default, house preference | `preference` |
| production release | the artifact class a factory cuts from, and the strictest column of the policy matrix | roadmap §8.2 · specified by `G0-CONTRACT.12` | release, production package, *Produktionsfreigabe* | — |
| provenance | where a value came from, recorded with it — a source, a profile, an assumption, a formula | [ontology §2.2](../ontology.md) | origin, lineage, source, *Herkunft* | `provenance` |
| release package | the immutable set of artifacts plus manifest that a factory receives | roadmap §9 · specified by `G0-CONTRACT.12` | package, delivery, signoff package | — |
| stale-ification ⚠ | any input or artifact change makes a prior approval void; approval is never inherited silently | roadmap §9 · specified by `G0-CONTRACT.12` | invalidation, approval expiry | — |
| target system | the specific vendor product and version a profile is written for, with its accepted artifacts | roadmap §8 | receiver, factory system, *Zielsystem* | `system` |
| unknown ⚠ | a fact that requires observation; it carries **no value**, and no solver invents one | [ontology §5](../ontology.md) | unresolved, missing, *unbekannt* | `unknown` |
