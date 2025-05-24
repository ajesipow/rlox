use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Not;
use std::ops::Sub;

use crate::ast::Expr;
use crate::ast::Literal;
use crate::ast::Stmt;
use crate::environment::Environment;
use crate::error::RunTimeError;
use crate::token::TokenKind;

pub(crate) struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub(crate) fn interpret(
        &mut self,
        stmts: Vec<Stmt>,
    ) -> Result<(), RunTimeError> {
        for stmt in stmts {
            match stmt {
                Stmt::Expr(expr) => {
                    self.eval_expr(expr)?;
                }
                Stmt::Print(expr) => {
                    let val = self.eval_expr(expr).map(|l| l.to_string())?;
                    println!("{val}");
                }
                Stmt::Var { name, expr } => {
                    let value = match expr {
                        None => Literal::None,
                        Some(initializer) => self.eval_expr(initializer)?,
                    };
                    self.environment.define(name, value);
                }
            }
        }
        Ok(())
    }

    fn eval_expr(
        &self,
        expr: Expr,
    ) -> Result<Literal, RunTimeError> {
        match expr {
            Expr::NumberLiteral(n) => Ok(Literal::Number(n)),
            Expr::BooleanLiteral(b) => Ok(Literal::Boolean(b)),
            Expr::StringLiteral(s) => Ok(Literal::String(s)),
            Expr::NoneLiteral => Ok(Literal::None),
            Expr::Grouping { expression } => self.eval_expr(*expression),
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.eval_expr(*left)?;
                let right = self.eval_expr(*right)?;
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
                    (_, l, r) => Err(RunTimeError::UnexpectedBinaryToken {
                        line: operator.line(),
                        left: l.to_string(),
                        right: r.to_string(),
                    }),
                }
            }
            Expr::Unary { operator, right } => {
                let right = Self::eval_expr(self, *right)?;
                let op = operator.kind();
                match (op, right) {
                    (TokenKind::Minus { .. }, Literal::Number(n)) => Ok(Literal::Number(-n)),
                    (TokenKind::Bang { .. }, Literal::Boolean(n)) => Ok(Literal::Boolean(n.not())),
                    (TokenKind::Bang { .. }, Literal::None) => Ok(Literal::Boolean(true)),
                    (_, r) => Err(RunTimeError::UnexpectedUnaryToken {
                        line: operator.line(),
                        literal: r.to_string(),
                    }),
                }
            }
            Expr::Variable(var) => {
                match var.kind()  {
                    TokenKind::Identifier { lexeme} => self.environment.get(lexeme),
                    _ => Err(RunTimeError::ExpectedIdentifierToken)
                }
            },
            Expr::Assign { name, value } => {
                let val = self.eval_expr(*value)?;
                self.environment.assign(name, val.clone())?;
                Ok(val)
            }
        }
    }
}
