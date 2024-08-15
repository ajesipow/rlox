use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub struct PublicError(#[from] Error);

#[derive(Debug, Error)]
pub enum Error {
    #[error("cannot read input")]
    IO(#[from] std::io::Error),
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
