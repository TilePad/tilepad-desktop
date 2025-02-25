use tokio::sync::mpsc;

use crate::device::DeviceRequestId;

pub mod processing;

pub type AppEventSender = mpsc::UnboundedSender<AppEvent>;
pub type AppEventReceiver = mpsc::UnboundedReceiver<AppEvent>;

#[derive(Debug)]
pub enum AppEvent {
    DeviceRequestAdded { request_id: DeviceRequestId },
    DeviceRequestRemoved { request_id: DeviceRequestId },
}
