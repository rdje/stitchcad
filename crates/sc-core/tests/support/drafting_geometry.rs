//! Opaque provider records exercise source binding only, not a geometric/value implementation.
use super::{limits, root, scope, EmptyHost, SourceError, TestResult};
use core::cell::Cell;
use sc_core::{
    command::DesignRevision,
    construction::{
        DraftingContextSource, DraftingGeometryBindingIssue as Issue, DraftingGeometryError,
        DraftingGeometryReference as Reference, DraftingGeometrySource,
        DraftingGeometrySourceResult, DraftingSourceRecord,
    },
    design::{
        charge_design_revision_resources, DesignInputs, DesignResourceLimitError,
        DesignResourceLocation, DesignResourceScope, DesignResourceSink,
    },
    ontology::{EdgeRef, EntityId, LocalTag, PointRef},
};
use std::error::Error;

fn point() -> Reference {
    Reference::Point(PointRef::new(EntityId::from_bits(100), LocalTag::new(2)))
}
fn edge() -> Reference {
    Reference::Edge(EdgeRef::new(EntityId::from_bits(101), LocalTag::new(7)))
}
// None of these source owners implements Clone or Debug.
struct Frame {
    label: String,
    origin: [i64; 2],
    axes: [[i64; 2]; 2],
}
struct GeometryRecord {
    reference: Reference,
    provenance: String,
}
struct Snapshot {
    owner: EntityId,
    revision: DesignRevision,
    label: String,
    records: [GeometryRecord; 2],
}
#[derive(Clone, Copy)]
enum Fault {
    None,
    Reference,
    Frame,
    Snapshot,
    All,
    Provider,
}
struct Provider<'a> {
    root: &'a DesignInputs<EmptyHost>,
    frame: DraftingSourceRecord<Frame>,
    snapshot: DraftingSourceRecord<Snapshot>,
    other_frame: DraftingSourceRecord<Frame>,
    other_snapshot: DraftingSourceRecord<Snapshot>,
    fault: Fault,
    queries: Cell<usize>,
}
impl<'a> Provider<'a> {
    fn new(root: &'a DesignInputs<EmptyHost>, fault: Fault) -> Self {
        let frame = || {
            DraftingSourceRecord::new(Frame {
                label: "private authored Cartesian source".into(),
                origin: [0, 0],
                axes: [[1, 0], [0, 1]],
            })
        };
        let snapshot = || {
            DraftingSourceRecord::new(Snapshot {
                owner: root.id(),
                revision: DesignRevision::from_counter(7),
                label: "private registry provenance".into(),
                records: [
                    GeometryRecord {
                        reference: point(),
                        provenance: "private point source".into(),
                    },
                    GeometryRecord {
                        reference: edge(),
                        provenance: "private edge source".into(),
                    },
                ],
            })
        };
        Self {
            root,
            frame: frame(),
            snapshot: snapshot(),
            other_frame: frame(),
            other_snapshot: snapshot(),
            fault,
            queries: Cell::new(0),
        }
    }
    fn location(&self, field: &'static str, index: usize) -> DesignResourceLocation {
        DesignResourceLocation {
            scope: DesignResourceScope::Inputs,
            owner: Some(self.root.id()),
            field,
            index: Some(index),
        }
    }
}
impl DraftingContextSource<EmptyHost> for Provider<'_> {
    type Frame = Frame;
    type Snapshot = Snapshot;
    type Geometry = GeometryRecord;
    type Error = SourceError;
    fn root(&self) -> &DesignInputs<EmptyHost> {
        self.root
    }
    fn snapshot_revision(&self) -> &DesignRevision {
        &self.snapshot.data().revision
    }
    fn frame(&self) -> &DraftingSourceRecord<Frame> {
        &self.frame
    }
    fn snapshot(&self) -> &DraftingSourceRecord<Snapshot> {
        &self.snapshot
    }
    fn charge_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        for (index, frame) in [&self.frame, &self.other_frame].into_iter().enumerate() {
            sink.charge_text(
                &frame.data().label,
                self.location("context.frame.label", index),
            )?;
        }
        for (index, snapshot) in [&self.snapshot, &self.other_snapshot]
            .into_iter()
            .enumerate()
        {
            sink.charge_text(
                &snapshot.data().label,
                self.location("context.snapshot.label", index),
            )?;
            charge_design_revision_resources(
                &snapshot.data().revision,
                sink,
                self.location("context.snapshot.revision", index),
            )?;
            for (record, entry) in snapshot.data().records.iter().enumerate() {
                sink.charge_text(
                    &entry.provenance,
                    self.location("context.output.provenance", index * 2 + record),
                )?;
            }
        }
        Ok(())
    }
    fn validate_current(&self) -> Result<(), Self::Error> {
        // Complete actual selected fixture registry; no coordinate or length values are supplied.
        let snapshot = self.snapshot.data();
        if snapshot.owner != self.root.id() || &snapshot.revision != self.root.revision() {
            return Err(SourceError::Rejected);
        }
        Ok(())
    }
    fn geometry(
        &self,
        requested: Reference,
    ) -> DraftingGeometrySourceResult<'_, Frame, Snapshot, GeometryRecord, SourceError> {
        self.queries.set(self.queries.get() + 1);
        if matches!(self.fault, Fault::Provider) {
            return Err(SourceError::Rejected);
        }
        let geometry = self
            .snapshot
            .data()
            .records
            .iter()
            .find(|entry| entry.reference == requested)
            .ok_or(SourceError::Missing)?;
        // Explicit adapter faults must retain their actual returned records for the core refusal.
        Ok(DraftingGeometrySource {
            reference: if matches!(self.fault, Fault::Reference | Fault::All) {
                edge()
            } else {
                geometry.reference
            },
            frame: if matches!(self.fault, Fault::Frame | Fault::All) {
                &self.other_frame
            } else {
                &self.frame
            },
            snapshot: if matches!(self.fault, Fault::Snapshot | Fault::All) {
                &self.other_snapshot
            } else {
                &self.snapshot
            },
            geometry,
        })
    }
}

#[test]
fn actual_point_and_edge_bindings_keep_complete_original_records_without_values_or_copies(
) -> TestResult {
    let root = root()?;
    let provider = Provider::new(&root, Fault::None);
    let context = scope(&root)?
        .bind_drafting_context(&provider, limits(1000, 10000, 1000)?)
        .map_err(|e| e.to_string())?;
    for actual in &provider.snapshot.data().records {
        let binding = context
            .bind_existing_geometry(actual.reference)
            .map_err(|e| e.to_string())?;
        let copied = binding;
        assert_eq!(copied.reference(), actual.reference);
        assert!(std::ptr::eq(copied.geometry(), actual));
        assert!(std::ptr::eq(copied.context().scope().root(), &root));
        assert!(copied.source().frame.is_same_record(&provider.frame));
        assert!(copied.source().snapshot.is_same_record(&provider.snapshot));
        assert!(!format!("{copied:?}").contains("private"));
    }
    assert_eq!(provider.frame.data().origin, [0, 0]);
    assert_eq!(provider.frame.data().axes, [[1, 0], [0, 1]]);
    assert_eq!(provider.queries.get(), 2);
    Ok(())
}

#[test]
fn first_mismatch_keeps_request_context_and_every_original_returned_source() -> TestResult {
    let root = root()?;
    for (fault, issue) in [
        (Fault::Reference, Issue::Reference),
        (Fault::Frame, Issue::Frame),
        (Fault::Snapshot, Issue::Snapshot),
        (Fault::All, Issue::Reference),
    ] {
        let provider = Provider::new(&root, fault);
        let context = scope(&root)?
            .bind_drafting_context(&provider, limits(1000, 10000, 1000)?)
            .map_err(|e| e.to_string())?;
        assert_eq!(
            provider.frame.data().label,
            provider.other_frame.data().label
        );
        assert_eq!(
            provider.snapshot.data().label,
            provider.other_snapshot.data().label
        );
        let Err(DraftingGeometryError::Mismatch(error)) = context.bind_existing_geometry(point())
        else {
            return Err("expected actual returned-source mismatch".into());
        };
        assert_eq!(error.issue(), issue);
        assert_eq!(error.requested(), point());
        assert!(std::ptr::eq(error.context().scope().root(), &root));
        let returned = error.returned();
        let first = provider
            .snapshot
            .data()
            .records
            .first()
            .ok_or("point source")?;
        assert!(std::ptr::eq(returned.geometry, first));
        if matches!(fault, Fault::Frame | Fault::All) {
            assert!(returned.frame.is_same_record(&provider.other_frame));
        }
        if matches!(fault, Fault::Snapshot | Fault::All) {
            assert!(returned.snapshot.is_same_record(&provider.other_snapshot));
        }
        if matches!(fault, Fault::Reference | Fault::All) {
            assert_eq!(returned.reference, edge());
        }
        assert!(!format!("{error:?}").contains("private"));
        assert_eq!(provider.queries.get(), 1);
    }
    Ok(())
}

#[test]
fn missing_and_original_provider_causes_keep_exact_request_without_manufactured_results(
) -> TestResult {
    let root = root()?;
    let missing = Reference::Point(PointRef::new(EntityId::from_bits(999), LocalTag::new(0)));
    for (fault, requested, expected) in [
        (Fault::None, missing, SourceError::Missing),
        (Fault::Provider, point(), SourceError::Rejected),
    ] {
        let provider = Provider::new(&root, fault);
        let context = scope(&root)?
            .bind_drafting_context(&provider, limits(1000, 10000, 1000)?)
            .map_err(|e| e.to_string())?;
        let result = context.bind_existing_geometry(requested);
        let Err(error) = result else {
            return Err("no provider result should be manufactured".into());
        };
        assert!(error
            .source()
            .is_some_and(|cause| cause.is::<SourceError>()));
        match error {
            DraftingGeometryError::Source {
                requested: actual,
                cause,
            } => {
                assert_eq!(actual, requested);
                assert_eq!(*cause, expected);
            }
            other => return Err(format!("expected original provider cause, got {other:?}").into()),
        }
        assert_eq!(provider.queries.get(), 1);
    }
    Ok(())
}
