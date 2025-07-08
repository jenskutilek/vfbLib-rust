use crate::{buffer::VfbReader, entries::VfbEntryType, error::VfbError};

pub fn decompile<R>(r: &mut VfbReader<R>) -> Result<Option<VfbEntryType>, VfbError>
where
    R: std::io::Read,
{
    let string = r.read_str_remainder()?;

    Ok(Some(VfbEntryType::String(string)))
}
