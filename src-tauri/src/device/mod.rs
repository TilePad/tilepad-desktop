use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use parking_lot::RwLock;
use protocol::ClientDeviceMessage;
use socket::{DeviceSessionId, DeviceSessionRef};
use uuid::Uuid;

use crate::{
    database::entity::device::DeviceId,
    events::{AppEvent, AppEventSender},
};

mod protocol;
pub mod socket;

pub type DeviceRequestId = Uuid;

/// Store for device sessions and requests
#[derive(Clone)]
pub struct Devices {
    inner: Arc<DevicesInner>,
}

impl Devices {
    /// Insert a new session
    pub fn insert_session(&self, session_id: DeviceSessionId, session_ref: DeviceSessionRef) {
        self.inner.sessions.write().insert(session_id, session_ref);
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
                self.inner.event_tx.send(AppEvent::DeviceRequestRemoved {
                    request_id: request.id,
                });
                false
            } else {
                true
            }
        });
    }

    /// Add a new device request
    pub fn add_device_request(&self, session_id: DeviceSessionId, device_name: String) {
        self.remove_session_device_requests(session_id);

        let inner = &*self.inner;

        let request_id = Uuid::new_v4();
        inner.requests.write().push(DeviceRequest {
            id: request_id,
            session_id,
            device_name,
        });

        inner
            .event_tx
            .send(AppEvent::DeviceRequestRemoved { request_id });
    }
}

pub struct DevicesInner {
    /// Sender for app events
    event_tx: AppEventSender,

    /// Current device socket sessions
    sessions: RwLock<HashMap<DeviceSessionId, DeviceSessionRef>>,

    /// Current requests for authorization from devices
    requests: RwLock<Vec<DeviceRequest>>,
}

pub struct DeviceRequest {
    /// Unique ID for the request itself
    id: DeviceRequestId,
    /// ID of the session the request is for
    session_id: DeviceSessionId,
    /// Name of the device requesting approval
    device_name: String,
}
