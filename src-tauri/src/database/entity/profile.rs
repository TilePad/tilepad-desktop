use sea_query::{Expr, IdenStatic, Order, Query};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::database::{
    DbPool, DbResult,
    helpers::{UpdateStatementExt, sql_exec, sql_query_all, sql_query_maybe_one},
};

pub type ProfileId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProfileModel {
    pub id: ProfileId,
    pub name: String,
    pub default: bool,
    #[sqlx(json)]
    pub config: ProfileConfig,
    pub order: u32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ProfileConfig {}

#[derive(Deserialize)]
pub struct CreateProfile {
    pub name: String,
    pub default: bool,
    pub config: ProfileConfig,
    pub order: u32,
}

#[derive(Deserialize)]
pub struct UpdateProfile {
    pub name: Option<String>,
    pub default: Option<bool>,
    pub config: Option<ProfileConfig>,
    pub order: Option<u32>,
}

impl ProfileModel {
    /// Create a new profile
    pub async fn create(db: &DbPool, create: CreateProfile) -> anyhow::Result<ProfileModel> {
        let mut model = ProfileModel {
            id: Uuid::new_v4(),
            name: create.name,
            default: false,
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
                    model.default.into(),
                    config.into(),
                    model.order.into(),
                ]),
        )
        .await?;

        if create.default {
            model = model.set_default(db).await?;
        }

        Ok(model)
    }

    pub async fn get_by_id(db: &DbPool, id: ProfileId) -> DbResult<Option<ProfileModel>> {
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
                .and_where(Expr::col(ProfilesColumn::Id).eq(id)),
        )
        .await
    }

    pub async fn update(
        mut self,
        db: &DbPool,
        update: UpdateProfile,
    ) -> anyhow::Result<ProfileModel> {
        sql_exec(
            db,
            Query::update()
                .table(ProfilesTable)
                .and_where(Expr::col(ProfilesColumn::Id).eq(self.id))
                .cond_value(ProfilesColumn::Name, update.name.as_ref())
                .cond_value_json(ProfilesColumn::Config, update.config.as_ref())?
                .cond_value(ProfilesColumn::Order, update.order),
        )
        .await?;

        self.name = update.name.unwrap_or(self.name);
        self.config = update.config.unwrap_or(self.config);
        self.order = update.order.unwrap_or(self.order);

        if let Some(true) = update.default {
            self = self.set_default(db).await?;
        }

        Ok(self)
    }

    /// Set this profile as the default profile
    async fn set_default(mut self, db: &DbPool) -> DbResult<ProfileModel> {
        sql_exec(
            db,
            Query::update().table(ProfilesTable).value(
                ProfilesColumn::Default,
                Expr::case(Expr::col(ProfilesColumn::Id).eq(self.id), true).finally(false),
            ),
        )
        .await?;

        self.default = true;

        Ok(self)
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

    pub async fn all(db: &DbPool) -> DbResult<Vec<ProfileModel>> {
        sql_query_all(
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
                .order_by(ProfilesColumn::Order, Order::Asc),
        )
        .await
    }

    pub async fn delete(db: &DbPool, profile_id: ProfileId) -> DbResult<()> {
        sql_exec(
            db,
            Query::delete()
                .from_table(ProfilesTable)
                .and_where(Expr::col(ProfilesColumn::Id).eq(profile_id))
                .and_where(Expr::col(ProfilesColumn::Default).eq(false)),
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
