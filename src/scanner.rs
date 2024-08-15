use std::collections::HashMap;

use itertools::peek_nth;
use itertools::PeekNth;
use once_cell::sync::Lazy;

use crate::error::LexicalError;
use crate::token::Token;
use crate::token::TokenKind;
use crate::token::Tokens;

static RESERVED_KEYWORDS: Lazy<HashMap<&str, TokenKind>> = Lazy::new(|| {
    HashMap::from_iter([
        ("and", TokenKind::And { lexeme: "and" }),
        ("class", TokenKind::Class { lexeme: "class" }),
        ("else", TokenKind::Else { lexeme: "else" }),
        ("false", TokenKind::False { lexeme: "false" }),
        ("for", TokenKind::For { lexeme: "for" }),
        ("fun", TokenKind::Fun { lexeme: "fun" }),
        ("if", TokenKind::If { lexeme: "if" }),
        ("nil", TokenKind::Nil { lexeme: "nil" }),
        ("or", TokenKind::Or { lexeme: "or" }),
        ("print", TokenKind::Print { lexeme: "print" }),
        ("return", TokenKind::Return { lexeme: "return" }),
        ("super", TokenKind::Super { lexeme: "super" }),
        ("this", TokenKind::This { lexeme: "this" }),
        ("true", TokenKind::True { lexeme: "true" }),
        ("var", TokenKind::Var { lexeme: "var" }),
        ("while", TokenKind::While { lexeme: "while" }),
    ])
});

#[derive(Debug)]
pub(crate) struct Lexer;

impl Lexer {
    pub(crate) fn lex(source: &str) -> Tokens {
        let mut tokens = vec![];
        let mut lexeme_start = 0;
        let mut line = 1;

        // TODO: Iterates over Unicode Scalar Values instead of grapheme clusters.
        let mut characters = indexed_iterator(peek_nth(source.chars()));
        while let Some(char) = characters.next() {
            let token_kind = match char {
                '(' => Ok(TokenKind::LeftParen {
                    lexeme: &source[lexeme_start..characters.current_idx()],
                }),
                ')' => Ok(TokenKind::RightParen {
                    lexeme: &source[lexeme_start..characters.current_idx()],
                }),
                '{' => Ok(TokenKind::LeftBrace {
                    lexeme: &source[lexeme_start..characters.current_idx()],
                }),
                '}' => Ok(TokenKind::RightBrace {
                    lexeme: &source[lexeme_start..characters.current_idx()],
                }),
                ',' => Ok(TokenKind::Comma {
                    lexeme: &source[lexeme_start..characters.current_idx()],
                }),
                '.' => Ok(TokenKind::Dot {
                    lexeme: &source[lexeme_start..characters.current_idx()],
                }),
                '-' => Ok(TokenKind::Minus {
                    lexeme: &source[lexeme_start..characters.current_idx()],
                }),
                '+' => Ok(TokenKind::Plus {
                    lexeme: &source[lexeme_start..characters.current_idx()],
                }),
                ';' => Ok(TokenKind::Semicolon {
                    lexeme: &source[lexeme_start..characters.current_idx()],
                }),
                '*' => Ok(TokenKind::Star {
                    lexeme: &source[lexeme_start..characters.current_idx()],
                }),
                c if c.is_ascii_alphabetic() || c == '_' => {
                    while characters
                        .next_if(|c| c.is_ascii_alphanumeric() || *c == '_')
                        .is_some()
                    {}

                    if let Some(token_kind) =
                        RESERVED_KEYWORDS.get(&source[lexeme_start..characters.current_idx()])
                    {
                        Ok(*token_kind)
                    } else {
                        Ok(TokenKind::Identifier {
                            lexeme: &source[lexeme_start..characters.current_idx()],
                        })
                    }
                }
                c if c.is_ascii_digit() => {
                    while characters.next_if(|c| c.is_ascii_digit()).is_some() {}

                    if let Some('.') = characters.peek() {
                        match characters.peek_nth(1) {
                            Some(c) if c.is_ascii_digit() => {
                                // Consume the '.'
                                characters.next();
                                while characters.next_if(|c| c.is_ascii_digit()).is_some() {}
                            }
                            _ => (),
                        }
                    }
                    match &source[lexeme_start..characters.current_idx()].parse::<f64>() {
                        Ok(v) => Ok(TokenKind::Number { lexeme: *v }),
                        Err(_) => Err(LexicalError::NaN { line }),
                    }
                }
                '"' => loop {
                    match characters.next() {
                        None => break Err(LexicalError::UnterminatedString { line }),
                        Some(new_char) => {
                            if new_char == '\n' {
                                line += 1;
                            } else if new_char == '"' {
                                break Ok(TokenKind::String {
                                    lexeme: &source[lexeme_start..characters.current_idx()],
                                });
                            }
                        }
                    }
                },
                '!' => {
                    if characters.next_if_eq(&'=').is_some() {
                        Ok(TokenKind::BangEqual {
                            lexeme: &source[lexeme_start..characters.current_idx()],
                        })
                    } else {
                        Ok(TokenKind::Bang {
                            lexeme: &source[lexeme_start..characters.current_idx()],
                        })
                    }
                }
                '=' => {
                    if characters.next_if_eq(&'=').is_some() {
                        Ok(TokenKind::EqualEqual {
                            lexeme: &source[lexeme_start..characters.current_idx()],
                        })
                    } else {
                        Ok(TokenKind::Equal {
                            lexeme: &source[lexeme_start..characters.current_idx()],
                        })
                    }
                }
                '<' => {
                    if characters.next_if_eq(&'=').is_some() {
                        Ok(TokenKind::LessEqual {
                            lexeme: &source[lexeme_start..characters.current_idx()],
                        })
                    } else {
                        Ok(TokenKind::Less {
                            lexeme: &source[lexeme_start..characters.current_idx()],
                        })
                    }
                }
                '>' => {
                    if characters.next_if_eq(&'=').is_some() {
                        Ok(TokenKind::GreaterEqual {
                            lexeme: &source[lexeme_start..characters.current_idx()],
                        })
                    } else {
                        Ok(TokenKind::Greater {
                            lexeme: &source[lexeme_start..characters.current_idx()],
                        })
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
                        lexeme_start = characters.current_idx();
                        continue;
                    } else {
                        Ok(TokenKind::Slash {
                            lexeme: &source[lexeme_start..characters.current_idx()],
                        })
                    }
                }
                '\t' | ' ' | '\r' => {
                    lexeme_start = characters.current_idx();
                    continue;
                }
                '\n' => {
                    line += 1;
                    lexeme_start = characters.current_idx();
                    continue;
                }
                _ => Err(LexicalError::UnexpectedCharacter { char, line }),
            };

            let lex_result = match token_kind {
                Ok(token_kind) => {
                    lexeme_start = characters.current_idx();
                    Ok(Token::new(token_kind, line))
                }
                Err(lexical_error) => Err(lexical_error),
            };
            tokens.push(lex_result);
        }

        tokens.push(Ok(Token::new(TokenKind::Eof, line)));

        Tokens::new(tokens)
    }
}

pub(crate) fn indexed_iterator<I>(iterable: PeekNth<I>) -> IndexedPeekNth<I>
where
    I: Iterator,
{
    IndexedPeekNth {
        current_idx: 0,
        iter: iterable,
    }
}

pub(crate) struct IndexedPeekNth<I: Iterator> {
    current_idx: usize,
    iter: PeekNth<I>,
}

impl<I> IndexedPeekNth<I>
where
    I: Iterator,
{
    pub(crate) fn current_idx(&self) -> usize {
        self.current_idx
    }

    fn next_if(
        &mut self,
        func: impl FnOnce(&I::Item) -> bool,
    ) -> Option<I::Item> {
        let item = self.iter.next_if(func);
        if item.is_some() {
            self.current_idx += 1;
        }
        item
    }

    fn next_if_eq<T>(
        &mut self,
        expected: &T,
    ) -> Option<I::Item>
    where
        T: ?Sized,
        I::Item: PartialEq<T>,
    {
        let item = self.iter.next_if_eq(expected);
        if item.is_some() {
            self.current_idx += 1;
        }
        item
    }

    fn peek(&mut self) -> Option<&I::Item> {
        self.iter.peek()
    }

    fn peek_nth(
        &mut self,
        n: usize,
    ) -> Option<&I::Item> {
        self.iter.peek_nth(n)
    }
}

impl<I, U> Iterator for IndexedPeekNth<I>
where
    I: Iterator<Item = U>,
{
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
        let tokens = Lexer::lex(input);

        assert_eq!(
            tokens.into_iter().flatten().collect_vec(),
            vec![
                Token::new(TokenKind::LeftParen { lexeme: "(" }, 1),
                Token::new(TokenKind::RightParen { lexeme: ")" }, 1),
                Token::new(TokenKind::LeftBrace { lexeme: "{" }, 1),
                Token::new(TokenKind::RightBrace { lexeme: "}" }, 1),
                Token::new(TokenKind::Comma { lexeme: "," }, 1),
                Token::new(TokenKind::Dot { lexeme: "." }, 1),
                Token::new(TokenKind::Minus { lexeme: "-" }, 1),
                Token::new(TokenKind::Plus { lexeme: "+" }, 1),
                Token::new(TokenKind::Semicolon { lexeme: ";" }, 1),
                Token::new(TokenKind::Equal { lexeme: "=" }, 1),
                Token::new(TokenKind::Star { lexeme: "*" }, 1),
                Token::new(TokenKind::Bang { lexeme: "!" }, 1),
                Token::new(TokenKind::Less { lexeme: "<" }, 1),
                Token::new(TokenKind::Greater { lexeme: ">" }, 1),
                Token::new(TokenKind::Slash { lexeme: "/" }, 1),
                Token::new(TokenKind::Eof, 1),
            ]
        )
    }

    #[test]
    fn scanning_double_character_lexemes_works() {
        let input = " != <= >= == = =\n!\n=";
        let tokens = Lexer::lex(input);

        assert_eq!(
            tokens.into_iter().flatten().collect_vec(),
            vec![
                Token::new(TokenKind::BangEqual { lexeme: "!=" }, 1),
                Token::new(TokenKind::LessEqual { lexeme: "<=" }, 1),
                Token::new(TokenKind::GreaterEqual { lexeme: ">=" }, 1),
                Token::new(TokenKind::EqualEqual { lexeme: "==" }, 1),
                Token::new(TokenKind::Equal { lexeme: "=" }, 1),
                Token::new(TokenKind::Equal { lexeme: "=" }, 1),
                Token::new(TokenKind::Bang { lexeme: "!" }, 2),
                Token::new(TokenKind::Equal { lexeme: "=" }, 3),
                Token::new(TokenKind::Eof, 3),
            ]
        )
    }

    #[test]
    fn ignoring_whitespaces_works() {
        let input = "(   \r)    {\t     }\n\n\n\n!";
        let tokens = Lexer::lex(input);

        assert_eq!(
            tokens.into_iter().flatten().collect_vec(),
            vec![
                Token::new(TokenKind::LeftParen { lexeme: "(" }, 1),
                Token::new(TokenKind::RightParen { lexeme: ")" }, 1),
                Token::new(TokenKind::LeftBrace { lexeme: "{" }, 1),
                Token::new(TokenKind::RightBrace { lexeme: "}" }, 1),
                Token::new(TokenKind::Bang { lexeme: "!" }, 5),
                Token::new(TokenKind::Eof, 5),
            ]
        )
    }

    #[test]
    fn scanning_basic_valid_strings_works() {
        let input = "\"this is a string\"";
        let tokens = Lexer::lex(input);

        assert_eq!(
            tokens.into_iter().flatten().collect_vec(),
            vec![
                Token::new(
                    TokenKind::String {
                        lexeme: r#""this is a string""#
                    },
                    1
                ),
                Token::new(TokenKind::Eof, 1),
            ]
        )
    }

    #[test]
    fn scanning_multiline_strings_works() {
        let input = "\"this is a string\nacross multiple lines\"";
        let tokens = Lexer::lex(input);

        assert_eq!(
            tokens.into_iter().flatten().collect_vec(),
            vec![
                Token::new(
                    TokenKind::String {
                        lexeme: "\"this is a string\nacross multiple lines\""
                    },
                    2
                ),
                Token::new(TokenKind::Eof, 2),
            ]
        )
    }

    #[test]
    fn scanning_unterminated_string_produces_error() {
        let input = "\"this is not a string";
        let tokens = Lexer::lex(input);

        assert_eq!(
            tokens.into_iter().collect_vec(),
            vec![
                Err(LexicalError::UnterminatedString { line: 1 }),
                Ok(Token::new(TokenKind::Eof, 1)),
            ]
        )
    }

    #[test]
    fn scanning_valid_integer_works() {
        let input = "  1 20 4212";
        let tokens = Lexer::lex(input);

        assert_eq!(
            tokens.into_iter().collect_vec(),
            vec![
                Ok(Token::new(TokenKind::Number { lexeme: 1.0 }, 1)),
                Ok(Token::new(TokenKind::Number { lexeme: 20.0 }, 1)),
                Ok(Token::new(TokenKind::Number { lexeme: 4212.0 }, 1)),
                Ok(Token::new(TokenKind::Eof, 1)),
            ]
        )
    }

    #[test]
    fn scanning_valid_fractional_number_works() {
        let input = "  0.0001 2.0 421.2";
        let tokens = Lexer::lex(input);

        assert_eq!(
            tokens.into_iter().collect_vec(),
            vec![
                Ok(Token::new(TokenKind::Number { lexeme: 0.0001 }, 1)),
                Ok(Token::new(TokenKind::Number { lexeme: 2.0 }, 1)),
                Ok(Token::new(TokenKind::Number { lexeme: 421.2 }, 1)),
                Ok(Token::new(TokenKind::Eof, 1)),
            ]
        )
    }

    #[test]
    fn scanning_invalid_fractional_number_works() {
        let input = "  0. 2123. .2 .0012";
        let tokens = Lexer::lex(input);

        assert_eq!(
            tokens.into_iter().collect_vec(),
            vec![
                Ok(Token::new(TokenKind::Number { lexeme: 0.0 }, 1)),
                Ok(Token::new(TokenKind::Dot { lexeme: "." }, 1)),
                Ok(Token::new(TokenKind::Number { lexeme: 2123.0 }, 1)),
                Ok(Token::new(TokenKind::Dot { lexeme: "." }, 1)),
                Ok(Token::new(TokenKind::Dot { lexeme: "." }, 1)),
                Ok(Token::new(TokenKind::Number { lexeme: 2.0 }, 1)),
                Ok(Token::new(TokenKind::Dot { lexeme: "." }, 1)),
                Ok(Token::new(TokenKind::Number { lexeme: 12.0 }, 1)),
                Ok(Token::new(TokenKind::Eof, 1)),
            ]
        )
    }

    #[test]
    fn scanning_identifiers_works() {
        let input = "some_identifier _anotherOne als0 c1 0no 001_no ";
        let tokens = Lexer::lex(input);

        assert_eq!(
            tokens.into_iter().flatten().collect_vec(),
            vec![
                Token::new(
                    TokenKind::Identifier {
                        lexeme: "some_identifier"
                    },
                    1
                ),
                Token::new(
                    TokenKind::Identifier {
                        lexeme: "_anotherOne"
                    },
                    1
                ),
                Token::new(TokenKind::Identifier { lexeme: "als0" }, 1),
                Token::new(TokenKind::Identifier { lexeme: "c1" }, 1),
                Token::new(TokenKind::Number { lexeme: 0.0 }, 1),
                Token::new(TokenKind::Identifier { lexeme: "no" }, 1),
                Token::new(TokenKind::Number { lexeme: 1.0 }, 1),
                Token::new(TokenKind::Identifier { lexeme: "_no" }, 1),
                Token::new(TokenKind::Eof, 1),
            ]
        )
    }

    #[test]
    fn scanning_reserved_words_works() {
        let input = "and class else false for fun if nil or print return super this true var while";
        let tokens = Lexer::lex(input);

        assert_eq!(
            tokens.into_iter().flatten().collect_vec(),
            vec![
                Token::new(TokenKind::And { lexeme: "and" }, 1),
                Token::new(TokenKind::Class { lexeme: "class" }, 1),
                Token::new(TokenKind::Else { lexeme: "else" }, 1),
                Token::new(TokenKind::False { lexeme: "false" }, 1),
                Token::new(TokenKind::For { lexeme: "for" }, 1),
                Token::new(TokenKind::Fun { lexeme: "fun" }, 1),
                Token::new(TokenKind::If { lexeme: "if" }, 1),
                Token::new(TokenKind::Nil { lexeme: "nil" }, 1),
                Token::new(TokenKind::Or { lexeme: "or" }, 1),
                Token::new(TokenKind::Print { lexeme: "print" }, 1),
                Token::new(TokenKind::Return { lexeme: "return" }, 1),
                Token::new(TokenKind::Super { lexeme: "super" }, 1),
                Token::new(TokenKind::This { lexeme: "this" }, 1),
                Token::new(TokenKind::True { lexeme: "true" }, 1),
                Token::new(TokenKind::Var { lexeme: "var" }, 1),
                Token::new(TokenKind::While { lexeme: "while" }, 1),
                Token::new(TokenKind::Eof, 1),
            ]
        )
    }

    #[test]
    fn scanning_multiple_lines_works() {
        let input = "(\n)\n{\n}\n";
        let tokens = Lexer::lex(input);

        assert_eq!(
            tokens.into_iter().flatten().collect_vec(),
            vec![
                Token::new(TokenKind::LeftParen { lexeme: "(" }, 1),
                Token::new(TokenKind::RightParen { lexeme: ")" }, 2),
                Token::new(TokenKind::LeftBrace { lexeme: "{" }, 3),
                Token::new(TokenKind::RightBrace { lexeme: "}" }, 4),
                Token::new(TokenKind::Eof, 5),
            ]
        )
    }

    #[test]
    fn scanning_comments_works() {
        let input = "() // this is a comment\n{} // another one";
        let tokens = Lexer::lex(input);

        assert_eq!(
            tokens.into_iter().flatten().collect_vec(),
            vec![
                Token::new(TokenKind::LeftParen { lexeme: "(" }, 1),
                Token::new(TokenKind::RightParen { lexeme: ")" }, 1),
                Token::new(TokenKind::LeftBrace { lexeme: "{" }, 2),
                Token::new(TokenKind::RightBrace { lexeme: "}" }, 2),
                Token::new(TokenKind::Eof, 2),
            ]
        )
    }
}
