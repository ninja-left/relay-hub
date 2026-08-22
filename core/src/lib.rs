//! RelayHub Core – library entry point
//!
//! This crate exposes a minimal API surface for Phase 1 so CI can build & test.
//! Future steps will implement actual networking, TLS, discovery, and transfers.
//!
#![allow(unused_imports)]
#![allow(clippy::new_without_default)]

pub mod connection;
pub mod transfer;

pub use connection::ConnectionManager;
pub use transfer::TransferManager;

/// Common result type for RelayHub core.
pub type RhResult<T> = Result<T, RhError>;

/// Error type for RelayHub core operations.
#[derive(Debug)]
pub enum RhError {
    /// Placeholder for unimplemented operations
    Unimplemented(&'static str),
    /// An I/O operation failed.
    Io(String),
}

impl core::fmt::Display for RhError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            RhError::Unimplemented(msg) => write!(f, "unimplemented: {msg}"),
            RhError::Io(msg) => write!(f, "I/O error: {msg}"),
        }
    }
}

impl std::error::Error for RhError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crate_compiles_and_basic_api_exists() {
        let _ = crate::connection::ConnectionManager::new();
    }
}
