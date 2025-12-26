mod buffer;
mod entries;
pub mod entry;
mod error;
pub mod header;
mod vfb_constants;

use serde::Serialize;
use std::fs::File;

use crate::buffer::VfbReader;
pub use error::VfbError; // Re-export the error type since we return it

/// The main struct representing the VFB
#[derive(Serialize)]
pub struct Vfb {
    header: header::Header,
    entries: Vec<entry::VfbEntry>,
}

pub fn read_vfb(path: &str) -> Result<Vfb, VfbError> {
    let file = File::open(path).map_err(VfbError::FileOpenError)?;
    let mut r = VfbReader::new(file);
    let header = r.read_header()?;
    let mut vfb = Vfb {
        header,
        entries: Vec::new(),
    };
    let mut entry: entry::VfbEntry;
    loop {
        entry = r.read_entry()?;
        if entry.key == "EOF" {
            // End of file, don't include
            break;
        }
        vfb.entries.push(entry);
    }
    Ok(vfb)
}
