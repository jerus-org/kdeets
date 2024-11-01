//! Error types for kdeets

// use std::ffi::OsString;

// use proc_exit::{Code, Exit};
use thiserror::Error;

// pub const EXIT_UNEXPECTED_ERROR: i32 = 10;
// pub const EXIT_NOT_CALCULATED_CODE: i32 = 12;
// pub const EXIT_MISSING_REQUIRED_CODE: i32 = 13;
// // pub const EXIT_NOT_REQUIRED_LEVEL: i32 = 14;
// pub const EXIT_NO_FILES_LISTED: i32 = 15;

/// The error type for nextsv.
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

// impl From<Error> for Exit {
//     /// Convert select error codes into an exit code to allow a calling application to exit
//     /// with an error code instead of handling the error.
//     fn from(err: Error) -> Self {
//         match err {
//             Error::Git2(_) => {
//                 Exit::new(Code::new(EXIT_NOT_CALCULATED_CODE)).with_message(err.to_string())
//             }
//             Error::MissingRequiredFile(_) => {
//                 Exit::new(Code::new(EXIT_MISSING_REQUIRED_CODE)).with_message(err.to_string())
//             }
//             Error::NoFilesListed => {
//                 Exit::new(Code::new(EXIT_NO_FILES_LISTED)).with_message(err.to_string())
//             }
//             // Error::MinimumChangeLevelMet => Exit::new(Code::SUCCESS).with_message(err.to_string()),
//             // Error::MinimumChangeLevelNotMet => {
//             //     Exit::new(Code::new(EXIT_NOT_REQUIRED_LEVEL)).with_message(err.to_string())
//             // }
//             _ => Exit::new(Code::new(EXIT_UNEXPECTED_ERROR)),
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_constants() {
//         assert_eq!(EXIT_UNEXPECTED_ERROR, 10);
//         assert_eq!(EXIT_NOT_CALCULATED_CODE, 12);
//         assert_eq!(EXIT_MISSING_REQUIRED_CODE, 13);
//         // assert_eq!(EXIT_NOT_REQUIRED_LEVEL, 14);
//         assert_eq!(EXIT_NO_FILES_LISTED, 15);
//     }
// }
