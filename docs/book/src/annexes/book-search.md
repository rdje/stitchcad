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

The diagnostic capture renders the same complete source twice:the section-level warning baseline
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
Both renderer children use prepared local stores and disable source creation. The baseline must
emit only the expected large-index warning;the chapter candidate must emit none. A failed child,
unexpected warning or changed source cannot publish successful measurements;its partial capture
remains distinct for diagnosis.

The read-only comparison requires the exact current complete source hashes,all registered
chapters and equal stored chapter words and actual indexed terms. It reads data without executing
search JavaScript. It supplies search-coverage evidence,not browser application or release approval.
The focused capture watcher separately intercepts dispatch and checks five writes,six failure
paths,the original/fixed ownership collision,thirteen path/input refusals and nine actual body
faults. Those simulated children do not count as native renderer evidence.

## Current index warning and upstream field isolation

D372 remains open:the0813 corpus produces a12993569-byte JSON index with118 chapter records.
The body index occupies5318916 bytes and the breadcrumb index5321651 bytes,although stored
breadcrumb text totals10580 bytes. Every body term/document pair also occurs in breadcrumbs.
Run the read-only `python3 -I -B docs/tasks/artifacts/book_publication/search_index_census.py`
through the local producer profile to rederive the actual current sizes and coverage.

The upstream [field-isolation fix](https://github.com/mattico/elasticlunr-rs/commit/b7efca70afbe7bbed0a428f8cfff2abff93270e3)
moves token frequencies inside the field loop and adds independent field/frequency regressions.
[Published3.1.0](https://crates.io/crates/elasticlunr-rs/3.1.0) follows that fix.
The next owned steps build the published mdBook0.5.4 locally with that dependency,verify the actual
renderer output,then select it explicitly. No upstream source is patched. Full text,chapter routes
and unexpected-warning refusals remain required. Search ranking and reader latency are unmeasured.
Until resolved,the historical level3/level1 capture above refuses its now-warning chapter candidate;
that refusal is diagnostic evidence,not a successful migration or permission to ignore warnings.

