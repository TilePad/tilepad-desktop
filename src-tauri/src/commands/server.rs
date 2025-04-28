use std::net::{IpAddr, Ipv4Addr};

use serde::Serialize;
use tauri::{AppHandle, Manager};

use crate::server::HTTP_PORT;

use super::CmdResult;

#[derive(Serialize)]
pub struct ServerConnectionInfo {
    interfaces: Vec<ServerInterface>,
    port: u16,
}

#[derive(Serialize)]
pub struct ServerInterface {
    name: String,
    addr: Ipv4Addr,
}

/// Gets a list of current device approval requests
#[tauri::command]
pub fn server_get_connection_info() -> CmdResult<ServerConnectionInfo> {
    let port = HTTP_PORT;
    let interfaces: Vec<ServerInterface> = local_ip_address::list_afinet_netifas()?
        .into_iter()
        .filter_map(|(name, addr)| match addr {
            IpAddr::V4(addr) => {
                if addr.is_loopback() {
                    return None;
                }

                Some((name, addr))
            }
            IpAddr::V6(_) => None,
        })
        .map(|(name, addr)| ServerInterface { name, addr })
        .collect();

    Ok(ServerConnectionInfo { interfaces, port })
}

/// Get the third party licenses file
#[tauri::command]
pub async fn server_get_licenses(app: AppHandle) -> CmdResult<String> {
    let file = app.path().resource_dir()?.join("THIRD_PARTY_LICENSES.md");
    let contents = tokio::fs::read_to_string(file).await?;
    Ok(contents)
}
