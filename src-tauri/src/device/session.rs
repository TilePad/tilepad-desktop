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
    database::entity::device::DeviceId,
    device::protocol::{ClientDeviceMessageEncrypted, ServerDeviceMessageEncrypted},
    utils::{
        error::try_cast_error,
        ws::{WebSocketFuture, WsTx},
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
    tx: WsTx<ServerDeviceMessage>,

    /// Access to the devices registry the session is apart of
    devices: Arc<Devices>,
}

#[derive(Default, Clone)]
pub enum DeviceSessionState {
    // Unauthenticated, no session
    #[default]
    Initial,

    /// Device has been challenged
    Challenge {
        /// Cipher for encrypted communication
        cipher: XChaCha20Poly1305,
        /// Challenge bytes
        challenge: Vec<u8>,

        /// Name of the client
        client_name: String,
        /// Client public key
        client_public_key: [u8; 32],
    },

    /// Awaiting approval
    AwaitingApproval {
        /// Cipher for encrypted communication
        cipher: XChaCha20Poly1305,
    },

    // Authenticated
    Authenticated {
        /// Cipher for encrypted communication
        cipher: XChaCha20Poly1305,
        device_id: DeviceId,
    },
}

impl DeviceSession {
    pub fn start(devices: Arc<Devices>, socket_addr: SocketAddr, socket: WebSocket) {
        let id = Uuid::new_v4();

        // Create and spawn a future for the websocket
        let (ws_future, ws_rx, ws_tx) =
            WebSocketFuture::<ServerDeviceMessage, ClientDeviceMessage>::new(socket);

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
        match *self.state.read() {
            DeviceSessionState::Authenticated { device_id, .. } => Some(device_id),
            _ => None,
        }
    }

    pub fn send_message(&self, msg: ServerDeviceMessage) -> bool {
        self.tx.send(msg).is_ok()
    }

    pub fn send_encrypted_message(&self, msg: ServerDeviceMessageEncrypted) -> bool {
        let state = self.state.read();
        let cipher = match &*state {
            DeviceSessionState::AwaitingApproval { cipher } => cipher,
            DeviceSessionState::Authenticated { cipher, .. } => cipher,
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
                // TODO: Disconnect client

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

    pub fn reset_state(&self) {
        *self.state.write() = DeviceSessionState::Initial
    }

    pub fn revoke(&self) {
        self.send_encrypted_message(ServerDeviceMessageEncrypted::Revoked);
        self.reset_state();
    }

    pub fn decline(&self) {
        self.send_encrypted_message(ServerDeviceMessageEncrypted::Declined);
        self.reset_state();
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

        // Generate a random challenge bytes
        let mut challenge = [0u8; 128];
        OsRng.fill_bytes(&mut challenge);

        // Encrypt challenge
        let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
        let encrypted_challenge = match cipher.encrypt(&nonce, challenge.as_slice()) {
            Ok(value) => value,
            Err(err) => {
                // TODO: Disconnect client

                tracing::error!(?err, "failed to create challenge");
                self.send_message(ServerDeviceMessage::Error {
                    message: "failed to create challenge".to_string(),
                });
                return;
            }
        };

        // Move to challenge state
        {
            *self.state.write() = DeviceSessionState::Challenge {
                cipher,
                challenge: challenge.to_vec(),
                client_name: name,
                client_public_key: client_public_key.to_bytes(),
            };
        }

        // Notify device of challenge
        self.send_message(ServerDeviceMessage::AuthenticateChallenge {
            server_public_key: self.devices.server_key_pair.public_key.to_bytes(),
            challenge: encrypted_challenge,
            nonce: nonce.into(),
        });
    }

    pub fn on_approved(&self, device_id: DeviceId) {
        {
            let state = &mut *self.state.write();
            let cipher = match state {
                DeviceSessionState::AwaitingApproval { cipher } => cipher.clone(),
                _ => return,
            };

            // Authenticate the device session
            *state = DeviceSessionState::Authenticated { cipher, device_id };
        };

        self.send_encrypted_message(ServerDeviceMessageEncrypted::Approved { device_id });
        self.send_encrypted_message(ServerDeviceMessageEncrypted::Authenticated { device_id });
    }

    async fn handle_challenge_response(
        &self,
        cipher: XChaCha20Poly1305,
        server_challenge: Vec<u8>,
        //
        client_challenge: Vec<u8>,
        client_nonce: [u8; 24],
        //
        client_name: String,
        client_public_key: [u8; 32],
    ) {
        let client_nonce = XNonce::from(client_nonce);
        let client_challenge = match cipher.decrypt(&client_nonce, client_challenge.as_slice()) {
            Ok(value) => value,
            Err(err) => {
                // TODO: Disconnect client

                tracing::error!(?err, "failed to decrypt challenge");
                self.send_message(ServerDeviceMessage::Error {
                    message: "failed to decrypt challenge".to_string(),
                });
                return;
            }
        };

        // Challenge didn't match
        if client_challenge != server_challenge {
            // TODO: Disconnect client
            tracing::error!("incorrect challenge");
            self.send_message(ServerDeviceMessage::Error {
                message: "challenge does not match".to_string(),
            });
            return;
        }

        match self
            .devices
            .attempt_authenticate_device(&client_public_key)
            .await
        {
            // Public key is known and authenticated with an existing device
            Ok(Some(device_id)) => {
                tracing::debug!("auth complete");
                {
                    // Authenticate the device session
                    *self.state.write() = DeviceSessionState::Authenticated { cipher, device_id };
                };

                self.send_encrypted_message(ServerDeviceMessageEncrypted::Authenticated {
                    device_id,
                });
            }
            // Public key is not known or approved yet add approval request
            Ok(None) => {
                tracing::debug!("auth need approval");

                {
                    // Awaiting approval
                    *self.state.write() = DeviceSessionState::AwaitingApproval { cipher }
                }

                self.devices.add_device_request(
                    self.id,
                    self.socket_addr,
                    client_name,
                    client_public_key,
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

    /// Handle messages from the socket
    pub async fn handle_message(&self, message: ClientDeviceMessage) {
        let state = { self.state.read().clone() };

        match state {
            DeviceSessionState::Initial => match message {
                ClientDeviceMessage::InitiateHandshake { name, public_key } => {
                    self.handle_initiate_handshake(name, public_key)
                }

                _ => {
                    tracing::warn!(?message, "got unexpected message from unauthorized device");
                }
            },
            DeviceSessionState::Challenge {
                challenge: server_challenge,
                cipher,
                client_name,
                client_public_key,
            } => match message {
                ClientDeviceMessage::AuthenticateChallengeResponse {
                    challenge: client_challenge,
                    nonce,
                } => {
                    self.handle_challenge_response(
                        cipher,
                        server_challenge,
                        client_challenge,
                        nonce,
                        client_name,
                        client_public_key,
                    )
                    .await;
                }
                _ => {
                    tracing::warn!(?message, "got unexpected message from unauthorized device");
                }
            },
            DeviceSessionState::AwaitingApproval { .. } => {
                tracing::warn!(?message, "got unexpected message from unauthorized device");
            }
            DeviceSessionState::Authenticated { device_id, cipher } => {
                match message {
                    ClientDeviceMessage::Encrypted { message, nonce } => {
                        let nonce = XNonce::from(nonce);
                        let message = match cipher.decrypt(&nonce, message.as_slice()) {
                            Ok(value) => value,
                            Err(err) => {
                                // TODO: Disconnect client
                                tracing::error!(?err, "failed to decrypt message");
                                self.send_message(ServerDeviceMessage::Error {
                                    message: "failed to decrypt message".to_string(),
                                });
                                return;
                            }
                        };

                        let msg: ClientDeviceMessageEncrypted =
                            match serde_json::from_slice(&message) {
                                Ok(value) => value,
                                Err(err) => {
                                    // TODO: Disconnect client
                                    tracing::error!(?err, "failed to decode message");
                                    self.send_message(ServerDeviceMessage::Error {
                                        message: "failed to decode message".to_string(),
                                    });
                                    return;
                                }
                            };

                        self.handle_message_authenticated(device_id, msg).await
                    }
                    _ => {
                        tracing::warn!(?message, "got unexpected message from device");
                    }
                };
            }
        };
    }

    /// Handle message when authenticated as `device_id`
    pub async fn handle_message_authenticated(
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
