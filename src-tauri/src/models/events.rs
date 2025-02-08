//! # Events
//!
//! Event models emitted from the server and received by plugin clients

use serde::Serialize;

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
        device: DeviceId,
        device_info: DeviceInfo,
    },

    DeviceDidDisconnect {
        device: DeviceId,
    },

    DialDown {
        action: ActionId,
        context: ContextId,
        device: DeviceId,
        payload: DialDownPayload,
    },

    DialRotate {
        action: ActionId,
        context: ContextId,
        device: DeviceId,
        payload: DialRotatePayload,
    },

    DialUp {
        action: ActionId,
        context: ContextId,
        device: DeviceId,
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
        context: ContextId,
        payload: serde_json::Value,
    },

    DidReceiveSettings {
        action: ActionId,
        context: ContextId,
        device: DeviceId,
        payload: DidReceiveSettingsPayload,
    },

    KeyDown {
        action: ActionId,
        context: ContextId,
        device: DeviceId,
        payload: KeyUpDownPayload,
    },

    KeyUp {
        action: ActionId,
        context: ContextId,
        device: DeviceId,
        payload: KeyUpDownPayload,
    },

    PropertyInspectorDidAppear {
        action: ActionId,
        context: ContextId,
        device: DeviceId,
    },

    PropertyInspectorDidDisappear {
        action: ActionId,
        context: ContextId,
        device: DeviceId,
    },

    SystemDidWakeUp,

    TitleParametersDidChange {
        action: ActionId,
        context: ContextId,
        device: DeviceId,
        payload: TitleParametersDidChangePayload,
    },

    TouchTap {
        action: ActionId,
        context: ContextId,
        device: DeviceId,
        payload: TouchTapPayload,
    },

    WillAppear {
        action: ActionId,
        context: ContextId,
        device: DeviceId,
        payload: WillAppearPayload,
    },

    WillDisappear {
        action: ActionId,
        context: ContextId,
        device: DeviceId,
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
pub struct TitleParameters {
    pub font_family: String,
    pub font_size: f32,
    pub font_style: FontStyle,
    pub font_underline: bool,
    pub show_title: bool,
    pub title_alignment: TitleAlignment,
    pub title_color: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum FontStyle {
    #[serde(rename = "")]
    Unset,
    #[serde(rename = "Bold Italic")]
    BoldItalic,
    Bold,
    Italic,
    Regular,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TitleAlignment {
    Bottom,
    Middle,
    Top,
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
pub enum Controller {
    Keypad,
    Encoder,
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
#[serde(transparent)]
pub struct Settings(pub serde_json::Value);

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

/// Coordinates that identify the location of the action.
#[derive(Debug, Serialize)]
pub struct Coordinates {
    pub column: u32,
    pub row: u32,
}

#[derive(Debug, Serialize)]
pub struct ApplicationDidLaunchPayload {
    pub application: String,
}

#[derive(Debug, Serialize)]
pub struct ApplicationDidTerminatePayload {
    pub application: String,
}

pub type DeviceId = String;
pub type ContextId = String;
pub type ActionId = String;

#[derive(Debug, Serialize)]
pub struct DeviceInfo {
    pub name: String,
    pub size: Size,
    #[serde(rename = "type")]
    pub ty: DeviceType,
}

// TODO: Typing
pub type DeviceType = String;

#[derive(Debug, Serialize)]
pub struct Size {
    pub columns: u32,
    pub rows: u32,
}
