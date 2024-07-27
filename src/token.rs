use crate::error::LexicalError;

#[derive(Debug)]
pub struct Tokens<'a>(pub(crate) Vec<LexResult<'a>>);

pub(crate) type LexResult<'a> = Result<Token<'a>, LexicalError>;

#[derive(Debug)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub(crate) struct Token<'a> {
    kind: TokenKind,
    lexeme: Option<&'a str>,
    line: usize,
}

impl<'a> Token<'a> {
    pub(crate) fn new(
        kind: TokenKind,
        lexeme: Option<&'a str>,
        line: usize,
    ) -> Self {
        Self { kind, lexeme, line }
    }

    pub(crate) fn lexeme(&self) -> Option<&str> {
        self.lexeme
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub(crate) enum TokenKind {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}
