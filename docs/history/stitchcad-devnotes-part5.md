# Sealed archive — StitchCAD dev notes, two more 2026-09-30 lessons

Immutable historical segment, sealed out of `DEV_NOTES.md` under the rolling-ledger protocol in
`LIVE_DOCUMENT_SIZE_CONTAINMENT.md`, performed by leaf `G0-CONTRACT.16` in the commit whose append
crossed the window's byte health target (200 lines / 16 384 bytes).

- **Sealed identity:** 43 lines, 3977 bytes, `sha256:859ce9812dddf27399f4e499d880053493e17a43b47e3b53a1c2a6edc3db9ef6`
- **Coverage:** the lessons `a rule whose only compliant path is "don't change the file" gets bypassed`
  and `a digest is a contract about BYTES, so the bytes have to be written down`, both dated 2026-09-30,
  oldest last, exactly as they stood.
- **Sealed by:** leaf `G0-CONTRACT.16` on `2026-09-30`, after `part4` (sealed by `G0-CONTRACT.11`).
- **Retrieval:** the sealed content is everything below the `---` rule and the blank line after it, and it
  ends with exactly one newline — the byte contract `run_changelog_ledger_probes.sh` states after defect
  D43, whose `DESCRIPTOR` rule reproduces this digest on every `make probes`.

---

## _(2026-09-30)_ — a rule whose only compliant path is "don't change the file" gets bypassed

- Applying the roadmap amendment was blocked by the containment gate, and blocked *correctly*: the `roadmap`
  row's transition debt was measured at exactly the file's size (`lines=919;bytes=50821` = `wc -lc`), so any
  growth read as a widened baseline. The rule is right; what was missing was a legitimate path, and the only
  one available was editing the number — the silent widening the rule exists to prevent. **When a gate's only
  passing move is to falsify its own input, the gate is incomplete, not the change.** The fix was to give the
  baseline a revision identity (`at=v0.3`) that the checker executes: a revision may re-base its baseline, but
  only in the commit that revises, which is where the authority has to be anyway.
- **An exit criterion must arrive with owners.** The same commit that gave G3 the envelope-coverage criterion
  made `G3-GRADING.5` required, created `.15` and made `.14`'s review fail without a leaf's evidence — because
  a roadmap clause no leaf owns is defect D32 one level up, and the tree was seeded from the gate's old text so
  it inherited the gate's gap exactly.
- **A census nobody runs is a claim, and this one had gone red for two committed slices.** The tree-coverage
  census defined a tree by *filename*, so the evidence siblings the containment registry prescribes looked like
  lane-less orphans — and `make probes` globs `run_*probe*.sh`, so nothing re-derived it. Three copies of the
  same `ls ${lane}-*.md | head -1` assumption existed; the third was in an *advisory* table, where it silently
  replaced `G0-CONTRACT` with its sibling and no exit code could reveal it. Reading a tool's output, not just
  its status, is part of running it.
- **Office can be held acting; competence cannot.** Two of the three empty governance seats are the director's
  authority already, so acting costs nothing and adds a record. The third — the sewing/factory expert — is
  knowledge nobody here has, so it is recorded as **vacant** with four explicit prohibitions, and the first G2
  golden stays gated on a real name. The tempting move (let the engineer act as reviewer) satisfies the wording
  of the two-step rule and destroys its meaning.

## _(2026-09-30)_ — a digest is a contract about BYTES, so the bytes have to be written down

- Sealing the third changelog segment of the day produced a refusal that read as the worst thing an archive
  can report — "content hashes to `3148dd0f…`, its descriptor declares `8553accc…`" — and the cause was one
  newline. The verifier hashes `content=$(sed -n 'rule+2,$p' f)` followed by `printf '%s\n'`, and bash command
  substitution strips EVERY trailing newline, so a segment whose sealed content ends with a blank line can
  never reproduce a raw-byte digest. Measured: `tail -c 12` showed `…touched\n\n` on the new segment and
  `…touched\n` on the one sealed an hour earlier.
- **The general shape: two honest implementations of "the sealed content" disagreed, and nothing said which
  one was the contract.** A digest proves identity only when both sides mean the same bytes, so the byte rule
  belongs in the instrument's header, not in the author's head. Fixed by normalizing the segment, recomputing
  its descriptor with the verifier's own method, teaching the rule to refuse a trailing blank line BY NAME,
  and pinning that with an arm — because a mismatch that looks like drift in an immutable file invites the one
  edit the archive forbids.
- **Corollary for the next rollover:** seal with the verifier's method, not with the language you happen to be
  scripting in. The check is one command (`sed | shasum`), so running it before writing the descriptor is
  cheaper than diagnosing the difference afterwards. Promoted, with the day's other containment derivation, to
  `docs/decisions/decision_maxline-health-derived-from-the-cell-budget.md`.
