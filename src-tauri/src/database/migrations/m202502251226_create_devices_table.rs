use super::{
    m202502251151_create_profiles_table::{ProfilesColumn, ProfilesTable},
    schema::*,
    Migration,
};
use sea_query::{ForeignKey, ForeignKeyAction, IdenStatic, SqliteQueryBuilder, Table};

pub struct ProfilesMigration;

#[async_trait::async_trait]
impl Migration for ProfilesMigration {
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
                .col(boolean(DevicesColumn::Connected))
                .col(string(DevicesColumn::AccessToken))
                .col(json(DevicesColumn::Config))
                .col(integer(DevicesColumn::Order).default(0))
                .col(uuid(DevicesColumn::ProfileId).null())
                .col(date_time(DevicesColumn::CreatedAt))
                .col(date_time(DevicesColumn::LastConnectedAt))
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_devices_profile")
                        .from(DevicesTable, DevicesColumn::ProfileId)
                        .to(ProfilesTable, ProfilesColumn::Id)
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
#[iden(rename = "devices")]
pub struct DevicesTable;

#[derive(IdenStatic, Copy, Clone)]
pub enum DevicesColumn {
    /// Unique ID for the device
    Id,
    /// Name of the device
    Name,
    /// Whether the device is connected
    Connected,
    /// Access token for the device
    AccessToken,
    /// Additional device configuration
    Config,
    /// Order of the device in the UI
    Order,
    /// Current profile on the device
    ProfileId,
    /// Timestamp of when the device was first approved
    CreatedAt,
    /// Timestamp of when the device last connected
    LastConnectedAt,
}
