use itertools::peek_nth;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::iter::Fuse;

use crate::error::LexicalError;
use crate::token::Token;
use crate::token::TokenKind;
use crate::token::Tokens;

static RESERVED_KEYWORDS: Lazy<HashMap<&str, TokenKind>> = Lazy::new(|| {
    HashMap::from_iter([
        ("and", TokenKind::And),
        ("class", TokenKind::Class),
        ("else", TokenKind::Else),
        ("false", TokenKind::False),
        ("for", TokenKind::For),
        ("fun", TokenKind::Fun),
        ("if", TokenKind::If),
        ("nil", TokenKind::Nil),
        ("or", TokenKind::Or),
        ("print", TokenKind::Print),
        ("return", TokenKind::Return),
        ("super", TokenKind::Super),
        ("this", TokenKind::This),
        ("true", TokenKind::True),
        ("var", TokenKind::Var),
        ("while", TokenKind::While),
    ])
});

#[derive(Debug)]
pub(crate) struct Scanner;

impl Scanner {
    pub(crate) fn scan_tokens(source: &str) -> Tokens {
        let mut tokens = vec![];
        let mut lexeme_start = 0;
        let mut lexeme_end = lexeme_start;
        let mut line = 1;

        // TODO: Iterates over Unicode Scalar Values instead of grapheme clusters.
        // TODO the indexed iterator doesn't do anything
        let mut characters = peek_nth(indexed_iterator(source.chars()));
        while let Some(char) = characters.next() {
            lexeme_end += 1;
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
                c if c.is_ascii_alphabetic() || c == '_' => {
                    while characters
                        .next_if(|c| c.is_ascii_alphanumeric() || *c == '_')
                        .is_some()
                    {
                        lexeme_end += 1;
                    }

                    if let Some(token_kind) =
                        RESERVED_KEYWORDS.get(&source[lexeme_start..lexeme_end])
                    {
                        Ok(*token_kind)
                    } else {
                        Ok(TokenKind::Identifier)
                    }
                }
                c if c.is_ascii_digit() => {
                    while characters.next_if(|c| c.is_ascii_digit()).is_some() {
                        lexeme_end += 1;
                    }

                    if let Some('.') = characters.peek() {
                        match characters.peek_nth(1) {
                            Some(c) if c.is_ascii_digit() => {
                                // Consume the '.'
                                characters.next();
                                lexeme_end += 1;
                                while characters.next_if(|c| c.is_ascii_digit()).is_some() {
                                    lexeme_end += 1;
                                }
                            }
                            _ => (),
                        }
                    }
                    Ok(TokenKind::Number)
                }
                '"' => loop {
                    match characters.next() {
                        None => break Err(LexicalError::UnterminatedString { line }),
                        Some(new_char) => {
                            lexeme_end += 1;
                            if new_char == '\n' {
                                line += 1;
                            } else if new_char == '"' {
                                break Ok(TokenKind::String);
                            }
                        }
                    }
                },
                '!' => {
                    if characters.next_if_eq(&'=').is_some() {
                        lexeme_end += 1;
                        Ok(TokenKind::BangEqual)
                    } else {
                        Ok(TokenKind::Bang)
                    }
                }
                '=' => {
                    if characters.next_if_eq(&'=').is_some() {
                        lexeme_end += 1;
                        Ok(TokenKind::EqualEqual)
                    } else {
                        Ok(TokenKind::Equal)
                    }
                }
                '<' => {
                    if characters.next_if_eq(&'=').is_some() {
                        lexeme_end += 1;
                        Ok(TokenKind::LessEqual)
                    } else {
                        Ok(TokenKind::Less)
                    }
                }
                '>' => {
                    if characters.next_if_eq(&'=').is_some() {
                        lexeme_end += 1;
                        Ok(TokenKind::GreaterEqual)
                    } else {
                        Ok(TokenKind::Greater)
                    }
                }
                '/' => {
                    if characters.next_if_eq(&'/').is_some() {
                        // Discard comments
                        lexeme_end += 1;
                        'comment: while let Some(c) = characters.peek() {
                            if *c == '\n' {
                                // Newlines are handled separately, don't consume them here
                                break 'comment;
                            } else {
                                // Consume the comment itself
                                lexeme_end += 1;
                                characters.next();
                            }
                        }
                        lexeme_start = lexeme_end;
                        continue;
                    } else {
                        Ok(TokenKind::Slash)
                    }
                }
                '\t' | ' ' | '\r' => {
                    lexeme_start = lexeme_end;
                    continue;
                }
                '\n' => {
                    line += 1;
                    lexeme_start = lexeme_end;
                    continue;
                }
                _ => Err(LexicalError::UnexpectedCharacter { char, line }),
            };

            let lex_result = match token_kind {
                Ok(token_kind) => {
                    let this_lexeme = &source[lexeme_start..lexeme_end];
                    lexeme_start = lexeme_end;
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


pub(crate) fn indexed_iterator<I>(iterable: I) -> IndexedIterator<I::IntoIter> where I: IntoIterator {
    IndexedIterator {
        current_idx: 0,
        iter: iterable.into_iter().fuse(),
    }
}

pub(crate) struct IndexedIterator<T> {
    current_idx: usize,
    iter: Fuse<T>,
}

impl<I> IndexedIterator<I> {
    pub(crate) fn current_idx(&self) -> usize {
        self.current_idx
    }
}

impl<T, U> Iterator for IndexedIterator<T> where T: Iterator<Item=U> {
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_idx += 1;
        self.iter.next()
    }
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn scanning_single_character_lexemes_works() {
        let input = "(){},.-+;=*!<>/";
        let tokens = Scanner::scan_tokens(input);

        assert_eq!(
            tokens.0.into_iter().flatten().collect_vec(),
            vec![
                Token::new(TokenKind::LeftParen, Some("("), 1),
                Token::new(TokenKind::RightParen, Some(")"), 1),
                Token::new(TokenKind::LeftBrace, Some("{"), 1),
                Token::new(TokenKind::RightBrace, Some("}"), 1),
                Token::new(TokenKind::Comma, Some(","), 1),
                Token::new(TokenKind::Dot, Some("."), 1),
                Token::new(TokenKind::Minus, Some("-"), 1),
                Token::new(TokenKind::Plus, Some("+"), 1),
                Token::new(TokenKind::Semicolon, Some(";"), 1),
                Token::new(TokenKind::Equal, Some("="), 1),
                Token::new(TokenKind::Star, Some("*"), 1),
                Token::new(TokenKind::Bang, Some("!"), 1),
                Token::new(TokenKind::Less, Some("<"), 1),
                Token::new(TokenKind::Greater, Some(">"), 1),
                Token::new(TokenKind::Slash, Some("/"), 1),
                Token::new(TokenKind::Eof, None, 1),
            ]
        )
    }

    #[test]
    fn scanning_double_character_lexemes_works() {
        let input = " != <= >= == = =\n!\n=";
        let tokens = Scanner::scan_tokens(input);

        assert_eq!(
            tokens.0.into_iter().flatten().collect_vec(),
            vec![
                Token::new(TokenKind::BangEqual, Some("!="), 1),
                Token::new(TokenKind::LessEqual, Some("<="), 1),
                Token::new(TokenKind::GreaterEqual, Some(">="), 1),
                Token::new(TokenKind::EqualEqual, Some("=="), 1),
                Token::new(TokenKind::Equal, Some("="), 1),
                Token::new(TokenKind::Equal, Some("="), 1),
                Token::new(TokenKind::Bang, Some("!"), 2),
                Token::new(TokenKind::Equal, Some("="), 3),
                Token::new(TokenKind::Eof, None, 3),
            ]
        )
    }

    #[test]
    fn ignoring_whitespaces_works() {
        let input = "(   \r)    {\t     }\n\n\n\n!";
        let tokens = Scanner::scan_tokens(input);

        assert_eq!(
            tokens.0.into_iter().flatten().collect_vec(),
            vec![
                Token::new(TokenKind::LeftParen, Some("("), 1),
                Token::new(TokenKind::RightParen, Some(")"), 1),
                Token::new(TokenKind::LeftBrace, Some("{"), 1),
                Token::new(TokenKind::RightBrace, Some("}"), 1),
                Token::new(TokenKind::Bang, Some("!"), 5),
                Token::new(TokenKind::Eof, None, 5),
            ]
        )
    }

    #[test]
    fn scanning_basic_valid_strings_works() {
        let input = "\"this is a string\"";
        let tokens = Scanner::scan_tokens(input);

        assert_eq!(
            tokens.0.into_iter().flatten().collect_vec(),
            vec![
                Token::new(TokenKind::String, Some(r#""this is a string""#), 1),
                Token::new(TokenKind::Eof, None, 1),
            ]
        )
    }

    #[test]
    fn scanning_multiline_strings_works() {
        let input = "\"this is a string\nacross multiple lines\"";
        let tokens = Scanner::scan_tokens(input);

        assert_eq!(
            tokens.0.into_iter().flatten().collect_vec(),
            vec![
                Token::new(
                    TokenKind::String,
                    Some("\"this is a string\nacross multiple lines\""),
                    2
                ),
                Token::new(TokenKind::Eof, None, 2),
            ]
        )
    }

    #[test]
    fn scanning_unterminated_string_produces_error() {
        let input = "\"this is not a string";
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
        let input = "  1 20 4212";
        let tokens = Scanner::scan_tokens(input);

        assert_eq!(
            tokens.0.into_iter().collect_vec(),
            vec![
                Ok(Token::new(TokenKind::Number, Some("1"), 1)),
                Ok(Token::new(TokenKind::Number, Some("20"), 1)),
                Ok(Token::new(TokenKind::Number, Some("4212"), 1)),
                Ok(Token::new(TokenKind::Eof, None, 1)),
            ]
        )
    }

    #[test]
    fn scanning_valid_fractional_number_works() {
        let input = "  0.0001 2.0 421.2";
        let tokens = Scanner::scan_tokens(input);

        assert_eq!(
            tokens.0.into_iter().collect_vec(),
            vec![
                Ok(Token::new(TokenKind::Number, Some("0.0001"), 1)),
                Ok(Token::new(TokenKind::Number, Some("2.0"), 1)),
                Ok(Token::new(TokenKind::Number, Some("421.2"), 1)),
                Ok(Token::new(TokenKind::Eof, None, 1)),
            ]
        )
    }

    #[test]
    fn scanning_invalid_fractional_number_works() {
        let input = "  0. 2123. .2 .0012";
        let tokens = Scanner::scan_tokens(input);

        assert_eq!(
            tokens.0.into_iter().collect_vec(),
            vec![
                Ok(Token::new(TokenKind::Number, Some("0"), 1)),
                Ok(Token::new(TokenKind::Dot, Some("."), 1)),
                Ok(Token::new(TokenKind::Number, Some("2123"), 1)),
                Ok(Token::new(TokenKind::Dot, Some("."), 1)),
                Ok(Token::new(TokenKind::Dot, Some("."), 1)),
                Ok(Token::new(TokenKind::Number, Some("2"), 1)),
                Ok(Token::new(TokenKind::Dot, Some("."), 1)),
                Ok(Token::new(TokenKind::Number, Some("0012"), 1)),
                Ok(Token::new(TokenKind::Eof, None, 1)),
            ]
        )
    }

    #[test]
    fn scanning_identifiers_works() {
        let input = "some_identifier _anotherOne als0 c1 0no 001_no ";
        let tokens = Scanner::scan_tokens(input);

        assert_eq!(
            tokens.0.into_iter().flatten().collect_vec(),
            vec![
                Token::new(TokenKind::Identifier, Some("some_identifier"), 1),
                Token::new(TokenKind::Identifier, Some("_anotherOne"), 1),
                Token::new(TokenKind::Identifier, Some("als0"), 1),
                Token::new(TokenKind::Identifier, Some("c1"), 1),
                Token::new(TokenKind::Number, Some("0"), 1),
                Token::new(TokenKind::Identifier, Some("no"), 1),
                Token::new(TokenKind::Number, Some("001"), 1),
                Token::new(TokenKind::Identifier, Some("_no"), 1),
                Token::new(TokenKind::Eof, None, 1),
            ]
        )
    }

    #[test]
    fn scanning_reserved_words_works() {
        let input = "and class else false for fun if nil or print return super this true var while";
        let tokens = Scanner::scan_tokens(input);

        assert_eq!(
            tokens.0.into_iter().flatten().collect_vec(),
            vec![
                Token::new(TokenKind::And, Some("and"), 1),
                Token::new(TokenKind::Class, Some("class"), 1),
                Token::new(TokenKind::Else, Some("else"), 1),
                Token::new(TokenKind::False, Some("false"), 1),
                Token::new(TokenKind::For, Some("for"), 1),
                Token::new(TokenKind::Fun, Some("fun"), 1),
                Token::new(TokenKind::If, Some("if"), 1),
                Token::new(TokenKind::Nil, Some("nil"), 1),
                Token::new(TokenKind::Or, Some("or"), 1),
                Token::new(TokenKind::Print, Some("print"), 1),
                Token::new(TokenKind::Return, Some("return"), 1),
                Token::new(TokenKind::Super, Some("super"), 1),
                Token::new(TokenKind::This, Some("this"), 1),
                Token::new(TokenKind::True, Some("true"), 1),
                Token::new(TokenKind::Var, Some("var"), 1),
                Token::new(TokenKind::While, Some("while"), 1),
                Token::new(TokenKind::Eof, None, 1),
            ]
        )
    }

    #[test]
    fn scanning_multiple_lines_works() {
        let input = "(\n)\n{\n}\n";
        let tokens = Scanner::scan_tokens(input);

        assert_eq!(
            tokens.0.into_iter().flatten().collect_vec(),
            vec![
                Token::new(TokenKind::LeftParen, Some("("), 1),
                Token::new(TokenKind::RightParen, Some(")"), 2),
                Token::new(TokenKind::LeftBrace, Some("{"), 3),
                Token::new(TokenKind::RightBrace, Some("}"), 4),
                Token::new(TokenKind::Eof, None, 5),
            ]
        )
    }

    #[test]
    fn scanning_comments_works() {
        let input = "() // this is a comment\n{} // another one";
        let tokens = Scanner::scan_tokens(input);

        assert_eq!(
            tokens.0.into_iter().flatten().collect_vec(),
            vec![
                Token::new(TokenKind::LeftParen, Some("("), 1),
                Token::new(TokenKind::RightParen, Some(")"), 1),
                Token::new(TokenKind::LeftBrace, Some("{"), 2),
                Token::new(TokenKind::RightBrace, Some("}"), 2),
                Token::new(TokenKind::Eof, None, 2),
            ]
        )
    }
}
