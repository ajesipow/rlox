use std::fmt::Display;
use std::fmt::Formatter;
use std::rc::Rc;

use crate::token::Token;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) enum Stmt {
    Expr(Expr),
    Print(Expr),
    Var { name: Rc<str>, expr: Option<Expr> },
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Variable(Rc<str>),
    StringLiteral(Rc<str>),
    BooleanLiteral(bool),
    NoneLiteral,
    NumberLiteral(f64),
}

#[derive(Debug, Clone)]
pub(crate) enum Literal {
    Number(f64),
    String(Rc<str>),
    Boolean(bool),
    None,
}

impl Display for Literal {
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
