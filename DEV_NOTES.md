# DEV_NOTES.md

Detailed technical notes — root cause, implementation, validation — per slice.

## _(2026-11-06 UTC)_ — D391: the census looked once

Probes failed on a process that was gone when I went to look,and the line had no command in it. I guessed a Chrome helper;eight
launches left nothing behind. A sampler named it:iTerm's status bar reads the checkout about every ten seconds. Then my two-second
second look failed too,and the log showed why:under an lsof hammer one reader lived past 21s. Measured with ps alone it never passed
0.9s,so the hammer made that tail,but contention is real and the wait now covers it. The same log also found a dying descriptor and
an undecodable ps line. Earlier full runs passed partly by luck:one look blocks about 15% of the time on this machine.

## _(2026-11-06 UTC)_ — D390: six checks outlived what they watch

The suite stopped at one missing inventory row;running its47 steps independently showed five more. Most were values:a
ledger case count,a fault's new file,two chapters linked since0655. One reversed my own D385 note above:deriving the
example count made the script read the book before it activates,and the producer contract forbids that. My first repair
split the entry in two,which the same contract also refuses,and rightly:a plan must be whole before any read. So the
count is pinned again,now with a message saying what to update. A pinned count costs one edit;a late plan costs the rule.

## _(2026-11-06 UTC)_ — D386: a rule that runs once per push reports months at once

The glossary census passed at0655 and sat at794 failures by this push,because nothing between pushes ran it. Each
chapter now carries its own declaration table,generated from the rustdoc of the item it names and reviewed by reading
every row;names shared by many refusal enums got a chapter-level meaning instead of an arbitrary first match. Rustdoc
links were stripped because they do not resolve in the book. The new gate takes4.5s per commit. Running my stricter
link rule against the real tree found two dead links publication cannot see,since it only inspects .md targets.

## _(2026-11-06 UTC)_ — D385: a set -e suite shows you one failure at a time

Running the80 formula_structure steps independently found9 failures at once,where the suite itself reported one.
0836 moved namespace collision logic into check_declaration:five namespace and four ordered-name anchors vanished or
doubled. Their unique-anchor assertion did its job,but only probes run it. Re-anchoring on behaviour rather than text
is proved by the full compiled runs,where every fault still reaches a real assertion. Separately,a fixed count of4
book examples had quietly stopped the contract;deriving it means the10 newer examples now compile and run too.

## _(2026-11-06 UTC)_ — D384: the commit gate does not render the book

0968 and 0971 both passed the commit gate with a book the publication check refuses. An HTML comment at the start
of a line opens a raw block in CommonMark,so my rust marker turned the span after it into a malformed tag;moving
the marker inside the sentence keeps it inline. A link out of the source tree fails the same way. Two chapters
had also lacked topic-index rows since0871/0874. G0-19 tested for the rendered folder before building it,which
only passed because an earlier build had left one behind;a cleanup or fresh clone showed the ordering bug.

## _(2026-11-06 UTC)_ — D383: count suites by exit status,not by failure lines

I first read the probes log as four failing suites by counting failure marks. A fifth had also failed:a set -e
suite stopped at its first step,so its later steps never ran and it printed no mark at all. The recipe's one rc bit
could not say which. It now names failing paths. The stop itself came from copied-book fixtures,which keep their
last copy and refuse entries the current book lacks;a chapter removed in0826 stayed in11 copies until the next
full run. That guard is right to refuse,so the repair moves the stale files aside and documents the recovery.

## _(2026-11-06 UTC)_ — D382: moving an index moves the rule's input

0970 repairs a check that had been red since0779 without anyone seeing it. That slice retired the changelog's
index into record954 to stay under its ceiling,and its book section said the live file pointed there;a later
prose edit dropped the link,and the rule still read live rows. Only make probes runs that rule,so the failure
waited for the next push. The new row is a real link,and the rule follows one declared record rather than any
linked record,so an ordinary lesson link cannot satisfy it. Removing the row refuses,and a not-followed mutant
goes red through a new contract case that an always-failing arm could not have distinguished.

## _(2026-11-06 UTC)_ — D381: an inventory row is not the producer guard

0966 reconciles the producer census before the due push. 370 maintained sources had369 rows;8 effect and
11 file rows were absent,and history moves had dropped the two n16 `Paths:` fields that native ownership
parses. The census stops at the first refusal,so the later drift only appears once the earlier row exists.
Reading the unlisted source found a separate defect:plan_sources.py guarded its scratch but passed ambient
stores to Cargo. A fake Cargo captured the escaped CARGO_HOME;after entering the shared profile it captures
target/cargo-home. The existing census refuses any later unlisted source by path.

## _(2026-11-05 UTC)_ — D380: occupied memory is not cache-adjusted used RAM

0964 corrects the19:36UTC top-based refusal. The top23G observation was real;interpreting it as
application/wired/compressed use above95% was wrong. Independent Activity Monitor reported20.44
of24GB plus3.60GB reusable cache. Those captures are not simultaneous measurements or exact CLI
calibration. Apple's category definitions and Stats' published VM-category arithmetic distinguish
the metrics. A new read-only diagnostic names its cache-adjusted estimate,units and timestamp;
it retains wired/compressed use,subtracts only purgeable/file-backed cache,and refuses bad data.

Frozen raw counters independently derive1288197 used pages from1533628 occupied/245431 cached.
Replacing the actual subtraction with the old occupied metric produces an assertion RED.
Pressure,exact86/88 thresholds and malformed/oversized reserve controls pass. The initial invalid
cache fixture was still physically valid;increased it beyond occupied pages and named failed arms.
Live21:34UTC estimate75.919%,projected84.253% for2GiB:within budget. This is captured evidence,
not a current baseline or exact Activity Monitor replica. Self-test joins existing producer probes.

RAM authorization stays valid. Resume/task records remove the incorrect blocker;G1still6/18
In Progress. Earlier top captures remain history. Existing inventory-reconciliation owner still
blocks pre-push signoff:current census refuses the previously recorded plan_sources.py omission.
The new diagnostic has explicit source/file ledger rows;no full producer or Rust verdict claimed.
The first commit hook refused the added TOOLBOX row at its fixed byte ceiling. Removed that new
row;the canonical book section and resume pointer retain the diagnostic route,without a cap change.
Promotion:the book's RAM admission section owns the rule and explicitly named estimate semantics.

## _(2026-11-05 UTC)_ — bounded placement reference implementation contract

0962 derives the next source-bound stage from existing bounded point/coverage producers and actual
ownership consumers. The book records complete query,retained-portion,structural resolver-visit
and ownership-scan bounds,checked before queries or derived allocations. An allocation-free
greedy union check prevents endpoint-only acceptance of missing interior material. The original
source/member/work owners supply the notion,Piece and ledger;no replacement parent is accepted.
One-record and complete-class implementation/checks remain pending under explicit child owners.

No Rust command was started:OS-visible host census reports21G used/2078M unused on24GiB,so the
director RAM policy defers compilation. Director will free memory and confirm a quiet window;
recheck<=80% before starting one worker. Planning changes receive structural/document checks and
mandatory commit gates only. Whole0960 acceptance moves byte-exact to its existing evidence sibling.
README/LIVE unchanged,G1still6/18 In Progress;no new runtime verification or feature closure.
Promotion declined:existing bounded resolver,original-source and finite cursor contracts apply.

## _(2026-11-05 UTC)_ — complete original Closure placement source joins

0960 borrows disjoint original operation/input/cursor and stable complete placement-work fields.
N+1 atomic reservation precedes fresh whole aggregate charging,result reservation and existing
original bind_source factories. Every role preserves its actual work pointer,canonical notion,
current copy/Piece and raw import/recipe/frame/snapshot/creator loans. First late typed causes
retain original work/index/returned response;private prefixes drop. N0 is idle and releasing
either outcome refunds no work. An internal result alias keeps the exact borrowing type readable.
The copied unused continuation was removed after strict library Clippy refused it;no waiver.

Ten focused cases cover actual imports,repeated/multiple roles,N0,whole shortage,aggregate/late
resources,late provider refusal and Piece/frame/snapshot mismatches. Existing import/Mirror/Join
association4,new API3,strict library/test-target lint,formatting and book build pass. Whole closed
0959 owner/proof moves verbatim to existing evidence. Book/LIVE/resume align;README/G1 6/18 stay
unchanged. Bounded current references,aggregate recipe/native/cohort and whole .check/.verify
remain pending. Promotion declined:existing original-source and finite cursor contracts own scope.

## _(2026-11-05 UTC)_ — complete Closure placement members and current copy work

0959 consumes complete prior operation/Pocket source proofs and reserves checked2N+1 aggregate/
member/component passes before fresh whole charging,N-result reservation and child factories.
Original instance fields expand First then Second. Targets derive only from0958 canonical notions;
existing component work supplies actual current CutPlan/Piece loans. No external component,value,
current anchor/direction or geometry query runs. Both outcomes retain the exact prior proof and
same cursor;private prefixes drop and into_parent refunds nothing. N0 is idle.

The first strict library lint found an unused future helper and oversized refused variant.
Removed the unused helper and boxed only refused member context,with no lint exemption. Public
contracts cover nine success/shortage/aggregate/late-member/late-component/missing-notion/missing-
copy/empty/multiple-instance cases. All7 affected Closure contracts and3 API guards pass,as do
strict library/test-target Clippy,formatting and book build. Whole0958 acceptance/0955 owner move
verbatim to existing evidence before growth. Book/LIVE/resume align;README/G1 6/18 unchanged.
Original sources/current references,native/cohort and .check/.verify remain pending.
Promotion declined:existing original-member/component-work and finite cursor contracts own scope.

## _(2026-11-05 UTC)_ — original Closure placement member relations

0958 freshly charges complete actual garment/Root/provider content and admits checked (E+8)^2
before original Closure,local instance and canonical notion scans. Explicit IDs/First-or-Second
select actual loans,not replacement bodies or programme positions. Partial requiring loans remain
typed diagnostics;no current copy/Piece/source/anchor/direction or physical proof is claimed.
Focused runtime contracts cover both roles,reordered notions,multiple instances and nonzero
original indices,Missing and quota priority. A valid First member can remain inspectable while
Second is missing;the explicit proof scope does not imply complete Closure currentness.

Strict affected-target lint found panic/indexing errors in preceding Pocket fixtures and the new
operation/cohort/member tests. Library-only lint did not cover them. Checked get/first loans and
typed test failures now pass without exemptions. Counting every attempted component query and
asserting zero in current-member work preserves detection formerly provided by a panic. This is
owned and resolved in the current leaf;strict affected test lint is now part of relevant checks.

Closure6/Pocket2,API3,arithmetic1,strict library/test-target lint,formatting and book build pass.
Whole closed0957 owner/proof moves byte-exact to existing evidence before working-part growth.
Book/LIVE/resume align;README scope/G1 6/18 unchanged. Placement copy/source/reference,native/cohort
and whole .check/.verify remain pending. Promotion declined:existing original-member/strict lint
contracts own these facts.

## _(2026-11-05 UTC)_ — preserve complete closed protocols before physical relations

0957 retains eight complete original Git spans fromfb02c244 in part1093 before physical relation
planning crosses fixed collection limits. Whole factory protocol/acceptance,original closed
bootstrap acceptance,recent relation plans/proofs and complete committed journals remain public.
Independent Git/hash/base64/public-reader checks precede narrative retirement;every original
live task identity/status/heading remains. All older archive seals and fixed bounds stay intact.

The independent decoder initially refused unjoined base64 lines before retirement. After joining
the declared line encoding,it proves exact source bytes. The graph guard then exposed only the
new authored retention owner;all original nodes remain exact and only that explicit addition is
permitted. Final public/source/retention checks pass. Product/book scope and G1 6/18 In Progress
stay unchanged;the book adds retrieval guidance. Original Closure physical relations remain next.
Promotion declined:existing whole-record retention and original graph contracts own scope.

[Whole earlier journal and navigation](docs/history/stitchcad-devnotes-part1093.md) retains exact committed bytes fromfb02c244.
