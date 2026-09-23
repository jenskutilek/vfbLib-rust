use crate::{buffer::VfbReader, error::VfbError};

use crate::buffer::ReadExt;
use error_stack::Report;
use serde::Serialize;

/// The header of a VFB file.
#[derive(Serialize)]
pub struct Header {
    /// The file signature, always `1A 57 4C 46` (`\x1aWLF`) for FontLab 3.0 and newer.
    pub signature: u32,
    /// The app version, always `0x31` for FontLab 3.0 and newer.
    pub app_version: u8,
    /// The file format version, always `0x30` for FontLab 3.0 and newer.
    pub file_version: u8,
    /// The major version, always `0x03` for FontLab 3.0 and newer.
    pub version_major: u8,
    /// The minor version, always `0x00` for FontLab 3.0 and newer.
    pub version_minor: u8,
}

impl<R: std::io::Read + std::io::Seek> VfbReader<R> {
    /// Read the header from the buffered reader.
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
