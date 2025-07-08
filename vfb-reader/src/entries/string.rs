use crate::{buffer, entries::VfbEntryTypes, error::VfbError};
use std::io::BufReader;

pub fn decompile<R>(r: &mut BufReader<R>) -> Result<Option<VfbEntryTypes>, VfbError>
where
    R: std::io::Read,
{
    let string = buffer::read_str_remainder(r)?;

    Ok(Some(VfbEntryTypes::String(string)))
}
