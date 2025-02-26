use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use anyhow::Context;
use parking_lot::RwLock;
use serde::Serialize;
use socket::{DeviceSessionId, DeviceSessionRef};
use uuid::Uuid;

use crate::{
    database::{
        entity::device::{CreateDevice, DeviceConfig, DeviceModel},
        DbPool,
    },
    events::{AppEvent, AppEventSender, DeviceRequestAppEvent},
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
                _ = self.inner.event_tx.send(AppEvent::DeviceRequest(
                    DeviceRequestAppEvent::Removed {
                        request_id: request.id,
                    },
                ));
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

        _ = inner
            .event_tx
            .send(AppEvent::DeviceRequest(DeviceRequestAppEvent::Added {
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

        session.set_device_id(Some(device.id));
        session.send_approved(device.id, access_token)?;

        _ = self
            .inner
            .event_tx
            .send(AppEvent::DeviceRequest(DeviceRequestAppEvent::Accepted {
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

        _ = self
            .inner
            .event_tx
            .send(AppEvent::DeviceRequest(DeviceRequestAppEvent::Decline {
                request_id,
            }));

        Ok(())
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
