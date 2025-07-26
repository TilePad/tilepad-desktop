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
    /// Authenticate using a device access token
    InitiateHandshake {
        /// Name of the device
        name: String,

        /// Client public key to authenticate using
        public_key: [u8; 32],
    },

    /// Response to an authentication challenge
    AuthenticateChallengeResponse {
        /// Challenge value encrypted using the clients private key
        challenge: Vec<u8>,
        /// Nonce for the message
        nonce: [u8; 24],
    },

    /// Encrypted message
    Encrypted {
        /// Encrypted message
        message: Vec<u8>,

        /// Nonce for the message
        nonce: [u8; 24],
    },
}

/// Device message coming from the server side
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum ServerDeviceMessage {
    /// Post "InitiateHandshake" the server will send back
    /// a challenge to ensure the client actually owns the private key to the
    /// public key it specified
    AuthenticateChallenge {
        /// Public key of the server
        server_public_key: [u8; 32],

        /// Encrypted challenge
        challenge: Vec<u8>,

        /// Nonce for the message
        nonce: [u8; 24],
    },

    /// Encrypted message from the server
    EncryptedMessage {
        /// The encrypted message
        message: Vec<u8>,
        /// Nonce for the message
        nonce: [u8; 24],
    },

    /// Error occurred
    Error { message: String },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ClientDeviceMessageEncrypted {
    /// Request the current tiles
    RequestTiles,

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

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum ServerDeviceMessageEncrypted {
    /// Device is not yet approved and approval has been requested
    ApprovalRequested,

    /// Device access was denied
    Declined,

    /// Device access was approved
    Approved {
        /// Unique ID of the device
        device_id: Uuid,
    },

    /// Access was revoked
    Revoked,

    /// Device is authenticated
    Authenticated { device_id: Uuid },

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
