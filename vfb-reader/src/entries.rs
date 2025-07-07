use crate::entry::VfbEntry;
use serde::Serialize;
use std::io::BufReader;
use std::io::Cursor;

mod encoding;
mod string;

#[derive(Serialize)]
pub enum VfbEntryTypes {
    Encoding((u16, String)),
    String(String),
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
        "518" => string::decompile(&mut r),
        "257" => string::decompile(&mut r),
        "font_name" => string::decompile(&mut r),
        "version" => string::decompile(&mut r),
        "notice" => string::decompile(&mut r),
        "full_name" => string::decompile(&mut r),
        "family_name" => string::decompile(&mut r),
        "pref_family_name" => string::decompile(&mut r),
        "menu_name" => string::decompile(&mut r),
        "apple_name" => string::decompile(&mut r),
        "weight" => string::decompile(&mut r),
        "width" => string::decompile(&mut r),
        "License" => string::decompile(&mut r),
        "License URL" => string::decompile(&mut r),
        "copyright" => string::decompile(&mut r),
        "trademark" => string::decompile(&mut r),
        "designer" => string::decompile(&mut r),
        "designer_url" => string::decompile(&mut r),
        "vendor_url" => string::decompile(&mut r),
        "source" => string::decompile(&mut r),
        "tt_version" => string::decompile(&mut r),
        "tt_u_id" => string::decompile(&mut r),
        "style_name" => string::decompile(&mut r),
        "pref_style_name" => string::decompile(&mut r),
        "mac_compatible" => string::decompile(&mut r),
        "vendor" => string::decompile(&mut r),
        "pcl_chars_set" => string::decompile(&mut r),
        "note" => string::decompile(&mut r),
        "customdata" => string::decompile(&mut r),
        "OpenType Class" => string::decompile(&mut r),
        "Axis Name" => string::decompile(&mut r),
        "Master Name" => string::decompile(&mut r),
        "default_character" => string::decompile(&mut r),
        "2034" => string::decompile(&mut r),
        "glyph.customdata" => string::decompile(&mut r),
        "glyph.note" => string::decompile(&mut r),
        _ => None,
    };
    decompiled
}
