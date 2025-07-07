use crate::buffer;
use crate::entries::VfbEntryTypes;
use std::io::BufReader;

pub fn decompile<R>(r: &mut BufReader<R>) -> Option<VfbEntryTypes>
where
    R: std::io::Read,
{
    let gid = buffer::read_u16(r);
    let name = buffer::read_str_remainder(r);

    Some(VfbEntryTypes::Encoding((gid, name)))
}
