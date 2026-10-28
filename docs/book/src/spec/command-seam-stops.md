# Semantic stops at paired seam cuts

Native contract cases now verify the existing graph's stop membership at paired cuts,including
closed endpoints,explicit side/copy identity,authored order and later-frame anchors. The bounded
production stop partition stage is still pending. These cases construct real retained child graphs
through public APIs;they do not install a command effect or create new semantic landmark identities.

| API token | Meaning |
| --- | --- |
| `StopLandmark` | Explicit side and existing semantic landmark ID |
| `SewingLandmark` | Actual semantic Notch or TurnPoint in the registry |
| `Notch` | Semantic matching mark with separate deferred physical representation |
| `TurnPoint` | Born-valid semantic edge anchor with geometric turning deferred |
| `SeamSide` | Actual physical-copy identity and positive original interval |
| `SeamSideId` | Explicit A or B label;never inferred from a matching edge |
| `SeamDirection` | Same or Opposite endpoint correspondence |
| `EdgeRange` | Positive interval whose membership includes both endpoints |
| `Param` | Stored bounded rational position;valid wide original views may refuse |
| `StopOutsideSide` | Actual stop cannot belong to the named child's side interval |

## A stop at the cut belongs to both closed children

For a side interval[0,1] cut at c,the two positive children are[0,c] and[c,1]. Existing graph
validation accepts a stop exactly at c on either child. The partition contract retains every closed
membership,so both children reference that same semantic object at their shared boundary. No new
Notch or TurnPoint is created. References to boundary marks carry no independently authored ease;
ease quantities remain original rounded cumulative boundary differences.

For example,with A cut1/2 and B cut2/3:

| Actual stop | Same direction: A-first/A-second | Opposite direction: A-first/A-second |
| --- | --- | --- |
| A at1/2 | both | both |
| B at1/2 | first | second |
| B at2/3 | both | both |

The pair order is always lower/upper A. Same pairs those with lower/upper B;Opposite pairs them
with upper/lower B. An arbitrary winner at a shared cut would lose a valid boundary reference.
The current graph constructor validates supplied child stop lists;it does not itself execute this
partition or enforce complete command conservation. The forthcoming partition stage owns that work.

## Preserve side,copy and authored order

A stop carries its named side and landmark identity. Its physical copy comes from that child's
actual side. A landmark on one Piece is not reassigned to another Piece because numeric parameters
or labels match. Different physical copies of one Piece remain distinct material domains.

Copy reflection and Same/Opposite correspondence remain separate metadata. A reflected B copy
still uses its own original B source frame;reflection does not silently complement a stored stop
parameter. Native cases retain copy200 on A and reflected copy201 on B while preserving direction.

Filter each child list in original authored order. If the parent lists Turn401 before Notch400,
each child that contains those stops keeps that order,including at a shared cut. Do not sort by
position,kind or identity. A semantic object can also appear under distinct A and B stop tags;
those explicit side references remain distinct even when the landmark ID is the same.

### Touching self-seam example

One physical copy may join disjoint intervals that touch at a boundary. For A[0,1/2] and B[1/2,1]
on copy200,a semantic landmark at1/2 can carry both explicit A and B stop references. The source
intervals have no positive overlap. With cuts A1/4 and B3/4,Same direction puts its B reference in
the first pair and A reference in the second. Opposite direction puts both references in the
second pair. The original authored B-before-A order remains intact. This is one semantic mark;
no new identity or material copy is inferred.

## Use actual current evidence

Born anchors and original source parameters can live in different topology frames. A real source
edge split at1/3 admits a new stop on its first fragment at1/9223372036854775807. Its exact original
position is1/27670116110564327421,which has a65-bit denominator and no stored Param view. The
native inverse diagnostic preserves that valid original coordinate. Existing graph membership
uses the actual current semantic point and resolved child interval coverage;it does not need to
narrow that original coordinate just to classify the stop.

The contract cases keep original A1/2 and B2/3 cuts,which are distinct from that topology split.
They build the actual current Notch and TurnPoint on the proper Piece and preserve their metadata.
A new anchor cannot be born on a retired source edge:the semantic constructor rightly refuses it.
The test captures its original definition before the edit,then supplies the real later anchor.

Actual choices,repairs,lost interiors,wrong sides and numeric refusals remain typed evidence. A
matching source ID or a surviving endpoint supplies no whole interval/currentness certificate.
The bounded production stage must admit its actual populations and point/range workspaces before
querying,retain all refusal context and return no partial accepted partition. Local membership
success will not supply physical correspondence,authority or atomic effects.

## Focused contract proof

Two native cases cover32 independent Fraction membership rows with four exact shared-cut controls,
65-bit later originals and28 wrong-child refusal controls. Four additional copy/direction scenarios
cover reflected copies and touching self-seams. Both semantic kinds,exact IDs,side tags,full original
stops and authored order survive. Physical Notch representation and actual geometric turning stay
deferred to their existing owners.

The new independent stop oracle is registered in the closed required oracle gate. Ten producers
match twelve frozen fixtures;the nine gate controls still pass. Bounded source/stop admission,
actual current membership and ordered borrowed partition views follow as separate owned slices.
Whole paired lineage/atomic effects and physical/mutation acceptance remain later work.
