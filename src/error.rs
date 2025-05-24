use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub struct PublicError(#[from] Error);

#[derive(Debug, Error)]
pub enum Error {
    #[error("cannot read input: {0}")]
    IO(#[from] std::io::Error),
    #[error("cannot parse input: {0}")]
    Pase(#[from] ParseError),
    #[error("cannot interpret input: {0}")]
    Interpret(#[from] RunTimeError),
}

#[derive(Debug, Error)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub enum LexicalError {
    #[error("unexpected character {char:?} on line {line:?}")]
    UnexpectedCharacter { char: char, line: usize },
    #[error("unterminated string on line {line:?}")]
    UnterminatedString { line: usize },
    #[error("cannot parse number on line {line:?}")]
    NaN { line: usize },
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("expected ')' after expression on line {line:?}")]
    ExpectedClosingParenAfterExpr { line: usize },
    #[error("expected ';' after value")]
    ExpectSemicolon,
    #[error("expected '}}' after block")]
    ExpectRightBraceAfterBlock,
    #[error("invalid assignment target")]
    InvalidAssignmentTarget,
    #[error("unexpected EOF")]
    UnexpectedEof,
    #[error("expected expression")]
    ExpectExpression,
    #[error("expected statement expression")]
    ExpectStatement,
    #[error("expected variable name")]
    ExpectIdentifier,
    #[error("internal parser error")]
    Internal(#[from] ParseErrorInternal),
}

#[derive(Debug, Error)]
pub enum ParseErrorInternal {
    #[error("unhandled token on line {line:?}")]
    UnhandledToken { line: usize },
}

#[derive(Debug, Error)]
pub enum RunTimeError {
    #[error("unexpected literal {literal:?} or operand on line {line:?}")]
    UnexpectedUnaryToken { line: usize, literal: String },
    #[error("unexpected literals {left:?}, {right:?} or operand on line {line:?}")]
    UnexpectedBinaryToken {
        line: usize,
        left: String,
        right: String,
    },
    #[error("undefined variable {variable:?}")]
    UndefinedVariable { variable: String },
    #[error("expected identifier token")]
    ExpectedIdentifierToken,
}
