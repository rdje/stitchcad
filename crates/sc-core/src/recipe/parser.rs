//! Iterative machine-expression parser. No recursive descent or recursive arena destruction.
use super::expression::{Node, NodeData};
use super::{
    FormulaBinaryOperator as B, FormulaExpression, FormulaExpressionLimit as L, FormulaLexeme,
    FormulaLexemeKind as K, FormulaLexer, FormulaParseError as Error, FormulaParseRule as R,
    FormulaSourceSpan as Span, FormulaUnit,
};
use std::iter::Peekable;

#[derive(Debug)]
enum FrameKind<'a> {
    Root,
    Group,
    Call(&'a str),
    Conditional,
}
#[derive(Debug)]
struct Frame<'a> {
    kind: FrameKind<'a>,
    span: Span,
    value_start: usize,
    operator_start: usize,
    arguments: Vec<usize>,
    comparison_seen: bool,
}
#[derive(Clone, Copy, Debug)]
enum Operator {
    Negate(Span),
    Binary(B),
}
impl Operator {
    fn precedence(self) -> u8 {
        match self {
            Self::Negate(_) => 4,
            Self::Binary(B::Multiply | B::Divide) => 3,
            Self::Binary(B::Add | B::Subtract) => 2,
            Self::Binary(_) => 1,
        }
    }
}
struct Parser<'a> {
    source: &'a str,
    lexer: Peekable<FormulaLexer<'a>>,
    nodes: Vec<Node<'a>>,
    values: Vec<usize>,
    operators: Vec<Operator>,
    frames: Vec<Frame<'a>>,
    seen_nodes: usize,
    open_ifs: usize,
    max_ifs: usize,
}
impl<'a> Parser<'a> {
    fn error(&self, rule: R, span: Span) -> Error {
        Error { rule, span }
    }
    fn eof(&self) -> Span {
        Span::new(self.source.len(), self.source.len())
    }
    fn lexical(error: super::FormulaLexicalError) -> Error {
        Error {
            span: error.span(),
            rule: R::Lexical(error.rule()),
        }
    }
    fn peek(&mut self) -> Result<Option<FormulaLexeme<'a>>, Error> {
        self.lexer
            .peek()
            .copied()
            .transpose()
            .map_err(Self::lexical)
    }
    fn take(&mut self) -> Result<Option<FormulaLexeme<'a>>, Error> {
        self.lexer.next().transpose().map_err(Self::lexical)
    }
    fn reserve(&mut self, span: Span) -> Result<(), Error> {
        self.seen_nodes += 1;
        if self.seen_nodes > L::Nodes.bound() {
            return Err(self.error(
                R::StructuralLimit {
                    limit: L::Nodes,
                    measured: self.seen_nodes,
                },
                span,
            ));
        }
        Ok(())
    }
    fn frame(&self) -> Result<&Frame<'a>, Error> {
        self.frames
            .last()
            .ok_or_else(|| self.error(R::InternalStructure, self.eof()))
    }
    fn node_span(&self, index: usize) -> Result<Span, Error> {
        self.nodes
            .get(index)
            .map(|node| node.span)
            .ok_or_else(|| self.error(R::InternalStructure, self.eof()))
    }
    fn push_node(&mut self, data: NodeData<'a>, span: Span) {
        let index = self.nodes.len();
        self.nodes.push(Node { data, span });
        self.values.push(index);
    }
    fn pop_value(&mut self) -> Result<usize, Error> {
        if self.values.len() <= self.frame()?.value_start {
            return Err(self.error(R::InternalStructure, self.eof()));
        }
        self.values
            .pop()
            .ok_or_else(|| self.error(R::InternalStructure, self.eof()))
    }
    fn reduce(&mut self) -> Result<(), Error> {
        let operator = self
            .operators
            .pop()
            .ok_or_else(|| self.error(R::InternalStructure, self.eof()))?;
        let right = self.pop_value()?;
        let end = self.node_span(right)?.end();
        match operator {
            Operator::Negate(span) => {
                self.push_node(NodeData::Negate(right), Span::new(span.start(), end))
            }
            Operator::Binary(operator) => {
                let left = self.pop_value()?;
                let start = self.node_span(left)?.start();
                self.push_node(
                    NodeData::Binary {
                        operator,
                        left,
                        right,
                    },
                    Span::new(start, end),
                );
            }
        }
        Ok(())
    }
    fn complete_argument(&mut self) -> Result<usize, Error> {
        let operator_start = self.frame()?.operator_start;
        while self.operators.len() > operator_start {
            self.reduce()?;
        }
        if self.values.len() != self.frame()?.value_start + 1 {
            return Err(self.error(R::InternalStructure, self.eof()));
        }
        self.pop_value()
    }
    fn open(&mut self, kind: FrameKind<'a>, span: Span) -> Result<(), Error> {
        if matches!(kind, FrameKind::Conditional) {
            self.open_ifs += 1;
            if self.open_ifs > L::ConditionalDepth.bound() {
                return Err(self.error(
                    R::StructuralLimit {
                        limit: L::ConditionalDepth,
                        measured: self.open_ifs,
                    },
                    span,
                ));
            }
            self.max_ifs = self.max_ifs.max(self.open_ifs);
        }
        self.frames.push(Frame {
            kind,
            span,
            value_start: self.values.len(),
            operator_start: self.operators.len(),
            arguments: Vec::new(),
            comparison_seen: false,
        });
        Ok(())
    }
    fn number(&mut self, token: FormulaLexeme<'a>) -> Result<(), Error> {
        let mut end = token.span().end();
        let mut unit = None;
        if let Some(next) = self.peek()? {
            if next.kind() == K::Identifier {
                if let Some(value) = FormulaUnit::from_token(next.text()) {
                    let gap = Span::new(end, next.span().start());
                    if self.source.get(gap.start()..gap.end()) != Some(" ") {
                        return Err(self.error(R::UnitSeparator, gap));
                    }
                    unit = Some(value);
                    end = next.span().end();
                    let _ = self.take()?;
                }
            }
        }
        self.reserve(token.span())?;
        self.push_node(
            NodeData::Literal {
                number: token.text(),
                unit,
            },
            Span::new(token.span().start(), end),
        );
        Ok(())
    }
    fn word(&mut self, token: FormulaLexeme<'a>) -> Result<bool, Error> {
        let conditional = token.kind() == K::If;
        if self.peek()?.is_some_and(|next| next.kind() == K::LeftParen) {
            let _ = self.take()?;
            self.reserve(token.span())?;
            self.open(
                if conditional {
                    FrameKind::Conditional
                } else {
                    FrameKind::Call(token.text())
                },
                token.span(),
            )?;
            Ok(true)
        } else if conditional {
            let span = self.peek()?.map_or(self.eof(), |next| next.span());
            Err(self.error(R::ExpectedCallParenthesis, span))
        } else {
            self.reserve(token.span())?;
            self.push_node(NodeData::Name(token.text()), token.span());
            Ok(false)
        }
    }
    fn close(&mut self, closing: Span) -> Result<(), Error> {
        if matches!(self.frame()?.kind, FrameKind::Root) {
            return Err(self.error(R::UnexpectedDelimiter, closing));
        }
        let last = self.complete_argument()?;
        let mut frame = self
            .frames
            .pop()
            .ok_or_else(|| self.error(R::InternalStructure, closing))?;
        let span = Span::new(frame.span.start(), closing.end());
        match frame.kind {
            FrameKind::Group => {
                let node = self.nodes.get_mut(last).ok_or(Error {
                    rule: R::InternalStructure,
                    span,
                })?;
                node.span = span;
                self.values.push(last);
            }
            FrameKind::Call(name) => {
                frame.arguments.push(last);
                self.push_node(
                    NodeData::Call {
                        name,
                        arguments: frame.arguments,
                    },
                    span,
                );
            }
            FrameKind::Conditional => {
                frame.arguments.push(last);
                let [condition, then_branch, else_branch] = frame.arguments.as_slice() else {
                    return Err(self.error(R::ConditionalArity, span));
                };
                self.open_ifs -= 1;
                self.push_node(
                    NodeData::Conditional {
                        condition: *condition,
                        then_branch: *then_branch,
                        else_branch: *else_branch,
                    },
                    span,
                );
            }
            FrameKind::Root => return Err(self.error(R::InternalStructure, span)),
        }
        Ok(())
    }
    fn binary(&mut self, operator: B, span: Span) -> Result<(), Error> {
        let pending = Operator::Binary(operator);
        if pending.precedence() == 1 {
            if self.frame()?.comparison_seen {
                return Err(self.error(R::ChainedComparison, span));
            }
            let frame = self.frames.last_mut().ok_or(Error {
                rule: R::InternalStructure,
                span,
            })?;
            frame.comparison_seen = true;
        }
        self.reserve(span)?;
        while self.operators.len() > self.frame()?.operator_start
            && self
                .operators
                .last()
                .is_some_and(|previous| previous.precedence() >= pending.precedence())
        {
            self.reduce()?;
        }
        self.operators.push(pending);
        Ok(())
    }
    fn run(mut self) -> Result<FormulaExpression<'a>, Error> {
        let mut needs_operand = true;
        let mut can_square = false;
        while let Some(token) = self.take()? {
            let span = token.span();
            if needs_operand {
                match token.kind() {
                    K::Minus => {
                        self.reserve(span)?;
                        self.operators.push(Operator::Negate(span));
                    }
                    K::LeftParen => self.open(FrameKind::Group, span)?,
                    K::Number => {
                        self.number(token)?;
                        needs_operand = false;
                        can_square = true;
                    }
                    K::Identifier | K::If => {
                        needs_operand = self.word(token)?;
                        can_square = !needs_operand;
                    }
                    K::RightParen
                        if matches!(
                            self.frame()?.kind,
                            FrameKind::Call(_) | FrameKind::Conditional
                        ) && self.frame()?.arguments.is_empty()
                            && self.operators.len() == self.frame()?.operator_start =>
                    {
                        return Err(self.error(R::EmptyArguments, span));
                    }
                    _ => return Err(self.error(R::ExpectedOperand, span)),
                }
                continue;
            }
            if let Some(operator) = binary_operator(token.kind()) {
                self.binary(operator, span)?;
                needs_operand = true;
                can_square = false;
                continue;
            }
            match token.kind() {
                K::Power => {
                    if !can_square {
                        return Err(self.error(R::RepeatedPower, span));
                    }
                    let exponent = self.take()?;
                    if !exponent
                        .is_some_and(|value| value.kind() == K::Number && value.text() == "2")
                    {
                        return Err(self.error(
                            R::UnsupportedExponent,
                            exponent.map_or(self.eof(), |value| value.span()),
                        ));
                    }
                    let end = exponent.map_or(span.end(), |value| value.span().end());
                    self.reserve(span)?;
                    let child = self.pop_value()?;
                    let start = self.node_span(child)?.start();
                    self.push_node(NodeData::Square(child), Span::new(start, end));
                    can_square = false;
                }
                K::Comma => {
                    if !matches!(
                        self.frame()?.kind,
                        FrameKind::Call(_) | FrameKind::Conditional
                    ) {
                        return Err(self.error(R::UnexpectedDelimiter, span));
                    }
                    let root = self.complete_argument()?;
                    let frame = self.frames.last_mut().ok_or(Error {
                        rule: R::InternalStructure,
                        span,
                    })?;
                    frame.arguments.push(root);
                    frame.comparison_seen = false;
                    needs_operand = true;
                    can_square = false;
                }
                K::RightParen => {
                    self.close(span)?;
                    can_square = true;
                }
                _ => return Err(self.error(R::UnexpectedToken, span)),
            }
        }
        if self.frames.len() != 1 {
            return Err(self.error(R::UnclosedParenthesis, self.frame()?.span));
        }
        if needs_operand {
            return Err(self.error(R::ExpectedOperand, self.eof()));
        }
        let root = self.complete_argument()?;
        if self.seen_nodes != self.nodes.len() || self.nodes.get(root).is_none() {
            return Err(self.error(R::InternalStructure, self.eof()));
        }
        Ok(FormulaExpression {
            nodes: self.nodes,
            root,
            if_depth: self.max_ifs,
        })
    }
}
fn binary_operator(kind: K) -> Option<B> {
    match kind {
        K::Plus => Some(B::Add),
        K::Minus => Some(B::Subtract),
        K::Multiply => Some(B::Multiply),
        K::Divide => Some(B::Divide),
        K::Equal => Some(B::Equal),
        K::NotEqual => Some(B::NotEqual),
        K::Less => Some(B::Less),
        K::LessEqual => Some(B::LessEqual),
        K::Greater => Some(B::Greater),
        K::GreaterEqual => Some(B::GreaterEqual),
        _ => None,
    }
}
pub(super) fn parse(source: &str) -> Result<FormulaExpression<'_>, Error> {
    Parser {
        source,
        lexer: FormulaLexer::new(source).peekable(),
        nodes: Vec::new(),
        values: Vec::new(),
        operators: Vec::new(),
        frames: vec![Frame {
            kind: FrameKind::Root,
            span: Span::new(0, 0),
            value_start: 0,
            operator_start: 0,
            arguments: Vec::new(),
            comparison_seen: false,
        }],
        seen_nodes: 0,
        open_ifs: 0,
        max_ifs: 0,
    }
    .run()
}
