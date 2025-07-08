use crate::{buffer, entries::VfbEntryTypes, error::VfbError};
use std::io::BufReader;

pub fn decompile<R>(r: &mut BufReader<R>) -> Result<Option<VfbEntryTypes>, VfbError>
where
    R: std::io::Read,
{
    let i = buffer::read_u16(r)?;

    Ok(Some(VfbEntryTypes::UInt16(i)))
}
