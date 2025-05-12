use crate::{
    database::{
        DbPool, JsonObject,
        entity::tile::{
            TileIcon, TileIconOptions, TileId, TileLabel, TileModel, TilePosition, UpdateKind,
        },
    },
    device::Devices,
    icons::Icons,
};
use anyhow::Context;
use std::sync::Arc;
use tilepad_manifest::plugin::PluginId;

pub struct Tiles {
    /// Access to the database
    db: DbPool,

    /// Access to icons
    icons: Arc<Icons>,

    /// Access to devices
    devices: Arc<Devices>,
}

impl Tiles {
    pub fn new(db: DbPool, icons: Arc<Icons>, devices: Arc<Devices>) -> Self {
        Self { db, icons, devices }
    }

    /// Requests a tile by ID, optionally including a `plugin_id` of the
    /// plugin requesting the tile, when specified `plugin_id` will be
    /// used to enforce access control
    async fn get_tile(
        &self,
        tile_id: TileId,
        plugin_id: Option<PluginId>,
    ) -> anyhow::Result<TileModel> {
        let tile = TileModel::get_by_id(&self.db, tile_id)
            .await?
            .context("tile not found")?;

        anyhow::ensure!(
            plugin_id.is_none_or(|plugin_id| tile.plugin_id == plugin_id),
            anyhow::anyhow!("tile is not apart of the same plugin")
        );

        Ok(tile)
    }

    /// Request the properties for a specific tile
    pub async fn get_tile_properties(
        &self,
        tile_id: TileId,
        plugin_id: Option<PluginId>,
    ) -> anyhow::Result<JsonObject> {
        let tile = self.get_tile(tile_id, plugin_id).await?;
        Ok(tile.properties)
    }

    /// Update a specific tile properties
    pub async fn update_tile_properties(
        &self,
        tile_id: TileId,
        plugin_id: Option<PluginId>,
        properties: JsonObject,
        partial: bool,
    ) -> anyhow::Result<TileModel> {
        let tile = self.get_tile(tile_id, plugin_id).await?;
        let tile = tile
            .update_properties(&self.db, properties, partial)
            .await?;
        self.devices.background_update_folder(tile.folder_id);
        Ok(tile)
    }

    /// Update a specific tile icons
    pub async fn update_tile_icon(
        &self,
        tile_id: TileId,
        plugin_id: Option<PluginId>,
        icon: TileIcon,
        kind: UpdateKind,
    ) -> anyhow::Result<TileModel> {
        let tile = self.get_tile(tile_id, plugin_id).await?;

        // Handle change in icon when using an uploaded icon (Remove the old file)
        self.icons
            .handle_tile_change_icon(&tile.config.icon)
            .await?;

        let tile = tile.update_icon(&self.db, icon, kind).await?;
        self.devices.background_update_folder(tile.folder_id);
        Ok(tile)
    }

    /// Update a specific tile icons
    pub async fn update_tile_icon_options(
        &self,
        tile_id: TileId,
        plugin_id: Option<PluginId>,
        icon_options: TileIconOptions,
    ) -> anyhow::Result<TileModel> {
        let tile = self.get_tile(tile_id, plugin_id).await?;
        let tile = tile.update_icon_options(&self.db, icon_options).await?;
        self.devices.background_update_folder(tile.folder_id);
        Ok(tile)
    }

    /// Update a specific tile label
    pub async fn update_tile_label(
        &self,
        tile_id: TileId,
        plugin_id: Option<PluginId>,
        label: TileLabel,
        kind: UpdateKind,
    ) -> anyhow::Result<TileModel> {
        let tile = self.get_tile(tile_id, plugin_id).await?;
        let tile = tile.update_label(&self.db, label, kind).await?;
        self.devices.background_update_folder(tile.folder_id);
        Ok(tile)
    }

    /// Change the row and column position of a tile
    pub async fn update_tile_position(
        &self,
        tile_id: TileId,
        plugin_id: Option<PluginId>,
        position: TilePosition,
    ) -> anyhow::Result<TileModel> {
        let tile = self.get_tile(tile_id, plugin_id).await?;
        let tile = tile.update_position(&self.db, position).await?;
        self.devices.background_update_folder(tile.folder_id);
        Ok(tile)
    }

    /// Get all tiles that are currently visible
    pub async fn get_visible_tiles(&self, plugin_id: PluginId) -> anyhow::Result<Vec<TileModel>> {
        // Load all connected devices
        let device_ids = self.devices.get_connected_device_ids();

        // Get all tiles linked to the devices that are for the plugin
        let tiles =
            TileModel::get_by_from_devices_by_plugin(&self.db, &device_ids, plugin_id).await?;

        Ok(tiles)
    }
}
