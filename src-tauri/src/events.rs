use tokio::sync::mpsc;

use crate::device::{DeviceRequest, DeviceRequestId};

pub type AppEventSender = mpsc::UnboundedSender<AppEvent>;
pub type AppEventReceiver = mpsc::UnboundedReceiver<AppEvent>;

pub enum AppEvent {
    DeviceRequestAdded { request_id: DeviceRequestId },
    DeviceRequestRemoved { request_id: DeviceRequestId },
}
