use serde::Deserialize;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::models::{Controller, FontStyle, TitleAlignment};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Manifest {
    pub actions: Vec<Action>,
    pub applications_to_monitor: Option<ApplicationMonitoring>,
    pub author: String,
    pub category: Option<String>,
    pub category_icon: Option<String>,
    /// Path to the plugin's main entry point
    pub code_path: Option<String>,
    pub code_path_mac: Option<String>,
    pub code_path_win: Option<String>,
    pub default_window_size: Option<[u32; 2]>,
    pub description: String,
    pub icon: String,
    pub name: String,
    pub nodejs: Option<Nodejs>,
    #[serde(rename = "OS")]
    pub os: Vec<OS>,
    pub profiles: Option<Vec<Profile>>,
    pub property_inspector_path: Option<String>,
    #[serde(rename = "SDKVersion")]
    pub sdk_version: Option<u32>,
    pub software: Option<Software>,
    #[serde(rename = "URL")]
    pub url: Option<String>,
    #[serde(rename = "UUID")]
    pub uuid: String,
    pub version: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Action {
    #[serde(default = "default_controllers")]
    pub controllers: Vec<Controller>,
    #[serde(default)]
    pub disable_automatic_states: bool,
    #[serde(default)]
    pub disable_caching: bool,
    #[serde(default)]
    pub encoder: Encoder,
    pub icon: String,
    pub name: String,
    #[serde(rename = "OS")]
    pub os: Option<Vec<String>>,
    pub property_inspector_path: Option<String>,
    pub states: Vec<ActionState>,
    #[serde(default = "default_true")]
    pub supported_in_multi_actions: bool,
    pub tooltip: Option<String>,
    #[serde(rename = "UUID")]
    pub uuid: String,
    #[serde(default = "default_true")]
    pub user_title_enabled: bool,
    #[serde(default = "default_true")]
    pub visible_in_actions_list: bool,
}

fn default_controllers() -> Vec<Controller> {
    vec![Controller::Keypad]
}

fn default_true() -> bool {
    true
}

#[derive(Default, Debug, Deserialize)]
#[serde(default)]
pub struct ApplicationMonitoring {
    pub mac: Vec<String>,
    pub windows: Vec<String>,
}

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct Encoder {
    pub icon: Option<String>,
    pub stack_color: Option<String>,
    pub trigger_description: Option<TriggerDescriptions>,
    pub background: Option<String>,
    pub layout: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Nodejs {
    pub debug: Option<String>,
    pub generate_profiler_output: Option<bool>,
    pub version: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OS {
    pub platform: Platform,
    pub minimum_version: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Platform {
    Mac,
    Windows,
    Linux,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Profile {
    #[serde(default = "default_true")]
    pub auto_install: bool,
    pub device_type: DeviceType,
    #[serde(default)]
    pub dont_auto_switch_when_installed: bool,
    pub name: String,
    #[serde(default)]
    pub readonly: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Software {
    pub minimum_version: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ActionState {
    pub font_family: Option<String>,
    pub font_size: Option<u16>,
    pub font_style: Option<FontStyle>,
    pub font_underline: Option<bool>,
    pub image: String,
    pub multi_action_image: Option<String>,
    pub name: Option<String>,
    pub show_title: Option<bool>,
    pub title: Option<String>,
    pub title_alignment: Option<TitleAlignment>,
    pub title_color: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TriggerDescriptions {
    pub long_touch: Option<String>,
    pub push: Option<String>,
    pub rotate: Option<String>,
    pub touch: Option<String>,
}

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum DeviceType {
    StreamDeck = 0,
    StreamDeckMini = 1,
    StreamDeckXL = 2,
    StreamDeckMobile = 3,
    CorsairGKeys = 4,
    StreamDeckPedal = 5,
    CorsairVoyager = 6,
    StreamDeckPlus = 7,
    SCUFController = 8,
    StreamDeckNeo = 9,
}
