use serde::Serialize;

pub mod commands;
pub mod events;

pub type Device = String;
pub type Context = String;
pub type ActionId = String;

pub type Target = String;

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

/// Coordinates that identify the location of the action.
#[derive(Debug, Serialize)]
pub struct Coordinates {
    pub column: u32,
    pub row: u32,
}

#[derive(Debug, Serialize)]
#[serde(transparent)]
pub struct Settings(pub serde_json::Value);

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
pub enum Controller {
    Keypad,
    Encoder,
}
