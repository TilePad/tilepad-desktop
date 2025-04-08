use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use anyhow::Context;
use parking_lot::RwLock;
use protocol::ServerDeviceMessage;
use serde::{Deserialize, Serialize};
use session::{DeviceSessionId, DeviceSessionRef};
use uuid::Uuid;

use crate::{
    database::{
        entity::{
            device::{CreateDevice, DeviceConfig, DeviceId, DeviceModel},
            folder::{FolderId, FolderModel},
            profile::ProfileModel,
            tile::{TileId, TileModel},
        },
        DbPool,
    },
    events::{
        AppEvent, AppEventSender, DeviceAppEvent, DeviceRequestAppEvent, TileInteractionContext,
    },
    plugin::Plugins,
    utils::random::generate_access_token,
};

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
    plugins: Plugins,

    /// Current device socket sessions
    sessions: RwLock<HashMap<DeviceSessionId, DeviceSessionRef>>,

    /// Current requests for authorization from devices
    requests: RwLock<Vec<DeviceRequest>>,
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
}

const DEVICES_TOKEN_LENGTH: usize = 32;

impl Devices {
    pub fn new(event_tx: AppEventSender, db: DbPool, plugins: Plugins) -> Self {
        Self {
            event_tx,
            db,
            plugins,
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
                // Skip closed sessions
                if session_ref.is_closed() {
                    return None;
                }

                let device_id = session_ref.get_device_id()?;

                Some(ConnectedDevice {
                    device_id,
                    session_id: *session_id,
                })
            })
            .collect()
    }

    /// Remove a session
    pub fn remove_session(&self, session_id: DeviceSessionId, device_id: Option<DeviceId>) {
        self.sessions.write().remove(&session_id);
        self.remove_session_device_requests(session_id);

        if let Some(device_id) = device_id {
            self.emit_app_event(AppEvent::Device(DeviceAppEvent::Disconnected { device_id }));
        }
    }

    /// Removes any device requests associated with the provided session
    /// (Notifying by sending an event)
    pub fn remove_session_device_requests(&self, session_id: DeviceSessionId) {
        self.requests.write().retain(|request| {
            if request.session_id == session_id {
                self.emit_app_event(AppEvent::DeviceRequest(DeviceRequestAppEvent::Removed {
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
    ) {
        self.remove_session_device_requests(session_id);

        let request_id = Uuid::new_v4();
        self.requests.write().push(DeviceRequest {
            id: request_id,
            socket_addr,
            session_id,
            device_name,
        });

        self.emit_app_event(AppEvent::DeviceRequest(DeviceRequestAppEvent::Added {
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

        let access_token = generate_access_token(DEVICES_TOKEN_LENGTH);

        let device = DeviceModel::create(
            db,
            CreateDevice {
                name: request.device_name,
                access_token: access_token.to_string(),
                config: DeviceConfig {},
                profile_id: default_folder.profile_id,
                folder_id: default_folder.id,
            },
        )
        .await?;

        session.send_message(ServerDeviceMessage::Approved {
            device_id: device.id,
            access_token,
        });

        self.emit_app_event(AppEvent::DeviceRequest(DeviceRequestAppEvent::Accepted {
            request_id,
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

        session.set_device_id(None);
        session.send_message(ServerDeviceMessage::Declined);

        self.emit_app_event(AppEvent::DeviceRequest(DeviceRequestAppEvent::Decline {
            request_id,
        }));

        Ok(())
    }

    /// Find a session from its device ID
    pub fn get_session_by_device(&self, device_id: DeviceId) -> Option<DeviceSessionRef> {
        self.sessions.read().values().find_map(|session_ref| {
            // Skip closed sessions
            if session_ref.is_closed() {
                return None;
            }

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
        session_id: DeviceSessionId,
        access_token: String,
    ) -> anyhow::Result<()> {
        let session = self.get_session(&session_id).context("session not found")?;
        let mut device = match DeviceModel::get_by_access_token(&self.db, &access_token).await? {
            Some(device) => device,
            None => {
                session.send_message(ServerDeviceMessage::InvalidAccessToken);
                return Ok(());
            }
        };

        // Update last connected
        device.set_connected_now(&self.db).await?;

        // Authenticate the device session
        session.set_device_id(Some(device.id));
        session.send_message(ServerDeviceMessage::Authenticated);

        // Notify frontend
        self.emit_app_event(AppEvent::Device(DeviceAppEvent::Authenticated {
            device_id: device.id,
        }));

        Ok(())
    }

    /// Revoke access for a device
    pub async fn revoke_device(&self, device_id: DeviceId) -> anyhow::Result<()> {
        DeviceModel::delete(&self.db, device_id).await?;

        self.emit_app_event(AppEvent::Device(DeviceAppEvent::Revoked { device_id }));

        // Tell the session its been revoked
        if let Some(session) = self.get_session_by_device(device_id) {
            session.set_device_id(None);
            session.send_message(ServerDeviceMessage::Revoked);
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

    /// Updates the device tiles on all devices using the provided folder
    pub async fn update_devices_tiles(&self, folder_id: FolderId) -> anyhow::Result<()> {
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

        for device in devices {
            self.send_to_device(
                device.id,
                ServerDeviceMessage::Tiles {
                    tiles: tiles.clone(),
                    folder: folder.clone(),
                },
            )
        }

        Ok(())
    }

    pub fn send_to_device(&self, device_id: DeviceId, message: ServerDeviceMessage) {
        let session = match self.get_session_by_device(device_id) {
            Some(value) => value,
            None => return,
        };

        _ = session.send_message(message);
    }

    pub async fn device_execute_tile(
        self: &Arc<Self>,
        device_id: DeviceId,
        tile_id: TileId,
    ) -> anyhow::Result<()> {
        let db = &self.db;
        let tile = TileModel::get_by_id(db, tile_id)
            .await?
            .context("tile instance not found")?;

        let context = TileInteractionContext {
            device_id,
            plugin_id: tile.config.plugin_id.clone(),
            action_id: tile.config.action_id.clone(),
            tile_id,
        };

        self.plugins.handle_action(self, context, tile).await?;

        Ok(())
    }

    fn emit_app_event(&self, event: AppEvent) {
        // Notify frontend
        _ = self.event_tx.send(event);
    }
}
