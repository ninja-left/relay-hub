use relayhub_core::{ConnectionManager, TransferManager};
use std::env;
use std::net::SocketAddr;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: relayhub_cli [send|serve] ...");
        return;
    }

    match args[1].as_str() {
        "serve" => {
            let port: u16 = args.get(2).and_then(|p| p.parse().ok()).unwrap_or(5000);
            let manager = ConnectionManager::new();

            match manager.listen("127.0.0.1", port) {
                Ok(message) => println!("Received: {message}"),
                Err(error) => eprintln!("Receive failed: {error}"),
            }
        }
        "send" => {
            if args.len() < 4 {
                eprintln!("Usage: relayhub_cli send <ip:port> <message>");
                return;
            }

            let target: SocketAddr = match args[2].parse() {
                Ok(address) => address,
                Err(error) => {
                    eprintln!("Invalid address '{}': {error}", args[2]);
                    return;
                }
            };

            let ip = target.ip().to_string();
            if let Err(error) = TransferManager::send_text(&ip, target.port(), &args[3]) {
                eprintln!("Send failed: {error}");
            }
        }
        _ => eprintln!("Unknown command"),
    }
}
