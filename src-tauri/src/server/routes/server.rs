use axum::Json;

use crate::server::models::server::ServerDetails;

const IDENTIFIER: &str = "TILEPAD_CONTROLLER_SERVER";

/// GET /server/details
///
/// Get simple details about the server, used to check if a server
/// is alive by device clients
pub async fn details() -> Json<ServerDetails> {
    Json(ServerDetails {
        identifier: IDENTIFIER,
        hostname: gethostname::gethostname().to_string_lossy().to_string(),
    })
}
