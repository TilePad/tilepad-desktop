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
        "m202502251153_create_folders_table"
    }

    async fn up(&self, db: &crate::database::DbPool) -> anyhow::Result<()> {
        sqlx::query(
            &Table::create()
                .table(FoldersTable)
                .if_not_exists()
                .col(pk_uuid(FoldersColumn::Id))
                .col(string(FoldersColumn::Name))
                .col(json(FoldersColumn::Config))
                .col(uuid(FoldersColumn::ProfileId))
                .col(date_time(FoldersColumn::CreatedAt))
                .col(boolean(FoldersColumn::Default))
                // Connect to profiles table
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_folders_profile")
                        .from(FoldersTable, FoldersColumn::ProfileId)
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
#[iden(rename = "folders")]
pub struct FoldersTable;

#[derive(IdenStatic, Copy, Clone)]
pub enum FoldersColumn {
    /// Unique ID for the folder
    Id,
    /// Name of the folder
    Name,
    /// Folder config (JSON)
    Config,
    /// Profile the folder belongs to
    ProfileId,
    /// Whether the folder is the default for the profile
    Default,
    /// When the profile was created
    CreatedAt,
}
