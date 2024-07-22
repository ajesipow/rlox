use std::mem;

use itertools::peek_nth;

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
        let mut characters = peek_nth(source.chars());
        while let Some(char) = characters.next() {
            let token_kind = match char {
                '(' => {
                    lexeme.push(char);
                    Ok(TokenKind::LeftParen)
                }
                ')' => {
                    lexeme.push(char);
                    Ok(TokenKind::RightParen)
                }
                '{' => {
                    lexeme.push(char);
                    Ok(TokenKind::LeftBrace)
                }
                '}' => {
                    lexeme.push(char);
                    Ok(TokenKind::RightBrace)
                }
                ',' => {
                    lexeme.push(char);
                    Ok(TokenKind::Comma)
                }
                '.' => {
                    lexeme.push(char);
                    Ok(TokenKind::Dot)
                }
                '-' => {
                    lexeme.push(char);
                    Ok(TokenKind::Minus)
                }
                '+' => {
                    lexeme.push(char);
                    Ok(TokenKind::Plus)
                }
                ';' => {
                    lexeme.push(char);
                    Ok(TokenKind::Semicolon)
                }
                '*' => {
                    lexeme.push(char);
                    Ok(TokenKind::Star)
                }
                c if c.is_ascii_digit() => {
                    lexeme.push(char);
                    while let Some(next_digit) = characters.next_if(|c| c.is_ascii_digit()) {
                        lexeme.push(next_digit);
                    }

                    if let Some('.') = characters.peek() {
                        match characters.peek_nth(1) {
                            Some(c) if c.is_ascii_digit() => {
                                // Consume the '.'
                                let dot = characters.next().unwrap();
                                lexeme.push(dot);
                                while let Some(next_digit) =
                                    characters.next_if(|c| c.is_ascii_digit())
                                {
                                    lexeme.push(next_digit);
                                }
                            }
                            _ => (),
                        }
                    }
                    Ok(TokenKind::Number)
                }
                '"' => {
                    lexeme.push(char);
                    loop {
                        match characters.next() {
                            None => break Err(LexicalError::UnterminatedString { line }),
                            Some(new_char) => {
                                lexeme.push(new_char);
                                if new_char == '\n' {
                                    line += 1;
                                } else if new_char == '"' {
                                    break Ok(TokenKind::String);
                                }
                            }
                        }
                    }
                }
                '!' => {
                    lexeme.push(char);
                    if let Some(c2) = characters.next_if_eq(&'=') {
                        lexeme.push(c2);
                        Ok(TokenKind::BangEqual)
                    } else {
                        Ok(TokenKind::Bang)
                    }
                }
                '=' => {
                    lexeme.push(char);
                    if let Some(c2) = characters.next_if_eq(&'=') {
                        lexeme.push(c2);
                        Ok(TokenKind::EqualEqual)
                    } else {
                        Ok(TokenKind::Equal)
                    }
                }
                '<' => {
                    lexeme.push(char);
                    if let Some(c2) = characters.next_if_eq(&'=') {
                        lexeme.push(c2);
                        Ok(TokenKind::LessEqual)
                    } else {
                        Ok(TokenKind::Less)
                    }
                }
                '>' => {
                    lexeme.push(char);
                    if let Some(c2) = characters.next_if_eq(&'=') {
                        lexeme.push(c2);
                        Ok(TokenKind::GreaterEqual)
                    } else {
                        Ok(TokenKind::Greater)
                    }
                }
                '/' => {
                    if characters.next_if_eq(&'/').is_some() {
                        // Discard comments
                        'comment: while let Some(c) = characters.peek() {
                            if *c == '\n' {
                                // Newlines are handled separately, don't consume them here
                                break 'comment;
                            } else {
                                // Consume the comment itself
                                characters.next();
                            }
                        }
                        continue;
                    } else {
                        lexeme.push(char);
                        Ok(TokenKind::Slash)
                    }
                }
                '\t' | ' ' | '\r' => continue,
                '\n' => {
                    line += 1;
                    continue;
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

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn scanning_single_character_lexemes_works() {
        let input = "(){},.-+;=*!<>/".to_string();
        let tokens = Scanner::scan_tokens(input);

        assert_eq!(
            tokens.0.into_iter().flatten().collect_vec(),
            vec![
                Token::new(TokenKind::LeftParen, Some("(".to_string()), 1),
                Token::new(TokenKind::RightParen, Some(")".to_string()), 1),
                Token::new(TokenKind::LeftBrace, Some("{".to_string()), 1),
                Token::new(TokenKind::RightBrace, Some("}".to_string()), 1),
                Token::new(TokenKind::Comma, Some(",".to_string()), 1),
                Token::new(TokenKind::Dot, Some(".".to_string()), 1),
                Token::new(TokenKind::Minus, Some("-".to_string()), 1),
                Token::new(TokenKind::Plus, Some("+".to_string()), 1),
                Token::new(TokenKind::Semicolon, Some(";".to_string()), 1),
                Token::new(TokenKind::Equal, Some("=".to_string()), 1),
                Token::new(TokenKind::Star, Some("*".to_string()), 1),
                Token::new(TokenKind::Bang, Some("!".to_string()), 1),
                Token::new(TokenKind::Less, Some("<".to_string()), 1),
                Token::new(TokenKind::Greater, Some(">".to_string()), 1),
                Token::new(TokenKind::Slash, Some("/".to_string()), 1),
                Token::new(TokenKind::Eof, None, 1),
            ]
        )
    }

    #[test]
    fn scanning_double_character_lexemes_works() {
        let input = " != <= >= == = =\n!\n=".to_string();
        let tokens = Scanner::scan_tokens(input);

        assert_eq!(
            tokens.0.into_iter().flatten().collect_vec(),
            vec![
                Token::new(TokenKind::BangEqual, Some("!=".to_string()), 1),
                Token::new(TokenKind::LessEqual, Some("<=".to_string()), 1),
                Token::new(TokenKind::GreaterEqual, Some(">=".to_string()), 1),
                Token::new(TokenKind::EqualEqual, Some("==".to_string()), 1),
                Token::new(TokenKind::Equal, Some("=".to_string()), 1),
                Token::new(TokenKind::Equal, Some("=".to_string()), 1),
                Token::new(TokenKind::Bang, Some("!".to_string()), 2),
                Token::new(TokenKind::Equal, Some("=".to_string()), 3),
                Token::new(TokenKind::Eof, None, 3),
            ]
        )
    }

    #[test]
    fn ignoring_whitespaces_works() {
        let input = "(   \r)    {\t     }\n\n\n\n!".to_string();
        let tokens = Scanner::scan_tokens(input);

        assert_eq!(
            tokens.0.into_iter().flatten().collect_vec(),
            vec![
                Token::new(TokenKind::LeftParen, Some("(".to_string()), 1),
                Token::new(TokenKind::RightParen, Some(")".to_string()), 1),
                Token::new(TokenKind::LeftBrace, Some("{".to_string()), 1),
                Token::new(TokenKind::RightBrace, Some("}".to_string()), 1),
                Token::new(TokenKind::Bang, Some("!".to_string()), 5),
                Token::new(TokenKind::Eof, None, 5),
            ]
        )
    }

    #[test]
    fn scanning_basic_valid_strings_works() {
        let input = "\"this is a string\"".to_string();
        let tokens = Scanner::scan_tokens(input);

        assert_eq!(
            tokens.0.into_iter().flatten().collect_vec(),
            vec![
                Token::new(
                    TokenKind::String,
                    Some(r#""this is a string""#.to_string()),
                    1
                ),
                Token::new(TokenKind::Eof, None, 1),
            ]
        )
    }

    #[test]
    fn scanning_multiline_strings_works() {
        let input = "\"this is a string\nacross multiple lines\"".to_string();
        let tokens = Scanner::scan_tokens(input);

        assert_eq!(
            tokens.0.into_iter().flatten().collect_vec(),
            vec![
                Token::new(
                    TokenKind::String,
                    Some("\"this is a string\nacross multiple lines\"".to_string()),
                    2
                ),
                Token::new(TokenKind::Eof, None, 2),
            ]
        )
    }

    #[test]
    fn scanning_unterminated_string_produces_error() {
        let input = "\"this is not a string".to_string();
        let tokens = Scanner::scan_tokens(input);

        assert_eq!(
            tokens.0.into_iter().collect_vec(),
            vec![
                Err(LexicalError::UnterminatedString { line: 1 }),
                Ok(Token::new(TokenKind::Eof, None, 1)),
            ]
        )
    }

    #[test]
    fn scanning_valid_integer_works() {
        let input = "  1 20 4212".to_string();
        let tokens = Scanner::scan_tokens(input);

        assert_eq!(
            tokens.0.into_iter().collect_vec(),
            vec![
                Ok(Token::new(TokenKind::Number, Some("1".to_string()), 1)),
                Ok(Token::new(TokenKind::Number, Some("20".to_string()), 1)),
                Ok(Token::new(TokenKind::Number, Some("4212".to_string()), 1)),
                Ok(Token::new(TokenKind::Eof, None, 1)),
            ]
        )
    }

    #[test]
    fn scanning_valid_fractional_number_works() {
        let input = "  0.0001 2.0 421.2".to_string();
        let tokens = Scanner::scan_tokens(input);

        assert_eq!(
            tokens.0.into_iter().collect_vec(),
            vec![
                Ok(Token::new(TokenKind::Number, Some("0.0001".to_string()), 1)),
                Ok(Token::new(TokenKind::Number, Some("2.0".to_string()), 1)),
                Ok(Token::new(TokenKind::Number, Some("421.2".to_string()), 1)),
                Ok(Token::new(TokenKind::Eof, None, 1)),
            ]
        )
    }

    #[test]
    fn scanning_invalid_fractional_number_works() {
        let input = "  0. 2123. .2 .0012".to_string();
        let tokens = Scanner::scan_tokens(input);

        assert_eq!(
            tokens.0.into_iter().collect_vec(),
            vec![
                Ok(Token::new(TokenKind::Number, Some("0".to_string()), 1)),
                Ok(Token::new(TokenKind::Dot, Some(".".to_string()), 1)),
                Ok(Token::new(TokenKind::Number, Some("2123".to_string()), 1)),
                Ok(Token::new(TokenKind::Dot, Some(".".to_string()), 1)),
                Ok(Token::new(TokenKind::Dot, Some(".".to_string()), 1)),
                Ok(Token::new(TokenKind::Number, Some("2".to_string()), 1)),
                Ok(Token::new(TokenKind::Dot, Some(".".to_string()), 1)),
                Ok(Token::new(TokenKind::Number, Some("0012".to_string()), 1)),
                Ok(Token::new(TokenKind::Eof, None, 1)),
            ]
        )
    }

    #[test]
    fn scanning_multiple_lines_works() {
        let input = "(\n)\n{\n}\n".to_string();
        let tokens = Scanner::scan_tokens(input);

        assert_eq!(
            tokens.0.into_iter().flatten().collect_vec(),
            vec![
                Token::new(TokenKind::LeftParen, Some("(".to_string()), 1),
                Token::new(TokenKind::RightParen, Some(")".to_string()), 2),
                Token::new(TokenKind::LeftBrace, Some("{".to_string()), 3),
                Token::new(TokenKind::RightBrace, Some("}".to_string()), 4),
                Token::new(TokenKind::Eof, None, 5),
            ]
        )
    }

    #[test]
    fn scanning_comments_works() {
        let input = "() // this is a comment\n{} // another one".to_string();
        let tokens = Scanner::scan_tokens(input);

        assert_eq!(
            tokens.0.into_iter().flatten().collect_vec(),
            vec![
                Token::new(TokenKind::LeftParen, Some("(".to_string()), 1),
                Token::new(TokenKind::RightParen, Some(")".to_string()), 1),
                Token::new(TokenKind::LeftBrace, Some("{".to_string()), 2),
                Token::new(TokenKind::RightBrace, Some("}".to_string()), 2),
                Token::new(TokenKind::Eof, None, 2),
            ]
        )
    }
}
