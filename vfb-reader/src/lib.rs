mod buffer;
mod header;
mod vfb_constants;

use hex;
use serde::Serialize;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct VfbEntryData {
    bytes: Vec<u8>,
}

impl Serialize for VfbEntryData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        hex::encode(self.bytes.clone()).serialize(serializer)
    }
}

#[derive(Serialize)]
struct VfbEntry {
    // VfbEntry<'a>
    key: String,
    size: u32,
    // bytes: Vec<u8>,
    // bytes: &'a Vec<u8>,
    data: VfbEntryData,
}

#[derive(Serialize)]
pub struct VfbObject {
    header: header::VfbHeader,
    entries: Vec<VfbEntry>,
}

/// Read a VfbEntry from the stream and return it
fn read_entry<R>(r: &mut BufReader<R>) -> VfbEntry
where
    R: std::io::Read,
{
    // Read the key
    let raw_key = buffer::read_u16(r);
    // The raw key may be masked with 0x8000 to indicate a u32 data size
    let key = raw_key & !0x8000;

    // Read the size
    let size: u32;
    if raw_key & 0x8000 > 0 {
        size = buffer::read_u32(r);
    } else {
        size = buffer::read_u16(r).into();
    }

    // Read the data
    // TODO: This may be inefficient. What is the best way to store it, to copy the
    // buffer, or use a Vec like now?
    let mut bytes: Vec<u8> = vec![0u8; size.try_into().unwrap()];
    r.read_exact(&mut bytes).expect("ValueError");

    // Convert the key to human-readable string form using the VFB_KEYS
    let strkey = key.to_string();
    let humankey: String;
    if vfb_constants::VFB_KEYS.contains_key(&strkey) {
        humankey = vfb_constants::VFB_KEYS
            .get(&strkey)
            .expect("Unknown VFB key")
            .to_string()
    } else {
        println!("Unknown key in VFB keys: {}", strkey);
        humankey = strkey;
    }

    // Return the entry
    return VfbEntry {
        key: humankey,
        size,
        data: VfbEntryData { bytes },
    };
}

pub fn read_vfb(path: &str) -> VfbObject {
    let file = File::open(path).expect("Failed to open file");
    let mut r = BufReader::new(file);
    let header = header::read(&mut r);
    let mut vfb = VfbObject {
        header,
        entries: Vec::new(),
    };
    let mut entry: VfbEntry;
    loop {
        entry = read_entry(&mut r);
        if entry.key == "5" {
            // End of file, don't include
            break;
        }
        vfb.entries.push(entry);
    }
    return vfb;
}
