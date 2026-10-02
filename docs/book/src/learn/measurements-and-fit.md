# Measurements and fit

A body measurement describes a person. A garment measurement describes the garment. Recording the
same number in both places does not establish that they measure the same thing.

## Say where and how you measure

“Waist” alone is ambiguous. A useful record identifies the quantity, the landmarks and the procedure:
where the measurement starts and ends, or which girth level it follows, and how it is taken.
A garment **point of measure (POM)** similarly identifies the garment quantity and its measuring method.
The [glossary](../spec/glossary/measurements-and-fit.md) defines these terms together.

The current library keeps body and garment measurement records separate, while allowing them to
refer to canonical scalar inputs. Their sources and uncertainty remain visible. A well-formed record
can establish that references exist; physical measurement quality still needs evidence.

## Connect them with Ease

**Ease** records the intended difference between a body quantity and a garment POM, together with
fit and correspondence intent. It has its own source and state.

For the reference skirt, the declared hip is 98 cm and intended hip Ease is +4 cm. The drafting
specification targets a finished hip of 102 cm. This simple arithmetic explains the intent; the
actual construction and later geometry measurement must verify the result.

A different POM may require a different relationship. A complete garment's fit cannot be proved by
one girth calculation. Negative Ease requires explicit compression intent in the model and remains
outside the supported v1 execution envelope.

## Preserve what you know

An assumed value lets you describe an exploratory design without claiming it was observed. An unknown
input says an observation is still needed. A derived input names a computation that still needs
execution. Numeric queries refuse unresolved inputs rather than returning zero or an earlier value.
Sources are references whose existence and truth have separate validation owners.

Next, [Pieces and assembly](pieces-and-assembly.md) explains how these intentions relate to physical
cut copies. For MTM, use the [single-person chart example](../spec/mtm-input-charts.md).
Experts can go directly to the [measurement](../spec/measurement-metadata.md) and
[Ease contracts](../spec/ease-inputs.md) in the annexes.
