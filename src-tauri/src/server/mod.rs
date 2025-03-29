use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use anyhow::Context;
use axum::Extension;
use tauri::AppHandle;
use tower_http::cors::CorsLayer;

use crate::{database::DbPool, device::Devices, icons::Icons, plugin::Plugins};

pub mod models;
pub mod routes;

pub const HTTP_PORT: u16 = 59371;

pub async fn start_http_server(
    db: DbPool,
    devices: Devices,
    app_handle: AppHandle,
    plugins: Plugins,
    icons: Icons,
) -> anyhow::Result<()> {
    // build our application with a single route
    let app = routes::router()
        .layer(Extension(db))
        .layer(Extension(devices))
        .layer(Extension(app_handle))
        .layer(Extension(plugins))
        .layer(Extension(icons))
        .layer(CorsLayer::very_permissive())
        .into_make_service_with_connect_info::<SocketAddr>();

    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, HTTP_PORT));

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .context("failed to bind http server socket")?;
    axum::serve(listener, app)
        .await
        .context("error while serving")?;

    Ok(())
}
