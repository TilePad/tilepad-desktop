use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database::entity::{folder::FolderModel, tile::TileModel};

/// Plugin message coming from the client side
#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ClientPluginMessage {}

/// Plugin message coming from the server side
#[derive(Serialize)]
#[serde(tag = "type")]
pub enum ServerPluginMessage {}
