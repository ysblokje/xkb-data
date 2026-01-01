use serde::Deserialize;
use serde_xml_rs as xml;
use std::{
    fs::File,
    io::{self, BufReader},
};

/// Fetches a section from the xml file
pub(crate) fn get_section<Section: for<'a> Deserialize<'a>>(path: &str) -> io::Result<Section> {
    xml::from_reader(BufReader::new(File::open(path)?))
        .map_err(|why| io::Error::new(io::ErrorKind::InvalidData, format!("{}", why)))
}
