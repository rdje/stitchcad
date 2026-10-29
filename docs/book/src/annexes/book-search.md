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

