# Publication checks and renderer authority

The publication instrument builds this book, checks its navigation and current API/status map,
and runs copied-fixture refusal controls. Run it from the repository root:

```bash
python3 -I -B docs/tasks/artifacts/book_publication/book_publication.py
```

The public output stays at `docs/book/book/`. This Python entry owns its guarded renderer path;
the remaining shell/platform audit is `G1-SLICE.5b.4c.h2.b.s`. The raw Make book recipe and
`docs/book/book.toml` retain their existing interfaces. D156 remains open for other producers.

## Before the renderer starts

The builder validates its profile/module, configuration and complete source tree without creating
stores. Root-relative paths derive from the current checkout. Sources must be local regular files;
directory aliases, parent escapes, foreign volumes and nested Git markers refuse. Every Markdown
chapter in `SUMMARY.md` must already be an enumerated source file.
The main entry additionally validates its status map, roadmap, task files and complete Rust reader
population before the first producing call; standalone rendering needs only its own book inputs.

The supported configuration is the current single-line metadata in `[book]` and `[output.html]`,
with JSON-compatible strings and author arrays. The source directory is `src`, and the renderer is
the built-in HTML backend. Additional configuration keys, custom preprocessors/renderers, theme
inputs and include directives require an explicit input/delegation plan before acceptance.
Incoming `MDBOOK_*` configuration overrides refuse; the builder forces warning-level logging.

For example, this refuses before rendering or changing the public output:

```bash
MDBOOK_BUILD__BUILD_DIR=target/unplanned-book \
  python3 -I -B docs/tasks/artifacts/book_publication/book_publication.py
```

The existing output tree is inspected in full before effects. Symlinks, special files,
cross-volume entries, nested Git markers and hard-linked output files refuse. A read-only Git
index query refuses tracked data in that tree, including when the caller supplies another Git
directory/index through environment variables. Child Git context is bound to this checkout;
system/global Git configuration is excluded and optional index locks are disabled. The parent
environment keeps its caller values. Git and mdBook are read-only host executable dependencies.

## Private rendering and public replacement

An exclusive temporary root is chosen and planned under `target/scratch/` before creation.
The child receives prepared repository-local Cargo, Rustup, build and scratch stores; valid caller
store choices and unrelated variables survive. It runs the existing argument vector and book root:

```text
mdbook build docs/book
```

Only the child's configuration redirects output to the private root and disables creation of
missing source chapters. These are documented [mdBook environment overrides](https://rust-lang.github.io/mdBook/format/configuration/environment-variables.html);
[create-missing](https://rust-lang.github.io/mdBook/format/configuration/general.html#build-options)
normally permits source creation, so the instrument explicitly sets it to false.

The private renderer owns its exclusive output namespace. After it returns, every generated file
and directory is enumerated and validated before any public replacement. All declared chapters
must have generated HTML files. Nonzero child status, warning output, missing chapters or changed
source inputs refuse while retaining the previous public output. Public destination shape and
authority are freshly checked before deletion and again before copying the complete generated tree.
An owned old asset may change between file and directory; both the old tree and new population
are validated before replacement. Source snapshots cover directory topology as well as file bytes.

This is renderer validation. The subsequent publication check verifies chapter/index coverage,
learning order, source/rendered links and anchors, and the scoped status map. Numerical, sewing,
runtime and approval correctness remain with their canonical contracts and later gates.

Temporary fixture roots similarly validate the full copied population before creation. Complete
teardown populations are checked before recursive deletion, including on exceptional exit.
Fixture mutations declare their mutable files before writes. Pure `check`, `rows` and
`formula_scope` calls activate no producer and preserve caller/source bytes. Continuity retrieves
logical archive records through the public reader, retaining exact predecessor bytes and ordered
task/window membership across rollover.

These identifiers belong to the publication Python module/configuration:

| Identifier | Meaning |
| --- | --- |
| `src` | The fixed book source-directory setting and its source folder |
| `check` | Pure chapter, navigation and current-status validation |
| `rows` | Pure reader for the scoped tabular API/status map |
| `formula_scope` | Pure validator for known formula availability claims and methods |

## Verification

The original publication corpus retains its 40 mutation branches and 45 cases, including the
actual malformed-generic warning and repaired rendering. The existing fixture contract checks
copied populations, physical refusals, pure calls and original main semantics. Builder-specific
controls separately capture inputs, child environments, generated files, public replacement,
temporary cleanup and the two direct main writes:

```bash
python3 -I -B docs/tasks/artifacts/ci_environment/python_publication_builder_contract.py
python3 -I -B docs/tasks/artifacts/ci_environment/python_publication_fixture_contract.py
```

Those controls use intercepted renderer children for fault safety; real mdBook execution remains
the publication entry above. Compiled production-body omissions must trigger independent assertion
failures. Device/index failures are controlled observations, without off-volume writes or Git index
mutation. The environment probe runner watches both contracts; `make probes` also runs the actual
publication entry. See [build and checks](../build-and-checks.md) for local stores and remaining audit scope.
