use zero2prod::run;
use std::net::TcpListener;
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind to any port using port 0");
    run(listener)?.await
}
