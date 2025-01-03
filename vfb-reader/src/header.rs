use crate::buffer;

use serde::Serialize;
use std::io::prelude::*;
use std::io::BufReader;

struct VfbHeaderReserved {
    data: [u8; 34],
}

impl Serialize for VfbHeaderReserved {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.data.as_slice().serialize(serializer)
    }
}

#[derive(Serialize)]
pub struct VfbHeader {
    header00: u8,
    filetype: [u8; 5],
    header01: u16,
    header02: u16,
    reserved: VfbHeaderReserved, // TODO: Better store as hexstring like in vfbLib?
    header03: u16,
    header04: u16,
    header05: u16,
    header06: u16,
    header07: u16,
    header08: u16,
    header09: (u8, i32),
    header10: (u8, i32),
    header11: (u8, i32),
    header12: u8,
    header13: u16,
    header14: u16,
}

pub fn read<R>(r: &mut BufReader<R>) -> VfbHeader
where
    R: std::io::Read,
{
    let header00 = buffer::read_u8(r); // 26
    let mut filetype = [0u8; 5]; // WLF10
    r.read_exact(&mut filetype).expect("ValueError");
    let header01 = buffer::read_u16(r); // 3
    let header02 = buffer::read_u16(r); // 44
    let mut res = [0u8; 34]; // 0000000000000000000000000000000000
    r.read_exact(&mut res).expect("ValueError");
    let reserved = VfbHeaderReserved { data: res };
    let header03 = buffer::read_u16(r); // 1
    let header04 = buffer::read_u16(r); // 0
    let header05 = buffer::read_u16(r); // 4
    let header06 = buffer::read_u16(r); // 0
    let mut header07 = buffer::read_u16(r); // 10
    let header08: u16;
    let header09: (u8, i32);
    let header10: (u8, i32);
    let header11: (u8, i32);
    let header12: u8;
    if header07 == 10 {
        // FLS5 additions over the FLS2 format
        header08 = buffer::read_u16(r);
        header09 = (buffer::read_u8(r), buffer::read_value(r));
        header10 = (buffer::read_u8(r), buffer::read_value(r));
        header11 = (buffer::read_u8(r), buffer::read_value(r));
        header12 = buffer::read_u8(r); // or stop byte?
    } else {
        // Upgrade the header to FLS5
        header07 = 10;
        header08 = 0;
        header09 = (1u8, 1i32);
        header10 = (2u8, 0x05020280i32);
        header11 = (3u8, 0i32);
        header12 = 0;
    }
    let header13 = buffer::read_u16(r);
    let header14 = buffer::read_u16(r);

    return VfbHeader {
        header00,
        filetype,
        header01,
        header02,
        reserved,
        header03,
        header04,
        header05,
        header06,
        header07,
        header08,
        header09,
        header10,
        header11,
        header12,
        header13,
        header14,
    };
}
