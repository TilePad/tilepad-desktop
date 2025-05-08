use axum::{
    Router,
    routing::{get, post},
};

mod dev;
mod devices;
mod fonts;
mod icons;
mod plugins;
mod server;

pub fn router() -> Router {
    Router::new()
        .nest(
            "/server",
            Router::new().route("/details", get(server::details)),
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
        .nest(
            "/uploaded-icons",
            Router::new().route("/{*path}", get(icons::get_uploaded_icon_file)),
        )
        .nest("/devices", Router::new().route("/ws", get(devices::ws)))
        .nest(
            "/fonts",
            Router::new().route("/{family}", get(fonts::get_font_file)),
        )
        .nest(
            "/dev",
            Router::new().route("/reload_plugins", post(dev::reload_plugins)),
        )
}
