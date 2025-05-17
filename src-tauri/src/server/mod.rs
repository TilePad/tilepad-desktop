use axum::Extension;
use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    sync::Arc,
};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use crate::{
    database::DbPool, device::Devices, fonts::Fonts, icons::Icons, plugin::Plugins, tile::Tiles,
};

pub mod extractors;
pub mod http_content;
pub mod models;
pub mod routes;

#[derive(Clone, Copy)]
pub struct ServerPort(pub u16);

pub async fn create_http_socket(port: u16) -> std::io::Result<TcpListener> {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port));
    TcpListener::bind(addr).await
}

pub async fn start_http_server(
    listener: TcpListener,
    db: DbPool,
    devices: Arc<Devices>,
    plugins: Arc<Plugins>,
    icons: Arc<Icons>,
    tiles: Arc<Tiles>,
    fonts: Arc<Fonts>,
) {
    // Create router and attach extensions
    let app = routes::router()
        .layer(Extension(db))
        .layer(Extension(devices))
        .layer(Extension(plugins))
        .layer(Extension(icons))
        .layer(Extension(tiles))
        .layer(Extension(fonts))
        .layer(CorsLayer::very_permissive())
        .into_make_service_with_connect_info::<SocketAddr>();

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
