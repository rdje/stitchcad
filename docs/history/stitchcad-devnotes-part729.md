# Sealed engineering continuity — part729

- **Identity:** 43 lines, 4886 bytes, sha256:33f0121f8078105f3d2af052d89aeac22630be5cb54da7c58b560a255029ca06
- **Coverage:** whole0570 task scope, lesson and changelog from 84970313e979e7e17eea735a1c309bfe460a0fc4.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 25254–28192, 2938 bytes, sha256:da2fa767c6c8a5c58420d336d087c14df3aa94ea8fc5fc36e7fd42972db889a8
- Whole source interval: DEV_NOTES.md bytes 193–1488, 1295 bytes, sha256:a220c15d6b34c2f9b8acdc1de0100a211d03cbc362dcc0eddda0774b0c4202a2
- Whole source interval: CHANGELOG.md bytes 207–860, 653 bytes, sha256:98a1f1a307b8dac6cc45c0647419a3ce21fb82dd1116f799dbd0da49cbc8587c
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.5.3.2.3.4.3.1`
  Status: `done`
  Goal: extend the actual accepted history by one split without rewriting its prefix or original
  snapshot. Admit complete next entry/issued-ID populations before any vector copy;preserve quota,
  parent and first-child/second-child refusal precedence and original birth/split locations.
  Supplied child IDs remain metadata,not generator/global freshness or execution authority.
  Requirements ROADMAP4.1/4.4,ontology1/1.1 and entity-identity decision read;existing history,
  graph ownership/resource,source/stop joins and native history contracts reviewed. Six current
  history contracts establish immutable supplied history;there is no bounded extension API yet.
  Own smallest native extension/current-history checks,scoped lint/fmt/doctest/book and commit.
  Complete candidate/reference and full paired proof remain explicitly owned below/later.
  Completion0570:with_split builds one new admitted immutable identity snapshot.
  - [x] ROOT CAUSE (WHY + WHERE): rg public methods in ontology/seam_span_history.rs showed
    only new/entries/identity_count/state;no accepted-prefix transition. Baseline local launcher
    cargo test -p sc-core --test seam_span_history_contract →six cases pass,rc=0. Candidate
    construction needs full next populations admitted before copying an existing history.
  - [x] ADDRESSED (verified): local_environment.py cargo test -p sc-core --test
    seam_span_history_contract →nine pass,rc=0. Three new extension cases preserve exact prefix,
    old snapshot,unrelated roots,full-width descending child IDs and policy-independent equality.
    Quota→parent→first/second child refusals retain original positions. A120-case component
    comparison agrees with separate existing full-map validation;no new global-ID oracle claim.
    All validation precedes one admitted vector copy;no second map/generator/current callback.
  - [x] NO REGRESSION: original six history cases remain green,rc=0. cargo test -p sc-core
    --doc ontology::seam_span_history →five pass,rc=0;scoped strict Clippy library/history test
    and cargo fmt --all --check pass,rc=0 after touched-file whitespace formatting. New book
    example byte-exact to passed doctest;positive book94/255/1595/2788 and glossary310/9/158/0
    pass,rc=0. Receipt728 →three exact Git spans,rc=0. Full paired acceptance remains deferred.
  - [x] LOCKSTEP: lineage book/MEMORY/DEV/CHANGE/LIVE/index/task synchronized;whole0569 scope,
    lesson and changelog retained728. LIVE reviewed:G1 still6/18,history focused6→9;G0 human
    closure unapproved. No generated/reserved identities,actual graph mutation or authority.
    Next .3.2 actual source/stop/graph candidate join;then .3 held references/global integration.
    promotion: declined (routine application of indexed immutable snapshot,complete preallocation admission and nonreuse contracts;no new cross-cutting rule).
## _(2026-10-29 UTC)_ — extend accepted span history without changing the old snapshot

0570 with_split admits complete next entry/issued populations before any copy,then parent liveness and original first/second-child nonreuse. Copy the exact accepted prefix once and append one record;no second map,generator or current/physical query. Original snapshot remains exact on success/refusal.
Native history9/doc5 pass,including120 component comparisons against complete map validation and exact old prefix/unrelated roots/descending full-width IDs. Scoped strict Clippy/fmt pass after touched-file whitespace fixes. Limits remain runtime inputs;no reservation/global freshness/actual graph effect.
Book example matches passed doctest byte-for-byte;positive book94/255/1595/2788 and glossary310/9/158/0 pass. [Whole0569 retention proof](docs/history/stitchcad-devnotes-part728.md) retains scope/lesson/changelog,three exact Git spans. LIVE G1 6/18,focused history6→9;G0 human closure unapproved.
Indexed immutable admission/nonreuse rules apply;promotion declined in leaf. Resume actual source/stop/graph candidate join .3.4.3.2,then held references/global integration. Real provider/physical/authority/effect/full paired proof and D334 P2/D156/D223/PLANNING.5 remain owned;full PNT continues.
## STITCHCAD-G1-0570 — extend immutable accepted span histories under explicit bounds

- with_split checks complete next counts,parent liveness and all issued child IDs before one prefix copy. Original history remains exact;new child order and refusal locations retained. No ID generation/reservation,global freshness or actual graph effect.
- History9/doc5/scoped strict lint/fmt pass;120 comparisons agree with existing complete-map validation. Book example exact;book94/255/1595/2788/glossary310/9/158/0 pass. Whole0569 retained728/three exact Git spans. LIVE G1 6/18,history6→9;G0 human unapproved. Actual candidate/reference integration next.

