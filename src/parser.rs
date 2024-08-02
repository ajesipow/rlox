use itertools::peek_nth;
use itertools::PeekNth;

use crate::ast::Expr;
use crate::token::Token;
use crate::token::TokenKind;

pub(crate) struct Parser<'a> {
    tokens: PeekNth<std::vec::IntoIter<Token<'a>>>,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(tokens: std::vec::IntoIter<Token<'a>>) -> Self {
        Self {
            tokens: peek_nth(tokens),
        }
    }

    pub(crate) fn parse(&mut self) -> Expr<'a> {
        self.expression()
    }

    fn expression(&mut self) -> Expr<'a> {
        self.equality()
    }

    fn equality(&mut self) -> Expr<'a> {
        let expr = self.comparison();

        while let Some(token) = self.tokens.peek() {
            match token.kind() {
                TokenKind::BangEqual | TokenKind::EqualEqual => {
                    let operator = self.tokens.next().expect("cannot fail");
                    let right = self.comparison();
                    return Expr::Binary {
                        left: Box::new(expr),
                        operator,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        expr
    }

    fn comparison(&mut self) -> Expr<'a> {
        let expr = self.term();

        while let Some(token) = self.tokens.peek() {
            match token.kind() {
                TokenKind::Greater
                | TokenKind::GreaterEqual
                | TokenKind::Less
                | TokenKind::LessEqual => {
                    let operator = self.tokens.next().expect("cannot fail");
                    let right = self.term();
                    return Expr::Binary {
                        left: Box::new(expr),
                        operator,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        expr
    }

    fn term(&mut self) -> Expr<'a> {
        let expr = self.factor();

        while let Some(token) = self.tokens.peek() {
            match token.kind() {
                TokenKind::Minus | TokenKind::Plus => {
                    let operator = self.tokens.next().expect("cannot fail");
                    let right = self.factor();
                    return Expr::Binary {
                        left: Box::new(expr),
                        operator,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        expr
    }

    fn factor(&mut self) -> Expr<'a> {
        let expr = self.unary();

        while let Some(token) = self.tokens.peek() {
            match token.kind() {
                TokenKind::Star | TokenKind::Slash => {
                    let operator = self.tokens.next().expect("cannot fail");
                    let right = self.unary();
                    return Expr::Binary {
                        left: Box::new(expr),
                        operator,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        expr
    }

    fn unary(&mut self) -> Expr<'a> {
        if let Some(token) = self.tokens.peek() {
            match token.kind() {
                TokenKind::Bang | TokenKind::Minus => {
                    let operator = self.tokens.next().expect("cannot fail");
                    let right = self.unary();
                    return Expr::Unary {
                        operator,
                        right: Box::new(right),
                    };
                }
                _ => (),
            }
        }
        self.primary()
    }

    fn primary(&mut self) -> Expr<'a> {
        if let Some(token) = self.tokens.next() {
            return match token.kind() {
                TokenKind::True => Expr::BooleanLiteral(true),
                TokenKind::False => Expr::BooleanLiteral(false),
                TokenKind::Nil => Expr::NoneLiteral,
                // TODO make TokenKind accept value in variant
                TokenKind::Number => Expr::NumberLiteral(0.0),
                TokenKind::String => Expr::StringLiteral("a"),
                TokenKind::LeftParen => {
                    let expr = self.expression();
                    if let Some(next_token) = self.tokens.next() {
                        if next_token.kind() != TokenKind::RightParen {
                            panic!("expected ')' after expression")
                        }
                    } else {
                        panic!("unexpected EOF")
                    }
                    Expr::Grouping {
                        expression: Box::new(expr),
                    }
                }
                k => panic!("unhandled token {k:?}"),
            };
        }
        panic!("unexpected EOF")
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::parser::Parser;
    use crate::scanner::Scanner;

    #[test]
    fn test_basic_parser() {
        let input = "(1 + 2) * 3";
        let tokens = Scanner::scan_tokens(input);

        let mut parser = Parser::new(tokens.into_iter().flatten().collect_vec().into_iter());
        let ast = parser.parse();
        println!("{ast:#?}");
    }
}
