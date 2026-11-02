//! Actual source-record identity rather than potentially aliased opaque payload addresses.
use sc_core::construction::DraftingSourceRecord;

#[test]
fn distinct_zero_sized_payload_positions_require_distinct_original_record_storage() {
    let raw = [(); 2];
    let [first, second] = &raw;
    assert!(std::ptr::eq(first, second));
    let records = [DraftingSourceRecord::new(()), DraftingSourceRecord::new(())];
    let [first, second] = &records;
    assert!(core::mem::size_of_val(first) > 0);
    assert!(!std::ptr::eq(first, second));
    assert!(!first.is_same_record(second));
    assert!(first.is_same_record(first));
    assert_eq!(first.data(), second.data());
}

#[test]
fn empty_unsized_payload_views_keep_their_actual_distinct_records() {
    let records = [
        DraftingSourceRecord::new([0_u8; 0]),
        DraftingSourceRecord::new([0_u8; 0]),
    ];
    let [first, second] = &records;
    let first: &DraftingSourceRecord<[u8]> = first;
    let second: &DraftingSourceRecord<[u8]> = second;
    assert!(first.data().is_empty() && second.data().is_empty());
    assert!(!std::ptr::eq(first, second));
    assert!(!first.is_same_record(second));
    assert!(first.is_same_record(first));
}

#[test]
fn equal_source_definitions_do_not_merge_records_or_disclose_private_payloads() {
    let first = DraftingSourceRecord::new(String::from("private frame definition"));
    let second = DraftingSourceRecord::new(String::from("private frame definition"));
    assert_eq!(first.data(), second.data());
    assert!(!std::ptr::eq(&first, &second));
    assert!(!first.is_same_record(&second));
    assert!(!format!("{first:?}").contains("private"));
}
