//! A view holds exactly, so it cannot drift, and picks and snaps match an answer worked out by brute force.
#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation
)]
use sc_viewport::{
    corpus, pick, snap, Point, View, ViewError, PICK_PROBES, PIECES, SNAP_PROBES,
    VERTICES_PER_PIECE, ZOOMS,
};

#[test]
fn the_corpus_is_what_the_decision_record_declared_and_depends_only_on_its_seed() {
    let first = corpus(7);
    assert_eq!(first.pieces.len(), PIECES);
    assert!(first
        .pieces
        .iter()
        .all(|piece| piece.outline.len() == VERTICES_PER_PIECE));
    assert_eq!(first.picks.len(), PICK_PROBES);
    assert_eq!(first.snaps.len(), SNAP_PROBES);
    assert_eq!(first, corpus(7));
    assert_ne!(first, corpus(8));
}

#[test]
fn every_pick_probe_names_its_declared_edge() {
    let set = corpus(7);
    for probe in &set.picks {
        let found = pick(&set.pieces, probe.at, 100).expect("the probe is on an edge");
        assert_eq!(
            (found.piece, found.edge),
            (probe.piece, probe.entity),
            "{probe:?}"
        );
    }
    assert_eq!(
        pick(
            &set.pieces,
            Point {
                x: -5_000_000,
                y: -5_000_000
            },
            100
        ),
        None,
        "nothing is there"
    );
}

#[test]
fn every_snap_probe_lands_on_its_declared_vertex_and_on_the_true_nearest() {
    let set = corpus(7);
    for probe in &set.snaps {
        let found = snap(&set.pieces, probe.at).expect("there are vertices");
        assert_eq!(
            (found.piece, found.edge),
            (probe.piece, probe.entity),
            "{probe:?}"
        );
        // Brute force, written apart from the function, in a different form.
        let mut best = (i128::MAX, 0, 0);
        for (piece_at, piece) in set.pieces.iter().enumerate() {
            for (vertex, at) in piece.outline.iter().enumerate() {
                let d = (i128::from(at.x) - i128::from(probe.at.x)).pow(2)
                    + (i128::from(at.y) - i128::from(probe.at.y)).pow(2);
                if d < best.0 {
                    best = (d, piece_at, vertex);
                }
            }
        }
        assert_eq!((best.1, best.2), (found.piece, found.edge));
    }
}

#[test]
fn pixel_and_world_agree_within_half_a_pixel_across_the_whole_zoom_ladder() {
    let set = corpus(7);
    let mut worst = 0.0_f64;
    for zoom in ZOOMS {
        let view = View::new(zoom).expect("a zoom").pan(13, 29).expect("a pan");
        for probe in set.picks.iter().take(50) {
            let (x, y) = view.pixel_of(probe.at).expect("in range");
            let (px, py) = (x.round() as i64, y.round() as i64);
            let back = view.point_of_pixel(px, py).expect("in range");
            let (ax, ay) = view.pixel_of(back).expect("in range");
            worst = worst
                .max((ax - px as f64).abs())
                .max((ay - py as f64).abs());
        }
    }
    assert!(worst <= 0.5, "worst {worst}");
}

#[test]
fn a_thousand_round_trips_of_the_ladder_return_to_exactly_the_same_view() {
    let start = View::new((1, 1))
        .expect("a zoom")
        .pan(123, 456)
        .expect("a pan");
    let mut view = start;
    for _ in 0..1000 {
        for zoom in ZOOMS.iter().chain(ZOOMS.iter().rev()) {
            view = view.zoom_about(*zoom, 400, 300).expect("in range");
        }
        view = view.zoom_about((1, 1), 400, 300).expect("in range");
        assert_eq!(view, start, "drift must be exactly zero after every trip");
    }
    assert_eq!(view.transform(), start.transform());
}

#[test]
fn zooming_keeps_the_point_under_the_cursor_exactly() {
    let view = View::new((1, 1))
        .expect("a zoom")
        .pan(17, 91)
        .expect("a pan");
    let before = view.world_of_pixel(250, 180).expect("in range");
    let closer = view.zoom_about((40, 1), 250, 180).expect("in range");
    assert_eq!(closer.world_of_pixel(250, 180).expect("in range"), before);
}

#[test]
fn a_bad_zoom_is_refused() {
    assert_eq!(View::new((0, 1)), Err(ViewError::BadZoom));
    assert_eq!(View::new((1, -1)), Err(ViewError::BadZoom));
}
