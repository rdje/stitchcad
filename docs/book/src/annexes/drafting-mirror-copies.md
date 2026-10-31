# Native mirror physical-copy contracts

Original [input/metadata](drafting-mirror-contracts.md) and [output/map requirements](drafting-mirror-outputs.md)
remain their canonical parents. These declarations supply no current Design or physical-effect certificate.

## Preserve the actual source physical copies

`context.preserve_source_copies(&plan,&limits)` associates an actual complete CutPlan with the
original source Piece held by RequiredMirrorPieceCorrespondence. It preserves every source
copy's actual id,definition,orientation and authored plan order. Copy ids never come from list
positions,and the method creates no result copies or physical transforms.

| API | Meaning |
| --- | --- |
| `MirrorPieceSourceCopyLimits::new(C)` | Explicit complete CutPlan population allowance |
| `BoundMirrorPieceSourceCopies` | Actual complete source-copy inventory under original source metadata |
| `MirrorPieceSourceCopyError`, `MirrorPieceSourceCopyRefusal` | Original context,whole plan,borrowed limits and first refusal |

C counts every actual plan entry,including unrelated Pieces and any existing result copies. It
must fit before any source-copy scan. Admission then checks the first unexpected source reflection,
the complete source quantity,and paired-population balance. MirroredPairs requires equal authored
and reflected source populations. Single and separately handed PairMember sources require authored
copies only. These checks use the actual original source Piece fields;the requested result's cut
quantity or mirroring cannot replace them. A plan's structural validity at birth does not prove
its association with this actual source metadata or current Design.

```rust
use sc_core::{construction::*,ontology::CutPlan};
fn preserve<'a>(context:&'a RequiredMirrorPieceCorrespondence<'a>,plan:&'a CutPlan,
    limits:&'a MirrorPieceSourceCopyLimits)
    ->Result<BoundMirrorPieceSourceCopies<'a>,MirrorPieceSourceCopyError<'a>> {
    context.preserve_source_copies(plan,limits)
}
```

For source quantity4/MirroredPairs,copies410 authored,207 reflected,900 authored and805 reflected
all remain. A separately chosen result quantity1/Single still requires these four original source
copies. Two unrelated copies raise C to6 without joining the returned source set. C5 then refuses
before source association. A valid birth plan with only two source copies cannot satisfy this
source quantity4;a valid authored-only birth plan of four copies cannot satisfy paired metadata.

`copies()` lends every actual source CutCopy in original order and supports reverse iteration.
Reordering records preserves their identities and orientations. Cached copy_count,authored_count
and reflected_count are immutable admission summaries,not another authored cut quantity. The
whole plan remains inspectable through plan(),including unrelated copies. Admission scans O(C)
with constant auxiliary space;each later iterator pass has its own scan responsibility. These
are population/work bounds,not allocation bytes or a complete execution budget.

No missing source copy is manufactured,and no source copy is selected for conversion into a result
copy. Existing source definitions,Piece,plan and held references stay unchanged. Current ownership,
global namespace/history,companion truth,actual reflection and all-copy/semantic effects remain
required execution evidence. The correspondence providers remain requested rather than accepted.
Explicit result-copy retention/retirement/issuance and complete companion association are the
following native steps;this source binding supplies no new identity or publication authority.

## Known identity exclusions for result-copy proposals

`source.bind_copy_identities(&limits)` prepares actual CutPlan copy lookup and known typed
exclusions before a result-copy proposal. It retains the complete source-copy/context owner.
Every actual plan copy is indexed,including unrelated Pieces and any existing result copies.
The index lends the original CutCopy by persistent id;no list position supplies an identity.

| API | Meaning |
| --- | --- |
| `MirrorPieceCopyIdentityLimits::new(C,R,P,W)` | Whole plan,source-occurrence,point-candidate and workspace allowances |
| `BoundMirrorPieceCopyIdentities` | Actual copy lookup and scoped known typed exclusions |
| `MirrorPieceCopyIdentityError`, `MirrorPieceCopyIdentityRefusal` | Actual source context,borrowed limits and first refusal |

The known exclusions include all actual copy ids and plan pattern-Piece ids,the declared source
and result Piece ids,both source/result companion ids when present,the actual mirror operation,
original edge/point/axis creating operations,and explicitly assigned source/result Materials.
Repeated source occurrences still consume their complete R/P input allowances;the identity sets
may share keys without dropping any earlier source/occurrence attribution.

A source/result/companion Piece id that also names a known creating operation or Material refuses
before plan record inspection,in stable Piece-id order. Each original plan record then checks
copy-id conflicts before pattern-Piece-id conflicts. A copy cannot alias a declared Piece or
known operation/Material;an unrelated plan Piece cannot alias a source-creating operation either.
The immutable CutPlan already supplies its own copy uniqueness and birth Piece association.

```rust
use sc_core::construction::*;
fn bind<'a>(source:&'a BoundMirrorPieceSourceCopies<'a>,limits:&'a MirrorPieceCopyIdentityLimits)
    ->Result<BoundMirrorPieceCopyIdentities<'a>,MirrorPieceCopyIdentityError<'a>> {
    source.bind_copy_identities(limits)
}
```

For C plan copies,R original source occurrences,P original point candidates and A actual axis
dependencies,checked2C+R+P+A+7 precedes C,R,P and workspace quotas,then any identity scan or
allocation. The workspace upper bound covers a C-entry copy index,at most C+4 known Piece ids,
and at most R+P+A+3 creating-operation/Material ids. The four Piece slots are source,result and
their two optional companions;the other three fixed slots are operation and two Materials.
These count entries,not allocation bytes,earlier model storage or a whole execution budget.
Work is O((C+R+P+A)log(C+R+P+A+2));no prefix index escapes any refusal.

With six plan copies,eight original edge occurrences,one point candidate and a two-point axis,
the entry bound is30. A C5 or W29 policy refuses before discovering any local identity conflict.
Existing copies410/207/900/805 remain excluded from new issuance even if a later intent retires
one of them. An axis-creating operation90 cannot also supply a Piece-companion identity90.
Source Material700 and result Material800 both remain excluded,as do independently named
source/result companions999/888,without certifying that those requested companions exist.

`copy(id)` returns an actual original plan record. `knows_piece(id)` reports a known typed Piece
reference or plan pattern id,without existence/currentness proof. `excludes_new_copy(id)` returns
true for these known conflicts. False means only unlisted in this local context;it is not global
freshness,issued-history completeness or permission to allocate. Zero is an ordinary explicit id,
not a missing-value sentinel. Unresolved evidence-provider ids remain requests rather than typed
entity declarations. The trusted host/current Design must still establish global namespace and
nonreuse before issuance. Explicit result retention/retirement/issuance entries follow below.

## Explicit result-copy intents

`MirrorPieceResultCopyManifest` declares an original operation and ordered Retain,Retire or Issue
choices. Admission against BoundMirrorPieceCopyIdentities produces a complete native proposal.
It issues no immutable CutCopy,retires no existing copy and edits no CutPlan or held object.

| Choice | Meaning |
| --- | --- |
| `Retain(id)` | Keep the actual old result copy's complete original definition and identity |
| `Retire(id)` | Explicitly request retirement of an actual old result copy;effects remain required |
| `Issue(CutCopyDefinition)` | Propose an explicit new id,canonical result Piece and physical orientation |

Retain and Retire must resolve to actual copies whose pattern Piece is the declared result.
Source and unrelated copies cannot transfer into the result,be selected for conversion or be
retired through this proposal. Issue must name the actual canonical result Piece and avoid every
known local identity exclusion. Unlisted still does not mean globally fresh or reserved.
Every actual old result copy needs one explicit Retain/Retire choice. Omission cannot silently
mean preservation or retirement,and an empty proposal cannot manufacture an initial result copy.

| API | Meaning |
| --- | --- |
| `MirrorPieceResultCopyIntent`, `MirrorPieceResultCopyManifest` | Whole original explicit choices and operation |
| `MirrorPieceResultCopyLimits::new(N,C,W)` | Complete intent,actual plan and workspace-entry allowances |
| `AdmittedMirrorPieceResultCopies` | Complete native proposal with actual inventory/context parents |
| `MirrorPieceResultCopyAction` | Authentic original intent and associated before/proposed definition |
| `MirrorPieceResultCopySource` | Borrowed payload data;the enum alone is no plan-membership proof |
| `MirrorPieceResultCopyError`, `MirrorPieceResultCopyRefusal` | Whole original proposal/parents/policy and first refusal |

Checked2N,whole N,whole actual C and workspace quotas precede operation comparison,all intent scans
and allocation. C includes source and unrelated copies. New workspace is an N-entry id index plus
N-entry associations;these count entries,not bytes or earlier inventory/index storage. Work is
O((N+C)log(N+K+2)) for K previously indexed identities. No accepted prefix escapes a refusal.

Each original intent first checks duplicate id across all choice kinds,then existing membership/
Piece or proposed Piece/known-id constraints. Active retained/new orientations use the actual
result cut policy. Single and PairMember accept authored copies only;MirroredPairs needs equal
authored/reflected populations. Retire contributes no active copy,so an old reflected copy may
be explicitly retired while changing to an authored-only result policy. First unmentioned old
result follows original plan order,then complete result quantity and pair balance are checked.
The source's different quantity/mirroring stays authoritative for its unchanged source copies.

```rust
use sc_core::{construction::*,ontology::*};
let raw=MirrorPieceResultCopyManifest::new(EntityId::from_bits(10),vec![
    MirrorPieceResultCopyIntent::Issue(CutCopyDefinition { id:EntityId::from_bits(500),
        piece:EntityId::from_bits(200),orientation:CopyOrientation::Authored })]);
fn admit<'a>(raw:&'a MirrorPieceResultCopyManifest,ids:&'a BoundMirrorPieceCopyIdentities<'a>)
    ->Result<AdmittedMirrorPieceResultCopies<'a>,MirrorPieceResultCopyError<'a>> {
    raw.admit(ids,MirrorPieceResultCopyLimits::new(100,1000,200))
}
assert_eq!(raw.intents().first().expect("explicit").id(),EntityId::from_bits(500));
```

An initial result quantity1/Single can propose Issue500 authored against a plan with no old
result copies. If old result copies550 authored and551 reflected exist,a new quantity2/Single
proposal `[Issue500 authored,Retire551,Retain550]` has one retention,one retirement,one issuance
and two proposed authored copies. All original source copies remain unchanged. Leaving551
unmentioned refuses even if the retained population otherwise matches the desired quantity.
`[Retire551,Issue551]` refuses repeated id;retirement never frees an old identity for reuse.

`actions()` retains every original choice and supports reverse iteration. `result_actions()`
excludes retirements while retaining request order;`action(id)` uses the persistent copy id,
independent of both proposal and old plan positions. Views lend actual original raw intent fields,
actual old CutCopy borrows for Retain/Retire,and the original raw CutCopyDefinition for Issue.
Issue never manufactures an immutable CutCopy. Request order is metadata,not permission to apply
intermediate mutations;actual publication must validate and commit the complete change atomically.

Counters summarize proposed retention,retirement,issuance and resulting orientations;none records
an applied effect. Original Piece,plan,copy definitions,source ids and held references remain
unchanged on success and refusal. Actual global reservation/nonreuse,current Design/ownership,
companion truth,all affected consumers,physical/geometry/semantic effects and publication remain
mandatory execution work. Native result-copy intents are complete;actual candidate inventory association follows below.

## Actual candidate Piece inventory

`AdmittedMirrorPieceResultCopies::bind_piece_inventory` associates the native proposal with an
actual immutable `PieceInventory`. Its original source member must equal the complete original
source definition. Its result member must match every canonical cut,material,layer,label and
contour field selected by the native declaration. This also checks construction repetitions,
hole groups,directed traversal and the cut-derived fold edge;no field is silently copied later.

| API | Meaning |
| --- | --- |
| `MirrorPieceCopyInventoryLimits::new(I,H,R,B)` | Whole inventory and complete inspected four-form allowances |
| `BoundMirrorPieceCopyInventory` | Actual candidate inventory,source/result members and native proposal |
| `MirrorPieceCopyInventoryField` | First mismatching canonical result field |
| `MirrorPieceCopyInventoryError`, `MirrorPieceCopyInventoryRefusal` | Whole original inputs,policy and refusal |

```rust
use sc_core::{construction::*,ontology::PieceInventory};
fn bind<'a>(copies:&'a AdmittedMirrorPieceResultCopies<'a>,inventory:&'a PieceInventory,
    limits:&'a MirrorPieceCopyInventoryLimits)
    ->Result<BoundMirrorPieceCopyInventory<'a>,MirrorPieceCopyInventoryError<'a>> {
    copies.bind_piece_inventory(inventory,limits)
}
let limits=MirrorPieceCopyInventoryLimits::new(100,1000,10000,100000);
assert_eq!(limits.pieces(),100);
```

I counts every actual candidate inventory member,including unrelated Pieces,and precedes member
lookup. Missing source then missing result precede content inspection. H,R and UTF-8 B cover four
forms:original source,candidate source,native desired result and actual candidate result. H counts
all hole groups before their child lengths are scanned. R counts every outer,hole,construction
and fold occurrence,including repetitions. B includes all four label fields and unresolved
Material reason text in all four forms. Checked totals and their quotas precede field comparisons.
Other inventory members' content and prior allocation retain their own resource obligations.

After resources,the complete original source definition is compared first. Result mismatches
report Quantity,Mirroring,CutOnFold,CuttingSide,Material,Layer,Label,Boundary,Holes,Construction,
then FoldEdges. No accepted prefix escapes. Admission allocates nothing,uses constant auxiliary
space and takes O(I+H+R+B) work. Whole inventory order and all original fields remain unchanged.
For the documented asymmetric fixture,three candidate Pieces consume I3,H8,R30,B220 with NoFold;
ReflectedSource consumes R32. Adding an actual source hole gives H9/R31 before SourceChanged.

`source_piece()` and `result_piece()` lend the actual candidate members;`copies()`, `inventory()`
and `limits()` retain actual whole parents. `source_companion()` and `result_companion()` resolve
the actual member's explicit companion id in that same inventory,with O(I) work per lookup.
Inventory birth already establishes reciprocal declarations,opposite hands and equal quantity.
An authored order `[result200,companion300,source100]` therefore resolves companion300 by identity,
without pairing list positions. MirroredPairs remains one pattern Piece with two orientations;
PairMember is a separate reciprocal pattern Piece and accepts authored physical copies only.

This stage establishes structural association,not reflected geometry or physical-copy pairing.
It does not authorize unrelated Piece-content changes,assert current Design ownership,reserve
identities,transfer neighbours or sew anything. Complete actual candidate CutPlan association,
whole consumer/effect evidence and atomic authorized publication remain separate requirements.

