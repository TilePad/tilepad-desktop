use serde::Serialize;

#[derive(Serialize)]
pub struct ServerDetails {
    pub identifier: &'static str,
}
