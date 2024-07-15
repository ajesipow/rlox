use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub struct PublicError(#[from] Error);

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("cannot read input")]
    IO(#[from] std::io::Error),
}
