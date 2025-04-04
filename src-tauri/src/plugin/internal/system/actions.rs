use serde::Deserialize;
use tauri_plugin_opener::open_url;

use crate::{
    database::{entity::tile::TileModel, DbPool},
    device::Devices,
    events::TileInteractionContext,
    plugin::Plugins,
};

#[derive(Deserialize)]
pub struct SystemWebsiteProperties {
    url: String,
}

pub async fn handle(
    devices: &Devices,
    plugins: &Plugins,
    db: &DbPool,
    context: TileInteractionContext,
    tile: TileModel,
) -> anyhow::Result<()> {
    match context.action_id.as_str() {
        "website" => {
            let data: SystemWebsiteProperties = serde_json::from_value(tile.config.properties)?;
            open_url(data.url, None::<&str>)?;
        }
        "open" => {}
        "close" => {}
        "text" => {}
        "multimedia" => {}
        action_id => {
            tracing::warn!(?action_id, ?context, "unknown internal action");
        }
    }

    Ok(())
}
