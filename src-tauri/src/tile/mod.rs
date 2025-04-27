use std::sync::Arc;

use anyhow::Context;
use tilepad_manifest::plugin::PluginId;

use crate::{
    database::{
        DbPool, JsonObject,
        entity::tile::{TileIcon, TileIconOptions, TileId, TileLabel, TileModel, UpdateKind},
    },
    device::Devices,
    icons::Icons,
};

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

    pub async fn get_tile_properties(
        &self,
        tile_id: TileId,
        plugin_id: Option<PluginId>,
    ) -> anyhow::Result<JsonObject> {
        let db = &self.db;
        let tile = TileModel::get_by_id(db, tile_id)
            .await?
            .context("tile not found")?;

        anyhow::ensure!(
            plugin_id.is_none_or(|plugin_id| tile.plugin_id == plugin_id),
            anyhow::anyhow!("tile is not apart of the same plugin")
        );

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
        let db = &self.db;
        let tile = TileModel::get_by_id(db, tile_id)
            .await?
            .context("tile not found")?;

        anyhow::ensure!(
            plugin_id.is_none_or(|plugin_id| tile.plugin_id == plugin_id),
            anyhow::anyhow!("tile is not apart of the same plugin")
        );

        let tile = tile.update_properties(db, properties, partial).await?;
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
        let db = &self.db;
        let tile = TileModel::get_by_id(db, tile_id)
            .await?
            .context("tile not found")?;

        anyhow::ensure!(
            plugin_id.is_none_or(|plugin_id| tile.plugin_id == plugin_id),
            anyhow::anyhow!("tile is not apart of the same plugin")
        );

        // Handle change in icon when using an uploaded icon (Remove the old file)
        self.icons
            .handle_tile_change_icon(&tile.config.icon)
            .await?;

        let tile = tile.update_icon(db, icon, kind).await?;
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
        let db = &self.db;
        let tile = TileModel::get_by_id(db, tile_id)
            .await?
            .context("tile not found")?;

        anyhow::ensure!(
            plugin_id.is_none_or(|plugin_id| tile.plugin_id == plugin_id),
            anyhow::anyhow!("tile is not apart of the same plugin")
        );

        let tile = tile.update_icon_options(db, icon_options).await?;
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
        let db = &self.db;
        let tile = TileModel::get_by_id(db, tile_id)
            .await?
            .context("tile not found")?;

        anyhow::ensure!(
            plugin_id.is_none_or(|plugin_id| tile.plugin_id == plugin_id),
            anyhow::anyhow!("tile is not apart of the same plugin")
        );

        let tile = tile.update_label(db, label, kind).await?;
        self.devices.background_update_folder(tile.folder_id);
        Ok(tile)
    }
}
