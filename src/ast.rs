use std::fmt::Display;
use std::fmt::Formatter;

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

#[derive(Debug)]
pub(crate) enum Literal<'a> {
    Number(f64),
    String(&'a str),
    Boolean(bool),
    None,
}

impl<'a> Display for Literal<'a> {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        let v = match self {
            Literal::Number(n) => n.to_string(),
            Literal::String(s) => s.to_string(),
            Literal::Boolean(b) => b.to_string(),
            Literal::None => "Nil".to_string(),
        };
        write!(f, "{v}")
    }
}
