use std::io;
use std::net::TcpListener;

use rust_http_server;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8001")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                match rust_http_server::handle_client(stream) {
                    Err(e) => eprintln!("Error handling client: {}", e),
                    _ => (),
                }
            },
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}
