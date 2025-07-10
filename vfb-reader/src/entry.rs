use crate::{
    buffer::VfbReader,
    entries::{decompile, RawData, VfbEntryType},
    error::VfbError,
    vfb_constants,
};
use serde::Serialize;
use std::io::prelude::*;

#[derive(Serialize)]
pub struct VfbEntry {
    pub key: String,
    pub entry: VfbEntryType,
}

impl VfbEntry {
    // Build the entry from binary data
    pub fn new_from_data(key: String, data: Vec<u8>, decompile: bool) -> Result<Self, VfbError> {
        let mut slf = Self {
            key,
            entry: VfbEntryType::Raw(RawData(data)),
        };
        if decompile {
            slf.decompile()?;
        }
        Ok(slf)
    }

    // Build the entry from structured data
    pub fn new_from_decompiled(key: String, entry: VfbEntryType) -> Self {
        Self { key, entry }
    }

    // Decompile the entry and store the result in the entry
    pub fn decompile(&mut self) -> Result<(), VfbError> {
        if let VfbEntryType::Raw(bytes) = &self.entry {
            if let Some(decompiled) = decompile(&self.key, &bytes.0)? {
                self.entry = decompiled;
            }
        }
        Ok(())
    }
}

impl<R> VfbReader<R>
where
    R: std::io::Read,
{
    /// Read a VfbEntry from the stream and return it
    pub fn read_entry(&mut self) -> Result<VfbEntry, VfbError> {
        // Read the key
        let raw_key = self.read_u16()?;
        // The raw key may be masked with 0x8000 to indicate a u32 data size
        let key = raw_key & !0x8000;

        // Read the size
        let size: u32 = if raw_key & 0x8000 > 0 {
            self.read_u32()?
        } else {
            self.read_u16()?.into()
        };

        // Read the data
        // TODO: This may be inefficient. What is the best way to store it, to copy the
        // buffer, or use a Vec like now?
        let mut bytes: Vec<u8> = vec![0u8; size.try_into().map_err(|_| VfbError::Overflow(size))?];
        self.reader().read_exact(&mut bytes)?;

        // Convert the key to human-readable string form using the VFB_KEYS
        let strkey = key.to_string();
        let humankey: String = vfb_constants::VFB_KEYS
            .get(&strkey)
            .map(|&s| s.to_string())
            .unwrap_or_else(|| {
                println!("Unknown key in VFB keys: {}", strkey);
                strkey
            });

        // Return the entry
        VfbEntry::new_from_data(humankey, bytes, true)
    }
}
