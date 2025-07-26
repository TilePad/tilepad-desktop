use crate::{
    database::{
        DbPool,
        entity::{
            device::{CreateDevice, DeviceConfig, DeviceId, DeviceModel},
            folder::{FolderId, FolderModel},
            profile::{ProfileId, ProfileModel},
            tile::{TileId, TileModel},
        },
    },
    device::protocol::ServerDeviceMessageEncrypted,
    events::{
        AppEvent, AppEventSender, DeviceAppEvent, DeviceRequestAppEvent, DisplayContext,
        TileInteractionContext,
    },
    plugin::Plugins,
    utils::encryption::ServerKeyPair,
};
use anyhow::Context;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use session::{DeviceSessionId, DeviceSessionRef};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use uuid::Uuid;

pub mod protocol;
pub mod session;

pub type DeviceRequestId = Uuid;

/// Store for device sessions and requests
pub struct Devices {
    /// Sender for app events
    event_tx: AppEventSender,

    /// Access to the database
    db: DbPool,

    /// Access to the plugins registry
    plugins: Arc<Plugins>,

    /// Current device socket sessions
    sessions: RwLock<HashMap<DeviceSessionId, DeviceSessionRef>>,

    /// Current requests for authorization from devices
    requests: RwLock<Vec<DeviceRequest>>,

    /// Server key pair
    server_key_pair: ServerKeyPair,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedDevice {
    pub device_id: DeviceId,
    pub session_id: DeviceSessionId,
}

#[derive(Clone, Serialize)]
pub struct DeviceRequest {
    /// Unique ID for the request itself
    id: DeviceRequestId,
    /// Address of the connecting device
    socket_addr: SocketAddr,
    /// ID of the session the request is for
    session_id: DeviceSessionId,
    /// Name of the device requesting approval
    device_name: String,
    /// Client public key
    client_public_key: [u8; 32],
}

impl Devices {
    pub fn new(
        event_tx: AppEventSender,
        db: DbPool,
        plugins: Arc<Plugins>,
        server_key_pair: ServerKeyPair,
    ) -> Self {
        Self {
            event_tx,
            db,
            plugins,
            server_key_pair,

            sessions: Default::default(),
            requests: Default::default(),
        }
    }

    /// Insert a new session
    pub fn insert_session(&self, session_id: DeviceSessionId, session_ref: DeviceSessionRef) {
        self.sessions.write().insert(session_id, session_ref);
    }

    /// Insert a new session
    pub fn get_session(&self, session_id: &DeviceSessionId) -> Option<DeviceSessionRef> {
        self.sessions.write().get(session_id).cloned()
    }

    /// Get all devices that have active sessions
    pub fn get_connected_devices(&self) -> Vec<ConnectedDevice> {
        self.sessions
            .read()
            .iter()
            .filter_map(|(session_id, session_ref)| {
                let device_id = session_ref.get_device_id()?;
                Some(ConnectedDevice {
                    device_id,
                    session_id: *session_id,
                })
            })
            .collect()
    }

    /// Get all device IDs that have active sessions
    pub fn get_connected_device_ids(&self) -> Vec<DeviceId> {
        self.sessions
            .read()
            .iter()
            .filter_map(|(_, session_ref)| {
                let device_id = session_ref.get_device_id()?;
                Some(device_id)
            })
            .collect()
    }

    /// Remove a session
    pub fn remove_session(&self, session_id: DeviceSessionId, device_id: Option<DeviceId>) {
        self.sessions.write().remove(&session_id);
        self.remove_session_device_requests(session_id);

        if let Some(device_id) = device_id {
            _ = self
                .event_tx
                .send(AppEvent::Device(DeviceAppEvent::Disconnected { device_id }));
        }
    }

    /// Removes any device requests associated with the provided session
    /// (Notifying by sending an event)
    pub fn remove_session_device_requests(&self, session_id: DeviceSessionId) {
        self.requests.write().retain(|request| {
            if request.session_id == session_id {
                _ = self
                    .event_tx
                    .send(AppEvent::DeviceRequest(DeviceRequestAppEvent::Removed {
                        request_id: request.id,
                    }));
                false
            } else {
                true
            }
        });
    }

    /// Consume a device request by ID
    pub fn take_device_request(&self, request_id: DeviceRequestId) -> Option<DeviceRequest> {
        let requests = &mut *self.requests.write();
        let index = requests
            .iter()
            .position(|request| request.id == request_id)?;

        let request = requests.swap_remove(index);
        Some(request)
    }

    /// Add a new device request
    pub fn add_device_request(
        &self,
        session_id: DeviceSessionId,
        socket_addr: SocketAddr,
        device_name: String,
        client_public_key: [u8; 32],
    ) {
        self.remove_session_device_requests(session_id);

        let request_id = Uuid::new_v4();
        self.requests.write().push(DeviceRequest {
            id: request_id,
            socket_addr,
            session_id,
            device_name,
            client_public_key,
        });

        _ = self
            .event_tx
            .send(AppEvent::DeviceRequest(DeviceRequestAppEvent::Added {
                request_id,
            }));
    }

    pub fn get_device_requests(&self) -> Vec<DeviceRequest> {
        self.requests.read().clone()
    }

    /// Approve a device request, creates a new device in the database
    pub async fn approve_device_request(&self, request_id: DeviceRequestId) -> anyhow::Result<()> {
        let db = &self.db;
        let default_profile = ProfileModel::get_default_profile(db)
            .await?
            .context("no default profile")?;
        let default_folder = FolderModel::get_default(db, default_profile.id)
            .await?
            .context("no default folder")?;

        let request = self
            .take_device_request(request_id)
            .context("request not found")?;
        let session = self
            .get_session(&request.session_id)
            .context("session not found")?;

        let device = DeviceModel::create(
            db,
            CreateDevice {
                name: request.device_name,
                public_key: request.client_public_key.to_vec(),
                config: DeviceConfig {},
                profile_id: default_folder.profile_id,
                folder_id: default_folder.id,
            },
        )
        .await?;

        session.on_approved(device.id);

        // Notify frontend
        _ = self
            .event_tx
            .send(AppEvent::DeviceRequest(DeviceRequestAppEvent::Accepted {
                request_id,
            }));

        _ = self
            .event_tx
            .send(AppEvent::Device(DeviceAppEvent::Authenticated {
                device_id: device.id,
            }));

        Ok(())
    }

    // Decline a device request
    pub fn decline_device_request(&self, request_id: DeviceRequestId) -> anyhow::Result<()> {
        let request = self
            .take_device_request(request_id)
            .context("request not found")?;
        let session = self
            .get_session(&request.session_id)
            .context("session not found")?;

        session.decline();

        _ = self
            .event_tx
            .send(AppEvent::DeviceRequest(DeviceRequestAppEvent::Decline {
                request_id,
            }));

        Ok(())
    }

    /// Find a session from its device ID
    pub fn get_session_by_device(&self, device_id: DeviceId) -> Option<DeviceSessionRef> {
        self.sessions.read().values().find_map(|session_ref| {
            let session_device_id = session_ref.get_device_id()?;
            if session_device_id != device_id {
                return None;
            }

            Some(session_ref.clone())
        })
    }

    /// Attempt to authenticate a session using a access token
    pub async fn attempt_authenticate_device(
        &self,
        public_key: &[u8],
    ) -> anyhow::Result<Option<DeviceId>> {
        let mut device = match DeviceModel::get_by_public_key(&self.db, public_key).await? {
            Some(value) => value,
            None => return Ok(None),
        };

        // Update last connected
        device.set_connected_now(&self.db).await?;

        // Notify frontend
        _ = self
            .event_tx
            .send(AppEvent::Device(DeviceAppEvent::Authenticated {
                device_id: device.id,
            }));

        Ok(Some(device.id))
    }

    /// Revoke access for a device
    pub async fn revoke_device(&self, device_id: DeviceId) -> anyhow::Result<()> {
        DeviceModel::delete(&self.db, device_id).await?;

        _ = self
            .event_tx
            .send(AppEvent::Device(DeviceAppEvent::Revoked { device_id }));

        // Tell the session its been revoked
        if let Some(session) = self.get_session_by_device(device_id) {
            session.revoke();
        }

        Ok(())
    }

    pub async fn request_device_tiles(
        &self,
        device_id: DeviceId,
    ) -> anyhow::Result<(FolderModel, Vec<TileModel>)> {
        let db = &self.db;
        let device = DeviceModel::get_by_id(db, device_id)
            .await?
            .context("device not found")?;

        let folder = FolderModel::get_by_id(db, device.folder_id)
            .await?
            .context("folder not found")?;

        let tiles = TileModel::get_by_folder(db, device.folder_id).await?;
        Ok((folder, tiles))
    }

    pub async fn update_device_profile(
        &self,
        device_id: DeviceId,
        profile_id: ProfileId,
    ) -> anyhow::Result<()> {
        let db = &self.db;
        let device = DeviceModel::get_by_id(db, device_id)
            .await?
            .context("device not found")?;
        let folder = FolderModel::get_default(db, profile_id)
            .await?
            .context("unknown folder")?;
        let tiles = TileModel::get_by_folder(db, folder.id).await?;

        // Update the profile on the device
        device.set_profile(db, profile_id, folder.id).await?;

        if let Some(session) = self.get_session_by_device(device_id) {
            session.send_encrypted_message(ServerDeviceMessageEncrypted::Tiles { tiles, folder });
        }

        Ok(())
    }

    pub async fn update_device_folder(
        &self,
        device_id: DeviceId,
        folder_id: FolderId,
    ) -> anyhow::Result<()> {
        let db = &self.db;
        let device = DeviceModel::get_by_id(db, device_id)
            .await?
            .context("device not found")?;
        let folder = FolderModel::get_by_id(db, folder_id)
            .await?
            .context("unknown folder")?;
        let tiles = TileModel::get_by_folder(db, folder.id).await?;

        device.set_profile(db, folder.profile_id, folder_id).await?;

        // Inform plugins of the new tile sets
        self.plugins.set_device_tiles(device_id, &tiles)?;

        // Inform the device of its new tile set
        if let Some(session) = self.get_session_by_device(device_id) {
            session.send_encrypted_message(ServerDeviceMessageEncrypted::Tiles { tiles, folder });
        }

        Ok(())
    }

    /// Spawn a background task to update the provided folder on all devices
    pub fn background_update_folder(self: &Arc<Self>, folder_id: FolderId) {
        let devices = self.clone();
        tokio::spawn(async move {
            _ = devices.update_folder_devices(folder_id).await;
        });
    }

    /// Updates the tiles on all devices that are using the
    /// provided `folder_id` folder
    pub async fn update_folder_devices(&self, folder_id: FolderId) -> anyhow::Result<()> {
        let db = &self.db;
        let devices = DeviceModel::all_by_folder(db, folder_id).await?;

        // No devices to update
        if devices.is_empty() {
            return Ok(());
        }

        let folder = FolderModel::get_by_id(db, folder_id)
            .await?
            .context("folder not found")?;

        let tiles = TileModel::get_by_folder(db, folder_id).await?;

        devices
            .iter()
            .filter_map(|device| self.get_session_by_device(device.id))
            .for_each(|session| {
                _ = session.send_encrypted_message(ServerDeviceMessageEncrypted::Tiles {
                    tiles: tiles.clone(),
                    folder: folder.clone(),
                });
            });

        Ok(())
    }

    /// Handle sending a display message from a plugin to its destination
    pub async fn handle_plugin_message(
        &self,
        ctx: DisplayContext,
        message: serde_json::Value,
    ) -> anyhow::Result<()> {
        if let Some(session) = self.get_session_by_device(ctx.device_id) {
            _ = session.send_encrypted_message(ServerDeviceMessageEncrypted::RecvFromPlugin {
                ctx: ctx.clone(),
                message: message.clone(),
            });
        }

        Ok(())
    }

    pub async fn device_execute_tile(
        &self,
        device_id: DeviceId,
        tile_id: TileId,
    ) -> anyhow::Result<()> {
        let tile = TileModel::get_by_id(&self.db, tile_id)
            .await?
            .context("tile instance not found")?;

        let context = TileInteractionContext {
            device_id,
            plugin_id: tile.plugin_id,
            action_id: tile.action_id,
            tile_id,
        };

        self.plugins
            .handle_action(self, context, tile.properties)
            .await?;

        Ok(())
    }
}
