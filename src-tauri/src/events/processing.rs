use std::{future::poll_fn, task::Poll};

use futures::{stream::FuturesUnordered, Stream};
use tauri::AppHandle;
use tracing::{debug, error};

use crate::database::DbPool;

use super::{AppEvent, AppEventReceiver};

pub async fn process_events(db: DbPool, app_handle: AppHandle, mut event_rx: AppEventReceiver) {
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

            futures.push(process_event(&db, &app_handle, event));
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

async fn process_event(db: &DbPool, app_handle: &AppHandle, event: AppEvent) -> anyhow::Result<()> {
    Ok(())
}
