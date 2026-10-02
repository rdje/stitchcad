# Canonical literals retain exact width; numeric bindings fit i64

- **Type:** `decision`
- **Date:** `2026-10-02`
- **Status:** `active`
- **Owner / source:** director's explicit D95 answer; recorded by `G1-SLICE.5a.3b.3b.3a`.

answers: "must a canonical literal fit i64?" · "how does the lowest signed angle parse?" · "where is numeric storage checked?"

## Context and decision

Unary minus is a separate operator. The lowest signed i64 microdegree angle therefore contains a
positive literal of 2^63, which cannot itself fit i64. Restricting every literal node to signed64
would reject that direct spelling or require an identity-changing sign fold.

The director chose: **allow exact literal nodes up to the existing 128-bit rational limit;
require i64 only for bound numeric values**. Canonical literal integers and completed rational
expression values obey `max_rational_bits`, with the existing scalar domains and input quantum
rules. Unary minus keeps its operator node. Numeric `let` binding rounds once, then checks the
inclusive signed i64 range; Count remains nonnegative. Boolean and opaque geometry references do
not acquire a numeric storage width. Length/area retain their smaller declared scalar domains.

For example, the machine angle -9223372036854.775808 deg parses as unary minus of angle literal
9223372036854775808 microdegrees and binds to -9223372036854775808. A positive bound value of
9223372036854775808 refuses, while an exact intermediate of that value may cancel into a valid
binding. Signed and multi-turn angles retain the separate D84 rule in `decision_angles.md`.

## Application and proof boundary

The formula chapter/grammar and roadmap adopt this clarification; gate exits and product scope
stay unchanged. The recording leaf verifies the reference binding boundary. `.3b` verifies146
independent canonical-node/Decimal controls and twelve compiled actual mutation reds, preserving
unary identity and the128-bit input boundary; D95 closes. `G1-SLICE.5a.3b.3b.3c.2` completes D83’s scoped reference review with four independent families/47 actual
mutation reds and exact restoration. Product numeric normalization, canonical serialization and evaluation remain future G1 work; this decision grants no production signoff.
