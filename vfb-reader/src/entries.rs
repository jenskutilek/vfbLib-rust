use crate::{buffer::VfbReader, error::VfbError};
use serde::Serialize;
use vfb_macros::VfbEntry;

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
pub struct Encoding(pub (u16, String));

impl<R> VfbReader<R>
where
    R: std::io::Read,
{
    pub fn read_encoding(&mut self) -> Result<Encoding, VfbError> {
        let gid = self.read_u16()?;
        let name = self.read_str_remainder()?;
        Ok(Encoding((gid, name)))
    }

    pub fn read_string(&mut self) -> Result<String, VfbError> {
        let string = self.read_str_remainder()?;

        Ok(string)
    }

    pub fn read_uint16(&mut self) -> Result<u16, VfbError> {
        let i = self.read_u16()?;
        Ok(i)
    }
}

#[derive(VfbEntry, Serialize)]
pub enum VfbEntry {
    #[vfb(key = 1501)]
    #[serde(rename = "Encoding Default")]
    EncodingDefault(Encoding),

    #[vfb(key = 1500)]
    #[serde(rename = "Encoding")]
    Encoding(Encoding),

    #[vfb(key = 1502)]
    #[serde(rename = "1502")]
    Unknown1502(u16),

    #[vfb(key = 518)]
    #[serde(rename = "518")]
    Unknown518,

    #[vfb(key = 257)]
    #[serde(rename = "257")]
    Unknown257(String),

    #[vfb(key = 1026)]
    #[serde(rename = "font_name")]
    FontName(String),

    #[vfb(key = 1503)]
    #[serde(rename = "Master Count")]
    MasterCount(u16),

    #[vfb(key = 1046)]
    #[serde(rename = "version")]
    Version(String),

    #[vfb(key = 1038)]
    #[serde(rename = "notice")]
    Notice(String),

    #[vfb(key = 1025)]
    #[serde(rename = "full_name")]
    FullName(String),

    #[vfb(key = 1027)]
    #[serde(rename = "family_name")]
    FamilyName(String),

    #[vfb(key = 1024)]
    #[serde(rename = "pref_family_name")]
    PrefFamilyName(String),

    #[vfb(key = 1056)]
    #[serde(rename = "menu_name")]
    MenuName(String),

    #[vfb(key = 1092)]
    #[serde(rename = "apple_name")]
    AppleName(String),

    #[vfb(key = 1028)]
    #[serde(rename = "weight")]
    Weight(String),

    #[vfb(key = 1065)]
    #[serde(rename = "width")]
    Width(String),

    #[vfb(key = 1069)]
    #[serde(rename = "License")]
    License(String),

    #[vfb(key = 1070)]
    #[serde(rename = "License URL")]
    LicenseUrl(String),

    #[vfb(key = 1037)]
    #[serde(rename = "copyright")]
    Copyright(String),

    #[vfb(key = 1061)]
    #[serde(rename = "trademark")]
    Trademark(String),

    #[vfb(key = 1062)]
    #[serde(rename = "designer")]
    Designer(String),

    #[vfb(key = 1063)]
    #[serde(rename = "designer_url")]
    DesignerUrl(String),

    #[vfb(key = 1064)]
    #[serde(rename = "vendor_url")]
    VendorUrl(String),

    #[vfb(key = 1039)]
    #[serde(rename = "source")]
    Source(String),

    #[vfb(key = 1034)]
    #[serde(rename = "is_fixed_pitch")]
    IsFixedPitch(u16),

    #[vfb(key = 1031)]
    #[serde(rename = "underline_thickness")]
    UnderlineThickness(u16),

    #[vfb(key = 1054)]
    #[serde(rename = "ms_charset")]
    MsCharset(u16),

    #[vfb(key = 1128)]
    #[serde(rename = "tt_version")]
    TtVersion(String),

    #[vfb(key = 1129)]
    #[serde(rename = "tt_u_id")]
    TtUId(String),

    #[vfb(key = 1127)]
    #[serde(rename = "style_name")]
    StyleName(String),

    #[vfb(key = 1137)]
    #[serde(rename = "pref_style_name")]
    PrefStyleName(String),

    #[vfb(key = 1139)]
    #[serde(rename = "mac_compatible")]
    MacCompatible(String),

    #[vfb(key = 1121)]
    #[serde(rename = "vendor")]
    Vendor(String),

    #[vfb(key = 1132)]
    #[serde(rename = "year")]
    Year(u16),

    #[vfb(key = 1130)]
    #[serde(rename = "version_major")]
    VersionMajor(u16),

    #[vfb(key = 1131)]
    #[serde(rename = "version_minor")]
    VersionMinor(u16),

    #[vfb(key = 1135)]
    #[serde(rename = "upm")]
    Upm(u16),

    #[vfb(key = 1090)]
    #[serde(rename = "fond_id")]
    FondId(u16),

    #[vfb(key = 1530)]
    #[serde(rename = "blue_values_num")]
    BlueValuesNum(u16),

    #[vfb(key = 1531)]
    #[serde(rename = "other_blues_num")]
    OtherBluesNum(u16),

    #[vfb(key = 1532)]
    #[serde(rename = "family_blues_num")]
    FamilyBluesNum(u16),

    #[vfb(key = 1533)]
    #[serde(rename = "family_other_blues_num")]
    FamilyOtherBluesNum(u16),

    #[vfb(key = 1534)]
    #[serde(rename = "stem_snap_h_num")]
    StemSnapHNum(u16),

    #[vfb(key = 1535)]
    #[serde(rename = "stem_snap_v_num")]
    StemSnapVNum(u16),

    #[vfb(key = 1267)]
    #[serde(rename = "font_style")]
    FontStyle(u16),

    #[vfb(key = 1057)]
    #[serde(rename = "pcl_id")]
    PclId(u16),

    #[vfb(key = 1058)]
    #[serde(rename = "vp_id")]
    VpId(u16),

    #[vfb(key = 1060)]
    #[serde(rename = "ms_id")]
    MsId(u16),

    #[vfb(key = 1059)]
    #[serde(rename = "pcl_chars_set")]
    PclCharsSet(String),

    #[vfb(key = 1270)]
    #[serde(rename = "hhea_line_gap")]
    HheaLineGap(u16),

    #[vfb(key = 1272)]
    #[serde(rename = "stemsnaplimit")]
    StemSnapLimit(u16),

    #[vfb(key = 1274)]
    #[serde(rename = "zoneppm")]
    ZonePpm(u16),

    #[vfb(key = 1275)]
    #[serde(rename = "codeppm")]
    CodePpm(u16),

    #[vfb(key = 1604)]
    #[serde(rename = "1604")]
    Unknown1604(u16),

    #[vfb(key = 2032)]
    #[serde(rename = "2032")]
    Unknown2032(u16),

    #[vfb(key = 2022)]
    #[serde(rename = "Export PCLT Table")]
    ExportPcltTable(u16),

    #[vfb(key = 2025)]
    #[serde(rename = "note")]
    Note(String),

    #[vfb(key = 2016)]
    #[serde(rename = "customdata")]
    CustomData(String),

    #[vfb(key = 1277)]
    #[serde(rename = "OpenType Class")]
    OpenTypeClass(String),

    #[vfb(key = 1513)]
    #[serde(rename = "Axis Count")]
    AxisCount(u16),

    #[vfb(key = 1514)]
    #[serde(rename = "Axis Name")]
    AxisName(String),

    #[vfb(key = 1504)]
    #[serde(rename = "Master Name")]
    MasterName(String),

    #[vfb(key = 1066)]
    #[serde(rename = "default_character")]
    DefaultCharacter(String),

    #[vfb(key = 2034)]
    #[serde(rename = "2034")]
    Unknown2034(String),

    #[vfb(key = 2012)]
    #[serde(rename = "mark")]
    Mark(u16),

    #[vfb(key = 2015)]
    #[serde(rename = "glyph.customdata")]
    GlyphCustomData(String),

    #[vfb(key = 2017)]
    #[serde(rename = "glyph.note")]
    GlyphNote(String),
}
