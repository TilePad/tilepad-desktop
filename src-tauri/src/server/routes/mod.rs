use axum::{routing::get, Router};

mod devices;
mod plugins;
mod server;

pub fn router() -> Router {
    Router::new()
        .nest(
            "/server",
            Router::new().route_service("/details", get(server::details)),
        )
        .nest("/plugins", Router::new().route("/ws", get(plugins::ws)))
        .nest("/devices", Router::new().route("/ws", get(devices::ws)))
}
