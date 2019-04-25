use std::{io, fs};
use std::io::prelude::*;
use std::net::TcpStream;
use bufstream::BufStream;
use http::StatusCode;


// TODO: Make this configurable.
static STATIC_ROOT: &str = "/static";

struct Request {
    method: String,
    path: String,
}

enum ContentType {
    XML,
    GIF,
    HTML,
    JPEG,
    PNG,
    SVG,
    CSS,
    TEXT,
}

impl ContentType {
    fn from_file_ext(ext: &str) -> ContentType {
        match ext {
            "css" => ContentType::CSS,
            "gif" => ContentType::GIF,
            "htm" => ContentType::HTML,
            "html" => ContentType::HTML,
            "jpeg" => ContentType::JPEG,
            "jpg" => ContentType::JPEG,
            "png" => ContentType::PNG,
            "svg" => ContentType::SVG,
            "txt" => ContentType::TEXT,
            "xml" => ContentType::XML,
            _ => ContentType::TEXT,
        }
    }

    fn value(&self) -> &str {
        match *self {
            ContentType::XML => "application/xml",
            ContentType::GIF => "image/gif",
            ContentType::HTML => "text/html",
            ContentType::JPEG => "image/jpeg",
            ContentType::PNG => "image/png",
            ContentType::SVG => "image/svg+xml",
            ContentType::CSS => "text/css",
            ContentType::TEXT => "text/plain",
        }
    }
}

struct ResponseHeaders {
    content_type: Option<ContentType>,
}

impl ResponseHeaders {
    fn new() -> ResponseHeaders {
        ResponseHeaders {
            content_type: None,
        }
    }
}

struct Response {
    body: Option<Vec<u8>>,
    headers: ResponseHeaders,
    status: StatusCode,
}

impl Response {
    fn new() -> Response {
        Response {
            body: None,
            headers: ResponseHeaders::new(),
            status: StatusCode::OK,
        }
    }
}

pub fn handle_client(stream: TcpStream) -> io::Result<()> {
    let mut buf = BufStream::new(stream);
    let request = parse_request(&mut buf);
    let response = build_response(request);
    let formatted = format_response(response);

    buf.write_all(&formatted)?;

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

    Request { method: method, path: path }
}

fn build_response(request: Request) -> Response {
    let mut response = Response::new();
    if request.method != "GET" {
        response.status = StatusCode::METHOD_NOT_ALLOWED;
    } else {
        add_file_to_response(&request.path, &mut response);
    }

    response
}

fn add_file_to_response(path: &String, response: &mut Response) {
    let path = format!("{}{}", STATIC_ROOT, path);
    let contents = fs::read(&path);
    match contents {
        Ok(contents) => {
            response.body = Some(contents);
            let ext = path.split(".").last().unwrap();
            response.headers.content_type = Some(ContentType::from_file_ext(ext));
        },
        Err(_e) => {
            // TODO: Handle specific errors.
            response.status = StatusCode::NOT_FOUND;
        }
    }
}

fn format_response(response: Response) -> Vec<u8> {
    let mut result;
    let status_reason = match response.status.canonical_reason() {
        Some(reason) => reason,
        None => "",
    };
    result = format!(
        "HTTP/1.0 {} {}\n",
        response.status.as_str(),
        status_reason,
    );
    result = format!("{}Allow: GET\n", result);

    match response.headers.content_type {
        Some(content_type) => {
            result = format!(
                "{}Content-type: {}\n", result, content_type.value());
        },
        _ => (),
    }

    let mut bytes = result.as_bytes().to_vec();

    match response.body {
        Some(mut body) => {
            bytes.append(&mut "\n".as_bytes().to_vec());
            bytes.append(&mut body);
        },
        _ => (),
    }

    bytes
}
