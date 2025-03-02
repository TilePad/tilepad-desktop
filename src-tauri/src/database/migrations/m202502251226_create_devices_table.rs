use super::{schema::*, Migration};
use sea_query::{IdenStatic, SqliteQueryBuilder, Table};

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
                .col(date_time(DevicesColumn::CreatedAt))
                .col(date_time(DevicesColumn::LastConnectedAt))
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
    /// Timestamp of when the device was first approved
    CreatedAt,
    /// Timestamp of when the device last connected
    LastConnectedAt,
}
