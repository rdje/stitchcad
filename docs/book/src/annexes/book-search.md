# Full-text book search

Search indexes every registered chapter's complete text and returns one result per matching
chapter. The supported heading-split-level=1 setting keeps the browser index compact;results open
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

