use anyhow::{Ok, Result};
use axum::{routing::get, Router};
use std::{net::SocketAddr, path::Path};

use tracing::info;

pub async fn process_http_serve(path: &Path, port: u16) -> Result<()> {
    // axum router
    let router = Router::new().route("/", get(index_handler));
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    info!("Serving {} on {}", path.display(), addr);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}

async fn index_handler() -> &'static str {
    "hello world"
}
