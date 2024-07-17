use crate::error::LexicalError;
use crate::token::TokenKind;
use crate::token::{Token, Tokens};
use std::mem;

#[derive(Debug)]
pub(crate) struct Scanner;

impl Scanner {
    pub(crate) fn scan_tokens(source: String) -> Tokens {
        let mut tokens = vec![];
        let mut lexeme = String::new();
        let line = 1;

        // TODO: Iterates over Unicode Scalar Values instead of grapheme clusters.
        let mut characters = source.chars().peekable();
        while let Some(char) = characters.next() {
            lexeme.push(char);
            let token_kind = match char {
                '(' => Ok(TokenKind::LeftParen),
                ')' => Ok(TokenKind::RightParen),
                '{' => Ok(TokenKind::LeftBrace),
                '}' => Ok(TokenKind::RightBrace),
                ',' => Ok(TokenKind::Comma),
                '.' => Ok(TokenKind::Dot),
                '-' => Ok(TokenKind::Minus),
                '+' => Ok(TokenKind::Plus),
                ';' => Ok(TokenKind::Semicolon),
                '*' => Ok(TokenKind::Star),
                '!' => {
                    if characters.next_if_eq(&'=').is_some() {
                        Ok(TokenKind::BangEqual)
                    } else {
                        Ok(TokenKind::Bang)
                    }
                },
                '=' => {
                    if characters.next_if_eq(&'=').is_some() {
                        Ok(TokenKind::EqualEqual)
                    } else {
                        Ok(TokenKind::Equal)
                    }
                },
                '<' => {
                    if characters.next_if_eq(&'=').is_some() {
                        Ok(TokenKind::LessEqual)
                    } else {
                        Ok(TokenKind::Less)
                    }
                },
                '>' => {
                    if characters.next_if_eq(&'=').is_some() {
                        Ok(TokenKind::GreaterEqual)
                    } else {
                        Ok(TokenKind::Greater)
                    }
                }
                _ => Err(LexicalError::UnexpectedCharacter { char, line }),
            };
            let lex_result = match token_kind {
                Ok(token_kind) => {
                    let this_lexeme = mem::take(&mut lexeme);
                    Ok(Token::new(token_kind, Some(this_lexeme), line))
                }
                Err(lexical_error) => Err(lexical_error),
            };
            tokens.push(lex_result);
        }

        tokens.push(Ok(Token::new(TokenKind::Eof, None, line)));

        Tokens(tokens)
    }
}
