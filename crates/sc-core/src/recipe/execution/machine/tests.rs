use super::*;
use crate::{recipe::*, value::CanonicalInputs};

#[test]
#[allow(clippy::expect_used)]
fn actual_operands_reuse_reserved_scratch_and_refuse_insufficient_capacity_before_reads() {
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
    let namespace = FormulaNamespace::new([]).expect("namespace");
    let first = FormulaExpression::parse("if(1==1,1 mm/3,eps_phys)")
        .expect("syntax")
        .normalize_literals()
        .expect("inputs");
    let second = FormulaExpression::parse("within(1 mm,1 mm,eps_num)")
        .expect("syntax")
        .normalize_literals()
        .expect("inputs");
    let first_proof = first.check_kinds(&namespace).expect("complete");
    let second_proof = second.check_kinds(&namespace).expect("complete");
    let mut first_plan = first_proof
        .admit_execution(FormulaExecutionLimits::new(8, 17))
        .expect("exact")
        .prepare_scalar(&inputs)
        .expect("plan");
    let mut second_plan = second_proof
        .admit_execution(FormulaExecutionLimits::new(4, 9))
        .expect("exact")
        .prepare_scalar(&inputs)
        .expect("plan");
    let first_root = first_plan.admission().root();
    let second_root = second_plan.admission().root();
    let context = Context::new(
        FormulaToleranceContext::empty(),
        FormulaSizeContext::empty(),
    );
    let error = Machine::new(0)
        .expect("empty allocation")
        .run(first_root, &mut first_plan, context)
        .expect_err("complete actual node count before source reads");
    assert!(
        matches!(error.cause(), FormulaScalarExecutionErrorCause::Evaluation(failure)
            if matches!(failure.as_ref(), Failure::WorkspaceReservation {
                workspace: Workspace::Tasks, admitted: 0, requested: 8 }))
    );
    assert!(first_plan.sources.iter().all(|slot| slot.read.is_none()));
    let mut machine = Machine::new(8).expect("complete reservation");
    let pointers = (machine.pending.as_ptr(), machine.values.as_ptr());
    let first_value = machine
        .run(first_root, &mut first_plan, context)
        .expect("actual first");
    assert_eq!(
        first_value.numeric().expect("Length").rational(),
        FormulaRational::new(1000, 3, false).expect("exact")
    );
    assert!(first_plan.sources.iter().all(|slot| slot.read.is_none()));
    let second_value = machine
        .run(second_root, &mut second_plan, context)
        .expect("actual second");
    assert_eq!(second_value.boolean(), Some(true));
    assert_eq!(
        second_plan
            .sources
            .iter()
            .filter(|slot| slot.read.is_some())
            .count(),
        1
    );
    assert_eq!(
        (machine.pending.as_ptr(), machine.values.as_ptr()),
        pointers
    );
    assert!(machine.pending.is_empty() && machine.values.is_empty());
}

// All suspended values below come from an actual mapped source's executed root,not a seeded V.
struct SourceFixture {
    inputs: CanonicalInputs,
    sources: FormulaSources,
    name: crate::name::MachineToken,
}
#[allow(clippy::expect_used)]
impl SourceFixture {
    fn new(consumer: &str, supplier: &str) -> Self {
        use crate::{ontology::EntityId, value::*};
        let inputs = CanonicalInputs::new(
            vec![LengthDeclaration::new(LengthDeclarationDefinition {
                id: EntityId::from_bits(10),
                source: EntityId::from_bits(4000),
                state: LengthState::Derived {
                    formula: EntityId::from_bits(9001),
                },
            })
            .expect("actual canonical Derived record")],
            vec![],
        )
        .expect("actual inventory");
        let sources: Vec<_> = [(9002, consumer), (9001, supplier)]
            .into_iter()
            .map(|(id, body)| {
                FormulaSource::new(
                    EntityId::from_bits(id),
                    FormulaExpressionDocument::new(body.into()).expect("original expression"),
                )
            })
            .collect();
        let limits = FormulaSourceLimits::new(
            2,
            sources.iter().map(|s| s.expression().source().len()).sum(),
            sources
                .iter()
                .map(|s| s.expression().canonical().as_str().len())
                .sum(),
        );
        Self {
            inputs,
            sources: FormulaSources::new(sources, limits).expect("actual source registry"),
            name: crate::name::MachineToken::new("a").expect("original name"),
        }
    }
    fn normalized(&self) -> FormulaNormalizedSources<'_> {
        let n = self
            .sources
            .sources()
            .iter()
            .map(|s| s.expression().node_count())
            .sum();
        FormulaSourceInputs::new(&self.inputs, &self.sources)
            .expect("actual Derived source join")
            .normalize_sources(FormulaSourceNormalizationLimits::new(2, n, 2 + 2 * n))
            .expect("all original source arenas")
    }
    fn namespace(&self) -> FormulaNamespace<'_> {
        FormulaNamespace::new([FormulaInitialDeclaration::try_from(
            FormulaDeclaration::length_input(
                &self.name,
                FormulaInputOrigin::Parameter,
                crate::ontology::EntityId::from_bits(5001),
                self.inputs
                    .length(crate::ontology::EntityId::from_bits(10))
                    .expect("actual target"),
            ),
        )
        .expect("actual initial metadata")])
        .expect("initial scope")
    }
}
#[allow(clippy::expect_used)]
fn source_plan<'a>(
    source: &'a FormulaCheckedSource<'a>,
    inputs: &'a CanonicalInputs,
) -> super::super::FormulaScalarExecutionPlan<'a> {
    let proof = source.proof();
    let n = proof.node_count();
    proof
        .admit_execution(FormulaExecutionLimits::new(
            n,
            2 * n + proof.dependencies().len(),
        ))
        .expect("actual complete operand")
        .prepare_scalar(inputs)
        .expect("actual canonical joins")
}
struct SuspendedSources<'a> {
    plan: super::super::FormulaScalarExecutionPlan<'a>,
    supplier: &'a FormulaCheckedSource<'a>,
    completed: Option<FormulaExecutedScalarExpression<'a>>,
    attempts: usize,
    derived_reads: usize,
}
#[allow(clippy::expect_used)]
impl<'a> SuspendedSources<'a> {
    fn complete(&mut self, result: FormulaExecutedScalarExpression<'a>) {
        assert!(self.completed.is_none());
        assert!(core::ptr::eq(
            result.root().expression(),
            self.supplier.proof().expression()
        ));
        assert!(core::ptr::eq(result.inputs(), self.plan.inputs()));
        self.completed = Some(result);
    }
}
impl<'a> Sources<'a> for SuspendedSources<'a> {
    #[allow(clippy::expect_used)]
    fn read_value(
        &mut self,
        node: Node<'a, 'a>,
        context: Context<'a>,
    ) -> Result<ReadValue<'a>, Failure<'a>> {
        self.attempts += 1;
        let slot = self
            .plan
            .sources
            .get(node.dependency_index().expect("actual name index"))
            .expect("complete actual source slot");
        assert!(slot.read.is_none());
        let kind = slot.kind.expect("prepared source");
        if let super::super::FormulaScalarExecutionSourceKind::Canonical(joined) = kind {
            let FormulaCanonicalInputAdapter::Length(input) = joined.adapter() else {
                unreachable!("this fixture has one dedicated Derived length");
            };
            let crate::value::LengthState::Derived { formula } = input.state() else {
                unreachable!("actual immutable canonical state is Derived");
            };
            assert_eq!(*formula, self.supplier.normalized().source().id());
            if let Some(result) = &self.completed {
                self.derived_reads += 1;
                return Ok(ReadValue::Ready(result.value()));
            }
            return Ok(ReadValue::AwaitDerived(joined));
        }
        self.plan.read_value(node, context)
    }
    fn tolerance(&self, node: Node<'a, 'a>) -> FormulaToleranceValue<'a> {
        self.plan.tolerance(node)
    }
}

#[test]
#[allow(clippy::expect_used)]
fn genuine_derived_suspension_retains_operands_calls_selected_conditions_and_original_buffers() {
    use crate::ontology::EntityId;
    for (consumer_body, supplier_body, expected, derived_reads, calls) in [
        ("eps_num+min(a*3,a*6)", "1 mm/3", 1001, 2, vec![]),
        (
            "if(sin(0 deg)==0.0,min(a*3,2 mm),a)",
            "1 mm/3+cos(0 deg)*0 mm",
            1000,
            1,
            vec![FormulaBuiltin::Cos, FormulaBuiltin::Sin],
        ),
        ("if(1==1,eps_num,a)", "1 mm/3", 1, 0, vec![]),
    ] {
        let fixture = SourceFixture::new(consumer_body, supplier_body);
        let before = fixture.inputs.clone();
        let normalized = fixture.normalized();
        let n = normalized.node_count();
        let checked = normalized
            .check_sources(
                &fixture.namespace(),
                FormulaSourceCheckLimits::new(2, n, 1, 2 + 2 * n),
            )
            .expect("actual initial source proofs");
        let consumer = checked
            .source(EntityId::from_bits(9002))
            .expect("actual consumer");
        let supplier = checked
            .source(EntityId::from_bits(9001))
            .expect("actual supplier");
        let context = Context::new(
            FormulaToleranceContext::empty(),
            FormulaSizeContext::empty(),
        );
        let mut sources = SuspendedSources {
            plan: source_plan(consumer, &fixture.inputs),
            supplier,
            completed: None,
            attempts: 0,
            derived_reads: 0,
        };
        let root = sources.plan.admission().root();
        let mut machine = Machine::new(root.all_nodes().len()).expect("complete scratch");
        let buffers = (machine.pending.as_ptr(), machine.values.as_ptr());
        machine
            .start(root, context)
            .expect("pin actual root and context");
        let progress = machine.resume(&mut sources).expect("selected execution");
        let value = match progress {
            Progress::AwaitDerived { node, input } => {
                assert!(derived_reads > 0);
                assert!(core::ptr::eq(
                    node.expression(),
                    consumer.proof().expression()
                ));
                assert_eq!(
                    consumer_body
                        .get(node.span().start()..node.span().end())
                        .expect("actual original name span"),
                    "a"
                );
                assert!(core::ptr::eq(input.inputs(), &fixture.inputs));
                let FormulaCanonicalInputAdapter::Length(input) = input.adapter() else {
                    unreachable!("actual Length supplier");
                };
                assert!(core::ptr::eq(
                    input.canonical_declaration(),
                    fixture
                        .inputs
                        .length(EntityId::from_bits(10))
                        .expect("actual canonical record")
                ));
                let pending = machine.pending.len();
                let values = machine.values.clone();
                let reserved_reads = sources
                    .plan
                    .sources
                    .iter()
                    .filter(|s| s.read.is_some())
                    .count();
                assert_eq!(sources.derived_reads, 0);
                assert!(
                    matches!(machine.pending.last(), Some(Task::Enter(actual)) if actual.span()==node.span())
                );
                let Progress::AwaitDerived { node: again, .. } = machine
                    .resume(&mut sources)
                    .expect("unresolved suspension remains suspension")
                else {
                    unreachable!("no supplier has executed yet");
                };
                assert_eq!(again.span(), node.span());
                assert_eq!(machine.pending.len(), pending);
                assert_eq!(machine.values, values);
                assert_eq!(
                    sources
                        .plan
                        .sources
                        .iter()
                        .filter(|s| s.read.is_some())
                        .count(),
                    reserved_reads
                );
                assert_eq!((machine.pending.as_ptr(), machine.values.as_ptr()), buffers);
                let result = source_plan(supplier, &fixture.inputs)
                    .evaluate(context)
                    .expect("execute original supplier,without storage rounding");
                assert_eq!(
                    result.value().numeric().expect("Length").rational(),
                    FormulaRational::new(1000, 3, false).expect("exact original root")
                );
                sources.complete(result);
                let Progress::Complete(value) = machine
                    .resume(&mut sources)
                    .expect("resume exact pending operand")
                else {
                    unreachable!("actual supplier is now completed");
                };
                value
            }
            Progress::Complete(value) => {
                assert_eq!(derived_reads, 0);
                assert!(sources.completed.is_none());
                value
            }
        };
        assert_eq!(
            value.numeric().expect("Length").rational(),
            FormulaRational::new(expected, 1, false).expect("exact")
        );
        assert_eq!(value.sources().iter().collect::<Vec<_>>(), calls);
        assert_eq!(sources.derived_reads, derived_reads);
        let reads = sources
            .plan
            .sources
            .iter()
            .filter(|s| s.read.is_some())
            .count();
        assert_eq!(
            sources.attempts,
            reads + derived_reads + if derived_reads > 0 { 2 } else { 0 }
        );
        assert_eq!((machine.pending.as_ptr(), machine.values.as_ptr()), buffers);
        assert!(machine.pending.is_empty() && machine.values.is_empty());
        assert!(matches!(machine.state, State::Idle));
        assert_eq!(fixture.inputs, before);
    }
}

#[test]
#[allow(clippy::expect_used)]
fn one_shot_derived_and_missing_provider_keep_original_causes_and_cannot_resume_or_restart() {
    use crate::{ontology::EntityId, value::LengthValueError};
    for body in ["a*3", "eps_phys+a"] {
        let fixture = SourceFixture::new(body, "1 mm/3");
        let normalized = fixture.normalized();
        let n = normalized.node_count();
        let checked = normalized
            .check_sources(
                &fixture.namespace(),
                FormulaSourceCheckLimits::new(2, n, 1, 2 + 2 * n),
            )
            .expect("actual proofs");
        let consumer = checked.source(EntityId::from_bits(9002)).expect("consumer");
        let mut sources = SuspendedSources {
            plan: source_plan(consumer, &fixture.inputs),
            supplier: checked.source(EntityId::from_bits(9001)).expect("supplier"),
            completed: None,
            attempts: 0,
            derived_reads: 0,
        };
        let root = sources.plan.admission().root();
        let context = Context::new(
            FormulaToleranceContext::empty(),
            FormulaSizeContext::empty(),
        );
        let mut machine = Machine::new(root.all_nodes().len()).expect("scratch");
        let error = machine
            .run(root, &mut sources, context)
            .expect_err("original real requirement");
        let FormulaScalarExecutionErrorCause::Evaluation(cause) = error.cause() else {
            unreachable!("actual read refusal");
        };
        if body == "a*3" {
            let Failure::LengthInput(error) = cause.as_ref() else {
                unreachable!("actual canonical source");
            };
            assert_eq!(
                error.reason(),
                LengthValueError::RequiresEvaluation {
                    declaration: EntityId::from_bits(10),
                    formula: EntityId::from_bits(9001)
                }
            );
            assert!(core::ptr::eq(
                error.input().canonical_declaration(),
                fixture
                    .inputs
                    .length(EntityId::from_bits(10))
                    .expect("actual record")
            ));
        } else {
            assert!(matches!(cause.as_ref(), Failure::ToleranceRead(_)));
        }
        assert_eq!(sources.attempts, 1);
        assert!(sources.plan.sources.iter().all(|s| s.read.is_none()));
        assert!(matches!(machine.state, State::Failed));
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(
            || machine.resume(&mut sources)
        ))
        .is_err());
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(
            || machine.start(root, context)
        ))
        .is_err());
    }
}
