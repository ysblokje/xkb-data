// Options as described in the XML files of xkeyboard-config
//

use serde::Deserialize;
use std::io::{self};

use crate::{
    fetch_section, fetch_user_section, read_section, section::fetch_extra_section, AccessList,
    ConfigItem,
};

#[derive(Debug, Deserialize, Clone)]
pub struct XkbOptions {
    #[serde(rename = "optionList")]
    pub option_list: XkbOptionGroupList,
}

impl AccessList for XkbOptions {
    type InnerType = XkbOptionGroup;

    fn get_list(&self) -> &[Self::InnerType] {
        &self.option_list.group
    }

    fn get_mut_list(&mut self) -> &mut [Self::InnerType] {
        &mut self.option_list.group
    }

    fn set_list(&mut self, list: Vec<Self::InnerType>) {
        self.option_list.group = list;
    }

    fn take_list(self) -> Vec<Self::InnerType> {
        self.option_list.group
    }

    fn new(list: Vec<Self::InnerType>) -> Self {
        Self {
            option_list: XkbOptionGroupList { group: list },
        }
    }
}

impl XkbOption {
    /// Fetches the name of the option group
    pub fn name(&self) -> &str {
        &self.config_item.name
    }

    /// Fetches a description of the option group
    pub fn description(&self) -> &str {
        &self.config_item.description
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct XkbOptionGroupList {
    pub group: Vec<XkbOptionGroup>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename(deserialize = "group"))]
pub struct XkbOptionGroup {
    #[serde(rename = "@allowMultipleSelection")]
    pub allow_multiple_selection: bool,
    #[serde(rename = "configItem")]
    pub config_item: ConfigItem,
    #[serde(rename = "option")]
    pub xkb_option: Vec<XkbOption>,
}

impl XkbOptionGroup {
    /// Fetches the name of the option group
    pub fn name(&self) -> &str {
        &self.config_item.name
    }

    /// Fetches a description of the option group
    pub fn description(&self) -> &str {
        &self.config_item.description
    }

    /// Fetches a list of possible options
    pub fn options(&self) -> &Vec<XkbOption> {
        self.xkb_option.as_ref()
    }

    /// Is selecting multiple options allowed
    pub fn multiple_options_allowed(&self) -> bool {
        self.allow_multiple_selection
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct XkbOption {
    #[serde(rename = "configItem")]
    pub config_item: ConfigItem,
}

/// Fetches a list of keyboard options from a path.
pub fn get_xkb_options(path: &str) -> io::Result<XkbOptions> {
    read_section(path)
}

/// Fetches a list of options
pub fn xkb_options() -> io::Result<XkbOptions> {
    fetch_section()
}

/// Fetches a list of options from extra's
pub fn extra_xkb_options() -> io::Result<XkbOptions> {
    fetch_extra_section()
}

/// Fetches a list of options from user files
pub fn user_xkb_options() -> io::Result<XkbOptions> {
    fetch_user_section()
}

/// Fetches a list of keyboard options from `/usr/share/X11/xkb/rules/base.xml` and
/// extends them with the list of keyboard options from `/usr/share/X11/xkb/rules/base.extras.xml`.
pub fn all_xkb_options() -> io::Result<XkbOptions> {
    let base_rules = xkb_options();
    let extras_rules = extra_xkb_options();

    match (base_rules, extras_rules) {
        (Ok(base_rules), Ok(extras_rules)) => Ok(merge_options(base_rules, extras_rules)),
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
fn merge_options(base: XkbOptions, extras: XkbOptions) -> XkbOptions {
    XkbOptions {
        option_list: concat_group_lists(vec![base.option_list, extras.option_list]),
    }
}

fn concat_group_lists(groups: Vec<XkbOptionGroupList>) -> XkbOptionGroupList {
    let mut new_groups = vec![];
    for group in groups.into_iter() {
        new_groups.extend(group.group);
    }
    XkbOptionGroupList { group: new_groups }
}
