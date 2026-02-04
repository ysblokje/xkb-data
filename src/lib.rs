// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use serde::Deserialize;
use std::io::{self};
use std::path::PathBuf;

use crate::section::{
    all_sections, fetch_extra_section, fetch_section, fetch_user_section, read_section,
};

pub mod models;
pub mod options;
pub mod section;

// As per XKB official configuration guide, on Linux we use `evdev.xml`
#[cfg(target_os = "linux")]
const RULES_FILE: &str = "evdev.xml";
#[cfg(target_os = "linux")]
const X11_BASE_RULES: &str = "/usr/share/X11/xkb/rules/evdev.xml";
#[cfg(target_os = "linux")]
const X11_EXTRAS_RULES: &str = "/usr/share/X11/xkb/rules/evdev.extras.xml";
// when not on Linux, we use `base.xml`
#[cfg(not(target_os = "linux"))]
const RULES_FILE: &str = "base.xml";
#[cfg(not(target_os = "linux"))]
const X11_BASE_RULES: &str = "/usr/share/X11/xkb/rules/base.xml";
#[cfg(not(target_os = "linux"))]
const X11_EXTRAS_RULES: &str = "/usr/share/X11/xkb/rules/base.extras.xml";

/// A list of keyboard layouts parsed from X11_BASE_RULES.
#[derive(Debug, Deserialize, Clone)]
pub struct KeyboardLayouts {
    #[serde(rename = "layoutList")]
    pub layout_list: LayoutList,
}

impl KeyboardLayouts {
    /// Fetch the layouts from the layout list.
    pub fn layouts(&self) -> &[KeyboardLayout] {
        &self.layout_list.layout
    }

    /// Fetch the layouts from the layout list.
    pub fn layouts_mut(&mut self) -> &mut [KeyboardLayout] {
        &mut self.layout_list.layout
    }
}

/// A list of keyboard layouts.
#[derive(Debug, Deserialize, Clone)]
pub struct LayoutList {
    layout: Vec<KeyboardLayout>,
}

/// A keyboard layout, which contains an optional list of variants, a name, and a description.
#[derive(Debug, Deserialize, Clone)]
pub struct KeyboardLayout {
    #[serde(rename = "configItem")]
    pub config_item: ConfigItem,
    #[serde(rename = "variantList")]
    pub variant_list: Option<VariantList>,
}

impl KeyboardLayout {
    /// Fetches the name of the keyboard layout.
    pub fn name(&self) -> &str {
        &self.config_item.name
    }

    /// Fetches a description of the layout.
    pub fn description(&self) -> &str {
        &self.config_item.description
    }

    /// Fetches a list of possible layout variants.
    pub fn variants(&self) -> Option<&Vec<KeyboardVariant>> {
        self.variant_list.as_ref().and_then(|x| x.variant.as_ref())
    }
}

/// Contains the name and description of a keyboard layout.
#[derive(Debug, Deserialize, Clone)]
pub struct ConfigItem {
    pub name: String,
    #[serde(rename = "shortDescription")]
    pub short_description: Option<String>,
    pub description: String,
    pub vendor: Option<String>,
}

/// A list of possible variants of a keyboard layout.
#[derive(Debug, Deserialize, Clone)]
pub struct VariantList {
    pub variant: Option<Vec<KeyboardVariant>>,
}

/// A variant of a keyboard layout.
#[derive(Debug, Deserialize, Clone)]
pub struct KeyboardVariant {
    #[serde(rename = "configItem")]
    pub config_item: ConfigItem,
}

impl KeyboardVariant {
    /// The name of this variant of a keybaord layout.
    pub fn name(&self) -> &str {
        &self.config_item.name
    }

    /// A description of this variant of a keyboard layout.
    pub fn description(&self) -> &str {
        &self.config_item.description
    }
}

/// Fetches a list of keyboard layouts from a path.
pub fn get_keyboard_layouts(path: &str) -> io::Result<KeyboardLayouts> {
    read_section(path)
}

/// Fetches a list of keyboard layouts from the X11_BASE_RULES const or the file defined in the X11_BASE_RULES_XML environment variable.
pub fn keyboard_layouts() -> io::Result<KeyboardLayouts> {
    fetch_section()
}

/// Fetches a list of keyboard layouts from the X11_EXTRAS_RULES const or the file defined in the X11_EXTRA_RULES_XML environment variable.
pub fn extra_keyboard_layouts() -> io::Result<KeyboardLayouts> {
    fetch_extra_section()
}

/// Fetches user-specific keyboard layouts from XDG config paths.
pub fn user_keyboard_layouts() -> io::Result<KeyboardLayouts> {
    fetch_user_section()
}

/// Returns the list of user XKB rules paths to check for a given rules file.
fn user_xkb_rules_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    // Defaults to $HOME/.config/xkb/rules/ if XDG_CONFIG_HOME is not set
    if let Ok(xdg_config_home) = std::env::var("XDG_CONFIG_HOME") {
        let mut path = PathBuf::from(xdg_config_home);
        path.push("xkb/rules");
        path.push(RULES_FILE);
        if path.exists() {
            paths.push(path);
        }
    } else if let Ok(home) = std::env::var("HOME") {
        let mut path = PathBuf::from(home);
        path.push(".config/xkb/rules");
        path.push(RULES_FILE);
        if path.exists() {
            paths.push(path);
        }
    }

    // Second priority: $HOME/.xkb/rules/
    if let Ok(home) = std::env::var("HOME") {
        let mut path = PathBuf::from(home);
        path.push(".xkb/rules");
        path.push(RULES_FILE);
        if path.exists() {
            paths.push(path);
        }
    }
    paths
}

/// Fetches a list of keyboard layouts from the system rules files and user config paths,
/// merging them together.
pub fn all_keyboard_layouts() -> io::Result<KeyboardLayouts> {
    all_sections()
}

/// A generic trait to access the Vec's
///
/// This helps DRY when it comes to reading different sections from the config
/// files.
///
pub trait AccessList {
    // The type that is stored in the list
    type InnerType;
    // read-only access to the embedded list
    fn get_list(&self) -> &[Self::InnerType];
    // read-write access to the embedded list
    fn get_mut_list(&mut self) -> &mut [Self::InnerType];
    // replace the embedded list
    fn set_list(&mut self, list: Vec<Self::InnerType>);
    // create a new object of type InnerType and fill with list
    fn new(list: Vec<Self::InnerType>) -> Self;
    // take ownership of the embedded list
    fn take_list(self) -> Vec<Self::InnerType>;
}

impl AccessList for KeyboardLayouts {
    type InnerType = KeyboardLayout;
    fn get_list(&self) -> &[KeyboardLayout] {
        self.layouts()
    }
    fn get_mut_list(&mut self) -> &mut [KeyboardLayout] {
        self.layouts_mut()
    }
    fn set_list(&mut self, list: Vec<KeyboardLayout>) {
        self.layout_list.layout = list;
    }

    fn take_list(self) -> Vec<Self::InnerType> {
        self.layout_list.layout
    }

    fn new(values: Vec<KeyboardLayout>) -> Self {
        Self {
            layout_list: LayoutList { layout: values },
        }
    }
}
