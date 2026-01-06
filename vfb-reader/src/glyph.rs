use std::collections::HashMap;

use crate::buffer::ReadExt;
use error_stack::{Report, ResultExt};
use serde::Serialize;
use vfb_macros::VfbEntry;

use crate::{buffer::EntryReader, entries::RawData, guides::Guides, VfbError};

#[derive(VfbEntry, Serialize)]
pub enum GlyphEntry {
    #[vfb(key = 1, reader = "read_str_with_len")]
    #[serde(rename = "Glyph Name")]
    GlyphName(String),
    #[vfb(key = 2, reader = "read_glyph_metrics")]
    Metrics(Vec<(i16, i16)>),
    #[vfb(key = 3)]
    Hints(RawData),
    #[vfb(key = 4, reader = "read_guides")]
    Guides(Guides),
    #[vfb(key = 5, reader = "read_components")]
    Components(Vec<Component>),
    #[vfb(key = 6, reader = "read_kerning")]
    Kerning(HashMap<i32, Vec<i32>>),
    #[vfb(key = 8)]
    Outlines(RawData),
    #[vfb(key = 9)]
    Binary(RawData),
    #[vfb(key = 10)]
    Instructions(RawData),
}

#[derive(Serialize)]
pub struct Component {
    pub glyph_index: u16,
    pub x_offset: Vec<i16>, // one per master
    pub y_offset: Vec<i16>,
    pub x_scale: Vec<f64>,
    pub y_scale: Vec<f64>,
}

impl<R: std::io::Read + std::io::Seek> EntryReader<'_, R> {
    pub fn read_glyph_metrics(&mut self) -> Result<Vec<(i16, i16)>, Report<VfbError>> {
        let count = self.number_of_masters;
        let mut metrics = Vec::with_capacity(count);
        for _ in 0..count {
            let left_side_bearing = self.read_i16()?;
            let advance_width = self.read_i16()?;
            metrics.push((left_side_bearing, advance_width));
        }
        Ok(metrics)
    }
    pub fn read_components(&mut self) -> Result<Vec<Component>, Report<VfbError>> {
        let mut components = Vec::new();
        let component_count = self.read_value()?;
        for _ in 0..component_count {
            let glyph_index = self.read_u16()?;
            components.push(Component {
                glyph_index,
                x_offset: vec![],
                y_offset: vec![],
                x_scale: vec![],
                y_scale: vec![],
            });
            for _ in 0..self.number_of_masters {
                let x_offset = self.read_i16()?;
                let y_offset = self.read_i16()?;
                let x_scale = self.read_f64()?;
                let y_scale = self.read_f64()?;
                let last_components = components.last_mut().unwrap();
                last_components.x_offset.push(x_offset);
                last_components.y_offset.push(y_offset);
                last_components.x_scale.push(x_scale);
                last_components.y_scale.push(y_scale);
            }
        }
        Ok(components)
    }

    pub fn read_kerning(&mut self) -> Result<HashMap<i32, Vec<i32>>, Report<VfbError>> {
        let mut kerning_map = HashMap::new();
        let pair_count = self.read_value()?;
        for _ in 0..pair_count {
            let right_glyph = self.read_i32()?;
            let mut values = Vec::new();
            for _ in 0..self.number_of_masters {
                let value = self.read_i32()?;
                values.push(value);
            }
            kerning_map.insert(right_glyph, values);
        }
        Ok(kerning_map)
    }

    pub fn read_glyph(&mut self) -> Result<Vec<GlyphEntry>, Report<VfbError>> {
        let glyph_header = self.read_bytes(4)?;
        // When was Yuri born?
        if glyph_header != [1, 9, 7, 1] {
            return Err(VfbError::InvalidGlyphHeader(glyph_header).into());
        }
        let mut entries = Vec::new();
        loop {
            let raw_key = self.read_u8()?;
            if raw_key == 0xf {
                break;
            }
            let entry =
                GlyphEntry::new_from_reader(u16::from(raw_key), self).attach_printable(format!(
                    "while reading {}",
                    GlyphEntry::key_to_variant(raw_key.into()).unwrap_or("an unknown key")
                ))?;
            if let Some(e) = entry {
                entries.push(e);
            }
        }
        Ok(entries)
    }
}
