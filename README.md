# Hello

A low-level Rust implementation of a multi-threaded web server, written following [the Rust Book](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html).

## Installation

- `git clone`
- `cd hello`

## Executing

Invoke `cargo run` and load `127.0.0.1:7878` in a web browser.

## Noteworthy features

- Multi-threading : load `127.0.0.1:7878/sleep`, which holds a thread for 5 seconds, and try loading another page in the meantime !
- Graceful shutdown : after two requests, the server will gracefully shutdown (let threads finish before disconnecting them and shutting down).
