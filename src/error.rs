use std::io;
use std::path::{Path, PathBuf};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid source")]
    InvalidSource,

    #[error("could not find `niter.json` in the current directory")]
    MainFileNotFound,

    #[error("format `{0}` is not supported")]
    UnsupportedFormat(String),

    #[error("a modpack project in the current directory is already initiated")]
    AlreadyInitiated,

    #[error("{0}")]
    Fetch(#[from] reqwest::Error),

    #[error("{0}")]
    URL(#[from] url::ParseError),

    #[error("failed to perform I/O on `{0}`: {1}")]
    IO(PathBuf, io::Error),

    #[error("failed to serialize: {0}")]
    Serde(#[from] serde_json::Error)
}


pub trait MapErrToNiterExt<T> {
    fn map_err_to_niter(self, path: &Path) -> Result<T>;
}

impl<T> MapErrToNiterExt<T> for core::result::Result<T, serde_json::Error> {
    fn map_err_to_niter(self, _: &Path) -> Result<T> {
        self.map_err(|err| err.into())
    }
}

impl<T> MapErrToNiterExt<T> for core::result::Result<T, io::Error> {
    fn map_err_to_niter(self, path: &Path) -> Result<T> {
        self.map_err(|err| Error::IO(path.into(), err))
    }
}
