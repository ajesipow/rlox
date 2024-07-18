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
pub enum LexicalError {
    #[error("unexpected character {char:?} on line {line:?}")]
    UnexpectedCharacter { char: char, line: usize },
}
