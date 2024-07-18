use crate::error::LexicalError;

#[derive(Debug)]
pub struct Tokens(pub(crate) Vec<LexResult>);

pub(crate) type LexResult = Result<Token, LexicalError>;

#[derive(Debug)]
pub(crate) struct Token {
    kind: TokenKind,
    lexeme: Option<String>,
    line: usize,
}

impl Token {
    pub(crate) fn new(
        kind: TokenKind,
        lexeme: Option<String>,
        line: usize,
    ) -> Self {
        Self { kind, lexeme, line }
    }
}

#[derive(Debug)]

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
