//! Complete actual garment candidate ownership, current failures, resources and reserve boundary.
use super::{command_request, id, limits, with_join};
use core::num::{NonZeroU64, NonZeroUsize};
use sc_core::{
    command::{
        CommandAuthoritySnapshot, CommandAuthoritySource, CommandBus, CommandIdentityError,
        CommandSeamSplitGarmentError as Error,
    },
    design::{
        charge_design_garment_resources, DesignGarment, DesignGarmentError,
        DesignGarmentPosition as Position, DesignResourceDemand, DesignResourceLimitError,
        DesignResourceLimits, DesignResourceMeter, DesignSewing, DesignSewingPosition,
    },
    ontology::{
        ConstructionInventoryError, GatherError, Param, Rational, SeamSideId, SeamSpanHistory,
        SeamSpanHistoryEntry as Entry, SewingSpanLookup,
    },
};
use sc_measure::DesignSources;
use std::cell::Cell;

fn original() -> DesignGarment<DesignSources> {
    let mut definition = crate::garment_ownership::definition();
    let mut sewing = definition.sewing.definition().clone();
    assert_eq!(
        sewing.graph.ease_sources().count(),
        0,
        "fixture has known independent roots"
    );
    let entries = sewing
        .graph
        .spans()
        .iter()
        .map(|span| Entry::Root(span.id()))
        .collect();
    sewing.graph = sewing
        .graph
        .with_span_history(
            SeamSpanHistory::new(entries, limits(100, 8, 16).history())
                .expect("actual fixture births"),
        )
        .expect("actual original graph leaves");
    definition.sewing = DesignSewing::new(sewing).expect("actual sewing ownership");
    let original = DesignGarment::new(definition).expect("complete real garment namespaces");
    original
        .validate_current()
        .expect("all actual original families current");
    original
}
fn demand(original: &DesignGarment<DesignSources>) -> DesignResourceDemand {
    let mut meter = DesignResourceMeter::new(DesignResourceLimits::new(
        NonZeroUsize::MAX,
        NonZeroUsize::MAX,
        NonZeroU64::MAX,
    ));
    charge_design_garment_resources(original, &mut meter)
        .expect("actual complete garment accounting");
    meter.finish().expect("no partial demand")
}
fn budget(demand: DesignResourceDemand, extra: usize) -> DesignResourceLimits {
    DesignResourceLimits::new(
        NonZeroUsize::new(demand.collection_entries() + extra)
            .expect("explicit clone/output allowance"),
        NonZeroUsize::new(demand.text_bytes().max(1)).expect("exact actual text allowance"),
        NonZeroU64::new(demand.counter_bits().max(1)).expect("exact actual counter allowance"),
    )
}
fn midpoint(graph: &sc_core::ontology::SewingGraph, index: usize, side: SeamSideId) -> Param {
    let span = graph.spans().get(index).expect("actual selected child");
    let range = match side {
        SeamSideId::A => span.definition().a.range,
        SeamSideId::B => span.definition().b.range,
    };
    Param::new(
        range
            .from()
            .as_rational()
            .checked_add(range.to().as_rational())
            .expect("small fixture ranges")
            .checked_div(Rational::from_integer(2))
            .expect("exact midpoint"),
    )
    .expect("interior")
}
fn proposed() -> [sc_core::ontology::EntityId; 2] {
    [id(u128::MAX - 1), id(u128::MAX)]
}

#[test]
fn seam_garment_candidate_preserves_complete_owners_and_exposes_original_gather_repair_after_split()
{
    let original = original();
    let saved = original.clone();
    let before = demand(&original);
    assert_eq!(
        (
            before.collection_entries(),
            before.text_bytes(),
            before.counter_bits()
        ),
        (107, 888, 0),
        "published complete garment fixture footprint stays exact"
    );
    let graph = original.sewing().graph();
    let request = command_request(
        original.sewing(),
        0,
        midpoint(graph, 0, SeamSideId::A),
        midpoint(graph, 0, SeamSideId::B),
    );
    let candidate = with_join(original.sewing(), &request, |source, joined| {
        let graph = joined
            .admit_graph_candidate(source, proposed(), limits(100, 2, 3))
            .expect("real graph admission");
        assert_eq!(graph.reservation().collection_entries(), 6);
        let admission = graph
            .admit_garment_candidate(&original, budget(before, 6))
            .expect("complete original owner and output admission");
        assert!(core::ptr::eq(admission.original(), &original));
        assert_eq!(admission.graph_admission().children(), proposed());
        assert_eq!(admission.limits(), budget(before, 6));
        assert_eq!(
            admission.reservation().collection_entries(),
            before.collection_entries() + 6
        );
        assert_eq!(admission.reservation().text_bytes(), before.text_bytes());
        assert_eq!(
            admission.reservation().counter_bits(),
            before.counter_bits()
        );
        admission
            .build()
            .expect("complete canonical ownership with held references preserved")
    });
    let after = demand(&candidate);
    assert_eq!(after.collection_entries(), before.collection_entries() + 3);
    assert_eq!(after.text_bytes(), before.text_bytes());
    assert_eq!(after.counter_bits(), before.counter_bits());
    assert_eq!(candidate.inputs(), original.inputs());
    assert_eq!(candidate.pockets(), original.pockets());
    assert_eq!(candidate.notions(), original.notions());
    assert_eq!(candidate.closures(), original.closures());
    assert_eq!(candidate.layers(), original.layers());
    assert_eq!(candidate.hems(), original.hems());
    assert_eq!(candidate.constructions(), original.constructions());
    assert_eq!(
        candidate.sewing().landmarks(),
        original.sewing().landmarks()
    );
    let gather = candidate
        .constructions()
        .gathers()
        .first()
        .expect("actual held Gather");
    assert_eq!(gather.definition().span, request.span());
    assert!(
        matches!(gather.inspect_span_reference(candidate.sewing().graph()).expect("actual retirement evidence"),SewingSpanLookup::RetiredSplit {children,..} if *children==proposed())
    );
    let issue = candidate
        .validate_current()
        .expect_err("owned inspection cannot erase original Gather repair");
    assert!(matches!(issue, DesignGarmentError::Constructions(error)
        if matches!(error.as_ref(),ConstructionInventoryError::Gather { issue,.. }
            if **issue==GatherError::MissingSpan(request.span()))));
    assert_eq!(
        gather.intake_source(candidate.sewing().graph()),
        Err(GatherError::MissingSpan(request.span()))
    );
    assert_eq!(original, saved);
    println!("whole candidate resources: original={before:?};candidate={after:?};reservation entries=original+6;rc=0");
}

#[test]
fn seam_garment_candidate_checks_owner_complete_resource_then_exact_cross_family_child_identity() {
    use std::error::Error as _;
    let original = original();
    let saved = original.clone();
    let other = original.clone();
    let before = demand(&original);
    let request = command_request(
        original.sewing(),
        0,
        midpoint(original.sewing().graph(), 0, SeamSideId::A),
        midpoint(original.sewing().graph(), 0, SeamSideId::B),
    );
    with_join(original.sewing(), &request, |source, joined| {
        let graph = || {
            joined
                .admit_graph_candidate(source, [id(200000), proposed()[1]], limits(100, 2, 3))
                .expect("local graph IDs cannot certify other canonical families")
        };
        assert!(matches!(
            graph().admit_garment_candidate(&other, budget(before, 0)),
            Err(Error::Owner { .. })
        ));
        let issue = graph()
            .admit_garment_candidate(&original, budget(before, 5))
            .expect_err("whole reservation before global identity census");
        assert!(
            matches!(issue,Error::Resources(DesignResourceLimitError::Count {consumed,requested:3,maximum,location,..})
            if consumed==before.collection_entries()+3 && maximum==before.collection_entries()+5 && location.field=="candidate.span_split.children_and_history")
        );
        assert!(issue
            .source()
            .expect("original resource cause")
            .downcast_ref::<DesignResourceLimitError>()
            .is_some());
        assert_eq!(
            graph()
                .admit_garment_candidate(&original, budget(before, 6))
                .expect_err("actual full Pocket identity is not fresh"),
            Error::ChildOwned {
                child: 0,
                identity: id(200000),
                owner: Position::Pocket { index: 0 }
            }
        );
        joined
            .admit_graph_candidate(source, proposed(), limits(100, 2, 3))
            .expect("valid retry after refusal")
            .admit_garment_candidate(&original, budget(before, 6))
            .expect("exact complete boundary")
            .build()
            .expect("valid owned inspection candidate");
    });
    assert_eq!(original, saved);
}

#[test]
fn seam_garment_candidate_retains_current_original_gather_failure_before_further_clones() {
    use std::error::Error as _;
    let original = original();
    let before = demand(&original);
    let first = command_request(
        original.sewing(),
        0,
        midpoint(original.sewing().graph(), 0, SeamSideId::A),
        midpoint(original.sewing().graph(), 0, SeamSideId::B),
    );
    let held = with_join(original.sewing(), &first, |source, joined| {
        joined
            .admit_graph_candidate(source, proposed(), limits(100, 2, 3))
            .expect("first actual split")
            .admit_garment_candidate(&original, budget(before, 6))
            .expect("original actual garment current")
            .build()
            .expect("inspectable held repair candidate")
    });
    let saved = held.clone();
    let before = demand(&held);
    let request = command_request(
        held.sewing(),
        0,
        midpoint(held.sewing().graph(), 0, SeamSideId::A),
        midpoint(held.sewing().graph(), 0, SeamSideId::B),
    );
    let expected = held
        .validate_current()
        .expect_err("actual existing held Gather repair");
    with_join(held.sewing(), &request, |source, joined| {
        let issue = joined
            .admit_graph_candidate(source, [id(300000), id(300001)], limits(100, 3, 5))
            .expect("only local sewing source is current")
            .admit_garment_candidate(&held, budget(before, 100))
            .expect_err("complete original garment refuses before any clone");
        assert_eq!(issue, Error::Current(Box::new(expected)));
        assert!(issue
            .source()
            .expect("original whole current cause")
            .downcast_ref::<DesignGarmentError<sc_measure::DesignSourcesError>>()
            .is_some());
    });
    assert_eq!(held, saved);
}

struct UnusedAuthority<'a>(&'a Cell<usize>);
impl CommandAuthoritySource for UnusedAuthority<'_> {
    fn snapshot(&self) -> CommandAuthoritySnapshot<'_> {
        self.0.set(self.0.get() + 1);
        CommandAuthoritySnapshot {
            authenticated_actor: None,
            observed_at: std::time::SystemTime::UNIX_EPOCH,
            grants: &[],
        }
    }
}
#[test]
fn seam_garment_candidate_retired_parent_and_new_children_remain_owned_by_actual_bus_reserve_guard()
{
    let original = original();
    let before = demand(&original);
    let reads = Cell::new(0);
    CommandBus::with_identities(
        original.clone(),
        UnusedAuthority(&reads),
        proposed().to_vec(),
    )
    .expect("new IDs disjoint from every original owner");
    let request = command_request(
        original.sewing(),
        0,
        midpoint(original.sewing().graph(), 0, SeamSideId::A),
        midpoint(original.sewing().graph(), 0, SeamSideId::B),
    );
    let candidate = with_join(original.sewing(), &request, |source, joined| {
        joined
            .admit_graph_candidate(source, proposed(), limits(100, 2, 3))
            .expect("actual graph preparation")
            .admit_garment_candidate(&original, budget(before, 6))
            .expect("complete original source")
            .build()
            .expect("owned candidate with history")
    });
    for (identity, entry, child) in [
        (request.span(), 0, None),
        (proposed()[0], 1, Some(0)),
        (proposed()[1], 1, Some(1)),
    ] {
        let issue =
            CommandBus::with_identities(candidate.clone(), UnusedAuthority(&reads), vec![identity])
                .expect_err("historical owned ID cannot enter a new private reserve");
        assert_eq!(
            issue,
            CommandIdentityError::AlreadyOwned {
                identity,
                index: 0,
                owner: Position::Sewing(DesignSewingPosition::SpanHistory { entry, child })
            }
        );
    }
    assert_eq!(
        reads.get(),
        0,
        "reserve constructor grants no authority or effect"
    );
}
