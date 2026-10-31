//! Requirement arithmetic and authentic candidates only;these are not geometry evidence tests.
#![allow(clippy::expect_used)]
use sc_core::{construction::*, ontology::*};
use sc_units::{Length, Tolerance, ToleranceClass, MAX_LENGTH_UM};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn parameter(n: i64, d: i64) -> Param {
    Param::new(Rational::new(n, d).expect("exact rational")).expect("unit interval")
}
fn length(n: i64) -> Length {
    Length::from_micrometres(n).expect("domain length")
}
#[path = "support/offset_sources.rs"]
mod sources;
fn manifest(input: &OffsetConstruction) -> OffsetCandidateManifest {
    OffsetCandidateManifest::new(
        input.operation(),
        *input.source(),
        vec![
            OffsetCandidateFragment::new(
                LocalTag::new(9),
                OffsetInterval::new(parameter(1, 4), parameter(1, 3)).expect("first"),
            ),
            OffsetCandidateFragment::new(
                LocalTag::new(2),
                OffsetInterval::new(parameter(1, 3), parameter(1, 2)).expect("second"),
            ),
        ],
    )
}
fn evidence() -> OffsetMappingEvidenceRequest {
    OffsetMappingEvidenceRequest::NormalOffset {
        source_arclength: id(71),
        output_arclength: id(72),
        normal_correspondence: id(73),
    }
}
fn allocation(values: [i64; 4]) -> OffsetMappingErrorAllocation {
    let [a, b, c, d] = values;
    OffsetMappingErrorAllocation::new(length(a), length(b), length(c), length(d))
}
fn tolerance(class: ToleranceClass, amount: i64) -> Tolerance {
    Tolerance::new(
        class,
        length(amount),
        "explicit test requirement,not achieved error",
    )
    .expect("derivation")
}
#[test]
fn actual_requirement_fields_and_all_ambiguous_candidates_survive_without_a_mapped_position() {
    let f = sources::fixture();
    let allowance = f.allowance();
    let input = f.input();
    let bound = input.bind_allowance(&allowance).expect("input");
    let raw = manifest(&input);
    let admitted = raw
        .admit(&bound, OffsetCandidateLimits::new(2))
        .expect("manifest");
    let query = admitted
        .query_source(EdgeAnchor {
            edge: input.source().range.edge(),
            param: parameter(1, 3),
        })
        .expect("shared endpoint");
    assert_eq!(query.kind(), OffsetCandidateMatchKind::Ambiguous);
    let tolerance = tolerance(ToleranceClass::GeometricApproximation, 10);
    let requirement =
        OffsetMappingRequirement::new(evidence(), id(70), &tolerance, allocation([4, 1, 4, 1]));
    assert_eq!(requirement.evidence(), &evidence());
    assert_eq!(
        requirement.evidence().kind(),
        OffsetCorrespondenceKind::NormalOffset
    );
    assert_eq!(requirement.tolerance_source(), id(70));
    assert!(std::ptr::eq(requirement.tolerance(), &tolerance));
    let parts = [
        OffsetMappingErrorPart::CurveApproximation,
        OffsetMappingErrorPart::SourceParameter,
        OffsetMappingErrorPart::Correspondence,
        OffsetMappingErrorPart::OutputParameter,
    ];
    assert_eq!(
        parts.map(|p| requirement.allocation().allowance(p).as_micrometres()),
        [4, 1, 4, 1]
    );
    let obligations = query
        .candidates()
        .map(|candidate| {
            candidate
                .require_mapping(&requirement)
                .expect("normal requirement")
        })
        .collect::<Vec<_>>();
    assert_eq!(obligations.len(), 2);
    for (index, obligation) in obligations.iter().copied().enumerate() {
        assert!(std::ptr::eq(obligation.requirement(), &requirement));
        assert_eq!(obligation.allocated_um(), 10);
        assert!(std::ptr::eq(obligation.candidate().query(), &query));
        assert_eq!(obligation.candidate().index(), index);
        assert!(std::ptr::eq(
            obligation.candidate().fragment(),
            raw.fragments().get(index).expect("raw candidate")
        ));
    }
    assert_eq!(
        obligations
            .iter()
            .map(|v| v.candidate().logical_output().tag().get())
            .collect::<Vec<_>>(),
        vec![9, 2]
    );
    assert_eq!(query.kind(), OffsetCandidateMatchKind::Ambiguous);
    let gap = admitted
        .query_source(EdgeAnchor {
            edge: input.source().range.edge(),
            param: parameter(2, 3),
        })
        .expect("in selected domain");
    assert_eq!(gap.kind(), OffsetCandidateMatchKind::Gap);
    assert_eq!(gap.candidates().count(), 0);
}
#[test]
fn actual_class_domain_budget_and_affine_law_refusals_keep_the_whole_requirement_and_match() {
    use OffsetMappingErrorPart as P;
    use OffsetMappingRequirementRefusal as R;
    let f = sources::fixture();
    let allowance = f.allowance();
    let input = f.input();
    let bound = input.bind_allowance(&allowance).expect("input");
    let raw = manifest(&input);
    let admitted = raw
        .admit(&bound, OffsetCandidateLimits::new(2))
        .expect("manifest");
    let query = admitted
        .query_source(EdgeAnchor {
            edge: input.source().range.edge(),
            param: parameter(1, 4),
        })
        .expect("source");
    let candidate = query.candidates().next().expect("one candidate");
    let rows = [
        (
            ToleranceClass::PhysicalAcceptance,
            -1,
            [-1, -1, -1, -1],
            R::ToleranceClass,
        ),
        (
            ToleranceClass::GeometricChordal,
            10,
            [0, 0, 0, 0],
            R::ToleranceClass,
        ),
        (
            ToleranceClass::GeometricApproximation,
            -1,
            [-1, -1, -1, -1],
            R::NegativeTolerance,
        ),
        (
            ToleranceClass::GeometricApproximation,
            10,
            [-1, -1, -1, -1],
            R::NegativeAllocation {
                part: P::CurveApproximation,
            },
        ),
        (
            ToleranceClass::GeometricApproximation,
            10,
            [0, -1, -1, -1],
            R::NegativeAllocation {
                part: P::SourceParameter,
            },
        ),
        (
            ToleranceClass::GeometricApproximation,
            10,
            [0, 0, -1, -1],
            R::NegativeAllocation {
                part: P::Correspondence,
            },
        ),
        (
            ToleranceClass::GeometricApproximation,
            10,
            [0, 0, 0, -1],
            R::NegativeAllocation {
                part: P::OutputParameter,
            },
        ),
        (
            ToleranceClass::GeometricApproximation,
            10,
            [4, 1, 4, 2],
            R::Budget {
                required_um: 11,
                allowed_um: 10,
            },
        ),
        (
            ToleranceClass::GeometricApproximation,
            MAX_LENGTH_UM,
            [MAX_LENGTH_UM; 4],
            R::Budget {
                required_um: i128::from(MAX_LENGTH_UM) * 4,
                allowed_um: MAX_LENGTH_UM,
            },
        ),
        (
            ToleranceClass::GeometricApproximation,
            10,
            [4, 1, 4, 1],
            R::CorrespondenceKind,
        ),
    ];
    for (class, limit, parts, cause) in rows {
        let tolerance = tolerance(class, limit);
        let requirement = OffsetMappingRequirement::new(
            OffsetMappingEvidenceRequest::DeclaredAffinePrimitive,
            id(70),
            &tolerance,
            allocation(parts),
        );
        assert_eq!(
            requirement.evidence().kind(),
            OffsetCorrespondenceKind::DeclaredAffinePrimitive
        );
        let error = candidate
            .require_mapping(&requirement)
            .expect_err("explicit refusal");
        assert!(std::ptr::eq(error.requirement(), &requirement));
        assert!(std::ptr::eq(error.candidate().query(), &query));
        assert!(std::ptr::eq(
            error.candidate().fragment(),
            candidate.fragment()
        ));
        assert_eq!(error.refusal(), cause);
        assert!(format!("{error}").contains("mapping requirement"));
        assert!(format!("{error:?}").contains("OffsetMappingRequirementError"));
    }
}
#[test]
fn zero_and_inclusive_exact_allocations_have_no_implicit_tolerance_or_achieved_error_claim() {
    let f = sources::fixture();
    let allowance = f.allowance();
    let input = f.input();
    let bound = input.bind_allowance(&allowance).expect("input");
    let raw = manifest(&input);
    let admitted = raw
        .admit(&bound, OffsetCandidateLimits::new(2))
        .expect("manifest");
    let query = admitted
        .query_source(EdgeAnchor {
            edge: input.source().range.edge(),
            param: parameter(1, 4),
        })
        .expect("source");
    let candidate = query.candidates().next().expect("candidate");
    for (limit, parts) in [
        (0, [0, 0, 0, 0]),
        (10, [4, 1, 4, 1]),
        (10, [0, 0, 0, 0]),
        (MAX_LENGTH_UM, [0, 0, MAX_LENGTH_UM, 0]),
    ] {
        let tolerance = tolerance(ToleranceClass::GeometricApproximation, limit);
        let requirement =
            OffsetMappingRequirement::new(evidence(), id(70), &tolerance, allocation(parts));
        let result = candidate
            .require_mapping(&requirement)
            .expect("valid requirement arithmetic");
        assert_eq!(
            result.allocated_um(),
            parts.into_iter().map(i128::from).sum::<i128>()
        );
        assert_eq!(result.requirement().tolerance().limit, length(limit));
        assert_eq!(
            result.requirement().tolerance().derivation(),
            "explicit test requirement,not achieved error"
        );
    }
}
