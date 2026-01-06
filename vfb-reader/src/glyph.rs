use std::collections::HashMap;

use crate::{buffer::ReadExt, error::AtByteIndex};
use error_stack::{Report, ResultExt};
use serde::Serialize;
use vfb_macros::VfbEntry;

use crate::{buffer::EntryReader, entries::RawData, guides::Guides, VfbError};

#[derive(VfbEntry, Serialize, Debug)]
enum HintMask {
    #[vfb(key = 1, reader = "read_value")]
    Horizontal(i32),
    #[vfb(key = 2, reader = "read_value")]
    Vertical(i32),
    #[vfb(key = 0xff, reader = "read_value")]
    ReplacementPoint(i32),
}

#[derive(serde::Serialize, Debug)]
pub struct Hint {
    position: i32,
    width: i32,
}

#[derive(serde::Serialize, Debug)]
pub struct Hints {
    horizontal: Vec<Vec<Hint>>,
    vertical: Vec<Vec<Hint>>,
    masks: Vec<HintMask>,
}

pub enum PathCommand {
    Move = 0,
    Line = 1,
    // The forbidden command goes here
    Curve = 3,
    QCurve = 4,
}

#[derive(Serialize, Debug)]
pub enum Node {
    Move {
        coords: Vec<(i32, i32)>,
        flags: u8,
    },
    Line {
        coords: Vec<(i32, i32)>,
        flags: u8,
    },
    Curve {
        coords: Vec<(i32, i32)>,
        c1_coords: Vec<(i32, i32)>,
        c2_coords: Vec<(i32, i32)>,
        flags: u8,
    },
    QCurve {
        coords: Vec<(i32, i32)>,
        c1_coords: Vec<(i32, i32)>,
        flags: u8,
    },
}

impl TryFrom<u8> for PathCommand {
    type Error = VfbError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PathCommand::Move),
            1 => Ok(PathCommand::Line),
            3 => Ok(PathCommand::Curve),
            4 => Ok(PathCommand::QCurve),
            _ => Err(VfbError::InvalidPathCommand(value)),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Component {
    pub glyph_index: u32,
    pub x_offset: Vec<i32>, // one per master
    pub y_offset: Vec<i32>,
    pub x_scale: Vec<f64>,
    pub y_scale: Vec<f64>,
}

#[derive(Serialize, Debug)]
pub struct Anchor {
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Debug)]
pub struct AnchorsSupplemental {
    pub hue: i32,
    pub reserved: i32,
}

#[derive(VfbEntry, Serialize, Debug)]
pub enum GlyphEntry {
    #[vfb(key = 1, reader = "read_str_with_len")]
    #[serde(rename = "Glyph Name")]
    GlyphName(String),
    #[vfb(key = 2, reader = "read_glyph_metrics")]
    Metrics(Vec<(i32, i32)>),
    #[vfb(key = 3, reader = "read_hints")]
    Hints(Hints),
    #[vfb(key = 4, reader = "read_guides")]
    Guides(Guides),
    #[vfb(key = 5, reader = "read_components")]
    Components(Vec<Component>),
    #[vfb(key = 6, reader = "read_kerning")]
    Kerning(HashMap<i32, Vec<i32>>),
    #[vfb(key = 8, reader = "read_outlines")]
    Outlines(Vec<Node>),
    #[vfb(key = 9)]
    Binary(RawData),
    #[vfb(key = 10)]
    Instructions(RawData),
}

impl<R: std::io::Read + std::io::Seek> EntryReader<'_, R> {
    fn read_glyph_metrics(&mut self) -> Result<Vec<(i32, i32)>, Report<VfbError>> {
        let count = self.number_of_masters;
        let mut metrics = Vec::with_capacity(count);
        for _ in 0..count {
            let left_side_bearing = self.read_value()?;
            let advance_width = self.read_value()?;
            metrics.push((left_side_bearing, advance_width));
        }
        Ok(metrics)
    }
    fn read_components(&mut self) -> Result<Vec<Component>, Report<VfbError>> {
        let mut components = Vec::new();
        let component_count = self.read_value()?;
        for _ in 0..component_count {
            let glyph_index = self.read_value()? as u32;
            components.push(Component {
                glyph_index,
                x_offset: vec![],
                y_offset: vec![],
                x_scale: vec![],
                y_scale: vec![],
            });
            for _ in 0..self.number_of_masters {
                let x_offset = self.read_value()?;
                let y_offset = self.read_value()?;
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

    fn read_kerning(&mut self) -> Result<HashMap<i32, Vec<i32>>, Report<VfbError>> {
        let mut kerning_map = HashMap::new();
        let pair_count = self.read_value()?;
        for _ in 0..pair_count {
            let right_glyph = self.read_value()?;
            let mut values = Vec::new();
            for _ in 0..self.number_of_masters {
                let value = self.read_value()?;
                values.push(value);
            }
            kerning_map.insert(right_glyph, values);
        }
        Ok(kerning_map)
    }

    fn read_hints(&mut self) -> Result<Hints, Report<VfbError>> {
        // Horizontal hints come first
        let num_horizontal_hints = self.read_value()? as usize;
        let mut horizontal_hints = Vec::with_capacity(num_horizontal_hints);
        for _ in 0..num_horizontal_hints {
            // One hint per master
            let mut this_hint = Vec::with_capacity(self.number_of_masters);
            for _ in 0..self.number_of_masters {
                let position = self.read_value()?;
                let width = self.read_value()?;
                this_hint.push(Hint { position, width });
            }
            horizontal_hints.push(this_hint);
        }
        // Vertical hints come next
        let num_vertical_hints = self.read_value()? as usize;
        let mut vertical_hints = Vec::with_capacity(num_vertical_hints);
        for _ in 0..num_vertical_hints {
            // One hint per master
            let mut this_hint = Vec::with_capacity(self.number_of_masters);
            for _ in 0..self.number_of_masters {
                let position = self.read_value()?;
                let width = self.read_value()?;
                this_hint.push(Hint { position, width });
            }
            vertical_hints.push(this_hint);
        }
        // Finally, read the hint masks
        let num_hint_masks = self.read_value()? as usize;
        let mut hint_masks = Vec::with_capacity(num_hint_masks);
        for _ in 0..num_hint_masks {
            let raw_key = self.read_u8()?;
            let hint_mask =
                HintMask::new_from_reader(u16::from(raw_key), self).attach_printable(format!(
                    "while reading {}",
                    HintMask::key_to_variant(raw_key.into()).unwrap_or("an unknown key")
                ))?;
            if let Some(mask) = hint_mask {
                hint_masks.push(mask);
            }
        }
        Ok(Hints {
            horizontal: horizontal_hints,
            vertical: vertical_hints,
            masks: hint_masks,
        })
    }

    fn read_outlines(&mut self) -> Result<Vec<Node>, Report<VfbError>> {
        // We have our own number of masters. I suspect they're actually what
        // other formats call "layers".
        let number_of_masters = self.read_value()? as usize;
        log::trace!("Number of masters in glyph outline: {}", number_of_masters);
        let _ = self.read_value()?; // unknown "node values" field
        let number_of_nodes = self.read_value()? as usize;
        log::trace!(
            "Number of nodes in glyph outline: {} at stream position {:04x}",
            number_of_nodes,
            self.stream_position().unwrap()
        );
        let mut nodes = vec![];
        let mut cur_pos_x = 0; // Wait, is this relative coordinate per-master?
        let mut cur_pos_y = 0;
        for _ in 0..number_of_nodes {
            let byte = self.read_u8()?;
            log::trace!("Path node byte: {:08b}", byte);
            let path_command = PathCommand::try_from(byte & 0x0f)
                .map_err(Report::from)
                .at_index(self)?;
            let flags = byte >> 4;
            // Each node has:
            // - an end point, which is an (x, y) coordinate per master
            let mut points = vec![];
            for _ in 0..number_of_masters {
                let (x_delta, y_delta) = (self.read_value()?, self.read_value()?);
                cur_pos_x += x_delta;
                cur_pos_y += y_delta;
                points.push((cur_pos_x, cur_pos_y));
            }
            match path_command {
                PathCommand::Move => {
                    nodes.push(Node::Move {
                        coords: points,
                        flags,
                    });
                }
                PathCommand::Line => {
                    nodes.push(Node::Line {
                        coords: points,
                        flags,
                    });
                }
                PathCommand::Curve => {
                    let mut c1_points = vec![];
                    let mut c2_points = vec![];
                    for _ in 0..number_of_masters {
                        let (x_delta, y_delta) = (self.read_value()?, self.read_value()?);
                        cur_pos_x += x_delta;
                        cur_pos_y += y_delta;
                        c1_points.push((cur_pos_x, cur_pos_y));
                    }
                    for _ in 0..number_of_masters {
                        let (x_delta, y_delta) = (self.read_value()?, self.read_value()?);
                        cur_pos_x += x_delta;
                        cur_pos_y += y_delta;
                        c2_points.push((cur_pos_x, cur_pos_y));
                    }
                    nodes.push(Node::Curve {
                        coords: points,
                        c1_coords: c1_points,
                        c2_coords: c2_points,
                        flags,
                    });
                }
                PathCommand::QCurve => {
                    let mut c1_points = vec![];
                    for _ in 0..number_of_masters {
                        let (x_delta, y_delta) = (self.read_value()?, self.read_value()?);
                        cur_pos_x += x_delta;
                        cur_pos_y += y_delta;
                        c1_points.push((cur_pos_x, cur_pos_y));
                    }
                    nodes.push(Node::QCurve {
                        coords: points,
                        c1_coords: c1_points,
                        flags,
                    });
                }
            }
        }
        Ok(nodes)
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
            log::trace!(
                "Reading {}",
                GlyphEntry::key_to_variant(u16::from(raw_key)).unwrap_or("an unknown key")
            );
            let entry = GlyphEntry::new_from_reader(u16::from(raw_key), self)?;
            if let Some(e) = entry {
                log::trace!("Read glyph entry: {:?}", e);
                entries.push(e);
            }
        }
        Ok(entries)
    }

    pub fn read_anchors(&mut self) -> Result<Vec<Vec<Anchor>>, Report<VfbError>> {
        let num_anchors = self.read_value()?;
        let num_master = self.read_value()?;
        let mut anchors = vec![];
        for _ in 0..num_anchors {
            let mut this_anchor = vec![];
            for _ in 0..num_master {
                let x = self.read_value()?;
                let y = self.read_value()?;
                this_anchor.push(Anchor { x, y });
            }
            anchors.push(this_anchor);
        }
        Ok(anchors)
    }

    pub fn read_anchors_supplemental(
        &mut self,
    ) -> Result<Vec<AnchorsSupplemental>, Report<VfbError>> {
        let num_anchors = self.read_value()?;
        let mut anchors = vec![];
        for _ in 0..num_anchors {
            let hue = self.read_value()?;
            let reserved = self.read_value()?;
            anchors.push(AnchorsSupplemental { hue, reserved });
        }
        Ok(anchors)
    }
}
