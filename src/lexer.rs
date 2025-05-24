use std::rc::Rc;

use itertools::peek_nth;
use itertools::PeekNth;

use crate::error::LexicalError;
use crate::token::AndToken;
use crate::token::BangEqualToken;
use crate::token::BangToken;
use crate::token::ClassToken;
use crate::token::CommaToken;
use crate::token::DotToken;
use crate::token::ElseToken;
use crate::token::EqualEqualToken;
use crate::token::EqualToken;
use crate::token::FalseToken;
use crate::token::ForToken;
use crate::token::FunToken;
use crate::token::GreaterEqualToken;
use crate::token::GreaterToken;
use crate::token::IdentifierToken;
use crate::token::IfToken;
use crate::token::LeftBraceToken;
use crate::token::LeftParenToken;
use crate::token::LessEqualToken;
use crate::token::LessToken;
use crate::token::MinusToken;
use crate::token::NilToken;
use crate::token::NumberToken;
use crate::token::OrToken;
use crate::token::PlusToken;
use crate::token::PrintToken;
use crate::token::ReturnToken;
use crate::token::RightBraceToken;
use crate::token::RightParenToken;
use crate::token::SemicolonToken;
use crate::token::SlashToken;
use crate::token::StarToken;
use crate::token::StringToken;
use crate::token::SuperToken;
use crate::token::ThisToken;
use crate::token::Token;
use crate::token::TokenKind;
use crate::token::Tokens;
use crate::token::TrueToken;
use crate::token::VarToken;
use crate::token::WhileToken;

#[derive(Debug)]
pub(crate) struct Lexer;

impl Lexer {
    pub(crate) fn lex(source: Rc<str>) -> Tokens {
        let mut tokens = vec![];
        let mut lexeme_start = 0;
        let mut line = 1;

        // TODO: Iterates over Unicode Scalar Values instead of grapheme clusters.
        let mut characters = indexed_iterator(peek_nth(source.chars()));
        while let Some(char) = characters.next() {
            let token_kind = match char {
                '(' => {
                    Ok(LeftParenToken::new(&source[lexeme_start..characters.current_idx()]).into())
                }
                ')' => Ok(
                    RightParenToken::new(&source[lexeme_start..characters.current_idx()]).into(),
                ),
                '{' => {
                    Ok(LeftBraceToken::new(&source[lexeme_start..characters.current_idx()]).into())
                }
                '}' => Ok(
                    RightBraceToken::new(&source[lexeme_start..characters.current_idx()]).into(),
                ),
                ',' => Ok(CommaToken::new(&source[lexeme_start..characters.current_idx()]).into()),
                '.' => Ok(DotToken::new(&source[lexeme_start..characters.current_idx()]).into()),
                '-' => Ok(MinusToken::new(&source[lexeme_start..characters.current_idx()]).into()),
                '+' => Ok(PlusToken::new(&source[lexeme_start..characters.current_idx()]).into()),
                ';' => {
                    Ok(SemicolonToken::new(&source[lexeme_start..characters.current_idx()]).into())
                }
                '*' => Ok(StarToken::new(&source[lexeme_start..characters.current_idx()]).into()),
                c if c.is_ascii_alphabetic() || c == '_' => {
                    while characters
                        .next_if(|c| c.is_ascii_alphanumeric() || *c == '_')
                        .is_some()
                    {}

                    let val = &source[lexeme_start..characters.current_idx()];
                    if let Some(token_kind) = get_reserved_keyword_token(val) {
                        Ok(token_kind)
                    } else {
                        Ok(
                            IdentifierToken::new(&source[lexeme_start..characters.current_idx()])
                                .into(),
                        )
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
                        Ok(v) => Ok(NumberToken::new(*v).into()),
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
                                break Ok(StringToken::new(
                                    &source[lexeme_start..characters.current_idx()],
                                )
                                .into());
                            }
                        }
                    }
                },
                '!' => {
                    if characters.next_if_eq(&'=').is_some() {
                        Ok(
                            BangEqualToken::new(&source[lexeme_start..characters.current_idx()])
                                .into(),
                        )
                    } else {
                        Ok(BangToken::new(&source[lexeme_start..characters.current_idx()]).into())
                    }
                }
                '=' => {
                    if characters.next_if_eq(&'=').is_some() {
                        Ok(
                            EqualEqualToken::new(&source[lexeme_start..characters.current_idx()])
                                .into(),
                        )
                    } else {
                        Ok(EqualToken::new(&source[lexeme_start..characters.current_idx()]).into())
                    }
                }
                '<' => {
                    if characters.next_if_eq(&'=').is_some() {
                        Ok(
                            LessEqualToken::new(&source[lexeme_start..characters.current_idx()])
                                .into(),
                        )
                    } else {
                        Ok(LessToken::new(&source[lexeme_start..characters.current_idx()]).into())
                    }
                }
                '>' => {
                    if characters.next_if_eq(&'=').is_some() {
                        Ok(
                            GreaterEqualToken::new(&source[lexeme_start..characters.current_idx()])
                                .into(),
                        )
                    } else {
                        Ok(
                            GreaterToken::new(&source[lexeme_start..characters.current_idx()])
                                .into(),
                        )
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
                        Ok(SlashToken::new(&source[lexeme_start..characters.current_idx()]).into())
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

fn get_reserved_keyword_token(val: &str) -> Option<TokenKind> {
    match val {
        "and" => Some(AndToken::new("and").into()),
        "class" => Some(ClassToken::new("class").into()),
        "else" => Some(ElseToken::new("else").into()),
        "false" => Some(FalseToken::new("false").into()),
        "for" => Some(ForToken::new("for").into()),
        "fun" => Some(FunToken::new("fun").into()),
        "if" => Some(IfToken::new("if").into()),
        "nil" => Some(NilToken::new("nil").into()),
        "or" => Some(OrToken::new("or").into()),
        "print" => Some(PrintToken::new("print").into()),
        "return" => Some(ReturnToken::new("return").into()),
        "super" => Some(SuperToken::new("super").into()),
        "this" => Some(ThisToken::new("this").into()),
        "true" => Some(TrueToken::new("true").into()),
        "var" => Some(VarToken::new("var").into()),
        "while" => Some(WhileToken::new("while").into()),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn scanning_single_character_lexemes_works() {
        let input = "(){},.-+;=*!<>/";
        let tokens = Lexer::lex(input.into());

        assert_eq!(
            tokens.into_iter().flatten().collect_vec(),
            vec![
                Token::new(LeftParenToken::new("(").into(), 1),
                Token::new(RightParenToken::new(")").into(), 1),
                Token::new(LeftBraceToken::new("{").into(), 1),
                Token::new(RightBraceToken::new("}").into(), 1),
                Token::new(CommaToken::new(",").into(), 1),
                Token::new(DotToken::new(".").into(), 1),
                Token::new(MinusToken::new("-").into(), 1),
                Token::new(PlusToken::new("+").into(), 1),
                Token::new(SemicolonToken::new(";").into(), 1),
                Token::new(EqualToken::new("=").into(), 1),
                Token::new(StarToken::new("*").into(), 1),
                Token::new(BangToken::new("!").into(), 1),
                Token::new(LessToken::new("<").into(), 1),
                Token::new(GreaterToken::new(">").into(), 1),
                Token::new(SlashToken::new("/").into(), 1),
                Token::new(TokenKind::Eof, 1),
            ]
        )
    }

    #[test]
    fn scanning_double_character_lexemes_works() {
        let input = " != <= >= == = =\n!\n=";
        let tokens = Lexer::lex(input.into());

        assert_eq!(
            tokens.into_iter().flatten().collect_vec(),
            vec![
                Token::new(BangEqualToken::new("!=").into(), 1),
                Token::new(LessEqualToken::new("<=").into(), 1),
                Token::new(GreaterEqualToken::new(">=").into(), 1),
                Token::new(EqualEqualToken::new("==").into(), 1),
                Token::new(EqualToken::new("=").into(), 1),
                Token::new(EqualToken::new("=").into(), 1),
                Token::new(BangToken::new("!").into(), 2),
                Token::new(EqualToken::new("=").into(), 3),
                Token::new(TokenKind::Eof, 3),
            ]
        )
    }

    #[test]
    fn ignoring_whitespaces_works() {
        let input = "(   \r)    {\t     }\n\n\n\n!";
        let tokens = Lexer::lex(input.into());

        assert_eq!(
            tokens.into_iter().flatten().collect_vec(),
            vec![
                Token::new(LeftParenToken::new("(").into(), 1),
                Token::new(RightParenToken::new(")").into(), 1),
                Token::new(LeftBraceToken::new("{").into(), 1),
                Token::new(RightBraceToken::new("}").into(), 1),
                Token::new(BangToken::new("!").into(), 5),
                Token::new(TokenKind::Eof, 5),
            ]
        )
    }

    #[test]
    fn scanning_basic_valid_strings_works() {
        let input = "\"this is a string\"";
        let tokens = Lexer::lex(input.into());

        assert_eq!(
            tokens.into_iter().flatten().collect_vec(),
            vec![
                Token::new(StringToken::new(r#""this is a string""#).into(), 1),
                Token::new(TokenKind::Eof, 1),
            ]
        )
    }

    #[test]
    fn scanning_multiline_strings_works() {
        let input = "\"this is a string\nacross multiple lines\"";
        let tokens = Lexer::lex(input.into());

        assert_eq!(
            tokens.into_iter().flatten().collect_vec(),
            vec![
                Token::new(
                    StringToken::new("\"this is a string\nacross multiple lines\"").into(),
                    2
                ),
                Token::new(TokenKind::Eof, 2),
            ]
        )
    }

    #[test]
    fn scanning_unterminated_string_produces_error() {
        let input = "\"this is not a string";
        let tokens = Lexer::lex(input.into());

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
        let tokens = Lexer::lex(input.into());

        assert_eq!(
            tokens.into_iter().collect_vec(),
            vec![
                Ok(Token::new(NumberToken::new(1.0).into(), 1)),
                Ok(Token::new(NumberToken::new(20.0).into(), 1)),
                Ok(Token::new(NumberToken::new(4212.0).into(), 1)),
                Ok(Token::new(TokenKind::Eof, 1)),
            ]
        )
    }

    #[test]
    fn scanning_valid_fractional_number_works() {
        let input = "  0.0001 2.0 421.2";
        let tokens = Lexer::lex(input.into());

        assert_eq!(
            tokens.into_iter().collect_vec(),
            vec![
                Ok(Token::new(NumberToken::new(0.0001).into(), 1)),
                Ok(Token::new(NumberToken::new(2.0).into(), 1)),
                Ok(Token::new(NumberToken::new(421.2).into(), 1)),
                Ok(Token::new(TokenKind::Eof, 1)),
            ]
        )
    }

    #[test]
    fn scanning_invalid_fractional_number_works() {
        let input = "  0. 2123. .2 .0012";
        let tokens = Lexer::lex(input.into());

        assert_eq!(
            tokens.into_iter().collect_vec(),
            vec![
                Ok(Token::new(NumberToken::new(0.0).into(), 1)),
                Ok(Token::new(DotToken::new(".").into(), 1)),
                Ok(Token::new(NumberToken::new(2123.0).into(), 1)),
                Ok(Token::new(DotToken::new(".").into(), 1)),
                Ok(Token::new(DotToken::new(".").into(), 1)),
                Ok(Token::new(NumberToken::new(2.0).into(), 1)),
                Ok(Token::new(DotToken::new(".").into(), 1)),
                Ok(Token::new(NumberToken::new(12.0).into(), 1)),
                Ok(Token::new(TokenKind::Eof, 1)),
            ]
        )
    }

    #[test]
    fn scanning_identifiers_works() {
        let input = "some_identifier _anotherOne als0 c1 0no 001_no ";
        let tokens = Lexer::lex(input.into());

        assert_eq!(
            tokens.into_iter().flatten().collect_vec(),
            vec![
                Token::new(IdentifierToken::new("some_identifier").into(), 1),
                Token::new(IdentifierToken::new("_anotherOne").into(), 1),
                Token::new(IdentifierToken::new("als0").into(), 1),
                Token::new(IdentifierToken::new("c1").into(), 1),
                Token::new(NumberToken::new(0.0).into(), 1),
                Token::new(IdentifierToken::new("no").into(), 1),
                Token::new(NumberToken::new(1.0).into(), 1),
                Token::new(IdentifierToken::new("_no").into(), 1),
                Token::new(TokenKind::Eof, 1),
            ]
        )
    }

    #[test]
    fn scanning_reserved_words_works() {
        let input = "and class else false for fun if nil or print return super this true var while";
        let tokens = Lexer::lex(input.into());

        assert_eq!(
            tokens.into_iter().flatten().collect_vec(),
            vec![
                Token::new(AndToken::new("and").into(), 1),
                Token::new(ClassToken::new("class").into(), 1),
                Token::new(ElseToken::new("else").into(), 1),
                Token::new(FalseToken::new("false").into(), 1),
                Token::new(ForToken::new("for").into(), 1),
                Token::new(FunToken::new("fun").into(), 1),
                Token::new(IfToken::new("if").into(), 1),
                Token::new(NilToken::new("nil").into(), 1),
                Token::new(OrToken::new("or").into(), 1),
                Token::new(PrintToken::new("print").into(), 1),
                Token::new(ReturnToken::new("return").into(), 1),
                Token::new(SuperToken::new("super").into(), 1),
                Token::new(ThisToken::new("this").into(), 1),
                Token::new(TrueToken::new("true").into(), 1),
                Token::new(VarToken::new("var").into(), 1),
                Token::new(WhileToken::new("while").into(), 1),
                Token::new(TokenKind::Eof, 1),
            ]
        )
    }

    #[test]
    fn scanning_multiple_lines_works() {
        let input = "(\n)\n{\n}\n";
        let tokens = Lexer::lex(input.into());

        assert_eq!(
            tokens.into_iter().flatten().collect_vec(),
            vec![
                Token::new(LeftParenToken::new("(").into(), 1),
                Token::new(RightParenToken::new(")").into(), 2),
                Token::new(LeftBraceToken::new("{").into(), 3),
                Token::new(RightBraceToken::new("}").into(), 4),
                Token::new(TokenKind::Eof, 5),
            ]
        )
    }

    #[test]
    fn scanning_comments_works() {
        let input = "() // this is a comment\n{} // another one";
        let tokens = Lexer::lex(input.into());

        assert_eq!(
            tokens.into_iter().flatten().collect_vec(),
            vec![
                Token::new(LeftParenToken::new("(").into(), 1),
                Token::new(RightParenToken::new(")").into(), 1),
                Token::new(LeftBraceToken::new("{").into(), 2),
                Token::new(RightBraceToken::new("}").into(), 2),
                Token::new(TokenKind::Eof, 2),
            ]
        )
    }
}
