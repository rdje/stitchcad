# Viewport

The `sc-viewport` crate maps between a pattern's coordinates and the screen, and answers what is under the pointer. It
draws nothing, so every renderer shares the same answers. **What works today:** the mapping, picking, snapping, and the
synthetic corpus [ADR-0002's canvas spike](../annexes/implementation-status.md) measures against. **What does not exist
yet:** any renderer in the product. The spike's renderers are experiments, and a real viewport comes with the dev shell.

## The mapping cannot drift

A view holds its position and zoom as exact fractions, so zooming out again returns to exactly the same view, however
many times. A renderer that accumulates floating-point moves loses a little on every step, and nobody sees it until a
pick lands on the wrong edge.

```rust
use sc_viewport::{Point, View, ZOOMS};

let start = View::new((1, 1)).expect("a zoom").pan(123, 456).expect("a pan");
let mut view = start;
for _ in 0..1000 {
    for zoom in ZOOMS {
        view = view.zoom_about(zoom, 400, 300).expect("in range");
    }
    view = view.zoom_about((1, 1), 400, 300).expect("in range");
}
assert_eq!(view, start);

// At 1:1, 4 pixels are 1 millimetre, which is 1000 micrometres.
let plain = View::new((1, 1)).expect("a zoom");
assert_eq!(plain.pixel_of(Point { x: 1000, y: 2000 }).expect("in range"), (4.0, 8.0));
assert_eq!(plain.point_of_pixel(4, 8).expect("in range"), Point { x: 1000, y: 2000 });
```

## Picking and snapping

Both compare distances as exact integers, so a near tie cannot go the wrong way, and a real tie goes to the earlier
piece and edge. `pick` answers which edge lies within reach of a point, and `snap` answers which vertex is nearest.

```rust
use sc_viewport::{pick, snap, Piece, Point};

let square = Piece {
    outline: vec![
        Point { x: 0, y: 0 }, Point { x: 1000, y: 0 }, Point { x: 1000, y: 1000 }, Point { x: 0, y: 1000 },
    ],
    annotations: vec![],
};
let pieces = [square];
let edge = pick(&pieces, Point { x: 500, y: 3 }, 10).expect("on the top edge");
assert_eq!((edge.piece, edge.edge), (0, 0));
assert_eq!(pick(&pieces, Point { x: 500, y: 500 }, 10), None);
let vertex = snap(&pieces, Point { x: 990, y: 980 }).expect("there are vertices");
assert_eq!((vertex.piece, vertex.edge), (0, 2));
```

## How it is checked

Run `cargo test -p sc-viewport`. Every pick and snap probe in the corpus must name its declared entity, and each snap is
checked again by brute force written apart from the function. Screen and world agree within half a pixel across the whole
zoom ladder, and 1,000 round trips return to exactly the same view.

## Names used in this chapter

| Name | Meaning |
| --- | --- |
| `sc-viewport` | The crate |
| `pick` | The edge within reach of a point |
| `snap` | The vertex nearest a point |
