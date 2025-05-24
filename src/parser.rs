use std::rc::Rc;

use itertools::peek_nth;
use itertools::PeekNth;

use crate::ast::Expr;
use crate::ast::Stmt;
use crate::error::ParseError;
use crate::error::ParseErrorInternal;
use crate::token::Token;
use crate::token::TokenKind;

pub(crate) struct Parser {
    tokens: PeekNth<std::vec::IntoIter<Token>>,
}

impl Parser {
    pub(crate) fn new<I>(tokens: I) -> Self
    where
        I: IntoIterator<Item = Token, IntoIter = std::vec::IntoIter<Token>>,
    {
        Self {
            tokens: peek_nth(tokens),
        }
    }

    pub(crate) fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = vec![];
        while let Some(t) = self.tokens.peek() {
            match t.kind() {
                TokenKind::Eof => break,
                _ => {
                    if let Some(d) = self.declaration()? {
                        statements.push(d);
                    }
                }
            }
        }
        Ok(statements)
    }

    fn synchronize(&mut self) {
        while let Some(t) = self.tokens.peek() {
            match t.kind() {
                TokenKind::Eof | TokenKind::Semicolon { .. } => {
                    self.tokens.next(); // Consume the token
                    break;
                }
                TokenKind::Class { .. }
                | TokenKind::For { .. }
                | TokenKind::Fun { .. }
                | TokenKind::If { .. }
                | TokenKind::Print { .. }
                | TokenKind::Return { .. }
                | TokenKind::Var { .. }
                | TokenKind::While { .. } => break,
                _ => {
                    self.tokens.next();
                }
            }
        }
    }

    fn declaration(&mut self) -> Result<Option<Stmt>, ParseError> {
        if let Some(t) = self.tokens.peek() {
            let func = match t.kind() {
                TokenKind::Var { .. } => {
                    self.tokens.next();
                    Self::var_declaration
                }
                _ => Self::statement,
            };
            match func(self) {
                Ok(stmt) => Ok(Some(stmt)),
                Err(_) => {
                    self.synchronize();
                    Ok(None)
                }
            }
        } else {
            Err(ParseError::UnexpectedEof)
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let token = self.tokens.next().ok_or(ParseError::UnexpectedEof)?;
        let name = match token.kind() {
            TokenKind::Identifier { lexeme } => lexeme,
            _ => return Err(ParseError::ExpectIdentifier),
        };

        let initializer = match self.tokens.next() {
            Some(t) => match t.kind() {
                TokenKind::Equal { .. } => Some(self.expression()?),
                _ => None,
            },
            None => None,
        };

        let t = self.tokens.next().ok_or(ParseError::UnexpectedEof)?;
        match t.kind() {
            TokenKind::Semicolon { .. } => (),
            _ => return Err(ParseError::ExpectSemicolon),
        }

        Ok(Stmt::Var {
            name: Rc::clone(name),
            expr: initializer,
        })
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        match self.tokens.peek() {
            Some(t) => {
                match t.kind() {
                    TokenKind::Print { .. } => {
                        self.tokens.next(); // Consume the print token
                        self.print_statement()
                    }
                    _ => self.expression_statement(),
                }
            }
            None => Err(ParseError::ExpectStatement),
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        if !matches!(self.tokens.next(), Some(t) if matches!(t.kind(), TokenKind::Semicolon { .. }))
        {
            return Err(ParseError::ExpectSemicolon);
        }
        Ok(Stmt::Print(expr))
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        if !matches!(self.tokens.next(), Some(t) if matches!(t.kind(), TokenKind::Semicolon { .. }))
        {
            return Err(ParseError::ExpectSemicolon);
        }
        Ok(Stmt::Expr(expr))
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.equality()?;

        if let Some(tok) = self.tokens.peek() {
            if matches!(tok.kind(), TokenKind::Equal { .. }) {
                self.tokens.next();
                let value = self.assignment()?;

                return match expr {
                    Expr::Variable(tok) => Ok(Expr::Assign {
                        name: tok,
                        value: Box::new(value),
                    }),
                    _ => Err(ParseError::InvalidAssignmentTarget),
                };
            }
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while let Some(t) = self.tokens.peek() {
            match t.kind() {
                TokenKind::BangEqual { .. } | TokenKind::EqualEqual { .. } => {
                    let operator = self.tokens.next().expect("cannot fail");
                    let right = self.comparison()?;
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        operator,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while let Some(token) = self.tokens.peek() {
            match token.kind() {
                TokenKind::Greater { .. }
                | TokenKind::GreaterEqual { .. }
                | TokenKind::Less { .. }
                | TokenKind::LessEqual { .. } => {
                    let operator = self.tokens.next().expect("cannot fail");
                    let right = self.term()?;
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        operator,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while let Some(token) = self.tokens.peek() {
            match token.kind() {
                TokenKind::Minus { .. } | TokenKind::Plus { .. } => {
                    let operator = self.tokens.next().expect("cannot fail");
                    let right = self.factor()?;
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        operator,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while let Some(token) = self.tokens.peek() {
            match token.kind() {
                TokenKind::Star { .. } | TokenKind::Slash { .. } => {
                    let operator = self.tokens.next().expect("cannot fail");
                    let right = self.unary()?;
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        operator,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if let Some(token) = self.tokens.peek() {
            match token.kind() {
                TokenKind::Bang { .. } | TokenKind::Minus { .. } => {
                    let operator = self.tokens.next().expect("cannot fail");
                    let right = self.unary()?;
                    return Ok(Expr::Unary {
                        operator,
                        right: Box::new(right),
                    });
                }
                _ => (),
            }
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if let Some(token) = self.tokens.next() {
            return match token.kind() {
                TokenKind::True { .. } => Ok(Expr::BooleanLiteral(true)),
                TokenKind::False { .. } => Ok(Expr::BooleanLiteral(false)),
                TokenKind::Nil { .. } => Ok(Expr::NoneLiteral),
                TokenKind::Number { lexeme } => Ok(Expr::NumberLiteral(*lexeme)),
                TokenKind::String { lexeme } => Ok(Expr::StringLiteral(Rc::clone(lexeme))),
                TokenKind::Identifier { .. } => Ok(Expr::Variable(token)),
                TokenKind::LeftParen { .. } => {
                    let expr = self.expression()?;
                    if let Some(next_token) = self.tokens.next() {
                        if !matches!(next_token.kind(), TokenKind::RightParen { .. }) {
                            return Err(ParseError::ExpectedClosingParenAfterExpr {
                                line: next_token.line(),
                            });
                        }
                    } else {
                        return Err(ParseError::UnexpectedEof);
                    }
                    Ok(Expr::Grouping {
                        expression: Box::new(expr),
                    })
                }
                _ => Err(ParseError::Internal(ParseErrorInternal::UnhandledToken {
                    line: token.line(),
                })),
            };
        }
        Err(ParseError::ExpectExpression)
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use itertools::Itertools;

    use crate::ast::Expr;
    use crate::ast::Stmt;
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::token::Token;
    use crate::token::TokenKind;

    #[test]
    fn test_parsing_basic_expression() {
        let input = "(1 + 2) * 3;";
        let tokens = Lexer::lex(Rc::from(input));

        let mut parser = Parser::new(tokens.into_iter().flatten().collect_vec());
        let ast = parser.parse().unwrap();
        assert_eq!(ast.len(), 1);
        assert_eq!(
            ast[0],
            Stmt::Expr(Expr::Binary {
                left: Box::new(Expr::Grouping {
                    expression: Box::new(Expr::Binary {
                        left: Box::new(Expr::NumberLiteral(1.0)),
                        operator: Token::new(
                            TokenKind::Plus {
                                lexeme: Rc::from("+")
                            },
                            1
                        ),
                        right: Box::new(Expr::NumberLiteral(2.0)),
                    })
                }),
                operator: Token::new(
                    TokenKind::Star {
                        lexeme: Rc::from("*")
                    },
                    1
                ),
                right: Box::new(Expr::NumberLiteral(3.0)),
            })
        )
    }

    #[test]
    fn test_parsing_basic_expression_2() {
        let input = "1 + 2 * 3;";
        let tokens = Lexer::lex(Rc::from(input));

        let mut parser = Parser::new(tokens.into_iter().flatten().collect_vec());
        let ast = parser.parse().unwrap();
        assert_eq!(ast.len(), 1);
        assert_eq!(
            ast[0],
            Stmt::Expr(Expr::Binary {
                left: Box::new(Expr::NumberLiteral(1.0)),
                operator: Token::new(
                    TokenKind::Plus {
                        lexeme: Rc::from("+")
                    },
                    1
                ),
                right: Box::new(Expr::Binary {
                    left: Box::new(Expr::NumberLiteral(2.0)),
                    operator: Token::new(
                        TokenKind::Star {
                            lexeme: Rc::from("*")
                        },
                        1
                    ),
                    right: Box::new(Expr::NumberLiteral(3.0)),
                }),
            })
        )
    }
}
