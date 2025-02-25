use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use parking_lot::RwLock;
use socket::{DeviceSessionId, DeviceSessionRef};
use uuid::Uuid;

use crate::database::entity::device::DeviceId;

mod protocol;
mod socket;

/// Store for device sessions and requests
#[derive(Default, Clone)]
pub struct Devices {
    inner: Arc<RwLock<DevicesInner>>,
}

#[derive(Default)]
pub struct DevicesInner {
    /// Current device socket sessions
    sessions: HashMap<DeviceSessionId, DeviceSessionRef>,

    /// Current requests for authorization from devices
    requests: Vec<DeviceRequest>,
}

pub struct DeviceRequest {
    /// Reference to the session requesting approval
    session: DeviceSessionRef,
    /// Name of the device requesting approval
    device_name: String,
}
