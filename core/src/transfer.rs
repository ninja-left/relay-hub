//! Transfer management
//!
//! Planned features:
//! - Text sending/receiving
//! - File sending/receiving with progress
//! - Optional compression via plugin
//!

use crate::{RhError, RhResult};

pub struct TransferManager;

impl TransferManager {
    pub fn new() -> Self {
        Self
    }

    /// Send a UTF-8 text message to the connected peer
    pub fn send_text(&self, _message: &str) -> RhResult<()> {
        Err(RhError::Unimplemented("send_text"))
    }

    /// Receive next UTF-8 text message from the connected peer
    pub fn receive_text(&self) -> RhResult<String> {
        Err(RhError::Unimplemented("receive_text"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transfer_manager_instantiates() {
        let tm = TransferManager::new();
        assert!(matches!(
            tm.send_text("hello"),
            Err(RhError::Unimplemented(_))
        ));
        assert!(matches!(tm.receive_text(), Err(RhError::Unimplemented(_))));
    }
}
