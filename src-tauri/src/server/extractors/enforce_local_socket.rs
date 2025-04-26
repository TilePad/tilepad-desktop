use std::net::SocketAddr;

use anyhow::Context;
use axum::{
    extract::{ConnectInfo, FromRequestParts},
    http::request::Parts,
};
use reqwest::StatusCode;
use thiserror::Error;

use crate::server::models::error::{DynHttpError, HttpError};

/// Extractor that enforces any requests must have a
/// loopback address
///
/// Used for securing the plugin API to ensure plugins
/// cannot attachment themselves over the network
pub struct EnforceLocalSocket;

impl<S> FromRequestParts<S> for EnforceLocalSocket
where
    S: Send + Sync,
{
    type Rejection = DynHttpError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let connect_info = parts
            .extensions
            .get::<ConnectInfo<SocketAddr>>()
            .context("missing connection info")?;
        if !connect_info.ip().is_loopback() {
            return Err(RemoteConnectionNotAllowed {
                addr: connect_info.0,
            }
            .into());
        }

        Ok(EnforceLocalSocket)
    }
}

#[derive(Debug, Error)]
#[error("remote connections are not allowed for this endpoint")]
pub struct RemoteConnectionNotAllowed {
    addr: SocketAddr,
}

impl HttpError for RemoteConnectionNotAllowed {
    fn log(&self) {
        tracing::warn!(addr = ?self.addr, "remote address attempted to access local only endpoint");
    }

    fn status(&self) -> reqwest::StatusCode {
        StatusCode::FORBIDDEN
    }
}
