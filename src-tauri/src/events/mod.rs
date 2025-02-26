use tokio::sync::mpsc;

use crate::device::DeviceRequestId;

pub mod processing;

pub type AppEventSender = mpsc::UnboundedSender<AppEvent>;
pub type AppEventReceiver = mpsc::UnboundedReceiver<AppEvent>;

#[derive(Debug)]
pub enum AppEvent {
    DeviceRequest(DeviceRequestAppEvent),
}

#[derive(Debug)]
pub enum DeviceRequestAppEvent {
    Added { request_id: DeviceRequestId },
    Removed { request_id: DeviceRequestId },

    Accepted { request_id: DeviceRequestId },
    Decline { request_id: DeviceRequestId },
}
