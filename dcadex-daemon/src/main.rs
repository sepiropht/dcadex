//! main.rs
use dcadex::run;
use std::net::TcpListener;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    run(listener)?.await
}
