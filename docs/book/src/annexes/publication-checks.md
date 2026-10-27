# Publication checks and renderer authority

Build the book and check navigation, current API/status and copied-fixture refusals from the root:

```bash
python3 -I -B docs/tasks/artifacts/book_publication/book_publication.py
```

Output stays at `docs/book/book/`; this entry owns guarded rendering. Shell/platform audit remains
`G1-SLICE.5b.4c.h2.b.s`, D156 other producers. Raw Make and `docs/book/book.toml` interfaces remain.

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

Fixtures validate complete copied populations before creation and teardown before recursive deletion,
also on exceptions. Mutations declare writable files first. Pure `check`/`rows`/`formula_scope` calls
produce nothing and preserve caller/source bytes. The public archive reader retains exact predecessor
bytes and ordered task/window membership through rollover.

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

These controls intercept renderer children for fault safety; the publication entry runs real mdBook.
Compiled production-body omissions must trigger independent assertion failures. Device/index controls
write neither off-volume nor to Git. Environment probes watch both contracts; `make probes` also runs
the publication entry. See [build/checks](../build-and-checks.md) for local stores and remaining audits.

## Exact authority for maintained book changes

The book is a maintained product reference. Its aggregate follows implemented product scope through
an exact change record, with its existing chapter limits retained:700 lines,40960 bytes and440 bytes
per content line. SUMMARY retains120 lines/8192 bytes/240 bytes per line, indexes every chapter once,
and allows at most three navigation levels. The numerical aggregate targets are explicitly inapplicable;
these part/index limits and a fresh record govern the collection instead.

After focused checks and completing the owning task leaf, record the proposal before staging:

```bash
python3 -I -B docs/tasks/artifacts/live_doc_size/maintained_reference.py record-change \
  --task G1-SLICE.new_book_leaf --reason 'Describe this exact book change'
```

Use the actual new leaf and reason for each change. The command validates the complete proposal,
prepares a private file under target, then atomically replaces only the canonical control. It edits
no chapter and preserves a conflicting caller edit. A failure before replacement leaves the previous
record; owned temporary files are cleaned. Preparation is repeatable within the same uncommitted
leaf; a previously completed leaf cannot authorize a later change.

The bounded UTF-8 JSONL control has one metadata header and one record, with8192 total bytes and6144
raw record bytes. It binds native Git baseline, exact file/line/byte deltas and framed Markdown digest
to a newly completed task. Even an edit with the same byte count needs a new record. Missing, stale,
reused, banked or inexact authority refuses, alongside incomplete navigation and oversized parts.
This is numerical document-change authority, never cryptographic signing or human release approval.

Git stores the governed Markdown and JSONL bytes unchanged. The repository attributes explicitly
disable text/newline conversion, clean filters, identifier expansion and working-tree encoding for
these paths; the verifier checks their effective attributes before accepting authority. A local or
nested override that enables a transformation refuses. Authors may use LF or CRLF: byte counts and
digests include the original line endings, while content width excludes LF and one preceding CR.
Recording never silently normalizes a chapter. The pinned completed garment task graph and sealed
Markdown history also retain exact stored bytes. Sealed source copies may end with a blank line:
their scoped whitespace rule preserves that EOF while ordinary source whitespace remains strict.

Task ownership follows the canonical family under docs/tasks: the root named for its Tree ID and
linked siblings named with that Tree ID plus a hyphen and suffix. For example, G1-SLICE.md links the
G1-SLICE-execution.md frontier. Links may traverse siblings and return to the root. The fixed reader
profile permits32 family files, each at most98304 bytes; directory/native-listing censuses are bounded.
Missing, unlinked, unsafe or duplicate owners refuse. A newly completed owner in a linked sibling is
valid; copying a previously completed sibling owner into the main file cannot make it fresh.

The baseline checks every historical family member, including a member whose link later disappeared.
Authoring freezes every current family source and rechecks its census before replacement. Late sibling
edits preserve the caller's change and prior authority. The completed structural/input/garment graph
is retained as a live canonical sibling with all98 node identities/statuses/proof pointers; its exact
native source copy is checked, and no task is replaced by an anonymous history summary.

Completed chronological evidence can move to sealed history after checking its current nodes and
consumers. Record572 preserves the complete old verification/commit journal; all seven task IDs have
current completed nodes and21 current proof links retain their fragments. This restores the generated
orientation map's existing bound while preserving the neutral generator and every historical byte.

The existing size-check command runs the canonical book verifier for normal commit/CI checks. Numeric
fixture overrides cannot replace that book; caller Git directory/index/configuration cannot redirect
its baseline. Read-only checking creates no book control or build store. Native measurement paths
are preflighted before opening bodies, including symlink/special-file/repository/volume boundaries.

Scoped acceptance covers reader23, guarded authoring8 and native newline/storage3 controls, with
native wc/SHA calibration and controlled rename/late-input failures. The actual installation commit
03460b0 passes committed-mode verification; ordinary candidate directories are not simulated commits.
The default numeric/book gate and its refusal probes remain required. These checks prove document
scope/freshness, not semantic correctness, cross-platform runtime parity, physical support or release
readiness. Broader product verification stays with G1 integration and the production gates.
