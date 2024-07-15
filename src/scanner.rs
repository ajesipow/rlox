use std::mem;

use crate::token::Token;
use crate::token::TokenKind;

#[derive(Debug)]
pub(crate) struct Scanner {
    source: String,
    tokens: Vec<Token>,
}

impl Scanner {
    pub(crate) fn new(source: String) -> Self {
        Self {
            source,
            tokens: vec![],
        }
    }

    pub(crate) fn scan_tokens(&mut self) -> &[Token] {
        let mut lexeme = String::new();
        let line = 1;

        // TODO: Iterates over Unicode Scalar Values instead of grapheme clusters.
        for char in self.source.chars() {
            lexeme.push(char);
            let token_kind = match char {
                '(' => Some(TokenKind::LeftParen),
                ')' => Some(TokenKind::RightParen),
                '{' => Some(TokenKind::LeftBrace),
                '}' => Some(TokenKind::RightBrace),
                ',' => Some(TokenKind::Comma),
                '.' => Some(TokenKind::Dot),
                '-' => Some(TokenKind::Minus),
                '+' => Some(TokenKind::Plus),
                ';' => Some(TokenKind::Semicolon),
                '*' => Some(TokenKind::Star),
                _ => None,
            };
            if let Some(token_kind) = token_kind {
                let this_lexeme = mem::take(&mut lexeme);
                self.tokens
                    .push(Token::new(token_kind, Some(this_lexeme), line))
            }
        }

        self.tokens.push(Token::new(TokenKind::Eof, None, line));

        &self.tokens
    }
}
