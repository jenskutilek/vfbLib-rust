use crate::{buffer::VfbReader, entries::VfbEntryType, error::VfbError};

pub fn decompile<R>(r: &mut VfbReader<R>) -> Result<Option<VfbEntryType>, VfbError>
where
    R: std::io::Read,
{
    let i = r.read_u16()?;

    Ok(Some(VfbEntryType::UInt16(i)))
}
