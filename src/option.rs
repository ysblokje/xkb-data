use serde::Deserialize;
use serde_xml_rs as xml;
use std::{
    fs::File,
    io::{self, BufReader},
};

use crate::{ConfigItem, X11_BASE_RULES, X11_EXTRAS_RULES};

#[derive(Debug, Deserialize, Clone)]
pub struct XkbOptions {
    #[serde(rename = "optionList")]
    pub option_list: XkbOptionGroupList,
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
    xml::from_reader(BufReader::new(File::open(path)?))
        .map_err(|why| io::Error::new(io::ErrorKind::InvalidData, format!("{}", why)))
}
/// Fetches a list of keyboard options from `/usr/share/X11/xkb/rules/base.xml` or the file defined in the X11_BASE_RULES_XML environment variable.
pub fn xkb_options() -> io::Result<XkbOptions> {
    if let Ok(x11_base_rules_xml) = std::env::var("X11_BASE_RULES_XML") {
        get_xkb_options(&x11_base_rules_xml)
    } else {
        get_xkb_options(X11_BASE_RULES)
    }
}

/// Fetches a list of keyboard layouts from `/usr/share/X11/xkb/rules/base.extras.xml` or the file defined in the X11_EXTRA_RULES_XML environment variable.
pub fn extra_xkb_options() -> io::Result<XkbOptions> {
    if let Ok(x11_extra_rules_xml) = std::env::var("X11_EXTRA_RULES_XML") {
        get_xkb_options(&x11_extra_rules_xml)
    } else {
        get_xkb_options(X11_EXTRAS_RULES)
    }
}

/// Fetches a list of keyboard options from `/usr/share/X11/xkb/rules/base.xml` and
/// extends them with the list of keyboard options from `/usr/share/X11/xkb/rules/base.extras.xml`.
pub fn all_keyboard_layouts() -> io::Result<XkbOptions> {
    let base_rules = xkb_options();
    let extras_rules = extra_xkb_options();

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
fn merge_rules(base: XkbOptions, extras: XkbOptions) -> XkbOptions {
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
