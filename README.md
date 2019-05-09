# Rust Static HTTP 1.0 Server

This is a static HTTP 1.0 server written in Rust. It supports GET requests for static documents (e.g. HTML, CSS, images, plain text) that exist in the document root on the server.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See deployment for notes on how to deploy the project on a live system.

### Prerequisites

[Install Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

### Development Setup

Clone this repository.

```
git clone git@github.com:levidyrek/rust_http_server.git
```

Create the document root. By default, this is at `/static/`, but can be set to a custom directory with the `STATIC_ROOT` environment variable.

```
mkdir /static/

or

export STATIC_ROOT="/custom_dir"
mkdir /custom_dir
```

Run the server in debug mode.

```
cargo run
```

## Deployment

Clone this repository.

```
git clone git@github.com:levidyrek/rust_http_server.git
```

Create the document root. By default, this is at `/static/`, but can be set to a custom directory with the `STATIC_ROOT` environment variable.

```
mkdir /static/

or

export STATIC_ROOT="/custom_dir"
mkdir /custom_dir
```

Build the package.

```
cargo build --release
```

Run the server.

```
./target/release/rust_http_server
```

## Usage

Place documents or images in the document root at `/static/`, or a in custom directory specified by the `STATIC_ROOT` environment variable.

Make an HTTP 1.0 request at the port `8001` to retrieve your documents.

```
$ curl -0 localhost:8001/index.html
<h1>Success!</h1>
```

## Built With

* [Rust](https://www.rust-lang.org/) - The language used

## Acknowledgments

* Inspired by [this /r/dailyprogrammer challenge](https://www.reddit.com/r/dailyprogrammer/comments/6lti17/20170707_challenge_322_hard_static_http_server/).
