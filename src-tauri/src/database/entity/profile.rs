use sea_query::{Expr, IdenStatic, Query};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::database::{
    helpers::{sql_exec, sql_query_maybe_one},
    DbPool, DbResult,
};

pub type ProfileId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProfileModel {
    pub id: ProfileId,
    pub name: String,
    pub active: bool,
    pub default: bool,
    #[sqlx(json)]
    pub config: ProfileConfig,
    pub order: u32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ProfileConfig {}

pub struct CreateProfile {
    pub name: String,
    pub default: bool,
    pub config: ProfileConfig,
    pub order: u32,
}

impl ProfileModel {
    /// Create a new profile
    pub async fn create(db: &DbPool, create: CreateProfile) -> anyhow::Result<ProfileModel> {
        let model = ProfileModel {
            id: Uuid::new_v4(),
            name: create.name,
            active: false,
            default: create.default,
            config: create.config,
            order: create.order,
        };

        let config = serde_json::to_value(&model.config)?;

        sql_exec(
            db,
            Query::insert()
                .into_table(ProfilesTable)
                .columns([
                    ProfilesColumn::Id,
                    ProfilesColumn::Name,
                    ProfilesColumn::Default,
                    ProfilesColumn::Config,
                    ProfilesColumn::Order,
                ])
                .values_panic([
                    model.id.into(),
                    model.name.clone().into(),
                    model.active.into(),
                    model.default.into(),
                    config.into(),
                    model.order.into(),
                ]),
        )
        .await?;

        Ok(model)
    }

    /// Set this profile as the default profile
    pub async fn set_default(&mut self, db: &DbPool) -> DbResult<()> {
        sql_exec(
            db,
            Query::update().table(ProfilesTable).value(
                ProfilesColumn::Default,
                Expr::case(Expr::col(ProfilesColumn::Id).eq(self.id), true).finally(false),
            ),
        )
        .await?;

        self.default = true;

        Ok(())
    }

    /// Get the first profile marked as default
    pub async fn get_default_profile(db: &DbPool) -> DbResult<Option<ProfileModel>> {
        sql_query_maybe_one(
            db,
            Query::select()
                .from(ProfilesTable)
                .columns([
                    ProfilesColumn::Id,
                    ProfilesColumn::Name,
                    ProfilesColumn::Default,
                    ProfilesColumn::Config,
                    ProfilesColumn::Order,
                ])
                .and_where(Expr::col(ProfilesColumn::Default).eq(true)),
        )
        .await
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
    /// Whether the profile is the default profile
    Default,
    /// Profile configuration (JSON)
    Config,
    /// Order position of the profile
    Order,
}
