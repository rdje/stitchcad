# Canonical expression byte spelling — symbolic unary and fixed-square operators

- **Type:** `decision`
- **Date:** `2026-10-02`
- **Status:** `active`
- **Owner / source:** director's explicit D103 answer; recorded by G1-SLICE.5a.3d.1.

answers: "which unary-minus and fixed-square bytes identify an expression?" · "can operator tags collide with call names?"

The director chose **(- child)** and **(^2 child)**. This establishes the expression byte contract;
production serialization remains .3d.2, with coupled review .3d.3. It changes no authored syntax.

The existing contract specifies ASCII S-expressions, bare names, typed integer literals, one space
between parts, no comments/trailing spaces/floats, and preservation of operator/ordered-child identity.
It previously left exact unary-minus/fixed-square bytes unstated; the received ruling resolves D103. The book reference produces tuples;
normalized shape fixtures use inspection labels, not a serialization protocol.

Complete expression mapping:

| Expression role | Canonical bytes |
| --- | --- |
| Literal | kind:unsigned-decimal-integer, after existing exact input normalization |
| Name | original valid bare lower-snake identifier |
| Unary minus | (- child) |
| Fixed square | (^2 child) |
| Binary arithmetic/comparison | (operator left right), with the actual machine symbol |
| Ordinary call | (function arg1 arg2 ...), with authored function name and argument order |
| Conditional | (if condition then else), with all three children |

The ten binary symbols are +, -, *, /, ==, !=, <, <=, >, >=. The symbolic unary minus is distinct
from binary subtraction by arity. The ^2 tag is not an ordinary identifier; square stays
one operator plus one child. Neither form folds sign, evaluates arithmetic, sorts arguments, validates
names/types, or drops a branch. Grouping/unit respellings disappear from identity; source spans remain
available separately. Canonical output has no terminal newline or padding and is explicit customer data.

The unselected alternative square spelling was (^ child count:2). Its count:2 is fixed operator payload, not an
additional semantic node. This keeps the source operator symbol but introduces an extra serialized
part. Both spellings are deterministic and distinguish calls from operators.

Do not reuse (neg child) or (square child) unless the language is also changed to reserve those call
names: neg(1 um) and square(1 um) currently parse as ordinary calls. They are distinct syntax from
-1 um and 1 um ^ 2, even before later function validation refuses them. A serializer should preserve
that distinction rather than merge unknown-call syntax into an operator identity.

Canonical expression examples:

```text
-0 deg                  => (- angle:0)
--0 deg                 => (- (- angle:0))
-2.5 cm ^ 2             => (- (^2 length:25000))
(2.5 cm + 25 mm) ^ 2    => (^2 (+ length:25000 length:25000))
if(a, 360 deg, -720 deg) => (if a angle:360000000 (- angle:720000000))
probe(-1 um, 1 um ^ 2)  => (probe (- length:1) (^2 length:1))
```

These are specified wire bytes, not executable recipe claims. Statement bind/assert serialization,
recipe envelopes, hashing/project persistence and numerical evaluation remain separate work.
The received ruling settles both operator spellings; .3d.2 must implement and verify these exact bytes.
