#![doc = include_str!("../../../docs/book/src/spec/constraints.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod ast;
mod problem;
mod rational;
mod render;
mod solve;

pub use ast::{
    Binding, Comparison, Domain, Expr, Linear, Rule, Scope, Status, Symbol, TemplatePart, Value,
    Variable, AST_VERSION,
};
pub use problem::{Problem, ProblemError, MAX_DOMAIN_VALUES};
pub use rational::Rational;
pub use render::Catalog;
pub use solve::{CoreItem, Limits, OpenFact, Origin, Outcome, Selection, Solution, Unsatisfiable};
