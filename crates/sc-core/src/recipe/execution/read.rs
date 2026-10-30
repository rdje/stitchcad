//! Actual selected-node reads retain canonical state and quantitative/size provider sources.
use super::{FormulaScalarExecutionFailure as Failure, FormulaScalarExecutionSourceKind as Kind};
use crate::recipe::{
    FormulaCanonicalInputAdapter, FormulaCheckedNode, FormulaEvaluatedScalar as V,
    FormulaLengthInputValue, FormulaNumericValue, FormulaScalarInputValue, FormulaSizeContext,
    FormulaSizeValue, FormulaToleranceContext, FormulaToleranceValue,
};
use core::fmt;

/// Explicit immutable runtime tolerance/size slots; canonical input ownership stays in the plan.
/// A size frame validates structure; derive it from the actual published membership adapter.
/// This context grants no membership/currentness/evidence approval and has no implicit defaults.
/// ```compile_fail
/// let context: sc_core::recipe::FormulaScalarExecutionContext<'static> = Default::default();
/// ```
#[derive(Clone, Copy)]
pub struct FormulaScalarExecutionContext<'a> {
    tolerances: FormulaToleranceContext<'a>,
    size: FormulaSizeContext,
}
impl fmt::Debug for FormulaScalarExecutionContext<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let definition = self.tolerances.definition();
        f.debug_struct("FormulaScalarExecutionContext")
            .field("format_present", &definition.format.is_some())
            .field("importer_present", &definition.importer.is_some())
            .field("physical_present", &definition.physical.is_some())
            .field("size_present", &self.size.source().is_some())
            .finish_non_exhaustive()
    }
}
impl<'a> FormulaScalarExecutionContext<'a> {
    /// Supply both actual context slots explicitly; empty slots stay empty until a selected read.
    #[must_use]
    pub const fn new(tolerances: FormulaToleranceContext<'a>, size: FormulaSizeContext) -> Self {
        Self { tolerances, size }
    }
    /// Actual class-checked quantitative tolerance context.
    #[must_use]
    pub const fn tolerances(self) -> FormulaToleranceContext<'a> {
        self.tolerances
    }
    /// Actual supplied size-value frame, without asserting canonical membership/currentness.
    #[must_use]
    pub const fn size(self) -> FormulaSizeContext {
        self.size
    }
}

/// Original successful selected read, retaining its actual state/provider/source witness.
#[derive(Clone, Copy)]
pub enum FormulaScalarExecutionReadKind<'a> {
    /// Dedicated canonical length with its complete original authored state/source.
    Length(FormulaLengthInputValue<'a>),
    /// General scalar, including actual false/zero and original state/source.
    Scalar(FormulaScalarInputValue<'a>),
    /// Once-read quantitative tolerance, class/derivation and actual provider/intrinsic source.
    Tolerance(FormulaToleranceValue<'a>),
    /// Actual supplied size-role value and pinned source frame.
    Size(FormulaSizeValue),
}
impl fmt::Debug for FormulaScalarExecutionReadKind<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Length(_) => "Length(..)",
            Self::Scalar(_) => "Scalar(..)",
            Self::Tolerance(_) => "Tolerance(..)",
            Self::Size(_) => "Size(..)",
        })
    }
}

#[derive(Clone, Copy)]
pub(super) struct ReadData<'a> {
    pub(super) source: super::FormulaScalarSourceGraphReadKind<'a>,
    pub(super) value: V,
}
impl<'a> ReadData<'a> {
    pub(super) const fn direct_source(self) -> FormulaScalarExecutionReadKind<'a> {
        match self.source {
            super::FormulaScalarSourceGraphReadKind::Direct(source) => source,
            super::FormulaScalarSourceGraphReadKind::Derived(_) => {
                // One-shot readers cannot publish an executed Derived source.
                unreachable!()
            }
        }
    }
}
impl fmt::Debug for ReadData<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ReadData")
            .field("source", &self.source)
            .field("kind", &self.value.kind())
            .finish_non_exhaustive()
    }
}

/// One actual executed name occurrence, its exact scalar value and original source/state witness.
/// Private fields prevent substituting a supplied annotation or value for an executed AST node.
#[derive(Clone, Copy, Debug)]
pub struct FormulaScalarExecutionRead<'a> {
    pub(super) node: FormulaCheckedNode<'a, 'a>,
    pub(super) data: ReadData<'a>,
}
impl<'a> FormulaScalarExecutionRead<'a> {
    /// Actual checked name and original authored span/declaration.
    #[must_use]
    pub const fn node(self) -> FormulaCheckedNode<'a, 'a> {
        self.node
    }
    /// Original successful canonical/provider read with its complete source/state.
    #[must_use]
    pub const fn source(self) -> FormulaScalarExecutionReadKind<'a> {
        self.data.direct_source()
    }
    /// Exact actual scalar read; false/zero remain values and are never coerced or defaulted.
    #[must_use]
    pub const fn value(self) -> V {
        self.data.value
    }
}

pub(super) fn read<'a>(
    kind: Kind<'a>,
    context: FormulaScalarExecutionContext<'a>,
) -> Result<ReadData<'a>, Failure<'a>> {
    use FormulaScalarExecutionReadKind as Read;
    let (source, value) = match kind {
        Kind::Canonical(joined) => match joined.adapter() {
            FormulaCanonicalInputAdapter::Length(input) => {
                let read = input.read_authored().map_err(Failure::LengthInput)?;
                (Read::Length(read), V::from_numeric(read.value()))
            }
            FormulaCanonicalInputAdapter::Scalar(input) => {
                let read = input.read_authored().map_err(Failure::ScalarInput)?;
                let value = if let Some(value) = read.value().boolean() {
                    V::from_boolean(value)
                } else {
                    V::from_numeric(
                        FormulaNumericValue::try_from(read.value()).map_err(Failure::Numeric)?,
                    )
                };
                (Read::Scalar(read), value)
            }
        },
        Kind::Reserved(name) => {
            if let Some(tolerance) = name.tolerance_name() {
                let read = context
                    .tolerances
                    .read(tolerance)
                    .map_err(Failure::ToleranceRead)?;
                (Read::Tolerance(read), V::from_numeric(read.value()))
            } else {
                let Some(size_name) = name.size_name() else {
                    unreachable!(
                        "the closed reserved vocabulary contains only tolerance and size roles"
                    );
                };
                let read = context.size.read(size_name).map_err(Failure::SizeRead)?;
                let value = if let Some(value) = read.boolean() {
                    V::from_boolean(value)
                } else if let Some(value) = read.numeric_value() {
                    V::from_numeric(value)
                } else {
                    unreachable!("a successful private size value is exactly Boolean or Count");
                };
                (Read::Size(read), value)
            }
        }
    };
    Ok(ReadData {
        source: super::FormulaScalarSourceGraphReadKind::Direct(source),
        value,
    })
}
