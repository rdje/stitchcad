# Director's ruling of 2026-09-30: the engineer decides and acts on the four surfaced findings

- **Type:** `decision` (a delegation of authority, and its limits)
- **Date:** `2026-09-30` (absolute)
- **Status:** `active`
- **Owner / source:** the director's instruction in session, recorded by the engineer immediately before a
  session handoff, so the authority survives the conversation it was given in

answers: "who decides the reference fixture's waistband construction?" · "may the engineer assign a proving gate to an envelope feature?" · "what is still reserved to the director?" · "is external research allowed, and is it reachable from this machine?" · "does a delegation let me upgrade a claim's verification status?"

## The ruling

The director reviewed the four items surfaced at the end of the 2026-09-30 session and instructed the
engineer to **decide and act** on all four — "at SOTA, signoff and production-grade" — searching external
sources where useful ("the internet, publications, academia … wherever you can"):

| Item | What it is | Leaf that owns the action |
| --- | --- | --- |
| **D27** | the reference fixture's waistband is two different garments at once (§4 a single band folded lengthwise at 10.0 cm, §6 a faced two-piece band at 6.0 cm per piece, §8 sewing only the outer) | `G0-CONTRACT.13d` |
| **D32** | five v1-envelope features — classic collar, trousers, button/buttonhole, pocket, fly — that no gate's exit criteria prove | `G0-CONTRACT.4b` |
| **`.14`** | the governance model, and the roles that need a named human | `G0-CONTRACT.14` (drafting unblocked; only the naming stays blocked) |
| containment | `book_collection`'s maxline *health* target was derived from prose chapters and every table-shaped reference part exceeds it while staying inside its ceiling | `SPINE.4.4` |

## Scope of the delegation

Delegated, to be decided and landed under the ordinary discipline (leaf ownership, tool-backed evidence,
claim labelling, one commit per leaf):

1. the fixture's waistband construction, its piece list, its spans and every number that follows;
2. the feature matrix's dispositions and the **proving gate** assigned to each of the five D32 rows;
3. the governance chapter's full draft, its review paths and its role definitions;
4. the containment derivation for table-shaped book parts, including the registry note that records it.

## Reserved to the director — NOT delegated

1. **Naming humans.** The project owner, the procurement owner and the sewing/factory domain reviewer are
   the director's to name. `.14` stays blocked on exactly that and nothing else; everything else in `.14`
   is drafted. D27's construction decision and the fixture's eight `assumed` constants remain subject to
   that reviewer's confirmation — deciding them now is an engineering decision under delegation, not a
   domain certificate.
2. **Amending `ROADMAP.md`.** The roadmap's owner is the director and its changes go through its own
   disposition log. Where resolving D32 requires a gate's exit criteria to name a feature, the engineer
   prepares the **exact amendment text** in a decision record, marked proposed, and the matrix acts
   consistently with it as a proposal. The roadmap file itself is not edited by the engineer.

## Claim discipline is unchanged by the delegation

A delegation to decide is not a delegation to upgrade evidence. External material read on this machine is
recorded with its URL and the date it was read, and labelled; a claim about a *standard* becomes
`read-in-repo` only when the standard's own text has been read here (`docs/book/src/spec/standards.md` §1).
A decision resting on general domain practice rather than a read source stays `assumed` with its reviewer
named. Where the two conflict, the label wins and the decision records that it is provisional.

## Reachability verified at the ruling

Research and push are both possible from this repository's machine, measured rather than assumed:

```bash
curl -sS -m 12 -o /dev/null -w '%{http_code}' https://github.com                 # → 200
curl -sS -m 12 -o /dev/null -w '%{http_code}' https://en.wikipedia.org/wiki/Waistband   # → 200
git ls-remote --heads origin                                                     # → refs/heads/main
```

At the ruling, `git rev-list --count origin/main..HEAD` → `11` unpushed commits and `make push-due` → no
exceptional push owed. If a later slice touches `.doctrine/`, CI or a doctrine check, the immediate-push
exception fires and the observed CI verdict must be recorded in the owning leaf — which this reachability
makes possible.

## How to resume

The frontier is `G0-CONTRACT.13d` → `.4b` → `.14` (draft) → `SPINE.4.4`, then back to `G0-CONTRACT.9`
(the formula language). Each of the four leaves states its own goal and acceptance; none needs this record
to be actionable, and this record exists so no future session re-asks whether the authority exists.

Related: [[decision_fixture-oracles-derive-the-finished-dimension]] · [[decision_size-set-ownership]] ·
[[decision_product-work-takes-the-frontier]] · `docs/tasks/PLANNING.md` (defects D27, D32) ·
`docs/book/src/spec/reference-skirt.md` §6 and §11 · `docs/book/src/spec/feature-matrix.md` §9.
