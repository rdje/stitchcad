//! Formula syntax/literal normalization and later ordered recipe validation/evaluation.
//!
//! The lexical stream and immutable expression trees borrow machine-form source with precise spans.
//! Expression parsing checks grammar and node/conditional limits, not numeric conversion,
//! canonical identity, statements, type/name validity or execution. Literal conversion is explicit
//! on parsed nodes or a separate whole normalized arena; neither evaluates operators. Normalized
//! expressions can produce owned canonical identity bytes without name/type validation or execution.
//! Standalone let/assert statements preserve closed annotations, full-source spans and operand
//! refusals without binding or evaluating. Complete recipes retain authored order, global spans and
//! the fixed statement bound. Whole statement/recipe input normalization preserves metadata and
//! contextual literal refusals. Immutable recipe documents own original source and syntax identity.
//! Owned expression/statement/recipe bytes preserve typed identity;
//! Closed kind/origin/reserved-context metadata and immutable source locators are available;
//! initial namespaces retain sources, reject collisions and resolve exact names; ordered metadata scopes
//! retain actual prior let declarations; closed operator/built-in/selector kind signatures and typed wanted-kind catalogs are available.
//! Bounded initial-scope expression proofs retain kinds, typed refusals and sourced dependencies.
//! Current scopes certify actual statements and both assertion operands. Whole-recipe proofs
//! check every statement before exposing complete sourced dependencies. Exact numeric values,
//! operators, authored quantization and binding conversion are available. Canonical length authored
//! reads retain source/state beside exact values or required observation/evaluation; recipe execution follows.
mod lexer;
pub use lexer::{
    FormulaLexeme, FormulaLexemeKind, FormulaLexer, FormulaLexicalError, FormulaLexicalRule,
    FormulaSourceSpan,
};

mod expression;
mod parser;
pub use expression::{
    FormulaArguments, FormulaBinaryOperator, FormulaExpression, FormulaExpressionLimit,
    FormulaNode, FormulaNodeKind, FormulaParseError, FormulaParseRule, FormulaUnit,
};

mod literal;
pub use literal::{
    FormulaLiteral, FormulaLiteralError, FormulaLiteralKind, FormulaLiteralRule,
    FormulaRationalComponent,
};

mod rational;
pub use rational::{FormulaRational, FormulaRationalError};

mod numeric;
pub use numeric::{FormulaNumericError, FormulaNumericValue};

mod roots;

mod arc;
mod inverse;
mod pi;
mod trig;

mod bound;
pub use bound::{FormulaBindingValueError, FormulaBindingValueRule, FormulaBoundValue};

mod provenance;
pub use provenance::{FormulaApproximationSources, FormulaApproximationToleranceError};

mod evaluated;
pub use evaluated::{
    FormulaAssertionFailure, FormulaConditionalBranch, FormulaConditionalValueError,
    FormulaEvaluatedBinding, FormulaEvaluatedScalar, FormulaSatisfiedAssertion,
    FormulaToleranceComparison, FormulaToleranceComparisonError, FormulaToleranceComparisonRefusal,
};

mod normalized;
pub use normalized::{
    FormulaNormalizedArguments, FormulaNormalizedExpression, FormulaNormalizedNode,
    FormulaNormalizedNodeKind,
};

mod canonical;
pub use canonical::FormulaCanonicalExpression;

mod statement;
pub use statement::{
    FormulaBindingKind, FormulaStatement, FormulaStatementError, FormulaStatementExpression,
    FormulaStatementKind, FormulaStatementRule, FormulaToleranceName,
};

mod ordered;
pub use ordered::{FormulaRecipe, FormulaRecipeError, FormulaRecipeRule};

mod normalized_recipe;
pub use normalized_recipe::{
    FormulaNormalizedRecipe, FormulaNormalizedStatement, FormulaNormalizedStatementKind,
    FormulaRecipeLiteralError, FormulaStatementLiteralError,
};

mod canonical_recipe;
pub use canonical_recipe::{FormulaCanonicalRecipe, FormulaCanonicalStatement};

mod document;
pub use document::{FormulaRecipeDocument, FormulaRecipeDocumentError};

mod semantic;
pub use semantic::{
    FormulaKind, FormulaOrigin, FormulaReservedContext, FormulaReservedName, FormulaSizeName,
};

mod operators;
pub use operators::FormulaUnaryOperator;

mod builtins;
pub use builtins::{
    FormulaBuiltin, FormulaBuiltinArity, FormulaBuiltinCategory, FormulaBuiltinOperand,
};

mod signatures;
pub use signatures::{FormulaKindSignature, FormulaOperandRequirement, FormulaResultRequirement};

mod calls;
pub use calls::{
    FormulaCallAlternative, FormulaCallLookupSource, FormulaCallRefusal, FormulaCallRefusalKind,
};

mod declaration;
pub use declaration::{
    FormulaDeclaration, FormulaDeclarationSource, FormulaInputOrigin, FormulaScalarInputOrigin,
};

mod input;
pub use input::{
    FormulaLengthInput, FormulaLengthInputError, FormulaLengthInputValue, FormulaScalarInput,
    FormulaScalarInputAdapterError, FormulaScalarInputAdapterRule, FormulaScalarInputError,
    FormulaScalarInputValue,
};

mod context;
pub use context::{
    FormulaSizeContext, FormulaSizeContextError, FormulaSizeContextSource, FormulaSizeReadError,
    FormulaSizeValue, FormulaToleranceContext, FormulaToleranceContextDefinition,
    FormulaToleranceProvider, FormulaToleranceProviderError, FormulaToleranceReadError,
    FormulaToleranceValue, FormulaToleranceValueSource,
};

mod namespace;
pub use namespace::{
    FormulaInitialDeclaration, FormulaNameCursor, FormulaNamespace, FormulaNamespaceError,
    FormulaStatementNameScope, FormulaUnboundName,
};

mod checked;
pub use checked::{
    FormulaCheckedArguments, FormulaCheckedExpression, FormulaCheckedNode, FormulaCheckedNodeForm,
    FormulaCheckedOperation, FormulaDimensionRefusal, FormulaExpressionCheckError,
    FormulaExpressionCheckRefusal, FormulaNameDependency,
};

mod checked_statement;
pub use checked_statement::{
    FormulaBindingDimensionRefusal, FormulaCheckedStatement, FormulaCheckedStatementKind,
    FormulaStatementCheckError, FormulaStatementCheckRefusal,
};

mod checked_recipe;
pub use checked_recipe::{
    FormulaCheckedRecipe, FormulaRecipeCheckError, FormulaRecipeCheckRefusal,
    FormulaRecipeDependencyRole, FormulaRecipeNameDependency,
};
