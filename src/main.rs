use tonic::transport::Server;

mod protos;
mod rest;
use protos::rest_client::rest_client_server::RestClientServer;
use rest::Client as RestClient;

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
