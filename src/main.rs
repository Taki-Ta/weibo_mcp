mod models;
mod service;
#[cfg(test)]
mod tests;

use rmcp::transport::sse_server::SseServer;
use service::WeiboHotSearch;
use std::error::Error;
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

// Define the port for our server
const BIND_ADDRESS: &str = "127.0.0.1:3000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Set up logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Weibo Hot Search MCP server at {}", BIND_ADDRESS);
    
    // Using SseServer as shown in the example
    let ct = SseServer::serve(BIND_ADDRESS.parse()?)
        .await?
        .with_service(WeiboHotSearch::new);

    tracing::info!("Server started. Press Ctrl+C to stop.");
    
    // Wait for Ctrl-C
    tokio::signal::ctrl_c().await?;
    tracing::info!("Shutting down...");
    ct.cancel();
    
    Ok(())
}
