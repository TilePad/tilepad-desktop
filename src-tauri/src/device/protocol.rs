use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    database::entity::{folder::FolderModel, tile::TileModel},
    events::DisplayContext,
};

/// Device message coming from the client side
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ClientDeviceMessage {
    /// Requests an access token and approval for a device
    RequestApproval {
        /// Name of the device
        name: String,
    },

    /// Request the current tiles
    RequestTiles,

    /// Authenticate using a device access token
    Authenticate {
        /// Access token for making requests from a device
        access_token: String,
    },

    /// User has clicked a tile
    TileClicked {
        /// ID of the tile that was touched
        tile_id: Uuid,
    },

    /// Got a message from a display
    RecvFromDisplay {
        ctx: DisplayContext,
        message: serde_json::Value,
    },
}

/// Device message coming from the server side
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum ServerDeviceMessage {
    /// Device access was denied
    Declined,

    /// Device access was approved
    Approved {
        /// Unique ID of the device
        device_id: Uuid,
        /// Device access token for future requests
        access_token: String,
    },

    /// Access was revoked
    Revoked,

    /// Device is authenticated
    Authenticated { device_id: Uuid },

    /// Provided access token was invalid
    InvalidAccessToken,

    /// Update the current tiles list
    Tiles {
        tiles: Vec<TileModel>,
        folder: FolderModel,
    },

    /// Got a message from the plugin
    RecvFromPlugin {
        ctx: DisplayContext,
        message: serde_json::Value,
    },
}
