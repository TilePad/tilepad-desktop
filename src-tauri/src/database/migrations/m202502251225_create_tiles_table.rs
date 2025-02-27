use super::{
    m202502251151_create_profiles_table::{ProfilesColumn, ProfilesTable},
    m202502251153_create_folders_table::{FoldersColumn, FoldersTable},
    m202502251226_create_devices_table::DevicesTable,
    schema::*,
    Migration,
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
                .col(uuid_null(TilesColumn::FolderId))
                .col(integer(TilesColumn::Row))
                .col(integer(TilesColumn::Column))
                .col(date_time(TilesColumn::CreatedAt))
                // Connect to profiles table
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_tiles_folder")
                        .from(DevicesTable, TilesColumn::FolderId)
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
    /// ID of a folder this tile is within
    FolderId,
    /// Row the tile is on
    Row,
    /// Column the tile is on
    Column,
    /// When the profile was created
    CreatedAt,
}
