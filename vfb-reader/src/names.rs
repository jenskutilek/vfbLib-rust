use encoding_rs::*;
use error_stack::Report;

use crate::{
    buffer::{EntryReader, ReadExt},
    VfbError,
};

#[derive(Debug, serde::Serialize)]
pub struct NameRecord {
    pub platform_id: u16,
    pub encoding_id: u16,
    pub language_id: u16,
    pub name_id: u16,
    pub string: String,
}

impl NameRecord {
    pub fn new(
        platform_id: u16,
        encoding_id: u16,
        language_id: u16,
        name_id: u16,
        codes: &[i32],
    ) -> Self {
        // Decode from Mac Roman if platform_id is 1, otherwise UTF-16BE
        let string = if platform_id == 1 {
            MACINTOSH
                .decode_without_bom_handling_and_without_replacement(
                    &codes.iter().map(|&c| c as u8).collect::<Vec<u8>>(),
                )
                .map(|s| s.to_string())
                .unwrap_or_default()
        } else {
            String::from_utf16(
                codes
                    .iter()
                    .map(|&c| c as u16)
                    .collect::<Vec<u16>>()
                    .as_slice(),
            )
            .unwrap_or_default()
        };
        Self {
            platform_id,
            encoding_id,
            language_id,
            name_id,
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
