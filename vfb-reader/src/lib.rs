mod buffer;
mod entries;
pub mod entry;
mod error;
pub mod header;
mod vfb_constants;

use serde::Serialize;
use std::{fs::File, io::BufReader};

use crate::error::VfbError;

/// The main struct representing the VFB
#[derive(Serialize)]
pub struct Vfb {
    header: header::Header,
    entries: Vec<entry::VfbEntry>,
}

pub fn read_vfb(path: &str) -> Result<Vfb, VfbError> {
    let file = File::open(path).map_err(VfbError::FileOpenError)?;
    let mut r = BufReader::new(file);
    let header = header::read(&mut r)?;
    let mut vfb = Vfb {
        header,
        entries: Vec::new(),
    };
    let mut entry: entry::VfbEntry;
    loop {
        entry = entry::read(&mut r)?;
        if entry.key == "EOF" {
            // End of file, don't include
            break;
        }
        vfb.entries.push(entry);
    }
    Ok(vfb)
}
