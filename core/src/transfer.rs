//! Transfer management
//!
//! Planned features:
//! - Text sending/receiving
//! - File sending/receiving with progress
//! - Optional compression via plugin
//!

use std::io::{Read, Write};
use std::net::{TcpStream, TcpListener};

use crate::error::{RhError, RhResult};

pub struct TransferManager;

impl TransferManager {
    /// Send a text message to a peer (client-side).
    pub fn send_text(ip: &str, port: u16, message: &str) -> RhResult<()> {
        match TcpStream::connect((ip, port)) {
            Ok(mut stream) => {
                stream
                    .write_all(message.as_bytes())
                    .map_err(|e| RhError::Io(format!("write failed: {e}")))?;
                Ok(())
            }
            Err(e) => Err(RhError::Io(format!("connect failed: {e}"))),
        }
    }

    /// Receive a text message (blocking, single connection).
    pub fn receive_text(port: u16) -> RhResult<String> {
        let listener = TcpListener::bind(("127.0.0.1", port))
            .map_err(|e| RhError::Io(format!("bind failed: {e}")))?;

        if let Ok((mut stream, _addr)) = listener.accept() {
            let mut buffer = [0; 1024];
            let n = stream
                .read(&mut buffer)
                .map_err(|e| RhError::Io(format!("read failed: {e}")))?;
            Ok(String::from_utf8_lossy(&buffer[..n]).to_string())
        } else {
            Err(RhError::Io("accept failed".into()))
        }
    }
}
#[cfg(test)]
mod tests {
    use super::TransferManager;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_send_and_receive_text() {
        let port = 45678;

        // Spawn server in a thread
        let handle = thread::spawn(move || {
           TransferManager::receive_text(port).unwrap()
        });

        // Give the server a moment to bind
        thread::sleep(Duration::from_millis(100));

        // Send a message
        let msg = "Hello RelayHub!";
        TransferManager::send_text("127.0.0.1", port, msg).unwrap();

        // Verify the received message
        let received = handle.join().unwrap();
        assert_eq!(received, msg);
    }
}
