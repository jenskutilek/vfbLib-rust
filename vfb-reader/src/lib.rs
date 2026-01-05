mod buffer;
pub mod entries;
mod error;
mod guides;
pub mod header;
mod names;
mod postscript;

use serde::Serialize;
use std::{fs::File, path::PathBuf};

use crate::buffer::VfbReader;
pub use entries::VfbEntry;
pub use error::VfbError; // Re-export the error type since we return it

/// The main struct representing the VFB
#[derive(Serialize)]
pub struct Vfb {
    header: header::Header,
    entries: Vec<VfbEntry>,
}

pub fn read_vfb(path: impl Into<PathBuf>) -> Result<Vfb, VfbError> {
    let file = File::open(path.into()).map_err(VfbError::FileOpenError)?;
    let mut r = VfbReader::new(file);
    let header = r.read_header()?;
    let mut vfb = Vfb {
        header,
        entries: Vec::new(),
    };
    loop {
        let (key, entry_opt) = r.read_entry()?;
        if key == 5 {
            // End of file marker (key 5 = EOF), don't include
            break;
        }
        if let Some(entry) = entry_opt {
            vfb.entries.push(entry);
        }
    }
    Ok(vfb)
}
