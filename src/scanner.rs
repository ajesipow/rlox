use std::mem;

use crate::error::LexicalError;
use crate::token::Token;
use crate::token::TokenKind;
use crate::token::Tokens;

#[derive(Debug)]
pub(crate) struct Scanner;

impl Scanner {
    pub(crate) fn scan_tokens(source: String) -> Tokens {
        let mut tokens = vec![];
        let mut lexeme = String::new();
        let mut line = 1;

        // TODO: Iterates over Unicode Scalar Values instead of grapheme clusters.
        let mut characters = source.chars().peekable();
        while let Some(char) = characters.next() {
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
                '!' => Ok(TokenKind::Bang),
                '=' => Ok(TokenKind::Equal),
                '<' => Ok(TokenKind::Less),
                '>' => Ok(TokenKind::Greater),
                '/' => Ok(TokenKind::Slash),
                '\t' | ' ' | '\r' => continue,
                '\n' => {
                    line += 1;
                    continue;
                }
                _ => Err(LexicalError::UnexpectedCharacter { char, line }),
            };
            lexeme.push(char);

            let lex_result = match token_kind {
                Ok(token_kind) => {
                    let (token_kind, extra_char_for_lexeme) = match token_kind {
                        TokenKind::Bang => {
                            if let Some(c) = characters.next_if_eq(&'=') {
                                (TokenKind::BangEqual, Some(c))
                            } else {
                                (token_kind, None)
                            }
                        }
                        TokenKind::Equal => {
                            if let Some(c) = characters.next_if_eq(&'=') {
                                (TokenKind::EqualEqual, Some(c))
                            } else {
                                (token_kind, None)
                            }
                        }
                        TokenKind::Less => {
                            if let Some(c) = characters.next_if_eq(&'=') {
                                (TokenKind::LessEqual, Some(c))
                            } else {
                                (token_kind, None)
                            }
                        }
                        TokenKind::Greater => {
                            if let Some(c) = characters.next_if_eq(&'=') {
                                (TokenKind::GreaterEqual, Some(c))
                            } else {
                                (token_kind, None)
                            }
                        }
                        TokenKind::Slash => {
                            if characters.next_if_eq(&'/').is_some() {
                                // We're discarding comments
                                'comment: while let Some(c) = characters.peek() {
                                    if *c == '\n' {
                                        // We handle newlines separately, so don't consume it
                                        break 'comment;
                                    } else {
                                        // Consume the comment
                                        characters.next();
                                    }
                                }
                                continue;
                            } else {
                                (token_kind, None)
                            }
                        }
                        _ => (token_kind, None),
                    };
                    if let Some(c) = extra_char_for_lexeme {
                        lexeme.push(c);
                    }

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
