use std::{io::ErrorKind, net::SocketAddr, sync::Arc};

use axum::extract::ws::WebSocket;
use chacha20poly1305::{
    AeadCore, KeyInit, XChaCha20Poly1305, XNonce,
    aead::{Aead, OsRng, rand_core::RngCore},
};
use parking_lot::RwLock;
use tauri::async_runtime::spawn;
use tracing::error;
use uuid::Uuid;
use x25519_dalek::PublicKey;

use crate::{
    database::entity::{
        device::DeviceId,
        folder::FolderModel,
        tile::{TileId, TileModel},
    },
    device::protocol::{
        ClientDeviceMessageEncrypted, DeviceIndicator, ServerDeviceMessageEncrypted,
    },
    events::DisplayContext,
    utils::{
        error::try_cast_error,
        ws_msgpack::{WebSocketMpFuture, WsMpTx},
    },
};

use super::{
    Devices,
    protocol::{ClientDeviceMessage, ServerDeviceMessage},
};

pub type DeviceSessionId = Uuid;
pub type DeviceSessionRef = Arc<DeviceSession>;

pub struct DeviceSession {
    /// Unique ID of the session
    id: DeviceSessionId,

    /// Address of the device session socket
    socket_addr: SocketAddr,

    /// Session state
    state: RwLock<DeviceSessionState>,

    /// Channel to send messages to the session
    tx: WsMpTx<ServerDeviceMessage>,

    /// Access to the devices registry the session is apart of
    devices: Arc<Devices>,
}

#[derive(Default, Clone)]
enum DeviceSessionState {
    // Unauthenticated, no session
    #[default]
    Initial,

    /// Device has been challenged
    Challenge(DeviceSessionChallengeState),

    /// Awaiting approval
    AwaitingApproval(DeviceSessionAwaitingApprovalState),

    // Authenticated
    Authenticated(DeviceSessionAuthenticatedState),
}

#[derive(Clone)]
struct DeviceSessionChallengeState {
    /// Cipher for encrypted communication
    cipher: XChaCha20Poly1305,
    /// Challenge bytes
    challenge: Vec<u8>,
    /// Name of the client
    client_name: String,
    /// Client public key
    client_public_key: [u8; 32],
}

#[derive(Clone)]
struct DeviceSessionAwaitingApprovalState {
    /// Cipher for encrypted communication
    cipher: XChaCha20Poly1305,
}
#[derive(Clone)]
struct DeviceSessionAuthenticatedState {
    /// Cipher for encrypted communication
    cipher: XChaCha20Poly1305,
    device_id: DeviceId,
}

impl DeviceSession {
    pub fn start(devices: Arc<Devices>, socket_addr: SocketAddr, socket: WebSocket) {
        let id = Uuid::new_v4();

        // Create and spawn a future for the websocket
        let (ws_future, ws_rx, ws_tx) =
            WebSocketMpFuture::<ServerDeviceMessage, ClientDeviceMessage>::new(socket);

        spawn(async move {
            if let Err(cause) = ws_future.await {
                // Handle device connection lost as just a warning
                if let Some(cause_io) = try_cast_error::<std::io::Error>(&cause) {
                    if cause_io.kind() == ErrorKind::ConnectionReset {
                        tracing::warn!(?cause_io, "plugin connection closed");
                        return;
                    }
                }

                error!(?cause, "error running device websocket future");
            }
        });

        let session = Arc::new(DeviceSession {
            id,
            socket_addr,
            state: Default::default(),
            tx: ws_tx,
            devices,
        });

        spawn(async move {
            // Add the session
            session.devices.insert_session(id, session.clone());

            let mut ws_rx = ws_rx;

            // Process messages from the session
            while let Some(msg) = ws_rx.recv().await {
                session.handle_message(msg).await;
            }

            let device_id = session.get_device_id();

            // Remove the session thats no longer running
            session.devices.remove_session(session.id, device_id);
        });
    }

    /// Get the current device ID
    pub fn get_device_id(&self) -> Option<DeviceId> {
        match &*self.state.read() {
            DeviceSessionState::Authenticated(state) => Some(state.device_id),
            _ => None,
        }
    }

    /// Send a plain-text message
    fn send_message(&self, msg: ServerDeviceMessage) -> bool {
        self.tx.send(msg).is_ok()
    }

    /// Send a message using the current encryption cipher
    fn send_encrypted_message(&self, msg: ServerDeviceMessageEncrypted) -> bool {
        let state = self.state.read();
        let cipher = match &*state {
            DeviceSessionState::AwaitingApproval(state) => &state.cipher,
            DeviceSessionState::Authenticated(state) => &state.cipher,
            _ => {
                tracing::error!("encrypted state not active");
                return false;
            }
        };

        let encoded_message = match serde_json::to_vec(&msg) {
            Ok(value) => value,
            Err(err) => {
                tracing::error!(?err, "failed to encode message");
                return false;
            }
        };

        // Encrypt challenge
        let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
        let encrypted_message = match cipher.encrypt(&nonce, encoded_message.as_slice()) {
            Ok(value) => value,
            Err(err) => {
                tracing::error!(?err, "failed to encrypt message");
                self.send_message(ServerDeviceMessage::Error {
                    message: "failed to encrypt message".to_string(),
                });
                return false;
            }
        };

        self.tx
            .send(ServerDeviceMessage::EncryptedMessage {
                message: encrypted_message,
                nonce: nonce.into(),
            })
            .is_ok()
    }

    pub fn revoke(&self) {
        self.send_encrypted_message(ServerDeviceMessageEncrypted::Revoked);
        *self.state.write() = Default::default();
    }

    pub fn decline(&self) {
        self.send_encrypted_message(ServerDeviceMessageEncrypted::Declined);
        *self.state.write() = Default::default();
    }

    pub fn on_approved(&self, device_id: DeviceId) {
        {
            let state = &mut *self.state.write();
            let cipher = match state {
                DeviceSessionState::AwaitingApproval(state) => state.cipher.clone(),
                _ => return,
            };

            // Authenticate the device session
            *state = DeviceSessionState::Authenticated(DeviceSessionAuthenticatedState {
                cipher,
                device_id,
            });
        };

        self.send_encrypted_message(ServerDeviceMessageEncrypted::Approved { device_id });
        self.send_encrypted_message(ServerDeviceMessageEncrypted::Authenticated { device_id });
    }

    pub fn on_plugin_message(&self, ctx: DisplayContext, message: serde_json::Value) {
        self.send_encrypted_message(ServerDeviceMessageEncrypted::RecvFromPlugin { ctx, message });
    }

    pub fn on_tile_indicator(&self, tile_id: TileId, indicator: DeviceIndicator, duration: u32) {
        self.send_encrypted_message(ServerDeviceMessageEncrypted::DisplayIndicator {
            tile_id,
            indicator,
            duration,
        });
    }

    pub fn on_tiles(&self, tiles: Vec<TileModel>, folder: FolderModel) {
        self.send_encrypted_message(ServerDeviceMessageEncrypted::Tiles { tiles, folder });
    }

    /// Handle messages from the socket
    async fn handle_message(&self, message: ClientDeviceMessage) {
        let state = { self.state.read().clone() };

        match state {
            DeviceSessionState::Initial => self.handle_message_initial(message),
            DeviceSessionState::Challenge(state) => {
                self.handle_message_challenge(state, message).await
            }
            DeviceSessionState::AwaitingApproval(_) => {
                tracing::warn!(?message, "got unexpected message from unauthorized device");
            }
            DeviceSessionState::Authenticated(state) => {
                self.handle_message_authenticated(state, message).await
            }
        };
    }

    fn handle_message_initial(&self, message: ClientDeviceMessage) {
        match message {
            ClientDeviceMessage::InitiateHandshake { name, public_key } => {
                self.handle_initiate_handshake(name, public_key)
            }

            _ => tracing::warn!(?message, "got unexpected message from unauthorized device"),
        }
    }

    /// Handles the initiation of a handshake
    fn handle_initiate_handshake(&self, name: String, public_key: [u8; 32]) {
        // Perform ECDH key exchange
        let client_public_key = PublicKey::from(public_key);
        let shared_secret = self
            .devices
            .server_key_pair
            .private_key
            .diffie_hellman(&client_public_key);

        // Create cipher
        let cipher = match XChaCha20Poly1305::new_from_slice(shared_secret.as_bytes()) {
            Ok(value) => value,
            Err(err) => {
                tracing::error!(?err, "failed to create cipher");
                self.send_message(ServerDeviceMessage::Error {
                    message: "failed to create cipher".to_string(),
                });
                return;
            }
        };

        let EncryptedChallenge {
            challenge,
            encrypted_challenge,
            nonce,
        } = match generate_encrypted_challenge(&cipher) {
            Ok(value) => value,
            Err(err) => {
                tracing::error!(?err, "failed to create challenge");
                self.send_message(ServerDeviceMessage::Error {
                    message: "failed to create challenge".to_string(),
                });
                return;
            }
        };

        {
            // Move to challenge state
            *self.state.write() = DeviceSessionState::Challenge(DeviceSessionChallengeState {
                cipher,
                challenge,
                client_name: name,
                client_public_key: client_public_key.to_bytes(),
            });
        }

        // Notify device of challenge
        self.send_message(ServerDeviceMessage::AuthenticateChallenge {
            server_public_key: self.devices.server_key_pair.public_key.to_bytes(),
            challenge: encrypted_challenge,
            nonce,
        });
    }

    async fn handle_challenge_response(
        &self,
        state: DeviceSessionChallengeState,
        //
        client_challenge: Vec<u8>,
        client_nonce: [u8; 24],
    ) {
        let client_nonce = XNonce::from(client_nonce);
        let client_challenge = match state
            .cipher
            .decrypt(&client_nonce, client_challenge.as_slice())
        {
            Ok(value) => value,
            Err(err) => {
                tracing::error!(?err, "failed to decrypt challenge");
                self.send_message(ServerDeviceMessage::Error {
                    message: "failed to decrypt challenge".to_string(),
                });
                return;
            }
        };

        // Challenge didn't match
        if client_challenge != state.challenge {
            tracing::error!("incorrect challenge");
            self.send_message(ServerDeviceMessage::Error {
                message: "challenge does not match".to_string(),
            });
            return;
        }

        match self
            .devices
            .attempt_authenticate_device(&state.client_public_key)
            .await
        {
            // Public key is known and authenticated with an existing device
            Ok(Some(device_id)) => {
                {
                    // Authenticate the device session
                    *self.state.write() =
                        DeviceSessionState::Authenticated(DeviceSessionAuthenticatedState {
                            cipher: state.cipher,
                            device_id,
                        });
                };

                self.send_encrypted_message(ServerDeviceMessageEncrypted::Authenticated {
                    device_id,
                });
            }
            // Public key is not known or approved yet add approval request
            Ok(None) => {
                {
                    // Awaiting approval
                    *self.state.write() =
                        DeviceSessionState::AwaitingApproval(DeviceSessionAwaitingApprovalState {
                            cipher: state.cipher,
                        });
                }

                self.devices.add_device_request(
                    self.id,
                    self.socket_addr,
                    state.client_name,
                    state.client_public_key,
                );

                self.send_encrypted_message(ServerDeviceMessageEncrypted::ApprovalRequested);
            }
            Err(cause) => {
                tracing::error!(?cause, "failed to authenticate device");
                self.send_message(ServerDeviceMessage::Error {
                    message: "failed to authenticate device".to_string(),
                });
            }
        }
    }

    async fn handle_message_challenge(
        &self,
        state: DeviceSessionChallengeState,
        message: ClientDeviceMessage,
    ) {
        match message {
            ClientDeviceMessage::AuthenticateChallengeResponse {
                challenge: client_challenge,
                nonce,
            } => {
                self.handle_challenge_response(state, client_challenge, nonce)
                    .await;
            }

            _ => tracing::warn!(?message, "got unexpected message from unauthorized device"),
        }
    }

    async fn handle_message_authenticated(
        &self,
        state: DeviceSessionAuthenticatedState,
        message: ClientDeviceMessage,
    ) {
        let (message, nonce) = match message {
            ClientDeviceMessage::Encrypted { message, nonce } => (message, nonce),

            _ => {
                tracing::warn!(?message, "got unexpected message from device");
                return;
            }
        };

        let nonce = XNonce::from(nonce);
        let message = match state.cipher.decrypt(&nonce, message.as_slice()) {
            Ok(value) => value,
            Err(err) => {
                tracing::error!(?err, "failed to decrypt message");
                self.send_message(ServerDeviceMessage::Error {
                    message: "failed to decrypt message".to_string(),
                });
                return;
            }
        };

        let msg: ClientDeviceMessageEncrypted = match serde_json::from_slice(&message) {
            Ok(value) => value,
            Err(err) => {
                tracing::error!(?err, "failed to decode message");
                self.send_message(ServerDeviceMessage::Error {
                    message: "failed to decode message".to_string(),
                });
                return;
            }
        };

        self.handle_message_encrypted(state.device_id, msg).await
    }

    /// Handle message when authenticated as `device_id`
    async fn handle_message_encrypted(
        &self,
        device_id: DeviceId,
        message: ClientDeviceMessageEncrypted,
    ) {
        match message {
            ClientDeviceMessageEncrypted::RequestTiles => {
                // Get the current folder the device is using
                let (folder, tiles) = match self.devices.request_device_tiles(device_id).await {
                    Ok(value) => value,
                    Err(cause) => {
                        tracing::error!(?cause, "failed to request device tiles");
                        return;
                    }
                };

                // Send the tiles to the device
                self.send_encrypted_message(ServerDeviceMessageEncrypted::Tiles { tiles, folder });
            }

            ClientDeviceMessageEncrypted::TileClicked { tile_id } => {
                let devices = self.devices.clone();

                _ = tokio::spawn(async move {
                    if let Err(cause) = devices.device_execute_tile(device_id, tile_id).await {
                        tracing::error!(?cause, "failed to execute tile");
                    }
                });
            }

            ClientDeviceMessageEncrypted::RecvFromDisplay { ctx, message } => {
                let plugins = self.devices.plugins.clone();

                _ = tokio::spawn(async move {
                    if let Err(cause) = plugins.handle_send_display_message(ctx, message).await {
                        tracing::error!(?cause, "failed to forward display message");
                    }
                });
            }
        }
    }
}

struct EncryptedChallenge {
    challenge: Vec<u8>,
    encrypted_challenge: Vec<u8>,
    nonce: [u8; 24],
}

fn generate_encrypted_challenge(cipher: &XChaCha20Poly1305) -> anyhow::Result<EncryptedChallenge> {
    // Generate a random challenge bytes
    let mut challenge = [0u8; 128];
    OsRng.fill_bytes(&mut challenge);

    // Encrypt challenge
    let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
    let encrypted_challenge = cipher
        .encrypt(&nonce, challenge.as_slice())
        .map_err(|_| anyhow::anyhow!("failed to encrypt challenge"))?;

    Ok(EncryptedChallenge {
        challenge: challenge.to_vec(),
        encrypted_challenge,
        nonce: nonce.into(),
    })
}
