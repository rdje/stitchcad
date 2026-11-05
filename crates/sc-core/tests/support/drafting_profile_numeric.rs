//! Genuine numeric Profile metadata/member/kind binding under exact original selected parents.
use super::{id, limits, scope, EmptyHost, Provider, Snapshot, SourceError, TestResult};
use sc_core::{construction::*, design::*, ontology::*, value::*};
pub(super) struct NumericMapping {
    pub(super) parameter: ProfileParameterRef,
    pub(super) target: CanonicalInputTarget,
    pub(super) text: String,
}
impl DraftingProfileNumericProvider<EmptyHost> for Provider<'_> {
    type NumericMetadata = NumericMapping;
    fn profile_numeric_source<'s>(
        &'s self,
        mut profile: DraftingProfileSource<'s, EmptyHost, Snapshot, [u8]>,
        parameter: ProfileParameterRef,
    ) -> Result<DraftingProfileNumericResponse<'s, EmptyHost, Self>, SourceError> {
        self.numeric_calls.set(self.numeric_calls.get() + 1);
        let mode = if self.numeric_late && self.numeric_calls.get() != 2 {
            0
        } else {
            self.numeric_mode
        };
        if mode == 1 {
            return Err(SourceError::Missing);
        }
        let metadata = self
            .numeric_metadata
            .iter()
            .find(|m| {
                if mode == 7 || mode == 2 {
                    m.data().parameter != parameter
                } else {
                    m.data().parameter == parameter
                }
            })
            .ok_or(SourceError::Missing)?;
        match mode {
            2 => {
                profile.root = self.other;
                profile.selected = &self.wrong_pin;
            }
            3 => profile.selected = &self.wrong_pin,
            4 => profile.inputs = self.base.garment.inputs().inputs(),
            5 => profile.profile = self.numeric_body.as_ref().ok_or(SourceError::Missing)?,
            6 => profile.snapshot = self.base.alternate.snapshot(),
            _ => {}
        }
        Ok(DraftingProfileNumericSource {
            profile,
            parameter: &metadata.data().parameter,
            target: &metadata.data().target,
            metadata,
        })
    }
}
type SourceView<'a> = dyn DraftingProfileNumericProvider<
        EmptyHost,
        Frame = super::Frame,
        Snapshot = Snapshot,
        Geometry = [u8],
        Error = SourceError,
        Profile = [u8],
        NumericMetadata = NumericMapping,
    > + 'a;
pub(super) fn prepare(source: &mut Provider<'_>, case: u8) -> TestResult {
    source.numeric_mode = if case == 12 || case == 13 { 1 } else { case };
    source.wrong_pin = source.selected.clone(); // Equal labels do not establish the same source pin.
    source.numeric_body = Some(DraftingSourceRecord::new(*source.body.data()));
    source.inputs = CanonicalInputs::with_logicals(
        vec![LengthDeclaration::new(LengthDeclarationDefinition {
            id: id(10),
            source: id(1010),
            state: LengthState::Derived { formula: id(500) },
        })?],
        [
            (20, ScalarInputKind::Length),
            (30, ScalarInputKind::Boolean),
        ]
        .into_iter()
        .map(|(n, kind)| {
            Ok(ScalarInputDeclaration::new(
                ScalarInputDeclarationDefinition {
                    id: id(n),
                    source: id(1000 + n),
                    kind,
                    state: ScalarInputState::Unknown {
                        observation: id(1100 + n),
                    },
                },
            )?)
        })
        .collect::<TestResult<Vec<_>>>()?,
        source.inputs.logicals().clone(),
    )?;
    let scalar = |n, kind| CanonicalInputTarget::Scalar {
        declaration: id(n),
        kind,
    };
    source.numeric_metadata = [
        (704, CanonicalInputTarget::Length(id(10))),
        (705, scalar(20, ScalarInputKind::Length)),
        (706, scalar(20, ScalarInputKind::Boolean)), // Annotation disagrees with actual member.
        (707, CanonicalInputTarget::Length(id(20))), // A general scalar must not substitute.
        (708, scalar(10, ScalarInputKind::Length)),  // A dedicated length must not substitute.
        (709, scalar(30, ScalarInputKind::Boolean)),
        (710, CanonicalInputTarget::Length(id(404))),
    ]
    .into_iter()
    .map(|(n, target)| {
        DraftingSourceRecord::new(NumericMapping {
            parameter: ProfileParameterRef::new(id(n)),
            target,
            text: format!("private original numeric metadata {n}"),
        })
    })
    .collect();
    Ok(())
}
#[test]
fn numeric_profile_parameter_keeps_exact_parent_metadata_representation_state_and_first_causes(
) -> TestResult {
    // Dedicated/general members,all original header axes,parameter identity,representation,
    // annotation versus actual kind,fresh complete content before missing,and a genuinely absent ID.
    for case in 0..16u8 {
        let garment = super::super::root_numeric_suppliers::fixture(true, false, 0, false)?;
        let other = super::super::root_numeric_suppliers::fixture(true, false, 0, false)?;
        let mut source = Provider::new(&garment, other.inputs(), 0)?;
        prepare(&mut source, case)?;
        let view: &SourceView<'_> = &source;
        let content = limits(100000, 100000, 100000)?;
        let context = scope(garment.inputs())?
            .bind_drafting_context(view, content)
            .map_err(|e| e.to_string())?;
        let requested = source.selected.clone();
        let parent = context
            .bind_profile_source(&requested, content)
            .map_err(|e| e.to_string())?;
        let mut meter = DesignResourceMeter::new(content);
        parent.charge_resources(&mut meter)?;
        let demand = meter.finish()?;
        let parameter = ProfileParameterRef::new(id(match case {
            8 => 707,
            9 => 708,
            10 => 706,
            11 | 14 => 709,
            15 => 710,
            _ => 704,
        }));
        let kind = if case == 14 {
            ScalarInputKind::Boolean
        } else {
            ScalarInputKind::Length
        };
        if case == 12 {
            source.base.extra_charge.set(100001); // Newly held cache must be walked again.
        }
        let allowance = if case == 13 {
            limits(100000, demand.text_bytes() - 1, 100000)?
        } else {
            content
        };
        let walks = source.base.walks.get();
        let header_calls = source.calls.get();
        let result = parent.bind_numeric_parameter(parameter, kind, allowance);
        assert_eq!(source.base.walks.get(), walks + 1);
        assert_eq!(source.calls.get(), header_calls);
        assert_eq!(source.parameter_calls.get(), 0);
        assert_eq!(source.rule_calls.get(), 0);
        assert_eq!(source.base.base.trace.get(), 2); // Only the fresh resource walk;no currentness/value call.
        assert_eq!(
            source.numeric_calls.get(),
            usize::from(case != 12 && case != 13)
        );
        if case == 0 || case == 14 {
            let accepted = result.map_err(|e| e.to_string())?;
            assert!(core::ptr::eq(accepted.parent(), &parent));
            assert_eq!(accepted.parameter(), parameter);
            assert_eq!(accepted.kind(), kind);
            assert_eq!(accepted.limits(), content);
            assert_eq!(accepted.demand(), demand);
            let index = if case == 0 { 0 } else { 5 };
            let original = source
                .numeric_metadata
                .get(index)
                .ok_or("original numeric metadata")?;
            let raw = accepted.source();
            assert!(raw.metadata.is_same_record(original));
            assert!(core::ptr::eq(raw.parameter, &original.data().parameter));
            assert!(core::ptr::eq(raw.target, &original.data().target));
            assert!(core::ptr::eq(accepted.binding().inputs(), &source.inputs));
            assert_eq!(accepted.binding().target(), *raw.target);
            match accepted.binding().member() {
                CanonicalNumericInputMember::Length { index, declaration } => {
                    assert_eq!(index, 0);
                    assert!(core::ptr::eq(
                        declaration,
                        source.inputs.lengths().first().ok_or("original length")?
                    ));
                    assert!(
                        matches!(declaration.state(), LengthState::Derived { formula } if *formula==id(500))
                    );
                    assert!(matches!(
                        garment
                            .inputs()
                            .inputs()
                            .lengths()
                            .first()
                            .ok_or("original Root length")?
                            .state(),
                        LengthState::Unknown { .. }
                    ));
                }
                CanonicalNumericInputMember::Scalar { index, declaration } => {
                    assert_eq!(index, 1);
                    assert!(core::ptr::eq(
                        declaration,
                        source
                            .inputs
                            .scalars()
                            .get(1)
                            .ok_or("original Boolean scalar")?
                    ));
                    assert!(matches!(
                        declaration.state(),
                        ScalarInputState::Unknown { .. }
                    ));
                }
            }
            let mut meter = DesignResourceMeter::new(content);
            let queries = source.numeric_calls.get();
            accepted.charge_resources(&mut meter)?;
            assert_eq!(meter.finish()?, demand);
            assert_eq!(source.numeric_calls.get(), queries);
            if case == 0 {
                let scalar = parent
                    .bind_numeric_parameter(ProfileParameterRef::new(id(705)), kind, content)
                    .map_err(|e| e.to_string())?;
                assert!(
                    matches!(scalar.binding().member(), CanonicalNumericInputMember::Scalar {index:0,declaration} if core::ptr::eq(declaration,source.inputs.scalars().first().ok_or("original scalar")?))
                );
                assert!(scalar.source().metadata.is_same_record(
                    source
                        .numeric_metadata
                        .get(1)
                        .ok_or("original scalar metadata")?
                ));
                assert_eq!(scalar.demand(), demand);
            }
        } else {
            let error = result.expect_err("original numeric association must refuse");
            assert!(core::ptr::eq(error.parent(), &parent));
            assert_eq!(error.parameter(), parameter);
            assert_eq!(error.kind(), kind);
            assert_eq!(error.limits(), allowance);
            assert_eq!(error.returned().is_some(), !matches!(case, 1 | 12 | 13));
            match (case, error.refusal()) {
                (1, DraftingProfileNumericRefusal::Source(SourceError::Missing)) => {}
                (2..=7, DraftingProfileNumericRefusal::Mismatch(actual)) => {
                    use DraftingProfileNumericIssue as I;
                    assert_eq!(
                        *actual,
                        [
                            I::Root,
                            I::SelectedPin,
                            I::Inventory,
                            I::Profile,
                            I::Snapshot,
                            I::Parameter
                        ]
                        .get(usize::from(case - 2))
                        .copied()
                        .ok_or("expected header axis")?
                    );
                }
                (8 | 9 | 10 | 15, DraftingProfileNumericRefusal::Canonical(cause)) => {
                    assert!(core::ptr::eq(cause.inputs(), &source.inputs));
                    assert_eq!(cause.target(), *error.returned().unwrap().target);
                    match (case, cause.refusal()) {
                        (
                            8,
                            CanonicalNumericInputRefusal::MissingLength {
                                other: Some(CanonicalInputPosition::Scalar { index: 0 }),
                            },
                        )
                        | (
                            9,
                            CanonicalNumericInputRefusal::MissingScalar {
                                other: Some(CanonicalInputPosition::Length { index: 0 }),
                            },
                        )
                        | (15, CanonicalNumericInputRefusal::MissingLength { other: None }) => {}
                        (
                            10,
                            CanonicalNumericInputRefusal::ScalarKind {
                                expected: ScalarInputKind::Boolean,
                                index: 0,
                                actual,
                            },
                        ) => {
                            assert!(core::ptr::eq(
                                actual,
                                source.inputs.scalars().first().ok_or("original scalar")?
                            ));
                        }
                        _ => {
                            return Err(format!(
                                "wrong original canonical numeric cause: {cause:?}"
                            )
                            .into())
                        }
                    }
                }
                (
                    11,
                    DraftingProfileNumericRefusal::Kind {
                        actual: ScalarInputKind::Boolean,
                    },
                ) => {}
                (12 | 13, DraftingProfileNumericRefusal::Resources(_)) => {}
                _ => return Err(format!("wrong first numeric cause: {:?}", error.refusal()).into()),
            }
        }
    }
    Ok(())
}
