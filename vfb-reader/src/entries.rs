use crate::entry::VfbEntry;
use serde::Serialize;
use std::io::BufReader;
use std::io::Cursor;

pub mod encoding;

#[derive(Serialize)]
pub enum VfbEntryTypes {
    Encoding((u16, String)),
}

/// Dispatch the decompilation to the appropriate function
pub fn decompile(entry: &VfbEntry) -> Option<VfbEntryTypes> {
    // The entry has no data
    if entry.data.is_none() {
        return None;
    }

    let bytes = &entry.data.as_ref().unwrap().bytes;

    // The entry has data, but it is empty
    if bytes.len() == 0 {
        return None;
    }

    // FIXME: Do we actually need to combine Cursor + BufReader?
    let mut r = BufReader::new(Cursor::new(bytes));

    // Match the entry key to the appropriate decompile function, return None for unknown keys
    let decompiled = match entry.key.as_str() {
        "Encoding Default" => encoding::decompile(&mut r),
        "Encoding" => encoding::decompile(&mut r),
        _ => None,
    };
    decompiled
}
