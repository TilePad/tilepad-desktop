use sea_query::IdenStatic;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

pub type ProfileId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProfileModel {}

impl ProfileModel {}
