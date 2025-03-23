use std::{collections::HashMap, fmt::Display};

use garde::{
    error::{Kind, PathComponentKind},
    Validate,
};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

/// Version of a node runtime
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(transparent)]
pub struct NodeVersion(pub node_semver::Version);

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct Manifest {
    /// Details about the plugin itself
    #[garde(dive)]
    pub plugin: ManifestPlugin,

    /// Details for running the plugin
    /// (Option not specified for internal plugins)
    #[garde(dive)]
    pub bin: Option<ManifestBin>,

    /// Category for the manifest actions
    #[garde(dive)]
    pub category: ManifestCategory,

    /// Map of available plugin actions
    #[garde(dive)]
    pub actions: HashMap<ActionId, ManifestAction>,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct ManifestCategory {
    #[garde(length(min = 1))]
    pub label: String,
    #[garde(skip)]
    pub icon: Option<String>,
}

impl Manifest {
    pub fn parse(value: &str) -> anyhow::Result<Manifest> {
        let manifest: Manifest = toml::from_str(value)?;
        manifest.validate()?;
        Ok(manifest)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[garde(transparent)]
#[serde(transparent)]
pub struct PluginId(#[garde(custom(is_valid_plugin_name))] pub String);

impl PluginId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for PluginId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct ManifestPlugin {
    /// Unique ID of the plugin (e.g com.jacobtread.tilepad.obs)
    #[garde(dive)]
    pub id: PluginId,
    #[garde(length(min = 1))]
    pub name: String,
    #[garde(length(min = 1))]
    pub version: String,
    #[garde(skip)]
    pub authors: Vec<String>,
    #[garde(skip)]
    pub description: Option<String>,
    #[garde(skip)]
    pub icon: Option<String>,
    #[garde(skip)]
    pub internal: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[garde(transparent)]
#[serde(transparent)]
pub struct ActionId(#[garde(custom(is_valid_action_name))] pub String);

impl ActionId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for ActionId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Display for ActionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl PathComponentKind for ActionId {
    fn component_kind() -> Kind {
        Kind::Key
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct ManifestAction {
    #[garde(length(min = 1))]
    pub label: String,
    #[garde(skip)]
    pub icon: Option<String>,
    #[garde(skip)]
    pub description: Option<String>,
    #[garde(skip)]
    pub inspector: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(untagged)]
pub enum ManifestBin {
    /// Program uses the node runtime
    Node {
        #[garde(dive)]
        node: ManifestBinNode,
    },

    /// Program uses a native binary
    Native {
        #[garde(dive)]
        native: Vec<ManifestBinNative>,
    },
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct ManifestBinNode {
    /// Entrypoint for the program
    #[garde(length(min = 1))]
    pub entrypoint: String,

    /// Version of node the program should run using
    #[garde(skip)]
    pub version: NodeVersion,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct ManifestBinNative {
    #[garde(skip)]
    pub os: OperatingSystem,
    #[garde(skip)]
    pub arch: Arch,
    #[garde(length(min = 1))]
    pub path: String,
}

/// Separators allowed within names
static NAME_SEPARATORS: [char; 2] = ['-', '_'];

// Validates that a plugin name is valid
fn is_valid_plugin_name(value: &str, _context: &()) -> garde::Result {
    let parts = value.split('.');

    for part in parts {
        // Must start with a letter
        if !part.starts_with(|char: char| char.is_ascii_alphabetic()) {
            return Err(garde::Error::new(
                "segment must start with a ascii alphabetic character",
            ));
        }

        // Must only contain a-zA-Z0-9_-
        if !part
            .chars()
            .all(|char| char.is_alphanumeric() || NAME_SEPARATORS.contains(&char))
        {
            return Err(garde::Error::new(
                "plugin name domain segment must only contain alpha numeric values and _ or -",
            ));
        }

        // Must not end with - or _
        if part.ends_with(NAME_SEPARATORS) {
            return Err(garde::Error::new(
                "plugin name domain segment must not end with _ or -",
            ));
        }
    }

    Ok(())
}

// Validates that a action name is valid
fn is_valid_action_name(value: &str, _context: &()) -> garde::Result {
    // Must start with a letter
    if !value.starts_with(|char: char| char.is_ascii_alphabetic()) {
        return Err(garde::Error::new(
            "action name must start with a ascii alphabetic character",
        ));
    }

    // Must only contain a-zA-Z0-9_-
    if !value
        .chars()
        .all(|char| char.is_alphanumeric() || NAME_SEPARATORS.contains(&char))
    {
        return Err(garde::Error::new(
            "action name must only contain alpha numeric values and _ or -",
        ));
    }

    // Must not end with - or _
    if value.ends_with(NAME_SEPARATORS) {
        return Err(garde::Error::new("action name must not end with _ or -"));
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, Display, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OperatingSystem {
    #[strum(serialize = "windows")]
    Windows,
    #[strum(serialize = "linux")]
    Linux,
}

/// CPU architecture the binary is compiled as
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, Display, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Arch {
    #[strum(serialize = "x86")]
    X86,
    #[strum(serialize = "x64")]
    X64,
}

impl Default for Arch {
    fn default() -> Self {
        platform_arch()
    }
}

#[cfg(all(
    target_pointer_width = "64",
    not(any(target_arch = "arm", target_arch = "aarch64"))
))]
pub fn platform_arch() -> Arch {
    Arch::X64
}

#[cfg(all(
    target_pointer_width = "32",
    not(any(target_arch = "arm", target_arch = "aarch64"))
))]
pub fn platform_arch() -> Arch {
    Arch::X86
}
