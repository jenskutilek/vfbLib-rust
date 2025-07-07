use crate::buffer;
use crate::entries::VfbEntryTypes;
use serde::Serialize;
use std::io::BufReader;

#[derive(Serialize)]
pub struct EncodingRecord {
    gid: u16,
    name: String,
}

pub fn decompile<R>(r: &mut BufReader<R>) -> Option<VfbEntryTypes>
where
    R: std::io::Read,
{
    let gid = buffer::read_u16(r);
    let name = buffer::read_str_remainder(r);
    let er = EncodingRecord { gid, name };

    Some(VfbEntryTypes::Encoding(er))
}
