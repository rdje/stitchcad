# ADR-0004 — interchange is dialects, not a format: six axes, a closed registry, no universal package

- **Type:** `decision` (ADR-0004 in `ROADMAP.md` §5)
- **Date:** `2026-09-30` (absolute); the roadmap recorded the withdrawal and the corrected layer table at
  v0.2, and §11's G0 exit clause requires the ADR recorded, so this settles it before a writer exists
- **Status:** `active`
- **Owner / source:** leaf `G0-CONTRACT.10`, from roadmap §5 ADR-0004, §7.5, §11 G0 and Appendix A's
  disposition log; the normative text is `docs/book/src/spec/interchange-dialects.md`

answers: "why is there no single DXF export?" · "what is an export target?" · "why are R12 and R13 both polyline-only?" · "what happens to a withdrawn standard?" · "who decides cut-as-1 or sew-as-1?" · "why are SST and PST fields not specified?" · "what would reopen ADR-0004?"

## The decision

**Interchange is specified as dialects over a format, not as one format with options.** An export
target is a named tuple over six declared axes (layer naming, release, line semantics on layer 1,
allowance inclusion, grading carriage, entity policy), the registry of targets is **closed**, and a
request for a tuple nobody has validated is refused rather than attempted. ASTM D6673-10's withdrawal
is recorded and its numbered-layer convention is implemented as a **de-facto convention** with no
conformance claim; both naming modes are separate targets; R12 and R13 share one polyline-only entity
set; one BLOCK per piece with SST and PST mandatory on the ASTM path; three grading carriages, never
combined; and the cut/sew line semantics are a Factory Profile mapping recorded everywhere a reader
could look.

## Ten decisions, and the alternative each rejected

| Decision | Rejected | Why |
| --- | --- | --- |
| a target is a tuple over six declared axes, each with a party that resolves it | a writer with flags | a flag nobody validated is a claim about a receiver nobody tested; a registry row is a commitment with a G6 validation behind it |
| the registry is closed, and a tuple outside it raises `dialect_unregistered` naming the nearest target | best-effort export of any combination | combinatorial possibility is not a capability, and a "should work" artifact is how a factory gets a wrong cut |
| both naming modes are separate targets, and the named mode's loss of separation is declared in the chapter and in the loss report | one layer table behind a rename switch | seven AAMA names cover seventeen numbered layers, so the switch loses information; declaring the loss is what makes an import honest |
| one polyline-only entity set for both releases, arcs exact as bulges, Béziers tessellated at the T2 chordal bound | writing `ARC`/`CIRCLE` where a release permits them | two entity families for one boundary is two ways to disagree with a receiver, and the receiver set is legacy |
| SST and PST field content is **not** specified here | inventing a field table at G0 | the syntax is case-sensitive and receiver-specific; a table invented now would be a guess in a normative font, and G6 is the only oracle |
| the unit is declared in the receiver-config record and agreed with the receiver | inferring millimetres | DXF carries no intrinsic unit, so an inference is a silent default in the one place a wrong number becomes a wrong cut |
| an unresolved line semantics refuses the export | defaulting to cut-as-1 | the roadmap names the swap a top rejection cause; a default here is a wrong cut at somebody else's factory |
| layers 84–87 and the blank layer 12 are absent from the entity list | emitting placeholder curves | geometry no design authored is a fabrication, and a receiver that requires it is a finding with a name |
| three grading carriages, each a separate artifact and validation | one file carrying base-plus-rules and contours | a receiver reads one carriage; two is an ambiguity only that receiver can resolve, and Appendix A already refuses the universal package |
| the receiver-config record travels with the artifact and enters the manifest's identity | keeping it in a log | a dispute becomes a comparison of fields, and a finding is scoped to a receiver *and* a writer version |

## What was read, and what was not

- **`read-external`, read `2026-09-30`** at `https://en.wikipedia.org/wiki/AutoCAD_DXF`: Autodesk
  publishes the DXF specification *incompletely* ("Autodesk now publishes the incomplete DXF
  specifications online"), certain object types are undocumented, and the reference Autodesk publishes
  is dated from Release 14 (1998). Those three facts are why the oracle for a writer is **a receiver
  reading the file back** (G6) rather than a document, and why the entity set is small and legacy.
- **Attempted and not read**, recorded so the next session does not re-try them: `help.autodesk.com`'s
  DXF reference returned a page with no content (script-rendered), and `astm.org`'s D6673 page returned
  a distribution refusal.
- **Everything else stays `cited-from-roadmap`**: the numbered layer table, the seven AAMA names, the
  BLOCK/SST/PST requirement, R13 as D6673's carrier, and the withdrawal itself. Naming them and stating
  their role is permitted; quoting a clause is not, and nothing in the chapter does
  (`docs/book/src/spec/standards.md` §1 keeps that vocabulary closed).

## How to apply

- G2 writes **one** target — `dxf-aama-named`, the mode its exit clause names — and freezes goldens that
  compare the metadata blocks as well as the geometry.
- G6 validates receiver by receiver, scoped to a receiver and a writer version. A lapsed validation
  removes the registry row; it does not downgrade it to a warning.
- Never widen the entity set to land a receiver: a new entity family is a new axis value, a new row and
  a new validation, in one change.
- A new target arrives as a registry row **and** its validation together, or it does not arrive.
- The layer mapping in the chapter's third column is this project's, so when a receiver disagrees, the
  receiver is right about itself and the row changes — with the finding recorded against it.

## What would reverse it

Three conditions, each needing a recorded decision: (1) a validated receiver set that requires curve
entities, which adds an `entity_policy` value and its own tessellation-free loss report; (2) the
standards seat obtaining D6673's text, which lets §3's table carry clause citations as `read-in-repo`
— and a contradiction between the text and the table is a defect owned by this chapter; (3) a revival
of the standard, which turns "de-facto convention" into a citation with a current status and re-opens
the withdrawal record.
