// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use serde::Deserialize;

pub mod layout;
pub mod option;

pub(crate) const X11_BASE_RULES: &str = "/usr/share/X11/xkb/rules/base.xml";
pub(crate) const X11_EXTRAS_RULES: &str = "/usr/share/X11/xkb/rules/base.extras.xml";

// Import these into the namespace to stay compatible with the older code in
// cosmic-settings
pub use layout::all_keyboard_layouts;
pub use layout::extra_keyboard_layouts;
pub use layout::keyboard_layouts;

pub use option::all_keyboard_options;
pub use option::extra_keyboard_options;
pub use option::keyboard_options;

/// Contains the name and description of a configuration.
#[derive(Debug, Deserialize, Clone)]
pub struct ConfigItem {
    pub name: String,
    #[serde(rename = "shortDescription")]
    pub short_description: Option<String>,
    pub description: String,
}
