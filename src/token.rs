use crate::error::LexicalError;

#[derive(Debug)]
pub struct Tokens(pub(crate) Vec<LexResult>);

pub(crate) type LexResult = Result<Token, LexicalError>;

#[derive(Debug)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub struct Token {
    kind: TokenKind,
    lexeme: Option<String>,
    line: usize,
}

impl Token {
    pub fn new(
        kind: TokenKind,
        lexeme: Option<String>,
        line: usize,
    ) -> Self {
        Self { kind, lexeme, line }
    }
    
    pub(crate) fn lexeme(&self) -> Option<&str> {
        self.lexeme.as_ref().map(|s| s.as_str())
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub enum TokenKind {
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
