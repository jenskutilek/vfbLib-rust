use error_stack::Report;

use crate::{
    buffer::{EntryReader, ReadExt},
    VfbError,
};

#[derive(serde::Serialize, Debug)]
pub struct Guide {
    position: i32,
    angle: f32,
}

#[derive(serde::Serialize, Debug)]
pub struct Guides {
    horizontal: Vec<Vec<Guide>>,
    vertical: Vec<Vec<Guide>>,
}

/// Name/colour properties for a guide. `index` is 1-based into the flat list
/// of horizontal guides followed by vertical guides.
#[derive(serde::Serialize, Debug)]
pub struct GuideProperty {
    pub index: i32,
    pub color: Option<String>,
    pub name: Option<String>,
}

impl<R: std::io::Read + std::io::Seek> EntryReader<'_, R> {
    pub fn read_guides(&mut self) -> Result<Guides, Report<VfbError>> {
        let horizontal_count = self.read_value()? as usize;
        let mut horizontal = Vec::with_capacity(horizontal_count);
        for _ in 0..horizontal_count {
            let mut this_guide = Vec::with_capacity(self.number_of_masters);
            for _ in 0..self.number_of_masters {
                let position = self.read_value()?;
                let angle = (self.read_value()? as f32).atan2(10000.0).to_degrees();
                this_guide.push(Guide { position, angle });
            }
            horizontal.push(this_guide);
        }

        let vertical_count = self.read_value()? as usize;
        let mut vertical = Vec::with_capacity(vertical_count);
        for _ in 0..vertical_count {
            let mut this_guide = Vec::with_capacity(self.number_of_masters);
            for _ in 0..self.number_of_masters {
                let position = self.read_value()?;
                let angle = (self.read_value()? as f32).atan2(10000.0).to_degrees();
                this_guide.push(Guide { position, angle });
            }
            vertical.push(this_guide);
        }

        Ok(Guides {
            horizontal,
            vertical,
        })
    }
}

impl<R: std::io::Read + std::io::Seek> EntryReader<'_, R> {
    pub fn read_guide_properties(&mut self) -> Result<Vec<GuideProperty>, Report<VfbError>> {
        // Two sections: horizontal guides then vertical guides. Each is a
        // sequence of (index, colour, name) entries terminated by index 0.
        let mut properties = Vec::new();
        for _ in 0..2 {
            loop {
                let index = self.read_value()?;
                if index == 0 {
                    break;
                }
                let color_value = self.read_value()?;
                let color = if color_value > -1 {
                    Some(format!("#{:06x}", color_value & 0xFF_FFFF))
                } else {
                    None
                };
                let name_length = self.read_value()?;
                let name = if name_length > 0 {
                    Some(self.read_str_of_len(name_length as u64)?)
                } else {
                    None
                };
                properties.push(GuideProperty { index, color, name });
            }
        }
        // Be defensive: ignore any trailing private data.
        let _ = self.read_bytes_remainder()?;
        Ok(properties)
    }
}

impl Guides {
    /// Get the horizontal guides
    pub fn horizontal(&self) -> &[Vec<Guide>] {
        &self.horizontal
    }

    /// Get the vertical guides
    pub fn vertical(&self) -> &[Vec<Guide>] {
        &self.vertical
    }
}

impl Guide {
    /// Get the guide position
    pub fn position(&self) -> i32 {
        self.position
    }

    /// Get the guide angle in degrees
    pub fn angle(&self) -> f32 {
        self.angle
    }
}
