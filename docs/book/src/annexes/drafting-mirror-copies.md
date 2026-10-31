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
nonreuse before issuance. Explicit result retention/retirement/issuance entries follow next.

