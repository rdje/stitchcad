# Decision & Fact Records — Index (memory layer C)

Durable, cross-cutting facts and decisions live here, one record per file (ADR-style). Every
record must be listed below (the MEMORY-ARCH doctrine check enforces it). New record: copy
`TEMPLATE.md` → `<type>_<short-kebab-slug>.md`, fill it in, and add its row.

Records carry an `answers:` line so a question can find them — that is what makes a lesson
*retrievable* rather than merely written down (`LESSON-PROMOTION`).

| Record | Type | One-line hook |
| --- | --- | --- |
| [`decision_acceptance-evidence-per-leaf.md`](decision_acceptance-evidence-per-leaf.md) | `decision` | acceptance checkboxes exist only for landed leaves, added fresh in the commit — because the inherited gate judges the first matching box in the file (defect D15) |
