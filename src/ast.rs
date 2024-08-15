use crate::token::Token;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) enum Expr<'a> {
    Binary {
        left: Box<Expr<'a>>,
        operator: Token<'a>,
        right: Box<Expr<'a>>,
    },
    Unary {
        operator: Token<'a>,
        right: Box<Expr<'a>>,
    },
    Grouping {
        expression: Box<Expr<'a>>,
    },
    StringLiteral(&'a str),
    BooleanLiteral(bool),
    NoneLiteral,
    NumberLiteral(f64),
}

pub(crate) trait Visitor<T> {
    fn visit_expr(
        &mut self,
        expr: &Expr,
    ) -> T;
}
