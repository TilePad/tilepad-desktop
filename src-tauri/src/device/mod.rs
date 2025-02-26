use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use anyhow::Context;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use socket::{DeviceSessionId, DeviceSessionRef};
use uuid::Uuid;

use crate::{
    database::{
        entity::device::{CreateDevice, DeviceConfig, DeviceId, DeviceModel},
        DbPool,
    },
    events::{AppEvent, AppEventSender, DeviceAppEvent, DeviceRequestAppEvent},
    utils::random::generate_access_token,
};

mod protocol;
pub mod socket;

pub type DeviceRequestId = Uuid;

/// Store for device sessions and requests
#[derive(Clone)]
pub struct Devices {
    inner: Arc<DevicesInner>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedDevice {
    pub device_id: DeviceId,
    pub session_id: DeviceSessionId,
}

const DEVICES_TOKEN_LENGTH: usize = 32;

impl Devices {
    pub fn new(event_tx: AppEventSender, db: DbPool) -> Self {
        Self {
            inner: Arc::new(DevicesInner {
                event_tx,
                db,
                sessions: Default::default(),
                requests: Default::default(),
            }),
        }
    }

    /// Insert a new session
    pub fn insert_session(&self, session_id: DeviceSessionId, session_ref: DeviceSessionRef) {
        self.inner.sessions.write().insert(session_id, session_ref);
    }

    /// Insert a new session
    pub fn get_session(&self, session_id: &DeviceSessionId) -> Option<DeviceSessionRef> {
        self.inner.sessions.write().get(session_id).cloned()
    }

    /// Get all devices that have active sessions
    pub fn get_connected_devices(&self) -> Vec<ConnectedDevice> {
        self.inner
            .sessions
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
    pub fn remove_session(&self, session_id: DeviceSessionId) {
        self.inner.sessions.write().remove(&session_id);
        self.remove_session_device_requests(session_id);
    }

    /// Removes any device requests associated with the provided session
    /// (Notifying by sending an event)
    pub fn remove_session_device_requests(&self, session_id: DeviceSessionId) {
        self.inner.requests.write().retain(|request| {
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
        let requests = &mut *self.inner.requests.write();
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

        let inner = &*self.inner;

        let request_id = Uuid::new_v4();
        inner.requests.write().push(DeviceRequest {
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
        self.inner.requests.read().clone()
    }

    /// Approve a device request, creates a new device in the database
    pub async fn approve_device_request(&self, request_id: DeviceRequestId) -> anyhow::Result<()> {
        let request = self
            .take_device_request(request_id)
            .context("request not found")?;
        let session = self
            .get_session(&request.session_id)
            .context("session not found")?;

        let access_token = generate_access_token(DEVICES_TOKEN_LENGTH);

        let device = DeviceModel::create(
            &self.inner.db,
            CreateDevice {
                name: request.device_name,
                access_token: access_token.to_string(),
                config: DeviceConfig {},
            },
        )
        .await?;

        session.send_approved(device.id, access_token)?;

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
        session.send_declined()?;

        self.emit_app_event(AppEvent::DeviceRequest(DeviceRequestAppEvent::Decline {
            request_id,
        }));

        Ok(())
    }

    /// Find a session from its device ID
    pub fn get_session_by_device(&self, device_id: DeviceId) -> Option<DeviceSessionRef> {
        self.inner.sessions.read().values().find_map(|session_ref| {
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
        let mut device =
            match DeviceModel::get_by_access_token(&self.inner.db, &access_token).await? {
                Some(device) => device,
                None => {
                    session.send_invalid_access_token()?;
                    return Ok(());
                }
            };

        // Update last connected
        device.set_connected_now(&self.inner.db).await?;

        // Authenticate the device session
        session.set_device_id(Some(device.id));
        session.send_authenticated()?;

        // Notify frontend
        self.emit_app_event(AppEvent::Device(DeviceAppEvent::Authenticated {
            device_id: device.id,
        }));

        Ok(())
    }

    /// Revoke access for a device
    pub async fn revoke_device(&self, device_id: DeviceId) -> anyhow::Result<()> {
        DeviceModel::delete(&self.inner.db, device_id).await?;

        self.emit_app_event(AppEvent::Device(DeviceAppEvent::Revoked { device_id }));

        // Tell the session its been revoked
        if let Some(session) = self.get_session_by_device(device_id) {
            session.set_device_id(None);
            session.send_revoked()?;
        }

        Ok(())
    }

    fn emit_app_event(&self, event: AppEvent) {
        // Notify frontend
        _ = self.inner.event_tx.send(event);
    }
}

pub struct DevicesInner {
    /// Sender for app events
    event_tx: AppEventSender,

    /// Access to the database
    db: DbPool,

    /// Current device socket sessions
    sessions: RwLock<HashMap<DeviceSessionId, DeviceSessionRef>>,

    /// Current requests for authorization from devices
    requests: RwLock<Vec<DeviceRequest>>,
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
