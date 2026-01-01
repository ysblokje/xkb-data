use serde::Deserialize;
use std::io;

use crate::{reader, ConfigItem, X11_BASE_RULES, X11_EXTRAS_RULES};

#[derive(Debug, Deserialize, Clone)]
pub struct KeyboardOptions {
    #[serde(rename = "optionList")]
    pub option_list: KeyboardOptionGroupList,
}

#[derive(Debug, Deserialize, Clone)]
pub struct KeyboardOptionGroupList {
    pub group: Vec<KeyboardOptionGroup>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename(deserialize = "group"))]
pub struct KeyboardOptionGroup {
    #[serde(rename = "@allowMultipleSelection")]
    pub allow_multiple_selection: bool,
    #[serde(rename = "configItem")]
    pub config_item: ConfigItem,
    #[serde(rename = "option")]
    pub keyboard_option: Vec<KeyboardOption>,
}

impl KeyboardOptionGroup {
    /// Fetches the name of the option group
    pub fn name(&self) -> &str {
        &self.config_item.name
    }

    /// Fetches a description of the option group
    pub fn description(&self) -> &str {
        &self.config_item.description
    }

    /// Fetches a list of possible options
    pub fn options(&self) -> &Vec<KeyboardOption> {
        self.keyboard_option.as_ref()
    }

    /// Is selecting multiple options allowed
    pub fn multiple_options_allowed(&self) -> bool {
        self.allow_multiple_selection
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct KeyboardOption {
    #[serde(rename = "configItem")]
    pub config_item: ConfigItem,
}

/// Fetches a list of keyboard options from a path.
pub fn get_keyboard_options(path: &str) -> io::Result<KeyboardOptions> {
    reader::get_section(path)
}
/// Fetches a list of keyboard options from `/usr/share/X11/xkb/rules/base.xml` or the file defined in the X11_BASE_RULES_XML environment variable.
pub fn keyboard_options() -> io::Result<KeyboardOptions> {
    if let Ok(x11_base_rules_xml) = std::env::var("X11_BASE_RULES_XML") {
        get_keyboard_options(&x11_base_rules_xml)
    } else {
        get_keyboard_options(X11_BASE_RULES)
    }
}

/// Fetches a list of keyboard layouts from `/usr/share/X11/xkb/rules/base.extras.xml` or the file defined in the X11_EXTRA_RULES_XML environment variable.
pub fn extra_keyboard_options() -> io::Result<KeyboardOptions> {
    if let Ok(x11_extra_rules_xml) = std::env::var("X11_EXTRA_RULES_XML") {
        get_keyboard_options(&x11_extra_rules_xml)
    } else {
        get_keyboard_options(X11_EXTRAS_RULES)
    }
}

/// Fetches a list of keyboard options from `/usr/share/X11/xkb/rules/base.xml` and
/// extends them with the list of keyboard options from `/usr/share/X11/xkb/rules/base.extras.xml`.
pub fn all_keyboard_options() -> io::Result<KeyboardOptions> {
    let base_rules = keyboard_options();
    let extras_rules = extra_keyboard_options();

    match (base_rules, extras_rules) {
        (Ok(base_rules), Ok(extras_rules)) => Ok(merge_rules(base_rules, extras_rules)),
        (Err(why), _) => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("{}", why),
        )),
        (_, Err(why)) => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("{}", why),
        )),
    }
}
fn merge_rules(base: KeyboardOptions, extras: KeyboardOptions) -> KeyboardOptions {
    KeyboardOptions {
        option_list: concat_group_lists(vec![base.option_list, extras.option_list]),
    }
}

fn concat_group_lists(groups: Vec<KeyboardOptionGroupList>) -> KeyboardOptionGroupList {
    let mut new_groups = vec![];
    for group in groups.into_iter() {
        new_groups.extend(group.group);
    }
    KeyboardOptionGroupList { group: new_groups }
}
