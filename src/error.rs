//! Error types for kdeets

use thiserror::Error;

/// The error type for kdeets.
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum Error {
    #[error("The crate was not found on crates.io")]
    CrateNotFoundonIndex,
    #[error("tame_index error says: {0:?}")]
    TameIndex(#[from] tame_index::Error),
    /// Error passed up from reqwest
    #[error("reqwest error says: {0:?}")]
    Reqwest(#[from] tame_index::external::reqwest::Error),
    /// Error passed up from http
    #[error("http error says: {0:?}")]
    Http(#[from] tame_index::external::http::Error),
}
