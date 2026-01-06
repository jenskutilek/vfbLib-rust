use crate::{
    buffer::{EntryReader, ReadExt},
    error::VfbError,
    glyph::GlyphEntry,
    guides::Guides,
    names::NameRecord,
    postscript::{PostScriptGlobalHintingOptions, PostScriptGlyphHintingOptions},
};
use error_stack::Report;
use font_types::Tag;
use serde::Serialize;
use std::io::Read;
use vfb_macros::VfbEntry;

#[derive(Debug)]
pub struct RawData(pub Vec<u8>);

#[derive(Serialize, Debug)]
pub struct BinaryTable((Tag, RawData));

// Placeholder type aliases for parser-derived types. These will be refined later.
pub type Panose = RawData;
pub type VendorId = RawData;
pub type EncodedValueListWithCount = RawData;
pub type TrueTypeInfo = RawData;
pub type Vdmx = RawData;
pub type TrueTypeStemPpems23 = RawData;
pub type TrueTypeStemPpems = RawData;
pub type TrueTypeStems = RawData;
pub type TrueTypeStemPpems1 = RawData;
pub type TrueTypeZones = RawData;
pub type UnicodeRanges = RawData;
pub type TrueTypeZoneDeltas = RawData;
pub type CustomCmap = RawData;
pub type Pclt = RawData;
pub type OpenTypeMetricsClassFlags = RawData;
pub type OpenTypeKerningClassFlags = RawData;
pub type AnisotropicInterpolations = RawData;
pub type AxisMappingsCount = RawData;
pub type AxisMappings = RawData;
pub type PrimaryInstances = RawData;
pub type GlobalMask = RawData;
pub type Mask = RawData;
pub type MaskMetrics = RawData;
pub type MaskMetricsMM = RawData;
pub type Glyph = Vec<GlyphEntry>;
pub type Link = RawData;
pub type BackgroundBitmap = RawData;
pub type GlyphBitmaps = RawData;
pub type EncodedValueList = RawData;
pub type GlyphSketch = RawData;
pub type GlyphOrigin = RawData;
pub type GlyphUnicode = RawData;
pub type GlyphUnicodesSupp = RawData;
pub type GlyphGDEF = RawData;
pub type GlyphAnchorsSupp = RawData;
pub type GlyphAnchors = RawData;
pub type GuideProperties = RawData;
pub type MappingMode = RawData;
pub type FL3Type1410 = RawData;
pub type MasterLocation = RawData;
pub type PostScriptInfo = RawData;

impl Serialize for RawData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        hex::encode(self.0.clone()).serialize(serializer)
    }
}

#[derive(Serialize, Debug)]
pub struct Encoding(pub (u16, String));

use bitflags::bitflags;
bitflags! {
    #[derive(Serialize, Debug)]
    pub struct ExportOptions: u16 {
        const USE_CUSTOM_OPENTYPE_EXPORT_OPTIONS = 1 << 0;
        const USE_DEFAULT_OPENTYPE_EXPORT_OPTIONS = 1 << 1;
        const USE_CUSTOM_CMAP_ENCODING = 1 << 2;
    }
}
impl From<u16> for ExportOptions {
    fn from(value: u16) -> Self {
        ExportOptions::from_bits_truncate(value)
    }
}

impl<R: std::io::Read + std::io::Seek> EntryReader<'_, R> {
    pub fn read_encoding(&mut self) -> Result<Encoding, Report<VfbError>> {
        let gid = self.read_u16()?;
        let name = self.read_str_remainder()?;
        Ok(Encoding((gid, name)))
    }

    // Not technically necessary but it makes the code read easier
    pub fn read_string(&mut self) -> Result<String, Report<VfbError>> {
        self.read_str_remainder()
    }

    pub fn read_double_list(&mut self) -> Result<Vec<f64>, Report<VfbError>> {
        // Just keep reading until the end
        let mut list = vec![];
        while let Ok(value) = self.read_f64() {
            list.push(value);
        }
        Ok(list)
    }

    pub fn read_int_list(&mut self) -> Result<Vec<u16>, Report<VfbError>> {
        // Just keep reading until the end
        let mut list = vec![];
        while let Ok(value) = self.read_u16() {
            list.push(value);
        }
        Ok(list)
    }

    pub fn read_binary_table(&mut self) -> Result<BinaryTable, Report<VfbError>> {
        let mut tag_bytes = [0u8; 4];
        self.inner
            .read_exact(&mut tag_bytes)
            .map_err(VfbError::ReadError)?;
        let tag = Tag::from_be_bytes(tag_bytes); // or LE?
        let mut remainder = vec![];
        self.inner
            .read_to_end(&mut remainder)
            .map_err(VfbError::ReadError)?;
        Ok(BinaryTable((tag, RawData(remainder))))
    }

    pub fn read_axis_mappings_count(&mut self) -> Result<[u32; 4], Report<VfbError>> {
        let mut counts = [0u32; 4];
        for entry in counts.iter_mut() {
            *entry = self.read_u32()?;
        }
        Ok(counts)
    }

    pub fn read_number_of_masters(&mut self) -> Result<u16, Report<VfbError>> {
        let count = self.read_u16()?;
        self.number_of_masters = count as usize;
        Ok(count)
    }

    pub fn read_panose(&mut self) -> Result<[i8; 10], Report<VfbError>> {
        let mut panose = [0i8; 10];
        for entry in panose.iter_mut() {
            *entry = self.read_i8()?;
        }
        Ok(panose)
    }

    pub fn read_unicode_ranges(&mut self) -> Result<[u32; 4], Report<VfbError>> {
        let mut ranges = [0u32; 4];
        for entry in ranges.iter_mut() {
            *entry = self.read_u32()?;
        }
        Ok(ranges)
    }
}

#[derive(VfbEntry, Serialize, Debug)]
pub enum VfbEntry {
    #[vfb(key = 1501, reader = "read_encoding")]
    #[serde(rename = "Encoding Default")]
    EncodingDefault(Encoding),

    #[vfb(key = 1500, reader = "read_encoding")]
    #[serde(rename = "Encoding")]
    Encoding(Encoding),

    #[vfb(key = 1502, reader = "read_u16")]
    #[serde(rename = "1502")]
    Unknown1502(u16),

    #[vfb(key = 518)]
    #[serde(rename = "518")]
    Unknown518,

    #[vfb(key = 257, reader = "read_string")]
    #[serde(rename = "257")]
    Unknown257(String),

    #[vfb(key = 1026, reader = "read_string")]
    #[serde(rename = "font_name")]
    FontName(String),

    #[vfb(key = 1503, reader = "read_number_of_masters")] // also sets
    #[serde(rename = "Master Count")]
    MasterCount(u16),

    #[vfb(key = 1046, reader = "read_string")]
    #[serde(rename = "version")]
    Version(String),

    #[vfb(key = 1038, reader = "read_string")]
    #[serde(rename = "notice")]
    Notice(String),

    #[vfb(key = 1025, reader = "read_string")]
    #[serde(rename = "full_name")]
    FullName(String),

    #[vfb(key = 1027, reader = "read_string")]
    #[serde(rename = "family_name")]
    FamilyName(String),

    #[vfb(key = 1024, reader = "read_string")]
    #[serde(rename = "pref_family_name")]
    PrefFamilyName(String),

    #[vfb(key = 1056, reader = "read_string")]
    #[serde(rename = "menu_name")]
    MenuName(String),

    #[vfb(key = 1092, reader = "read_string")]
    #[serde(rename = "apple_name")]
    AppleName(String),

    #[vfb(key = 1028, reader = "read_string")]
    #[serde(rename = "weight")]
    Weight(String),

    #[vfb(key = 1065, reader = "read_string")]
    #[serde(rename = "width")]
    Width(String),

    #[vfb(key = 1069, reader = "read_string")]
    #[serde(rename = "License")]
    License(String),

    #[vfb(key = 1070, reader = "read_string")]
    #[serde(rename = "License URL")]
    LicenseUrl(String),

    #[vfb(key = 1037, reader = "read_string")]
    #[serde(rename = "copyright")]
    Copyright(String),

    #[vfb(key = 1061, reader = "read_string")]
    #[serde(rename = "trademark")]
    Trademark(String),

    #[vfb(key = 1062, reader = "read_string")]
    #[serde(rename = "designer")]
    Designer(String),

    #[vfb(key = 1063, reader = "read_string")]
    #[serde(rename = "designer_url")]
    DesignerUrl(String),

    #[vfb(key = 1064, reader = "read_string")]
    #[serde(rename = "vendor_url")]
    VendorUrl(String),

    #[vfb(key = 1039, reader = "read_string")]
    #[serde(rename = "source")]
    Source(String),

    #[vfb(key = 1034, reader = "read_u16")]
    #[serde(rename = "is_fixed_pitch")]
    IsFixedPitch(u16),

    #[vfb(key = 1031, reader = "read_u16")]
    #[serde(rename = "underline_thickness")]
    UnderlineThickness(u16),

    #[vfb(key = 1054, reader = "read_i16")]
    #[serde(rename = "ms_charset")]
    MsCharset(i16),

    #[vfb(key = 1118, reader = "read_panose")]
    #[serde(rename = "panose")]
    Panose([i8; 10]),

    #[vfb(key = 1128, reader = "read_string")]
    #[serde(rename = "tt_version")]
    TtVersion(String),

    #[vfb(key = 1129, reader = "read_string")]
    #[serde(rename = "tt_u_id")]
    TtUId(String),

    #[vfb(key = 1127, reader = "read_string")]
    #[serde(rename = "style_name")]
    StyleName(String),

    #[vfb(key = 1137, reader = "read_string")]
    #[serde(rename = "pref_style_name")]
    PrefStyleName(String),

    #[vfb(key = 1139, reader = "read_string")]
    #[serde(rename = "mac_compatible")]
    MacCompatible(String),

    #[vfb(key = 1121, reader = "read_string")]
    #[serde(rename = "vendor")]
    Vendor(String),

    #[vfb(key = 1132, reader = "read_u16")]
    #[serde(rename = "year")]
    Year(u16),

    #[vfb(key = 1130, reader = "read_u16")]
    #[serde(rename = "version_major")]
    VersionMajor(u16),

    #[vfb(key = 1131, reader = "read_u16")]
    #[serde(rename = "version_minor")]
    VersionMinor(u16),

    #[vfb(key = 1135, reader = "read_u16")]
    #[serde(rename = "upm")]
    Upm(u16),

    #[vfb(key = 1090, reader = "read_u16")]
    #[serde(rename = "fond_id")]
    FondId(u16),

    #[vfb(key = 1530, reader = "read_u16")]
    #[serde(rename = "blue_values_num")]
    BlueValuesNum(u16),

    #[vfb(key = 1531, reader = "read_u16")]
    #[serde(rename = "other_blues_num")]
    OtherBluesNum(u16),

    #[vfb(key = 1532, reader = "read_u16")]
    #[serde(rename = "family_blues_num")]
    FamilyBluesNum(u16),

    #[vfb(key = 1533, reader = "read_u16")]
    #[serde(rename = "family_other_blues_num")]
    FamilyOtherBluesNum(u16),

    #[vfb(key = 1534, reader = "read_u16")]
    #[serde(rename = "stem_snap_h_num")]
    StemSnapHNum(u16),

    #[vfb(key = 1535, reader = "read_u16")]
    #[serde(rename = "stem_snap_v_num")]
    StemSnapVNum(u16),

    #[vfb(key = 1267, reader = "read_u16")]
    #[serde(rename = "font_style")]
    FontStyle(u16),

    #[vfb(key = 1057, reader = "read_u16")]
    #[serde(rename = "pcl_id")]
    PclId(u16),

    #[vfb(key = 1058, reader = "read_u16")]
    #[serde(rename = "vp_id")]
    VpId(u16),

    #[vfb(key = 1060, reader = "read_u16")]
    #[serde(rename = "ms_id")]
    MsId(u16),

    #[vfb(key = 1059, reader = "read_string")]
    #[serde(rename = "pcl_chars_set")]
    PclCharsSet(String),

    #[vfb(key = 1270, reader = "read_u16")]
    #[serde(rename = "hhea_line_gap")]
    HheaLineGap(u16),

    #[vfb(key = 1272, reader = "read_u16")]
    #[serde(rename = "stemsnaplimit")]
    StemSnapLimit(u16),

    #[vfb(key = 1274, reader = "read_u16")]
    #[serde(rename = "zoneppm")]
    ZonePpm(u16),

    #[vfb(key = 1275, reader = "read_u16")]
    #[serde(rename = "codeppm")]
    CodePpm(u16),

    #[vfb(key = 1604, reader = "read_u16")]
    #[serde(rename = "1604")]
    Unknown1604(u16),

    #[vfb(key = 2032, reader = "read_u16")]
    #[serde(rename = "2032")]
    Unknown2032(u16),

    #[vfb(key = 2022, reader = "read_u16")]
    #[serde(rename = "Export PCLT Table")]
    ExportPcltTable(u16),

    #[vfb(key = 2025, reader = "read_string")]
    #[serde(rename = "note")]
    Note(String),

    #[vfb(key = 2016, reader = "read_string")]
    #[serde(rename = "customdata")]
    CustomData(String),

    #[vfb(key = 1277, reader = "read_string")]
    #[serde(rename = "OpenType Class")]
    OpenTypeClass(String),

    #[vfb(key = 1513, reader = "read_u16")]
    #[serde(rename = "Axis Count")]
    AxisCount(u16),

    #[vfb(key = 1514, reader = "read_string")]
    #[serde(rename = "Axis Name")]
    AxisName(String),

    #[vfb(key = 1504, reader = "read_string")]
    #[serde(rename = "Master Name")]
    MasterName(String),

    #[vfb(key = 1066, reader = "read_string")]
    #[serde(rename = "default_character")]
    DefaultCharacter(String),

    #[vfb(key = 2034, reader = "read_string")]
    #[serde(rename = "2034")]
    Unknown2034(String),

    #[vfb(key = 2012, reader = "read_u16")]
    #[serde(rename = "mark")]
    Mark(u16),

    #[vfb(key = 2015, reader = "read_string")]
    #[serde(rename = "glyph.customdata")]
    GlyphCustomData(String),

    #[vfb(key = 2017, reader = "read_string")]
    #[serde(rename = "glyph.note")]
    GlyphNote(String),

    #[vfb(key = 1517, reader = "read_double_list")]
    #[serde(rename = "Default Weight vector")]
    WeightVector(Vec<f64>),

    #[vfb(key = 1044, reader = "read_i32")]
    #[serde(rename = "unique_id")]
    UniqueId(i32),

    #[vfb(key = 1048, reader = "read_i16")]
    #[serde(rename = "weight_code")]
    WeightCode(i16),

    #[vfb(key = 1029, reader = "read_f64")]
    #[serde(rename = "italic_angle")]
    ItalicAngle(f64),

    #[vfb(key = 1047, reader = "read_f64")]
    #[serde(rename = "slant_angle")]
    SlantAngle(f64),

    #[vfb(key = 1030, reader = "read_i16")]
    #[serde(rename = "underline_position")]
    UnderlinePosition(i16),

    #[vfb(key = 1140)]
    #[serde(rename = "1140")]
    E1140(RawData),

    #[vfb(key = 1133, reader = "read_int_list")]
    #[serde(rename = "xuid")]
    Xuid(Vec<u16>),

    #[vfb(key = 1134, reader = "read_i16")]
    #[serde(rename = "xuid_num")]
    XuidNum(i16),

    #[vfb(key = 1093, reader = "read_u16")]
    #[serde(rename = "PostScript Hinting Options")]
    PostScriptHintingOptions(PostScriptGlobalHintingOptions),

    #[vfb(key = 1068, reader = "read_encoded_value_list")]
    #[serde(rename = "1068")]
    E1068(Vec<i32>),

    #[vfb(key = 1264)]
    #[serde(rename = "ttinfo")]
    TtInfo(TrueTypeInfo),

    #[vfb(key = 2021, reader = "read_unicode_ranges")]
    #[serde(rename = "unicoderanges")]
    UnicodeRanges(Vec<u32>), // Maybe use a bitflags array?

    #[vfb(key = 1138, reader = "read_namerecords")]
    #[serde(rename = "fontnames")]
    FontNames(Vec<NameRecord>),

    #[vfb(key = 1141)]
    #[serde(rename = "Custom CMAPs")]
    CustomCmaps(CustomCmap),

    #[vfb(key = 1136)]
    #[serde(rename = "PCLT Table")]
    PcltTable(Pclt),

    #[vfb(key = 2030)]
    #[serde(rename = "2030")]
    E2030(RawData),

    #[vfb(key = 2024)]
    #[serde(rename = "OpenType Metrics Class Flags")]
    MetricsClassFlags(OpenTypeMetricsClassFlags),

    #[vfb(key = 2026)]
    #[serde(rename = "OpenType Kerning Class Flags")]
    KerningClassFlags(OpenTypeKerningClassFlags),

    #[vfb(key = 2014, reader = "read_binary_table")]
    #[serde(rename = "TrueTypeTable")]
    TrueTypeTable(BinaryTable),

    #[vfb(key = 1276, reader = "read_string")]
    #[serde(rename = "features")]
    Features(String),

    #[vfb(key = 513)]
    #[serde(rename = "513")]
    E513(RawData),

    #[vfb(key = 271)]
    #[serde(rename = "271")]
    E271(RawData),

    #[vfb(key = 1523)]
    #[serde(rename = "Anisotropic Interpolation Mappings")]
    AnisotropicInterpolationMappings(AnisotropicInterpolations),

    #[vfb(key = 1515, reader = "read_axis_mappings_count")]
    #[serde(rename = "Axis Mappings Count")]
    AxisMappingsCount([u32; 4]),

    #[vfb(key = 1516)]
    #[serde(rename = "Axis Mappings")]
    AxisMappings(AxisMappings),

    #[vfb(key = 1247, reader = "read_double_list")]
    #[serde(rename = "Primary Instance Locations")]
    PrimaryInstanceLocations(Vec<f64>),

    #[vfb(key = 1254)]
    #[serde(rename = "Primary Instances")]
    PrimaryInstances(PrimaryInstances),

    #[vfb(key = 527)]
    #[serde(rename = "527")]
    E527(RawData),

    #[vfb(key = 1294, reader = "read_guides")]
    #[serde(rename = "Global Guides")]
    GlobalGuides(Guides),

    #[vfb(key = 1296)]
    #[serde(rename = "Global Guide Properties")]
    GlobalGuideProperties(GuideProperties),

    #[vfb(key = 1295)]
    #[serde(rename = "Global Mask")]
    GlobalMask(GlobalMask),

    #[vfb(key = 1743)]
    #[serde(rename = "OpenType Export Options")]
    OpenTypeExportOptions(RawData),

    #[vfb(key = 1744, reader = "read_u16")]
    #[serde(rename = "Export Options")]
    ExportOptions(ExportOptions),

    #[vfb(key = 1742)]
    #[serde(rename = "Mapping Mode")]
    MappingMode(MappingMode),

    #[vfb(key = 272)]
    #[serde(rename = "272")]
    E272(RawData),

    #[vfb(key = 1410)]
    #[serde(rename = "1410")]
    E1410(FL3Type1410),

    #[vfb(key = 528)]
    #[serde(rename = "528")]
    E528(RawData),

    #[vfb(key = 1505)]
    #[serde(rename = "Master Location")]
    MasterLocation(MasterLocation),

    #[vfb(key = 1536)]
    #[serde(rename = "PostScript Info")]
    PostScriptInfo(PostScriptInfo),

    #[vfb(key = 1261)]
    #[serde(rename = "cvt")]
    Cvt(RawData),

    #[vfb(key = 1262)]
    #[serde(rename = "prep")]
    Prep(RawData),

    #[vfb(key = 1263)]
    #[serde(rename = "fpgm")]
    Fpgm(RawData),

    #[vfb(key = 1265)]
    #[serde(rename = "gasp")]
    Gasp(RawData),

    #[vfb(key = 1271)]
    #[serde(rename = "vdmx")]
    Vdmx(Vdmx),

    #[vfb(key = 1278, reader = "read_i16")]
    #[serde(rename = "hhea_ascender")]
    HheaAscender(i16),

    #[vfb(key = 1279, reader = "read_i16")]
    #[serde(rename = "hhea_descender")]
    HheaDescender(i16),

    #[vfb(key = 1266)]
    #[serde(rename = "TrueType Stem PPEMs 2 And 3")]
    TrueTypeStemPpems2And3(TrueTypeStemPpems23),

    #[vfb(key = 1268)]
    #[serde(rename = "TrueType Stem PPEMs")]
    TrueTypeStemPpems(TrueTypeStemPpems),

    #[vfb(key = 1269)]
    #[serde(rename = "TrueType Stems")]
    TrueTypeStems(TrueTypeStems),

    #[vfb(key = 1524)]
    #[serde(rename = "TrueType Stem PPEMs 1")]
    TrueTypeStemPpems1(TrueTypeStemPpems1),

    #[vfb(key = 1255)]
    #[serde(rename = "TrueType Zones")]
    TrueTypeZones(TrueTypeZones),

    #[vfb(key = 1273)]
    #[serde(rename = "TrueType Zone Deltas")]
    TrueTypeZoneDeltas(TrueTypeZoneDeltas),

    #[vfb(key = 2001, reader = "read_glyph")]
    #[serde(rename = "Glyph")]
    Glyph(Glyph),

    #[vfb(key = 2008)]
    #[serde(rename = "Links")]
    Links(Link),

    #[vfb(key = 2007)]
    #[serde(rename = "image")]
    Image(BackgroundBitmap),

    #[vfb(key = 2013)]
    #[serde(rename = "Glyph Bitmaps")]
    Bitmaps(GlyphBitmaps),

    #[vfb(key = 2023)]
    #[serde(rename = "2023")]
    E2023(EncodedValueList),

    #[vfb(key = 2019)]
    #[serde(rename = "Glyph Sketch")]
    Sketch(GlyphSketch),

    #[vfb(key = 2010, reader = "read_u32")]
    #[serde(rename = "Glyph Hinting Options")]
    HintingOptions(PostScriptGlyphHintingOptions),

    #[vfb(key = 2009)]
    #[serde(rename = "mask")]
    Mask(Mask),

    #[vfb(key = 2011)]
    #[serde(rename = "mask.metrics")]
    MaskMetrics(MaskMetrics),

    #[vfb(key = 2028)]
    #[serde(rename = "mask.metrics_mm")]
    MaskMetricsMm(MaskMetricsMM),

    #[vfb(key = 2027)]
    #[serde(rename = "Glyph Origin")]
    Origin(GlyphOrigin),

    #[vfb(key = 1250)]
    #[serde(rename = "unicodes")]
    Unicodes(GlyphUnicode),

    #[vfb(key = 1253)]
    #[serde(rename = "Glyph Unicode Non-BMP")]
    UnicodesNonBmp(GlyphUnicodesSupp),

    #[vfb(key = 2018)]
    #[serde(rename = "Glyph GDEF Data")]
    GdefData(GlyphGDEF),

    #[vfb(key = 2020)]
    #[serde(rename = "Glyph Anchors Supplemental")]
    AnchorsProperties(GlyphAnchorsSupp),

    #[vfb(key = 2029)]
    #[serde(rename = "Glyph Anchors MM")]
    AnchorsMm(GlyphAnchors),

    #[vfb(key = 2031)]
    #[serde(rename = "Glyph Guide Properties")]
    GuideProperties(GuideProperties),
}
