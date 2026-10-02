//! Canonical expression identity, independent of source locations and numerical execution.
use super::normalized::NormalizedData;
use super::{FormulaBinaryOperator, FormulaNormalizedExpression};
use core::fmt;

/// Owned canonical expression bytes, produced only from a validated normalized arena.
///
/// Equality compares bytes, including kinds, operators and ordered children. It does not compare
/// evaluated numerical values. Debug deliberately omits customer names and literal magnitudes.
/// ```compile_fail
/// fn forge() -> sc_core::recipe::FormulaCanonicalExpression {
///     sc_core::recipe::FormulaCanonicalExpression { text: String::from("count:1") }
/// }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct FormulaCanonicalExpression {
    text: String,
}
impl FormulaCanonicalExpression {
    /// Explicit access to customer-bearing ASCII identity bytes, without a terminal newline.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.text
    }

    /// Transfer the owned bytes; the returned string may be edited independently of this identity.
    #[must_use]
    pub fn into_string(self) -> String {
        self.text
    }
}
impl fmt::Debug for FormulaCanonicalExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaCanonicalExpression")
            .field("byte_count", &self.text.len())
            .finish_non_exhaustive()
    }
}

enum Action<'a> {
    Node(usize),
    Text(&'a str),
}

impl FormulaNormalizedExpression<'_> {
    /// Serialize all unevaluated syntax into the exact canonical expression byte contract.
    ///
    /// An explicit heap stack preserves all ordered children without recursive traversal. Literal
    /// conversion has already succeeded: this operation neither rounds nor evaluates anything.
    /// The result owns its bytes and may outlive both source and syntax/normalized arenas.
    /// ```
    /// use sc_core::recipe::FormulaExpression;
    /// let syntax = FormulaExpression::parse("-2.5 cm ^ 2")?;
    /// let normalized = syntax.normalize_literals()?;
    /// let canonical = normalized.canonical_form();
    /// assert_eq!(canonical.as_str(), "(- (^2 length:25000))");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    #[allow(clippy::indexing_slicing)] // Private indices preserve the validated parser arena.
    pub fn canonical_form(&self) -> FormulaCanonicalExpression {
        let mut text = String::new();
        let mut pending = vec![Action::Node(self.root)];
        while let Some(action) = pending.pop() {
            let index = match action {
                Action::Text(part) => {
                    text.push_str(part);
                    continue;
                }
                Action::Node(index) => index,
            };
            // Actions are pushed in reverse output order because this is a LIFO stack.
            match &self.nodes[index].data {
                NormalizedData::Literal(literal) => {
                    text.push_str(literal.kind().token());
                    text.push(':');
                    text.push_str(&literal.magnitude().to_string());
                }
                NormalizedData::Name(name) => text.push_str(name),
                NormalizedData::Negate(child) => {
                    pending.push(Action::Text(")"));
                    pending.push(Action::Node(*child));
                    pending.push(Action::Text("(- "));
                }
                NormalizedData::Square(child) => {
                    pending.push(Action::Text(")"));
                    pending.push(Action::Node(*child));
                    pending.push(Action::Text("(^2 "));
                }
                NormalizedData::Binary {
                    operator,
                    left,
                    right,
                } => {
                    pending.push(Action::Text(")"));
                    pending.push(Action::Node(*right));
                    pending.push(Action::Text(" "));
                    pending.push(Action::Node(*left));
                    pending.push(Action::Text(" "));
                    pending.push(Action::Text(binary_symbol(*operator)));
                    pending.push(Action::Text("("));
                }
                NormalizedData::Call { name, arguments } => {
                    pending.push(Action::Text(")"));
                    for child in arguments.iter().rev() {
                        pending.push(Action::Node(*child));
                        pending.push(Action::Text(" "));
                    }
                    pending.push(Action::Text(name));
                    pending.push(Action::Text("("));
                }
                NormalizedData::Conditional {
                    condition,
                    then_branch,
                    else_branch,
                } => {
                    pending.push(Action::Text(")"));
                    pending.push(Action::Node(*else_branch));
                    pending.push(Action::Text(" "));
                    pending.push(Action::Node(*then_branch));
                    pending.push(Action::Text(" "));
                    pending.push(Action::Node(*condition));
                    pending.push(Action::Text("(if "));
                }
            }
        }
        FormulaCanonicalExpression { text }
    }
}

const fn binary_symbol(operator: FormulaBinaryOperator) -> &'static str {
    match operator {
        FormulaBinaryOperator::Add => "+",
        FormulaBinaryOperator::Subtract => "-",
        FormulaBinaryOperator::Multiply => "*",
        FormulaBinaryOperator::Divide => "/",
        FormulaBinaryOperator::Equal => "==",
        FormulaBinaryOperator::NotEqual => "!=",
        FormulaBinaryOperator::Less => "<",
        FormulaBinaryOperator::LessEqual => "<=",
        FormulaBinaryOperator::Greater => ">",
        FormulaBinaryOperator::GreaterEqual => ">=",
    }
}
