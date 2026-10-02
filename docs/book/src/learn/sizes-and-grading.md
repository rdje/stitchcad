# Sizes and grading

A size label is a name, such as “M” or “Client A”. It is not a measurement and does not determine
where that size belongs in a range. StitchCAD records member identity, authored order and an explicit
base member separately. Changing a label does not change the member's identity.

## A chart connects named quantities

A garment chart connects each member with observations for named garment POMs. A partial chart can
be saved as a draft. Complete structural coverage requires every current Design garment POM for
every current member. Adding a POM to the Design creates new correspondence work.

Coverage is different from numeric readiness: a cell can name a measurement that is still unknown.
It is also different from measuring regenerated geometry or proving physical fit. The current chart
libraries preserve those boundaries and keep source/state/provenance attached to their current inputs.

## Two ways to produce sizes

| Path | Starting point | What the planned execution does |
| --- | --- | --- |
| Regeneration | Measurements for each body or size | Evaluate the construction recipe again |
| Grade rules | A base pattern and authored rules/breaks | Apply per-point changes to produce the range |

The two paths are semantically different. A recipe-drafted range does not automatically round-trip
through grade rules without information loss. Their later comparison needs declared tolerances and
checks of reconstructed extreme sizes, not just the base.

**Made-to-measure** uses a custom member of one and explicit body-to-garment Ease correspondence.
Its current input chart refuses direct grade-rule input. The [MTM example](../spec/mtm-input-charts.md)
shows how the body input, Ease and garment target remain separate.

## What exists today

Membership and authored garment/MTM chart foundations are implemented. Complete SizeSet composition,
axes, breaks, resolved profile transformations and execution remain work in progress or future work.
There is an unresolved specification decision about explicit axes; the model does not invent a default.

Next, [Working with agents](agents-and-workflows.md) connects these ideas to the planned command API.
Experts can use the [size-set](../spec/size-sets.md), [membership](../spec/size-membership.md),
[chart collection](../spec/size-chart-collections.md) and [instantiation](../spec/instantiation-paths.md) annexes.
