use crate::{buffer::VfbReader, VfbError};

#[derive(serde::Serialize)]
pub struct Guide {
    position: i32,
    angle: f32,
}

#[derive(serde::Serialize)]
pub struct Guides {
    horizontal: Vec<Vec<Guide>>,
    vertical: Vec<Vec<Guide>>,
}

impl<R> VfbReader<R>
where
    R: std::io::Read,
{
    pub fn read_guides(&mut self) -> Result<Guides, VfbError> {
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
