use phf::phf_map;

pub static VFB_KEYS: phf::Map<&'static str, &'static str> = phf_map! {
    // Sorted by appearance in the VFB
    "1501" => "Encoding Default",
    "1500" => "Encoding",
    "1502" => "1502",
    "518" => "518",  // maybe marker? has 0 bytes data
    "257" => "257",
    "1026" => "font_name",  // psn
    "1503" => "Master Count",
    "1517" => "weight_vector",  // Default Weight Vector, one value per master
    "1044" => "unique_id",  // Type 1 Unique ID
    "1046" => "version",  // version full
    "1038" => "notice",  // description
    "1025" => "full_name",  // ffn
    "1027" => "family_name",  // tfn
    "1024" => "pref_family_name",  // sgn
    "1056" => "menu_name",  // Menu Name
    "1092" => "apple_name",  // FOND Name
    "1028" => "weight",  // weight_name
    "1065" => "width",  // width_name

    // Is license/url not in Python API?
    "1069" => "License",
    "1070" => "License URL",

    "1037" => "copyright",  // OK!
    "1061" => "trademark",  // OK!
    "1062" => "designer",  // OK!
    "1063" => "designer_url",  // designerURL
    "1064" => "vendor_url",  // manufacturerURL
    "1039" => "source",  // manufacturer, "created by"
    "1034" => "is_fixed_pitch",  // Monospaced
    "1048" => "weight_code",  // Weight Class
    "1029" => "italic_angle",  // Italic Angle
    "1047" => "slant_angle",  // Slant Angle
    "1030" => "underline_position",  // underlinePosition
    "1031" => "underline_thickness",  // underlineThickness
    "1054" => "ms_charset",  // MS Character Set
    "1118" => "panose",  // OK!
    "1128" => "tt_version",  // version
    "1129" => "tt_u_id",  // UniqueID
    "1127" => "style_name",  // Style Name
    "1137" => "pref_style_name",  // tsn
    "1139" => "mac_compatible",  // OT Mac Name
    "1140" => "1140",
    "1121" => "vendor",  // vendorID
    "1133" => "xuid",  // Type 1 XUIDs
    "1134" => "xuid_num",  // Type 1 XUIDs Count
    "1132" => "year",  // OK!
    "1130" => "version_major",  // versionMajor
    "1131" => "version_minor",  // versionMinor
    "1135" => "upm",  // OK!
    "1090" => "fond_id",  // FOND Family ID
    "1093" => "PostScript Hinting Options",
    "1068" => "1068",
    "1530" => "blue_values_num",  // Blue Values Count
    "1531" => "other_blues_num",  // Other Blues Count
    "1532" => "family_blues_num",  // Family Blues Count
    "1533" => "family_other_blues_num",  // Family Other Blues Count  // noqa: E501
    "1534" => "stem_snap_h_num",  // StemSnapH Count
    "1535" => "stem_snap_v_num",  // StemSnapV Count
    "1267" => "font_style",  // Selection
    "1057" => "pcl_id",  // PCL ID
    "1058" => "vp_id",  // VP ID
    "1060" => "ms_id",  // MS ID
    "1059" => "pcl_chars_set",
    "1261" => "Binary cvt Table",
    "1262" => "Binary prep Table",
    "1263" => "Binary fpgm Table",

    // Goes to font.ttinfo:
    "1265" => "gasp",  // Gasp Ranges
    "1264" => "ttinfo",  // TrueType Info
    // Goes to font.ttinfo:
    "1271" => "vdmx",
    // Goes to font.ttinfo:
    "1270" => "hhea_line_gap",
    "1278" => "hhea_ascender",
    "1279" => "hhea_descender",
    // hstem_data and vstem_data:
    "1266" => "TrueType Stem PPEMs 2 And 3",
    "1268" => "TrueType Stem PPEMs",
    "1524" => "TrueType Stem PPEMs 1",
    // Probably in font.ttinfo, but not accessible through API:
    "1269" => "TrueType Stems",
    "1255" => "TrueType Zones",
    // Goes to font:
    "2021" => "unicoderanges",
    // Probably in font.ttinfo, but not accessible through API:
    "1272" => "stemsnaplimit",
    "1274" => "zoneppm",
    "1275" => "codeppm",
    "1604" => "1604",  // Binary import? e.g. 255
    "2032" => "2032",  // Binary import? e.g. 300
    "1273" => "TrueType Zone Deltas",
    // Goes to font again:
    "1138" => "fontnames",
    "1141" => "Custom CMAPs",
    "1136" => "PCLT Table",
    "2022" => "Export PCLT Table",
    "2025" => "note",
    "2030" => "2030",
    "2016" => "customdata",
    "2024" => "OpenType Metrics Class Flags",
    "2026" => "OpenType Kerning Class Flags",

    // Repeat for each binary table:
    // truetypetables: TrueTypeTable
    "2014" => "TrueTypeTable",  // Binary Table

    "1276" => "features",  // openTypeFeatures

    // Repeat for each OpenType class:
    // font.classes
    "1277" => "OpenType Class",  // OpenType Class

    "513" => "513",
    "271" => "271",
    "1513" => "Axis Count",
    "1514" => "Axis Name",
    "1523" => "Anisotropic Interpolation Mappings",
    "1515" => "Axis Mappings Count",
    "1516" => "Axis Mappings",

    // Repeat the next two for each master:
    "1504" => "Master Name",
    "1505" => "Master Location",

    "1247" => "Primary Instance Locations",
    "1254" => "Primary Instances",

    // Repeat PostScript Info for each master:
    "1536" => "PostScript Info",

    "527" => "527",
    "1294" => "Global Guides",
    "1296" => "Global Guide Properties",
    "1295" => "Global Mask",
    "1066" => "default_character",  // Default Glyph

    // Begin: Repeat for each glyph
    "2001" => "Glyph",
    // Glyph.hlinks and Glyph.vlinks:
    "2008" => "Links",
    "2007" => "image",  // Background Bitmap
    "2013" => "Glyph Bitmaps",
    "2023" => "2023",  // 1 encoded value per master
    "2019" => "Glyph Sketch",
    "2010" => "Glyph Hinting Options",
    "2009" => "mask",  // Mask
    "2011" => "mask.metrics",  // Single master mask metrics
    "2028" => "mask.metrics_mm",  // Mask metrics master 2 to 16
    "2027" => "Glyph Origin",
    "1250" => "unicodes",  // Glyph Unicode
    "2034" => "2034",  // some kind of string
    "1253" => "Glyph Unicode Non-BMP",
    "2012" => "mark",  // Mark Color
    "2015" => "glyph.customdata",  // Glyph User Data
    "2017" => "glyph.note",  // Glyph Note
    "2018" => "Glyph GDEF Data",
    "2020" => "Glyph Anchors Supplemental",
    "2029" => "Glyph Anchors MM",  // MM-compatible
    "2031" => "Glyph Guide Properties",
    // End: Repeat for each glyph

    "1743" => "OpenType Export Options",
    "1744" => "Export Options",
    "1742" => "Mapping Mode",

    // Not seen in FontNames.vfb:
    "1410" => "1410",

    // File end marker:
    "5" => "EOF",
};
