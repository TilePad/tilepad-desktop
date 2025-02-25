use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use anyhow::Context;
use axum::Extension;
use tauri::AppHandle;
use tower_http::cors::CorsLayer;

use crate::{database::DbPool, device::Devices};

pub mod models;
pub mod routes;

pub async fn start_http_server(
    db: DbPool,
    devices: Devices,
    app_handle: AppHandle,
) -> anyhow::Result<()> {
    let port = 58371;

    // build our application with a single route
    let app = routes::router()
        .layer(Extension(db))
        .layer(Extension(devices))
        .layer(Extension(app_handle))
        .layer(CorsLayer::very_permissive());

    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port));

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .context("failed to bind http server socket")?;
    axum::serve(listener, app)
        .await
        .context("error while serving")?;

    Ok(())
}
