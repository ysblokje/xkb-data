//! Read sections present in xkb XML config files
//!
use serde_xml_rs as xml;
use std::{
    fs::File,
    io::{self, BufReader},
};

use serde::Deserialize;

use crate::{user_xkb_rules_paths, AccessList, X11_BASE_RULES, X11_EXTRAS_RULES};

/// Fetches a section from an xml file
pub fn read_section<Section>(path: &str) -> io::Result<Section>
where
    Section: for<'a> Deserialize<'a>,
{
    let retval = xml::from_reader(BufReader::new(File::open(path)?))
        .map_err(|why| io::Error::new(io::ErrorKind::InvalidData, format!("{}", why)));
    retval
}

/// Fetches a section from the X11_BASE_RULES const or the file defined in the
/// X11_BASE_RULES_XML environment variable.
pub fn fetch_section<Section: for<'a> Deserialize<'a>>() -> io::Result<Section> {
    if let Ok(x11_base_rules_xml) = std::env::var("X11_BASE_RULES_XML") {
        read_section(&x11_base_rules_xml)
    } else {
        read_section(X11_BASE_RULES)
    }
}

/// Fetches a section from the X11_EXTRAS_RULES const or the file defined in the X11_EXTRA_RULES_XML environment variable.
pub fn fetch_extra_section<Section: for<'a> Deserialize<'a>>() -> io::Result<Section> {
    if let Ok(x11_extra_rules_xml) = std::env::var("X11_EXTRA_RULES_XML") {
        read_section(&x11_extra_rules_xml)
    } else {
        read_section(X11_EXTRAS_RULES)
    }
}

/// Fetches user-specific section from XDG config paths.
/// On Linux, looks for `evdev.xml`, on other systems looks for `base.xml`.
/// Returns all layouts found from these paths:
/// 1. `$XDG_CONFIG_HOME/xkb/rules/` (defaults to `$HOME/.config/xkb/rules/`)
/// 2. `$HOME/.xkb/rules/`
/// Returns empty layouts if no files are found.
pub fn fetch_user_section<Section, InnerType>() -> io::Result<Section>
where
    Section: for<'a> Deserialize<'a>,
    Section: AccessList<InnerType = InnerType>,
    InnerType: Clone,
{
    let paths = user_xkb_rules_paths();

    let lists: Vec<Vec<InnerType>> = paths
        .into_iter()
        .filter_map(|path| read_section::<Section>(path.to_string_lossy().as_ref()).ok())
        .map(|thing| AccessList::get_list(&thing).to_vec())
        .collect();

    let new_list = lists.into_iter().flatten().collect::<Vec<InnerType>>();
    Ok(AccessList::new(new_list))
}

/// Fetches a section from the system rules files and user config paths,
/// merging them together.
pub fn all_sections<Section, InnerType>() -> io::Result<Section>
where
    Section: for<'a> Deserialize<'a>,
    Section: AccessList<InnerType = InnerType>,
    InnerType: Clone,
{
    let base_rules = fetch_section::<Section>().unwrap_or(AccessList::new(vec![]));
    let extras_rules = fetch_extra_section::<Section>().unwrap_or(AccessList::new(vec![]));
    let user_rules = fetch_user_section::<Section, InnerType>().unwrap_or(AccessList::new(vec![]));

    let lists = vec![
        base_rules.take_list(),
        extras_rules.take_list(),
        user_rules.take_list(),
    ];

    let new_list = lists.into_iter().flatten().collect::<Vec<InnerType>>();
    Ok(AccessList::new(new_list))
}
