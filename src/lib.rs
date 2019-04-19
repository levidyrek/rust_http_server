use std::{io, fs};
use std::io::prelude::*;
use std::net::TcpStream;
use bufstream::BufStream;


static STATIC_ROOT: &str = "/static";

struct Request {
    method: String,
    path: String,
}

pub fn handle_client(stream: TcpStream) -> io::Result<()> {
    let mut buf = BufStream::new(stream);
    let request = parse_request(&mut buf);

    let mut response = String::from("\
        HTTP/1.0 200 OK\n\
        Content-type: text/html\n\
        \n\
        <h1>Success!</h1>\n\
    ");
    if request.method != "GET" {
        response = String::from("\
            HTTP/1.0 405 Method Not Allowed\n\
            Allow: GET\n\
        ");
    }

    buf.write_all(response.as_bytes())?;
    println!("Response: {}", response);

    Ok(())
}

fn parse_request(buf: &mut BufStream<TcpStream>) -> Request {
    let mut request = String::new();

    // Get only the first line of the request, since this
    // is a static HTTP 1.0 server.
    buf.read_line(&mut request).unwrap();

    println!("Request: {}", request);

    let mut parts = request.split(" ");
    let method = parts.next().unwrap().to_string();
    let path = parts.next().unwrap().to_string();

    Request{ method: method, path: path }
}
