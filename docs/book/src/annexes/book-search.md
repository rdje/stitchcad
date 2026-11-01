# Full-text book search

Search indexes every registered chapter's complete text and returns one result per matching
chapter. The supported heading-split-level=1 setting groups results by chapter;results open
the containing chapter. Use its section navigation for finer anchors. No chapter is excluded and
no legitimate product prose is removed to meet a search-size warning. See the
[mdBook search contract](https://rust-lang.github.io/mdBook/format/configuration/renderers.html#outputhtmlsearch).

Publication checks require exactly one local search document for every SUMMARY chapter,nonempty
stored text and actual full-text body postings for every document. Missing or duplicated chapters,
empty text,lost body postings and invalid result anchors refuse. The standard renderer's warning check remains mandatory.
D339 adopted this policy after the default section index crossed mdBook's warning threshold;the
same-source migration census retained every chapter's searchable words. Focused in-memory controls
exercise actual data refusals without changing generated or source bytes. No extra runtime tool
or external service is required by this checker.

Copied publication fixtures include the generated searchindex-*.js alongside the rendered HTML.
Both the complete copy plan and each copy effect check the index as an input/output. Before each
intentional publication fault,the copied book must pass the same checker and match the live
baseline. A missing index therefore cannot masquerade as an intended source-link refusal.
The checker reads the data wrapper without executing JavaScript. Its50 publication controls
retain the original45 checks and add five search-data refusals. Independent controls reject
actual omissions of the index plan and copy;historical AST checks retain the exact approved
search additions and all earlier checker/copier/mutation behavior.

## Capture a fresh comparison

The diagnostic capture renders the same complete source twice:the section-level baseline
and the supported chapter-level candidate. It records source hashes,the actual mdBook version,
both logs and document/JSON/script sizes. It neither replaces the public book nor edits its sources.
Choose an unused directory under target;the following example refuses if its capture already exists.

```bash
python3 -I -B docs/tasks/artifacts/book_publication/search_migration_capture.py --output target/book_search_my_new_capture
python3 -I -B docs/tasks/artifacts/book_publication/search_migration_contract.py --baseline target/book_search_my_new_capture/level3 --candidate target/book_search_my_new_capture/level1 --sources target/book_search_my_new_capture/source-sha256.json
```

The writer validates all original inputs and five metadata/log destinations before reserving
the capture directory exclusively. A competing creator causes refusal before any metadata is
overwritten. Existing captures are immutable inputs;select a fresh name rather than rerunning one.
Nested paths,symlinks,foreign volumes and Git boundaries retain the shared producer guards.
Both renderer children use the guarded local tool,prepare local stores and disable source creation.
The baseline may be warning-free or emit only the expected large-index warning;the chapter candidate
must emit none. A failed child,
unexpected warning or changed source cannot publish successful measurements;its partial capture
remains distinct for diagnosis.

The read-only comparison requires the exact current complete source hashes,all registered
chapters and equal stored chapter words and actual indexed terms. It reads data without executing
search JavaScript. It supplies search-coverage evidence,not browser application or release approval.
The focused capture watcher separately checks warning/warning-free section baselines,five writes,five failure
paths,the original/fixed ownership collision,thirteen path/input refusals and nine actual body
faults. Those simulated children do not count as native renderer evidence.

## Upstream field-isolation diagnosis

The0814 D372 diagnosis measured the0813 corpus:a12993569-byte JSON index with118 chapter records.
The body index occupies5318916 bytes and the breadcrumb index5321651 bytes,although stored
breadcrumb text totals10580 bytes. Every body term/document pair also occurs in breadcrumbs.
Run the read-only `python3 -I -B docs/tasks/artifacts/book_publication/search_index_census.py`
through the local producer profile to rederive the actual current sizes and coverage.

The upstream [field-isolation fix](https://github.com/mattico/elasticlunr-rs/commit/b7efca70afbe7bbed0a428f8cfff2abff93270e3)
moves token frequencies inside the field loop and adds independent field/frequency regressions.
[Published3.1.0](https://crates.io/crates/elasticlunr-rs/3.1.0) follows that fix.
0815 built published mdBook0.5.4 locally with that dependency;0817 selects it explicitly for Make,
publication and diagnostic capture. No upstream source is patched. Full text,chapter routes
and unexpected-warning refusals remain required. Search ranking and reader latency are unmeasured.
The old ambient renderer remains a diagnostic baseline;normal builds never select it. Actual query
verification remains separately owned after activation. No warning is hidden or reclassified.

## Isolated renderer checkpoint

0815 built the published mdBook0.5.4 package in target/cargo-home;Cargo resolved and compiled
elasticlunr-rs3.1.0. No external executable,repository or dependency source was modified.
The native field probe authors a tiny book with distinct title,body and breadcrumb words plus
a repeated title word. The ambient renderer fails with body frequency2 and a body-only word in
breadcrumbs;the local renderer passes with body frequency1 and the word absent from breadcrumbs.
Both binaries report0.5.4,so a version string alone cannot establish corrected indexing.

```bash
python3 -I -B scripts/local_environment.py -- python3 -I -B docs/tasks/artifacts/book_publication/search_field_probe.py --output target/book_search_my_local_field_probe
```

Use a fresh capture path. The probe reserves it before writing all three fixture inputs and
retains the native build log and result,including a failed observation. `--ambient` selects the
read-only host renderer solely for a diagnostic baseline. The normal probe uses the explicit
target/cargo-home/bin/mdbook path;symlink,foreign-volume and nested-repository inputs refuse.

The isolated same-source118-chapter candidate has a7712945-byte payload and no renderer warnings.
Stored document fields,chapter URLs,search/display options and per-chapter stored words/indexed
term sets match the baseline exactly. Breadcrumb postings shrink to36944 bytes;field membership
and frequencies intentionally change to the upstream-corrected values. This is tool-build evidence;
actual query verification remains separately owned after0817 activation.

## Guarded local setup

```bash
python3 -I -B scripts/book_tool.py install
python3 -I -B scripts/book_tool.py version
```

Setup uses Cargo's published install API with version0.5.4 and the explicit target/cargo-home root.
It resolves compatible dependencies without the older bundled lockfile,then checks the installed
version and runs the native field probe in a fresh target/book_search_install_checks capture.
An existing same-version binary must pass that probe too. If it retains the older dependency,
`python3 -I -B scripts/book_tool.py install --force` rebuilds only the project-local executable.
Every tool/metadata destination is planned before installation;aliased or hard-linked metadata
and unsafe tool paths refuse before Cargo dispatch. Missing tools refuse before version dispatch.
The selector API returns the guarded explicit executable used by Make,publication and capture.
Selector/profile/binary hashes join complete build inputs;missing or unsafe tools refuse before
output preparation. `make book` publishes through the existing private renderer/warning boundary.
Host executables and shared caches receive no writes.

The environment probe suite watches four pre-dispatch path refusals and two exact argument/status
controls in book_tool_contract.py. Its fake children provide path/dispatch evidence;the native
post-install probe supplies field-isolation evidence. These are distinct checks.
All four new search/tool sources are registered in producer_entries.tsv. D376 repaired the
missing rows that made the complete source census refuse;the passing census remains inspection
inventory evidence and grants no runtime/locality signoff. Broader D156 integration is still owned.

