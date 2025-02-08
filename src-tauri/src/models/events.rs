//! # Events
//!
//! Event models emitted from the server and received by plugin clients

use serde::Serialize;

use super::{
    ActionId, Context, Controller, Coordinates, Device, DeviceInfo, Settings, TitleParameters,
};

#[derive(Debug, Serialize)]
#[serde(tag = "event")]
#[serde(rename_all = "camelCase")]
#[serde(rename_all_fields = "camelCase")]
pub enum Events {
    ApplicationDidLaunch {
        payload: ApplicationDidLaunchPayload,
    },
    ApplicationDidTerminate {
        payload: ApplicationDidTerminatePayload,
    },

    DeviceDidConnect {
        device: Device,
        device_info: DeviceInfo,
    },

    DeviceDidDisconnect {
        device: Device,
    },

    DialDown {
        action: ActionId,
        context: Context,
        device: Device,
        payload: DialDownPayload,
    },

    DialRotate {
        action: ActionId,
        context: Context,
        device: Device,
        payload: DialRotatePayload,
    },

    DialUp {
        action: ActionId,
        context: Context,
        device: Device,
        payload: DialUpPayload,
    },

    DidReceiveDeepLink {
        payload: DidReceiveDeepLinkPayload,
    },

    DidReceiveGlobalSettings {
        payload: DidReceiveGlobalSettings,
    },

    #[serde(rename = "sendToPlugin")]
    DidReceivePropertyInspectorMessage {
        action: ActionId,
        context: Context,
        payload: serde_json::Value,
    },

    DidReceiveSettings {
        action: ActionId,
        context: Context,
        device: Device,
        payload: DidReceiveSettingsPayload,
    },

    KeyDown {
        action: ActionId,
        context: Context,
        device: Device,
        payload: KeyUpDownPayload,
    },

    KeyUp {
        action: ActionId,
        context: Context,
        device: Device,
        payload: KeyUpDownPayload,
    },

    PropertyInspectorDidAppear {
        action: ActionId,
        context: Context,
        device: Device,
    },

    PropertyInspectorDidDisappear {
        action: ActionId,
        context: Context,
        device: Device,
    },

    SystemDidWakeUp,

    TitleParametersDidChange {
        action: ActionId,
        context: Context,
        device: Device,
        payload: TitleParametersDidChangePayload,
    },

    TouchTap {
        action: ActionId,
        context: Context,
        device: Device,
        payload: TouchTapPayload,
    },

    WillAppear {
        action: ActionId,
        context: Context,
        device: Device,
        payload: WillAppearPayload,
    },

    WillDisappear {
        action: ActionId,
        context: Context,
        device: Device,
        payload: WillDisappearPayload,
    },
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WillDisappearPayload {
    pub controller: Controller,
    pub is_in_multi_action: bool,
    pub coordinates: Option<Coordinates>,
    pub settings: Settings,
    pub state: Option<u32>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WillAppearPayload {
    pub controller: Controller,
    pub is_in_multi_action: bool,
    pub coordinates: Option<Coordinates>,
    pub settings: Settings,
    pub state: Option<u32>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TouchTapPayload {
    pub controller: Controller,
    pub coordinates: Coordinates,
    pub hold: bool,
    pub settings: Settings,
    pub tap_pos: [f32; 2],
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TitleParametersDidChangePayload {
    pub controller: Controller,
    pub coordinates: Coordinates,
    pub settings: Settings,
    pub state: Option<u32>,
    pub title: String,
    pub title_parameters: TitleParameters,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyUpDownPayload {
    pub controller: Controller,
    pub coordinates: Option<Coordinates>,
    pub is_in_multi_action: bool,
    pub settings: Settings,
    pub state: Option<u32>,
    pub user_desired_state: Option<u32>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DidReceiveSettingsPayload {
    pub controller: Controller,
    pub coordinates: Option<Coordinates>,
    pub is_in_multi_action: bool,
    pub settings: Settings,
    pub state: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct DidReceiveDeepLinkPayload {
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct DidReceiveGlobalSettings {
    pub settings: Settings,
}

#[derive(Debug, Serialize)]
pub struct DialDownPayload {
    pub controller: Controller,
    pub coordinates: Coordinates,
    pub settings: Settings,
}

#[derive(Debug, Serialize)]
pub struct DialUpPayload {
    pub controller: Controller,
    pub coordinates: Coordinates,
    pub settings: Settings,
}

#[derive(Debug, Serialize)]
pub struct DialRotatePayload {
    pub controller: Controller,
    /// Coordinates that identify the location of the action.
    pub coordinates: Coordinates,
    /// Determines whether the dial was pressed whilst the rotation occurred.
    pub pressed: bool,
    /// Settings associated with the action instance.
    pub settings: Settings,
    /// Number of ticks the dial was rotated; this can be a positive (clockwise) or negative (counter-clockwise) number.
    pub ticks: f32,
}

#[derive(Debug, Serialize)]
pub struct ApplicationDidLaunchPayload {
    pub application: String,
}

#[derive(Debug, Serialize)]
pub struct ApplicationDidTerminatePayload {
    pub application: String,
}
