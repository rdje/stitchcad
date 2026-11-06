//! The offset pathology corpus the G2 exit names: `conformance/offsets/cases.json`, run by one command, each case judged by what it
//! says a pass means, and judged by an oracle shown here to catch broken results.
#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::cast_precision_loss
)]
mod common;

use common::{clean, p, valid, valid_widths};
use sc_geometry::{Allowance, Corner, Curve, OffsetError, Outline, Point, MITER_LIMIT};
use sc_store::{parse, Value};
use std::collections::{BTreeMap, BTreeSet};

const CASES: &str = include_str!("../../../conformance/offsets/cases.json");

fn text<'a>(fields: &'a BTreeMap<String, Value>, name: &str) -> &'a str {
    fields[name].as_text().expect(name)
}

fn int(fields: &BTreeMap<String, Value>, name: &str) -> Option<i64> {
    fields.get(name).and_then(Value::as_int)
}

fn pair(value: &Value) -> (i64, i64) {
    let items = value.as_list().expect("a point");
    (items[0].as_int().expect("x"), items[1].as_int().expect("y"))
}

fn build(fields: &BTreeMap<String, Value>) -> (Outline, Vec<Allowance>, i64) {
    let curves = fields["curves"]
        .as_list()
        .expect("curves")
        .iter()
        .map(|curve| {
            let curve = curve.as_object().expect("a curve");
            let points: Vec<(i64, i64)> = curve["points"]
                .as_list()
                .expect("points")
                .iter()
                .map(pair)
                .collect();
            Curve::from_kind(text(curve, "kind"), &points).expect("a curve")
        })
        .collect();
    let allowances = fields["allowances"]
        .as_list()
        .expect("allowances")
        .iter()
        .map(|allowance| {
            let allowance = allowance.as_object().expect("an allowance");
            let corner = match text(allowance, "corner") {
                "miter" => Corner::Miter,
                "slant" => Corner::Slant,
                "trim" => Corner::Trim,
                "step" => Corner::Step,
                "envelope" => Corner::Envelope,
                other => panic!("no corner is named {other}"),
            };
            Allowance {
                width: int(allowance, "width").expect("a width"),
                corner,
            }
        })
        .collect();
    (
        Outline::new(curves).expect("a closed boundary"),
        allowances,
        int(fields, "tolerance").expect("a tolerance"),
    )
}

fn refusal_name(error: OffsetError) -> &'static str {
    match error {
        OffsetError::WrongCount { .. } => "wrong_count",
        OffsetError::NegativeWidth { .. } => "negative_width",
        OffsetError::NotCounterclockwise => "not_counterclockwise",
        OffsetError::NetCrossing { .. } => "net_crossing",
        OffsetError::BoundExceeded { .. } => "bound_exceeded",
        OffsetError::Tangled => "tangled",
    }
}

fn run_case(fields: &BTreeMap<String, Value>) -> Result<(), String> {
    let (net, allowances, tolerance) = build(fields);
    let result = net.offset(&allowances, tolerance);
    if let Some(wanted) = fields.get("expect_refusal").and_then(Value::as_text) {
        return match result {
            Err(error) if refusal_name(error) == wanted => Ok(()),
            other => Err(format!("wanted the refusal {wanted}, and got {other:?}")),
        };
    }
    let offset = match result {
        Ok(offset) => offset,
        Err(error) if fields.get("may_refuse").is_some() => {
            return if error.to_string().is_empty() {
                Err("an empty refusal".to_owned())
            } else {
                Ok(())
            };
        }
        Err(error) => return Err(format!("refused: {error}")),
    };
    let points = offset.points();
    if fields.get("clearance").is_some() {
        // The case fills part of the net, so only cleanliness applies.
        clean(points)?;
    } else {
        let widths: Vec<i64> = allowances.iter().map(|allowance| allowance.width).collect();
        valid_widths(&net, points, &widths, tolerance)?;
    }
    let swallowed: Vec<i64> = fields
        .get("expect_swallowed")
        .and_then(Value::as_list)
        .map_or_else(Vec::new, |list| {
            list.iter().filter_map(Value::as_int).collect()
        });
    if !swallowed.iter().all(|edge| {
        offset
            .swallowed_edges()
            .contains(&usize::try_from(*edge).unwrap_or(usize::MAX))
    }) {
        return Err(format!(
            "wanted {swallowed:?} swallowed, and got {:?}",
            offset.swallowed_edges()
        ));
    }
    if fields.get("reach_limit").is_some() {
        let widest = allowances
            .iter()
            .map(|allowance| allowance.width)
            .max()
            .unwrap_or(0);
        let lowest = points.iter().map(|point| point.x()).min().unwrap_or(0);
        if lowest < -(MITER_LIMIT * widest + 3_000) {
            return Err(format!("a join reaches to {lowest}, past the limit"));
        }
    }
    if let Some(wanted) = fields.get("expect_points").and_then(Value::as_list) {
        for point in wanted {
            let (x, y) = pair(point);
            if !points.contains(&p(x, y)) {
                return Err(format!("wanted ({x}, {y}) in {points:?}"));
            }
        }
    }
    let extreme = |pick: fn(&Point) -> i64, name: &str, least: bool| -> Result<(), String> {
        let found = if least {
            points.iter().map(pick).min()
        } else {
            points.iter().map(pick).max()
        };
        match (int(fields, name), found) {
            (Some(wanted), Some(found)) if wanted != found => {
                Err(format!("{name} is {found}, wanting {wanted}"))
            }
            _ => Ok(()),
        }
    };
    if let (Some(limit), Some(found)) = (
        int(fields, "max_y_at_most"),
        points.iter().map(|point| point.y()).max(),
    ) {
        if found > limit {
            return Err(format!("it reaches up to {found}, past {limit}"));
        }
    }
    extreme(|point| point.x(), "min_x", true)?;
    extreme(|point| point.y(), "min_y", true)?;
    extreme(|point| point.y(), "max_y", false)?;
    if let Some(bound) = int(fields, "length_bound_um") {
        let offset_wanted = int(fields, "offset_length_um").expect("an offset length");
        let net_wanted = int(fields, "net_length_um").expect("a net length");
        let offset_error = (offset.length() - offset_wanted).abs();
        let net_error = (net.length(tolerance) - net_wanted).abs();
        if offset_error > bound || net_error > bound {
            return Err(format!("offset length is {offset_error} off, net length {net_error} off, and the bound is {bound}"));
        }
    }
    Ok(())
}

#[test]
fn every_corpus_case_passes_as_it_says_and_the_required_families_are_all_there() {
    let value = parse(CASES).expect("canonical text");
    let cases = value.as_object().expect("an object")["cases"]
        .as_list()
        .expect("cases");
    let mut tags = BTreeSet::new();
    let mut failures = Vec::new();
    for case in cases {
        let fields = case.as_object().expect("a case");
        // A case must say what it is for, so a reader can judge it.
        assert!(
            !text(fields, "exercises").is_empty() && !text(fields, "pass_means").is_empty(),
            "{}",
            text(fields, "name")
        );
        for tag in fields["tags"].as_list().expect("tags") {
            tags.insert(tag.as_text().expect("a tag").to_owned());
        }
        if let Err(reason) = run_case(fields) {
            failures.push(format!("{}: {reason}", text(fields, "name")));
        }
    }
    assert!(
        failures.is_empty(),
        "{} failed:\n{}",
        failures.len(),
        failures.join("\n")
    );
    for family in [
        "acute",
        "loops",
        "seam-length",
        "near-tangent",
        "cusp",
        "variable",
        "fixture",
        "refusal",
    ] {
        assert!(tags.contains(family), "the corpus has no {family} case");
    }
    println!("{} corpus cases pass", cases.len());
}

#[test]
fn the_oracle_catches_broken_results_so_a_pass_means_something() {
    let net = Outline::new(vec![
        Curve::from_kind("line", &[(0, 0), (100_000, 0)]).expect("a line"),
        Curve::from_kind("line", &[(100_000, 0), (100_000, 100_000)]).expect("a line"),
        Curve::from_kind("line", &[(100_000, 100_000), (0, 100_000)]).expect("a line"),
        Curve::from_kind("line", &[(0, 100_000), (0, 0)]).expect("a line"),
    ])
    .expect("closed");
    let allowance = vec![
        Allowance {
            width: 10_000,
            corner: Corner::Miter
        };
        4
    ];
    let good = net.offset(&allowance, 10).expect("offsets");
    let points = good.points().to_vec();
    valid(&net, &points, 10_000, 10).expect("the real result passes");

    // A corner pulled inward leaves the edge short of its width.
    let mut short = points.clone();
    short[1] = p(short[1].x() - 2_000, short[1].y() + 2_000);
    assert!(
        valid(&net, &short, 10_000, 10)
            .expect_err("short")
            .contains("only"),
        "{:?}",
        valid(&net, &short, 10_000, 10)
    );
    // Two points swapped make the boundary cross itself.
    let mut crossed = points.clone();
    crossed.swap(1, 2);
    assert!(valid(&net, &crossed, 10_000, 10)
        .expect_err("crossed")
        .contains("cross"));
    // The same points the other way round are inside out.
    let mut reversed = points.clone();
    reversed.reverse();
    assert!(valid(&net, &reversed, 10_000, 10)
        .expect_err("reversed")
        .contains("counterclockwise"));
    // An allowance that misses a net corner leaves it outside.
    let mut missing = points.clone();
    missing[2] = p(90_000, 90_000);
    assert!(valid(&net, &missing, 10_000, 10)
        .expect_err("missing")
        .contains("outside"));
}
