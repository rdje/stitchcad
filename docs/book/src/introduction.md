# Welcome to StitchCAD

StitchCAD is being built for people who make garment patterns and need to explain how those patterns
were constructed. A design records measurements, decisions and construction steps so that a change
can be traced through the pattern. The long-term goal is a checked package for a named factory.

**Today, the project provides Rust foundation libraries, not a finished drafting application.**
The [availability page](availability.md) separates what exists from the planned workflows.
This book explains both, with technical contracts kept in the annexes.

## Choose your route

| Reader | Start here |
| --- | --- |
| Student or newcomer | [From an idea to a pattern](learn/design-to-pattern.md), then follow the learning chapters |
| Patternmaker or sewing expert | [Measurements and fit](learn/measurements-and-fit.md), then the [reference skirt](spec/reference-skirt.md) |
| Experienced developer or agent integrator | [Availability](availability.md), then the [topic index](topic-index.md) for direct API and contract links |
| Looking up a word | The [glossary and its A–Z term index](spec/glossary.md) |

The learning chapters introduce one set of ideas at a time: a recipe, its measurements, the pieces
it produces, sizes, and collaboration with agents. You do not need to learn API names or file-format
rules to follow them. Experienced readers can go straight to the annex they need.

## Two principles to carry with you

**An unknown stays unknown.** If a measurement or factory requirement is missing, the model records
what still needs observation. It does not silently use a plausible number. An assumption remains
labelled as an assumption, even when it helps you explore a design.

**A successful check has a scope.** Valid input records do not prove a pattern fits. A file accepted
by one factory does not prove compatibility everywhere. The evidence must say what was checked and
what that result supports. This is how the project works toward production-grade behavior.

## Where the detail lives

- **Learning path:** short explanations and declared examples, in reading order.
- **Availability:** implemented library behavior and future work, stated separately.
- **Glossary:** shared meanings, synonyms and a term index.
- **Annexes:** normative model, API contracts, numerical rules, formats, validation and governance.
- **Topic index:** direct links across the whole book, including the expert references.

The roadmap defines the required behavior; code and tests establish what exists; this book explains
that same state. A feature change and its documentation belong to the same task-owned commit.
The repository's LIVE_STATUS.md carries detailed gate progress. G0 contract review is mostly complete
with human closure unapproved; G1 executable foundations are in progress.

Continue with [From an idea to a pattern](learn/design-to-pattern.md).
