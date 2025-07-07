use crate::buffer;
use crate::entries::VfbEntryTypes;
use std::io::BufReader;

pub fn decompile<R>(r: &mut BufReader<R>) -> Option<VfbEntryTypes>
where
    R: std::io::Read,
{
    let i = buffer::read_u16(r);

    Some(VfbEntryTypes::UInt16(i))
}
