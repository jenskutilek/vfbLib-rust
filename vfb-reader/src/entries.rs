use crate::{buffer::VfbReader, error::VfbError};
use serde::Serialize;

pub struct RawData(pub Vec<u8>);

impl Serialize for RawData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        hex::encode(self.0.clone()).serialize(serializer)
    }
}

#[derive(Serialize)]
pub enum VfbEntryType {
    Raw(RawData),
    Encoding((u16, String)),
    String(String),
    UInt16(u16),
}

impl<R> VfbReader<R>
where
    R: std::io::Read,
{
    pub fn decompile_encoding(&mut self) -> Result<Option<VfbEntryType>, VfbError> {
        let gid = self.read_u16()?;
        let name = self.read_str_remainder()?;

        Ok(Some(VfbEntryType::Encoding((gid, name))))
    }

    pub fn decompile_string(&mut self) -> Result<Option<VfbEntryType>, VfbError> {
        let string = self.read_str_remainder()?;

        Ok(Some(VfbEntryType::String(string)))
    }

    pub fn decompile_uint16(&mut self) -> Result<Option<VfbEntryType>, VfbError> {
        let i = self.read_u16()?;
        Ok(Some(VfbEntryType::UInt16(i)))
    }
}

/// Dispatch the decompilation to the appropriate function
pub fn decompile(key: &str, bytes: &[u8]) -> Result<Option<VfbEntryType>, VfbError> {
    // The entry has data, but it is empty
    if bytes.is_empty() {
        return Ok(None);
    }

    let mut r = VfbReader::new(bytes);

    // Match the entry key to the appropriate decompile function, return None for unknown keys
    match key {
        "Encoding Default" => r.decompile_encoding(),
        "Encoding" => r.decompile_encoding(),
        "1502" => r.decompile_uint16(),
        "518" => r.decompile_string(),
        "257" => r.decompile_string(),
        "font_name" => r.decompile_string(),
        "Master Count" => r.decompile_uint16(),
        "version" => r.decompile_string(),
        "notice" => r.decompile_string(),
        "full_name" => r.decompile_string(),
        "family_name" => r.decompile_string(),
        "pref_family_name" => r.decompile_string(),
        "menu_name" => r.decompile_string(),
        "apple_name" => r.decompile_string(),
        "weight" => r.decompile_string(),
        "width" => r.decompile_string(),
        "License" => r.decompile_string(),
        "License URL" => r.decompile_string(),
        "copyright" => r.decompile_string(),
        "trademark" => r.decompile_string(),
        "designer" => r.decompile_string(),
        "designer_url" => r.decompile_string(),
        "vendor_url" => r.decompile_string(),
        "source" => r.decompile_string(),
        "is_fixed_pitch" => r.decompile_uint16(),
        "underline_thickness" => r.decompile_uint16(),
        "ms_charset" => r.decompile_uint16(),
        "tt_version" => r.decompile_string(),
        "tt_u_id" => r.decompile_string(),
        "style_name" => r.decompile_string(),
        "pref_style_name" => r.decompile_string(),
        "mac_compatible" => r.decompile_string(),
        "vendor" => r.decompile_string(),
        "year" => r.decompile_uint16(),
        "version_major" => r.decompile_uint16(),
        "version_minor" => r.decompile_uint16(),
        "upm" => r.decompile_uint16(),
        "fond_id" => r.decompile_uint16(),
        "blue_values_num" => r.decompile_uint16(),
        "other_blues_num" => r.decompile_uint16(),
        "family_blues_num" => r.decompile_uint16(),
        "family_other_blues_num" => r.decompile_uint16(),
        "stem_snap_h_num" => r.decompile_uint16(),
        "stem_snap_v_num" => r.decompile_uint16(),
        "font_style" => r.decompile_uint16(),
        "pcl_id" => r.decompile_uint16(),
        "vp_id" => r.decompile_uint16(),
        "ms_id" => r.decompile_uint16(),
        "pcl_chars_set" => r.decompile_string(),
        "hhea_line_gap" => r.decompile_uint16(),
        "stemsnaplimit" => r.decompile_uint16(),
        "zoneppm" => r.decompile_uint16(),
        "codeppm" => r.decompile_uint16(),
        "1604" => r.decompile_uint16(),
        "2032" => r.decompile_uint16(),
        "Export PCLT Table" => r.decompile_uint16(),
        "note" => r.decompile_string(),
        "customdata" => r.decompile_string(),
        "OpenType Class" => r.decompile_string(),
        "Axis Count" => r.decompile_uint16(),
        "Axis Name" => r.decompile_string(),
        "Master Name" => r.decompile_string(),
        "default_character" => r.decompile_string(),
        "2034" => r.decompile_string(),
        "mark" => r.decompile_uint16(),
        "glyph.customdata" => r.decompile_string(),
        "glyph.note" => r.decompile_string(),
        _ => Ok(None),
    }
}
