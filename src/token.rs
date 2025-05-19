use std::rc::Rc;

use crate::error::LexicalError;

#[derive(Debug)]
pub(crate) struct Tokens(Vec<LexResult>);

impl Tokens {
    pub fn new(tokens: Vec<LexResult>) -> Self {
        Self(tokens)
    }
}

impl IntoIterator for Tokens {
    type IntoIter = std::vec::IntoIter<Self::Item>;
    type Item = LexResult;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub(crate) type LexResult = Result<Token, LexicalError>;

#[derive(Debug, PartialEq)]
pub(crate) struct Token {
    kind: TokenKind,
    line: usize,
}

impl Token {
    pub(crate) fn new(
        kind: TokenKind,
        line: usize,
    ) -> Self {
        Self { kind, line }
    }

    pub(crate) fn kind(&self) -> &TokenKind {
        &self.kind
    }

    pub(crate) fn line(&self) -> usize {
        self.line
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TokenKind {
    // Single-character tokens
    LeftParen { lexeme: Rc<str> },
    RightParen { lexeme: Rc<str> },
    LeftBrace { lexeme: Rc<str> },
    RightBrace { lexeme: Rc<str> },
    Comma { lexeme: Rc<str> },
    Dot { lexeme: Rc<str> },
    Minus { lexeme: Rc<str> },
    Plus { lexeme: Rc<str> },
    Semicolon { lexeme: Rc<str> },
    Slash { lexeme: Rc<str> },
    Star { lexeme: Rc<str> },

    // One or two character tokens
    Bang { lexeme: Rc<str> },
    BangEqual { lexeme: Rc<str> },
    Equal { lexeme: Rc<str> },
    EqualEqual { lexeme: Rc<str> },
    Greater { lexeme: Rc<str> },
    GreaterEqual { lexeme: Rc<str> },
    Less { lexeme: Rc<str> },
    LessEqual { lexeme: Rc<str> },

    // Literals
    Identifier { lexeme: Rc<str> },
    String { lexeme: Rc<str> },
    Number { lexeme: f64 },

    // Keywords
    And { lexeme: Rc<str> },
    Class { lexeme: Rc<str> },
    Else { lexeme: Rc<str> },
    False { lexeme: Rc<str> },
    Fun { lexeme: Rc<str> },
    For { lexeme: Rc<str> },
    If { lexeme: Rc<str> },
    Nil { lexeme: Rc<str> },
    Or { lexeme: Rc<str> },
    Print { lexeme: Rc<str> },
    Return { lexeme: Rc<str> },
    Super { lexeme: Rc<str> },
    This { lexeme: Rc<str> },
    True { lexeme: Rc<str> },
    Var { lexeme: Rc<str> },
    While { lexeme: Rc<str> },

    Eof,
}
