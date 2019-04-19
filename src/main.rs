use std::io;
use std::net::TcpListener;

use rust_http_server;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8001")?;

    for stream in listener.incoming() {
        rust_http_server::handle_client(stream?)?;
    }

    Ok(())
}
