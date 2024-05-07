use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;

use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on {}", path, addr);
    // path 会被借用，所以这里需要 clone 一下
    let state = HttpServeState { path: path.clone() };
    // axum router
    // Arc 会 clone 一份对应的 state 新增一个 Arc 指针，是比较轻量的
    let router = Router::new()
        .route("/*path", get(file_handler))
        .nest_service("/tower", ServeDir::new(path))
        .with_state(Arc::new(state));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}

// 文件读取内容
async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    // format!("{:?}, {:?},  {}", state, state.path, path)
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}", p);
    if !p.exists() {
        (
            StatusCode::NOT_FOUND,
            format!("File {} not found", p.display()),
        )
    } else {
        // TODO: test p is a directory
        // 如果是一个文件夹，以列表的方式展示出所有的文件和子文件夹，可以对对应的文件和文件夹生成链接访问
        // 使用异步的读写
        // 暂时不能读取二进制的内容
        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("Read {} bytes", content.len());
                (StatusCode::OK, content)
            }
            Err(e) => {
                warn!("Error reading file: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });
        let (status, content) = file_handler(State(state), Path("Cargo.toml".to_string())).await;
        assert_eq!(status, StatusCode::OK);
        assert!(content.trim().starts_with("[package]"));
    }
}
