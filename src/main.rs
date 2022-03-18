use std::net::TcpListener;

use ausgleichende_gerechtigkeit::startup;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind port");
    startup::run(listener)?.await
}
