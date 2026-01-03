use serde::Deserialize;
use serde_xml_rs as xml;
use std::{
    fs::File,
    io::{self, BufReader},
};

use crate::{layout::KeyboardLayouts, option::KeyboardOptions};

/// Fetches a section from the xml file
pub(crate) fn get_section<Section: for<'a> Deserialize<'a>>(path: &str) -> io::Result<Section> {
    xml::from_reader(BufReader::new(File::open(path)?))
        .map_err(|why| io::Error::new(io::ErrorKind::InvalidData, format!("{}", why)))
}

/// Fetch all known sections in one go
pub fn read_config(path: &str) -> io::Result<(KeyboardLayouts, KeyboardOptions)> {
    xml::from_reader(BufReader::new(File::open(path)?))
        .map_err(|why| io::Error::new(io::ErrorKind::InvalidData, format!("{}", why)))
}
