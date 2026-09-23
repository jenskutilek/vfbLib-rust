use error_stack::Report;

use crate::{
    buffer::{EntryReader, ReadExt},
    encodings::{
        decode_big5, decode_gbk, decode_macintosh, decode_macintosh_cyrillic, decode_utf16,
        decode_windows1255,
    },
    VfbError,
};

#[derive(Debug, serde::Serialize)]
pub struct NameRecord {
    pub name_id: u16,
    pub platform_id: u16,
    pub encoding_id: u16,
    pub language_id: u16,
    pub string: String,
}

impl NameRecord {
    pub fn new(
        name_id: u16,
        platform_id: u16,
        encoding_id: u16,
        language_id: u16,
        codes: &[i32],
    ) -> Self {
        let string = match platform_id {
            0 => {
                // Unicode
                match encoding_id {
                    3 => decode_utf16(codes),
                    4 => decode_utf16(codes),
                    _ => decode_utf16(codes), // Try with UTF-16 anyway
                }
            }
            1 => {
                // Macintosh
                match encoding_id {
                    0 => decode_macintosh(codes),
                    // TODO: Support Mac Greek; now we decode with Mac Roman and accept
                    // garbled output.
                    // 6 => decode_macintosh_greek(codes), // Mac Greek, not in encoding_rs
                    6 => decode_macintosh(codes), // Mac Greek, not in encoding_rs
                    7 => decode_macintosh_cyrillic(codes), // Mac Russian (i.e. Cyrillic)
                    _ => decode_utf16(codes),     // Try with UTF-16 anyway
                }
            }
            2 => {
                // Windows
                match encoding_id {
                    3 => decode_gbk(codes),         // PRC, CP 936
                    4 => decode_big5(codes),        // Big5, CP 950
                    5 => decode_windows1255(codes), // Wansung, CP 949
                    _ => decode_utf16(codes),       // IDs > 10 are undefined, but we don't care
                }
            }
            _ => decode_utf16(codes), // Unknown
        };
        Self {
            name_id,
            platform_id,
            encoding_id,
            language_id,
            string,
        }
    }
}

impl<R: std::io::Read + std::io::Seek> EntryReader<'_, R> {
    pub fn read_namerecords(&mut self) -> Result<Vec<NameRecord>, Report<VfbError>> {
        let count = self.read_value()? as usize;
        let mut records = Vec::with_capacity(count);
        for _ in 0..count {
            let platform_id = self.read_value()? as u16;
            let encoding_id = self.read_value()? as u16;
            let language_id = self.read_value()? as u16;
            let name_id = self.read_value()? as u16;
            let name_length = self.read_value()? as u32;
            let mut codes = vec![0i32; name_length as usize];
            for code in &mut codes {
                *code = self.read_value()?;
            }
            records.push(NameRecord::new(
                platform_id,
                encoding_id,
                language_id,
                name_id,
                &codes,
            ));
        }
        Ok(records)
    }
}
