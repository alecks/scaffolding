use tonic::transport::Server;

mod rest;
use rest::{Client as RestClient, Server as RestClientServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let addr = std::env::var("SERVER_ADDR")?.parse()?;
    tracing::info!(message = "Starting server.", %addr);

    Server::builder()
        .trace_fn(|_| tracing::info_span!("scaffolding"))
        .add_service(RestClientServer::new(RestClient::default()))
        .serve(addr)
        .await?;
    Ok(())
}
