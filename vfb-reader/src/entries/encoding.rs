use crate::{buffer::VfbReader, entries::VfbEntryType, error::VfbError};

pub fn decompile<R>(r: &mut VfbReader<R>) -> Result<Option<VfbEntryType>, VfbError>
where
    R: std::io::Read,
{
    let gid = r.read_u16()?;
    let name = r.read_str_remainder()?;

    Ok(Some(VfbEntryType::Encoding((gid, name))))
}
