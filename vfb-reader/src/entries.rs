use crate::entry::VfbEntry;
use serde::Serialize;
use std::io::BufReader;
use std::io::Cursor;

pub mod encoding;

#[derive(Serialize)]
pub enum VfbEntryTypes {
    Encoding(encoding::EncodingRecord),
}

/// Dispatch the decompilation to the appropriate function
pub fn decompile(entry: &VfbEntry) -> Option<VfbEntryTypes> {
    if entry.data.is_none() {
        return None;
    }
    let bytes = &entry.data.as_ref().unwrap().bytes;
    if bytes.len() == 0 {
        return None;
    }
    let mut r = BufReader::new(Cursor::new(bytes));
    let decompiled = match entry.key.as_str() {
        "Encoding Default" => encoding::decompile(&mut r),
        "Encoding" => encoding::decompile(&mut r),
        _ => None,
    };
    decompiled
}
