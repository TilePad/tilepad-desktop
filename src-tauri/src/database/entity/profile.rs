use sea_query::IdenStatic;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProfileModel {}

impl ProfileModel {}

#[derive(IdenStatic, Copy, Clone)]
#[iden(rename = "items")]
pub struct ItemsTable;

#[derive(IdenStatic, Copy, Clone)]
pub enum ItemsColumn {
    Id,
    Name,
    Config,
    Order,
    CreatedAt,
}

#[derive(IdenStatic, Copy, Clone)]
#[iden(rename = "items_sounds")]
pub struct ItemsSoundsTable;

#[derive(IdenStatic, Copy, Clone)]
pub enum ItemsSoundsColumn {
    ItemId,
    SoundId,
    SoundType,
}
