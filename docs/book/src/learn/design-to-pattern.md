# From an idea to a pattern

A pattern begins with an intention: for example, an A-line skirt with a waistband and a back zipper.
To turn that intention into something repeatable, record the measurements you start from, the steps
you use and the decisions that affect the result. StitchCAD calls that record a **construction recipe**.

## Think of a recipe you can revisit

Imagine changing the intended skirt length. A recipe identifies the measurement and the construction
steps that depend on it. Re-evaluating those steps should produce the new shape without guessing
which drawing lines must move. Recipe evaluation is planned work; the current libraries describe
many of the inputs and garment objects it will consume. A lexical scanner can now read the recipe
words and symbols with precise source locations; it does not yet validate or execute a recipe.
The [formula syntax annex](../annexes/formula-syntax.md) gives the developer contract.

A design therefore has several layers:

| Layer | A skirt example |
| --- | --- |
| Inputs | Body waist, hip and intended garment length |
| Construction | Draft panels, close darts and construct the waistband |
| Pieces and assembly | Identify the cut pieces and the edges that are sewn together |
| Factory requirements | Choose the named cutting room's documented export settings |
| Release evidence | Record checks and the human approvals for the resulting package |

These layers answer different questions. A factory file is an output of the design; it does not
replace the measurements or explain a construction history that was never recorded. Imported
geometry is represented explicitly rather than being given an invented recipe.

## Follow one example through the book

The [reference skirt](../spec/reference-skirt.md) is the detailed worked specification used for
future conformance work. Its declared body waist is 74 cm, hip is 98 cm and garment length is 62 cm.
These are fixture assumptions, not a size standard or evidence that the skirt fits a person.
You can explore the ideas without reading its drafting arithmetic yet.

First ask: which inputs describe the body, and which describe the garment? That distinction is the
subject of [Measurements and fit](measurements-and-fit.md). Then we will connect pieces, sizes and
agent workflows. Check [availability](../availability.md) whenever you need to know what can run today.

For the exact construction model, experts can use the [ontology annex](../spec/ontology.md) and
[formula contract](../spec/formula-language.md).
