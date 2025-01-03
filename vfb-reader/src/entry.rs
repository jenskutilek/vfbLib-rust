use crate::buffer;
use crate::vfb_constants;

use hex;
use serde::Serialize;
use std::io::prelude::*;
use std::io::BufReader;

pub struct VfbEntryData {
    pub bytes: Vec<u8>,
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
pub struct VfbEntry {
    pub key: String,
    pub size: u32,
    pub data: VfbEntryData,
}

/// Read a VfbEntry from the stream and return it
pub fn read<R>(r: &mut BufReader<R>) -> VfbEntry
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
