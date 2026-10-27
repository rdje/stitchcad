# Live-document containment is adopted in proportion to the repository, with deferrals named

- **Type:** `decision`
- **Date:** `2026-09-29` (absolute)
- **Status:** `active`
- **Owner / source:** repo-local workflow, leaf `SPINE.4.1`; adopts the director's external
  containment doctrine and its adoption guide (session directive §18)

answers: "do we use the live-document size containment doctrine?" · "why is there no JSONL registry and no neutral size checker yet?" · "when do we adopt the full containment package?" · "what stops CHANGELOG.md or a task tree from growing without bound?" · "who sets the size ceilings and how?"

## The fact / decision

1. `LIVE_DOCUMENT_SIZE_CONTAINMENT.md` is adopted at the repository root: the neutral body verbatim,
   behind a fenced StitchCAD adoption note. This copy is authoritative; the external reference is
   read-only and is not an upstream.
2. **The adoption is partial, and the note says so.** Adopted now: the doctrine text, the surface
   inventory with lifecycle classes, measured pressure axes, health targets, inclusive enforcement
   ceilings, the routing-destination inventory, and one deterministic checker in the project doctrine
   slot (`SPINE.4.2`, `SPINE.4.3`).
3. **Deferred:** the neutral JSONL-registry checker package (a ~2 100-line interpreter plus the
   archive-descriptor, ledger-manifest, derived-state, ceiling-increase-authority and
   version-retention registries). Three triggers reopen the decision, and whoever hits one owns it:
   - a surface reaches its ceiling and cannot be trimmed → adopt the atomic partition / rollover /
     archive protocol and the archive-descriptor contract;
   - the inventory exceeds roughly 24 surfaces, or the awk checker becomes the limiting factor →
     evaluate the neutral checker and migrate the two TSV registries to its JSONL data plane;
   - a stored copy of a mechanically owned value needs an executed freshness oracle → adopt the
     derived-state contract family. **FIRED `2026-09-30` and discharged locally:** the `roadmap` row's
     transition-debt baseline blocked a legitimate revision (roadmap v0.3), so the debt column gained an
     `at=<revision>` axis that the checker executes — a baseline whose revision the file no longer declares
     is refused, which is the freshness oracle the trigger asks for. Adopted as a contract, implemented in
     the existing awk checker rather than by importing the neutral interpreter, whose other two triggers have
     not fired at that capture: `decision_revision-aware-containment-baseline.md`. Archive adoption later
     fired the first trigger: SPINE.19.2 and the v2 continuation enforce local descriptor/retention
     contracts. The neutral package and version-retention registry remain unadopted.
4. **Local parameters:** warn at 80 % of a health target; the enforcement ceiling is inclusive
   (equality passes, excess fails); a ceiling rises only by a recorded authority (a decision record
   plus the registry row citing it), never to land content. The data plane is bounded **TSV** parsed
   with awk, so no new interpreter enters the commit path.

## Why

Containment exists to stop a bounded file from routing its overflow into an unbounded neighbour — the
measured upstream failure was a status file that reached 1 547 057 bytes, 94.7 % of it dated changelog,
after a README cap displaced the pressure instead of removing it. This repository has that shape of
risk in front of it: 18 specification chapters, a changelog entry per slice, and task-tree files that
grow one evidence section per leaf (`docs/tasks/SPINE.md` was already 772 lines / 60 433 bytes at
adoption).

At initial adoption, partitioned archives, sealed rolling ledgers and freshness oracles were absent.
They now exist through local checks. That historical deferral rationale does not describe the current
repository. Full neutral-package and version-retention-registry adoption remain separate decisions;
0390 corrects the obsolete present-tense claim without claiming either has been implemented.

The proportionality call is recorded rather than left implicit because "we adopted the doctrine" and
"we adopted part of it" are different claims, and a reader who assumes the full package is enforcing
something would trust a gate that does not exist. Naming the deferrals with triggers converts a
silent gap into tracked work.

## How to apply

- Before growing a live document, find its row in `.doctrine/live_document_size/surfaces.tsv`
  (`SPINE.4.2`): lifecycle class, owner, health target, ceiling. No row means the surface is
  unclassified, and the checker refuses.
- A ceiling increase needs a decision record and a registry row citing it. Trimming the content is
  almost always the right answer instead; `MEMORY_ARCHITECTURE.md` §6 and `README_POLICY.md` both say
  never to raise a cap to fit content.
- When a trigger fires, adopt the corresponding part of the package and update the adoption note in
  the same commit — the note is the record of what is and is not enforced.
- Related: [[decision_adopted-external-policy-references]] (how external policy is adopted at all),
  [[decision_scaffold-sync-protects-project-content]] (why this file cannot be clobbered by a sync).

## Maintained product book — design adopted by G1-SLICE.6.5.1

answers: "how can the growing product book preserve readable examples without arbitrary aggregate cap increases?"

At d0481b5, the book occupies1199996/1200000 bytes and all80 allowed Markdown files; native file
census and LIVE-DOC-SIZE agree. Repeated prose tightening now consumes feature work and risks
readability. The source doctrine classifies unique maintained product/specification prose as
maintained_reference: govern exact aggregate changes, keeping semantic parts and complete mandatory
navigation bounded. This is the book's information role, not chronological evidence for an archive.

Adopt that contract locally under .6.5.book: bounded data-only control, exact current content/counts,
baseline revision/tree identity and signed per-change delta tied to a fresh owning task. Refuse
inexact/stale/reused/banked authority, same-size content substitution, missing/duplicate/index-external
chapters, links/foreign paths and oversized parts/index. Preserve all readable current chapters,
examples, URLs, per-part700L/40960B/440B and SUMMARY120L/8192B/240B bounds. Fix aggregate
file/line/byte targets as explicitly inapplicable only when the full replacement guard lands atomically.
Until then, the existing80-file/20000-line/1200000-byte ceilings stay enforced; no cap is raised.
Full neutral-package/version-retention adoption remains separate. Source policy is already local;
no project dependency or writes cross repository boundaries. Implementation/refusal/feature acceptance
are three owned leaves before new bus documentation; this section is design authority, not a claim
that the new guard has passed or been installed.

The replacement checker is read-only and uses this repository's Git history for freshness. Candidate
mode requires the exact HEAD baseline; committed mode binds the latest authority-file change to its
first parent, then checks the current book digest. Unchanged book content may retain its authority;
returning an old control or changing equal-size content cannot bypass freshness. Authoring writes
only the project-local control, never source chapters; CI runs the resulting-tree check unconditionally.
Tests use ordinary candidate directories under target and read this repo's Git; no other Git repo is
created or amended. Committed-mode proof is scheduled after atomic adoption, never simulated as an
observed commit verdict. Exact control schema/bounds, index grammar and hash framing land in .book.1;
.book.2 wires producer/checker/classification atomically; .book.3 closes actual committed acceptance.
