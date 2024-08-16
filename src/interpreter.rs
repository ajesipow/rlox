use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Not;
use std::ops::Sub;

use crate::ast::Expr;
use crate::ast::Literal;
use crate::error::RunTimeError;
use crate::token::TokenKind;

pub(crate) struct Interpreter {}

impl Interpreter {
    pub(crate) fn interpret(exr: Box<Expr>) -> Result<String, RunTimeError> {
        Self::interpret_inner(exr).map(|l| l.to_string())
    }

    fn interpret_inner(expr: Box<Expr>) -> Result<Literal, RunTimeError> {
        match *expr {
            Expr::NumberLiteral(n) => Ok(Literal::Number(n)),
            Expr::BooleanLiteral(b) => Ok(Literal::Boolean(b)),
            Expr::StringLiteral(s) => Ok(Literal::String(s)),
            Expr::NoneLiteral => Ok(Literal::None),
            Expr::Grouping { expression } => Self::interpret_inner(expression),
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = Self::interpret_inner(left)?;
                let right = Self::interpret_inner(right)?;
                let op = operator.kind();
                match (op, left, right) {
                    (TokenKind::Minus { .. }, Literal::Number(l), Literal::Number(r)) => {
                        Ok(Literal::Number(l.sub(r)))
                    }
                    (TokenKind::Slash { .. }, Literal::Number(l), Literal::Number(r)) => {
                        Ok(Literal::Number(l.div(r)))
                    }
                    (TokenKind::Star { .. }, Literal::Number(l), Literal::Number(r)) => {
                        Ok(Literal::Number(l.mul(r)))
                    }
                    (TokenKind::Plus { .. }, Literal::Number(l), Literal::Number(r)) => {
                        Ok(Literal::Number(l.add(r)))
                    }
                    (TokenKind::Greater { .. }, Literal::Number(l), Literal::Number(r)) => {
                        Ok(Literal::Boolean(l > r))
                    }
                    (TokenKind::GreaterEqual { .. }, Literal::Number(l), Literal::Number(r)) => {
                        Ok(Literal::Boolean(l >= r))
                    }
                    (TokenKind::Less { .. }, Literal::Number(l), Literal::Number(r)) => {
                        Ok(Literal::Boolean(l < r))
                    }
                    (TokenKind::LessEqual { .. }, Literal::Number(l), Literal::Number(r)) => {
                        Ok(Literal::Boolean(l <= r))
                    }

                    (TokenKind::BangEqual { .. }, Literal::Number(l), Literal::Number(r)) => {
                        Ok(Literal::Boolean(l != r))
                    }
                    (TokenKind::BangEqual { .. }, Literal::String(l), Literal::String(r)) => {
                        Ok(Literal::Boolean(l != r))
                    }
                    (TokenKind::BangEqual { .. }, Literal::None, Literal::None) => {
                        Ok(Literal::Boolean(false))
                    }
                    (TokenKind::BangEqual { .. }, Literal::None, _) => Ok(Literal::Boolean(true)),
                    (TokenKind::EqualEqual { .. }, Literal::Number(l), Literal::Number(r)) => {
                        Ok(Literal::Boolean(l == r))
                    }
                    (TokenKind::EqualEqual { .. }, Literal::String(l), Literal::String(r)) => {
                        Ok(Literal::Boolean(l == r))
                    }
                    (TokenKind::EqualEqual { .. }, Literal::None, Literal::None) => {
                        Ok(Literal::Boolean(true))
                    }
                    (TokenKind::EqualEqual { .. }, Literal::None, _) => Ok(Literal::Boolean(false)),
                    (o, l, r) => Err(RunTimeError::UnexpectedBinaryToken {
                        line: operator.line(),
                        left: l.to_string(),
                        right: r.to_string(),
                    }),
                }
            }
            Expr::Unary { operator, right } => {
                let right = Self::interpret_inner(right)?;
                let op = operator.kind();
                match (op, right) {
                    (TokenKind::Minus { .. }, Literal::Number(n)) => Ok(Literal::Number(-n)),
                    (TokenKind::Bang { .. }, Literal::Boolean(n)) => Ok(Literal::Boolean(n.not())),
                    (TokenKind::Bang { .. }, Literal::None) => Ok(Literal::Boolean(true)),
                    (o, r) => Err(RunTimeError::UnexpectedUnaryToken {
                        line: operator.line(),
                        literal: r.to_string(),
                    }),
                }
            }
        }
    }
}
