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

    /// Start a simple TCP server that receives text messages.
    /// Runs in a blocking loop (for now).
    pub fn start_server(port: u16) -> std::io::Result<()> {
        use std::io::Read;
        use std::net::TcpListener;
        use std::thread;

        let listener = TcpListener::bind(("0.0.0.0", port))?;
        println!("Server listening on port {}", port);

        for stream in listener.incoming() {
            match stream {
                Ok(mut s) => {
                    thread::spawn(move || {
                        let mut buffer = [0; 1024];
                        match s.read(&mut buffer) {
                            Ok(n) if n > 0 => {
                                let msg = String::from_utf8_lossy(&buffer[..n]);
                                println!("Received: {}", msg);
                            }
                            Ok(_) => {}
                            Err(e) => eprintln!("Read error: {}", e),
                        }
                    });
                }
                Err(e) => eprintln!("Connection failed: {}", e),
            }
        }

        Ok(())
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
