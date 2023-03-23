//! # Errors
//!
//! Custom errors for the whole library.
//! Utility types related to errors (Result).
//! Convert errors from dependencies.

////////////////////////////////////////////////////////////////////////////////

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Failed to clean {0}: {1}")]
    CleanError(String, String),

    #[error("Failed to compile {0}: {1}")]
    CompileError(String, String),

    #[error("Failed to retrieve from IPFS {0}: {1}")]
    RetrieveFromIpfsError(String, String),
}

/// Utility result type to be used throughout
pub type Result<T> = std::result::Result<T, RuntimeError>;
