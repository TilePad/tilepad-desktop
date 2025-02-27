use super::{schema::*, Migration};
use sea_query::{IdenStatic, SqliteQueryBuilder, Table};

pub struct ProfilesMigration;

#[async_trait::async_trait]
impl Migration for ProfilesMigration {
    fn name(&self) -> &str {
        "m202502251151_create_profiles_table"
    }

    async fn up(&self, db: &crate::database::DbPool) -> anyhow::Result<()> {
        sqlx::query(
            &Table::create()
                .table(ProfilesTable)
                .if_not_exists()
                .col(pk_uuid(ProfilesColumn::Id))
                .col(string(ProfilesColumn::Name))
                .col(boolean(ProfilesColumn::Active))
                .col(boolean(ProfilesColumn::Default))
                .col(json(ProfilesColumn::Config))
                .col(integer(ProfilesColumn::Order).default(0))
                .col(date_time(ProfilesColumn::CreatedAt))
                .build(SqliteQueryBuilder),
        )
        .execute(db)
        .await?;

        Ok(())
    }
}

#[derive(IdenStatic, Copy, Clone)]
#[iden(rename = "profiles")]
pub struct ProfilesTable;

#[derive(IdenStatic, Copy, Clone)]
pub enum ProfilesColumn {
    /// Unique ID for the profile
    Id,
    /// Name of the profile
    Name,
    /// Whether the profile is the active profile
    Active,
    /// Whether the profile is the default profile
    Default,
    /// Profile configuration (JSON)
    Config,
    /// Order position of the profile
    Order,
    /// When the profile was created
    CreatedAt,
}
