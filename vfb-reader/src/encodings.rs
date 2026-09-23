use encoding_rs::{BIG5, GBK, MACINTOSH, WINDOWS_1255, X_MAC_CYRILLIC};

pub(crate) fn decode_big5(codes: &[i32]) -> String {
    return BIG5
        .decode_without_bom_handling_and_without_replacement(
            &codes.iter().map(|&c| c as u8).collect::<Vec<u8>>(),
        )
        .map(|s| s.to_string())
        .unwrap_or_default();
}

pub(crate) fn decode_gbk(codes: &[i32]) -> String {
    return GBK
        .decode_without_bom_handling_and_without_replacement(
            &codes.iter().map(|&c| c as u8).collect::<Vec<u8>>(),
        )
        .map(|s| s.to_string())
        .unwrap_or_default();
}

pub(crate) fn decode_macintosh(codes: &[i32]) -> String {
    return MACINTOSH
        .decode_without_bom_handling_and_without_replacement(
            &codes.iter().map(|&c| c as u8).collect::<Vec<u8>>(),
        )
        .map(|s| s.to_string())
        .unwrap_or_default();
}

pub(crate) fn decode_macintosh_cyrillic(codes: &[i32]) -> String {
    return X_MAC_CYRILLIC
        .decode_without_bom_handling_and_without_replacement(
            &codes.iter().map(|&c| c as u8).collect::<Vec<u8>>(),
        )
        .map(|s| s.to_string())
        .unwrap_or_default();
}

pub(crate) fn decode_utf16(codes: &[i32]) -> String {
    return String::from_utf16(
        codes
            .iter()
            .map(|&c| c as u16)
            .collect::<Vec<u16>>()
            .as_slice(),
    )
    .unwrap_or_default();
}

pub(crate) fn decode_windows1255(codes: &[i32]) -> String {
    return WINDOWS_1255
        .decode_without_bom_handling_and_without_replacement(
            &codes.iter().map(|&c| c as u8).collect::<Vec<u8>>(),
        )
        .map(|s| s.to_string())
        .unwrap_or_default();
}

// const MACGREEK: [u16; 128] = [
//     0x00C4, 0x00B9, 0x00B2, 0x00C9, 0x00B3, 0x00D6, 0x00DC, 0x0385, 0x00E0, 0x00E2, 0x00E4, 0x0384,
//     0x00A8, 0x00E7, 0x00E9, 0x00E8, 0x00EA, 0x00EB, 0x00A3, 0x2122, 0x00EE, 0x00EF, 0x2022, 0x00BD,
//     0x2030, 0x00F4, 0x00F6, 0x00A6, 0x20AC, 0x00F9, 0x00FB, 0x00FC, 0x2020, 0x0393, 0x0394, 0x0398,
//     0x039B, 0x039E, 0x03A0, 0x00DF, 0x00AE, 0x00A9, 0x03A3, 0x03AA, 0x00A7, 0x2260, 0x00B0, 0x00B7,
//     0x0391, 0x00B1, 0x2264, 0x2265, 0x00A5, 0x0392, 0x0395, 0x0396, 0x0397, 0x0399, 0x039A, 0x039C,
//     0x03A6, 0x03AB, 0x03A8, 0x03A9, 0x03AC, 0x039D, 0x00AC, 0x039F, 0x03A1, 0x2248, 0x03A4, 0x00AB,
//     0x00BB, 0x2026, 0x00A0, 0x03A5, 0x03A7, 0x0386, 0x0388, 0x0153, 0x2013, 0x2015, 0x201C, 0x201D,
//     0x2018, 0x2019, 0x00F7, 0x0389, 0x038A, 0x038C, 0x038E, 0x03AD, 0x03AE, 0x03AF, 0x03CC, 0x038F,
//     0x03CD, 0x03B1, 0x03B2, 0x03C8, 0x03B4, 0x03B5, 0x03C6, 0x03B3, 0x03B7, 0x03B9, 0x03BE, 0x03BA,
//     0x03BB, 0x03BC, 0x03BD, 0x03BF, 0x03C0, 0x03CE, 0x03C1, 0x03C3, 0x03C4, 0x03B8, 0x03C9, 0x03C2,
//     0x03C7, 0x03C5, 0x03B6, 0x03CA, 0x03CB, 0x0390, 0x03B0, 0x00AD,
// ];

// pub(crate) fn decode_macintosh_greek(codes: &[i32]) -> String {}
