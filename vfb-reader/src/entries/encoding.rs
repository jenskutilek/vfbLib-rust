use crate::{buffer, entries::VfbEntryTypes, error::VfbError};
use std::io::BufReader;

pub fn decompile<R>(r: &mut BufReader<R>) -> Result<Option<VfbEntryTypes>, VfbError>
where
    R: std::io::Read,
{
    let gid = buffer::read_u16(r)?;
    let name = buffer::read_str_remainder(r)?;

    Ok(Some(VfbEntryTypes::Encoding((gid, name))))
}
