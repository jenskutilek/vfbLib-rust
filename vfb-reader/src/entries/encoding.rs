use std::io::BufReader;
use serde::Serialize;
use crate::buffer;
use crate::entries::VfbEntryTypes;

#[derive(Serialize)]
pub struct EncodingRecord {
    gid: u16,
    name: str,
}

pub fn decompile<R>(r: &mut BufReader<R>) -> Option<VfbEntryTypes> {
    let gid = buffer::read_u16(r);
    let name = buffer::read_str(r, bytes_to_read)
    let er = EncodingRecord {gid, name: &name};

    Some(VfbEntryTypes::Encoding(er))
}
