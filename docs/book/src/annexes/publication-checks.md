# Publication checks and renderer authority

Build the book and check navigation, current API/status and copied-fixture refusals from the root:

```bash
python3 -I -B docs/tasks/artifacts/book_publication/book_publication.py
```

Output stays at `docs/book/book/`; this entry and `make book` use guarded rendering. Install the
published local tool with `python3 -I -B scripts/book_tool.py install` first. Shell/platform audit
remains `G1-SLICE.5b.4c.h2.b.s`, D156 other producers;the book.toml interface remains.

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
environment keeps its caller values. Git is a read-only host dependency;the renderer is the explicit
project-local target/cargo-home/bin/mdbook. The selector,profile and binary are validated and hashed
with the complete build inputs before producing;missing or unsafe tool inputs refuse.

## Private rendering and public replacement

An exclusive temporary root is chosen and planned under `target/scratch/` before creation.
The child receives prepared repository-local Cargo, Rustup, build and scratch stores; valid caller
store choices and unrelated variables survive. It runs the existing argument vector and book root:

```text
target/cargo-home/bin/mdbook build docs/book
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

D377 reconciles the checker with the registered glossary/topic-index routes after their optional
standalone headings retired in0788/0801. The glossary must follow the seven learning chapters and
precede annexes;the topic index must be the final chapter after annexes. Specification routes stay
inside that annex boundary. Chapter/link/index coverage remains;the director2026-11-04 retires the SUMMARY size limit. Focused controls accept current/historical headings and reject eight missing/misplaced
route/part cases;historical AST checks preserve all other checker behavior exactly.

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
| `PublicationResult` | Tuple-compatible checked counts with named fields and a labelled summary |
| `chapters` | Registered book chapters |
| `scoped_api_rows` | Rows in the explicitly scoped current API/status map |
| `source_links` | Local Markdown links checked in chapters and SUMMARY |
| `rendered_links` | Local links checked in rendered chapter pages |

The result fields are `chapters`, `scoped_api_rows`, `source_links` and `rendered_links`, in that
original tuple order. `summary()` names each population explicitly. The fixture's historical-body
comparison verifies the exact approved result class, unchanged ordered payload and summary call
before projecting this adapter; every remaining original checker/copier/mutation/main node is compared.
Distinct-field values reproduce the original formatter, and class/import/payload/summary/body drift refuses.

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
The publication runner then checks actual generated search queries and chapter landings under
read-only host Node22.8+;see [native query verification](book-search.md#native-query-verification).
Historical continuity reads logical reports with `read` and original control/payload identities with
`read-backing`. The retained v1 master is distinct from the active generation index at the same path;
original schema/window-prefix checks apply to that retained master. Catalog and source-interval bytes
remain compared exactly. Ten whole captures, predecessor order/uniqueness, twelve corruption or
unavailable-reader refusals and three actual guard omissions pass without rewriting retained data.

For example, this retrieves the original window control even after its physical file is retired:

```bash
python3 -I -B scripts/history_archive.py --root . read-backing .doctrine/history_archive/window1.json
```

Compiled production-body omissions must trigger independent assertion failures. Device/index controls
write neither off-volume nor to Git. Environment probes watch both contracts; `make probes` also runs
the publication entry. See [build/checks](../build-and-checks.md) for local stores and remaining audits.

## Book integrity without size containment

The director2026-11-04 removes every book size limit and growth-authority requirement. Chapter
and index lines/bytes/width,file counts,aggregate content and navigation depth are unrestricted.
Book changes require ordinary task ownership and accurate complete documentation;no numerical
headroom,receipt,baseline/delta approval or record-change step is needed.

The legacy-named maintained_reference.py now checks structure only:

```bash
python3 -I -B docs/tasks/artifacts/live_doc_size/maintained_reference.py
python3 -I -B docs/tasks/artifacts/live_doc_size/maintained_reference.py --index
```

Both modes require every Markdown chapter in SUMMARY exactly once,valid local chapter paths and
UTF-8 text. Missing,duplicate,self-referential,escaping or unindexed chapters refuse. Working
sources must be regular,on the repository volume and outside nested repositories;staged entries
must be ordinary stage-zero native objects. Caller Git overrides cannot redirect staged checking;
the original object listing must remain stable. No canonical file,index or book control is written.
Navigation indentation must retain parents;there is no maximum nesting depth.

Focused controls exceed all former book size axes and permit edits without a receipt,including
same-size content changes. Independent missing/duplicate/path/encoding/staged-mode refusals remain.
The numeric document checker excludes the book from measurements and budgets;all non-book
containment and its original refusal probes remain. Full publication continues to check rendered
coverage,links,anchors,learning routes and declared API/status owners. These structural checks
make no numerical,sewing,runtime or release-readiness claim.

The superseded policy,its exact proofs and last control remain in retained history;Git also keeps
the retired writer and tests. Shared task/path/Git helpers retain their independent non-book
contracts for continuity and measurement consumers. Scoped newline/storage controls still protect
original task/history bytes and ordinary source whitespace.

## Checks

Prioritize complete feature implementation. Check each addition with one affected unit/contract
test; add focused regressions when needed and reuse established tools. Broad regressions, mutations,
full publication probes and repeated platform sweeps wait for integration milestones. Full gates
remain mandatory before authorized pushes; concrete failures/risks may warrant wider checks sooner.

Code quality, task ownership, repository-local storage, book updates, commits and automatic
doctrine hooks remain required. State exactly what was checked and keep unrun verification
owned and scheduled. A focused pass does not imply that the full suite passed.

Maintenance work takes the frontier only when it directly blocks the selected feature or
can destroy or corrupt current work. Other audit and tooling followups remain pending behind
feature implementation. Finish and commit a dirty unit before changing course.

The product-progress report names implemented capabilities and roadmap acceptance. Maintenance
progress is reported separately, including maintenance tracked inside a G1 task.

Director2026-10-25 ruling: `docs/decisions/decision_product-work-takes-the-frontier.md`;
`COMMIT.md` defines the workflow.

Commit, clear the successful brief and check post-conditions in one shell. Consume any running tool's
terminal result before edits/retries/another slice: it may already have committed. Before retrying,
inspect HEAD/status/brief. Failure retains the brief for diagnosis; success clears it immediately.
D227's duplicate receipt established this guard.

Size checks own unique repository-local scratch, removed on trapped exit. D274's native-wc overlap
control protects in-flight measurements from another checker's cleanup. D275 mutates the actual
row's at= token and requires its exact stale-revision refusal. Caps/coverage/refusal rules stay fixed.

The native static-loader entry freezes its complete current input population, including tracked Rust
files and the book, before executing its consumers. The same hash guard runs before execution and
after each of eight consumers. Newly added Rust files and later feature implementations enter that
current snapshot; the original Python-body oracle retains its historical source pin.

```bash
python3 -I -B docs/tasks/artifacts/ci_environment/python_static_loader_contract.py --native
```

This checks the existing consumer arguments and expected outputs, preserves every declared input,
and records the actual current library identity after the coupled run. Isolated new-Rust, old-Rust
and Python-input drift controls must refuse. This scoped result leaves the full pre-push gate owned
separately by its task-tree milestone.

The G0 parse-only review currently prints an engineering-met sentence after skipping its instrument
checks (D334). That mode provides structural inspection. Use the normal executed review for the
engineering verdict; human closure remains separate. The wording repair is owned by
`G0-CONTRACT.15f`, deferred behind the current G1 product activity and required before G0 closure.

Adding a selected producer source also requires its exact disposition in
producer_entries.tsv and,for Python,python_file_entries.tsv. Output/delegation entries additionally
need python_effect_entries.tsv and a live bounded audit owner. The source census intentionally
retains ordinary Rust modules named build.rs for conservative inspection;that name does not make
a module a Cargo build script. Run producer_census.py and the focused file/effect-plan controls
when adding these entries. The existing pre-push probes independently check source/path coverage
and refuse missing/duplicate/wrong-kind/wrong-owner rows. Inventory completeness is inspection,
not runtime locality or production approval. D341 repairs missing entries;D156 retains the
separate complete producer/effect audits,including fresh diagnostic search captures.
