use axum::{routing::get, Router};

mod devices;
mod plugins;

pub fn router() -> Router {
    Router::new()
        .nest("/plugins", Router::new().route("/ws", get(plugins::ws)))
        .nest("/devices", Router::new().route("/ws", get(devices::ws)))
}
