use super::{
    Migration,
    m202502251153_create_folders_table::{FoldersColumn, FoldersTable},
    schema::*,
};
use sea_query::{ForeignKey, ForeignKeyAction, IdenStatic, SqliteQueryBuilder, Table};

pub struct TilesMigration;

#[async_trait::async_trait]
impl Migration for TilesMigration {
    fn name(&self) -> &str {
        "m202502251225_create_tiles_table"
    }

    async fn up(&self, db: &crate::database::DbPool) -> anyhow::Result<()> {
        sqlx::query(
            &Table::create()
                .table(TilesTable)
                .if_not_exists()
                .col(pk_uuid(TilesColumn::Id))
                .col(json(TilesColumn::Config))
                .col(json(TilesColumn::Properties))
                .col(uuid(TilesColumn::FolderId))
                .col(string(TilesColumn::PluginId))
                .col(string(TilesColumn::ActionId))
                .col(integer(TilesColumn::Row))
                .col(integer(TilesColumn::Column))
                // Connect to profiles table
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_tiles_folder")
                        .from(TilesTable, TilesColumn::FolderId)
                        .to(FoldersTable, FoldersColumn::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .build(SqliteQueryBuilder),
        )
        .execute(db)
        .await?;

        Ok(())
    }
}

#[derive(IdenStatic, Copy, Clone)]
#[iden(rename = "tiles")]
pub struct TilesTable;

#[derive(IdenStatic, Copy, Clone)]
pub enum TilesColumn {
    /// Unique ID for the tile
    Id,
    /// Tile configuration (JSON)
    Config,
    /// Plugin properties for this tile
    Properties,
    /// ID of a folder this tile is within
    FolderId,
    /// ID of the plugin the action is apart of
    PluginId,
    /// ID of the action within the plugin to execute
    ActionId,
    /// Row the tile is on
    Row,
    /// Column the tile is on
    Column,
}
