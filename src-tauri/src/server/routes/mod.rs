use axum::{routing::get, Router};

mod plugins;

pub fn router() -> Router {
    Router::new().route("/", get(plugins::ws))
}
