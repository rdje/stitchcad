# Introduction

**StitchCAD** is pattern-engineering software for garment makers. This book is its public
documentation surface: the specification of what the tool means, the guides for using it, and the
reference for the formats it writes.

## The idea in one paragraph

A garment design in StitchCAD is not a drawing. It is a **construction recipe**: a measurement table,
a formula graph over those measurements, and an ordered sequence of drafting operations that produce
pieces, seams, darts and ease relationships. Geometry is *derived* from that recipe, so the same
design can be re-evaluated for a different body or a different size without redrawing anything. To
turn a design into files a factory can cut, StitchCAD uses a **Factory Profile** — a versioned bundle
of typed parameters and constraints, each carrying its own evidence — and the artifact generator
serializes the result to industry formats (DXF in AAMA and ASTM dialects, HPGL/PLT, PDF, tech pack) as
an immutable release package with a manifest.

Two commitments shape everything else:

- **Unknown facts stay unknown.** A parameter is a known fact (with scoped evidence), an unknown fact
  (requiring observation), a selectable design choice, an overridable preference, or a derived value.
  Unknown facts are never silently defaulted into geometry, and a policy matrix decides which
  artifacts they block. A plausible guess that produces an unusable pattern is worse than a question.
- **Nothing is asserted without evidence.** Compatibility with a cutting room is a claim, and a claim
  carries its scope: which target system, which version, which import settings, which artifact, who
  checked, when, with what result. One factory's acceptance is evidence for that envelope, never a
  general certificate.

## Who this book is for

| Reader | Start here |
| --- | --- |
| A patternmaker wanting to know what the tool will do | This page, then the specification part |
| A sewing or factory expert asked to author a profile | The specification part, and the profile guide when it lands |
| An AI agent driving the tool | The command-API and MCP chapters when they land — the API returns stable diagnostic codes, never prose to parse |
| A contributor | The repository's `README.md`, `ROADMAP.md`, `docs/TASK_TREE.md` and `COMMIT.md` |

## What is true right now

The project is in gate **G0** — the product-and-semantic-contract gate. The chapters in the
specification part are being written from `ROADMAP.md` and are **normative contracts for
implementation**, not descriptions of shipped behavior. Where a chapter states a rule, the rule is
what the code must do when the code exists; the repository's `LIVE_STATUS.md` is the authoritative
record of what has actually been built, and each specification chapter names the gate that implements
it.

That distinction is deliberate and it is enforced by how the project works: nothing changes without a
task-tree leaf owning it, every code change lands with tool-backed evidence, and this book is updated
in the same commit as the change that affects it.

## How the book is organized

- **Specification** — the normative contract: glossary, units and tolerances, the garment ontology,
  the supported envelope, both instantiation paths, size sets, the command layer, release and
  approval, interchange dialects, the formula language, internationalization, the measurement
  standards the model draws on, and the reference garment that every conformance suite is built
  around.
- **Guides** — task-oriented: drafting a garment, grading it, authoring a factory profile, exporting
  a release package, driving the tool from an agent. These appear as the features they describe exist.
- **Reference** — formats, diagnostic codes, command list. Also appears with the implementation.

Chapters are added as they are written; the plan they follow is owned by the `G0-CONTRACT` task-tree
in the repository, which is the authority for what is due and in what order.
