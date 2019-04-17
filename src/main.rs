use std::io;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use bufstream::BufStream;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8001")?;

    for stream in listener.incoming() {
        handle_client(stream?)?;
    }

    Ok(())
}

fn handle_client(stream: TcpStream) -> io::Result<()> {
    let mut request = String::new();
    let mut buf = BufStream::new(stream);

    // Get only the first line of the request, since this
    // is a static HTTP 1.0 server.
    buf.read_line(&mut request).unwrap();

    println!("Request: {}", request);

    let mut parts = request.split(" ");
    let method = parts.next().unwrap();  // Should only be GET
    let path = parts.next().unwrap();  // Requested path

    let mut response = String::from("HTTP/1.0 200 OK
Content-type: text/html

<h1>Success!</h1>");
    if method != "GET" {
        response = String::from("HTTP/1.0 405 Method Not Allowed");
    }

    buf.write_all(response.as_bytes())?;
    println!("Response: {}", response);

    Ok(())
}
