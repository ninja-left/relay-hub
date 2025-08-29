//! Connection management
//!
//! Planned features:
//! - Manual IP connect (TCP)
//! - TLS with self-signed certs
//! - Listener for incoming connections
//! - LAN discovery/broadcast (later)
//!

use crate::{RhError, RhResult};

/// Connection manager stub
pub struct ConnectionManager;

impl ConnectionManager {
    pub fn new() -> Self {
        Self
    }

    /// Connect to a peer by IP/port (planned TCP/TLS)
    pub fn connect(&self, _ip: &str, _port: u16) -> RhResult<()> {
        Err(RhError::Unimplemented("connect"))
    }

    /// Start listening for incoming connections on addr:port
    pub fn listen(&self, _addr: &str, _port: u16) -> RhResult<()> {
        Err(RhError::Unimplemented("listen"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connection_manager_instantiates() {
        let cm = ConnectionManager::new();
        // For now just ensure methods return the stub error
        assert!(matches!(
            cm.connect("127.0.0.1", 9000),
            Err(RhError::Unimplemented(_))
        ));
        assert!(matches!(
            cm.listen("0.0.0.0", 9000),
            Err(RhError::Unimplemented(_))
        ));
    }
}
