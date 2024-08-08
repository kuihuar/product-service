mod proto;

mod service;
mod server;
mod repository;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    server::server::start_server().await?;
    Ok(())
}