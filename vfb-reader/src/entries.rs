use crate::error::VfbError;
use serde::Serialize;
use std::io::{BufReader, Cursor};

mod encoding;
mod string;
mod uint16;

#[derive(Serialize)]
pub enum VfbEntryTypes {
    Encoding((u16, String)),
    String(String),
    UInt16(u16),
}

/// Dispatch the decompilation to the appropriate function
pub fn decompile(key: &str, bytes: &[u8]) -> Result<Option<VfbEntryTypes>, VfbError> {
    // The entry has data, but it is empty
    if bytes.is_empty() {
        return Ok(None);
    }

    // FIXME: Do we actually need to combine Cursor + BufReader?
    let mut r = BufReader::new(Cursor::new(bytes));

    // Match the entry key to the appropriate decompile function, return None for unknown keys
    match key {
        "Encoding Default" => encoding::decompile(&mut r),
        "Encoding" => encoding::decompile(&mut r),
        "1502" => uint16::decompile(&mut r),
        "518" => string::decompile(&mut r),
        "257" => string::decompile(&mut r),
        "font_name" => string::decompile(&mut r),
        "Master Count" => uint16::decompile(&mut r),
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
        "is_fixed_pitch" => uint16::decompile(&mut r),
        "underline_thickness" => uint16::decompile(&mut r),
        "ms_charset" => uint16::decompile(&mut r),
        "tt_version" => string::decompile(&mut r),
        "tt_u_id" => string::decompile(&mut r),
        "style_name" => string::decompile(&mut r),
        "pref_style_name" => string::decompile(&mut r),
        "mac_compatible" => string::decompile(&mut r),
        "vendor" => string::decompile(&mut r),
        "year" => uint16::decompile(&mut r),
        "version_major" => uint16::decompile(&mut r),
        "version_minor" => uint16::decompile(&mut r),
        "upm" => uint16::decompile(&mut r),
        "fond_id" => uint16::decompile(&mut r),
        "blue_values_num" => uint16::decompile(&mut r),
        "other_blues_num" => uint16::decompile(&mut r),
        "family_blues_num" => uint16::decompile(&mut r),
        "family_other_blues_num" => uint16::decompile(&mut r),
        "stem_snap_h_num" => uint16::decompile(&mut r),
        "stem_snap_v_num" => uint16::decompile(&mut r),
        "font_style" => uint16::decompile(&mut r),
        "pcl_id" => uint16::decompile(&mut r),
        "vp_id" => uint16::decompile(&mut r),
        "ms_id" => uint16::decompile(&mut r),
        "pcl_chars_set" => string::decompile(&mut r),
        "hhea_line_gap" => uint16::decompile(&mut r),
        "stemsnaplimit" => uint16::decompile(&mut r),
        "zoneppm" => uint16::decompile(&mut r),
        "codeppm" => uint16::decompile(&mut r),
        "1604" => uint16::decompile(&mut r),
        "2032" => uint16::decompile(&mut r),
        "Export PCLT Table" => uint16::decompile(&mut r),
        "note" => string::decompile(&mut r),
        "customdata" => string::decompile(&mut r),
        "OpenType Class" => string::decompile(&mut r),
        "Axis Count" => uint16::decompile(&mut r),
        "Axis Name" => string::decompile(&mut r),
        "Master Name" => string::decompile(&mut r),
        "default_character" => string::decompile(&mut r),
        "2034" => string::decompile(&mut r),
        "mark" => uint16::decompile(&mut r),
        "glyph.customdata" => string::decompile(&mut r),
        "glyph.note" => string::decompile(&mut r),
        _ => Ok(None),
    }
}
