//! Connection management
//!
//! Planned features:
//! - Manual IP connect (TCP)
//! - TLS with self-signed certs
//! - Listener for incoming connections
//! - LAN discovery/broadcast (later)
//!

use crate::transfer::TransferManager;
use crate::RhResult;

pub struct ConnectionManager;

impl ConnectionManager {
    pub fn new() -> Self {
        Self
    }

    /// Connect to a peer and send a text message
    pub fn connect(&self, ip: &str, port: u16, message: &str) -> RhResult<()> {
        TransferManager::send_text(ip, port, message)
    }

    /// Listen for incoming connections and return the first received text
    pub fn listen(&self, _addr: &str, port: u16) -> RhResult<String> {
        // For now we ignore addr and just bind to localhost
        TransferManager::receive_text(port)
    }
}

#[cfg(test)]
mod tests {
    use super::ConnectionManager;

    #[test]
    fn connection_manager_instantiates() {
        let cm = ConnectionManager::new();

        // Ensure object can be created
        assert!(cm.connect("127.0.0.1", 9000, "test").is_err());
    }
}
