# Measurement and size standards

> **Status:** normative specification, gate **G0** (roadmap §11 G0 exit clause: "measurement/POM standards
> identified"). This chapter is the registry of every external standard the model draws on, and the record
> of what is known about each. It owns the verification other chapters defer here — the
> [ontology](ontology.md) §8, the [reference skirt](reference-skirt.md) §11 and
> [size sets](size-sets.md) §8 all name this chapter as the owner.

**This chapter claims conformance to nothing.** StitchCAD does not assert that its garments, its files or
its measurements conform to any external standard, and no chapter of this book may say so. What the
product does is *draw on* standards: for body-measurement landmarks and procedures, and for size
designation systems. Drawing on a standard is a claim about the standard's content, and a claim carries a
verification status — which is what this chapter exists to record honestly.

## 1. The three claim statuses

Every statement in this book about an external standard carries exactly one of these, and the closed
vocabulary is normative:

| Status | Meaning | What it permits |
| --- | --- | --- |
| `read-in-repo` | the document's text has been read in this repository and the claim cites what was read | a clause number, a table, a definition may be quoted |
| `cited-from-roadmap` | the claim comes from `ROADMAP.md`, which records its own peer-review provenance | the standard may be *named* and its role stated; its content may not be quoted |
| `unverified-with-owner` | nobody has established it; a named leaf owns doing so | the claim may be listed as a requirement, never as a fact |

Two rules follow, and they are the reason this chapter is short:

1. **No clause number, table number or quoted definition of a standard appears anywhere in this book
   unless its status is `read-in-repo`.** None is, at present. So none appears.
2. **A standard is data, not authority.** A landmark, a procedure or a designation enters the model as an
   entry with provenance and an uncertainty state (ontology §2.1, §5). "ISO 8559 says so" is not a
   substitute for a measured value with a scoped evidence record, and it never satisfies an `unknown`.

## 2. The registry

The first cell of a row is the designation's **normalized form** (`ASTM D6673`, not `ASTM D6673-10`),
because the census in §7 matches designations used anywhere in the book against these keys.

| Designation | Status | Owner | Role in this model |
| --- | --- | --- | --- |
| ISO 8559 | `cited-from-roadmap` (§3.1) | the domain expert — seat **vacant** ([governance §8.1](../governance.md)) | the landmark and procedure semantics body measurements draw on |
| ASTM D5219 | `cited-from-roadmap` (§3.1) | the domain expert — seat **vacant** | terminology for body measurement landmarks, alongside ISO 8559 |
| EN 13402 | `cited-from-roadmap` (§3.4) | `G0-CONTRACT.6`, data via `.14` | a size-designation system the model must be able to express |
| ASTM D5585 | `cited-from-roadmap` (§3.4) | `G0-CONTRACT.6`, data via `.14` | a size-designation and figure-table system for missy sizes |
| ASTM D6673 | `cited-from-roadmap` (ADR-0004) | `G0-CONTRACT.10` | the withdrawn specification whose numbered-layer DXF convention cutting rooms enforce |
| AAMA | `cited-from-roadmap` (ADR-0004) | `G0-CONTRACT.10` | the named-layer DXF interchange convention |

Every row's status is `cited-from-roadmap`, with the roadmap clause that carries it in parentheses. That is
not a placeholder — it is the honest state of this repository, and it is why §1's first rule leaves the book
without a single quoted clause.

### 2.1 ISO 8559

- **Adopted here:** the *distinction* between a body measurement and a garment point of measure, and the
  requirement that each carries a landmark and a procedure as references rather than prose.
- **Not adopted:** any specific landmark list, procedure text or measurement value. None has been read here.

### 2.2 ASTM D5219

- **Adopted here:** the requirement that a landmark is a named, referenceable entity with an identity.
- **Not adopted:** its terminology as such, until the document has been read.

### 2.3 EN 13402

- **Adopted here:** the `en13402` value of `size_system` — that is, the *mechanism* of labels, order, chart
  and breaks that [size sets §8](size-sets.md) specifies independently of any standard's content.
- **Not adopted:** any designation-to-measurement mapping, and the size pictogram.

### 2.4 ASTM D5585

- **Adopted here:** the `astm_d5585` value of `size_system`, the same mechanism.
- **Not adopted:** its figure tables, until read. The reference fixture's body measurements are declared
  constants of this repository and are copied from no standard (§4).

### 2.5 ASTM D6673

- **Adopted here:** the record that D6673-10 is withdrawn (January 2019, no replacement) and that StitchCAD
  implements a de-facto convention without claiming a current standard.
- **Not adopted:** any claim of conformance to it.

### 2.6 AAMA

- **Adopted here:** that a named-layer mode and a numbered-layer mode both exist, and are separately
  validated receiver by receiver rather than merged into one package.
- **Not adopted:** the layer table as a quoted fact. `G0-CONTRACT.10` records it with its own status.

## 3. What the model needs from a standard

Four requirements, stated so that reading a standard later has something to satisfy rather than a blank
page. Each is a requirement on *this model*, not a claim about any document:

1. **Landmark identity.** A body measurement names the two points it runs between, as references with
   stable identity (ontology §2.1). A standard supplies candidate landmarks; the model supplies the
   identity, and a measurement without one is rejected rather than defaulted.
2. **Procedure repeatability.** A measurement names how it is taken — tape position, posture, tension,
   which side — as a reference to a documented procedure. Two people taking "the chest" differently is the
   dispute a procedure reference exists to prevent.
3. **Designation mapping.** A size label maps to a chart of garment POMs, never to a measurement by itself
   ([size sets §8](size-sets.md)). A designation system therefore supplies *labels and their meaning*, and
   the meaning enters as chart data with provenance.
4. **Tolerance semantics.** A standard's tolerance, where it has one, is a *published acceptance limit*,
   which in this model is the physical-acceptance class — the factory's number, never ours
   ([units §3](units-and-tolerances.md)). A standard's tolerance is never silently adopted as a numerical,
   geometric or format tolerance.

## 4. The deferral ledger

Other chapters handed their standards claims here. Each deferral is recorded with this chapter's
disposition, so a deferral is a routing decision with an outcome rather than a way of not answering. These
are bounded prose entries, not table rows, for the reason the defect census in `docs/tasks/PLANNING.md`
gives: a long cell in a single-line row is invisible pressure under a line cap.

- **[ontology §8](ontology.md)** deferred *"ISO 8559 / ASTM D5219 describe body measurement landmarks and
  procedures"*. **Disposition:** registered, `cited-from-roadmap`. The ontology's own requirement — a
  landmark and a procedure per measurement, as references rather than prose — stands without the standards'
  content, so nothing in that chapter waits on a document nobody has read here.
- **[reference skirt §11](reference-skirt.md)** deferred *"reconcile the fixture's declared constants with
  the ISO 8559 / ASTM D5585 landmarks"*. **Disposition: still open.** The fixture's four body measurements
  are declared constants of this repository, chosen as a plausible mid-range woven base and copied from no
  standard. Reading the standards would confirm or replace them; until then they stay `assumed`, and §11 of
  that chapter says so.
- **[size sets §8](size-sets.md)** deferred *"what the EN 13402 and ASTM D5585 mappings must express"*.
  **Disposition: discharged.** The mechanism is normative in that chapter and independent of the standards'
  content; the data is `unknown` until sourced, and an `unknown` blocks what the artifact policy matrix
  says it blocks.
- **[the glossary](glossary.md)** deferred *"trade synonyms in other languages and other CAD systems"*.
  **Disposition: routed to `G0-CONTRACT.14`.** Synonyms are recognition aids, not claims about a vendor's
  documentation; the ⚠ terms are the ones a reviewer must confirm before a termbase ships.
- **[feature matrix §7](feature-matrix.md)** deferred *"EN 13402 and ASTM D5585 mappings are `supported`"*.
  **Disposition: clarified.** What is supported is the mechanism — a `size_system` value plus a chart — not
  a mapping nobody has read. The row stays `supported` and this chapter records why that is honest.

## 5. The verification plan, and what would change

Verification of a standard's content requires the document, and the documents are not free. So the plan is
a dependency chain with named owners rather than an intention:

1. **The domain expert seat must be filled.** [Governance §8.1](../governance.md) records it as **vacant**,
   not acting: no signature this repository can produce reviews domain content, and the procurement owner's
   seat is held acting by the director, so the search for texts has an owner but no buyer. Both are the
   director's to name.
2. **Procurement** obtains the texts. Until then, no claim in this book can become `read-in-repo`, and this
   chapter says so instead of implying that reading is a formality.
3. **Reading changes exactly three things:** landmark and procedure references may gain clause-level
   citations; the fixture's `assumed` body measurements become `known` or are corrected; and the size-set
   mappings gain real designation data. Nothing else in the model depends on a standard's content, which is
   deliberate — the model was specified so that a standard is *data with provenance*, not a load-bearing
   assumption.
4. **A read that contradicts this book is a defect**, logged and owned like any other, and the correction is
   made in the chapter that asserted the claim.

## 6. What is deliberately not adopted

- **Conformance claims.** No artifact, tech pack or release manifest asserts conformance to a standard.
  A manifest records what was produced, from which inputs, checked by whom — and a standard's designation
  appears there as data (which size system a chart came from), never as a certificate.
- **Designation as measurement.** A label never supplies a number ([size sets §3](size-sets.md)).
- **Copying tables into this repository without provenance.** A measurement table entered by hand from a
  standard is `assumed` with the person and date recorded, not `known` because a standard exists.
- **A standard's tolerance as ours.** See §3.4: the physical-acceptance class belongs to the factory.
- **Treating a withdrawn standard as current.** ASTM D6673-10 is implemented as a de-facto convention and
  recorded as withdrawn, because a receiver enforcing it does not care that it lapsed.

## 7. How the registry claim is derived

"No standard is cited anywhere in this book without a status and an owner" quantifies over every chapter,
including ones not yet written, so it is derived:

```bash
bash docs/tasks/artifacts/standards/run_standards_census.sh
# → standards census: <designations> registered / <uses> uses in the book / 0 failure(s)
```

The census extracts every standard designation that appears anywhere under `docs/book/src/` — the shapes
ISO-n, ASTM-Dn and EN-n, the bare word AAMA, and a declared list of others — and requires each to be a row
of §2's
registry with a status from §1's closed vocabulary and a non-empty owner. It reports, per designation,
where in the book it is used, so a claim cannot hide in one chapter while the registry forgets it. It
cannot judge whether a status is *true*: a row claiming `read-in-repo` is checked for the citation it must
then carry, and nothing more.

## 8. Verification status of the claims in this chapter

- **The four standards exist and are named by the roadmap** — `cited-from-roadmap` (§3.1, §3.4, §11 G0).
- **What each is used for in this model** — *project decision*, stated in §2 and §3. These are requirements
  on StitchCAD, not claims about the documents.
- **ASTM D6673-10 is withdrawn (Jan 2019, no replacement)** — `cited-from-roadmap` (ADR-0004), which
  records the withdrawal as a review finding. `G0-CONTRACT.10` owns the interchange consequences.
- **The AAMA layer names (CUT, DRAW, INTCUT, NOTCH, DRILL, TEXT, REF)** — `cited-from-roadmap` (ADR-0004's
  corrected layer table). Not quoted here as the standard's text.
- **Everything else in this chapter** — *project decision*: the status vocabulary, the no-unsourced-clause
  rule, the four model requirements, the deferral ledger and the verification plan are this repository's
  own contract, and are normative because this chapter says so.

## 9. What must be true in tests

- **No unsourced clause:** a lint over the book fails on a clause or table number attributed to a standard
  whose registry status is not `read-in-repo`. The census is that lint, and it runs on every `make probes`.
- **Every designation is registered:** adding a chapter that cites a standard the registry does not carry
  fails the census, naming the file and the designation.
- **Statuses come from the closed vocabulary:** a row saying "probably" or "as is well known" is refused.
- **A `read-in-repo` row carries its citation:** the status is only as good as the reference behind it, so
  the census requires one and refuses a bare claim.
- **The fixture's measurements stay `assumed`:** a test asserts that the reference skirt's body measurements
  carry the `assumed` state until a `read-in-repo` citation exists, so nobody can promote them by editing
  prose.
