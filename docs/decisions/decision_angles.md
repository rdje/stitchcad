# Formula angles preserve signed and multi-turn sweeps

- **Type:** `decision`
- **Date:** `2026-10-02`
- **Status:** `active`
- **Owner / source:** director's explicit D84 answer; recorded by `G1-SLICE.5a.3b.3a.1`.

answers: "do formula angle bindings normalize?" · "can a bound angle retain a full turn?" · "where do directions normalize?"

## Context and decision

The formula kind table called every stored angle normalized while also using that kind for sweeps.
That would turn a bound 360-degree sweep into zero and change its arc length; modulo also loses a
negative sweep's sign. The director chose: **preserve signed and multi-turn formula angles;
normalize direction fields on entities**.

Formula literal, arithmetic and binding values retain their signed microdegree value and complete
turns. Binding rounds at its declared quantum; it does not apply direction modulo. Formula equality
compares those values, so 0 degrees and 360 degrees differ. Entity fields whose contract is a direction
normalize to [0, 360) degrees and compare normalized directions; sweep fields retain their sweep.
The normalized `sc-units::Angle` direction type cannot represent raw formula sweeps.

## Application and proof boundary

The formula/units specifications adopt this clarification in the recording commit. It changes no
locked product scope or gate exit. `dir` remains explicitly normalized by its function contract;
this does not normalize other angle values. `G1-SLICE.5a.3b.3c` owns D84 implementation verification,
including bindings/equality, signed inverse-trig result contracts and full/signed/multi-turn examples.
G1-SLICE.5a.3b.3c.2 verifies90 independent principal/binding/equality/sweep/replay controls and
fifteen compiled actual assertion reds, with byte-identical source restoration. The existing42-row
math oracle/72 angular controls now retain signed inverse outputs while dir stays normalized;
seven existing angular mutation reds retain conversion/pole proof. G1-SLICE.5a.3b.3c.3 completes
the original-obligation review; D84 closes for the received contract/scoped reference repair.
Individual product angle literals retain complete turns under G1-SLICE.5a.3c.2. Whole-expression literal
normalization under .5a.3c.3 preserves unary sign and every angle child. Entity integration,
binding/evaluation in the product and arbitrary-input
transcendental/cross-platform certification remain separate G1 obligations. D83 scoped reference
boundaries are reviewed separately. This ruling grants no production or general numerical signoff.
