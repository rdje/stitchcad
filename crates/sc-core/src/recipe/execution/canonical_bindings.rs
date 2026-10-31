//! Typed deterministic stored-binding output; derived data is not project persistence authority.
use super::{
    FormulaExecutedScalarRecipe, FormulaExecutedScalarRecipeStatement,
    FormulaExecutedScalarSourceRecipe, FormulaScalarRecipeOutcome,
};
use core::fmt::{self, Write};
use std::collections::TryReserveError;

/// Owned canonical stored bindings from a complete accepted scalar recipe.
/// Preserves binding order, names, stored kinds/values and approximation contributions.
/// Equality covers only these bytes, not recipe/proof identity, assertions, reads or source caches.
/// This distinct type defines no project envelope, reader, hash framing or complete result identity.
/// ```compile_fail
/// fn forge() -> sc_core::recipe::FormulaCanonicalBindings {
///     sc_core::recipe::FormulaCanonicalBindings { text: String::from("(bindings)") }
/// }
/// ```
/// ```compile_fail
/// fn compare(output:sc_core::recipe::FormulaCanonicalBindings,
///     recipe:sc_core::recipe::FormulaCanonicalRecipe)->bool { output==recipe }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct FormulaCanonicalBindings {
    text: String,
}
impl FormulaCanonicalBindings {
    /// Explicit customer-bearing ASCII bytes, without a newline, locale or wall-clock data.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.text
    }
    /// Transfer owned bytes; editing this string changes no result or other output.
    #[must_use]
    pub fn into_string(self) -> String {
        self.text
    }
}
impl fmt::Debug for FormulaCanonicalBindings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaCanonicalBindings")
            .field("byte_count", &self.text.len())
            .finish_non_exhaustive()
    }
}

/// Complete output measurement or reservation refusal, without a partial owned output.
#[derive(Debug)]
pub enum FormulaCanonicalBindingsError {
    /// Complete byte population cannot be represented.
    PopulationOverflow,
    /// Complete measured ASCII output exceeds the explicit caller allowance, before allocation.
    Bytes {
        /// Explicit output-byte allowance.
        allowed: usize,
        /// Complete measured bytes, including framing and source contributions.
        required: usize,
    },
    /// Actual single output reservation failed, retaining its original cause inline.
    Allocation {
        /// Complete measured output bytes requested.
        requested: usize,
        /// Original fallible reservation cause.
        error: TryReserveError,
    },
}
impl fmt::Display for FormulaCanonicalBindingsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PopulationOverflow => {
                f.write_str("canonical binding output byte population overflow")
            }
            Self::Bytes { allowed, required } => write!(
                f,
                "canonical binding output requires {required} bytes; allowed {allowed}"
            ),
            Self::Allocation { requested, error } => write!(
                f,
                "canonical binding output reservation of {requested} bytes: {error}"
            ),
        }
    }
}
impl std::error::Error for FormulaCanonicalBindingsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Allocation { error, .. } => Some(error),
            _ => None,
        }
    }
}

struct Count(usize);
impl Write for Count {
    fn write_str(&mut self, part: &str) -> fmt::Result {
        self.0 = self.0.checked_add(part.len()).ok_or(fmt::Error)?;
        Ok(())
    }
}
struct Buffer {
    text: String,
    limit: usize,
}
impl Write for Buffer {
    fn write_str(&mut self, part: &str) -> fmt::Result {
        let end = self.text.len().checked_add(part.len()).ok_or(fmt::Error)?;
        if end > self.limit {
            return Err(fmt::Error);
        }
        // The entire limit was fallibly reserved before writing; capacity cannot grow here.
        self.text.push_str(part);
        Ok(())
    }
}
fn append(
    out: &mut impl Write,
    statements: &[FormulaExecutedScalarRecipeStatement<'_>],
) -> fmt::Result {
    out.write_str("(bindings")?;
    for statement in statements {
        let FormulaScalarRecipeOutcome::Binding { binding, .. } = statement.outcome() else {
            continue;
        };
        let value = binding.value();
        write!(
            out,
            " (bind {} {}:",
            statement.statement().statement().name(),
            value.kind().token()
        )?;
        if let Some(integer) = value.numeric_integer() {
            write!(out, "{integer}")?;
        } else if let Some(boolean) = value.boolean() {
            write!(out, "{boolean}")?;
        } else {
            unreachable!("private completed binding is exactly numeric or Boolean");
        }
        out.write_str(" (sources")?;
        for source in binding.sources().iter() {
            write!(out, " {}", source.token())?;
        }
        out.write_str("))")?;
    }
    out.write_char(')')
}
fn encode(
    statements: &[FormulaExecutedScalarRecipeStatement<'_>],
    max_bytes: usize,
) -> Result<FormulaCanonicalBindings, FormulaCanonicalBindingsError> {
    let mut count = Count(0);
    append(&mut count, statements)
        .map_err(|_| FormulaCanonicalBindingsError::PopulationOverflow)?;
    if count.0 > max_bytes {
        return Err(FormulaCanonicalBindingsError::Bytes {
            allowed: max_bytes,
            required: count.0,
        });
    }
    let mut text = String::new();
    text.try_reserve_exact(count.0)
        .map_err(|error| FormulaCanonicalBindingsError::Allocation {
            requested: count.0,
            error,
        })?;
    let mut buffer = Buffer {
        text,
        limit: count.0,
    };
    if append(&mut buffer, statements).is_err() || buffer.text.len() != count.0 {
        unreachable!("same immutable completed bindings reproduce their complete measured bytes");
    }
    Ok(FormulaCanonicalBindings { text: buffer.text })
}

impl FormulaExecutedScalarRecipe<'_> {
    /// Serialize actual stored bindings in declaration order, with their approximation sources.
    /// Measures all bytes before the explicit quota check and one fallible output reservation.
    /// No RHS is reevaluated, value rounded again, or recipe/read/source-cache identity granted.
    /// ```
    /// use sc_core::recipe::*;
    /// fn output(result:&FormulaExecutedScalarRecipe<'_>) {
    ///     let output=result.canonical_bindings(4096).expect("explicit byte allowance");
    ///     assert!(output.as_str().starts_with("(bindings"));
    /// }
    /// ```
    /// # Errors
    /// Complete byte overflow/quota or original allocation refusal exposes no partial output.
    pub fn canonical_bindings(
        &self,
        max_bytes: usize,
    ) -> Result<FormulaCanonicalBindings, FormulaCanonicalBindingsError> {
        encode(self.statements(), max_bytes)
    }
}
impl FormulaExecutedScalarSourceRecipe<'_> {
    /// Serialize only the accepted recipe's stored bindings, using the direct mode's byte contract.
    /// Unbound Derived roots, reads, assertions and execution policies are outside these bytes.
    /// # Errors
    /// Complete byte overflow/quota or original allocation refusal exposes no partial output.
    pub fn canonical_bindings(
        &self,
        max_bytes: usize,
    ) -> Result<FormulaCanonicalBindings, FormulaCanonicalBindingsError> {
        encode(self.statements(), max_bytes)
    }
}
