mod buffer;
mod entry;
mod entry_ref;
mod header;
mod vfb_constants;

use serde::Serialize;
use std::fs::File;
use std::io::BufReader;

/// The main struct representing the VFB
#[derive(Serialize)]
pub struct Vfb {
    header: header::Header,
    entries: Vec<entry::VfbEntry>,
}

pub fn read_vfb(path: &str) -> Vfb {
    let file = File::open(path).expect("Failed to open file");
    let mut r = BufReader::new(file);
    let header = header::read(&mut r);
    let mut vfb = Vfb {
        header,
        entries: Vec::new(),
    };
    let mut entry: entry::VfbEntry;
    loop {
        entry = entry::read(&mut r);
        if entry.key == "5" {
            // End of file, don't include
            break;
        }
        vfb.entries.push(entry);
    }
    return vfb;
}
