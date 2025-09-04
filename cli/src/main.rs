use relay_core::{ConnectionManager, TransferManager};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: relay_cli [send|serve] ...");
        return;
    }

    match args[1].as_str() {
        "serve" => {
            let port: u16 = args.get(2).and_then(|p| p.parse().ok()).unwrap_or(5000);
            ConnectionManager::start_server(port).unwrap();
        }
        "send" => {
            if args.len() < 4 {
                eprintln!("Usage: relay_cli send <ip:port> <message>");
                return;
            }
            TransferManager::send_text(&args[2], &args[3]).unwrap();
        }
        _ => eprintln!("Unknown command"),
    }
}
