use std::error::Error as ErrorTrait;

use thiserror::Error;
use tracing_subscriber::fmt;

pub type BoxedError = Box<dyn ErrorTrait + Send + Sync + 'static>;

#[derive(Debug, Error)]
#[error("error initializing tracing: {error}")]
pub struct Error {
    #[from]
    error: BoxedError,
}

impl Error {
    pub const fn new(error: BoxedError) -> Self {
        Self { error }
    }

    pub fn get(self) -> BoxedError {
        self.error
    }
}

pub fn init() -> Result<(), Error> {
    fmt().try_init()?;

    Ok(())
}
