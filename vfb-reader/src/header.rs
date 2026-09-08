use crate::{buffer::VfbReader, error::VfbError};

use crate::buffer::ReadExt;
use error_stack::Report;
use serde::Serialize;

/// The header of a VFB
#[derive(Serialize)]
pub struct Header {
    signature: u32,
    app_version: u8,
    file_version: u8,
    version_major: u8,
    version_minor: u8,
}

impl<R: std::io::Read + std::io::Seek> VfbReader<R> {
    /// Read the header from the buffered reader
    pub fn read_header(&mut self) -> Result<Header, Report<VfbError>> {
        let signature = self.read_u32()?;
        let app_version = self.read_u8()?;
        let file_version = self.read_u8()?;
        let version_major = self.read_u8()?;
        let version_minor = self.read_u8()?;
        Ok(Header {
            signature,
            app_version,
            file_version,
            version_major,
            version_minor,
        })
    }
}
