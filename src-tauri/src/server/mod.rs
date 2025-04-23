use axum::Extension;
use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    sync::Arc,
};
use tauri::AppHandle;
use tower_http::cors::CorsLayer;

use crate::{database::DbPool, device::Devices, icons::Icons, plugin::Plugins, tile::Tiles};

pub mod models;
pub mod routes;

pub const HTTP_PORT: u16 = 59371;

pub async fn start_http_server(
    db: DbPool,
    devices: Arc<Devices>,
    app_handle: AppHandle,
    plugins: Arc<Plugins>,
    icons: Arc<Icons>,
    tiles: Arc<Tiles>,
) {
    // build our application with a single route
    let app = routes::router()
        .layer(Extension(db))
        .layer(Extension(devices))
        .layer(Extension(app_handle))
        .layer(Extension(plugins))
        .layer(Extension(icons))
        .layer(Extension(tiles))
        .layer(CorsLayer::very_permissive())
        .into_make_service_with_connect_info::<SocketAddr>();

    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, HTTP_PORT));

    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(value) => value,
        Err(cause) => {
            tracing::error!(?cause, "failed to bind http server socket");
            return;
        }
    };

    if let Err(cause) = axum::serve(listener, app)
        // Attach graceful shutdown to the shutdown receiver
        .with_graceful_shutdown(async move {
            _ = tokio::signal::ctrl_c().await;
        })
        .await
    {
        tracing::error!(?cause, "error while serving http server");
    }
}
