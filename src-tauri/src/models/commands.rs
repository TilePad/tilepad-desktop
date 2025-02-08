//! # Commands
//!
//! Plugins send command payloads to the app

use serde::Serialize;

use super::{Context, Device, Settings, Target};

#[derive(Debug, Serialize)]
#[serde(tag = "event")]
#[serde(rename_all = "camelCase")]
#[serde(rename_all_fields = "camelCase")]
pub enum PluginCommand {
    GetGlobalSettings {
        context: String,
    },
    GetSettings {
        context: String,
    },
    OpenUrl {
        payload: OpenUrlPayload,
    },
    SendToPropertyInspector {
        context: Context,
        payload: serde_json::Value,
    },
    SetFeedback {
        context: Context,
        payload: serde_json::Value,
    },
    SetFeedbackLayout {
        context: Context,
        payload: SetFeedbackLayoutPayload,
    },
    SetGlobalSettings {
        context: Context,
        payload: Settings,
    },
    SetImage {
        context: Context,
        payload: Settings,
    },
    SetSettings {
        context: Context,
        payload: Settings,
    },
    SetState {
        context: Context,
        payload: SetStatePayload,
    },
    SetTitle {
        context: Context,
        payload: SetTitlePayload,
    },
    SetTriggerDescription {
        context: Context,
        payload: SetTriggerDescriptionPayload,
    },
    ShowAlert {
        context: Context,
    },
    ShowOk {
        context: Context,
    },
    SwitchToProfile {
        context: Context,
        device: Device,

        payload: SwitchToProfilePayload,
    },
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogMessagePayload {
    pub message: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenUrlPayload {
    pub url: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetFeedbackLayoutPayload {
    pub layout: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetImagePayload {
    pub image: Option<String>,
    pub state: Option<u32>,
    pub target: Option<Target>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetStatePayload {
    pub state: u32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetTitlePayload {
    pub state: Option<u32>,
    pub target: Option<Target>,
    pub title: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetTriggerDescriptionPayload {
    pub long_touch: Option<String>,
    pub push: Option<String>,
    pub rotate: Option<String>,
    pub touch: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwitchToProfilePayload {
    pub page: Option<u32>,
    pub profile: Option<String>,
}
