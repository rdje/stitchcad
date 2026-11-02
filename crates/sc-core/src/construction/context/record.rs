//! Original source-record identity independently of an opaque payload's storage layout.
use core::fmt;

/// An owned original source record whose borrowed identity survives zero-sized/empty payloads.
///
/// Distinct payload positions can share an address when their type occupies no bytes. Each record
/// therefore has its own fixed nonzero storage footprint. Context bindings borrow the actual record,
/// not just its payload. This creates no persistent entity id, registry entry, geometry or approval.
/// Before borrowing, a record can move normally; while borrowed it cannot be replaced or moved.
/// Providers keep one original record per actual frame/snapshot source and account its held payload.
///
/// Unsized payload views keep that same record identity:
/// ```
/// use sc_core::construction::DraftingSourceRecord;
/// let record = DraftingSourceRecord::new([11_u8, 12]);
/// let held: &DraftingSourceRecord<[u8]> = &record;
/// assert_eq!(held.data(), &[11, 12]);
/// assert!(core::mem::size_of_val(held) > 0);
/// ```
/// Reconstructing equal data creates a different original record; no implicit clone is provided:
/// ```compile_fail,E0599
/// fn duplicate<T>(record:sc_core::construction::DraftingSourceRecord<T>)
///     ->sc_core::construction::DraftingSourceRecord<T> { record.clone() }
/// ```
pub struct DraftingSourceRecord<T: ?Sized> {
    _identity_storage: u8,
    data: T,
}
impl<T> DraftingSourceRecord<T> {
    /// Retain the exact owned source payload in its explicit original record, without lookup.
    #[must_use]
    pub fn new(data: T) -> Self {
        Self {
            _identity_storage: 0,
            data,
        }
    }
}
impl<T: ?Sized> DraftingSourceRecord<T> {
    /// Borrow the actual original payload without copying or accepting its semantic content.
    #[must_use]
    pub const fn data(&self) -> &T {
        &self.data
    }
    /// Whether both borrows hold this same original live record, independently of payload equality.
    /// Compare the record's nonzero storage address, not slice lengths or trait-object metadata.
    /// This identity is local to the held source borrows and must never be persisted as an entity id.
    #[must_use]
    pub fn is_same_record(&self, other: &Self) -> bool {
        core::ptr::addr_eq(self, other)
    }
}
impl<T: ?Sized> fmt::Debug for DraftingSourceRecord<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingSourceRecord")
            .finish_non_exhaustive()
    }
}
