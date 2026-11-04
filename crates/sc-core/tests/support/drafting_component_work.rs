//! Complete fresh component work before source queries or current copy/Piece scans.
use super::{component_port::Provider, garment_source::garment, limits, scope, TestResult};
use sc_core::{construction::*, design::*, ontology::*};
fn target(copy: u128, piece: u128) -> PocketPieceRef {
    PocketPieceRef {
        copy: EntityId::from_bits(copy),
        piece: EntityId::from_bits(piece),
    }
}
#[test]
fn fresh_complete_work_keeps_original_current_members_without_accepting_sources() -> TestResult {
    for copy_id in [10001, 10002] {
        let garment = garment(false)?;
        let provider = Provider::new(&garment, false);
        let content = limits(10000, 100000, 1000)?;
        let context = scope(garment.inputs())?
            .bind_drafting_context(&provider, content)
            .map_err(|e| e.to_string())?;
        let parent = context
            .admit_garment_source(content)
            .map_err(|e| e.to_string())?;
        let demand = parent.demand();
        let entries = demand.collection_entries();
        let required = (entries + 8) * (entries + 8);
        let policy = DraftingComponentBindingLimits::new(
            limits(entries, demand.text_bytes(), demand.counter_bits())?,
            required,
        );
        let before = provider.charge_count();
        let requested = target(copy_id, 100);
        let work = parent
            .prepare_component_binding(requested, policy)
            .map_err(|e| e.to_string())?;
        assert_eq!(provider.charge_count(), before + 1);
        assert_eq!(provider.query_count(), 0);
        assert!(core::ptr::eq(work.parent(), &parent));
        assert_eq!(work.target(), requested);
        assert_eq!(work.limits(), policy);
        assert_eq!(work.demand(), demand);
        assert_eq!(work.shape().held_collection_entries(), entries);
        assert_eq!(work.shape().matching_inspection_bound(), required);
        let structure = garment.inputs().structure();
        let copy = structure
            .cut_plan()
            .copy(requested.copy)
            .ok_or("original copy")?;
        assert!(core::ptr::eq(work.copy(), copy));
        assert_eq!(work.copy().orientation(), copy.orientation());
        assert!(core::ptr::eq(
            work.piece(),
            structure
                .pieces()
                .piece(requested.piece)
                .ok_or("original Piece")?
        ));
        let mut meter =
            DesignResourceMeter::new(limits(entries + 2, demand.text_bytes() + 3, 139)?);
        let at = DesignResourceLocation {
            scope: DesignResourceScope::CommandState,
            owner: None,
            field: "prior",
            index: None,
        };
        meter.charge_entries(2, at)?;
        meter.charge_text("abc", at)?;
        meter.charge_counter_bits(1, at)?;
        work.charge_resources(&mut meter)?;
        let combined = meter.finish()?;
        assert_eq!(combined.collection_entries(), entries + 2);
        assert_eq!(combined.text_bytes(), demand.text_bytes() + 3);
        assert_eq!(combined.counter_bits(), 139);
        assert_eq!(
            provider.query_count(),
            0,
            "unsupported source copy10001 is only a current member here"
        );
    }
    Ok(())
}
#[test]
fn complete_content_and_work_refuse_before_missing_or_reassigned_members() -> TestResult {
    let garment = garment(false)?;
    for ignore in [false, true] {
        let provider = Provider::new(&garment, ignore);
        let content = limits(10000, 100000, 1000)?;
        let context = scope(garment.inputs())?
            .bind_drafting_context(&provider, content)
            .map_err(|e| e.to_string())?;
        let parent = context
            .admit_garment_source(content)
            .map_err(|e| e.to_string())?;
        let requested = target(404, 100);
        let policy = DraftingComponentBindingLimits::new(limits(10000, 100000, 137)?, 0);
        let error = parent
            .prepare_component_binding(requested, policy)
            .err()
            .ok_or("unused import refusal")?;
        assert!(core::ptr::eq(error.parent(), &parent));
        assert_eq!(error.target(), requested);
        assert_eq!(error.limits(), policy);
        assert!(
            matches!(error.refusal(), DraftingComponentWorkRefusal::Resources(
            DesignResourceLimitError::CounterBits {maximum:137,consumed:9,requested:129,location}
        ) if location.field=="component.import.revision" && location.owner==Some(EntityId::from_bits(10001)) && location.index==Some(1))
        );
        assert!(std::error::Error::source(&error).is_some());
        let entries = parent.demand().collection_entries();
        let required = (entries + 8) * (entries + 8);
        let error = parent
            .prepare_component_binding(
                requested,
                DraftingComponentBindingLimits::new(content, required - 1),
            )
            .err()
            .ok_or("work before member")?;
        assert!(
            matches!(error.refusal(),DraftingComponentWorkRefusal::MatchingInspections {allowed,required:actual} if *allowed==required-1 && *actual==required)
        );
        let policy = DraftingComponentBindingLimits::new(content, required);
        let error = parent
            .prepare_component_binding(requested, policy)
            .err()
            .ok_or("missing copy")?;
        assert!(matches!(
            error.refusal(),
            DraftingComponentWorkRefusal::MissingCopy
        ));
        let requested = target(10002, 999);
        let error = parent
            .prepare_component_binding(requested, policy)
            .err()
            .ok_or("reassigned copy")?;
        assert_eq!(error.target(), requested);
        let DraftingComponentWorkRefusal::CopyPiece { actual } = error.refusal() else {
            return Err("original copy refusal".into());
        };
        assert!(core::ptr::eq(
            *actual,
            garment
                .inputs()
                .structure()
                .cut_plan()
                .copy(requested.copy)
                .ok_or("copy")?
        ));
        assert_eq!(provider.query_count(), 0);
    }
    Ok(())
}
#[test]
fn admitted_parent_composition_retains_an_earlier_latched_cause() -> TestResult {
    let garment = garment(false)?;
    let provider = Provider::new(&garment, true);
    let content = limits(10000, 100000, 1000)?;
    let context = scope(garment.inputs())?
        .bind_drafting_context(&provider, content)
        .map_err(|e| e.to_string())?;
    let parent = context
        .admit_garment_source(content)
        .map_err(|e| e.to_string())?;
    let mut meter = DesignResourceMeter::new(limits(1, 100000, 1000)?);
    let at = DesignResourceLocation {
        scope: DesignResourceScope::CommandState,
        owner: None,
        field: "prior",
        index: None,
    };
    let original = meter.charge_entries(2, at).err().ok_or("earlier refusal")?;
    assert_eq!(parent.charge_resources(&mut meter), Err(original));
    assert_eq!(meter.finish(), Err(original));
    assert_eq!(provider.query_count(), 0);
    Ok(())
}
