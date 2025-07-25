use std::{future::poll_fn, task::Poll};

use futures::{Stream, stream::FuturesUnordered};
use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tilepad_manifest::plugin::PluginId;
use tracing::{debug, error};

use crate::{
    database::DbPool,
    events::{DisplayContext, InspectorContext},
    plugin::runner::PluginTaskState,
};

use super::{
    AppEvent, AppEventReceiver, DeviceAppEvent, DeviceRequestAppEvent, IconPackAppEvent,
    PluginAppEvent,
};

pub async fn process_events(app_handle: AppHandle, db: DbPool, mut event_rx: AppEventReceiver) {
    let futures = FuturesUnordered::new();
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

async fn process_event(
    app_handle: &AppHandle,
    _db: &DbPool,
    event: AppEvent,
) -> anyhow::Result<()> {
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
            DeviceAppEvent::Disconnected { device_id } => {
                app_handle.emit("device:disconnected", device_id)?;
            }
        },
        AppEvent::Plugin(plugin_app_event) => match plugin_app_event {
            PluginAppEvent::Message { context, message } => {
                #[derive(Serialize)]
                struct Payload {
                    context: InspectorContext,
                    message: serde_json::Value,
                }

                app_handle.emit("plugin:recv_plugin_message", &Payload { context, message })?;
            }
            PluginAppEvent::DisplayMessage { context, message } => {
                #[derive(Serialize)]
                struct Payload {
                    context: DisplayContext,
                    message: serde_json::Value,
                }

                app_handle.emit(
                    "plugin:recv_plugin_display_message",
                    &Payload { context, message },
                )?;
            }

            PluginAppEvent::Loaded { plugin_id } => {
                app_handle.emit("plugins:loaded", plugin_id)?;
            }
            PluginAppEvent::Unloaded { plugin_id } => {
                app_handle.emit("plugins:unloaded", plugin_id)?;
            }
            PluginAppEvent::TaskStateChanged { plugin_id, state } => {
                #[derive(Serialize)]
                struct Payload {
                    plugin_id: PluginId,
                    state: PluginTaskState,
                }

                app_handle.emit("plugins:task_state_changed", &Payload { plugin_id, state })?;
            }
        },
        AppEvent::IconPack(icon_pack_app_event) => match icon_pack_app_event {
            IconPackAppEvent::Loaded { pack_id } => {
                app_handle.emit("icon_packs:loaded", pack_id)?;
            }
            IconPackAppEvent::Unloaded { pack_id } => {
                app_handle.emit("icon_packs:unloaded", pack_id)?;
            }
        },
    }

    Ok(())
}
