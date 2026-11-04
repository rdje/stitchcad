//! Original raw component query and complete held import fields;accepted binding stays separate.
use super::{
    garment_source::garment, limits, scope, EmptyHost, Frame, Snapshot, Source, SourceError,
    TestResult,
};
use sc_core::{command::DesignRevision, construction::*, design::*, ontology::*};
use std::{cell::Cell, error::Error, fmt};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn target(copy: u128, piece: u128) -> PocketPieceRef {
    PocketPieceRef {
        copy: id(copy),
        piece: id(piece),
    }
}
pub(super) struct Imported {
    name: String,
    bytes: Vec<u8>,
    revision: DesignRevision,
}
struct Row {
    target: PocketPieceRef,
    body: DraftingSourceRecord<Imported>,
    supported: bool,
}
#[derive(Debug)]
pub(super) enum Cause {
    Base(SourceError),
    Missing {
        target: PocketPieceRef,
    },
    Piece {
        requested: PocketPieceRef,
        actual: PocketPieceRef,
    },
    Unsupported {
        target: PocketPieceRef,
    },
}
impl fmt::Display for Cause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("original component fixture cause")
    }
}
impl Error for Cause {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Base(cause) => Some(cause),
            _ => None,
        }
    }
}
pub(super) struct Provider<'a> {
    base: Source<'a>,
    garment: &'a DesignGarment<EmptyHost>,
    imports: Vec<Row>,
    cache: Vec<u8>,
    ignore: bool,
    queries: Cell<usize>,
    charges: Cell<usize>,
}
impl<'a> Provider<'a> {
    pub(super) fn new(garment: &'a DesignGarment<EmptyHost>, ignore: bool) -> Self {
        Self {
            base: Source::new(garment.inputs()),
            garment,
            imports: vec![
                Row {
                    target: target(10002, 100),
                    body: DraftingSourceRecord::new(Imported {
                        name: "original import é".into(),
                        bytes: vec![11, 12, 13],
                        revision: DesignRevision::from_counter(7),
                    }),
                    supported: true,
                },
                Row {
                    target: target(10001, 100),
                    body: DraftingSourceRecord::new(Imported {
                        name: "unused original import".into(),
                        bytes: vec![21, 22],
                        revision: DesignRevision::from_counter(u128::MAX).successor(),
                    }),
                    supported: false,
                },
            ],
            cache: vec![8, 9],
            ignore,
            queries: Cell::new(0),
            charges: Cell::new(0),
        }
    }
    pub(super) fn query_count(&self) -> usize {
        self.queries.get()
    }
    pub(super) fn charge_count(&self) -> usize {
        self.charges.get()
    }
    fn walk(&self, sink: &mut dyn DesignResourceSink) -> Result<(), DesignResourceLimitError> {
        let Self {
            base,
            garment,
            imports,
            cache,
            ignore,
            queries,
            charges,
        } = self;
        fixed(garment);
        fixed(ignore);
        let _: &Cell<usize> = queries;
        let _: &Cell<usize> = charges;
        base.charge_resources(sink)?;
        sink.charge_entries(imports.len(), at(None, "component.imports", None))?;
        for (index, row) in imports.iter().enumerate() {
            let Row {
                target,
                body,
                supported,
            } = row;
            fixed(target);
            fixed(supported);
            let Imported {
                name,
                bytes,
                revision,
            } = body.data();
            let location = |field| at(Some(target.copy), field, Some(index));
            sink.charge_text(name, location("component.import.name"))?;
            sink.charge_entries(bytes.len(), location("component.import.bytes"))?;
            let _: &[u8] = bytes;
            charge_design_revision_resources(
                revision,
                sink,
                location("component.import.revision"),
            )?;
        }
        sink.charge_entries(cache.len(), at(None, "component.cache", None))?;
        let _: &[u8] = cache;
        Ok(())
    }
}
fn fixed<T: Copy>(_: &T) {}
fn at(
    owner: Option<EntityId>,
    field: &'static str,
    index: Option<usize>,
) -> DesignResourceLocation {
    DesignResourceLocation {
        scope: DesignResourceScope::SourceHost,
        owner,
        field,
        index,
    }
}
impl DraftingContextSource<EmptyHost> for Provider<'_> {
    type Frame = Frame;
    type Snapshot = Snapshot;
    type Geometry = [u8];
    type Error = Cause;
    fn root(&self) -> &DesignInputs<EmptyHost> {
        self.base.root()
    }
    fn snapshot_revision(&self) -> &DesignRevision {
        self.base.snapshot_revision()
    }
    fn frame(&self) -> &DraftingSourceRecord<Frame> {
        self.base.frame()
    }
    fn snapshot(&self) -> &DraftingSourceRecord<Snapshot> {
        self.base.snapshot()
    }
    fn charge_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        self.charges.set(self.charges.get() + 1);
        let result = self.walk(sink);
        if self.ignore {
            Ok(())
        } else {
            result
        }
    }
    fn validate_current(&self) -> Result<(), Cause> {
        self.base.validate_current().map_err(Cause::Base)
    }
    fn geometry(
        &self,
        r: DraftingGeometryReference,
    ) -> DraftingGeometrySourceResult<'_, Frame, Snapshot, [u8], Cause> {
        self.base.geometry(r).map_err(Cause::Base)
    }
}
impl DraftingGarmentProvider<EmptyHost> for Provider<'_> {
    fn garment_source(&self) -> DraftingGarmentSource<'_, EmptyHost, Snapshot> {
        DraftingGarmentSource {
            garment: self.garment,
            snapshot: &self.base.snapshot,
        }
    }
}
impl DraftingComponentProvider<EmptyHost> for Provider<'_> {
    type Imported = Imported;
    fn component_source<'a>(
        &'a self,
        requested: PocketPieceRef,
    ) -> Result<DraftingComponentResponse<'a, EmptyHost, Self>, Cause> {
        self.queries.set(self.queries.get() + 1);
        let row = self
            .imports
            .iter()
            .find(|row| row.target.copy == requested.copy)
            .ok_or(Cause::Missing { target: requested })?;
        if row.target.piece != requested.piece {
            return Err(Cause::Piece {
                requested,
                actual: row.target,
            });
        }
        if !row.supported {
            return Err(Cause::Unsupported { target: row.target });
        }
        let piece = self
            .garment
            .inputs()
            .structure()
            .pieces()
            .piece(row.target.piece)
            .ok_or(Cause::Missing { target: row.target })?;
        Ok(DraftingComponentSource {
            origin: DraftingComponentOrigin::Imported(DraftingImportedComponentSource {
                piece,
                source: &row.body,
            }),
            frame: &self.base.frame,
            snapshot: &self.base.snapshot,
        })
    }
}
#[test]
fn original_import_piece_body_and_frame_snapshot_loans_survive_short_request_storage() -> TestResult
{
    let garment = garment(false)?;
    let provider = Provider::new(&garment, false);
    let context = scope(garment.inputs())?
        .bind_drafting_context(&provider, limits(10000, 100000, 1000)?)
        .map_err(|e| e.to_string())?;
    let admitted = context
        .admit_garment_source(limits(10000, 100000, 1000)?)
        .map_err(|e| e.to_string())?;
    assert_eq!(admitted.demand().counter_bits(), 138);
    let source = {
        let requested = target(10002, 100);
        provider.component_source(requested)?
    };
    let copied = source;
    assert!(core::ptr::eq(copied.origin.piece(), source.origin.piece()));
    let DraftingComponentOrigin::Imported(imported) = source.origin else {
        return Err("original imported origin".into());
    };
    assert!(core::ptr::eq(
        imported.piece,
        garment
            .inputs()
            .structure()
            .pieces()
            .piece(id(100))
            .ok_or("actual Piece")?
    ));
    assert!(imported
        .source
        .is_same_record(&provider.imports.first().ok_or("original row")?.body));
    assert_eq!(imported.source.data().bytes.as_slice(), &[11, 12, 13]);
    assert!(source.frame.is_same_record(context.frame()));
    assert!(source.snapshot.is_same_record(context.snapshot()));
    assert!(!format!("{source:?}").contains("original import"));
    assert_eq!(provider.queries.get(), 1);
    Ok(())
}
#[test]
fn complete_unused_import_counter_and_vector_fields_refuse_before_component_queries() -> TestResult
{
    let garment = garment(false)?;
    for ignore in [false, true] {
        let provider = Provider::new(&garment, ignore);
        let context = scope(garment.inputs())?
            .bind_drafting_context(&provider, limits(10000, 100000, 1000)?)
            .map_err(|e| e.to_string())?;
        let error = context
            .admit_garment_source(limits(10000, 100000, 137)?)
            .err()
            .ok_or("unused import counter refusal")?;
        assert_eq!(
            error.issue(),
            DraftingGarmentSourceIssue::Resources(DesignResourceLimitError::CounterBits {
                maximum: 137,
                consumed: 9,
                requested: 129,
                location: at(Some(id(10001)), "component.import.revision", Some(1))
            })
        );
        assert_eq!(provider.queries.get(), 0);
        let mut meter = DesignResourceMeter::new(limits(4, 100000, 1000)?);
        let expected = DesignResourceLimitError::Count {
            unit: DesignResourceCount::CollectionEntries,
            maximum: 4,
            consumed: 3,
            requested: 2,
            location: at(None, "component.imports", None),
        };
        let result = provider.charge_resources(&mut meter);
        if ignore {
            assert_eq!(result, Ok(()));
        } else {
            assert_eq!(result, Err(expected));
        }
        assert_eq!(meter.finish(), Err(expected));
        assert_eq!(provider.queries.get(), 0);
    }
    Ok(())
}
#[test]
fn missing_reassigned_and_unsupported_source_causes_keep_exact_original_requests() -> TestResult {
    let garment = garment(false)?;
    let provider = Provider::new(&garment, false);
    let requested = target(404, 100);
    let error = provider
        .component_source(requested)
        .err()
        .ok_or("missing component")?;
    assert!(matches!(error,Cause::Missing {target} if target==requested));
    let requested = target(10002, 999);
    let error = provider
        .component_source(requested)
        .err()
        .ok_or("wrong expected Piece")?;
    assert!(
        matches!(error,Cause::Piece {requested:actual,actual:original} if actual==requested && original==target(10002,100))
    );
    let requested = target(10001, 100);
    let error = provider
        .component_source(requested)
        .err()
        .ok_or("unsupported original source")?;
    assert!(matches!(error,Cause::Unsupported {target} if target==requested));
    assert_eq!(provider.queries.get(), 3);
    Ok(())
}
