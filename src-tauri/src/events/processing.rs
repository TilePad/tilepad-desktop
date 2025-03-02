use std::{future::poll_fn, task::Poll};

use futures::{stream::FuturesUnordered, Stream};
use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tracing::{debug, error};

use crate::{database::DbPool, events::PluginMessageContext};

use super::{AppEvent, AppEventReceiver, DeviceAppEvent, DeviceRequestAppEvent, PluginAppEvent};

pub async fn process_events(app_handle: AppHandle, db: DbPool, mut event_rx: AppEventReceiver) {
    let mut futures = FuturesUnordered::new();
    let mut futures = std::pin::pin!(futures);

    poll_fn::<(), _>(|cx| {
        // Poll new event execution
        while let Poll::Ready(result) = event_rx.poll_recv(cx) {
            let event = match result {
                Some(value) => value,

                // All channels have been closed and the app is likely shutting down,
                // finish the future and stop processing
                None => return Poll::Ready(()),
            };

            debug!(?event, "app event received");

            futures.push(process_event(&app_handle, &db, event));
        }

        // Poll completions until no more ready
        while let Poll::Ready(Some(result)) = futures.as_mut().poll_next(cx) {
            if let Err(cause) = result {
                error!(?cause, "failed to process app event",);
            }
        }

        Poll::Pending
    })
    .await;
}

async fn process_event(app_handle: &AppHandle, db: &DbPool, event: AppEvent) -> anyhow::Result<()> {
    match event {
        AppEvent::DeviceRequest(request) => match request {
            DeviceRequestAppEvent::Added { request_id } => {
                app_handle.emit("device_requests:added", request_id)?;
            }
            DeviceRequestAppEvent::Removed { request_id } => {
                app_handle.emit("device_requests:removed", request_id)?;
            }
            DeviceRequestAppEvent::Accepted { request_id } => {
                app_handle.emit("device_requests:accepted", request_id)?;
            }
            DeviceRequestAppEvent::Decline { request_id } => {
                app_handle.emit("device_requests:declined", request_id)?;
            }
        },
        AppEvent::Device(device_app_event) => match device_app_event {
            DeviceAppEvent::Authenticated { device_id } => {
                app_handle.emit("device:authenticated", device_id)?;
            }
            DeviceAppEvent::Revoked { device_id } => {
                app_handle.emit("device:revoked", device_id)?;
            }
        },
        AppEvent::Plugin(plugin_app_event) => match plugin_app_event {
            PluginAppEvent::RecvPluginMessage { context, message } => {
                #[derive(Serialize)]
                struct Payload {
                    context: PluginMessageContext,
                    message: serde_json::Value,
                }

                app_handle.emit("plugin:recv_plugin_message", &Payload { context, message })?;
            }
            PluginAppEvent::OpenInspector { context } => {}
            PluginAppEvent::CloseInspector { context } => {}
        },
    }

    Ok(())
}
