# Actual notch source context

Complete actual input admission is implemented. Bounded local source validation and original-notch
context construction remain pending;the [point and interval primitives](command-notch-context.md)
are implemented with focused checks. Quantities and paired effects retain their subsequent owners.

| API token | Meaning |
| --- | --- |
| `NotchContextLimits` | Explicit complete dependency and later per-query point/range limits |
| `NotchSourceAdmission` | Borrowed actual source/plan/Pieces/registry/ledger after input admission |
| `NotchSourceAdmissionError` | Fixed original source identity and actual admission refusal |
| `NotchSourceAdmissionIssue` | Original intent or complete journal/dependency population refusal |
| `NotchSourceCollection` | Actual source,copy,Piece edge/hole/fold or landmark collection |
| `admit_notch_context` | Admit complete actual source dependencies before indexes or derived work |
| `SeamEaseSource` | Actual retained original declaration,domains,copies and full stops |
| `CutPlan` | Actual complete physical-copy plan |
| `Piece` | Actual immutable source Piece with held interval references |
| `SewingLandmark` | Actual Notch/Turn registry entry |
| `IdentityLedger` | Actual typed topology journal |
| `OriginalPointLimits` | Explicit current-input,candidate and integer limits per inverse query |
| `BetweenNotches` | Original side and named notch identities,without supplied coordinates |

## Admit complete actual source dependencies

Original-notch context input admission is implemented. Supply the actual retained original source,
complete physical-copy plan,Piece registry,semantic Notch/Turn registry and topology ledger. The
returned token borrows those exact objects,without a copied declaration,synthetic graph or supplied
notch coordinates. It retains the declaration's actual side and named start/end IDs in named order;
input admission does not reorder them by coordinate or alter the original authored stop list.

Choose positive dependency-item,original-point and range-portion limits explicitly. The point
limits include complete journal/nested input and per-point candidate/integer budgets. The range
limit is the peak portion budget per later coverage query. These respective query limits are
retained for context construction;this input-only operation performs no projection or range query,
reserves no derived numeric result and does not certify an aggregate allocator or CPU-time bound.

Eligibility comes first. Uniform intent returns its actual source identity;weighted intent also
retains its actual density side and complete region count,without cloning the density. For an
actual BetweenNotches source,complete journal/nested admission precedes the dependency census.
Then charge every original source stop,every plan copy and every supplied Piece. For each Piece,
charge its whole boundary,hole-loop collection,each hole's whole edge list,construction lines and
fold declarations in schema order. Finally charge the complete supplied semantic registry,
including unused Notch and Turn entries. Filtering down to the two named notches would omit actual
source dependencies. Each collection is admitted before traversal;quota equality is accepted.

A refusal retains the original source span,actual collection and source/Piece owner,authored Piece
or hole index,prior consumed count,whole next request and selected limit. Journal refusals retain
the original operation and nested collection. There is no partial index or truncated registry.
Private exhaustive source/plan/Piece/mark views and Copy guards expose future owned-field changes.
Piece label and Material reason text stays borrowed without scanning or cloning;its bytes are not
part of this dependency-item census. No formula is evaluated and no physical profile value is
supplied. Explicit,formula and profile amounts all preserve their actual source declarations.

```rust
use core::num::NonZeroUsize;
use sc_core::ontology::{CutPlan, IdentityLedger, NotchContextLimits, NotchSourceAdmission,
    NotchSourceAdmissionError, OriginalPointLimits, Piece, SeamEaseSource, SewingLandmark};
fn borrow_actual<'a>(source: &'a SeamEaseSource, plan: &'a CutPlan, pieces: &'a [Piece],
    marks: &'a [SewingLandmark], ledger: &'a IdentityLedger,
    dependency_items: NonZeroUsize, point_limits: OriginalPointLimits,
    range_portions: NonZeroUsize)
    -> Result<NotchSourceAdmission<'a>, NotchSourceAdmissionError>
{
    let limits = NotchContextLimits::new(dependency_items, point_limits, range_portions);
    source.admit_notch_context(plan, pieces, marks, ledger, limits)
}
```

Admission success proves complete input populations,not ownership,currentness,unique original notch
positions or quantity availability. The immutable borrows prevent changing the supplied sources
while the token remains in use. Bounded Debug reports fixed identities,limits and population counts
without traversing label text,Notch definitions or the journal body. Current graph validation can
still refuse a deleted source edge while this input token preserves the actual held source;the
Uniform-only amount helper still reports its original BetweenNotches context requirement.

Focused native checks use actual validated graphs,original sources,Pieces,CutPlans,Notch/Turn
objects and ledgers. They cover symbolic declarations,named versus authored stop order,complete
unused registry/Piece/copy dependencies,hole/fold boundaries,journal refusal precedence and damaged
held-source preservation. No-default and immutable-borrow compile-fail checks cover the public
boundary. Actual source validation and original-coordinate context construction remain the next
owned step;their success and notch quantity availability are not implied by this input token.

## Next local source-validation contract

The next implementation validates this original source against the supplied actual inputs. It must
check current copy/Piece binding and local identity uniqueness,then every registry landmark,
including unused entries. Each anchor needs bounded unique current resolution and membership in
its actual Piece's surviving intervals. Full original side ranges need complete positive coverage,
unique endpoints and actual copy/Piece ownership. Same-copy positive interiors cannot overlap.
Every full original stop needs the actual side/Piece/current-interval membership;named boundaries
must still be semantic Notches and have distinct current locations.

This is local original-source validation. It has no supplied graph identity and must not invent
one or reconstruct a synthetic graph. Other graph members,global Design/root ownership,current
formula metadata and physical correspondence retain their actual separate validators. Surviving
Piece intervals supply local membership;this does not strengthen the existing contract into a
whole-Piece topology certificate or erase repairs elsewhere. Current Design sewing validation
continues to validate its actual complete root,all landmarks,full graph and amount metadata.

Derived interval/index work must be admitted from complete actual populations and explicit limits
before allocation. Use bounded current/coverage queries,not the legacy endpoint choice trees.
Retain actual owner,held reference,operation,choice,repair and numeric causes. A current unique
position can still project to multiple original coordinates/directions,so complete source success
cannot imply a usable original notch interval.

## Next original-coordinate contract

Project the actual two named Notch anchors into the actual original distribution-side creation
frame. Retain every ambiguous candidate and mapping direction;never select by proximity or compare
directions from different holder frames as absolute point identities. Wide valid coordinates stay
exact even when bounded views refuse. Compare them to the original full side bounds without
narrowing,require distinct in-domain coordinates,and keep named IDs/authored stop order while
ordering the mathematical interval. Admit comparison and later amount arithmetic separately.
Symbolic amount/profile declarations remain unresolved;no quantity or physical value is defaulted.

The canonical decision is docs/decisions/decision_sewing-spans-address-copies-and-permit-disjoint-self-seams.md.
Its whole pre-0512 source is logical record docs/history/stitchcad-devnotes-part670.md,available
through the [public archive reader](../annexes/archive-reader.md). It retains the superseded
prospective wording and all historical evidence exactly. Both next contracts above are implementation
requirements;this page does not claim they have passed or are available.
