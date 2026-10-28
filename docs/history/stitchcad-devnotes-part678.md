# Sealed engineering continuity — part678

- **Identity:** 55 lines, 5993 bytes, sha256:d5da7a6ff259addf36e02752b766c282220bd6a9e2e0c12a9b4e7c44331d6914
- **Coverage:** whole0519 task scope, lesson and changelog from 5da6b27f5e689b3882319eb8a5cbb34580924441.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 19917–23647, 3730 bytes, sha256:025cd93786aabac643319bea9c0860b26396be92c9e078a36781a8a5b55a3d3c
- Whole source interval: DEV_NOTES.md bytes 193–1481, 1288 bytes, sha256:3253f848339e65dc29a8a8f9419e082c230b22156f2e06551815a2755355521f
- Whole source interval: CHANGELOG.md bytes 207–1182, 975 bytes, sha256:3ab79a6387993aa559bb38cbd7867bd75fb1d91639d34f943ac52c53599f25b8
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.5.3.2.3.2.4.3.4.2.integrity`
  Status: `done`
  Goal:0519 immediate P1 repair of0518 omitted book control and false-green commit gate. Source
  quantity implementation/checks remain committed;fresh control uses this new repair owner.
  - [x] REPRODUCE / ISSUE: 0519 git show00e1d74:.doctrine/live_document_size/
    maintained_reference.jsonl retains0517 target1515568B while actual committed book is1519183B.
    Correct unstaged0518 authority929B/SHA256721690108705542f99542b88d00199965eaba9d0d3cf6ab9f1114ef74978da35
    preserved in artifacts/live_doc_size/0518_unstaged_authority.jsonl;Python exact byte/hash check
    →pass,rc=0. New maintained_reference.py --index on the actual old index refuses target,rc=2.
  - [x] ROOT CAUSE: 0519 native --index reproduction →target refusal,rc=2;working-tree verifier
    and pre-commit source read confirm original enforcer judged work bytes while Git committed
    earlier staged bytes. Preparation exec yielded a session whose ID/result was discarded by
    reading only output;dependent stage/commit began early. HEAD/status/empty brief checked before
    repair;OS-visible census found no owned preparation/commit still running. No product test failed.
  - [x] FIX: 0519 read-only --index uses bounded native stage-zero object listing and immutable
    blobs for exact book/control/registry/task authority,existing counts/profile/fresh-owner checks
    and final listing stability. No Git write,index mutation or temporary repository. Final hook
    invokes it after working-tree doctrine checks. COMMIT now explicitly consumes producer terminal
    result before staging. Focused index contract7 →OK,rc=0;standing live-doc probe runner owns reruns.
  - [x] ADDRESSED: 0519 python3 -I -B docs/tasks/artifacts/live_doc_size/
    maintained_reference_index_contract.py →7 tests OK,rc=0. Older control/new book,unstaged task
    completion,missing chapter,changed staged profile,index drift,nonregular/merge-stage/duplicate/
    malformed native records refuse. Exact staged fixture passes with native status unchanged.
    --index cannot select candidates or record authority;actual control unchanged. Native old-index
    reproduction refuses target,rc=2. Fresh actual native --index →index-candidate,91 files/
    21705 lines/1519832B,correct repair owner,rc=0. Final hook reruns after all preparation.
  - [x] NO REGRESSION: 0519 maintained_reference_contract.py Candidate.test_candidate_matches_
    native_counts_and_reader_is_read_only and Candidate.test_caller_git_context_cannot_redirect_
    native_authority_or_change_parent_environment →2 tests OK,rc=0. bash -n hook/runner →pass,rc=0.
    Book/publication90/211/1572/2706,map unchanged,677 receipt3 exact Git spans,retention1131 logical/
    36 working/63439 decodedL/4898183 decodedB/2475520 residentB →pass,rc=0. Fresh authority is
    recorded after this owner completes and terminal result consumed before stage. Product/broad/
    full hook and pre-push verification retain their required scopes;no earlier CI verdict invented.
  - [x] LOCKSTEP: 0519 original omitted authority preserved;whole0518 scope/lesson/changelog in677.
    Native first commit00e1d74 remains recorded;repair neither amends history nor reuses closed
    authority. New exact book authority/control,COMMIT/DEV/CHANGE/MEMORY/book/canonical decision/
    task index/D301 align. LIVE/README reviewed unchanged,G1 6/18. Resume .3.4.acceptance after
    successful commit/empty brief/clean tree. No foreign repository changed.
    promotion: declined (mechanical repair of indexed exact raw-byte/fresh-task commit authority;failure evidence retained and canonical answer updated without a new policy or map row).
## _(2026-10-28 UTC)_ — repair staged book authority and preparation sequencing

0519 fixes current commit integrity:0518 preparation yielded,but output-only handling discarded its session ID and stage/commit began early. Commit00e1d74 kept0517 control while working verifier saw fresh unstaged0518 authority. Product checks passed;exact929B omitted authority retained,whole0518 in677.
Read-only --index verifies exact bounded native staged book/control/registry/task blobs,ordinary stage-zero modes and stable listing. Final hook invokes it after working doctrines. Shares freshness/raw-byte checks;COMMIT consumes preparation terminal result before staging. No index or temp repo writes.
Actual old-index CLI refuses target2. Focused7+existing reader2/shell syntax pass0;book90/211/1572/2706,map/retention1131/677 receipt3 spans pass0. Fresh control uses new repair owner,not closed0518;no history amend. Original evidence remains exact and standing probes own the index controls.
LIVE/README unchanged,G1 6/18. Resume actual Design quantity integration after clean commit/empty brief. Promotion declined:mechanical enforcement of indexed exact commit authority;current canonical answer updated. Always print full exec metadata and consume terminal result before dependent mutations.
## STITCHCAD-G1-0519 — repair staged book authority after preparation race

- Preserve exact omitted0518 authority929B and whole0518 scope/lesson/changelog677. Earlier00e1d74 feature commit kept old control because yielded preparation result was discarded and staging began early;working-tree gate saw fresh unstaged bytes. Product verification remains
  valid.
- Read-only staged book/control/registry/task verification now runs as final pre-commit gate,with bounded immutable native blobs,ordinary stage-zero modes and listing stability. Actual bad index refuses target2;focused7/existing reader2/shell syntax pass. COMMIT explicitly waits
  for preparation terminal result.
- Fresh control uses new completed repair owner;no history amend or closed-owner reuse. Book90/211/1572/2706,map/retention1131/677 receipt3 exact Git spans pass. LIVE/README unchanged,G1 6/18. Actual Design notch quantity integration resumes after repair;full pre-push gates
  remain required.


