use super::{
    Migration,
    m202502251151_create_profiles_table::{ProfilesColumn, ProfilesTable},
    m202502251153_create_folders_table::{FoldersColumn, FoldersTable},
    schema::*,
};
use sea_query::{ForeignKey, ForeignKeyAction, IdenStatic, SqliteQueryBuilder, Table};

pub struct DevicesMigration;

#[async_trait::async_trait]
impl Migration for DevicesMigration {
    fn name(&self) -> &str {
        "m202502251226_create_devices_table"
    }

    async fn up(&self, db: &crate::database::DbPool) -> anyhow::Result<()> {
        sqlx::query(
            &Table::create()
                .table(DevicesTable)
                .if_not_exists()
                .col(pk_uuid(DevicesColumn::Id))
                .col(string(DevicesColumn::Name))
                .col(string(DevicesColumn::AccessToken))
                .col(json(DevicesColumn::Config))
                .col(integer(DevicesColumn::Order).default(0))
                .col(integer(DevicesColumn::ProfileId))
                .col(integer(DevicesColumn::FolderId))
                .col(date_time(DevicesColumn::CreatedAt))
                .col(date_time(DevicesColumn::LastConnectedAt))
                // Connect to folders table
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_device_folder")
                        .from(DevicesTable, DevicesColumn::FolderId)
                        .to(FoldersTable, FoldersColumn::Id)
                        .on_delete(ForeignKeyAction::Restrict)
                        .on_update(ForeignKeyAction::Cascade),
                )
                // Connect to profiles table
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_device_profile")
                        .from(DevicesTable, DevicesColumn::ProfileId)
                        .to(ProfilesTable, ProfilesColumn::Id)
                        .on_delete(ForeignKeyAction::Restrict)
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
#[iden(rename = "devices")]
pub struct DevicesTable;

#[derive(IdenStatic, Copy, Clone)]
pub enum DevicesColumn {
    /// Unique ID for the device
    Id,
    /// Name of the device
    Name,
    /// Access token for the device
    AccessToken,
    /// Additional device configuration
    Config,
    /// Order of the device in the UI
    Order,
    /// Current profile the device is using
    ProfileId,
    /// Current folder the device is using
    FolderId,
    /// Timestamp of when the device was first approved
    CreatedAt,
    /// Timestamp of when the device last connected
    LastConnectedAt,
}
