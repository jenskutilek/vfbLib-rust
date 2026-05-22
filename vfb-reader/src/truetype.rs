use crate::{
    buffer::{EntryReader, ReadExt},
    error::AtByteIndex,
    VfbError,
};
use bitflags::bitflags;
use error_stack::Report;
use serde::Serialize;
use vfb_macros::VfbEntry;

bitflags! {
    #[derive(serde::Serialize, Debug)]
    pub struct TrueTypeOptions: u16 {
        /// Use custom TrueType values
        const USE_CUSTOM_TT_VALUES = 1 << 0;
        /// Create VDMX table
        const CREATE_VDMX = 1 << 1;
        /// Add null character and space glyphs
        const ADD_NULL_CR_SPACE = 1 << 2;
    }
}

#[derive(VfbEntry, Serialize, Debug)]
pub enum TrueTypeValue {
    #[vfb(key = 0x33, reader = "read_value")]
    #[serde(rename = "max_zones")]
    MaxZones(i32),
    #[vfb(key = 0x34, reader = "read_value")]
    #[serde(rename = "max_twilight_points")]
    MaxTwilightPoints(i32),
    #[vfb(key = 0x35, reader = "read_value")]
    #[serde(rename = "max_storage")]
    MaxStorage(i32),
    #[vfb(key = 0x36, reader = "read_value")]
    #[serde(rename = "max_function_defs")]
    MaxFunctionDefs(i32),
    #[vfb(key = 0x37, reader = "read_value")]
    #[serde(rename = "max_instruction_defs")]
    MaxInstructionDefs(i32),
    #[vfb(key = 0x38, reader = "read_value")]
    #[serde(rename = "max_stack_elements")]
    MaxStackElements(i32),
    #[vfb(key = 0x39, reader = "read_head_flags_and_options")]
    #[serde(rename = "head_flags")]
    HeadFlags((TrueTypeOptions, u16)), // Top two bytes are options, bottom two bytes are head table flags
    #[vfb(key = 0x3a, reader = "read_value")]
    #[serde(rename = "head_units_per_em")]
    HeadUnitsPerEm(i32),
    #[vfb(key = 0x3b, reader = "read_value")]
    #[serde(rename = "head_mac_style")]
    HeadMacStyle(i32),
    #[vfb(key = 0x3c, reader = "read_value")]
    #[serde(rename = "head_lowest_rec_ppem")]
    HeadLowestRecPpem(i32),
    #[vfb(key = 0x3d, reader = "read_value")]
    #[serde(rename = "head_font_direction_hint")]
    HeadFontDirectionHint(i32),
    #[vfb(key = 0x3e, reader = "read_value")]
    #[serde(rename = "os2_us_weight_class")]
    Os2UsWeightClass(i32),
    #[vfb(key = 0x3f, reader = "read_value")]
    #[serde(rename = "os2_us_width_class")]
    Os2UsWidthClass(i32),
    #[vfb(key = 0x40, reader = "read_value")]
    #[serde(rename = "os2_fs_type")]
    Os2FsType(i32),
    #[vfb(key = 0x41, reader = "read_value")]
    #[serde(rename = "os2_y_subscript_x_size")]
    Os2YSubscriptXSize(i32),
    #[vfb(key = 0x42, reader = "read_value")]
    #[serde(rename = "os2_y_subscript_y_size")]
    Os2YSubscriptYSize(i32),
    #[vfb(key = 0x43, reader = "read_value")]
    #[serde(rename = "os2_y_subscript_x_offset")]
    Os2YSubscriptXOffset(i32),
    #[vfb(key = 0x44, reader = "read_value")]
    #[serde(rename = "os2_y_subscript_y_offset")]
    Os2YSubscriptYOffset(i32),
    #[vfb(key = 0x45, reader = "read_value")]
    #[serde(rename = "os2_y_superscript_x_size")]
    Os2YSuperscriptXSize(i32),
    #[vfb(key = 0x46, reader = "read_value")]
    #[serde(rename = "os2_y_superscript_y_size")]
    Os2YSuperscriptYSize(i32),
    #[vfb(key = 0x47, reader = "read_value")]
    #[serde(rename = "os2_y_superscript_x_offset")]
    Os2YSuperscriptXOffset(i32),
    #[vfb(key = 0x48, reader = "read_value")]
    #[serde(rename = "os2_y_superscript_y_offset")]
    Os2YSuperscriptYOffset(i32),
    #[vfb(key = 0x49, reader = "read_value")]
    #[serde(rename = "os2_y_strikeout_size")]
    Os2YStrikeoutSize(i32),
    #[vfb(key = 0x4A, reader = "read_value")]
    #[serde(rename = "os2_y_strikeout_position")]
    Os2YStrikeoutPosition(i32),
    #[vfb(key = 0x4B, reader = "read_value")]
    #[serde(rename = "os2_s_family_class")]
    Os2SFamilyClass(i32),
    #[vfb(key = 0x4C, reader = "read_panose")]
    #[serde(rename = "OpenTypeOS2Panose")]
    Os2Panose([i8; 10]),
    #[vfb(key = 0x4D, reader = "read_value")]
    #[serde(rename = "OpenTypeOS2TypoAscender")]
    Os2TypoAscender(i32),
    #[vfb(key = 0x4E, reader = "read_value")]
    #[serde(rename = "OpenTypeOS2TypoDescender")]
    Os2TypoDescender(i32),
    #[vfb(key = 0x4F, reader = "read_value")]
    #[serde(rename = "OpenTypeOS2TypoLineGap")]
    Os2TypoLineGap(i32),
    #[vfb(key = 0x50, reader = "read_value")]
    #[serde(rename = "os2_fs_selection")]
    Os2FsSelection(i32),
    #[vfb(key = 0x51, reader = "read_value")]
    #[serde(rename = "OpenTypeOS2WinAscent")]
    Os2UsWinAscent(i32),
    #[vfb(key = 0x52, reader = "read_value")]
    #[serde(rename = "OpenTypeOS2WinDescent")]
    Os2UsWinDescent(i32),
    #[vfb(key = 0x53, reader = "read_vec_u8_with_value_count")]
    #[serde(rename = "hdmx_ppms1")]
    HdmxPPMs1(Vec<u8>),
    #[vfb(key = 0x54, reader = "read_codepages")]
    #[serde(rename = "os2_ul_code_page_range")]
    Os2UlCodePageRange((i32, i32)),
    // There is no 0x55
    #[vfb(key = 0x56, reader = "read_value")]
    #[serde(rename = "head_creation")]
    HeadCreation(i32),
    #[vfb(key = 0x57, reader = "read_value")]
    #[serde(rename = "head_creation2")]
    HeadCreation2(i32),
    #[vfb(key = 0x58, reader = "read_vec_u8_with_value_count")]
    #[serde(rename = "hdmx_ppms2")]
    HdmxPPMs2(Vec<u8>),
    #[vfb(key = 0x5C, reader = "read_value")]
    #[serde(rename = "Average Width")]
    AverageWidth(i32),
}

impl<R: std::io::Read + std::io::Seek> EntryReader<'_, R> {
    pub fn read_truetype_values(&mut self) -> Result<Vec<TrueTypeValue>, Report<VfbError>> {
        let mut values = Vec::new();
        loop {
            let key = self.read_u8()?;
            if key == 0x32 {
                break;
            } // End marker
            if let Some(value) = TrueTypeValue::new_from_reader(key as u16, self)? {
                values.push(value);
            } else {
                return Err(VfbError::UnknownEntryKey { key: key as u16 }.into()).at_index(self);
            }
        }
        Ok(values)
    }

    fn read_head_flags_and_options(&mut self) -> Result<(TrueTypeOptions, u16), Report<VfbError>> {
        let flags_and_options = self.read_value()?;
        let flags: u16 = (flags_and_options & 0xFFFF) as u16;
        let options: u16 = ((flags_and_options >> 16) & 0xFFFF) as u16;
        Ok((TrueTypeOptions::from_bits_truncate(options), flags))
    }

    fn read_codepages(&mut self) -> Result<(i32, i32), Report<VfbError>> {
        let cp1 = self.read_value()?;
        let cp2 = self.read_value()?;
        Ok((cp1, cp2))
    }
}
