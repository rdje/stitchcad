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

Fresh authority also binds its owner: absent/pending/active at the baseline, completed in the resulting
task tree. A previously completed owner cannot be reused for a new record. One bounded current record
replaces a growing authorization ledger; exact content digests prevent banking future headroom even
for zero-count-delta edits. "Signed delta" means signed integer file/line/byte changes, not invented
cryptographic or human approval. First-parent Git binding works for merges without treating a prior
content match as new authority. Existing Python3.9+ CI/full-history checkout supports this local tool.

answers: "what does the maintained-book reader prove before its default-gate installation?"

0416 implements the bounded UTF-8 JSONL header/one-record contract:8192 control bytes,6144 raw record
bytes, one record and fixed schema/limits. Parts/index keep their original bounds and complete
depth-three SUMMARY. Exact framed content/counts/native baseline, signed integer deltas and newly
completed task ownership refuse same-size substitution and stale/reused/banked authority. Optional
missing Git paths differ from Git errors. Caller Git configuration/directory/index/object overrides
are excluded without mutating the parent; native children have bounded output/time and are reaped.
Candidate/native23 plus optimized2 controls pass, with independent wc/SHA calibration. Tests never
initialize another Git repo; their fresh owner exists only in ordinary target candidates. This is
reader proof only at0416: class/caps/gate then remained; installation follows below.

Native manifest duplicates/links/submodules refuse; watchdog startup failure still reaps the actual
owned child. Native-contract4 rerun checks the direct first-parent query. Git and the OS null device
are host dependencies; no project-owned stores/logs are written there. Uninstalled default invocation
refuses missing control, rather than advertising gate adoption or committed-state verification.

answers: "how does atomic book-change recording preserve prior authority and prevent workspace/source escapes?"

0417 installs the local maintained-book record and existing size-check/CI path atomically. Source/
output planning precedes local temporary creation; one replacement publishes only the canonical JSONL,
with fresh HEAD/full inputs/destination checks. Native authoring8/reader23/O2 controls pass. Actual
committed-mode observation is after commit, then .book.3 closes scoped adoption; no remote CI claim.
D278 repairs existing root/Git/part preflight: canonical script root, target-only ordinary fixture
roots, guarded controls and all parts before native wc. Old0dc reads ../fixture/pass0, fixed refuses1;
source-boundary/regular/overlap controls remain. Native Git ignores caller context and disables
filesystem-monitor/submodule recursion. No other Git repo is initialized or amended. Numeric fixture
overrides never select a replacement canonical book. Parts/index/depth remain fixed; only explicitly
inapplicable aggregate targets change information role, not a raised numerical ceiling. The neutral
package/version-retention adoption remains separate; policy's neutral body is unchanged.

answers: "how do exact book digests survive Git newline conversion?"

0418 scoped acceptance observes actual committed verification at03460b0, then fixes D279/D280 before
closure. Native old measurement counts441 for440 content bytes plus CRLF; strip exactly one terminal
CR for width while native wc retains all bytes. Git autocrlf can otherwise normalize CRLF after the
authority hashes it. Root attributes now explicitly unset text/filter/ident/working-tree-encoding
for book Markdown and its JSONL only; the checker requires matching effective native attributes.
The writer freezes the root attribute source and rechecks before publication. Preserve source bytes,
including LF/CRLF, rather than silently rewriting prose. Native Git hash-object without object writes
and actual checker awk provide independent before/after controls; ordinary target fixtures use only
this repository's existing Git metadata read-only. Newline controls run in the existing probe suite.
Reader23/authoring8/newline3 pass; broader runtime/physical/release proof remains independently owned.

answers: "how does book authority follow canonical task siblings without reusing a completed owner?"

0421 fixes D281:root-only lookup refused a fresh linked-sibling owner and accepted an old completed
sibling owner copied into the root. Committed7b controls show both failures; bounded family lookup
checks the canonical root and same-Tree-ID hyphenated siblings. Current files must form its linked
local graph; native baseline lookup also retains unlinked historical members for no-reuse checks.
The fixed profile permits32 files/98304 bytes each, with bounded directory/Git listings. Authoring
freezes all family inputs and rechecks their census; late sibling edits cannot silently publish.
This extends the existing canonical sibling convention, not filename guessing over arbitrary docs.
All98 completed .6.4 nodes/proof pointers move as one exact live graph to G1-SLICE-garment.md, pinned
to native7b bytes and raw Git storage. No nodes disappear or become anonymous archival prose.
Reader23/author8 and focused task/native controls establish local scope; runtime/physical/release
proof remains separately owned. The neutral doctrine body and book part/index limits are unchanged.

D282 in the same slice:sealed Markdown history also requires raw Git bytes. Native autocrlf changes
unprotected CRLF object identity; exact source571 ends with a blank line that default whitespace
checking refuses. Archive-only attributes disable transformations and allow blank EOF. Native
newline/storage5 controls retain raw bytes and accept that data, while ordinary source trailing
whitespace still refuses through Git apply --check without creating files. No sealed record changes.

## Paired work-memory containment0495

answers: "how does paired task planning grow without another map row or a raised root ceiling?"

Use the existing canonical execution sibling,which had126 lines while root had1196/1200. Retain
whole0494 in649 and the exact native70-line/5726B branch in650. Copy the working branch byte-exact
after the one authorized retirement,then evolve live plans there. All258 original root IDs stay live;
24 current paired IDs are unique. A new native source/family guard pins the original Git extent,
checks unique live nodes and preserves done states;all12 sibling/source controls pass. Root navigation
and current task authority follow the existing family contract. No map row,cap or Rust runtime code changes.

## Staged book authority0519

0518 exposed a sequencing defect:authority preparation yielded,but staging started before its
terminal result. Commit00e1d74 kept the preceding control while the working-tree verifier saw the
fresh unstaged record. Preserve that exact929B record as0518_unstaged_authority.jsonl in the live
doc-size artifact family;repair under a newly completed owner rather than reuse closed authority.
The final pre-commit --index reader validates exact native staged book/control/registry/task blobs,
ordinary stage-zero modes and a stable listing. It reads immutable object IDs with bounded output;
no index/temp repository is written. Old staged control/new book now refuses target,rc=2. Seven
finite index controls and two existing reader controls pass;the standing probe suite owns reruns.
