use std::env;
use std::io;
use std::net::TcpListener;
use std::path::Path;

use rust_http_server;

extern crate chrono;


fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8001")?;
    let static_root = match env::var("STATIC_ROOT") {
        Ok(val) => val,
        Err(_) => "/static".to_string(),
    };

    // If static root does not exist or is not a directory, terminate.
    let path = Path::new(&static_root);
    if !path.exists() {
        panic!("Static root directory ({}) does not exist or can not be accessed.", &static_root);
    }
    if !path.is_dir() {
        panic!("Static root ({}) is not a directory.", &static_root);
    }

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
