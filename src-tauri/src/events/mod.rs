use tokio::sync::mpsc;

use crate::{database::entity::device::DeviceId, device::DeviceRequestId};

pub mod processing;

pub type AppEventSender = mpsc::UnboundedSender<AppEvent>;
pub type AppEventReceiver = mpsc::UnboundedReceiver<AppEvent>;

#[derive(Debug)]
pub enum AppEvent {
    DeviceRequest(DeviceRequestAppEvent),

    Device(DeviceAppEvent),
}

#[derive(Debug)]
pub enum DeviceAppEvent {
    Authenticated { device_id: DeviceId },
    Revoked { device_id: DeviceId },
}

#[derive(Debug)]
pub enum DeviceRequestAppEvent {
    Added { request_id: DeviceRequestId },
    Removed { request_id: DeviceRequestId },

    Accepted { request_id: DeviceRequestId },
    Decline { request_id: DeviceRequestId },
}
