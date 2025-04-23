use crate::{
    database::{
        JsonObject,
        entity::{folder::FolderId, profile::ProfileId},
    },
    device::Devices,
    events::TileInteractionContext,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SwitchFolderProperties {
    folder: FolderId,
}

#[derive(Deserialize)]
pub struct SwitchProfileProperties {
    profile: ProfileId,
}

pub async fn handle(
    devices: &Devices,
    context: TileInteractionContext,
    properties: JsonObject,
) -> anyhow::Result<()> {
    match context.action_id.as_str() {
        "switch_folder" => {
            let data: SwitchFolderProperties =
                serde_json::from_value(serde_json::Value::Object(properties))?;
            devices
                .update_device_folder(context.device_id, data.folder)
                .await?;
        }
        "switch_profile" => {
            let data: SwitchProfileProperties =
                serde_json::from_value(serde_json::Value::Object(properties))?;
            devices
                .update_device_profile(context.device_id, data.profile)
                .await?;
        }
        action_id => {
            tracing::warn!(?action_id, ?context, "unknown internal action");
        }
    }
    Ok(())
}
