use tonic::transport::Server;

mod http;
mod protos;
use http::Client as HttpClient;
use protos::http_client::http_client_server::HttpClientServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let addr = std::env::var("SERVER_ADDR")?.parse()?;
    tracing::info!(message = "Starting server.", %addr);

    Server::builder()
        .trace_fn(|_| tracing::info_span!("scaffolding"))
        .add_service(HttpClientServer::new(HttpClient::default()))
        .serve(addr)
        .await?;
    Ok(())
}
