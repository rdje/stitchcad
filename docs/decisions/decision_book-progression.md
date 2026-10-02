# The book progresses from learning to expert annexes

- **Type:** `decision`
- **Date:** `2026-10-02`
- **Status:** `active`
- **Owner / source:** director's explicit publication directive; adopted by `G1-SLICE.4d.1`.

answers: "how should the book serve students and experts?" · "where do detailed contracts belong?" · "how do roadmap, code and book stay aligned?"

## The decision

Keep roadmap requirements, code and mdBook behavior in lockstep in each task-owned commit.
Introduce concepts incrementally for students and newcomers; provide a glossary, topic index and
direct expert routes. Numerical, API, format and verification detail belongs in annexes.

## Why

The director reviews the book as the public view of the implementation. A specification-only landing
page incorrectly continued to report G0 after G1 libraries existed (D71); it also required newcomers
to enter through technical contracts. Learning order and truthful availability are both needed.

## How to apply

- Teach recipe, measurements, pieces, sizes and agents in short progressive chapters, with declared
  examples. Teach only supported workflow steps; future scenarios are explicitly illustrative.
- Put exact contracts and evidence in the Annexes navigation section. Preserve published technical
  chapter URLs and clause anchors; their source files may remain under `docs/book/src/spec/`.
- Keep the existing glossary/term index and a separate topic index covering registered chapters.
  Experts can enter through availability, implementation status or direct API references.
- Review affected roadmap clauses, public code/tests and book contracts before each change. Update
  implemented/deferred boundaries in the same commit; never turn a structural API into a geometry,
  fit, receiver-compatibility or production claim. Human approval remains distinct.
- Run `make book`, the glossary census and
  `docs/tasks/artifacts/book_publication/run_book_publication_probes.sh`. The latter checks source/
  rendered local navigation, chapter/index coverage and a scoped public-API/status map. Those checks
  do not prove all prose or domain semantics; tests, manual review and later gates retain that work.

## Application and authority

The director explicitly requested this publication requirement; the engineer applies it to roadmap
§2 and records it in the v0.3 Disposition Log. Locked product scope and gate exits remain unchanged.
No new certification is granted. Future book changes inherit this policy and the existing commit rules.
