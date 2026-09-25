use std::{
    io::Error as IoError,
    path::{Path, PathBuf},
};

use bon::builder;
use thiserror::Error;
use tokio::fs::{create_dir_all, try_exists};
use tracing::info;

#[derive(Debug, Error)]
#[error("failed to check existence of `{path}`: {error}")]
pub struct ExistenceError {
    path: PathBuf,
    error: IoError,
}

impl ExistenceError {
    pub const fn new(path: PathBuf, error: IoError) -> Self {
        Self { path, error }
    }
}

#[derive(Debug, Error)]
#[error("failed to create `{path}`: {error}")]
pub struct CreateError {
    path: PathBuf,
    error: IoError,
}

impl CreateError {
    pub const fn new(path: PathBuf, error: IoError) -> Self {
        Self { path, error }
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    Existence(#[from] ExistenceError),
    Create(#[from] CreateError),
}

async fn create_dir(directory: &Path) -> Result<(), CreateError> {
    create_dir_all(directory)
        .await
        .map_err(|error| CreateError::new(directory.to_owned(), error))
}

async fn existence(path: &Path) -> Result<bool, ExistenceError> {
    try_exists(path)
        .await
        .map_err(|error| ExistenceError::new(path.to_owned(), error))
}

#[builder]
pub async fn ensure<D: AsRef<Path>>(directory: D, create: bool) -> Result<bool, Error> {
    let path = directory.as_ref();

    let exists = existence(path).await?;

    let did_not_exist = !exists;

    if did_not_exist && create {
        info!("creating `{display}`", display = path.display());

        create_dir(path).await?;
    }

    Ok(did_not_exist)
}
