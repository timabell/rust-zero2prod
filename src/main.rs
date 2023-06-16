use std::net::TcpListener;
use zero2prod::build_server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("localhost:8000").expect("failed to bind to port");
    build_server(listener)?.await
}
