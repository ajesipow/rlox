use crate::error::LexicalError;

#[derive(Debug)]
pub(crate) struct Tokens<'a>(Vec<LexResult<'a>>);

impl<'a> Tokens<'a> {
    pub fn new(tokens: Vec<LexResult<'a>>) -> Self {
        Self(tokens)
    }
}

impl<'a> IntoIterator for Tokens<'a> {
    type IntoIter = std::vec::IntoIter<Self::Item>;
    type Item = LexResult<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

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

    pub(crate) fn kind(&self) -> TokenKind {
        self.kind
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
