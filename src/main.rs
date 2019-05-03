use std::env;
use std::io;
use std::net::TcpListener;
use std::path::Path;

use rust_http_server;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8001")?;
    let static_root = match env::var("STATIC_ROOT") {
        Ok(val) => {
            let path = Path::new(val);
            if !path.exists() {
                panic!("Path set by STATIC_ROOT does not exist or can not be accessed.");
            }
            val
        },
        Err(_) => "/static".to_string(),
    };

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                match rust_http_server::handle_client(stream, &static_root) {
                    Err(e) => eprintln!("Error handling client: {}", e),
                    _ => (),
                }
            },
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}
