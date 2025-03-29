use axum::{routing::get, Router};

mod devices;
mod icons;
mod plugins;
mod server;

pub fn router() -> Router {
    Router::new()
        .nest(
            "/server",
            Router::new().route_service("/details", get(server::details)),
        )
        .nest(
            "/plugins",
            Router::new().route("/ws", get(plugins::ws)).nest(
                "/{plugin_id}",
                Router::new().route("/assets/{*path}", get(plugins::get_plugin_file)),
            ),
        )
        .nest(
            "/icons",
            Router::new().nest(
                "/{pack_id}",
                Router::new().route("/assets/{*path}", get(icons::get_icon_file)),
            ),
        )
        .nest("/devices", Router::new().route("/ws", get(devices::ws)))
}
