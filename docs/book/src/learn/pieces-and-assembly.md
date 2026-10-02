# Pieces and assembly

A pattern Piece describes what to cut. A physical cut copy describes one actual use of that Piece.
That difference becomes useful as soon as an instruction says “cut two”.

## Two copies can have different neighbors

Two copies may share pattern geometry while being sewn to different neighbors. StitchCAD therefore
gives each physical copy a stable identity. A seam can identify a range on a specific copy's edge;
it does not rely on “the first piece in the list”. Paired and folded cutting intent remains explicit.

The **sewing graph** records these assembly relationships. It can represent partial-edge seams and
relationships involving several spans. The current library checks the authored references; walking
edges and checking realized lengths remain later geometry/construction work.

## Keep sewing and cutting lines distinct

A seam allowance adds material outside the sewing boundary. In the reference skirt, side allowances
are declared as 1 cm and hem allowances as 3 cm. These fixture inputs illustrate different roles;
they are not factory defaults or a claim that allowance geometry has been generated.

Notches help identify matching positions. Grainlines carry direction and material intent. Darts,
pleats, gathers, hems, closures and pockets describe construction choices with their own dependencies.
Each needs appropriate geometric and physical checks before a garment can be approved.

## Changes can reveal repairs

If an edge is split, reversed or deleted, its references must either resolve through the edit history
or expose a repair. A stable identity does not make an invalid reference valid. Current structural
queries preserve that distinction and do not silently assign a seam or mark to a different edge.

Continue with [Sizes and grading](sizes-and-grading.md). Experts can consult the
[executable ontology](../spec/ontology-implementation.md),
[garment constructions](../spec/ontology-constructions.md) and
[closure contract](../spec/ontology-closures.md) in the annexes. The
[structural review](../spec/ontology-review.md) records the tested scope and later proofs.
