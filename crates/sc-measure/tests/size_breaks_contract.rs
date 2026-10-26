//! Explicit requirement inventories, named coverage gaps and singleton grading refusal.
#![allow(clippy::expect_used, clippy::indexing_slicing)]
#[path = "support/size_break_fixture.rs"]
mod support;
use sc_core::value::{LengthState, LengthValueError};
use sc_measure::*;
use sc_units::Count;
use std::error::Error;
use support::{declaration, id, Fixture};
fn cell(dimension: u128, from: u128, to: u128) -> SizeBreakCell {
    SizeBreakCell {
        axis: id(10),
        dimension: id(dimension),
        from: id(from),
        to: id(to),
    }
}
fn complete(f: &Fixture, context: &SizeBreakContext<'_>) -> SizeBreaksDefinition {
    let dimensions = [1, 0]
        .into_iter()
        .map(|index| SizeBreakDimension {
            axis: id(10),
            dimension_table: id(500),
            dimension: MeasurementBinding::from(&f.measurements[index]),
        })
        .collect();
    let breaks = [
        (0, 2, 3, 100),
        (1, 3, 4, 101),
        (1, 2, 3, 101),
        (0, 3, 4, 101),
    ]
    .into_iter()
    .enumerate()
    .map(|(index, (dimension, from, to, delta))| {
        let mut d = f.definition();
        d.id = id(700 + index as u128);
        d.dimension = MeasurementBinding::from(&f.measurements[dimension]);
        d.from = id(from);
        d.to = id(to);
        d.delta = id(delta);
        SizeBreak::new(d, context).expect("actual declaration")
    })
    .collect();
    SizeBreaksDefinition {
        reference: f.membership.reference(),
        dimensions,
        breaks,
    }
}
#[test]
fn complete_explicit_coverage_keeps_requirement_and_break_orders_and_original_borrows() {
    let f = Fixture::new(MeasurementKind::Garment);
    f.with_context(|context| {
        let d = complete(&f, context);
        let values = SizeBreaks::new(d.clone(), context).expect("explicit coverage");
        assert_eq!(values.definition(), &d);
        assert_eq!(values.missing(context).expect("actual gaps"), vec![]);
        assert_eq!(values.require_grade_coverage(context), Ok(()));
        let first = values
            .declaration(cell(200, 2, 3), context)
            .expect("actual declaration");
        assert!(std::ptr::eq(first, &values.definition().breaks[0]));
        assert!(std::ptr::eq(
            first.delta(context).expect("actual canonical delta"),
            &f.lengths[0]
        ));
        assert_eq!(
            values.definition().dimensions[0].dimension.measurement,
            id(201)
        );
        assert_eq!(
            values.definition().breaks[0]
                .definition()
                .dimension
                .measurement,
            id(200)
        );
    });
}
#[test]
fn partial_drafts_report_all_exact_gaps_without_inferred_dimensions_or_zero_values() {
    let f = Fixture::new(MeasurementKind::Garment);
    f.with_context(|context| {
        let mut d = complete(&f, context);
        d.breaks.truncate(1);
        let values = SizeBreaks::new(d, context).expect("partial authored draft");
        let gaps = vec![cell(201, 2, 3), cell(201, 3, 4), cell(200, 3, 4)];
        assert_eq!(values.missing(context).expect("ordered actual gaps"), gaps);
        assert_eq!(
            values.require_grade_coverage(context),
            Err(SizeBreaksError::MissingCell(gaps[0]))
        );
        assert_eq!(
            values.declaration(cell(200, 3, 4), context).err(),
            Some(SizeBreaksError::MissingCell(cell(200, 3, 4)))
        );
        assert_eq!(
            values.declaration(cell(999, 2, 3), context).err(),
            Some(SizeBreaksError::UndeclaredDimension {
                axis: id(10),
                dimension: id(999)
            })
        );
        let refusal = SizeBreaksError::Context(SizeBreakError::NotAdjacent {
            set: id(1),
            from: id(2),
            to: id(4),
        });
        assert_eq!(
            values.declaration(cell(200, 2, 4), context).err(),
            Some(refusal.clone())
        );
        assert!(refusal.source().is_some());
        let empty = SizeBreaks::new(
            SizeBreaksDefinition {
                reference: f.membership.reference(),
                dimensions: vec![],
                breaks: vec![],
            },
            context,
        )
        .expect("empty draft");
        assert!(empty
            .missing(context)
            .expect("no invented requirements")
            .is_empty());
        assert_eq!(
            empty.require_grade_coverage(context),
            Err(SizeBreaksError::MissingDimensions(id(1)))
        );
    });
}
#[test]
fn duplicate_or_undeclared_requirements_ids_and_cells_refuse() {
    let f = Fixture::new(MeasurementKind::Garment);
    f.with_context(|context| {
        let d = complete(&f, context);
        let mut wrong = d.clone();
        wrong.dimensions.push(wrong.dimensions[0].clone());
        assert_eq!(
            SizeBreaks::new(wrong, context).err(),
            Some(SizeBreaksError::DuplicateDimension {
                axis: id(10),
                dimension: id(201)
            })
        );
        let mut wrong = d.clone();
        wrong.breaks.push(wrong.breaks[0].clone());
        assert_eq!(
            SizeBreaks::new(wrong, context).err(),
            Some(SizeBreaksError::DuplicateIdentity(id(700)))
        );
        let mut wrong = d.clone();
        let mut duplicate = wrong.breaks[0].definition().clone();
        duplicate.id = id(799);
        wrong
            .breaks
            .push(SizeBreak::new(duplicate, context).expect("individually valid same cell"));
        assert_eq!(
            SizeBreaks::new(wrong, context).err(),
            Some(SizeBreaksError::DuplicateCell(cell(200, 2, 3)))
        );
        let mut wrong = d.clone();
        wrong
            .dimensions
            .retain(|value| value.dimension.measurement == id(200));
        assert_eq!(
            SizeBreaks::new(wrong, context).err(),
            Some(SizeBreaksError::UndeclaredDimension {
                axis: id(10),
                dimension: id(201)
            })
        );
        let mut wrong = d.clone();
        wrong.dimensions[0].axis = id(999);
        assert_eq!(
            SizeBreaks::new(wrong, context).err(),
            Some(SizeBreaksError::Context(SizeBreakError::MissingAxis(id(
                999
            ))))
        );
        let mut wrong = d.clone();
        wrong.reference.revision = Count::new(4);
        assert_eq!(
            SizeBreaks::new(wrong.clone(), context).err(),
            Some(SizeBreaksError::MembershipMismatch {
                expected: wrong.reference,
                actual: d.reference
            })
        );
    });
}
#[test]
fn singleton_body_set_has_no_adjacent_gaps_and_still_refuses_grading() {
    let mut f = Fixture::new(MeasurementKind::Body);
    let mut membership = f.membership.definition().clone();
    membership.members.truncate(1);
    membership.base = id(2);
    f.membership = SizeMembership::new(membership).expect("custom singleton");
    let mut axes = f.axes.definition().clone();
    axes.points.retain(|point| point.member == id(2));
    let refs = f.lengths.iter().collect::<Vec<_>>();
    let sources = SizeAxisSourceContext::new(&refs, &[]).expect("sources");
    f.axes =
        SizeAxes::new(axes, &f.membership, &sources).expect("singleton still has authored axis");
    f.with_context(|context| {
        for dimensions in [
            vec![],
            vec![SizeBreakDimension {
                axis: id(10),
                dimension_table: id(500),
                dimension: MeasurementBinding::from(&f.measurements[0]),
            }],
        ] {
            let values = SizeBreaks::new(
                SizeBreaksDefinition {
                    reference: f.membership.reference(),
                    dimensions,
                    breaks: vec![],
                },
                context,
            )
            .expect("singleton inventory");
            assert_eq!(values.missing(context).expect("no adjacent pairs"), vec![]);
            assert_eq!(
                values.require_grade_coverage(context),
                Err(SizeBreaksError::Singleton {
                    set: id(1),
                    member: id(2)
                })
            );
        }
    });
}
#[test]
fn coverage_preserves_unknown_sources_and_revalidates_current_member_order() {
    let mut f = Fixture::new(MeasurementKind::Garment);
    let values = f.with_context(|context| {
        SizeBreaks::new(complete(&f, context), context).expect("original collection")
    });
    f.lengths[0] = declaration(
        100,
        LengthState::Unknown {
            observation: id(8888),
        },
    );
    f.with_context(|context| {
        assert_eq!(values.require_grade_coverage(context), Ok(()));
        let value = values
            .declaration(cell(200, 2, 3), context)
            .expect("actual current declaration");
        assert!(std::ptr::eq(
            value.delta(context).expect("unknown source"),
            &f.lengths[0]
        ));
        assert_eq!(
            value.authored_value(context),
            Err(SizeBreakError::Value(
                LengthValueError::RequiresObservation {
                    declaration: id(100),
                    observation: id(8888)
                }
            ))
        );
    });
    let mut membership = f.membership.definition().clone();
    membership.members.reverse();
    f.membership = SizeMembership::new(membership).expect("explicit reorder");
    f.with_context(|context| {
        let refusal = SizeBreaksError::Break {
            declaration: id(700),
            issue: Box::new(SizeBreakError::NotAdjacent {
                set: id(1),
                from: id(2),
                to: id(3),
            }),
        };
        assert_eq!(values.missing(context), Err(refusal.clone()));
        assert!(refusal.source().is_some());
    });
}
