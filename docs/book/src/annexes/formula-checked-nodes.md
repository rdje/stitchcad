# Actual checked formula nodes

`FormulaCheckedExpression` now retains one static certificate for every node of its actual
normalized expression. `root` exposes typed child views only after the complete static traversal
succeeds. Every view borrows both the proof's certificates and original expression owner. Callers
cannot attach another arena, supply a node index or create a checked prefix after a later failure.

This is static inspection. No input value, authored state, provider, geometry or numerical operator
is evaluated. A checked sin call has a resolved kind/callee; it is not a computed sine result.
A separate prepared scalar worker executes admitted checked expressions and ordered recipes,
including explicit-policy tangent. Geometry and full application acceptance remain integration work.

## Inspect actual branch certificates

```rust
use sc_core::recipe::{FormulaCheckedNodeForm as Form, FormulaExpression, FormulaKind as K,
    FormulaNamespace};

let expression = FormulaExpression::parse("if(is_base_size,(1 mm),(2 mm))")?
    .normalize_literals()?;
let namespace = FormulaNamespace::new([])?;
let proof = expression.check_kinds(&namespace)?;
assert_eq!(proof.node_count(), expression.node_count());
let root = proof.root();
assert!(std::ptr::eq(root.expression(), &expression));
assert_eq!(root.kind(), K::Length);
if let Form::Conditional { condition, then_branch, else_branch } = root.form() {
    assert_eq!(condition.kind(), K::Boolean);
    assert_eq!(then_branch.kind(), K::Length);
    assert_eq!(else_branch.kind(), K::Length);
    assert_eq!(then_branch.expression().node_count(), proof.node_count());
}
# Ok::<(), Box<dyn std::error::Error>>(())
```

Both branch certificates come from the successful traversal of that exact source. An executor can
therefore select a branch using its actual static proof, rather than promoting supplied callback
annotations into whole-expression acceptance. The value-level checked_if helper remains separate.
Root kind, canonical expression identity and the existing dependency slice retain their behavior.

| Node API | Meaning |
| --- | --- |
| `FormulaCheckedExpression` | Complete static proof over one actual normalized expression owner |
| `root` | Borrow the certified root node after complete traversal succeeds |
| expression | Actual original normalized owner |
| span | Original half-open source extent, including grouping |
| kind | Inferred quantity/Boolean/geometry kind of this actual node |
| operand | Ordinary kind or direct symbolic tolerance-name role |
| form | Actual literal/name/unary/binary/call/conditional with certified children |

FormulaCheckedNodeForm keeps the original node shape. Literal inspection returns the once-normalized
literal. A Name exposes its authored token and exact resolved FormulaDeclaration. That declaration
retains the real canonical input borrow, geometry reference or prior-let source; inspection performs
no new name search and does not prove registry/currentness or input availability.

Binary operands and conditional branches remain ordered. A Call exposes its actual FormulaBuiltin,
resolved before its arguments during checking, and FormulaCheckedArguments. The argument view offers
len, is_empty and an exact double-ended iterator in original source order. Every child retains the
same proof and arena. No copied AST or recursive ownership tree is introduced.

## Symbolic roles and source context

Grouping preserves a direct class name: `(eps_geo)` retains Tolerance(Geometric). A computed length
such as eps_geo+0um has Value(Length), even though its left child retains the tolerance role.
Static within checking still requires the actual direct reserved-name operand; these certificates
do not permit an ordinary length to impersonate a named class.

Name occurrences keep their individual original spans and resolved sources, including repetitions
and untaken branches. In a whole statement, spans use the original recipe coordinates and prior-let
sources retain their actual declaration ordinal. A detached node invents no statement ordinal.
Default Debug omits authored names and literal content; explicit form/name/literal inspection
deliberately exposes source to the authorized caller.

The normalized owner and source may outlive the proof. Its checked node/argument views cannot:
their certificates belong to the successful proof allocation. Constructors and fields
remain private; compile-fail examples watch forgery and lifetime boundaries.

## Account for actual retained proof buffers

`FormulaCheckedExpression::charge_retained_resources` appends the actual certificate-vector
length,then the actual ordered dependency-vector length,to a caller-owned cumulative resource
sink. The caller supplies each containing field's `DesignResourceLocation`. A shortage preserves
that original field,scope,owner,index and attempted complete length. An earlier sink refusal wins,
including when the dependency vector is empty. The method never finishes or resets the meter.

```rust
use sc_core::{recipe::FormulaCheckedExpression,design::*};
fn add_original_proof(proof:&FormulaCheckedExpression<'_, '_, '_>,sink:&mut dyn DesignResourceSink,
    certificates:DesignResourceLocation,dependencies:DesignResourceLocation)
    ->Result<(),DesignResourceLimitError> {
    proof.charge_retained_resources(sink,certificates,dependencies)
}
```

Each original dependency occurrence counts,including repeated names and both conditional branches.
Calling the producer twice charges two logical occurrences of the held proof;the previous demand
or constructor policy provides no credit. The fixed certificate and dependency records borrow
syntax,declarations and source strings. Those bodies compose through their original resource
owners;this producer adds no text or counter charge and grants no complete source/work allowance.
Its exhaustive owner destructuring and fixed-Copy record guards require future owned fields to
receive explicit accounting.

The focused public-factory check covers literal,unary,builtin and nested conditional trees. It
checks exact and short certificate/dependency allowances,repeated composition,earlier refusal
latching and unchanged owner loans. A checked zero divisor remains unevaluated. This producer
supports the next aggregate semantic accounting stage;complete proof-chain composition and
aggregate bounds are still pending.

## Focused evidence and remaining work

The new native target passes six tests: every node shape, all eight named kinds, actual Unknown
source borrows, direct/computed tolerance roles, closed call/argument inspection, whole statement
locations and an arena containing all 256 permitted nodes. The existing expression target passes seven tests preserving
normative operator/call signatures, dependency order, callee priority and bounded static behavior.
Selected strict lint and the relevant privacy/lifetime documentation tests pass. Broad/platform
verification remains deferred to the existing integration/pre-push owners.

No runtime result, approval, geometry, statement execution or complete recipe acceptance follows
from node inspection alone. The [execution sequence](formula-execution-plan.md) documents the separate
actual scalar worker and remaining integration slices.
