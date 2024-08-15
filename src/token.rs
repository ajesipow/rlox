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

#[derive(Debug, PartialEq)]
pub(crate) struct Token<'a> {
    kind: TokenKind<'a>,
    line: usize,
}

impl<'a> Token<'a> {
    pub(crate) fn new(
        kind: TokenKind<'a>,
        line: usize,
    ) -> Self {
        Self { kind, line }
    }

    pub(crate) fn kind(&self) -> TokenKind<'a> {
        self.kind
    }

    pub(crate) fn line(&self) -> usize {
        self.line
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum TokenKind<'a> {
    // Single-character tokens
    LeftParen { lexeme: &'a str },
    RightParen { lexeme: &'a str },
    LeftBrace { lexeme: &'a str },
    RightBrace { lexeme: &'a str },
    Comma { lexeme: &'a str },
    Dot { lexeme: &'a str },
    Minus { lexeme: &'a str },
    Plus { lexeme: &'a str },
    Semicolon { lexeme: &'a str },
    Slash { lexeme: &'a str },
    Star { lexeme: &'a str },

    // One or two character tokens
    Bang { lexeme: &'a str },
    BangEqual { lexeme: &'a str },
    Equal { lexeme: &'a str },
    EqualEqual { lexeme: &'a str },
    Greater { lexeme: &'a str },
    GreaterEqual { lexeme: &'a str },
    Less { lexeme: &'a str },
    LessEqual { lexeme: &'a str },

    // Literals
    Identifier { lexeme: &'a str },
    String { lexeme: &'a str },
    Number { lexeme: f64 },

    // Keywords
    And { lexeme: &'a str },
    Class { lexeme: &'a str },
    Else { lexeme: &'a str },
    False { lexeme: &'a str },
    Fun { lexeme: &'a str },
    For { lexeme: &'a str },
    If { lexeme: &'a str },
    Nil { lexeme: &'a str },
    Or { lexeme: &'a str },
    Print { lexeme: &'a str },
    Return { lexeme: &'a str },
    Super { lexeme: &'a str },
    This { lexeme: &'a str },
    True { lexeme: &'a str },
    Var { lexeme: &'a str },
    While { lexeme: &'a str },

    Eof,
}
