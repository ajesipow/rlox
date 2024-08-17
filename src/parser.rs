use itertools::peek_nth;
use itertools::PeekNth;

use crate::ast::Expr;
use crate::error::ParseError;
use crate::error::ParseErrorInternal;
use crate::token::Token;
use crate::token::TokenKind;

pub(crate) struct Parser<'a> {
    tokens: PeekNth<std::vec::IntoIter<Token<'a>>>,
}

impl<'a> Parser<'a> {
    pub(crate) fn new<I>(tokens: I) -> Self
    where
        I: IntoIterator<Item = Token<'a>, IntoIter = std::vec::IntoIter<Token<'a>>>,
    {
        Self {
            tokens: peek_nth(tokens),
        }
    }

    pub(crate) fn parse(&mut self) -> Result<Expr<'a>, ParseError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr<'a>, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr<'a>, ParseError> {
        let mut expr = self.comparison()?;

        while let Some(token) = self.tokens.peek() {
            match token.kind() {
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

    fn comparison(&mut self) -> Result<Expr<'a>, ParseError> {
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

    fn term(&mut self) -> Result<Expr<'a>, ParseError> {
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

    fn factor(&mut self) -> Result<Expr<'a>, ParseError> {
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

    fn unary(&mut self) -> Result<Expr<'a>, ParseError> {
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

    fn primary(&mut self) -> Result<Expr<'a>, ParseError> {
        if let Some(token) = self.tokens.next() {
            return match token.kind() {
                TokenKind::True { .. } => Ok(Expr::BooleanLiteral(true)),
                TokenKind::False { .. } => Ok(Expr::BooleanLiteral(false)),
                TokenKind::Nil { .. } => Ok(Expr::NoneLiteral),
                TokenKind::Number { lexeme } => Ok(Expr::NumberLiteral(lexeme)),
                TokenKind::String { lexeme } => Ok(Expr::StringLiteral(lexeme)),
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
    use itertools::Itertools;

    use crate::ast::Expr;
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::token::Token;
    use crate::token::TokenKind;

    #[test]
    fn test_parsing_basic_expression() {
        let input = "(1 + 2) * 3";
        let tokens = Lexer::lex(input);

        let mut parser = Parser::new(tokens.into_iter().flatten().collect_vec());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            Expr::Binary {
                left: Box::new(Expr::Grouping {
                    expression: Box::new(Expr::Binary {
                        left: Box::new(Expr::NumberLiteral(1.0)),
                        operator: Token::new(TokenKind::Plus { lexeme: "+" }, 1),
                        right: Box::new(Expr::NumberLiteral(2.0)),
                    })
                }),
                operator: Token::new(TokenKind::Star { lexeme: "*" }, 1),
                right: Box::new(Expr::NumberLiteral(3.0)),
            }
        )
    }

    #[test]
    fn test_parsing_basic_expression_2() {
        let input = "1 + 2 * 3";
        let tokens = Lexer::lex(input);

        let mut parser = Parser::new(tokens.into_iter().flatten().collect_vec());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            Expr::Binary {
                left: Box::new(Expr::NumberLiteral(1.0)),
                operator: Token::new(TokenKind::Plus { lexeme: "+" }, 1),
                right: Box::new(Expr::Binary {
                    left: Box::new(Expr::NumberLiteral(2.0)),
                    operator: Token::new(TokenKind::Star { lexeme: "*" }, 1),
                    right: Box::new(Expr::NumberLiteral(3.0)),
                }),
            }
        )
    }
}
