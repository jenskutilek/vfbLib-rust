use crate::{buffer, entries::VfbEntryType, error::VfbError};
use std::io::BufReader;

pub fn decompile<R>(r: &mut BufReader<R>) -> Result<Option<VfbEntryType>, VfbError>
where
    R: std::io::Read,
{
    let i = buffer::read_u16(r)?;

    Ok(Some(VfbEntryType::UInt16(i)))
}
