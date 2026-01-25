use std::net::TcpListener;

use email_newsletter::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listner = TcpListener::bind("127.0.0.1:8000").expect("Failed to bin to 8000");

    run(listner)?.await
}

