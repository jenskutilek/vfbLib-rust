use crate::buffer;
use crate::entries::VfbEntryTypes;
use std::io::BufReader;

pub fn decompile<R>(r: &mut BufReader<R>) -> Option<VfbEntryTypes>
where
    R: std::io::Read,
{
    let string = buffer::read_str_remainder(r);

    Some(VfbEntryTypes::String(string))
}
