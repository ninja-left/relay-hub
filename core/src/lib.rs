//! RelayHub Core – library entry point
//!
//! This crate exposes a minimal API surface for Phase 1 so CI can build & test.
//! Future steps will implement actual networking, TLS, discovery, and transfers.
//!

pub mod connection;
pub mod transfer;

/// Common result type for RelayHub core.
pub type RhResult<T> = Result<T, RhError>;

/// Minimal error type for now.
#[derive(Debug)]
pub enum RhError {
    /// Placeholder for unimplemented operations
    Unimplemented(&'static str),
}

impl core::fmt::Display for RhError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            RhError::Unimplemented(msg) => write!(f, "unimplemented: {}", msg),
        }
    }
}

impl std::error::Error for RhError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crate_compiles_and_basic_api_exists() {
        // Call a couple of stub APIs just to ensure they link
        let _ = crate::connection::ConnectionManager::new();
        let _ = crate::transfer::TransferManager::new();
    }
}
