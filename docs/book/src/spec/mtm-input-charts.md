# Made-to-measure input charts

Made-to-measure (MTM) starts with one person's body measurements and the garment you want to make.
An input chart records how each body measurement relates to a named garment measurement, or
[point of measure (POM)](glossary/measurements-and-fit.md). That connection includes **Ease**:
the intended difference between body and garment measurements.

**Available now:** G1 implements immutable MTM input charts in the sc-measure Rust library.
There is no drafting application or MCP server yet. These charts describe inputs; generating and
checking the garment remains later work. Experts can go directly to the
[API and validation contract](../annexes/mtm-input-contract.md).

## Start with a waist example

Imagine a person with an assumed waist measurement of 74 cm. You intend to allow 4 cm of Ease at
the garment waist. The chart keeps these three roles separate:

| Role | Example | What it records |
| --- | --- | --- |
| Body input | 74 cm | The person's waist measurement and its source |
| Ease input | +4 cm | The intended signed amount and its own source |
| Garment target | Waist POM | Which garment measurement the inputs are intended to serve |

The target also describes where and how the garment measurement is taken. The chart can return the
body measurement and Ease separately. It does not calculate or certify a finished waist of 78 cm.
The construction recipe and its later evaluation must establish the garment result. This distinction
matters when a garment measurement depends on several pieces, folds or construction steps.

Several garment targets can use the same body measurement when their connections are explicitly
recorded. Shared input does not establish that the targets measure the same physical quantity.

## One person, one stable member

An MTM chart uses a custom size membership with exactly one member. That member is also its base.
Its label, such as “Client A”, is for display; its stable identity identifies the member. Replacing
that member with another bearing the same label does not silently transfer the chart to the new person.
See [size membership](size-membership.md) for labels, order and base identity.

The connections also identify their body and garment tables. Changing a connection's target requires
an explicit chart replacement. Editing a measurement's current value, source or uncertainty remains
visible through the existing connection; the chart does not retain a hidden earlier value.

## Build a draft, then check its coverage

You can author a partial chart while collecting measurements. Empty charts and unresolved inputs
remain inspectable drafts. A request for an unknown measurement identifies the observation still
needed; a derived measurement requires evaluation. Neither supplies zero as a substitute.

A complete input chart needs at least one connection and connections for **every current garment POM
in its Design table**. Adding a target to that table means the chart needs a new connection. Checking
one connection does not certify the others. Complete coverage can still contain unknown inputs;
it does not establish readiness to generate a pattern or prove fit.

MTM charts belong to the **regeneration** path. They refuse direct grade-rule input; recording a
single custom member does not create a graded size range. The [two instantiation paths](instantiation-paths.md)
explain that distinction. Negative Ease requires explicit compression intent in the structural model;
the supported v1 garment envelope still excludes negative-Ease execution.

Continue with [garment chart observations](size-chart-observations.md) to compare authored garment
inputs, or use the [expert annex](../annexes/mtm-input-contract.md) for identity, current-reference,
API and verification details. The [size-set contract](size-sets.md) describes the complete planned model.
