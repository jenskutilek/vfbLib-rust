use crate::{buffer, entries::VfbEntryType, error::VfbError};
use std::io::BufReader;

pub fn decompile<R>(r: &mut BufReader<R>) -> Result<Option<VfbEntryType>, VfbError>
where
    R: std::io::Read,
{
    let string = buffer::read_str_remainder(r)?;

    Ok(Some(VfbEntryType::String(string)))
}
